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

use cumulus_primitives_core::relay_chain::CoreIndex;
use {
	cumulus_primitives_core::ParaId,
	sp_core::Encode,
	sp_io::hashing::{twox_256, twox_64},
	sp_std::vec::Vec,
};

pub const PARAS_PARA_LIFECYCLES: &[u8] =
	&hex_literal::hex!["cd710b30bd2eab0352ddcc26417aa194281e0bfde17b36573208a06cb5cfba6b"];

pub fn paras_para_lifecycles(para_id: ParaId) -> Vec<u8> {
	para_id.using_encoded(|para_id: &[u8]| {
		PARAS_PARA_LIFECYCLES
			.iter()
			.chain(twox_64(para_id).iter())
			.chain(para_id.iter())
			.cloned()
			.collect()
	})
}

pub const SYSTEM_BLOCKHASH: &[u8] = &hex_literal::hex![
	"26aa394eea5630e07c48ae0c9558cef7a44704b568d21667356a5a050c118746b4def25cfda6ef3a00000000"
];

pub const SYSTEM_ACCOUNT: &[u8] =
	&hex_literal::hex!["26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9"];

pub const SYSTEM_EVENTS: &[u8] =
	&hex_literal::hex!["26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7"];

//OnDemandAssignmentProvider OnDemandQueue
pub const ON_DEMAND_QUEUE: &[u8] =
	&hex_literal::hex!["8f32430b49607f8d60bfd3a003ddf4b53f35b69d817556cf6b886e5b4f01fbdc"];

//OnDemandAssignmentProvider SpotTraffic
pub const SPOT_TRAFFIC: &[u8] =
	&hex_literal::hex!["8f32430b49607f8d60bfd3a003ddf4b5c9308a8e0e640735727536bd9069b11e"];

//Configuration ActiveConfig
pub const ACTIVE_CONFIG: &[u8] =
	&hex_literal::hex!["06de3d8a54d27e44a9d5ce189618f22db4b49d95320d9021994c850f25b8e385"];

pub const CORE_DESCRIPTORS: &[u8] =
	&hex_literal::hex!["638595eebaa445ce03a13547bece90e704e6ac775a3245623103ffec2cb2c92f"];

pub fn paras_core_descriptors(core_index: CoreIndex) -> Vec<u8> {
	core_index.using_encoded(|core_index: &[u8]| {
		CORE_DESCRIPTORS.iter().chain(twox_256(core_index).iter()).cloned().collect()
	})
}
