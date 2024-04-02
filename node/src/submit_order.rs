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
use crate::metadata;
use cumulus_primitives_core::{
	relay_chain::BlockId, relay_chain::BlockNumber as RelayBlockNumber, ParaId,
};
use cumulus_relay_chain_interface::{RelayChainInterface, RelayChainResult};
use sp_application_crypto::AppCrypto;
use sp_core::ByteArray;
use sp_core::H256;
use sp_keystore::KeystorePtr;
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	MultiSignature as SpMultiSignature,
};
use subxt::client::OfflineClientT;
use subxt::{
	config::polkadot::PolkadotExtrinsicParamsBuilder as Params, tx::Signer, utils::MultiSignature,
	Config, OnlineClient, PolkadotConfig,
};

#[derive(Debug)]
pub enum SubmitOrderError {
	RPCUrlError,
	RPCConnectError,
	RPCCallException,
	NonceGetError,
	StorageGetError,
	GetBlockError,
	GetHeadError,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Signature(pub [u8; 64]);

impl From<Signature> for MultiSignature {
	fn from(value: Signature) -> Self {
		MultiSignature::Sr25519(value.0)
	}
}
pub struct SignerKeystore<T: Config> {
	account_id: T::AccountId,
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

pub async fn build_rpc_for_submit_order(
	url: &str,
	para_id: ParaId,
	max_amount: u128,
	hash: H256,
	keystore: KeystorePtr,
	slot_block: u32,
	height: RelayBlockNumber,
	relay_chain: impl RelayChainInterface + Clone,
	number: u32,
) -> Result<(), SubmitOrderError> {
	let client = OnlineClient::<PolkadotConfig>::from_url(url)
		.await
		.map_err(|_e| SubmitOrderError::RPCConnectError)?;

	let place_order = metadata::api::tx().on_demand_assignment_provider().place_order_allow_death(
		max_amount,
		metadata::api::runtime_types::polkadot_parachain_primitives::primitives::Id(para_id.into()),
	);

	let signer_keystore = SignerKeystore::<PolkadotConfig>::new(keystore.clone());

	// let tx_params = Params::new().mortal_unchecked(number.into(), hash , slot_block.into()).build();

	let submit_result = client.tx().sign_and_submit_default(&place_order, &signer_keystore).await;
	log::info!("submit_result:{:?},{:?},{:?}", submit_result, height, hash);
	submit_result.map_err(|_e| SubmitOrderError::RPCCallException)?;

	Ok(())
}
