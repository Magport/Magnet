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

use super::*;

use crate::mock::*;
use frame_support::{assert_noop, assert_ok};
use Event as PotEvent;

#[test]
fn deposit_to_pot_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000);

		assert_noop!(Pot::ensure_pot("ttt"), Error::<Test>::NotPot);
		assert_ok!(Pot::ensure_pot("treasury"));
		let treasury = Pot::ensure_pot("treasury").unwrap();
		let _ = Balances::deposit_creating(&treasury, 10_000_000_000_000);

		let treasury_before = Balances::free_balance(treasury.clone());
		let deposit_value = 80_000_000_000;

		assert_ok!(Pot::deposit(
			RuntimeOrigin::signed(ALICE),
			"treasury".to_string(),
			deposit_value
		));
		expect_event(PotEvent::Deposit(ALICE, "treasury".to_string(), deposit_value));
		assert_eq!(Balances::free_balance(treasury), treasury_before + deposit_value);
	})
}

#[test]
fn withdraw_from_pot_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000);

		assert_noop!(Pot::ensure_pot("ttt"), Error::<Test>::NotPot);
		assert_ok!(Pot::ensure_pot("treasury"));
		let treasury = Pot::ensure_pot("treasury").unwrap();
		let _ = Balances::deposit_creating(&treasury, 10_000_000_000_000);

		let treasury_before = Balances::free_balance(treasury.clone());
		let withdraw_value = 80_000_000_000;

		assert_ok!(Pot::withdraw(
			RuntimeOrigin::root(),
			ALICE,
			"treasury".to_string(),
			withdraw_value
		));
		expect_event(PotEvent::Withdraw(ALICE, "treasury".to_string(), withdraw_value));
		assert_eq!(Balances::free_balance(treasury), treasury_before - withdraw_value);
	})
}

#[test]
fn withdraw_from_base_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000);
		let _ = Balances::deposit_creating(&BASE_ACCOUNT, 10_000_000_000_000);

		let base_before = Balances::free_balance(BASE_ACCOUNT);
		let withdraw_value = 80_000_000_000;

		assert_ok!(Pot::withdraw_base(RuntimeOrigin::root(), ALICE, withdraw_value));
		expect_event(PotEvent::WithdrawBase(ALICE, withdraw_value));
		assert_eq!(Balances::free_balance(BASE_ACCOUNT), base_before - withdraw_value);
	})
}
