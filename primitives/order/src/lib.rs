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

#![cfg_attr(not(feature = "std"), no_std)]
use cumulus_primitives_core::{
	relay_chain::BlockNumber as RelayBlockNumber, relay_chain::Hash as PHash, ParaId,
	PersistedValidationData,
};
use sp_runtime::traits::MaybeDisplay;
#[cfg(feature = "std")]
pub mod inherent_client;
pub mod well_known_keys;
#[cfg(feature = "std")]
pub use inherent_client::*;
use {
	parity_scale_codec::{Codec, Decode, Encode},
	scale_info::TypeInfo,
	sp_inherents::InherentIdentifier,
};

#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct OrderInherentData<AuthorityId> {
	pub relay_storage_proof: sp_trie::StorageProof,
	pub validation_data: Option<PersistedValidationData>,
	pub para_id: ParaId,
	pub sequence_number: u64,
	pub author_pub: Option<AuthorityId>,
}

// Identifier of the order inherent
pub const INHERENT_IDENTIFIER: InherentIdentifier = *b"orderiht";

#[derive(Clone, PartialEq)]
pub enum OrderStatus {
	Init,
	Order,
	Execute,
	Complete,
}

#[derive(Clone)]
pub struct OrderRecord<AuthorityId> {
	pub relay_parent: Option<PHash>,
	pub relay_height: RelayBlockNumber,
	pub relay_base: PHash,
	pub relay_base_height: RelayBlockNumber,
	pub order_status: OrderStatus,
	pub validation_data: Option<PersistedValidationData>,
	pub para_id: ParaId,
	pub sequence_number: u64,
	pub author_pub: Option<AuthorityId>,
}

sp_api::decl_runtime_apis! {
	#[api_version(2)]
	pub trait OrderRuntimeApi<Balance, AuthorityId> where
		Balance: Codec + MaybeDisplay,
		AuthorityId:Codec
	{

		fn slot_width()-> u32;

		fn sequence_number()-> u64;

		fn current_relay_height()->u32;

		fn order_max_amount() -> Balance;

		fn order_placed(
			relay_storage_proof: sp_trie::StorageProof,
			validation_data: PersistedValidationData,
			para_id:ParaId,
		)-> Option<AuthorityId>;

		fn reach_txpool_threshold(gas_balance:Balance) -> bool;

		fn order_executed(sequence_number:u64) -> bool ;
	}
}
