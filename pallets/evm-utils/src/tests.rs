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

#![cfg(test)]

use super::*;

use crate::mock::*;
use frame_support::assert_ok;
use Event as EvmUtilEvent;

#[test]
fn transfer_to_evm_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let bob_evm: H160 = H160::from_low_u64_be(123_123_123_123_123);
		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);

		let alice_value_before = Balances::free_balance(&ALICE);
		let bob_evm_value_before: u64 = bob_evm_account.balance.as_u64();

		let transfer_value = 80_000_000_000;

		assert_ok!(EvmUtil::transfer_to_evm(RuntimeOrigin::signed(ALICE), bob_evm, transfer_value));
		expect_event(EvmUtilEvent::TransferedToEVM(bob_evm, transfer_value, ALICE));

		let alice_value_after = Balances::free_balance(&ALICE);
		assert_eq!(alice_value_before, alice_value_after + transfer_value);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);
		let bob_evm_value_after: u64 = bob_evm_account.balance.as_u64();
		assert_eq!(bob_evm_value_before + transfer_value, bob_evm_value_after);
	})
}
