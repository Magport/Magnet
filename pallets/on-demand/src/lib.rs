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
use cumulus_pallet_parachain_system::RelayChainStateProof;
use frame_support::{
	dispatch::DispatchResultWithPostInfo, dispatch::PostDispatchInfo, pallet_prelude::*,
	traits::Currency,
};
use frame_system::pallet_prelude::*;
use frame_system::{self, EventRecord};
use mp_coretime_on_demand::{
	metadata::api::{runtime_types, runtime_types::rococo_runtime as polakdot_runtime},
	well_known_keys::SYSTEM_EVENTS,
};
pub use pallet::*;
use primitives::Balance;
use primitives::{Id as ParaId, PersistedValidationData};
use sp_runtime::sp_std::{prelude::*, vec};
use sp_runtime::{traits::Member, RuntimeAppPublic};
pub mod weights;
use sp_core::crypto::ByteArray;
use sp_runtime::Perbill;
use weights::WeightInfo;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(any(test, feature = "runtime-benchmarks"))]
mod benchmarking;
mod proof_data;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// ondemand order information.
#[derive(Encode, Decode, Default, Clone, Copy, TypeInfo, MaxEncodedLen, Debug)]
pub struct Order<AuthorityId> {
	/// The number used to record the order, incremented each time.
	pub sequence_number: u64,
	// relaychain_block_hash:Hash,
	// relaychain_block_height:u32,
	/// Account for placing order.
	pub orderer: AuthorityId,
	/// Order price.
	pub price: Balance,
	/// Whether the order was executed.
	pub executed: bool,
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

	/// Sequence number,number of each order.
	#[pallet::storage]
	#[pallet::getter(fn sequence_number)]
	pub type SequenceNumber<T> = StorageValue<_, u64, ValueQuery>;

	/// Record the relaychain block height of the latest order
	#[pallet::storage]
	#[pallet::getter(fn current_relay_height)]
	pub type CurrentRelayHeight<T> = StorageValue<_, u32, ValueQuery>;

	/// The order interval is 2^slotwidth.
	#[pallet::storage]
	#[pallet::getter(fn slot_width)]
	pub(super) type SlotWidth<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// The maximum price the user is willing to pay when placing an order.
	#[pallet::storage]
	#[pallet::getter(fn order_max_amount)]
	pub(super) type OrderMaxAmount<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	/// Gas threshold that triggers order placement.
	#[pallet::storage]
	#[pallet::getter(fn txpool_threshold)]
	pub(super) type TxPoolThreshold<T: Config> = StorageValue<_, Perbill, ValueQuery>;

	/// Order Information Map.
	#[pallet::storage]
	#[pallet::getter(fn order_map)]
	pub type OrderMap<T: Config> =
		StorageMap<_, Twox64Concat, u64, Order<T::AuthorityId>, OptionQuery>;

