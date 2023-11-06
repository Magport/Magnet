#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
use frame_support::{
	pallet_prelude::*,
	traits::Currency,
	dispatch::DispatchResultWithPostInfo,  dispatch::PostDispatchInfo,
};
use frame_system::pallet_prelude::*;
use magnet_chain_state_snapshot::{GenericStateProof, ReadEntryErr};
use frame_system::{self, EventRecord};
use codec::{Decode, MaxEncodedLen};
use sp_runtime::sp_std::{prelude::*, vec};
use runtime_parachains::assigner_on_demand as parachains_assigner_on_demand;
use primitives::{Id as ParaId, PersistedValidationData};
use sp_core::crypto::ByteArray;
use sp_runtime::{
		traits::Member,
		RuntimeAppPublic
};
use magnet_primitives_order::well_known_keys::SYSTEM_EVENTS;
use sp_std::result;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;


type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[derive(Encode, Decode, Default, Clone, Copy, TypeInfo, MaxEncodedLen, Debug)]
pub struct Order<AuthorityId> {
	sequence_number: u64,
	// relaychain_block_hash:Hash,
    // relaychain_block_height:u32,
    orderer:AuthorityId,
    // price:Balance,
	executed:bool,
}
#[frame_support::pallet]
pub mod pallet {
    use super::*;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_transaction_payment::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Currency: Currency<Self::AccountId>;

		type AuthorityId: Member
			+ Parameter
			+ RuntimeAppPublic
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen;
		/// The default value of w.
		#[pallet::constant]
		type SlotWidth: Get<u32>;

		/// The max value of place order.
		#[pallet::constant]
		type OrderMaxAmount: Get<BalanceOf<Self>>;

		
		#[pallet::constant]
		type TxPoolThreshold: Get<BalanceOf<Self>>;
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn sequence_number)]
	pub type SequenceNumber<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::type_value]
	pub fn SlotWidthOnEmpty<T: Config>() -> u32 {
		T::SlotWidth::get()
	}
	#[pallet::type_value]
	pub fn OrderMaxAmountOnEmpty<T: Config>() -> BalanceOf<T> {
		T::OrderMaxAmount::get()
	}
	#[pallet::type_value]
	pub fn TxPoolThresholdOnEmpty<T: Config>() -> BalanceOf<T> {
		T::TxPoolThreshold::get()
	}

	#[pallet::storage]
	#[pallet::getter(fn slot_width)]
	pub(super) type SlotWidth<T: Config> =
		StorageValue<_, u32, ValueQuery, SlotWidthOnEmpty<T>>;


	#[pallet::storage]
	#[pallet::getter(fn order_max_amount)]
	pub(super) type OrderMaxAmount<T: Config> =
		StorageValue<_, BalanceOf<T>, ValueQuery, OrderMaxAmountOnEmpty<T>>;

	#[pallet::storage]
	#[pallet::getter(fn txpool_threshold)]
	pub(super) type TxPoolThreshold<T: Config> =
		StorageValue<_, BalanceOf<T>, ValueQuery, TxPoolThresholdOnEmpty<T>>;


	#[pallet::storage]
	#[pallet::getter(fn order_map)]
	pub type OrderMap<T: Config> = StorageMap<_, Twox64Concat, u64, Order<T::AuthorityId>, OptionQuery>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		OrderCreate {},
	}

	#[pallet::error]
	pub enum Error<T> {
		FailedReading,
		OrderNotExist,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_finalize(_: BlockNumberFor<T>) {
			let old_sequence_number = SequenceNumber::<T>::get();
			let order = OrderMap::<T>::get(old_sequence_number);
			if let Some(t_order) = order {
				let new_order = t_order.clone();
				OrderMap::<T>::remove(old_sequence_number);
				OrderMap::<T>::insert(old_sequence_number,Order::<T::AuthorityId>{
					sequence_number: old_sequence_number,
					orderer:new_order.orderer,
					executed:true,
				});
				SequenceNumber::<T>::set(old_sequence_number + 1);
				Self::deposit_event(Event::OrderCreate { });
			}
			log::info!("{:?}", OrderMap::<T>::get(old_sequence_number));
        }
	}


	#[pallet::inherent]
    impl<T: Config> ProvideInherent for Pallet<T> {
        type Call = Call<T>;
		type Error = MakeFatalError<()>;

        const INHERENT_IDENTIFIER: InherentIdentifier =
            magnet_primitives_order::INHERENT_IDENTIFIER;
		fn create_inherent(data: &InherentData) -> Option<Self::Call> {
			let data: magnet_primitives_order::OrderInherentData<T::AuthorityId> = data
			.get_data(&magnet_primitives_order::INHERENT_IDENTIFIER)
			.ok()
			.flatten()
			.expect("there is not data to be posted; qed");
			Some(Call::check_order { data })
		}
		fn is_inherent(call: &Self::Call) -> bool {
			matches!(call, Call::check_order { .. })
		}

		fn check_inherent(
			call: &Self::Call,
			_data: &InherentData,
		) -> result::Result<(), Self::Error> {
			match call {
				Call::check_order { .. } => {
					log::info!("add order==========check_inherent");
					return Ok(());
				},
				_ => return Ok(()),
			};
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
        #[pallet::weight((0, DispatchClass::Mandatory))]
        pub fn check_order(
            origin: OriginFor<T>,
            data: magnet_primitives_order::OrderInherentData<T::AuthorityId>,
        ) -> DispatchResultWithPostInfo {
            ensure_none(origin)?;

            let total_weight =
			T::DbWeight::get().reads_writes(1, 1);
			let magnet_primitives_order::OrderInherentData {
				relay_storage_proof,
				validation_data,
				sequence_number,
				para_id,
				author_pub,
		} = data;
			if let Some(validation_data) = validation_data{
				if let Some(author_pub) = author_pub {
					let _check_pass = Self::check_order_proof(relay_storage_proof, validation_data, author_pub.clone(), para_id);
					let old_sequence_number = SequenceNumber::<T>::get();
					let order = OrderMap::<T>::get(old_sequence_number);
					if sequence_number ==  old_sequence_number{
						log::info!("add order================={:?}", order);
						if order.is_none() {
							log::info!("{:?}", old_sequence_number);
							OrderMap::<T>::insert(old_sequence_number, Order::<T::AuthorityId>{
								sequence_number: old_sequence_number,
								orderer:author_pub,
								executed:false,
							});
							log::info!("{:?}", OrderMap::<T>::get(old_sequence_number));
						}
					}
				}
			}

            Ok(PostDispatchInfo {
                actual_weight: Some(total_weight),
                pays_fee: Pays::No,
            })
        }
	}
}

