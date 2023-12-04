#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use frame_support::{
	storage::types::StorageMap,
	traits::{Currency, ExistenceRequirement, Get},
	weights::WeightToFeePolynomial,
	Twox64Concat,
};
use frame_system::pallet_prelude::BlockNumberFor;
use mp_system::BASE_ACCOUNT;
pub use pallet::*;
use sp_runtime::{traits::Zero, AccountId32, Perbill, Saturating};

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type Balance = u128;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use pallet_order::OrderGasCost;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_order::Config + pallet_pot::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		///handle transfer
		type Currency: frame_support::traits::Currency<Self::AccountId>
			+ frame_support::traits::ReservableCurrency<Self::AccountId>;

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

		/// parameters. [accountId, balance]
		///transfer profit from BaseAccount to SystemAccount
		SystemAccountProfit(T::AccountId, BalanceOf<T>),

		/// parameters. [accountId, balance]
		///transfer total fee from BaseAccount to SystemAccount
		TransferBaseToSystem(T::AccountId, T::AccountId, BalanceOf<T>),

		/// parameters. [accountId, balance]
		///transfer profit from BaseAccount to TreasuryAccount
		TreasuryAccountProfit(T::AccountId, BalanceOf<T>),

		/// parameters. [accountId, balance]
		///transfer profit from BaseAccount to operationAccount
		OperationAccountProfit(T::AccountId, BalanceOf<T>),

		/// parameters. [accountId, balance]
		///transfer profit from BaseAccount to collators account
		CollatorProfit(T::AccountId, BalanceOf<T>),

		/// parameters. [accountId, balance]
		///transfer principal from BaseAccount to system account if deficit
		CollatorPrincipal(T::AccountId, BalanceOf<T>),

		/// parameters. [accountId, balance]
		///transfer from SystemAccount to collators account for compensate
		CollatorCompensate(T::AccountId, BalanceOf<T>),

		/// profit distributed succeed
		ProfitDistributed,

		/// collators compensated
		CollatorsCompensated,

		/// error occurred
		Error(Error<T>),
	}

	#[pallet::error]
	#[derive(Clone, PartialEq, Eq)]
	pub enum Error<T> {
		/// get real gas cost failed
		FailedToFetchRealGasCost,

		/// internal errors
		InternalError,

		///get pot account errors
		PotAccountError,

		///failed to process liquidation
		ProcessLiquidationError,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
	where
		T::AccountId: From<AccountId32>,
	{
		fn on_finalize(n: BlockNumberFor<T>) {
			let base_account = <T::AccountId>::from(BASE_ACCOUNT);
			let base_account_balance = <T as pallet::Config>::Currency::free_balance(&base_account);

			let (collator, real_gas_cost) = match T::OrderGasCost::gas_cost(n) {
				Some((collator, real_gas_cost)) => (collator, real_gas_cost),
				None => {
					Self::deposit_event(Event::Error(Error::<T>::FailedToFetchRealGasCost.into()));
					return;
				},
			};

			CollatorRealGasCosts::<T>::mutate(collator.clone(), |cost| {
				*cost = cost.saturating_add(real_gas_cost);
			});

			let reserved_balance: BalanceOf<T> =
				T::ExistentialDeposit::get().try_into().unwrap_or_else(|_| Zero::zero());

			let block_fee_except_ed = base_account_balance.saturating_sub(reserved_balance);
			let current_block_fee_u128: Balance =
				block_fee_except_ed.try_into().unwrap_or_else(|_| 0);

			let base_account = <T::AccountId>::from(BASE_ACCOUNT);
			let system_account =
				match pallet_pot::Pallet::<T>::ensure_pot(T::SystemAccountName::get()) {
					Ok(account) => account,
					Err(err) => {
						log::error!("get system account err:{:?}", err);
						Self::deposit_event(Event::Error(Error::<T>::InternalError.into()));
						return;
					},
				};

			match Self::transfer_funds(&base_account, &system_account, block_fee_except_ed.clone())
			{
				Ok(_) => {
					Self::deposit_event(Event::TransferBaseToSystem(
						base_account.clone(),
						system_account.clone(),
						block_fee_except_ed.clone(),
					));
				},
				Err(err) => {
					log::error!("Transfer to system account failed: {:?}", err);
					Self::deposit_event(Event::Error(Error::<T>::InternalError.into()));
					return;
				},
			}

			TotalIncome::<T>::mutate(|income| {
				*income = income.saturating_add(current_block_fee_u128)
			});
			TotalCost::<T>::mutate(|cost| *cost = cost.saturating_add(real_gas_cost));

			let mut count = DistributionBlockCount::<T>::get();
			count = count.saturating_add(1u32.into());
			DistributionBlockCount::<T>::put(count);
			if count % T::ProfitDistributionCycle::get() == Zero::zero() {
				DistributionBlockCount::<T>::put(BlockNumberFor::<T>::zero());
				match Self::distribute_profit() {
					Ok(_) => {
						Self::deposit_event(Event::BlockProcessed(
							n,
							block_fee_except_ed.clone(),
							real_gas_cost,
							collator,
						));
					},
					Err(err) => {
						log::error!("process liquidation failed: {:?}", err);
						Self::deposit_event(Event::Error(
							Error::<T>::ProcessLiquidationError.into(),
						));
					},
				}
			}
		}
	}

	impl<T: Config> Pallet<T>
	where
		T::AccountId: From<AccountId32>,
	{
		fn transfer_funds(
			source: &T::AccountId,
			to: &T::AccountId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			ensure!(
				<T as pallet::Config>::Currency::free_balance(source) >= amount,
				"Not enough balance"
			);
			<T as pallet::Config>::Currency::transfer(
				source,
				to,
				amount,
				ExistenceRequirement::KeepAlive,
			)?;

			Ok(())
		}

		fn distribute_profit() -> DispatchResult {
			let total_income = TotalIncome::<T>::get();
			let total_cost = TotalCost::<T>::get();

			if total_income > total_cost {
				Self::distribute_positive_profit()?;
				Self::deposit_event(Event::ProfitDistributed);
			} else {
				Self::compensate_collators()?;
				Self::deposit_event(Event::CollatorsCompensated);
			}

			let _ = <CollatorRealGasCosts<T>>::clear(u32::max_value(), None);
			TotalIncome::<T>::put(0u128);
			TotalCost::<T>::put(0u128);

			Ok(())
		}

		#[cfg(test)]
		pub fn test_distribute_profit() -> DispatchResult {
			Self::distribute_profit()
		}

		fn distribute_positive_profit() -> DispatchResult {
			let total_income = TotalIncome::<T>::get();
			let total_cost = TotalCost::<T>::get();
			let total_profit = total_income.saturating_sub(total_cost);

			let system_account =
				match pallet_pot::Pallet::<T>::ensure_pot(T::SystemAccountName::get()) {
					Ok(account) => account,
					Err(err) => {
						log::error!("get system account err:{:?}", err);
						Err(Error::<T>::PotAccountError)?
					},
				};
			let treasury_account =
				match pallet_pot::Pallet::<T>::ensure_pot(T::TreasuryAccountName::get()) {
					Ok(account) => account,
					Err(err) => {
						log::error!("get treasury account err:{:?}", err);
						Err(Error::<T>::PotAccountError)?
					},
				};
			let operation_account =
				match pallet_pot::Pallet::<T>::ensure_pot(T::OperationAccountName::get()) {
					Ok(account) => account,
					Err(err) => {
						log::error!("get maintenance account err:{:?}", err);
						Err(Error::<T>::PotAccountError)?
					},
				};

			let system_ratio = T::SystemRatio::get();
			let treasury_ratio = T::TreasuryRatio::get();
			let operation_ratio = T::OperationRatio::get();

			let treasury_amount = treasury_ratio * total_profit;
			let operation_amount = operation_ratio * total_profit;
			let system_amount = system_ratio * total_profit;
			let total_collators_profit =
				total_profit.saturating_sub(treasury_amount + operation_amount + system_amount);

			let treasury_account_profit =
				treasury_amount.try_into().unwrap_or_else(|_| Zero::zero());
			match Self::transfer_funds(&system_account, &treasury_account, treasury_account_profit)
			{
				Ok(_) => {
					Self::deposit_event(Event::TreasuryAccountProfit(
						treasury_account.clone(),
						treasury_account_profit,
					));
				},
				Err(err) => {
					log::error!("Transfer to treasury account failed: {:?}", err);
				},
			}

			let operation_account_profit =
				operation_amount.try_into().unwrap_or_else(|_| Zero::zero());
			match Self::transfer_funds(
				&system_account,
				&operation_account,
				operation_account_profit,
			) {
				Ok(_) => {
					Self::deposit_event(Event::OperationAccountProfit(
						operation_account.clone(),
						operation_account_profit,
					));
				},
				Err(err) => {
					log::error!("Transfer to maintenance account failed: {:?}", err);
				},
			}

			// distribute profit and compensate cost to every collator
			for (collator, collator_cost) in CollatorRealGasCosts::<T>::iter() {
				let collator_ratio = Perbill::from_rational(collator_cost, total_cost);
				let collator_profit = collator_ratio * total_collators_profit;

				let collator_addr_profit =
					collator_profit.try_into().unwrap_or_else(|_| Zero::zero());
				match Self::transfer_funds(&system_account, &collator.clone(), collator_addr_profit)
				{
					Ok(_) => {
						Self::deposit_event(Event::CollatorProfit(
							collator.clone(),
							collator_addr_profit,
						));
					},
					Err(err) => {
						log::error!("Transfer profit to collator account failed: {:?}", err);
					},
				}

				let collator_addr_cost = collator_cost.try_into().unwrap_or_else(|_| Zero::zero());
				match Self::transfer_funds(&system_account, &collator.clone(), collator_addr_cost) {
					Ok(_) => {
						Self::deposit_event(Event::CollatorCompensate(
							collator.clone(),
							collator_addr_cost,
						));
					},
					Err(err) => {
						log::error!("Transfer principal to collator account failed: {:?}", err);
					},
				}
			}

			Ok(())
		}

		fn compensate_collators() -> DispatchResult {
			let system_account =
				match pallet_pot::Pallet::<T>::ensure_pot(T::SystemAccountName::get()) {
					Ok(account) => account,
					Err(err) => {
						log::error!("get system account err:{:?}", err);
						Err(Error::<T>::PotAccountError)?
					},
				};

			// compensate for every collator
			for (collator, collator_cost) in CollatorRealGasCosts::<T>::iter() {
				let collator_addr_cost = collator_cost.try_into().unwrap_or_else(|_| Zero::zero());
				match Self::transfer_funds(&system_account, &collator.clone(), collator_addr_cost) {
					Ok(_) => {
						Self::deposit_event(Event::CollatorCompensate(
							collator,
							collator_addr_cost,
						));
					},
					Err(err) => {
						log::error!("Transfer principal to collator account failed: {:?}", err);
					},
				}
			}

			Ok(())
		}
	}
}
