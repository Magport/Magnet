use pallet_evm::{
	ExitRevert, IsPrecompileResult, Precompile, PrecompileFailure, PrecompileHandle,
	PrecompileResult, PrecompileSet,
};
use sp_core::{crypto::AccountId32, H160, U256};
use sp_runtime::traits::UniqueSaturatedInto;
use sp_std::marker::PhantomData;

use pallet_evm_precompile_modexp::Modexp;
use pallet_evm_precompile_sha3fips::Sha3FIPS256;
use pallet_evm_precompile_simple::{ECRecover, ECRecoverPublicKey, Identity, Ripemd160, Sha256};
use pallet_precompile_substrate_utils::SubstrateUtils;
use pallet_precompile_transfer_to_magnet::TransferToMagnet;

pub struct FrontierPrecompiles<R>(PhantomData<R>);

impl<R> FrontierPrecompiles<R>
where
	R: pallet_evm::Config,
{
	pub fn new() -> Self {
		Self(Default::default())
	}
	pub fn used_addresses() -> [H160; 9] {
		[
			hash(1),
			hash(2),
			hash(3),
			hash(4),
			hash(5),
			hash(1024),
			hash(1025),
			hash(2048),
			hash(2049),
		]
	}
}
impl<R> PrecompileSet for FrontierPrecompiles<R>
where
	R: pallet_evm::Config
		+ pallet_assets_bridge::Config
		+ pallet_assets::Config<AssetIdParameter = codec::Compact<u32>>,
	R::AccountId: From<AccountId32>,
	R::AssetId: From<u32> + Into<u32>,
	<R as pallet_assets::Config>::Balance: From<u128>,
	U256: UniqueSaturatedInto<pallet_evm::BalanceOf<R>>,
{
	fn execute(&self, handle: &mut impl PrecompileHandle) -> Option<PrecompileResult> {
		let remaining_gas = handle.remaining_gas();

		let is_precompile_result = self.is_precompile(handle.code_address(), remaining_gas);
		if let IsPrecompileResult::Answer { is_precompile, .. } = is_precompile_result {
			if is_precompile
				&& handle.code_address() > hash(5)
				&& handle.code_address() != handle.context().address
			{
				return Some(Err(PrecompileFailure::Revert {
					exit_status: ExitRevert::Reverted,
					output: "cannot be called with DELEGATECALL or CALLCODE".into(),
				}));
			}
		}

		match handle.code_address() {
			// Ethereum precompiles :
			a if a == hash(1) => Some(ECRecover::execute(handle)),
			a if a == hash(2) => Some(Sha256::execute(handle)),
			a if a == hash(3) => Some(Ripemd160::execute(handle)),
			a if a == hash(4) => Some(Identity::execute(handle)),
			a if a == hash(5) => Some(Modexp::execute(handle)),
			// Non-Frontier specific nor Ethereum precompiles :
			a if a == hash(1024) => Some(Sha3FIPS256::execute(handle)),
			a if a == hash(1025) => Some(ECRecoverPublicKey::execute(handle)),
			a if a == hash(2048) => Some(SubstrateUtils::<R>::execute(handle)),
			a if a == hash(2049) => Some(TransferToMagnet::<R>::execute(handle)),
			_ => None,
		}
	}

	fn is_precompile(&self, address: H160, _gas: u64) -> IsPrecompileResult {
		IsPrecompileResult::Answer {
			is_precompile: Self::used_addresses().contains(&address),
			extra_cost: 0,
		}
	}
}

fn hash(a: u64) -> H160 {
	H160::from_low_u64_be(a)
}
