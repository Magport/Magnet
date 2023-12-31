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

//Modified by Alex Wang, 2023/11

use frame_support::traits::ContainsPair;
use scale_info::TypeInfo;
use sp_runtime::codec::{Decode, Encode};
use xcm::prelude::*;
use xcm_executor::traits::TransactAsset;

/// Errors related to determining asset transfer support.
#[derive(Copy, Clone, Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
pub enum Error {
	/// Invalid non-concrete asset.
	NotConcrete,
	/// Reserve chain could not be determined for assets.
	UnknownReserve,
}

/// Specify which type of asset transfer is required for a particular `(asset, dest)` combination.
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TransferType {
	/// should teleport `asset` to `dest`
	Teleport,
	/// should reserve-transfer `asset` to `dest`, using local chain as reserve
	LocalReserve,
	/// should reserve-transfer `asset` to `dest`, using `dest` as reserve
	DestinationReserve,
	/// should reserve-transfer `asset` to `dest`, using remote chain `MultiLocation` as reserve
	RemoteReserve(MultiLocation),
}

/// A trait for identifying asset transfer type based on `IsTeleporter` and `IsReserve`
/// configurations.
pub trait XcmAssetTransfers {
	/// Combinations of (Asset, Location) pairs which we trust as reserves. Meaning
	/// reserve-based-transfers are to be used for assets matching this filter.
	type IsReserve: ContainsPair<MultiAsset, MultiLocation>;

	/// Combinations of (Asset, Location) pairs which we trust as teleporters. Meaning teleports are
	/// to be used for assets matching this filter.
	type IsTeleporter: ContainsPair<MultiAsset, MultiLocation>;

	/// How to withdraw and deposit an asset.
	type AssetTransactor: TransactAsset;

	/// Determine transfer type to be used for transferring `asset` from local chain to `dest`.
	fn determine_for(asset: &MultiAsset, dest: &MultiLocation) -> Result<TransferType, Error> {
		if Self::IsTeleporter::contains(asset, dest) {
			// we trust destination for teleporting asset
			return Ok(TransferType::Teleport);
		} else if Self::IsReserve::contains(asset, dest) {
			// we trust destination as asset reserve location
			return Ok(TransferType::DestinationReserve);
		}

		// try to determine reserve location based on asset id/location
		let asset_location = match asset.id {
			Concrete(location) => Ok(Self::chain_location(&location)),
			_ => Err(Error::NotConcrete),
		}?;
		if asset_location == MultiLocation::here()
			|| Self::IsTeleporter::contains(asset, &asset_location)
		{
			// if the asset is local, then it's a local reserve
			// it's also a local reserve if the asset's location is not `here` but it's a location
			// where it can be teleported to `here` => local reserve
			Ok(TransferType::LocalReserve)
		} else if Self::IsReserve::contains(asset, &asset_location) {
			// remote location that is recognized as reserve location for asset
			Ok(TransferType::RemoteReserve(asset_location))
		} else {
			// remote location that is not configured either as teleporter or reserve => cannot
			// determine asset reserve
			Err(Error::UnknownReserve)
		}
	}

	fn chain_location(slf: &MultiLocation) -> MultiLocation {
		let mut clone = *slf;
		// start popping junctions until we reach chain identifier
		while let Some(j) = clone.last() {
			if matches!(j, Junction::Parachain(_) | Junction::GlobalConsensus(_)) {
				// return chain subsection
				return clone;
			} else {
				(clone, _) = clone.split_last_interior();
			}
		}
		MultiLocation::new(clone.parents, Junctions::Here)
	}
}
