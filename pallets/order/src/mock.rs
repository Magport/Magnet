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

use crate::{self as order_pallet, OrderGasCost};
use codec::Encode;
pub use frame_support::{
	construct_runtime, derive_impl, parameter_types,
	traits::{Everything, Hooks},
};
use frame_system as system;
use frame_system::{pallet_prelude::BlockNumberFor, EnsureRoot};
pub use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_core::{crypto::AccountId32, H256};
use sp_runtime::{
	traits::{BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
	BuildStorage, MultiSignature,
};

type Block = frame_system::mocking::MockBlock<Test>;
type Signature = MultiSignature;
type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;
type Balance = u128;
// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Event<T>},
		OrderPallet: order_pallet::{Pallet, Call, Storage, Event<T>},
		MockPallet: mock_pallet,
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

#[derive_impl(frame_system::config_preludes::ParaChainDefaultConfig as frame_system::DefaultConfig)]
impl system::Config for Test {
	type BaseCallFilter = Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<Balance>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}
parameter_types! {
	pub const ExistentialDeposit: u64 = 5;
}
impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type Balance = u128;
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type RuntimeFreezeReason = ();
}
parameter_types! {
	pub const SlotWidth: u32 = 2;
	pub const OrderMaxAmount:Balance = 200000000;
	pub const TxPoolThreshold:Balance = 3000000000;
}
impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AuthorityId = AuraId;
	type UpdateOrigin = EnsureRoot<AccountId>;
	type Currency = Balances;
	type OrderMaxAmount = OrderMaxAmount;
	type SlotWidth = SlotWidth;
	type TxPoolThreshold = TxPoolThreshold;
	type WeightInfo = ();
}
pub struct OrderGasCostHandler();

impl<T> OrderGasCost<T> for OrderGasCostHandler
where
	T: crate::Config,
	T::AccountId: From<[u8; 32]>,
{
	fn gas_cost(block_number: BlockNumberFor<T>) -> Option<(T::AccountId, Balance)> {
		let sequece_number = <crate::Pallet<T>>::block_2_sequence(block_number)?;
		let order = <crate::Pallet<T>>::order_map(sequece_number)?;
		let mut r = [0u8; 32];
		r.copy_from_slice(order.orderer.encode().as_slice());
		let account = T::AccountId::try_from(r).ok()?;
		Some((account, order.price))
	}
}

#[frame_support::pallet]
pub mod mock_pallet {
	use super::*;
	#[pallet::config]
	pub trait Config: frame_system::Config {
		type OrderGasCost: OrderGasCost<Self>;
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	impl<T: Config> Pallet<T> {
		pub fn get_gas_cost(block_number: BlockNumberFor<T>) -> Option<(T::AccountId, Balance)> {
			T::OrderGasCost::gas_cost(block_number)
		}
	}
}

impl mock_pallet::Config for Test {
	type OrderGasCost = OrderGasCostHandler;
}
pub struct ExtBuilder {
	balances: Vec<(AccountId32, u128)>,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { balances: Default::default() }
	}
}

impl ExtBuilder {
	pub fn build(self) -> sp_io::TestExternalities {
		// Build genesis storage according to the mock runtime.
		let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
		pallet_balances::GenesisConfig::<Test> { balances: self.balances }
			.assimilate_storage(&mut t)
			.unwrap();
		let mut ext = sp_io::TestExternalities::new(t);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
