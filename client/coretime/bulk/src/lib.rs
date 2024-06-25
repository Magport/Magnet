// Copyright (C) Magnet.
// This file is part of Magnet.

// Magnet is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Magnet is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Magnet.  If not, see <http://www.gnu.org/licenses/>.
use cumulus_primitives_core::ParaId;
use cumulus_relay_chain_interface::RelayChainInterface;
use polkadot_primitives::AccountId;
use polkadot_primitives::Balance;
use sc_client_api::UsageProvider;
use sc_service::TaskManager;
use sp_api::ProvideRuntimeApi;
use sp_core::ByteArray;
use std::error::Error;
use std::sync::Arc;
mod metadata;
use codec::{Codec, Decode};
use cumulus_primitives_core::relay_chain::BlockNumber as RelayBlockNumber;
use dp_chain_state_snapshot::GenericStateProof;
use futures::{lock::Mutex, pin_mut, select, FutureExt, Stream, StreamExt};
use mc_coretime_common::is_parathread;
use metadata::api::{runtime_types, runtime_types::coretime_rococo_runtime as polakdot_runtime};
use mp_coretime_bulk::well_known_keys::REGIONS;
use mp_coretime_bulk::BulkMemRecord;
use mp_coretime_bulk::{self, well_known_keys::broker_regions};
use pallet_broker::RegionRecord;
use pallet_broker::{CoreMask, RegionId};
use sp_application_crypto::AppPublic;
use sp_consensus_aura::AuraApi;
use sp_core::{crypto::Pair, H256};
use sp_keystore::KeystorePtr;
use sp_runtime::{
	codec::Encode,
	traits::{AtLeast32BitUnsigned, Block as BlockT, Header as HeaderT, MaybeDisplay, Member},
};
use sp_state_machine::StorageProof;
use sp_storage::StorageKey;
use std::fmt::Debug;
use subxt::client::OfflineClientT;
use subxt::{
	backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
	config::polkadot::PolkadotExtrinsicParamsBuilder as Params,
	tx::Signer,
	utils::MultiSignature,
	Config, OnlineClient, PolkadotConfig,
};

fn u8_array_to_u128(array: [u8; 10]) -> u128 {
	let mut result: u128 = 0;
	for &byte in &array {
		result = (result << 8) | byte as u128;
	}
	result
}

pub async fn coretime_bulk_task<P, R, Block, PB>(
	parachain: &P,
	relay_chain: R,
	height: RelayBlockNumber,
	p_hash: H256,
	para_id: ParaId,
	bulk_record: Arc<Mutex<BulkMemRecord>>,
	keystore: KeystorePtr,
) -> Result<(), Box<dyn Error>>
where
	Block: BlockT,
	P: ProvideRuntimeApi<Block> + UsageProvider<Block>,
	R: RelayChainInterface + Clone,
	P::Api: AuraApi<Block, PB::Public>,
	PB: Pair + 'static,
	PB::Public: AppPublic + Member + Codec,
	PB::Signature: TryFrom<Vec<u8>> + Member + Codec,
{
	// Determine whether it is a parathread
	let parathread = is_parathread(relay_chain, p_hash, para_id).await?;
	if !parathread {
		return Ok(());
	}

	// Randomly select a collator to perform the following operations.
	let hash = parachain.usage_info().chain.finalized_hash;
	let authorities = parachain.runtime_api().authorities(hash).map_err(Box::new)?;
	let auth_len = authorities.len() as u32;
	let idx = height % auth_len;
	let collator_public = mc_coretime_common::order_slot::<PB>(idx, &authorities, &keystore).await;

	if collator_public.is_none() {
		return Ok(());
	}
	// Query Broker Assigned Event
	let api = OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:8855").await?;
	let block = api.blocks().at_latest().await?;
	{
		let mut bulk_record_local = bulk_record.lock().await;
		let pre_block_height = bulk_record_local.coretime_para_height;
		let block_number = block.number();
		if pre_block_height != block_number {
			bulk_record_local.coretime_para_height = block_number;
		} else {
			return Ok(());
		}
	}

	let events = block.events().await?;
	for event in events.iter() {
		let event = event?;
		let event_detail = event.as_event::<metadata::api::broker::events::Assigned>();
		if let Ok(assigned_event) = event_detail {
			if let Some(ev) = assigned_event {
				log::info!("{:?},{:?},{:?}", ev.region_id, ev.task, ev.duration);
				let pid: u32 = para_id.into();
				if ev.task == pid {
					//
					let rpc_client = RpcClient::from_url("ws://127.0.0.1:8855").await?;
					let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client.clone());
					let mask = u8_array_to_u128(ev.region_id.mask.0);
					let core_mask = CoreMask::from(mask);
					let region_id = RegionId {
						begin: ev.region_id.begin,
						core: ev.region_id.core,
						mask: core_mask,
					};
					println!("region_id:{:?}", region_id);
					let key = broker_regions(region_id);
					println!("key:{:?}", key);
					let mut relevant_keys = Vec::new();
					relevant_keys.push(key.as_slice());
					let proof = rpc
						.state_get_read_proof(relevant_keys, Some(events.block_hash()))
						.await
						.unwrap();
					let storage_proof =
						StorageProof::new(proof.proof.into_iter().map(|bytes| bytes.to_vec()));
					println!("{:?}", storage_proof);
					let storage_root = block.header().state_root;
					let relay_storage_rooted_proof: GenericStateProof<
						cumulus_primitives_core::relay_chain::Block,
					> = GenericStateProof::new(storage_root, storage_proof.clone()).unwrap();
					let head_data = relay_storage_rooted_proof
						.read_entry::<RegionRecord<AccountId, Balance>>(key.as_slice(), None)
						.ok();
					println!("head_data:{:?}", head_data);
					if let Some(region_record) = head_data {
						let mut bulk_record_local = bulk_record.lock().await;
						bulk_record_local.storage_proof = storage_proof;
						bulk_record_local.storage_root = storage_root;
						bulk_record_local.region_id = region_id;
					}
				}
			}
		}
	}
	Ok(())
}

