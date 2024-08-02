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

//! Relaychain runtime metadata.
//!
//! The data type here is generated by the subxt tool and represents the related data type of the relaychain.
//!

#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
	Eq,
	PartialEq,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub enum CoreAssignment {
	#[codec(index = 0)]
	Idle,
	#[codec(index = 1)]
	Pool,
	#[codec(index = 2)]
	Task(u32),
}

#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub struct QueueDescriptor<_0> {
	pub first: _0,
	pub last: _0,
}
#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub struct PartsOf57600(pub u16);
#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub struct AssignmentState {
	pub ratio: PartsOf57600,
	pub remaining: PartsOf57600,
}

#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub struct WorkState<_0> {
	pub assignments: Vec<(CoreAssignment, AssignmentState)>,
	pub end_hint: Option<_0>,
	pub pos: u16,
	pub step: PartsOf57600,
}
#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub struct CoreDescriptor<_0> {
	pub queue: Option<QueueDescriptor<_0>>,
	pub current_work: Option<WorkState<_0>>,
}

#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: CompactAs,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub struct Id(pub u32);

#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub struct PlaceOrderAllowDeath {
	pub max_amount: u128,
	pub para_id: Id,
}

impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for PlaceOrderAllowDeath {
	const PALLET: &'static str = "OnDemandAssignmentProvider";
	const CALL: &'static str = "place_order_allow_death";
}

pub fn place_order_allow_death(
	max_amount: u128,
	para_id: Id,
) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<PlaceOrderAllowDeath> {
	::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
		"OnDemandAssignmentProvider",
		"place_order_allow_death",
		PlaceOrderAllowDeath { max_amount, para_id },
		[
			42u8, 115u8, 192u8, 118u8, 20u8, 174u8, 114u8, 94u8, 177u8, 195u8, 175u8, 214u8, 175u8,
			25u8, 167u8, 135u8, 194u8, 251u8, 186u8, 185u8, 218u8, 153u8, 182u8, 166u8, 28u8,
			238u8, 72u8, 64u8, 115u8, 67u8, 58u8, 165u8,
		],
	)
}

#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub struct PlaceOrderKeepAlive {
	pub max_amount: u128,
	pub para_id: Id,
}

impl ::subxt::ext::subxt_core::blocks::StaticExtrinsic for PlaceOrderKeepAlive {
	const PALLET: &'static str = "OnDemandAssignmentProvider";
	const CALL: &'static str = "place_order_keep_alive";
}

#[allow(dead_code)]
pub fn place_order_keep_alive(
	max_amount: u128,
	para_id: Id,
) -> ::subxt::ext::subxt_core::tx::payload::StaticPayload<PlaceOrderKeepAlive> {
	::subxt::ext::subxt_core::tx::payload::StaticPayload::new_static(
		"OnDemandAssignmentProvider",
		"place_order_keep_alive",
		PlaceOrderKeepAlive { max_amount, para_id },
		[
			112u8, 56u8, 202u8, 218u8, 85u8, 138u8, 45u8, 213u8, 119u8, 36u8, 62u8, 138u8, 217u8,
			95u8, 25u8, 86u8, 119u8, 192u8, 57u8, 245u8, 34u8, 225u8, 247u8, 116u8, 114u8, 230u8,
			130u8, 180u8, 163u8, 190u8, 106u8, 5u8,
		],
	)
}

#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub struct OnDemandOrderPlaced {
	pub para_id: Id,
	pub spot_price: u128,
	pub ordered_by: ::subxt::ext::subxt_core::utils::AccountId32,
}

#[derive(
	:: subxt :: ext :: subxt_core :: ext :: codec :: Decode,
	:: subxt :: ext :: subxt_core :: ext :: codec :: Encode,
	:: subxt :: ext :: subxt_core :: ext :: scale_decode :: DecodeAsType,
	:: subxt :: ext :: subxt_core :: ext :: scale_encode :: EncodeAsType,
	Debug,
)]
# [codec (crate = :: subxt :: ext :: subxt_core :: ext :: codec)]
#[decode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_decode")]
#[encode_as_type(crate_path = ":: subxt :: ext :: subxt_core :: ext :: scale_encode")]
pub struct OnDemandOrderPlacedV0 {
	pub para_id: Id,
	pub spot_price: u128,
}

impl ::subxt::ext::subxt_core::events::StaticEvent for OnDemandOrderPlaced {
	const PALLET: &'static str = "OnDemandAssignmentProvider";
	const EVENT: &'static str = "OnDemandOrderPlaced";
}

impl ::subxt::ext::subxt_core::events::StaticEvent for OnDemandOrderPlacedV0 {
	const PALLET: &'static str = "OnDemandAssignmentProvider";
	const EVENT: &'static str = "OnDemandOrderPlaced";
}
