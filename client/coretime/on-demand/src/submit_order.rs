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

//! The code here is implemented, constructing a transaction from the parachain and sending it to the relay chain to purchase core.
//!
//! Subxt is used here to construct and submit the transaction.
//!

use crate::metadata;
use cumulus_primitives_core::ParaId;
use sp_application_crypto::AppCrypto;
use sp_core::ByteArray;
use sp_keystore::KeystorePtr;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	MultiSignature as SpMultiSignature,
};
use subxt::{tx::Signer, utils::MultiSignature, Config, OnlineClient, PolkadotConfig};

#[derive(Debug)]
pub enum SubmitOrderError {
	RPCConnectError,
	RPCCallException,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Signature(pub [u8; 64]);

impl From<Signature> for MultiSignature {
	fn from(value: Signature) -> Self {
		MultiSignature::Sr25519(value.0)
	}
}
pub struct SignerKeystore<T: Config> {
	/// Account ID
	account_id: T::AccountId,
	/// Keystore of node
	keystore: KeystorePtr,
}
impl<T> SignerKeystore<T>
where
	T: Config,
	T::AccountId: From<[u8; 32]>,
{
	pub fn new(keystore: KeystorePtr) -> Self {
		let pub_key =
			keystore.sr25519_public_keys(sp_consensus_aura::sr25519::AuthorityPair::ID)[0];

		let binding = <SpMultiSignature as Verify>::Signer::from(pub_key).into_account().clone();

		let account_id = binding.as_slice();
		let mut r = [0u8; 32];
		r.copy_from_slice(account_id);
		let acc = T::AccountId::try_from(r).ok().unwrap();
		Self { account_id: acc.clone(), keystore }
	}
}
impl<T> Signer<T> for SignerKeystore<T>
where
	T: Config,
	T::AccountId: From<[u8; 32]>,
	T::Signature: From<Signature>,
{
	fn account_id(&self) -> T::AccountId {
		self.account_id.clone()
	}

	fn address(&self) -> T::Address {
		self.account_id.clone().into()
	}

	/// Use aura's key to sign
	/// TODO:Modify to other keys, or load the key in some way.
	fn sign(&self, signer_payload: &[u8]) -> T::Signature {
		let pub_key =
			self.keystore.sr25519_public_keys(sp_consensus_aura::sr25519::AuthorityPair::ID)[0];

		let signature = self
			.keystore
			.sr25519_sign(sp_consensus_aura::sr25519::AuthorityPair::ID, &pub_key, signer_payload)
			.unwrap()
			.unwrap();

		Signature(signature.0).into()
	}
}

/// Construct the transaction and sign it, and then submit the transaction through the rpc interface.
pub async fn build_rpc_for_submit_order(
	url: &str,
	para_id: ParaId,
	max_amount: u128,
	keystore: KeystorePtr,
) -> Result<(), SubmitOrderError> {
	let client = OnlineClient::<PolkadotConfig>::from_url(url)
		.await
		.map_err(|_e| SubmitOrderError::RPCConnectError)?;

	let place_order = metadata::place_order_allow_death(max_amount, metadata::Id(para_id.into()));

	let signer_keystore = SignerKeystore::<PolkadotConfig>::new(keystore.clone());

	let submit_result = client.tx().sign_and_submit_default(&place_order, &signer_keystore).await;
	log::info!("submit_result:{:?}", submit_result);
	submit_result.map_err(|_e| SubmitOrderError::RPCCallException)?;

	Ok(())
}
