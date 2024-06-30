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

//! Coretime bulk mode Spawner
//!
//! The technical implementation logic here is to periodically call the background task here based on the interval between
//! parachain releases as the time period.
//! Each time it is called, it first checks the final block of the coretime parachain to see if there is an Assigned event
//! and assigns it to the current parachain. If so, it records it and then looks for subsequent block events to see if there
//! is a CoreAssigned event. If so, it means that coretime is assigned to the parachain,
//! and then the information recorded in the memory is passed to the block through inherent data.
//!
mod metadata;

use codec::Codec;
use cumulus_primitives_core::{relay_chain::BlockNumber as RelayBlockNumber, ParaId};
use cumulus_relay_chain_interface::RelayChainInterface;
use futures::{lock::Mutex, pin_mut, select, FutureExt, StreamExt};
use mc_coretime_common::is_parathread;
use mp_coretime_bulk::{
	self, well_known_keys::broker_regions, BulkMemRecord, BulkMemRecordItem, BulkRuntimeApi,
	BulkStatus,
};
use mp_coretime_common::{
	chain_state_snapshot::GenericStateProof, well_known_keys::SYSTEM_BLOCKHASH_GENESIS,
};
use pallet_broker::{CoreMask, RegionId, RegionRecord};
use polkadot_primitives::{AccountId, Balance};
use sc_client_api::UsageProvider;
use sc_service::TaskManager;
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::AppPublic;
use sp_consensus_aura::AuraApi;
use sp_core::{crypto::Pair, H256};
use sp_runtime::traits::{Block as BlockT, Member};
use sp_state_machine::StorageProof;
use std::{error::Error, sync::Arc};
use subxt::{
	backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
	OnlineClient, PolkadotConfig,
};

/// [u8;10] to u128
fn u8_array_to_u128(array: [u8; 10]) -> u128 {
	let mut result: u128 = 0;
	for &byte in &array {
		result = (result << 8) | byte as u128;
	}
	result
}

