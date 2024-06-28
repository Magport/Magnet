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

use super::*;

#[allow(unused)]
use crate::Pallet as Bulk;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	set_rpc_url {
		let s in 0 .. 100;
		let url = BoundedVec::try_from("ws://127.0.0.1:8855".as_bytes().to_vec()).unwrap();
	}: _(RawOrigin::Root, url.clone())
	verify {
		assert_eq!(RpcUrl::<T>::get(), Some(url));
	}
}

impl_benchmark_test_suite!(Bulk, crate::mock::ExtBuilder::default().build(), crate::mock::Test,);
