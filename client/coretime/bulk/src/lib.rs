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
use cumulus_primitives_core::BlockT;
use cumulus_primitives_core::ParaId;
use cumulus_relay_chain_interface::RelayChainInterface;
use polkadot_primitives::AccountId;
use polkadot_primitives::Balance;
use sc_client_api::UsageProvider;
use sc_service::TaskManager;
use sp_api::ProvideRuntimeApi;
use std::error::Error;
use std::sync::Arc;
mod metadata;
use dp_chain_state_snapshot::GenericStateProof;
use magnet_primitives_order::well_known_keys::REGIONS;
use magnet_primitives_order::{self, well_known_keys::broker_regions};
use metadata::api::{runtime_types, runtime_types::coretime_rococo_runtime as polakdot_runtime};
use pallet_broker::RegionRecord;
use pallet_broker::{CoreMask, RegionId};
use sp_state_machine::StorageProof;
use sp_storage::StorageKey;
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

pub fn spawn_bulk_task<T, R, Block>(
	parachain: Arc<T>,
	para_id: ParaId,
	relay_chain: R,
	task_manager: &TaskManager,
) -> sc_service::error::Result<()>
where
	Block: BlockT,
	R: RelayChainInterface + Clone + 'static,
	T: Send + Sync + 'static + ProvideRuntimeApi<Block> + UsageProvider<Block>,
{
	task_manager.spawn_essential_handle().spawn("bulk task", "magport", {
		let parachain = parachain.clone();

		async move {
			let rpc_client = RpcClient::from_url("ws://127.0.0.1:8855").await.unwrap();
			let api =
				OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:8855").await.unwrap();

			loop {
				let mut blocks_sub = api.blocks().subscribe_finalized().await.unwrap();

				// For each block, print a bunch of information about it:
				while let Some(block) = blocks_sub.next().await {
					let block = block.unwrap();

					let block_number = block.header().number;
					let block_hash = block.hash();

					println!("Block #{block_number}:");
					println!("  Hash: {block_hash}");
					println!("  Extrinsics:");
					let extrinsics = block.extrinsics().await.unwrap();
					for ext in extrinsics.iter() {
						let ext = ext.unwrap();
						let events = ext.events().await.unwrap();
						let purchase_event = events
							.find_first::<metadata::api::broker::events::Purchased>()
							.unwrap();
						if let Some(ev) = purchase_event {
							println!(
								"Purchased success: value: {:?},{:?},{:?},{:?}",
								ev.who, ev.region_id, ev.price, ev.duration
							);
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
							let storage_proof = StorageProof::new(
								proof.proof.into_iter().map(|bytes| bytes.to_vec()),
							);
							println!("{:?}", storage_proof);
							let storage_root = block.header().state_root;
							let relay_storage_rooted_proof: GenericStateProof<
								cumulus_primitives_core::relay_chain::Block,
							> = GenericStateProof::new(storage_root, storage_proof).unwrap();
							let head_data = relay_storage_rooted_proof
								.read_entry::<RegionRecord<AccountId, Balance>>(
									key.as_slice(),
									None,
								)
								.ok();
							println!("head_data:{:?}", head_data);
						}
					}
				}
			}
		}
	});
	Ok(())
}