/// The main logic of bulk task.
pub async fn coretime_bulk_task<P, R, Block, PB>(
	parachain: &P,
	relay_chain: R,
	_height: RelayBlockNumber,
	p_hash: H256,
	para_id: ParaId,
	bulk_record: Arc<Mutex<BulkMemRecord>>,
) -> Result<(), Box<dyn Error>>
where
	Block: BlockT,
	P: ProvideRuntimeApi<Block> + UsageProvider<Block>,
	R: RelayChainInterface + Clone,
	P::Api: AuraApi<Block, PB::Public> + BulkRuntimeApi<Block>,
	PB: Pair + 'static,
	PB::Public: AppPublic + Member + Codec,
	PB::Signature: TryFrom<Vec<u8>> + Member + Codec,
{
	// Determine whether it is a parathread
	let parathread = is_parathread(relay_chain, p_hash, para_id).await?;
	if !parathread {
		return Ok(());
	}

	let hash = parachain.usage_info().chain.finalized_hash;

	let mut bulk_record_local = bulk_record.lock().await;

	// Get the final block of the coretime parachain through subxt.
	let url = parachain.runtime_api().rpc_url(hash)?;

	let rpc_url = std::str::from_utf8(&url)?;

	let api = OnlineClient::<PolkadotConfig>::from_url(rpc_url).await?;

	let block = api.blocks().at_latest().await?;

	let pre_block_height = bulk_record_local.coretime_para_height;

	let block_number = block.number();

	// If the block number has not changed, it will be returned without any processing.
	if pre_block_height != block_number {
		bulk_record_local.coretime_para_height = block_number;
	} else {
		return Ok(());
	}

	let events = block.events().await?;
	for event in events.iter() {
		let event = event?;
		// Query Broker Assigned Event
		let ev_assigned = event.as_event::<metadata::Assigned>();

		if let Ok(assigned_event) = ev_assigned {
			if let Some(ev) = assigned_event {
				log::info!(
					"=====================Find Assigned event:{:?},{:?},{:?}================",
					ev.region_id,
					ev.task,
					ev.duration
				);

				let pid: u32 = para_id.into();

				if ev.task == pid {
					// Call rpc state_getReadProof.
					let rpc_client = RpcClient::from_url(rpc_url).await?;

					let rpc = LegacyRpcMethods::<PolkadotConfig>::new(rpc_client.clone());

					let mask = u8_array_to_u128(ev.region_id.mask.0);

					let core_mask = CoreMask::from(mask);

					let region_id = RegionId {
						begin: ev.region_id.begin,
						core: ev.region_id.core,
						mask: core_mask,
					};

					let region_key = broker_regions(region_id);
					// coretime parachain genesis hash key
					let block_hash_key = SYSTEM_BLOCKHASH_GENESIS;
					let mut relevant_keys = Vec::new();
					relevant_keys.push(region_key.as_slice());
					relevant_keys.push(block_hash_key);

					let proof = rpc
						.state_get_read_proof(relevant_keys, Some(events.block_hash()))
						.await
						.unwrap();
					let storage_proof =
						StorageProof::new(proof.proof.into_iter().map(|bytes| bytes.to_vec()));

					let storage_root = block.header().state_root;
					// Create coretime parachain storage root proof.
					let relay_storage_rooted_proof: GenericStateProof<
						cumulus_primitives_core::relay_chain::Block,
					> = GenericStateProof::new(storage_root, storage_proof.clone()).unwrap();

					let head_data = relay_storage_rooted_proof
						.read_entry::<RegionRecord<AccountId, Balance>>(region_key.as_slice(), None)
						.ok();
					// Check proof is ok.
					if head_data.is_some() {
						// Record some data.
						let record_item = BulkMemRecordItem {
							storage_proof,
							storage_root,
							region_id,
							duration: ev.duration,
							status: BulkStatus::Assigned,
							start_relaychain_height: 0,
							end_relaychain_height: 0,
						};
						bulk_record_local.items.push(record_item);
					}
				}
				continue;
			}
		}

		// Query CoreAssigned event.
		let ev_core_assigned = event.as_event::<metadata::CoreAssigned>();

		if let Ok(core_assigned_event) = ev_core_assigned {
			if let Some(ev) = core_assigned_event {
				log::info!(
					"=====================Find CoreAssigned event: {:?},{:?},{:?}=================",
					ev.core,
					ev.when,
					ev.assignment
				);

				for (core_assign, _) in ev.assignment {
					if let metadata::CoreAssignment::Task(id) = core_assign {
						let pid: u32 = para_id.into();
						if id == pid {
							let items = &mut bulk_record_local.items;
							for item in items {
								if item.status == BulkStatus::Assigned {
									item.start_relaychain_height = ev.when;

									let constant_query =
										subxt::dynamic::constant("Broker", "TimeslicePeriod");

									let time_slice =
										api.constants().at(&constant_query)?.to_value()?.context;

									item.end_relaychain_height =
										ev.when + item.duration * time_slice;
									// find it.
									item.status = BulkStatus::CoreAssigned;
								}
							}
						}
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
) where
	Block: BlockT,
	P: ProvideRuntimeApi<Block> + UsageProvider<Block>,
	R: RelayChainInterface + Clone,
	P::Api: AuraApi<Block, PB::Public> + BulkRuntimeApi<Block>,
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
						coretime_bulk_task::<_,_,_, PB>(&*parachain, relay_chain.clone(), height, hash, para_id, bulk_record.clone()).await?;
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

/// Spawn task for bulk mode
pub fn spawn_bulk_task<P, R, Block, PB>(
	parachain: Arc<P>,
	para_id: ParaId,
	relay_chain: R,
	task_manager: &TaskManager,
	bulk_record: Arc<Mutex<BulkMemRecord>>,
) -> sc_service::error::Result<()>
where
	Block: BlockT,
	R: RelayChainInterface + Clone + 'static,
	P: Send + Sync + 'static + ProvideRuntimeApi<Block> + UsageProvider<Block>,
	P::Api: AuraApi<Block, PB::Public> + BulkRuntimeApi<Block>,
	PB: Pair + 'static,
	PB::Public: AppPublic + Member + Codec,
	PB::Signature: TryFrom<Vec<u8>> + Member + Codec,
{
	let coretime_bulk_task = run_coretime_bulk_task::<_, _, _, PB>(
		parachain.clone(),
		relay_chain.clone(),
		para_id,
		bulk_record.clone(),
	);
	task_manager
		.spawn_essential_handle()
		.spawn("bulk task", "magport", coretime_bulk_task);
	Ok(())
}
