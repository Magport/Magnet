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

#[derive(Clone)]
pub struct OrderRecord<AuthorityId> {
	pub relay_parent: Option<PHash>,
	pub relay_height: RelayBlockNumber,
	pub relay_base: PHash,
	pub order_complete: bool,
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
		fn place_order()-> Option<u64>;

		fn slot_width()-> u32;

		fn sequence_number()-> u64;

		fn order_max_amount() -> Balance;

		fn order_placed(
			relay_storage_proof: sp_trie::StorageProof,
			validation_data: PersistedValidationData,
			author_pub: AuthorityId,
			para_id:ParaId,
		)-> bool;

		fn reach_txpool_threshold(gas_balance:Balance) -> bool;

		fn order_executed(sequence_number:u64) -> bool ;
	}
}
