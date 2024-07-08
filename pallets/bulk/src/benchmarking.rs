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

//! Benchmarking setup for pallet-bulk
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Bulk;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use pallet_broker::{CoreMask, RegionId};

mod test_sproof {
	use sp_trie::StorageProof;
	#[derive(Clone, Default)]
	pub struct ParaHeaderSproofBuilder {
		pub num_items: usize,
	}

	impl ParaHeaderSproofBuilder {
		pub fn into_state_root_and_proof(
			self,
		) -> (cumulus_primitives_core::relay_chain::Hash, StorageProof) {
			let encoded = crate::proof_data::STORAGE_ROOT[self.num_items];

			let root = hex::decode(encoded).unwrap();
			let proof = StorageProof::new(
				crate::proof_data::STORAGE_PROOF.iter().map(|s| hex::decode(s).unwrap()),
			);

			(<[u8; 32]>::try_from(root).unwrap().into(), proof)
		}
	}
}

benchmarks! {
	create_record {
		let s in 0..100;
		let mut sproof_builder = test_sproof::ParaHeaderSproofBuilder::default();
		sproof_builder.num_items = 0;

		let (storage_root, coretime_chain_state_proof) = sproof_builder.into_state_root_and_proof();
		let core_mask = CoreMask::from(0xFFFFFFFFFFFFFFFFFFFF);
		let region_id = RegionId { begin: 13, core: 1, mask: core_mask };
		let bulk_inherent_data = mp_coretime_bulk::BulkInherentData {
			storage_proof: Some(coretime_chain_state_proof),
			storage_root,
			region_id,
			duration: 100,
			start_relaychain_height: 130,
			end_relaychain_height: 230,
		};
		T::RelayChainStateProvider::set_current_relay_chain_state(cumulus_pallet_parachain_system::RelayChainState {
			state_root: storage_root,
			number: 0,
		});
	}: _(RawOrigin::None, bulk_inherent_data)
	verify {
		assert_eq!(RecordIndex::<T>::get(), 1);
	}

	set_rpc_url {
		let s in 0 .. 100;
		let url = BoundedVec::try_from("ws://127.0.0.1:8855".as_bytes().to_vec()).unwrap();
	}: _(RawOrigin::Root, url.clone())
	verify {
		assert_eq!(RpcUrl::<T>::get(), Some(url));
	}
}

impl_benchmark_test_suite!(Bulk, crate::mock::ExtBuilder::default().build(), crate::mock::Test,);
