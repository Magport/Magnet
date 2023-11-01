#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::{vec, vec::Vec};
use core::marker::PhantomData;
use fp_evm::{ExitError, ExitRevert, ExitSucceed, Precompile, PrecompileFailure};
use fp_evm::{PrecompileHandle, PrecompileOutput, PrecompileResult};
use frame_support::traits::{Currency, ExistenceRequirement};
use pallet_evm::AddressMapping;
use precompile_utils::prelude::*;
use sp_core::{
	crypto::{AccountId32, Ss58Codec},
	Get, U256,
};
use sp_runtime::traits::UniqueSaturatedInto;

struct MaxSize;
impl Get<u32> for MaxSize {
	fn get() -> u32 {
		256u32
	}
}

pub struct SubstrateUtils<T> {
	_marker: PhantomData<T>,
}

impl<T: pallet_evm::Config> Precompile for SubstrateUtils<T>
where
	U256: UniqueSaturatedInto<pallet_evm::BalanceOf<T>>,
	T::AccountId: From<AccountId32>,
{
	fn execute(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let selector = handle.read_u32_selector()?;

		let selector_bytes: [u8; 4] = sp_io::hashing::keccak_256(b"transferToSubstrate(string)")
			[0..4]
			.try_into()
			.map_err(|_| PrecompileFailure::Error {
				exit_status: ExitError::Other(
					"get transferToSubstrate selector_bytes failed".into(),
				),
			})?;
		let transfer_selector = u32::from_be_bytes(selector_bytes);

		match selector {
			a if a == transfer_selector => _ = Self::transfer_to_substrate(handle)?,
			_ => {
				return Err(PrecompileFailure::Revert {
					exit_status: ExitRevert::Reverted,
					output: "Not find the selector error".into(),
				})
			},
		}

		Ok(PrecompileOutput { exit_status: ExitSucceed::Returned, output: Vec::new() })
	}
}

impl<T: pallet_evm::Config> SubstrateUtils<T>
where
	U256: UniqueSaturatedInto<pallet_evm::BalanceOf<T>>,
	T::AccountId: From<AccountId32>,
{
	fn transfer_to_substrate(handle: &mut impl PrecompileHandle) -> PrecompileResult {
		let code_address = handle.code_address();
		let input = handle.input();
		let target_gas = handle.gas_limit();
		let context = handle.context();

		let caller = context.caller.clone();
		let code_origin = T::AddressMapping::into_account_id(code_address);
		let value = context.apparent_value;
		let to_who_bstr = solidity::decode_arguments::<BoundedString<MaxSize>>(&input[4..])?;

		if handle.is_static() {
			return Err(PrecompileFailure::Error {
				exit_status: ExitError::Other("Can't be static call error".into()),
			});
		}

		let to_who_ss58 =
			String::try_from(to_who_bstr.clone()).map_err(|_| PrecompileFailure::Error {
				exit_status: ExitError::Other("String try from BoundedString failed".into()),
			})?;

		let to_who_id32 =
			AccountId32::from_ss58check(&to_who_ss58).map_err(|_| PrecompileFailure::Error {
				exit_status: ExitError::Other("AccountId32 from ss58check(string) failed".into()),
			})?;

		let to_who: <T>::AccountId = to_who_id32.clone().into();

		//Base gas 3000 + write balance gas 5000 + write substrate gas
		//write balance from zero 20000 gas and
		//a 15,000 gas refund when a non-zero value is set to zero.
		let mut gas_cost: u64 = 3000 + 5000 + 20000;
		let log_costs = precompile_utils::evm::costs::log_costs(4, 32)?;

		gas_cost = gas_cost + log_costs;

		if let Some(gas) = target_gas {
			if gas <= gas_cost {
				return Err(PrecompileFailure::Error { exit_status: ExitError::OutOfGas });
			}
		}

		handle.record_cost(gas_cost)?;

		T::Currency::transfer(
			&code_origin,
			&to_who,
			value.unique_saturated_into(),
			ExistenceRequirement::AllowDeath,
		)
		.map_err(|_| PrecompileFailure::Error {
			exit_status: ExitError::Other("Currency transfer failed".into()),
		})?;

		let event = sp_io::hashing::keccak_256(b"TransferOut(address,string,uint256,string)");
		let ss58_bytes = to_who_ss58.as_bytes();
		let ss58_hash = sp_io::hashing::keccak_256(ss58_bytes);
		let ss58_arg = solidity::encode_arguments::<BoundedString<MaxSize>>(to_who_bstr);
		let mut value_bytes: [u8; 32] = [0u8; 32];
		value.to_big_endian(&mut value_bytes);
		handle.log(
			code_address,
			vec![event.into(), caller.into(), ss58_hash.into(), value_bytes.into()],
			ss58_arg.into(),
		)?;

		Ok(PrecompileOutput { exit_status: ExitSucceed::Returned, output: Vec::new() })
	}
}
