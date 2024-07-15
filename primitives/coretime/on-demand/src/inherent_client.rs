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

// use sp_consensus_aura::sr25519::AuthorityId;
use crate::well_known_keys::{acount_balance, ON_DEMAND_QUEUE};
use crate::Codec;
use sp_consensus_aura::sr25519::AuthorityId;
use sp_core::{crypto::UncheckedFrom, ByteArray};
use sp_runtime::AccountId32;
use {
	crate::OrderInherentData,
	cumulus_primitives_core::{ParaId, PersistedValidationData},
	cumulus_relay_chain_interface::{PHash, RelayChainInterface},
};

impl OrderInherentData<AuthorityId> {
	/// Create the [`OrderInherentData`] at the given `relay_parent`.
	///
	/// Returns `None` if the creation failed.
	pub async fn create_at(
		relay_chian_number: u32,
		author_pub: &Option<AuthorityId>,
		price: u128,
	) -> Option<OrderInherentData<AuthorityId>> {
		Some(OrderInherentData { relay_chian_number, author_pub: author_pub.clone(), price })
	}
}

// Implementation of InherentDataProvider
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
