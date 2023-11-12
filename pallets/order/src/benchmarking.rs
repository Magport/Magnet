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
	}: _(RawOrigin::Root, 4, test_balance)
	verify {
		assert_eq!(SlotWidth::<T>::get(), 4);
		assert_eq!(OrderMaxAmount::<T>::get(), test_balance);
	}
}

impl_benchmark_test_suite!(Order, crate::mock::ExtBuilder::default().build(), crate::mock::Test,);
