#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

use frame_support::{
	storage::types::StorageMap,
	traits::{Currency, Get},
	weights::{Weight, WeightToFee, WeightToFeePolynomial},
	Twox64Concat,
};
use frame_system::pallet_prelude::BlockNumberFor;
pub use pallet::*;
use sp_runtime::{traits::Zero, Perbill, Saturating};

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
	pub trait Config: frame_system::Config + pallet_order::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		///handle transfer
		type Currency: frame_support::traits::Currency<Self::AccountId>;

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

		/// system accountId
		#[pallet::constant]
		type SystemAccount: Get<Self::AccountId>;

		/// treasury accountId
		#[pallet::constant]
		type TreasuryAccount: Get<Self::AccountId>;

		/// operation accountId
		#[pallet::constant]
		type OperationAccount: Get<Self::AccountId>;

		///how many blocks to distribute a profit distribution
		#[pallet::constant]
		type ProfitDistributionCycle: Get<BlockNumberFor<Self>>;
	}

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
		/// parameters. [blockNumber, weight, weightBalance, costBalance, collator]
		/// block has been handled include weight and fee
		BlockProcessed(BlockNumberFor<T>, Weight, Balance, Balance, T::AccountId),

		/// parameters. [accountId, balance]
		///mint native coin on system account
		DepositSystemAccount(T::AccountId, BalanceOf<T>),

		/// parameters. [accountId, balance]
		///mint native coin on treasury account
		DepositTreasuryAccount(T::AccountId, BalanceOf<T>),

		/// parameters. [accountId, balance]
		///mint native coin on operation account
		DepositOperaionAccount(T::AccountId, BalanceOf<T>),

		/// parameters. [accountId, balance]
		///mint native coin on collator account for profit
		DepositCollatorProfit(T::AccountId, BalanceOf<T>),

		/// parameters. [accountId, balance]
		///mint native coin on collator account for compensate
		DepositCollatorCompensate(T::AccountId, BalanceOf<T>),

		/// profit distributed succeed
		ProfitDistributed,

		/// collators compensated
		CollatorsCompensated,

		///Deducts up to value from the combined balance of who
		Slash(T::AccountId, BalanceOf<T>),

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
	}

	fn weight_to_fee<T: Config>(weight: Weight) -> Balance {
		T::WeightToFee::weight_to_fee(&weight)
	}

	fn convert_u128_to_balance<T: Config>(value: u128) -> Option<BalanceOf<T>> {
		value.try_into().ok()
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(n: BlockNumberFor<T>) {
			let block_weight = <frame_system::Pallet<T>>::block_weight().total();
			let balance_weight = weight_to_fee::<T>(block_weight);

			let (collator, real_gas_cost) = match T::OrderGasCost::gas_cost(n) {
				Some((collator, real_gas_cost)) => (collator, real_gas_cost),
				None => {
					Self::deposit_event(Event::Error(Error::<T>::FailedToFetchRealGasCost.into()));
					return;
				},
			};

			frame_support::runtime_print!(
				"\n=== block: {:?}, blockFee: {:?}, collator: {:?}, realGasCost: {:?} ====\n",
				&n,
				&balance_weight,
				&collator,
				&real_gas_cost
			);
			CollatorRealGasCosts::<T>::mutate(collator.clone(), |cost| {
				*cost = cost.saturating_add(real_gas_cost);
			});

			TotalIncome::<T>::mutate(|income| *income = income.saturating_add(balance_weight));
			TotalCost::<T>::mutate(|cost| *cost = cost.saturating_add(real_gas_cost));

			let mut count = DistributionBlockCount::<T>::get();
			count = count.saturating_add(1u32.into());
			DistributionBlockCount::<T>::put(count);
			if count % T::ProfitDistributionCycle::get() == Zero::zero() {
				// reset cycle count
				DistributionBlockCount::<T>::put(BlockNumberFor::<T>::zero());
				let _ = Self::distribute_profit();
			}

			Self::deposit_event(Event::BlockProcessed(
				n,
				block_weight,
				balance_weight,
				real_gas_cost,
				collator,
			));
		}
	}

	impl<T: Config> Pallet<T> {
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

			let system_ratio = T::SystemRatio::get();
			let treasury_ratio = T::TreasuryRatio::get();
			let operation_ratio = T::OperationRatio::get();

			let treasury_amount = treasury_ratio * total_profit;
			let operation_amount = operation_ratio * total_profit;
			let system_amount = system_ratio * total_profit;
			let total_collators_profit =
				total_profit.saturating_sub(treasury_amount + operation_amount + system_amount);

			if let Some(system_account_balance) = convert_u128_to_balance::<T>(system_amount) {
				<T as pallet::Config>::Currency::deposit_creating(
					&T::SystemAccount::get(),
					system_account_balance,
				);
				Self::deposit_event(Event::DepositSystemAccount(
					T::SystemAccount::get(),
					system_account_balance,
				));
			}
			if let Some(treasury_account_balance) = convert_u128_to_balance::<T>(treasury_amount) {
				<T as pallet::Config>::Currency::deposit_creating(
					&T::TreasuryAccount::get(),
					treasury_account_balance,
				);

				Self::deposit_event(Event::DepositTreasuryAccount(
					T::TreasuryAccount::get(),
					treasury_account_balance,
				));
			}
			if let Some(operation_account_balance) = convert_u128_to_balance::<T>(operation_amount)
			{
				<T as pallet::Config>::Currency::deposit_creating(
					&T::OperationAccount::get(),
					operation_account_balance,
				);

				Self::deposit_event(Event::DepositOperaionAccount(
					T::OperationAccount::get(),
					operation_account_balance,
				));
			}

			// distribute profit and compensate cost to every collator
			for (collator, collator_cost) in CollatorRealGasCosts::<T>::iter() {
				let collator_ratio = Perbill::from_rational(collator_cost, total_cost);
				let collator_profit = collator_ratio * total_collators_profit;
				if let Some(collator_addr_profit) = convert_u128_to_balance::<T>(collator_profit) {
					<T as pallet::Config>::Currency::deposit_creating(
						&collator.clone(),
						collator_addr_profit,
					);
					Self::deposit_event(Event::DepositCollatorProfit(
						collator.clone(),
						collator_addr_profit,
					));
				}
				if let Some(collator_addr_cost) = convert_u128_to_balance::<T>(collator_cost) {
					<T as pallet::Config>::Currency::deposit_creating(
						&collator.clone(),
						collator_addr_cost,
					);
					Self::deposit_event(Event::DepositCollatorCompensate(
						collator.clone(),
						collator_addr_cost,
					));
				}
			}

			Ok(())
		}

		fn compensate_collators() -> DispatchResult {
			let total_income = TotalIncome::<T>::get();
			let total_cost = TotalCost::<T>::get();

			if total_cost > total_income {
				let diff = total_cost.saturating_sub(total_income);
				if let Some(collator_compensate) = convert_u128_to_balance::<T>(diff) {
					<T as pallet::Config>::Currency::slash(
						&T::SystemAccount::get(),
						collator_compensate,
					);
					Self::deposit_event(Event::Slash(T::SystemAccount::get(), collator_compensate));
				}
			}
			frame_support::runtime_print!(
				"compensate collators: totalIncome:{:?}, totalCost:{:?}\n",
				&total_income,
				&total_cost
			);

			// compensate for every collator
			for (collator, collator_cost) in CollatorRealGasCosts::<T>::iter() {
				if let Some(collator_addr_cost) = convert_u128_to_balance::<T>(collator_cost) {
					frame_support::runtime_print!(
						"\n+-+-+-+ compensate collators-->> collator:{:?}, cost:{:?} +-+-+-+\n",
						&collator,
						&collator_cost
					);
					<T as pallet::Config>::Currency::deposit_creating(
						&collator,
						collator_addr_cost,
					);
					Self::deposit_event(Event::DepositCollatorCompensate(
						collator,
						collator_addr_cost,
					));
				}
			}

			Ok(())
		}
	}
}
