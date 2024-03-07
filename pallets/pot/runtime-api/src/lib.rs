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

#![cfg_attr(not(feature = "std"), no_std)]

use scale_info::prelude::string::String;

sp_api::decl_runtime_apis! {
	/// API for Pot rpc.
	pub trait PotRPCApi {
		/// return pot balance
		fn balance_of(pot_name: String) -> Result<u128, sp_runtime::DispatchError>;
		/// return base balance
		fn balance_of_base() -> Result<u128, sp_runtime::DispatchError>;
	}
}
