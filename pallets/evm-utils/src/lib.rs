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

pub use pallet::*;
use sp_core::H160;
use frame_support::traits::Currency;
use pallet_evm::{BalanceOf, AddressMapping};

#[frame_support::pallet]
pub mod pallet {
	use super::*;	
	use frame_support::{dispatch::DispatchResultWithPostInfo, traits::ExistenceRequirement, pallet_prelude::*};
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_evm::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TransferedToEVM(H160, BalanceOf<T>, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		TransferFailed,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000, 0) + T::DbWeight::get().writes(1))]
		pub fn transfer_to_evm(origin: OriginFor<T>, address: H160, value: BalanceOf<T>) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			let address_account_id = <T as pallet_evm::Config>::AddressMapping::into_account_id(address);

			<T as pallet_evm::Config>::Currency::transfer(
				&who,
				&address_account_id,
				value,
				ExistenceRequirement::AllowDeath,
			)?;
			
			Self::deposit_event(Event::TransferedToEVM(address, value, who));
			Ok(().into())
		}
	}
}