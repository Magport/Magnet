// // Copyright (C) Magnet.
// // This file is part of Magnet.

// // Magnet is free software: you can redistribute it and/or modify
// // it under the terms of the GNU General Public License as published by
// // the Free Software Foundation, either version 3 of the License, or
// // (at your option) any later version.

// // Magnet is distributed in the hope that it will be useful,
// // but WITHOUT ANY WARRANTY; without even the implied warranty of
// // MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// // GNU General Public License for more details.

// // You should have received a copy of the GNU General Public License
// // along with Magnet.  If not, see <http://www.gnu.org/licenses/>.

// use crate::mock::*;
// use codec::Decode;
// use cumulus_primitives_core::{ParaId, PersistedValidationData};
// use frame_support::{
// 	inherent::{InherentData, ProvideInherent},
// 	traits::UnfilteredDispatchable,
// };
// use frame_system::RawOrigin;
// use parachains_common::AccountId;
// use primitives::HeadData;
// use sp_trie::StorageProof;

// use crate::proof_data::{ENCODED_PROOFS, HEAD_DATA};
// #[test]
// fn order_default_value() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		assert_eq!(OrderPallet::slot_width(), 2);
// 	});
// }

// #[test]
// fn order_normal_test() {
// 	ExtBuilder::default().build().execute_with(|| {
// 		System::set_block_number(1);

// 		let encoded = ENCODED_PROOFS[0];
// 		let root = hex::decode(encoded.0).unwrap();
// 		let relay_chain_state_proof =
// 			StorageProof::new(encoded.1.iter().map(|s| hex::decode(s).unwrap()));
// 		let relay_root: cumulus_primitives_core::relay_chain::Hash =
// 			<[u8; 32]>::try_from(root).unwrap().into();
// 		let mut inherent_data = InherentData::default();
// 		let head_data = hex::decode(HEAD_DATA[0]).unwrap();
// 		let perist_data = PersistedValidationData {
// 			parent_head: HeadData::decode(&mut head_data.as_slice()).unwrap(),
// 			relay_parent_number: 29,
// 			relay_parent_storage_root: relay_root,
// 			max_pov_size: 5242880 as u32,
// 		};
// 		let order_inherent_data = magnet_primitives_order::OrderInherentData {
// 			relay_storage_proof: relay_chain_state_proof,
// 			validation_data: Some(perist_data),
// 			para_id: ParaId::from(1000),
// 			sequence_number: 0,
// 			author_pub: Some(AccountId::from(hex_literal::hex!(
// 				"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
// 			))),
// 		};
// 		inherent_data
// 			.put_data(magnet_primitives_order::INHERENT_IDENTIFIER, &order_inherent_data)
// 			.expect("failed to put VFP inherent");
// 		OrderPallet::create_inherent(&inherent_data)
// 			.expect("got an inherent")
// 			.dispatch_bypass_filter(RawOrigin::None.into())
// 			.expect("dispatch succeeded");
// 		OrderPallet::on_finalize(1);
// 		assert_eq!(OrderPallet::sequence_number(), 1);
// 		let gas_cost = MockPallet::get_gas_cost(1).unwrap();
// 		assert_eq!(
// 			gas_cost.0,
// 			AccountId::from(hex_literal::hex!(
// 				"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
// 			))
// 		);
// 	});
// }
