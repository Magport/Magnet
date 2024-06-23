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
use codec::{Codec, Decode};
use cumulus_primitives_core::BlockT;
use cumulus_primitives_core::ParaId;
use cumulus_relay_chain_interface::RelayChainInterface;
use mp_coretime_common::well_known_keys::paras_para_lifecycles;
use pallet_broker::RegionRecord;
use pallet_broker::{CoreMask, RegionId};
use polkadot_primitives::AccountId;
use polkadot_primitives::Balance;
use runtime_parachains::{configuration::HostConfiguration, paras::ParaLifecycle};
use sc_client_api::UsageProvider;
use sc_service::TaskManager;
use sp_api::ProvideRuntimeApi;
use sp_core::H256;
use sp_state_machine::StorageProof;
use sp_storage::StorageKey;
use std::error::Error;
use std::sync::Arc;
use subxt::client::OfflineClientT;
use subxt::{
	backend::{legacy::LegacyRpcMethods, rpc::RpcClient},
	config::polkadot::PolkadotExtrinsicParamsBuilder as Params,
	tx::Signer,
	utils::MultiSignature,
	Config, OnlineClient, PolkadotConfig,
};

pub async fn is_parathread(
	relay_chain: impl RelayChainInterface + Clone,
	p_hash: H256,
	para_id: ParaId,
) -> Result<bool, Box<dyn Error>> {
	let para_lifecycles_storage = relay_chain
		.get_storage_by_key(p_hash, paras_para_lifecycles(para_id).as_slice())
		.await?;
	let para_lifecycles = para_lifecycles_storage
		.map(|raw| <ParaLifecycle>::decode(&mut &raw[..]))
		.transpose()?;
	let is_parathread = match para_lifecycles {
		Some(lifecycles) => matches!(
			lifecycles,
			ParaLifecycle::Parathread
				| ParaLifecycle::UpgradingParathread
				| ParaLifecycle::OffboardingParathread
		),
		None => false,
	};
	Ok(is_parathread)
}
