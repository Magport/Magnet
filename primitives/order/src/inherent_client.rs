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

use sp_consensus_aura::sr25519::AuthorityId;
use {
	crate::OrderInherentData,
	cumulus_primitives_core::{ParaId, PersistedValidationData},
	cumulus_relay_chain_interface::{PHash, RelayChainInterface},
};

async fn collect_relay_storage_proof(
	relay_chain_interface: &impl RelayChainInterface,
	relay_parent: PHash,
) -> Option<sp_state_machine::StorageProof> {
	let mut relevant_keys = Vec::new();
	//System Events
	relevant_keys.push(
		hex_literal::hex!["26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7"]
			.to_vec(),
	);

	let relay_storage_proof = relay_chain_interface.prove_read(relay_parent, &relevant_keys).await;
	match relay_storage_proof {
		Ok(proof) => Some(proof),
		Err(err) => {
			log::info!("RelayChainError:{:?}", err);
			None
		},
	}
}

impl OrderInherentData<AuthorityId> {
	pub async fn create_at(
		relay_parent: PHash,
		relay_chain_interface: &impl RelayChainInterface,
		validation_data: &Option<PersistedValidationData>,
		para_id: ParaId,
		sequence_number: u64,
		author_pub: &Option<AuthorityId>,
	) -> Option<OrderInherentData<AuthorityId>> {
		let relay_storage_proof =
			collect_relay_storage_proof(relay_chain_interface, relay_parent).await?;

		Some(OrderInherentData {
			relay_storage_proof: relay_storage_proof.clone(),
			validation_data: validation_data.clone(),
			para_id,
			sequence_number,
			author_pub: author_pub.clone(),
		})
	}
}

#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for OrderInherentData<AuthorityId> {
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
