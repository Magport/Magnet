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

pub mod collator;
pub mod collators;

const LOG_TARGET: &str = "on_demand_aura::magnet";

use sp_core::crypto::{ByteArray, Pair};
use sp_keystore::KeystorePtr;

type AuthorityId<P> = <P as Pair>::Public;

pub async fn order_slot<P: Pair>(
	idx: u32,
	authorities: &[AuthorityId<P>],
	keystore: &KeystorePtr,
) -> Option<P::Public> {
	if authorities.is_empty() {
		return None;
	}

	let expected_author = authorities.get(idx as usize).expect(
		"authorities not empty; index constrained to list length;this is a valid index; qed",
	);

	if keystore.has_keys(&[(expected_author.to_raw_vec(), sp_application_crypto::key_types::AURA)])
	{
		Some(expected_author.clone())
	} else {
		None
	}
}
