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

//! Keys of well known.
#![cfg_attr(not(feature = "std"), no_std)]

use cumulus_primitives_core::relay_chain::CoreIndex;
use {
	cumulus_primitives_core::ParaId,
	pallet_broker::RegionId,
	sp_core::Encode,
	sp_io::hashing::{blake2_128, twox_256, twox_64},
	sp_std::vec::Vec,
};

// XXHash a String:Broker Regions
pub const REGIONS: &[u8] =
	&hex_literal::hex!["4dcb50595177a3177648411a42aca0f53dc63b0b76ffd6f80704a090da6f8719"];

/// Broker Regions
pub fn broker_regions(region_id: RegionId) -> Vec<u8> {
	region_id.using_encoded(|region_id_bytes: &[u8]| {
		REGIONS
			.iter()
			.chain(blake2_128(region_id_bytes).iter())
			.chain(region_id_bytes.iter())
			.cloned()
			.collect()
	})
}
