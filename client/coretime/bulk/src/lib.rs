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
use sc_client_api::UsageProvider;
use sc_service::TaskManager;
use sp_api::ProvideRuntimeApi;
use std::error::Error;
use std::sync::Arc;
mod metadata;
use subxt::client::OfflineClientT;
use subxt::{
	config::polkadot::PolkadotExtrinsicParamsBuilder as Params, tx::Signer, utils::MultiSignature,
	Config, OnlineClient, PolkadotConfig,
};

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
			let api =
				OnlineClient::<PolkadotConfig>::from_url("ws://127.0.0.1:8855").await.unwrap();

			loop {
				let events = api.events().at_latest().await.unwrap();

				let purchase_event =
					events.find_first::<metadata::api::broker::events::Purchased>().unwrap();
				if let Some(ev) = purchase_event {
					println!(
						"Purchased success: value: {:?},{:?},{:?},{:?}",
						ev.who, ev.region_id, ev.price, ev.duration
					);
				}
			}
		}
	});
	Ok(())
}
