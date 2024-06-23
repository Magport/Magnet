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

use crate::well_known_keys::broker_regions;
use cumulus_primitives_core::relay_chain::Balance;
use pallet_broker::RegionId;
use sp_consensus_aura::sr25519::AuthorityId;
use {
	crate::BulkInherentData,
	cumulus_primitives_core::{ParaId, PersistedValidationData},
	cumulus_relay_chain_interface::{PHash, RelayChainInterface},
};

/// Collect the relevant coretime para chain state in form of a proof
/// for putting it into the bulk inherent.
// async fn collect_coretime_parachain_storage_proof(
// 	region_id: RegionId,
// ) -> Option<sp_state_machine::StorageProof> {
// 	let mut relevant_keys = Vec::new();
// 	//Broker Regions
// 	relevant_keys.push(broker_regions(region_id));

// 	// let relay_storage_proof = relay_chain_interface.prove_read(relay_parent, &relevant_keys).await;
// 	// match relay_storage_proof {
// 	// 	Ok(proof) => Some(proof),
// 	// 	Err(err) => {
// 	// 		log::info!("RelayChainError:{:?}", err);
// 	// 		None
// 	// 	},
// 	// }
// 	None
// }

impl BulkInherentData {
	/// Create the [`BulkInherentData`] at the given `relay_parent`.
	///
	/// Returns `None` if the creation failed.
	pub async fn create_at(
		// relay_parent: PHash,
		// author_pub: &AuthorityId,
		// region_id: RegionId,
		storage_proof: &sp_trie::StorageProof,
		storage_root: PHash,
		region_id: RegionId,
	) -> Option<BulkInherentData> {
		// let storage_proof = collect_coretime_parachain_storage_proof(region_id).await?;

		Some(BulkInherentData {
			storage_proof: storage_proof.clone(),
			storage_root,
			region_id,
			// purchaser: author_pub.clone(),
			// price: 0,
			// duration: 0,
		})
	}
}

// Implementation of InherentDataProvider
#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for BulkInherentData {
	async fn provide_inherent_data(
		&self,
		inherent_data: &mut sp_inherents::InherentData,
	) -> Result<(), sp_inherents::Error> {
		inherent_data.put_data(crate::INHERENT_IDENTIFIER, &self)
	}

	async fn try_handle_error(
		&self,
		_: &sp_inherents::InherentIdentifier,
		_: &[u8],
	) -> Option<Result<(), sp_inherents::Error>> {
		None
	}
}
