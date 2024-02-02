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

//! Benchmarking setup for pallet-order

use super::*;

#[allow(unused)]
use crate::Pallet as Order;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	set_parameter {
		let s in 0 .. 100;
		let caller: T::AccountId = whitelisted_caller();
		let test_balance = BalanceOf::<T>::from(200000000 as u32);
		let test_threshold = BalanceOf::<T>::from(3000000000 as u32);
	}: _(RawOrigin::Root, Some(4), Some(test_balance), Some(test_threshold))
	verify {
		assert_eq!(SlotWidth::<T>::get(), 4);
		assert_eq!(OrderMaxAmount::<T>::get(), test_balance);
		assert_eq!(TxPoolThreshold::<T>::get(), test_threshold);
	}
}

impl_benchmark_test_suite!(Order, crate::mock::ExtBuilder::default().build(), crate::mock::Test,);
