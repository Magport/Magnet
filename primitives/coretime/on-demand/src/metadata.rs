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
pub enum BalancesEvent {
	#[codec(index = 8)]
	Withdraw { who: ::sp_core::crypto::AccountId32, amount: ::core::primitive::u128 },
}

#[derive(
	:: codec :: Decode, :: codec :: Encode, Clone, Debug, PartialEq, Eq, scale_info::TypeInfo,
)]
pub enum RelaychainRuntimeEvent {
	#[codec(index = 4)]
	Balances(BalancesEvent),
	#[codec(index = 66)]
	OnDemandAssignmentProvider(OnDemandEvent),
}
