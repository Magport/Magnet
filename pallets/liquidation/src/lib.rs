#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use frame_support::{
	dispatch::DispatchResult,
	storage::types::StorageMap,
	traits::{Currency, ExistenceRequirement, Get},
	weights::WeightToFeePolynomial,
	Twox64Concat,
};
use frame_system::pallet_prelude::{BlockNumberFor, OriginFor};
use mp_system::BASE_ACCOUNT;
pub use pallet::*;
use sp_runtime::{
	traits::{StaticLookup, Zero},
	AccountId32, Perbill, Saturating,
};
use sp_std::{prelude::*, sync::Arc, vec};

use xcm::{
	opaque::v4::Junctions::X1,
	prelude::*,
	v4::{Asset, AssetId, Junction, Location, NetworkId},
	VersionedAssets, VersionedLocation,
};

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type Balance = u128;

pub const PARACHAIN_TO_RELAYCHAIN_UNIT: u128 = 1_000_000;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use pallet_balances;
	// use pallet_order::OrderGasCost;
	use pallet_utility;
	pub trait OrderGasCost<T: frame_system::Config> {
		/// Gas consumed by placing an order in a certain block.
		///
		/// Parameters:
		/// - `block_number`: The block number of para chain.
		fn gas_cost(
			block_number: BlockNumberFor<T>,
		) -> Result<Option<(T::AccountId, Balance)>, DispatchError>;
	}

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		// + pallet_order::Config
		+ pallet_pot::Config
		+ pallet_balances::Config
		+ pallet_utility::Config
	{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		///handle transfer
		type Currency: frame_support::traits::Currency<Self::AccountId>
			+ frame_support::traits::ReservableCurrency<Self::AccountId>;

		/// The XCM sender
		type XcmSender: SendXcm;

		//Treasury account on the Relay Chain
		//#[pallet::constant]
		//type RelayChainTreasuryAccountId: Get<AccountId32>;

		///Handles converting weight to fee value
		type WeightToFee: WeightToFeePolynomial<Balance = Balance>;

		///get real weight cost from coreTime placeOrder pallet
		type OrderGasCost: OrderGasCost<Self>;

		///profit distribute ratio to treasury account
		#[pallet::constant]
		type SystemRatio: Get<Perbill>;

		///profit distribute ratio to treasury account
		#[pallet::constant]
		type TreasuryRatio: Get<Perbill>;

		/// profit distribute ratio to operation account
		#[pallet::constant]
		type OperationRatio: Get<Perbill>;

		/// ED necessitate the account to exist
		#[pallet::constant]
		type ExistentialDeposit: Get<Balance>;

		///minimum liquidation threshold
		#[pallet::constant]
		type MinLiquidationThreshold: Get<Balance>;

		/// system accountId
		#[pallet::constant]
		type SystemAccountName: Get<&'static str>;

		/// treasury accountId
		#[pallet::constant]
		type TreasuryAccountName: Get<&'static str>;

		/// operation accountId
		#[pallet::constant]
		type OperationAccountName: Get<&'static str>;

		///how many blocks to distribute a profit distribution
		#[pallet::constant]
		type ProfitDistributionCycle: Get<BlockNumberFor<Self>>;
	}

	#[pallet::storage]
	#[pallet::getter(fn base_account_balance)]
	pub type BaseAccountReserved<T: Config> = StorageValue<_, Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn collator_real_gas_costs)]
	pub type CollatorRealGasCosts<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_income)]
	pub type TotalIncome<T: Config> = StorageValue<_, Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn total_cost)]
	pub type TotalCost<T: Config> = StorageValue<_, Balance, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn block_count)]
	pub type DistributionBlockCount<T: Config> = StorageValue<_, BlockNumberFor<T>, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// parameters. [blockNumber, blockFee, orderCost, collator]
		/// block has been handled include weight and fee
		BlockProcessed(BlockNumberFor<T>, BalanceOf<T>, Balance, T::AccountId),

		/// liquidation and distribution finished
		DistributionFinished,

		/// parameters. [baseAccount, systemAccount, balance]
		///transfer total block fee from BaseAccount to SystemAccount
		TransferBaseToSystem(T::AccountId, T::AccountId, BalanceOf<T>),

		///parameters. [totalFee, totalCost]
		/// profit distributed succeed
		ProfitDistributed(Balance, Balance),

		///parameters. [totalFee, totalCost]
		/// collators compensated
		CollatorsCompensated(Balance, Balance),

		/// error occurred
		Error(Error<T>),
	}

	#[pallet::error]
	#[derive(Clone, PartialEq, Eq)]
	pub enum Error<T> {
		/// get real gas cost failed
		FailedToFetchRealGasCost,

		/// internal errors
		TransferBaseToSystemError,

		///get pot account errors
		GetPotAccountError,

		///failed to process liquidation
		ProcessLiquidationError,

		///xcm error
		XcmError,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		T::AccountId: From<AccountId32> + Into<AccountId32>,
		<T as pallet_utility::Config>::RuntimeCall: From<pallet_balances::Call<T>>,
		<T as pallet_balances::Config>::Balance: From<BalanceOf<T>>,
		T: pallet_xcm::Config,
	{
		fn on_finalize(n: BlockNumberFor<T>) {
			let base_account = <T::AccountId>::from(BASE_ACCOUNT);
			let base_account_balance = <T as pallet::Config>::Currency::free_balance(&base_account);

			let (collator, real_gas_cost) = match T::OrderGasCost::gas_cost(n) {
				Ok(cost) => match cost {
					Some((collator, real_gas_cost)) => (collator, real_gas_cost),
					None => return,
				},
				Err(_) => {
					Self::deposit_event(Event::Error(Error::<T>::FailedToFetchRealGasCost.into()));
					return;
				},
			};

			let reserved_balance: BalanceOf<T> = <T as pallet::Config>::ExistentialDeposit::get()
				.try_into()
				.unwrap_or_else(|_| Zero::zero());

			let block_fee_except_ed = base_account_balance.saturating_sub(reserved_balance);
			let current_block_fee_u128: Balance =
				block_fee_except_ed.try_into().unwrap_or_else(|_| 0);

			TotalCost::<T>::mutate(|cost| *cost = cost.saturating_add(real_gas_cost));
			CollatorRealGasCosts::<T>::mutate(collator.clone(), |cost| {
				*cost = cost.saturating_add(real_gas_cost);
			});

			let mut count = DistributionBlockCount::<T>::get();
			count = count.saturating_add(1u32.into());
			DistributionBlockCount::<T>::put(count);

			let system_account =
				match pallet_pot::Pallet::<T>::ensure_pot(T::SystemAccountName::get()) {
					Ok(account) => account,
					Err(err) => {
						log::error!("get system account err:{:?}", err);
						Self::deposit_event(Event::Error(Error::<T>::GetPotAccountError.into()));
						return;
					},
				};

			match <T as pallet::Config>::Currency::transfer(
				&base_account,
				&system_account,
				block_fee_except_ed.clone(),
				ExistenceRequirement::KeepAlive,
			) {
				Ok(_) => {
					Self::deposit_event(Event::TransferBaseToSystem(
						base_account.clone(),
						system_account.clone(),
						block_fee_except_ed.clone(),
					));
				},
				Err(err) => {
					log::error!("Transfer to system account failed: {:?}", err);
					Self::deposit_event(Event::Error(Error::<T>::TransferBaseToSystemError.into()));
					return;
				},
			}

			TotalIncome::<T>::mutate(|income| {
				*income = income.saturating_add(current_block_fee_u128)
			});

			let min_liquidation_threshold: Balance =
				<T as pallet::Config>::MinLiquidationThreshold::get()
					.try_into()
					.unwrap_or_else(|_| 0);
			let profit = TotalIncome::<T>::get().saturating_sub(TotalCost::<T>::get());

			if profit >= min_liquidation_threshold
				&& count % T::ProfitDistributionCycle::get() == Zero::zero()
			{
				DistributionBlockCount::<T>::put(BlockNumberFor::<T>::zero());
				match Self::distribute_profit() {
					Ok(_) => {
						Self::deposit_event(Event::DistributionFinished);
					},
					Err(err) => {
						log::error!("process liquidation failed: {:?}", err);
						Self::deposit_event(Event::Error(
							Error::<T>::ProcessLiquidationError.into(),
						));
					},
				}
			}

			Self::deposit_event(Event::BlockProcessed(
				n,
				block_fee_except_ed.clone(),
				real_gas_cost,
				collator,
			));
		}
	}

	impl<T: Config> Pallet<T>
	where
		T::AccountId: From<AccountId32> + Into<AccountId32>,
		<T as pallet_utility::Config>::RuntimeCall: From<pallet_balances::Call<T>>,
		<T as pallet_balances::Config>::Balance: From<BalanceOf<T>>,
		T::XcmSender: SendXcm,
		T: pallet_xcm::Config,
	{
		fn execute_batch_transfers(
			transfers: Vec<(T::AccountId, BalanceOf<T>)>,
		) -> DispatchResultWithPostInfo {
			let system_account =
				match pallet_pot::Pallet::<T>::ensure_pot(T::SystemAccountName::get()) {
					Ok(account) => account,
					Err(err) => {
						log::error!("get system account err:{:?}", err);
						Err(Error::<T>::GetPotAccountError)?
					},
				};

			let mut calls: Vec<<T as pallet_utility::Config>::RuntimeCall> = vec![];

			for (recipient, amount) in transfers {
				if amount.is_zero() {
					continue;
				}

				let transfer_call = pallet_balances::Call::<T>::transfer_allow_death {
					dest: T::Lookup::unlookup(recipient),
					value: amount.into(),
				};

				let utility_call: <T as pallet_utility::Config>::RuntimeCall = transfer_call.into();
				calls.push(utility_call);
			}

			pallet_utility::Pallet::<T>::batch_all(
				frame_system::RawOrigin::Signed(system_account).into(),
				calls,
			)
			.map_err(|err| {
				log::error!("Batch transfer failed: {:?}", err);
				err
			})?;

			Ok(().into())
		}

		fn distribute_profit() -> DispatchResultWithPostInfo {
			let total_income = TotalIncome::<T>::get();
			let total_cost = TotalCost::<T>::get();

			if total_income > total_cost {
				Self::distribute_positive_profit()?;
				Self::deposit_event(Event::ProfitDistributed(
					total_income.clone(),
					total_cost.clone(),
				));
			} else {
				Self::compensate_collators()?;
				Self::deposit_event(Event::CollatorsCompensated(
					total_income.clone(),
					total_cost.clone(),
				));
			}

			let _ = <CollatorRealGasCosts<T>>::clear(u32::max_value(), None);
			TotalIncome::<T>::put(0u128);
			TotalCost::<T>::put(0u128);

			Ok(().into())
		}

		#[cfg(test)]
		pub fn test_distribute_profit() -> DispatchResultWithPostInfo {
			Self::distribute_profit()
		}

		fn distribute_positive_profit() -> DispatchResultWithPostInfo {
			let total_income = TotalIncome::<T>::get();
			let total_cost = TotalCost::<T>::get();
			let total_profit = total_income.saturating_sub(total_cost);

			let treasury_account = pallet_pot::Pallet::<T>::ensure_pot(
				T::TreasuryAccountName::get(),
			)
			.map_err(|err| {
				log::error!("get treasury account err:{:?}", err);
				Error::<T>::GetPotAccountError
			})?;

			let operation_account = pallet_pot::Pallet::<T>::ensure_pot(
				T::OperationAccountName::get(),
			)
			.map_err(|err| {
				log::error!("get maintenance account err:{:?}", err);
				Error::<T>::GetPotAccountError
			})?;

			let system_ratio = T::SystemRatio::get();
			let treasury_ratio = T::TreasuryRatio::get();
			let operation_ratio = T::OperationRatio::get();

			let treasury_amount = treasury_ratio * total_profit / PARACHAIN_TO_RELAYCHAIN_UNIT;
			let operation_amount = operation_ratio * total_profit;
			let system_amount = system_ratio * total_profit;
			let total_collators_profit =
				total_profit.saturating_sub(treasury_amount + operation_amount + system_amount);

			let origin: OriginFor<T> =
				frame_system::RawOrigin::Signed(treasury_account.clone()).into();

			let _send_treasury_profit = Self::send_assets_to_relaychain_treasury(
				origin,
				treasury_account.into(),
				treasury_amount,
			)?;

			let mut transfers = Vec::new();
			/*
			let treasury_account_profit =
				treasury_amount.try_into().unwrap_or_else(|_| Zero::zero());
			transfers.push((treasury_account, treasury_account_profit));
			*/

			let operation_account_profit =
				operation_amount.try_into().unwrap_or_else(|_| Zero::zero());
			transfers.push((operation_account, operation_account_profit));

			for (collator, collator_cost) in CollatorRealGasCosts::<T>::iter() {
				let collator_ratio = Perbill::from_rational(collator_cost, total_cost);
				let collator_profit = collator_ratio * total_collators_profit;

				let collator_addr_profit =
					collator_profit.try_into().unwrap_or_else(|_| Zero::zero());
				let collator_addr_cost = collator_cost.try_into().unwrap_or_else(|_| Zero::zero());

				transfers.push((collator.clone(), collator_addr_profit));
				transfers.push((collator.clone(), collator_addr_cost));
			}

			Self::execute_batch_transfers(transfers)
		}

		fn compensate_collators() -> DispatchResultWithPostInfo {
			let mut transfers = Vec::new();

			for (collator, collator_cost) in CollatorRealGasCosts::<T>::iter() {
				let collator_addr_cost = collator_cost.try_into().unwrap_or_else(|_| Zero::zero());

				transfers.push((collator.clone(), collator_addr_cost));
			}
			Self::execute_batch_transfers(transfers)
		}

		fn send_assets_to_relaychain_treasury(
			origin: OriginFor<T>,
			recipient: AccountId32,
			amount: u128,
		) -> DispatchResult {
			let recipient_account_id = recipient.into();

			let junction = Junction::AccountId32 {
				id: recipient_account_id,
				network: Some(NetworkId::Rococo),
			};
			let arc_junctions = Arc::new([junction]);

			let beneficiary = Location::new(0, X1(arc_junctions));

			let asset =
				Asset { id: AssetId(Location::new(1, Here)), fun: Fungibility::Fungible(amount) };

			let assets = Assets::from(vec![asset]);
			let versioned_assets = VersionedAssets::from(assets);

			match pallet_xcm::Pallet::<T>::reserve_transfer_assets(
				origin,
				Box::new(VersionedLocation::from(Location::parent())),
				Box::new(VersionedLocation::from(beneficiary)),
				Box::new(versioned_assets),
				0,
			) {
				Ok(_) => {
					frame_support::runtime_print!("reserve_transfer_assets executed successfully.");
				},
				Err(e) => {
					log::error!("Error occurred while executing reserve_transfer_assets: {:?}", e);
					Self::deposit_event(Event::Error(Error::<T>::XcmError.into()));
					return Err(Error::<T>::XcmError.into());
				},
			}
			Ok(())
		}
	}
}

impl<T> OrderGasCost<T> for ()
where
	T: frame_system::Config,
	T::AccountId: From<[u8; 32]>,
{
	fn gas_cost(
		block_number: BlockNumberFor<T>,
	) -> Result<Option<(T::AccountId, Balance)>, sp_runtime::DispatchError> {
		Ok(None)
	}
}
