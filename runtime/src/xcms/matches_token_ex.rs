// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Various implementations for the `MatchesFungible` trait.

// Modified by Alex Wang for extending xcm transmit currency and token
// with different precision, 2024/03

use frame_support::traits::Get;
use sp_std::{collections::btree_map::BTreeMap, marker::PhantomData};
use xcm::latest::{
	Asset, AssetId, AssetInstance,
	Fungibility::{Fungible, NonFungible},
	Location,
};
use xcm_executor::traits::{MatchesFungible, MatchesNonFungible};

pub struct IsConcreteEx<T, M>(PhantomData<T>, PhantomData<M>);
impl<T: Get<Location>, M: Get<BTreeMap<Location, u64>>, B: TryFrom<u128>> MatchesFungible<B>
	for IsConcreteEx<T, M>
{
	fn matches_fungible(a: &Asset) -> Option<B> {
		match (&a.id, &a.fun) {
			(AssetId(ref id), Fungible(ref amount)) if id == &T::get() => {
				let precision_multiplier = if let Some(v) = M::get().get(id) { *v } else { 1u64 };
				(*amount * u128::from(precision_multiplier)).try_into().ok()
			},
			_ => None,
		}
	}
}
impl<T: Get<Location>, I: TryFrom<AssetInstance>, M> MatchesNonFungible<I> for IsConcreteEx<T, M> {
	fn matches_nonfungible(a: &Asset) -> Option<I> {
		match (&a.id, &a.fun) {
			(AssetId(id), NonFungible(instance)) if id == &T::get() => (*instance).try_into().ok(),
			_ => None,
		}
	}
}
