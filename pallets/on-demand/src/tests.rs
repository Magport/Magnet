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

use crate::mock::*;
use codec::Decode;
use cumulus_primitives_core::{ParaId, PersistedValidationData};
use frame_support::{
	inherent::{InherentData, ProvideInherent},
	traits::UnfilteredDispatchable,
};
use frame_system::RawOrigin;
use parachains_common::AccountId;
use primitives::HeadData;
use sp_consensus_aura::sr25519::AuthorityId;
use sp_trie::StorageProof;

#[test]
fn order_default_value() {
	ExtBuilder::default().build().execute_with(|| {
		assert_eq!(OrderPallet::slot_width(), 3);
	});
}

#[test]
fn order_normal_test() {
	ExtBuilder::default().build().execute_with(|| {
		System::set_block_number(1);

		let mut inherent_data = InherentData::default();
		let order_inherent_data: mp_coretime_on_demand::OrderInherentData<AuthorityId> =
			mp_coretime_on_demand::OrderInherentData {
				relay_chian_number: 40,
				author_pub: Some(get_from_seed::<sp_core::sr25519::Public>("Alice").into()),
				price: 10000000,
			};
		inherent_data
			.put_data(mp_coretime_on_demand::INHERENT_IDENTIFIER, &order_inherent_data)
			.expect("failed to put VFP inherent");
		OrderPallet::create_inherent(&inherent_data)
			.expect("got an inherent")
			.dispatch_bypass_filter(RawOrigin::None.into())
			.expect("dispatch succeeded");
		OrderPallet::on_finalize(1);
		assert_eq!(OrderPallet::sequence_number(), 1);
		let gas_cost = MockPallet::get_gas_cost(1).unwrap();
		assert_eq!(
			gas_cost.0,
			AccountId::from(hex_literal::hex!(
				"d43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d"
			))
		);
	});
}