	/// Convert block height to sequence number.
	#[pallet::storage]
	#[pallet::getter(fn block_2_sequence)]
	pub type Block2Sequence<T: Config> =
		StorageMap<_, Twox64Concat, BlockNumberFor<T>, u64, OptionQuery>;

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub slot_width: u32,
		pub price_limit: BalanceOf<T>,
		pub price_threshold: Perbill,
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			SlotWidth::<T>::put(&self.slot_width);
			OrderMaxAmount::<T>::put(&self.price_limit);
			TxPoolThreshold::<T>::put(&self.price_threshold);
		}
	}

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
		/// Order already exists.
		OrderExist,
		/// Failed to create order.
		CreateOrderFail,
		/// Invalid Validation data.
		InvalidValidation,
		/// Incorrect sequence number
		WrongSequenceNumber,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Called at the end of each block to check whether an order has been placed.
		/// If so, modify the execution status and increase the sequencer number.
		fn on_finalize(block_number: BlockNumberFor<T>) {
			let old_sequence_number = SequenceNumber::<T>::get();
			let order = OrderMap::<T>::get(old_sequence_number);
			if let Some(t_order) = order {
				let orderer = t_order.orderer;
				OrderMap::<T>::remove(old_sequence_number);
				OrderMap::<T>::insert(
					old_sequence_number,
					Order::<T::AuthorityId> {
						sequence_number: old_sequence_number,
						orderer: orderer.clone(),
						price: t_order.price,
						executed: true,
					},
				);
				SequenceNumber::<T>::set(old_sequence_number + 1);
				Block2Sequence::<T>::insert(block_number, old_sequence_number);
				Self::deposit_event(Event::OrderCreate {
					sequence_number: old_sequence_number,
					orderer,
				});
			}
		}
	}

	#[pallet::inherent]
	impl<T: Config> ProvideInherent for Pallet<T> {
		type Call = Call<T>;
		type Error = MakeFatalError<()>;

		const INHERENT_IDENTIFIER: InherentIdentifier = mp_coretime_on_demand::INHERENT_IDENTIFIER;
		fn create_inherent(data: &InherentData) -> Option<Self::Call> {
			let data: mp_coretime_on_demand::OrderInherentData<T::AuthorityId> = data
				.get_data(&mp_coretime_on_demand::INHERENT_IDENTIFIER)
				.ok()
				.flatten()
				.expect("there is not data to be posted; qed");
			if data.validation_data.is_some() {
				Some(Call::create_order { data })
			} else {
				None
			}
		}
		fn is_inherent(call: &Self::Call) -> bool {
			matches!(call, Call::create_order { .. })
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
		pub fn create_order(
			origin: OriginFor<T>,
			data: mp_coretime_on_demand::OrderInherentData<T::AuthorityId>,
		) -> DispatchResultWithPostInfo {
			ensure_none(origin)?;

			let mp_coretime_on_demand::OrderInherentData {
				relay_storage_proof,
				validation_data,
				sequence_number,
				para_id,
				author_pub,
			} = data;
			let total_weight = match validation_data {
				Some(validation_data) => {
					let (_, price) = Self::check_order_proof(
						relay_storage_proof,
						validation_data.clone(),
						author_pub.clone(),
						para_id,
					)
					.ok_or(Error::<T>::CreateOrderFail)?;
					let old_sequence_number = SequenceNumber::<T>::get();
					let order = OrderMap::<T>::get(old_sequence_number);
					if sequence_number != old_sequence_number {
						// In the worst-case scenario, if there are multiple orders at the same
						// time,  it may be due to system issues or it may be due to human
						// intervention.   Currently, we only support running one order at the same
						// time Err(Error::<T>::WrongSequenceNumber)?;
						// Continuing to produce blocks, recording errors
						log::info!("========WrongSequenceNumber:{:?}========", sequence_number);
					}
					if order.is_none() {
						OrderMap::<T>::insert(
							old_sequence_number,
							Order::<T::AuthorityId> {
								sequence_number: old_sequence_number,
								orderer: author_pub.unwrap(),
								price,
								executed: false,
							},
						);
						CurrentRelayHeight::<T>::set(validation_data.relay_parent_number);
					} else {
						Err(Error::<T>::OrderExist)?;
					}
					T::DbWeight::get().reads_writes(2, 1)
				},
				None => T::DbWeight::get().reads_writes(0, 0),
			};
			Ok(PostDispatchInfo { actual_weight: Some(total_weight), pays_fee: Pays::No })
		}

		/// Order pallet parameter settings.
		/// Set slot width
		/// It can only be called by accounts with sudo privileges or authorized organization members.
		///
		/// Parameters:
		/// - `slot_width`: The order interval is 2^slotwidth..
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn set_slot_width(origin: OriginFor<T>, slot_width: u32) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;

			<SlotWidth<T>>::put(slot_width);
			Ok(().into())
		}

		/// Order pallet parameter settings.
		/// Set price limit
		/// It can only be called by accounts with sudo privileges or authorized organization members.
		///
		/// Parameters:
		/// - `price_limit`: The maximum price the user is willing to pay when placing an order.
		#[pallet::call_index(2)]
		#[pallet::weight(0)]
		pub fn set_price_limit(
			origin: OriginFor<T>,
			price_limit: BalanceOf<T>,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;

			<OrderMaxAmount<T>>::put(price_limit);
			Ok(().into())
		}
		/// Order pallet parameter settings.
		/// Set gas threshold
		/// It can only be called by accounts with sudo privileges or authorized organization members.
		///
		/// Parameters:
		/// - `threshold`: Gas threshold that triggers order placement.
		#[pallet::call_index(3)]
		#[pallet::weight(0)]
		pub fn set_gas_threshold(
			origin: OriginFor<T>,
			threshold: Perbill,
		) -> DispatchResultWithPostInfo {
			T::UpdateOrigin::ensure_origin(origin)?;

			<TxPoolThreshold<T>>::put(threshold);
			Ok(().into())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Obtain the order account and price from the relaychain's validation.
	///
	/// Parameters:
	/// - `relay_storage_proof`: The proof of relay chain storage.
	///- `validation_data`: The validation data.
	/// - `para_id`: ID of parachain.
	fn get_author_from_proof(
		relay_storage_proof: sp_trie::StorageProof,
		validation_data: PersistedValidationData,
		para_id: ParaId,
	) -> Option<(T::AuthorityId, Balance)> {
		let relay_storage_root = validation_data.relay_parent_storage_root;
		let relay_storage_rooted_proof =
			RelayChainStateProof::new(para_id, relay_storage_root, relay_storage_proof)
				.expect("Invalid relay chain state proof");
		let head_data = relay_storage_rooted_proof
			.read_entry::<Vec<Box<EventRecord<polakdot_runtime::RuntimeEvent, T::Hash>>>>(
				SYSTEM_EVENTS,
				None,
			)
			.ok()?;
		let v_price: Vec<u128> = head_data
			.iter()
			.filter_map(|item| {
				if let polakdot_runtime::RuntimeEvent::OnDemandAssignmentProvider(
					runtime_types::polkadot_runtime_parachains::assigner_on_demand::pallet::Event::OnDemandOrderPlaced{
							para_id: pid,
							spot_price: sprice,
						}) = &item.event
				{
					if pid.encode() == para_id.encode() {
						Some(*sprice)
					} else {
						None
					}
				} else {
					None
				}
			})
			.collect();
		let orderer: Vec<(T::AuthorityId, u128)> = v_price
			.iter()
			.filter_map(|item| {
				let mut orderer = None;
				let _: Vec<_> = head_data
					.iter()
					.filter_map(|event| {
						if let polakdot_runtime::RuntimeEvent::Balances(
							runtime_types::pallet_balances::pallet::Event::Withdraw {
								who: ref order,
								amount: eprice,
							},
						) = event.event
						{
							if eprice == *item {
								orderer = match T::AuthorityId::try_from(order.clone().as_slice()) {
									Ok(order) => Some((order, eprice)),
									Err(_) => None,
								};
								Some(())
							} else {
								None
							}
						} else {
							None
						}
					})
					.collect();
				orderer
			})
			.collect();
		if orderer.len() > 0 {
			Some(orderer[0].clone())
		} else {
			None
		}
	}

	/// Check whether the account is in the validation of relaychain.
	///
	/// Parameters:
	/// - `relay_storage_proof`: The proof of relay chain storage.
	/// - `validation_data`: The validation data.
	/// - `author_pub`: Account.
	/// - `para_id`: ID of parachain.
	fn check_order_proof(
		relay_storage_proof: sp_trie::StorageProof,
		validation_data: PersistedValidationData,
		author_pub: Option<T::AuthorityId>,
		para_id: ParaId,
	) -> Option<(T::AuthorityId, Balance)> {
		let op_author = Self::get_author_from_proof(relay_storage_proof, validation_data, para_id);
		match op_author {
			Some((author, spot_price)) => {
				if author_pub == Some(author.clone()) {
					Some((author, spot_price))
				} else {
					None
				}
			},
			None => None,
		}
	}

	/// Check whether there is an order event in the validation of relaychain.
	///
	/// Parameters:
	/// - `relay_storage_proof`: The proof of relay chain storage.
	/// - `validation_data`: The validation data.
	/// - `para_id`: ID of parachain.
	pub fn order_placed(
		relay_storage_proof: sp_trie::StorageProof,
		validation_data: PersistedValidationData,
		para_id: ParaId,
	) -> Option<T::AuthorityId> {
		let op_author = Self::get_author_from_proof(relay_storage_proof, validation_data, para_id);
		match op_author {
			Some((author, _)) => Some(author),
			None => None,
		}
	}

	/// Whether the gas threshold for placing an order has been reached.
	///
	/// Parameters:
	/// - `gas_balance`: The total gas.
	pub fn reach_txpool_threshold(gas_balance: BalanceOf<T>, core_price: BalanceOf<T>) -> bool {
		let txpool_threshold = TxPoolThreshold::<T>::get();
		gas_balance > txpool_threshold * core_price
	}

	/// Whether the order with the specified sequence number is executed.
	///
	/// Parameters:
	/// - `sequence_number`: The sequence number.
	pub fn order_executed(sequence_number: u64) -> bool {
		let order_map = OrderMap::<T>::get(sequence_number);
		match order_map {
			Some(order) => order.executed,
			None => false,
		}
	}
}
