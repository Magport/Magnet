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

//! Coretime parachain runtime metadata.
//!
//! The data type here is generated by the subxt tool and represents the event-related data of the coretime parachain.
//!
#[derive(
	:: subxt :: ext :: codec :: Decode,
	:: subxt :: ext :: codec :: Encode,
	:: subxt :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
pub enum CoreAssignment {
	#[codec(index = 0)]
	Idle,
	#[codec(index = 1)]
	Pool,
	#[codec(index = 2)]
	Task(u32),
}

#[derive(
	:: subxt :: ext :: codec :: Decode,
	:: subxt :: ext :: codec :: Encode,
	:: subxt :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
pub struct CoreAssigned {
	pub core: u16,
	pub when: u32,
	pub assignment: Vec<(CoreAssignment, u16)>,
}

impl subxt::events::StaticEvent for CoreAssigned {
	const PALLET: &'static str = "Broker";
	const EVENT: &'static str = "CoreAssigned";
}

#[derive(
	:: subxt :: ext :: codec :: Decode,
	:: subxt :: ext :: codec :: Encode,
	:: subxt :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
pub struct CoreMask(pub [u8; 10usize]);

#[derive(
	:: subxt :: ext :: codec :: Decode,
	:: subxt :: ext :: codec :: Encode,
	:: subxt :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
pub struct RegionId {
	pub begin: u32,
	pub core: u16,
	pub mask: CoreMask,
}

#[derive(
	:: subxt :: ext :: codec :: Decode,
	:: subxt :: ext :: codec :: Encode,
	:: subxt :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: scale_encode")]
pub struct Assigned {
	pub region_id: RegionId,
	pub duration: u32,
	pub task: u32,
}

impl ::subxt::events::StaticEvent for Assigned {
	const PALLET: &'static str = "Broker";
	const EVENT: &'static str = "Assigned";
}
