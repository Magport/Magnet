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

pub fn to_checksum_address(address: H160) -> String {
	let addr_hex = format!("{:x}", address);
	let hash = keccak_256(addr_hex.as_bytes());
	let hash_hex = hex::encode(hash);

	addr_hex
		.chars()
		.enumerate()
		.map(|(i, c)| {
			if c.is_digit(10) {
				c.to_string()
			} else {
				// Each byte in the hash controls the case of two characters.
				let hash_char = hash_hex.chars().nth(i).unwrap();
				if hash_char.to_digit(16).unwrap() >= 8 {
					c.to_uppercase().to_string()
				} else {
					c.to_lowercase().to_string()
				}
			}
		})
		.collect::<String>()
}

/// Converts the given binary data into ASCII-encoded hex. It will be twice
/// the length.
pub fn to_ascii_hex(data: &[u8]) -> Vec<u8> {
	let mut r = Vec::with_capacity(data.len() * 2);
	let mut push_nibble = |n| r.push(if n < 10 { b'0' + n } else { b'a' - 10 + n });
	for &b in data.iter() {
		push_nibble(b / 16);
		push_nibble(b % 16);
	}
	r
}

/// Attempts to recover the Ethereum address from a message signature signed by
/// using the Ethereum RPC's `personal_sign` and `eth_sign`.
pub fn eth_recover(s: &EcdsaSignature, what: &[u8], extra: &[u8]) -> Option<H160> {
	let msg = keccak_256(&ethereum_signable_message(what, extra));
	let mut res = H160::default();
	res.0
		.copy_from_slice(&keccak_256(&secp256k1_ecdsa_recover(&s.0, &msg).ok()?[..])[12..]);
	Some(res)
}

/// Constructs the message that Ethereum RPC's `personal_sign` and `eth_sign`
/// would sign.
pub fn ethereum_signable_message(what: &[u8], extra: &[u8]) -> Vec<u8> {
	let prefix = b"evm:";
	let mut l = prefix.len() + what.len() + extra.len();
	let mut rev = Vec::new();
	while l > 0 {
		rev.push(b'0' + (l % 10) as u8);
		l /= 10;
	}
	let mut v = b"\x19Ethereum Signed Message:\n".to_vec();
	v.extend(rev.into_iter().rev());
	v.extend_from_slice(&prefix[..]);
	v.extend_from_slice(what);
	v.extend_from_slice(extra);
	v
}

pub fn beta_eth_recover(signature: &ecdsa::Signature, what: &[u8]) -> Option<H160> {
	let mut v = Vec::new();
	let prefix = b"evm:";
	v.extend_from_slice(&prefix[..]);
	v.extend_from_slice(what);

	let message_hash: [u8; 32] = keccak_256(v.as_ref());
	let mut sig = [0u8; 65];
	sig[0..64].copy_from_slice(&signature.0[0..64]);
	sig[64] = signature.0[64];

	let public_key = sp_io::crypto::secp256k1_ecdsa_recover(&sig, &message_hash).ok()?;

	let address_hash = keccak_256(&public_key);

	Some(H160::from_slice(&address_hash[12..32]))
}
