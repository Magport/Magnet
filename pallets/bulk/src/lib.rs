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

//! # Order Pallet
//!
//! This pallet implements the recording and query functions of purchasing ondemand core.
//!
//! By obtaining the inherent nature of the block, parsing it out of the validation_data of the relaychain,
//! and querying whether there is an OnDemandOrderPlaced event, obtaining the order account and price from the event,
//! and then writing this record to the blockchain.
//!
//! Provides many query methods for node or other pallets to use, such as querying the gas consumed by placing an order in a certain block,
//! whether the order has been executed, whether the order threshold has been reached, etc.

#![cfg_attr(not(feature = "std"), no_std)]
use codec::{Decode, MaxEncodedLen};
use cumulus_pallet_parachain_system::{
	relay_state_snapshot::Error as Relay_Error, RelayChainStateProof,
};
use frame_support::{
	dispatch::DispatchResultWithPostInfo, dispatch::PostDispatchInfo, pallet_prelude::*,
	traits::Currency,
};
use frame_system::pallet_prelude::*;
use frame_system::{self, EventRecord};
use mp_coretime_bulk::well_known_keys::{broker_regions, REGIONS};
pub use pallet::*;
use primitives::Balance;
use primitives::{Id as ParaId, PersistedValidationData};
use sp_runtime::sp_std::{prelude::*, vec};
use sp_runtime::{traits::Member, RuntimeAppPublic};
pub mod weights;
use dp_chain_state_snapshot::GenericStateProof;
use pallet_broker::RegionRecord;
use sp_core::crypto::ByteArray;
use weights::WeightInfo;
// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(any(test, feature = "runtime-benchmarks"))]
// mod benchmarking;
// mod proof_data;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// ondemand order information.
#[derive(Encode, Decode, Default, Clone, Copy, TypeInfo, MaxEncodedLen, Debug)]
pub struct BulkRecord<Balance, AuthorityId> {
	/// Account for purchase.
	pub purchaser: AuthorityId,
	/// Purchase price.
	pub price: Balance,
	/// Purchase duration.
	pub duration: u32,
}
#[frame_support::pallet]
pub mod pallet {
	use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: Currency<Self::AccountId>;

		type AuthorityId: Member
			+ Parameter
			+ RuntimeAppPublic
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ for<'a> TryFrom<&'a [u8]>;

		type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		type WeightInfo: WeightInfo;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	#[pallet::type_value]
	pub fn RecordIndexOnEmpty<T: Config>() -> u32 {
		0
	}

	#[pallet::storage]
	#[pallet::getter(fn record_index)]
	pub type RecordIndex<T> = StorageValue<_, u32, ValueQuery, RecordIndexOnEmpty<T>>;

	/// Order Information Map.
	#[pallet::storage]
	#[pallet::getter(fn bulk_records)]
	pub type BulkRecords<T: Config> =
		StorageMap<_, Twox64Concat, u32, BulkRecord<BalanceOf<T>, T::AuthorityId>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Create order event.
		OrderCreate { sequence_number: u64, orderer: T::AuthorityId },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Error reading data.
		FailedReading,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(block_number: BlockNumberFor<T>) {}
	}

	#[pallet::inherent]
	impl<T: Config> ProvideInherent for Pallet<T> {
		type Call = Call<T>;
		type Error = MakeFatalError<()>;

		const INHERENT_IDENTIFIER: InherentIdentifier = mp_coretime_bulk::INHERENT_IDENTIFIER;

		fn create_inherent(data: &InherentData) -> Option<Self::Call> {
			let data = data.get_data(&mp_coretime_bulk::INHERENT_IDENTIFIER).ok().flatten();
			match data {
				Some(data) => Some(Call::create_record { data }),
				None => None,
			}
		}
		fn is_inherent(call: &Self::Call) -> bool {
			matches!(call, Call::create_record { .. })
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Create an order, which is called by the pallet.
		/// Users cannot actively call this function.
		/// Obtain order information by parsing inherited data.
		///
		/// Parameters:
		/// - `data`: The inherent data.
		#[pallet::call_index(0)]
		#[pallet::weight((0, DispatchClass::Mandatory))]
		pub fn create_record(
			origin: OriginFor<T>,
			data: mp_coretime_bulk::BulkInherentData,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;

			let mp_coretime_bulk::BulkInherentData { storage_proof, storage_root, region_id } =
				data;
			let relay_storage_rooted_proof: GenericStateProof<
				cumulus_primitives_core::relay_chain::Block,
			> = GenericStateProof::new(storage_root, storage_proof).unwrap();
			let key = broker_regions(region_id);
			let region_record_data = relay_storage_rooted_proof
				.read_entry::<RegionRecord<T::AuthorityId, BalanceOf<T>>>(key.as_slice(), None)
				.ok();
			if let Some(region_record) = region_record_data {
				let old_record_index = RecordIndex::<T>::get();
				BulkRecords::<T>::insert(
					old_record_index,
					BulkRecord::<BalanceOf<T>, T::AuthorityId> {
						purchaser: region_record.owner,
						price: region_record.paid.unwrap(),
						duration: region_record.end,
					},
				);
			}

			let total_weight = T::DbWeight::get().reads_writes(2, 1);
			Ok(PostDispatchInfo { actual_weight: Some(total_weight), pays_fee: Pays::No })
		}
	}
}

impl<T: Config> Pallet<T> {}

// pub trait OrderGasCost<T: frame_system::Config> {
// 	/// Gas consumed by placing an order in a certain block.
// 	///
// 	/// Parameters:
// 	/// - `block_number`: The block number of para chain.
// 	fn gas_cost(
// 		block_number: BlockNumberFor<T>,
// 	) -> Result<Option<(T::AccountId, Balance)>, DispatchError>;
// }
