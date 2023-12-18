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

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

use frame_support::traits::{
	tokens::{fungible::Inspect, ExistenceRequirement},
	Currency, Get,
};
use scale_info::prelude::string::String;
use sp_core::{crypto::AccountId32, Hasher, H256};
use sp_std::collections::btree_map::BTreeMap;
use sp_std::vec::Vec;

pub use self::pallet::*;
use mp_system::BASE_ACCOUNT;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{dispatch::DispatchResultWithPostInfo, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Pot collect
		type Pots: Get<BTreeMap<String, Self::AccountId>>;
		/// Mapping from pot name to account id.
		type PotNameMapping: PotNameMapping<Self::AccountId>;
		/// Currency type for balance storage.
		type Currency: Currency<Self::AccountId> + Inspect<Self::AccountId>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Deposit(T::AccountId, String, BalanceOf<T>),
		Withdraw(T::AccountId, String, BalanceOf<T>),
		WithdrawBase(T::AccountId, BalanceOf<T>),
	}

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		DepositFailed,
		NotPot,
		TransferFailed,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T::AccountId: From<AccountId32>,
	{
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn deposit(
			origin: OriginFor<T>,
			pot_name: String,
			value: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;

			let name_account_id = Self::ensure_pot(&pot_name)?;

			T::Currency::transfer(&who, &name_account_id, value, ExistenceRequirement::AllowDeath)?;

			Self::deposit_event(Event::Deposit(who, pot_name, value));
			Ok(().into())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn withdraw(
			origin: OriginFor<T>,
			who: T::AccountId,
			pot_name: String,
			value: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			Self::withdraw_inner(&who, &pot_name, value)?;

			Self::deposit_event(Event::Withdraw(who, pot_name, value));
			Ok(().into())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn withdraw_base(
			origin: OriginFor<T>,
			who: T::AccountId,
			value: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;
			Self::withdraw_base_inner(&who, value)?;

			Self::deposit_event(Event::WithdrawBase(who, value));
			Ok(().into())
		}
	}

	impl<T: Config> Pallet<T>
	where
		T::AccountId: From<AccountId32>,
	{
		pub fn ensure_pot(name: &str) -> Result<T::AccountId, Error<T>> {
			match T::Pots::get().get(name) {
				Some(account) => Ok(account.clone()),
				None => Err(Error::<T>::NotPot),
			}
		}

		pub fn withdraw_inner(
			who: &T::AccountId,
			pot_name: &str,
			value: BalanceOf<T>,
		) -> Result<(), Error<T>> {
			let name_account_id = Self::ensure_pot(pot_name)?;

			let _ = T::Currency::transfer(
				&name_account_id,
				&who,
				value,
				ExistenceRequirement::AllowDeath,
			)
			.map_err(|_| Error::<T>::TransferFailed)?;

			Ok(())
		}

		pub fn withdraw_base_inner(
			who: &T::AccountId,
			value: BalanceOf<T>,
		) -> Result<(), Error<T>> {
			let _ = T::Currency::transfer(
				&<T::AccountId>::from(BASE_ACCOUNT),
				&who,
				value,
				ExistenceRequirement::AllowDeath,
			)
			.map_err(|_| Error::<T>::TransferFailed)?;

			Ok(())
		}

		pub fn balance_of(pot_name: &str) -> Result<BalanceOf<T>, Error<T>> {
			let name_account_id = Self::ensure_pot(pot_name)?;

			Ok(T::Currency::free_balance(&name_account_id))
		}

		pub fn balance_of_base() -> Result<BalanceOf<T>, Error<T>> {
			Ok(T::Currency::free_balance(&<T::AccountId>::from(BASE_ACCOUNT)))
		}
	}
}

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait PotNameMapping<A> {
	fn into_account_id(name: &str) -> A;
}

/// Hashed PotName mapping.
pub struct HashedPotNameMapping<H>(sp_std::marker::PhantomData<H>);

impl<H: Hasher<Out = H256>> PotNameMapping<AccountId32> for HashedPotNameMapping<H> {
	fn into_account_id(name: &str) -> AccountId32 {
		let mut data = String::from("syspot:");
		data += name;
		let hash = H::hash(data.as_bytes());

		AccountId32::from(Into::<[u8; 32]>::into(hash))
	}
}

pub trait PotNameBtreemap<C> {
	fn pots_btreemap(names: &[&str]) -> BTreeMap<String, C>;
}

use sp_std::marker::PhantomData;
/// Hashed PotName mapping Btree.
pub struct HashedPotNameBtreemap<C, P>(PhantomData<C>, PhantomData<P>);

impl<C, P> PotNameBtreemap<C::AccountId> for HashedPotNameBtreemap<C, P>
where
	C: frame_system::Config,
	C::AccountId: From<AccountId32>,
	P: PotNameMapping<AccountId32>,
{
	fn pots_btreemap(names: &[&str]) -> BTreeMap<String, C::AccountId> {
		let mut pots_map = BTreeMap::new();
		let _: Vec<_> = names
			.iter()
			.map(|n| pots_map.insert(String::from(*n), C::AccountId::from(P::into_account_id(n))))
			.collect();

		pots_map
	}
}
