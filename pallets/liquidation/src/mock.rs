pub(crate) use crate as pallet_liquidation;
pub(crate) use crate::Event as LiquidationEvent;
use frame_support::{
	derive_impl, parameter_types,
	traits::{ConstU32, ConstU64},
	weights::{
		constants::ExtrinsicBaseWeight, WeightToFeeCoefficient, WeightToFeeCoefficients,
		WeightToFeePolynomial,
	},
};
use frame_system as system;
use smallvec::smallvec;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	AccountId32, BuildStorage, Perbill,
};

use frame_system::pallet_prelude::BlockNumberFor;
use pallet_order::OrderGasCost;
use sp_consensus_aura::sr25519::AuthorityId as AuraId;
use sp_std::collections::btree_map::BTreeMap;

type Balance = u128;
type BlockNumber = u32;
type Block = frame_system::mocking::MockBlock<Test>;

pub const UNIT: Balance = 1_000_000_000_000_000_000;
const MILLIUNIT: Balance = 1_000_000_000;

frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		OrderPallet: pallet_order::{Pallet, Storage, Event<T>},
		Pot: pallet_pot::{Pallet, Call, Storage, Event<T>},
		Utility: pallet_utility::{Pallet, Call, Storage, Event},
		Liquidation: pallet_liquidation::{Pallet, Storage, Event<T>},
	}
);

//ALICE
const COLLATOR_BYTES: [u8; 32] = [
	212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
	76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
];
const SYSTEM_ACCOUNT_BYTES: [u8; 32] = [
	54, 99, 32, 239, 79, 115, 118, 121, 15, 239, 57, 41, 2, 255, 91, 189, 21, 193, 175, 83, 111,
	196, 75, 126, 82, 14, 205, 184, 6, 168, 148, 234,
];

const COLLATOR: AccountId32 = AccountId32::new(COLLATOR_BYTES);
const SYSTEM_ACCOUNT: AccountId32 = AccountId32::new(SYSTEM_ACCOUNT_BYTES);

/// The existential deposit. Set to 1/10 of the Connected Relay Chain.
pub const EXISTENTIAL_DEPOSIT: Balance = MILLIUNIT;

parameter_types! {
	pub const SystemRatio: Perbill = Perbill::from_percent(20); // 20% for system
	pub const TreasuryRatio: Perbill = Perbill::from_percent(33); // 33% for treasury
	pub const OperationRatio: Perbill = Perbill::from_percent(25); // 25% for maintenance
	pub const ProfitDistributionCycle: BlockNumber = 10;
	pub const ExistDeposit: Balance = EXISTENTIAL_DEPOSIT;
	pub const SystemAccountName: &'static str = "system";
	pub const TreasuryAccountName: &'static str = "treasury";
	pub const OperationAccountName: &'static str = "maintenance";
}

#[derive_impl(frame_system::config_preludes::ParaChainDefaultConfig as frame_system::DefaultConfig)]
impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Nonce = u64;
	type Block = Block;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = AccountId32;
	type Lookup = IdentityLookup<Self::AccountId>;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<5>;
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type RuntimeOrigin = RuntimeOrigin;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = EXISTENTIAL_DEPOSIT;
}

impl pallet_balances::Config for Test {
	type Balance = u128;
	type DustRemoval = ();
	type RuntimeEvent = RuntimeEvent;
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type RuntimeHoldReason = ();
	type FreezeIdentifier = ();
	type RuntimeFreezeReason = ();
	type MaxFreezes = ();
}

impl pallet_utility::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RuntimeCall = RuntimeCall;
	type PalletsOrigin = OriginCaller;
	type WeightInfo = ();
}

parameter_types! {
	pub const SlotWidth: u32 = 2;
	pub const OrderMaxAmount:Balance = 200000000;
	pub const TxPoolThreshold:Balance = 3000000000;
}

impl pallet_order::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AuthorityId = AuraId;
	type Currency = Balances;
	type UpdateOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type OrderMaxAmount = OrderMaxAmount;
	type SlotWidth = SlotWidth;
	type TxPoolThreshold = TxPoolThreshold;
	type WeightInfo = ();
}

use pallet_pot::PotNameBtreemap;
pub type AccountId = AccountId32;
parameter_types! {
	pub const PotNames: [&'static str;3] = ["system", "treasury", "maintenance"];
	pub Pots: BTreeMap<String, AccountId> = pallet_pot
											::HashedPotNameBtreemap
											::<Test, pallet_pot::HashedPotNameMapping<BlakeTwo256>>
											::pots_btreemap(&(PotNames::get()));
}
impl pallet_pot::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type PotNameMapping = pallet_pot::HashedPotNameMapping<BlakeTwo256>;
	type Currency = Balances;
	type Pots = Pots;
}

impl pallet_liquidation::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type WeightToFee = WeightToFee;
	type OrderGasCost = MockOrderGasCostHandler;
	type SystemRatio = SystemRatio;
	type TreasuryRatio = TreasuryRatio;
	type OperationRatio = OperationRatio;
	type ExistentialDeposit = ExistDeposit;
	type SystemAccountName = SystemAccountName;
	type TreasuryAccountName = TreasuryAccountName;
	type OperationAccountName = OperationAccountName;
	type ProfitDistributionCycle = ProfitDistributionCycle;
}

pub struct WeightToFee;
impl WeightToFeePolynomial for WeightToFee {
	type Balance = Balance;
	fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
		let p = MILLIUNIT / 10;
		let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
		smallvec![WeightToFeeCoefficient {
			degree: 1,
			negative: false,
			coeff_frac: Perbill::from_rational(p % q, q),
			coeff_integer: p / q,
		}]
	}
}

pub struct MockOrderGasCostHandler;
impl<T: pallet_liquidation::Config> OrderGasCost<T> for MockOrderGasCostHandler
where
	T: pallet_order::Config,
	T::AccountId: From<[u8; 32]>,
{
	fn gas_cost(_block_number: BlockNumberFor<T>) -> Option<(T::AccountId, Balance)> {
		let account = T::AccountId::try_from(COLLATOR_BYTES).unwrap();
		Some((account, 10000000 as u128))
	}
}

pub struct ExtBuilder {
	existential_deposit: u128,
}

impl Default for ExtBuilder {
	fn default() -> Self {
		Self { existential_deposit: 1 }
	}
}

impl ExtBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u128) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}

	pub fn build(self) -> sp_io::TestExternalities {
		let mut storage = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();

		let balances_config = pallet_balances::GenesisConfig::<Test> {
			balances: vec![(COLLATOR, UNIT), (SYSTEM_ACCOUNT, UNIT)],
		};
		balances_config.assimilate_storage(&mut storage).unwrap();

		let mut ext = sp_io::TestExternalities::new(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}

pub(crate) fn expected_event(event: &LiquidationEvent<Test>) -> bool {
	matches!(
		event,
		LiquidationEvent::ProfitDistributed(_, _) | LiquidationEvent::CollatorsCompensated(_, _)
	)
}
