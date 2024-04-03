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

//! Pot RPC.

use jsonrpsee::{
	core::RpcResult,
	proc_macros::rpc,
	types::{error::ErrorObject, ErrorObjectOwned},
};
use std::sync::Arc;

use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_runtime::traits::Block as BlockT;

pub use pallet_pot_runtime_api::PotRPCApi;

/// Pot RPC interface
#[rpc(client, server)]
pub trait PotApi<BlockHash> {
	/// Get pot balance.
	#[method(name = "pot_balance")]
	fn pot_balance(&self, pot_name: String, at: Option<BlockHash>) -> RpcResult<u128>;
	/// Get base balance.
	#[method(name = "base_balance")]
	fn base_balance(&self, at: Option<BlockHash>) -> RpcResult<u128>;
}

///Impl pot rpc
pub struct Pot<C, P> {
	/// Shared reference to the client.
	client: Arc<C>,
	_marker: std::marker::PhantomData<P>,
}

impl<C, P> Pot<C, P> {
	/// Creates a new instance of the Pot Rpc helper.
	pub fn new(client: Arc<C>) -> Self {
		Self { client, _marker: Default::default() }
	}
}

/// Error type of this RPC api.
pub enum Error {
	/// The call to runtime failed.
	RuntimeError,
}

impl From<Error> for i32 {
	fn from(e: Error) -> i32 {
		match e {
			Error::RuntimeError => 1,
		}
	}
}

impl<C, Block> PotApiServer<<Block as BlockT>::Hash> for Pot<C, Block>
where
	Block: BlockT,
	C: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	C::Api: PotRPCApi<Block>,
{
	fn pot_balance(&self, pot_name: String, at: Option<Block::Hash>) -> RpcResult<u128> {
		let api = self.client.runtime_api();
		let at_hash = at.unwrap_or_else(|| self.client.info().best_hash);

		let res = api
			.balance_of(at_hash, pot_name)
			.map_err(|e| map_err(e, "Unable to query pot balance."))?;

		let res = res.map_err(|e| map_err(<&str>::from(e), "Query pot balance error."))?;

		Ok(res)
	}

	fn base_balance(&self, at: Option<Block::Hash>) -> RpcResult<u128> {
		let api = self.client.runtime_api();
		let at_hash = at.unwrap_or_else(|| self.client.info().best_hash);

		let res = api
			.balance_of_base(at_hash)
			.map_err(|e| map_err(e, "Unable to query base balance."))?;

		let res = res.map_err(|e| map_err(<&str>::from(e), "Query base balance error."))?;

		Ok(res)
	}
}

fn map_err(error: impl ToString, desc: &'static str) -> ErrorObjectOwned {
	//CallError::Custom(ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string())))
	ErrorObject::owned(Error::RuntimeError.into(), desc, Some(error.to_string()))
}