impl<T: Config> Pallet<T>
{
	fn check_order_proof(
		relay_storage_proof: sp_trie::StorageProof,
		validation_data: PersistedValidationData,
		author_pub: T::AuthorityId,
		para_id:ParaId,
	)-> bool
	{
		let relay_storage_root = validation_data.relay_parent_storage_root;
		let relay_storage_rooted_proof:GenericStateProof<cumulus_primitives_core::relay_chain::Block>=
			GenericStateProof::new(relay_storage_root, relay_storage_proof)
				.expect("Invalid relay chain state proof");
		// key = System Events
		let head_data = relay_storage_rooted_proof
			.read_entry::<Vec<Box<EventRecord<rococo_runtime::RuntimeEvent,T::Hash>>>>(SYSTEM_EVENTS, None)
			.map_err(|e| match e {
				ReadEntryErr::Proof => panic!("Invalid proof provided for system events key"),
				_ => Error::<T>::FailedReading,
			}).unwrap();
		let mut spot_price:u128=0;
		for i in 0..head_data.len() {
			match head_data[i].event {
		rococo_runtime::RuntimeEvent::OnDemandAssignmentProvider (parachains_assigner_on_demand::Event::OnDemandOrderPlaced{
			para_id: pid,
			spot_price: sprice,
				}
			)
			=> if pid == para_id {
				spot_price = sprice;
			},
			_=> continue,
			};
		};
		let mut order_is_collator = false;
		for i in 0..head_data.len() {
			match head_data[i].event {
			rococo_runtime::RuntimeEvent::Balances (pallet_balances::Event::Withdraw{
					who: ref order,
					amount: eprice,
					}
				)
				=> if spot_price == eprice {
					if author_pub.encode() ==order.as_slice() {
						order_is_collator = true;
						break;
					}
				},
					_=>continue,
				};
		}
		log::info!("========check_order_proof========={:?}", order_is_collator);
		order_is_collator
	}

	pub fn place_order() -> Option<u64> {
		Some(66)
	}

	pub fn order_placed(
		relay_storage_proof: sp_trie::StorageProof,
		validation_data: PersistedValidationData,
		author_pub: T::AuthorityId,
		para_id:ParaId,
	)-> bool {
		Self::check_order_proof(relay_storage_proof, validation_data, author_pub, para_id)
	}

	pub fn reach_txpool_threshold(gas_balance:BalanceOf<T>) -> bool {
		
		let txpool_threshold = TxPoolThreshold::<T>::get();
		gas_balance > txpool_threshold
	}

	pub fn order_executed(sequence_number:u64) -> bool {
		let order = OrderMap::<T>::get(sequence_number);
		order.is_some()
	}

}