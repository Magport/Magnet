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

use frame_support::weights::Weight;
use sp_core::crypto::AccountId32;

// Base account id b"system:base' and fill with 1u32
const A: [u8; 32] = [
	115, 121, 115, 116, 101, 109, 58, 98, 97, 115, 101, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
	1, 1, 1, 1, 1, 1, 1,
];
pub const BASE_ACCOUNT: AccountId32 = AccountId32::new(A);

pub trait Liquidate {
	fn liquidate() -> Weight;
}

impl Liquidate for () {
	fn liquidate() -> Weight {
		Weight::zero()
	}
}

sp_api::decl_runtime_apis! {
	// API for on_relaychain call
	pub trait OnRelayChainApi {
		// return on_relaychain call result, 1 for force bid coretime
		fn on_relaychain(blocknumber: u32) -> i32;
	}
}