pub async fn run_coretime_bulk_task<P, R, Block, PB>(
	parachain: Arc<P>,
	relay_chain: R,
	para_id: ParaId,
	bulk_record: Arc<Mutex<BulkMemRecord>>,
	keystore: KeystorePtr,
) where
	Block: BlockT,
	P: ProvideRuntimeApi<Block> + UsageProvider<Block>,
	R: RelayChainInterface + Clone,
	P::Api: AuraApi<Block, PB::Public>,
	PB: Pair + 'static,
	PB::Public: AppPublic + Member + Codec,
	PB::Signature: TryFrom<Vec<u8>> + Member + Codec,
{
	let relay_chain_notification = async move {
		let new_best_heads = relay_chain
			.new_best_notification_stream()
			.await?
			.filter_map(move |n| async move { Some((n.number, n.hash())) })
			.fuse();
		pin_mut!(new_best_heads);
		loop {
			select! {
				h = new_best_heads.next() => {
								match h {
					Some((height, hash)) => {
						coretime_bulk_task::<_,_,_, PB>(&*parachain, relay_chain.clone(), height, hash, para_id, bulk_record.clone(), keystore.clone()).await?;
					},
					None => {
						return Ok::<(), Box<dyn Error>>(());
					}
				}
				}
			}
		}
	};
	select! {
		_ = relay_chain_notification.fuse() =>  {},
	}
}

pub fn spawn_bulk_task<P, R, Block, PB>(
	parachain: Arc<P>,
	para_id: ParaId,
	relay_chain: R,
	task_manager: &TaskManager,
	bulk_record: Arc<Mutex<BulkMemRecord>>,
	keystore: KeystorePtr,
) -> sc_service::error::Result<()>
where
	Block: BlockT,
	R: RelayChainInterface + Clone + 'static,
	P: Send + Sync + 'static + ProvideRuntimeApi<Block> + UsageProvider<Block>,
	P::Api: AuraApi<Block, PB::Public>,
	PB: Pair + 'static,
	PB::Public: AppPublic + Member + Codec,
	PB::Signature: TryFrom<Vec<u8>> + Member + Codec,
{
	let coretime_bulk_task = run_coretime_bulk_task::<_, _, _, PB>(
		parachain.clone(),
		relay_chain.clone(),
		para_id,
		bulk_record.clone(),
		keystore,
	);
	task_manager
		.spawn_essential_handle()
		.spawn("bulk task", "magport", coretime_bulk_task);
	Ok(())
}
