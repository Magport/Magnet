#![cfg_attr(not(feature = "std"), no_std)]

#[allow(dead_code, unused_imports, non_camel_case_types)]
#[allow(clippy::all)]
#[allow(rustdoc::broken_intra_doc_links)]
#[derive(
	:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq, Eq, scale_info::TypeInfo,
)]
pub struct Id(pub u32);

#[derive(
	:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq, Eq, scale_info::TypeInfo,
)]
pub enum OnDemandEvent {
	#[codec(index = 0)]
	OnDemandOrderPlaced { para_id: Id, spot_price: u128 },
}
#[derive(
	:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq, Eq, scale_info::TypeInfo,
)]
pub enum BalanceStatus {
	#[codec(index = 0)]
	Free,
	#[codec(index = 1)]
	Reserved,
}
#[derive(
	:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq, Eq, scale_info::TypeInfo,
)]
pub enum BalancesEvent {
	// #[codec(index = 8)]
	// Withdraw { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 0)]
	Endowed { account: ::sp_core::crypto::AccountId32, free_balance: ::core::primitive::u128 },
	#[codec(index = 1)]
	DustLost { account: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 2)]
	Transfer {
		from: ::sp_core::crypto::AccountId32,
		to: ::sp_core::crypto::AccountId32,
		amount: ::core::primitive::u128,
	},
	#[codec(index = 3)]
	BalanceSet { who: ::sp_core::crypto::AccountId32, free: ::core::primitive::u128 },
	#[codec(index = 4)]
	Reserved { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 5)]
	Unreserved { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 6)]
	ReserveRepatriated {
		from: ::sp_core::crypto::AccountId32,
		to: ::sp_core::crypto::AccountId32,
		amount: ::core::primitive::u128,
		destination_status: BalanceStatus,
	},
	#[codec(index = 7)]
	Deposit { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 8)]
	Withdraw { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 9)]
	Slashed { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 10)]
	Minted { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 11)]
	Burned { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 12)]
	Suspended { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 13)]
	Restored { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 14)]
	Upgraded { who: ::sp_core::crypto::AccountId32 },
	#[codec(index = 15)]
	Issued { amount: ::core::primitive::u128 },
	#[codec(index = 16)]
	Rescinded { amount: ::core::primitive::u128 },
	#[codec(index = 17)]
	Locked { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 18)]
	Unlocked { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 19)]
	Frozen { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
	#[codec(index = 20)]
	Thawed { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
}

#[derive(
	:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq, Eq, scale_info::TypeInfo,
)]
pub enum RelaychainRuntimeEvent {
	#[codec(index = 4)]
	Balances(BalancesEvent),
	#[codec(index = 66)]
	OnDemandAssignmentProvider(OnDemandEvent),
	#[codec(index = 255)]
	NULL,
}

impl Default for RelaychainRuntimeEvent {
	fn default() -> Self {
		RelaychainRuntimeEvent::NULL
	}
}

#[derive(
	:: codec::Decode, :: codec :: Encode, Clone, Debug, PartialEq, Eq, scale_info::TypeInfo,
)]
pub enum MyPhase {
	/// Applying an extrinsic.
	ApplyExtrinsic(::core::primitive::u32),
	/// Finalizing the block.
	Finalization,
	/// Initializing the block.
	Initialization,
}

impl Default for MyPhase {
	fn default() -> Self {
		Self::Initialization
	}
}
