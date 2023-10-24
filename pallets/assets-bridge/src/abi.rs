// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::*;

pub fn mint_into_encode(account: H160, amount: u128) -> Vec<u8> {
	// signature ++ account ++ amount
	let length = 16 + 20 + 32;
	let mut v = Vec::with_capacity(length);

	// bytes4(keccak256(bytes("mint_into(address,uint256)"))
	// 0xefe51695
	let sig_mint = [239u8, 229, 22, 149];

	// first 16-bytes
	v.extend_from_slice(&sig_mint[..]);
	v.extend_from_slice(&[0u8; 12][..]);

	// second 20-bytes
	v.extend_from_slice(&account[..]);

	// third 32-bytes
	v.extend_from_slice(&[0u8; 16][..]);
	v.extend_from_slice(&amount.to_be_bytes()[..]);

	v
}

pub fn burn_from_encode(account: H160, amount: u128) -> Vec<u8> {
	// signature ++ account ++ amount
	let length = 16 + 20 + 32;
	let mut v = Vec::with_capacity(length);

	// bytes4(keccak256(bytes("burn_from(address,uint256)"))
	// 0x0f536f84
	let sig_burn = [15u8, 83, 111, 132];

	// first 16-bytes
	v.extend_from_slice(&sig_burn[..]);
	v.extend_from_slice(&[0u8; 12][..]);

	// second 20-bytes
	v.extend_from_slice(&account[..]);

	// third 32-bytes
	v.extend_from_slice(&[0u8; 16][..]);
	v.extend_from_slice(&amount.to_be_bytes()[..]);

	v
}
