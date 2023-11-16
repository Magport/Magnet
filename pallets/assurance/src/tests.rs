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

use crate as pallet_assurance;
use crate::mock::*;
use cumulus_pallet_parachain_system::{RelaychainDataProvider, RelaychainStateProvider};
use frame_support::assert_ok;
use frame_support::traits::Hooks;
use Event as AssuranceEvent;

#[test]
fn set_bid_threshold_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let default_bid_threshold = pallet_assurance::BidThreshold::<Test>::get();
		assert_eq!(default_bid_threshold, 8u32);
		assert_ok!(Assurance::set_bid_threshold(RuntimeOrigin::root(), 12u32));
		expect_event(AssuranceEvent::NewBidThreshold(12u32));
		assert_eq!(pallet_assurance::BidThreshold::<Test>::get(), 12u32);
	})
}

#[test]
fn set_liquidate_threshold_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let default_liquidate_threshold = pallet_assurance::LiquidateThreshold::<Test>::get();
		assert_eq!(default_liquidate_threshold, 100_000_000_000_000u64);
		assert_ok!(Assurance::set_liquidate_threshold(
			RuntimeOrigin::root(),
			200_000_000_000_000u64
		));
		expect_event(AssuranceEvent::NewLiquidateThreshold(200_000_000_000_000u64));
		assert_eq!(pallet_assurance::LiquidateThreshold::<Test>::get(), 200_000_000_000_000u64);
	})
}

#[test]
fn on_relaychain_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let parent_relay_blocknumber =
			u32::from(RelaychainDataProvider::<Test>::current_relay_chain_state().number);
		let bid_threshold = pallet_assurance::BidThreshold::<Test>::get();
		assert_eq!(bid_threshold, 8u32);

		let on_relay_return = Assurance::on_relaychain(parent_relay_blocknumber);
		assert_eq!(on_relay_return, 0);

		let on_relay_return =
			Assurance::on_relaychain(parent_relay_blocknumber + bid_threshold + 1u32);
		assert_eq!(on_relay_return, 1);
	})
}

#[test]
fn force_liquidate_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		assert_ok!(Pot::ensure_pot("system"));
		let pot_system = Pot::ensure_pot("system").unwrap();
		let liquidate_threshold = pallet_assurance::LiquidateThreshold::<Test>::get();
		assert_eq!(liquidate_threshold, 100_000_000_000_000u64);

		let _ = Balances::deposit_creating(&pot_system, liquidate_threshold);

		System::on_finalize(System::block_number());
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Assurance::on_initialize(System::block_number());
		System::on_finalize(System::block_number());

		assert_ok!(Pot::withdraw(RuntimeOrigin::root(), ALICE, "system".to_string(), 1));

		System::on_finalize(System::block_number());
		System::reset_events();
		System::set_block_number(System::block_number() + 1);
		System::on_initialize(System::block_number());
		Assurance::on_initialize(System::block_number());
		expect_event(AssuranceEvent::ForceLiquidate(liquidate_threshold - 1));
		System::on_finalize(System::block_number());
	})
}
