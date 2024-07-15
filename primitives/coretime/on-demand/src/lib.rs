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

//! # On demand Order Inherent Primitives
//!
//! This crate defines those primitives that should be taken into account when building
//! the on demand order pallet inherent
//!
#![cfg_attr(not(feature = "std"), no_std)]
use core::default;

use cumulus_primitives_core::{
	relay_chain::BlockNumber as RelayBlockNumber, relay_chain::Hash as PHash, ParaId,
	PersistedValidationData,
};
use sp_core::H256;
use sp_runtime::sp_std::vec::Vec;
use sp_runtime::traits::MaybeDisplay;
#[cfg(feature = "std")]
pub mod inherent_client;
pub mod well_known_keys;
use codec::{Codec, Decode, Encode};
use {scale_info::TypeInfo, sp_inherents::InherentIdentifier};

#[derive(Encode, Decode, sp_core::RuntimeDebug, Clone, PartialEq, TypeInfo)]
pub struct OrderInherentData<AuthorityId> {
	/// Relaychain block height, for check.
	pub relay_chian_number: RelayBlockNumber,
	/// Author of order.
	pub author_pub: Option<AuthorityId>,
	/// Order price.
	pub price: u128,
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
	/// Hash of relaychain block.
	pub relay_parent: PHash,
	/// Relaychain block height.
	pub relay_height: RelayBlockNumber,
	/// Order status
	pub order_status: OrderStatus,
	/// Author of order.
	pub author_pub: Option<AuthorityId>,
	/// Order price.
	pub price: u128,
	/// Backup transactions hash.
	pub txs: Vec<H256>,
}

impl<AuthorityId> OrderRecord<AuthorityId> {
	pub fn new() -> OrderRecord<AuthorityId> {
		OrderRecord {
			relay_parent: Default::default(),
			relay_height: 0,
			order_status: OrderStatus::Init,
			author_pub: None,
			price: 0,
			txs: Vec::new(),
		}
	}
	pub fn reset(&mut self) {
		self.relay_parent = Default::default();
		self.relay_height = 0;
		self.order_status = OrderStatus::Init;
		self.author_pub = None;
		self.price = 0;
		self.txs = Vec::new();
	}
}

sp_api::decl_runtime_apis! {
	#[api_version(2)]
	pub trait OrderRuntimeApi<Balance, AuthorityId> where
		Balance: Codec + MaybeDisplay,
		AuthorityId:Codec
	{

		fn slot_width()-> u32;

		fn order_max_amount() -> Balance;

		fn reach_txpool_threshold(gas_balance:Balance, core_price:Balance) -> bool;
	}
}
