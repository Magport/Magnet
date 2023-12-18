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

#![cfg(test)]

use super::*;

use crate::mock::*;
use frame_support::assert_ok;
use pallet_evm::{CallInfo, Error, ExitError, ExitReason, Runner};

struct MaxSize;
impl Get<u32> for MaxSize {
	fn get() -> u32 {
		256u32
	}
}

#[test]
fn transfer_to_substrate_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let bob_evm: H160 = H160::from_low_u64_be(123_123_123_123_123);
		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);

		let alice_value_before = Balances::free_balance(&ALICE);
		let bob_evm_value_before: u64 = bob_evm_account.balance.as_u64();
		let transfer_value = 800_000_000_000_000u64;

		let target: H160 = H160::from_low_u64_be(2048);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let selector_bytes: [u8; 4] = sp_io::hashing::keccak_256(b"transferToSubstrate(string)")
			[0..4]
			.try_into()
			.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);
		let call_data = solidity::encode_with_selector(selector, alice_ss58_bstring);

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			transfer_value.into(),
			91776,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			Some(U256::from(0)),
			Vec::new(),
			is_transactional,
			validate,
			None,
			None,
			<Test as pallet_evm::Config>::config(),
		);
		assert_ok!(&call_result);
		let call_result = call_result.unwrap();

		let used_gas: u64;
		match call_result {
			CallInfo { exit_reason: ExitReason::Succeed(_), value: _, used_gas: gas, .. } => {
				used_gas = gas.effective.as_u64();
			},
			CallInfo { exit_reason: reason, value: err_value, .. } => {
				println!("error : {:?}", std::str::from_utf8(&err_value));
				panic!("Call transferToSubstrate failed!({:?})", reason);
			},
		};

		let alice_value_after = Balances::free_balance(&ALICE);
		assert_eq!(alice_value_before + transfer_value, alice_value_after);
		let gas_fee: u64 = used_gas * 1_000_000_000;
		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);
		let bob_evm_value_after: u64 = bob_evm_account.balance.as_u64();
		assert_eq!(bob_evm_value_before, bob_evm_value_after + transfer_value + gas_fee);
	})
}

#[test]
fn gas_not_enoght_error_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let bob_evm: H160 = H160::from_low_u64_be(123_123_123_123_123);
		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000);

		let transfer_value = 800_000_000_000_000u64;

		let target: H160 = H160::from_low_u64_be(2048);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let selector_bytes: [u8; 4] = sp_io::hashing::keccak_256(b"transferToSubstrate(string)")
			[0..4]
			.try_into()
			.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);
		let call_data = solidity::encode_with_selector(selector, alice_ss58_bstring);

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			transfer_value.into(),
			917,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			Some(U256::from(0)),
			Vec::new(),
			is_transactional,
			validate,
			None,
			None,
			<Test as pallet_evm::Config>::config(),
		);
		assert!(call_result.is_err());
		let err = call_result.unwrap_err().error;
		match err {
			Error::<Test>::GasLimitTooLow => assert!(true),
			_ => panic!("Not GasLimitTooLow but {:?}", err),
		}
	})
}

#[test]
fn gas_price_too_low_error_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let bob_evm: H160 = H160::from_low_u64_be(123_123_123_123_123);
		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000);

		let transfer_value = 800_000_000_000_000u64;

		let target: H160 = H160::from_low_u64_be(2048);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let selector_bytes: [u8; 4] = sp_io::hashing::keccak_256(b"transferToSubstrate(string)")
			[0..4]
			.try_into()
			.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);
		let call_data = solidity::encode_with_selector(selector, alice_ss58_bstring);

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			transfer_value.into(),
			91776,
			Some(U256::from(1_000)),
			Some(U256::default()),
			Some(U256::from(0)),
			Vec::new(),
			is_transactional,
			validate,
			None,
			None,
			<Test as pallet_evm::Config>::config(),
		);
		assert!(call_result.is_err());
		let err = call_result.unwrap_err().error;
		match err {
			Error::<Test>::GasPriceTooLow => assert!(true),
			_ => panic!("Not GasPriceTooLow but {:?}", err),
		}
	})
}

#[test]
fn balance_not_enoght_error_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let bob_evm: H160 = H160::from_low_u64_be(123_123_123_123_123);
		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000);

		let transfer_value = 800_000_000_000_000_000u64;

		let target: H160 = H160::from_low_u64_be(2048);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let selector_bytes: [u8; 4] = sp_io::hashing::keccak_256(b"transferToSubstrate(string)")
			[0..4]
			.try_into()
			.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);
		let call_data = solidity::encode_with_selector(selector, alice_ss58_bstring);

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			transfer_value.into(),
			91776,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			Some(U256::from(0)),
			Vec::new(),
			is_transactional,
			validate,
			None,
			None,
			<Test as pallet_evm::Config>::config(),
		);
		assert!(call_result.is_err());
		let err = call_result.unwrap_err().error;
		match err {
			Error::<Test>::BalanceLow => assert!(true),
			_ => panic!("Not BalanceLow but {:?}", err),
		}
	})
}

#[test]
fn selector_error_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let bob_evm: H160 = H160::from_low_u64_be(123_123_123_123_123);
		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000);

		let transfer_value = 800_000_000_000_000u64;

		let target: H160 = H160::from_low_u64_be(2048);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let selector_bytes: [u8; 4] = sp_io::hashing::keccak_256(b"123transferToSubstrate(string)")
			[0..4]
			.try_into()
			.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);
		let call_data = solidity::encode_with_selector(selector, alice_ss58_bstring);

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			transfer_value.into(),
			91776,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			Some(U256::from(0)),
			Vec::new(),
			is_transactional,
			validate,
			None,
			None,
			<Test as pallet_evm::Config>::config(),
		);
		assert_ok!(&call_result);
		let call_result = call_result.unwrap();

		match call_result {
			CallInfo { exit_reason: ExitReason::Succeed(_), value: _, used_gas: _, .. } => {
				panic!("exit_reason must not Succeed!");
			},
			CallInfo { exit_reason: _, value: err_value, .. } => {
				assert_eq!(err_value, "Not find the selector error".as_bytes().to_owned());
			},
		};
	})
}

#[test]
fn ss58address_error_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let bob_evm: H160 = H160::from_low_u64_be(123_123_123_123_123);
		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000);

		let transfer_value = 800_000_000_000_000u64;

		let target: H160 = H160::from_low_u64_be(2048);

		//let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from("1234567890");

		let selector_bytes: [u8; 4] = sp_io::hashing::keccak_256(b"transferToSubstrate(string)")
			[0..4]
			.try_into()
			.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);
		let call_data = solidity::encode_with_selector(selector, alice_ss58_bstring);

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			transfer_value.into(),
			91776,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			Some(U256::from(0)),
			Vec::new(),
			is_transactional,
			validate,
			None,
			None,
			<Test as pallet_evm::Config>::config(),
		);
		assert_ok!(&call_result);
		let call_result = call_result.unwrap();

		match call_result {
			CallInfo { exit_reason: ExitReason::Succeed(_), value: _, used_gas: _, .. } => {
				panic!("exit_reason must not Succeed!");
			},
			CallInfo { exit_reason: reason, value: _, .. } => {
				assert_eq!(
					reason,
					ExitReason::Error(ExitError::Other(std::borrow::Cow::Borrowed(
						"AccountId32 from ss58check(string) failed"
					)))
				);
			},
		};
	})
}
