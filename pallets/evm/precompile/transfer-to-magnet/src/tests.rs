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
use frame_support::traits::Currency;
use pallet_evm::{AddressMapping, CallInfo, Error, ExitError, ExitReason, Runner};

use codec::Encode;
use sp_core::H160;

struct MaxSize;
impl Get<u32> for MaxSize {
	fn get() -> u32 {
		256u32
	}
}

fn deploy_contract(caller: H160) -> H160 {
	let bytecode_path = "./contracts/AssetsBridgeErc20_ByteCode.txt";

	let mut bytecode_str =
		std::fs::read_to_string(bytecode_path).expect("Unable to read bytecode file");
	log::info!("First 10 characters of bytecode: {}", &bytecode_str[..10]);

	bytecode_str.retain(|c| !c.is_whitespace());
	if bytecode_str.starts_with("0x") {
		if (bytecode_str.len() - 2) % 2 != 0 {
			panic!("Bytecode has an odd length, ensure it is a valid hex string.");
		}
		bytecode_str = bytecode_str[2..].to_string();
	} else if bytecode_str.len() % 2 != 0 {
		panic!("Bytecode has an odd length, ensure it is a valid hex string.");
	}

	let input = hex::decode(&bytecode_str).expect("Failed to decode hex bytecode");

	let gas_limit = 10000000u64;
	let value = U256::zero();

	let create_result = <Test as pallet_evm::Config>::Runner::create(
		caller,
		input,
		value,
		gas_limit,
		None,
		None,
		None,
		Vec::new(),
		false,
		true,
		None,
		None,
		<Test as pallet_evm::Config>::config(),
	)
	.expect("contract creation runs successfully");

	let balance = query_balance_of(create_result.value, caller, caller);
	log::info!("balance of caller:{:?}", balance);

	create_result.value
}

fn create_and_register_asset(erc20_address: H160) -> u32 {
	let asset_id = 3u32;
	let root_origin: frame_system::Origin<Test> = frame_system::RawOrigin::Root;
	let alice_balance = Balances::free_balance(&ALICE);
	log::info!("create and register asset -> alice balance:{:?}", alice_balance);

	let alice_origin: frame_system::Origin<Test> = frame_system::Origin::<Test>::Signed(ALICE);

	assert_ok!(Assets::force_create(
		root_origin.clone().into(),
		asset_id.into(),
		ALICE.into(),
		false,
		1u128.into()
	));
	assert_ok!(AssetsBridge::set_admin(root_origin.into(), ALICE));
	assert_ok!(AssetsBridge::register(alice_origin.clone().into(), asset_id, erc20_address));

	let bound_assets_id = pallet_assets_bridge::AssetIds::<Test>::get(erc20_address)
		.expect("Failed to get bound assets id.");
	log::info!("bound assets id succeed:{:?}", bound_assets_id);

	asset_id
}

fn create_without_register_asset() -> u32 {
	let asset_id = 3u32;
	let root_origin: frame_system::Origin<Test> = frame_system::RawOrigin::Root;
	let alice_balance = Balances::free_balance(&ALICE);
	log::info!("create assets without register asset -> alice balance:{:?}", alice_balance);

	assert_ok!(Assets::force_create(
		root_origin.clone().into(),
		asset_id.into(),
		ALICE.into(),
		false,
		1u128.into()
	));
	assert_ok!(AssetsBridge::set_admin(root_origin.into(), ALICE));

	asset_id
}

fn mint_erc20_tokens(erc20_address: H160, recipient: H160, amount: u128, origin: H160) {
	let selector_bytes: [u8; 4] = sp_io::hashing::keccak_256(b"mint_into(address,uint256)")[0..4]
		.try_into()
		.unwrap();

	let mint_selector = u32::from_be_bytes(selector_bytes);

	let recipient_encoded = recipient.to_fixed_bytes();
	let amount_encoded = solidity::encode_arguments(U256::from(amount));

	let mut input = mint_selector.to_be_bytes().to_vec();
	input.extend_from_slice(&[0u8; 12][..]);
	input.extend_from_slice(&recipient_encoded);
	//input.extend_from_slice(&[0u8; 16][..]);
	input.extend_from_slice(&amount_encoded);
	log::info!("mint erc20 tokens input:{:?}", hex::encode(&input));

	let gas_limit = 10000000u64;

	assert_ok!(<Test as pallet_evm::Config>::Runner::call(
		origin.into(),
		erc20_address,
		input,
		U256::zero(),
		gas_limit,
		None,
		None,
		None,
		Vec::new(),
		false,
		true,
		None,
		None,
		<Test as pallet_evm::Config>::config(),
	));
}

fn burn_erc20_tokens(
	erc20_address: H160,
	account: H160,
	amount: u128,
	origin: H160,
) -> Result<(), String> {
	let selector_bytes: [u8; 4] = sp_io::hashing::keccak_256(b"burn_from(address,uint256)")[0..4]
		.try_into()
		.unwrap();

	let burn_selector = u32::from_be_bytes(selector_bytes);

	let account_encoded = account.to_fixed_bytes();
	let amount_encoded = solidity::encode_arguments(U256::from(amount));

	let mut input = burn_selector.to_be_bytes().to_vec();
	input.extend_from_slice(&[0u8; 12][..]);
	input.extend_from_slice(&account_encoded);
	//input.extend_from_slice(&[0u8; 16][..]);
	input.extend_from_slice(&amount_encoded);

	let gas_limit = 10000000u64;

	let call_result = <Test as pallet_evm::Config>::Runner::call(
		origin.into(),
		erc20_address,
		input,
		U256::zero(),
		gas_limit,
		None,
		None,
		None,
		Vec::new(),
		false,
		true,
		None,
		None,
		<Test as pallet_evm::Config>::config(),
	);

	match call_result {
		Ok(info) => match info.exit_reason {
			pallet_evm::ExitReason::Succeed(_) => Ok(()),
			pallet_evm::ExitReason::Revert(reason) => {
				log::info!("reason:{:?}.", reason);
				let error_message = String::from_utf8(reason.encode())
					.unwrap_or_else(|_| "Failed to decode error message".to_string());
				Err(error_message)
			},
			_ => Err("Transaction failed for an unknown reason".to_string()),
		},
		Err(_) => Err("EVM call failed".to_string()),
	}
}

fn query_balance_of(erc20_address: H160, account: H160, origin: H160) -> U256 {
	let selector_bytes: [u8; 4] =
		sp_io::hashing::keccak_256(b"balanceOf(address)")[0..4].try_into().unwrap();

	let account_encoded = account.to_fixed_bytes();

	let mut input = selector_bytes.to_vec();
	input.extend_from_slice(&[0u8; 12][..]);
	input.extend_from_slice(&account_encoded);
	log::info!("Query balance input: {:?}", hex::encode(&input));

	let gas_limit = 10000000u64;

	let result = <Test as pallet_evm::Config>::Runner::call(
		origin.into(),
		erc20_address,
		input,
		U256::zero(),
		gas_limit,
		None,
		None,
		None,
		Vec::new(),
		false,
		true,
		None,
		None,
		<Test as pallet_evm::Config>::config(),
	)
	.expect("EVM call BalanceOf failed.");

	U256::from_big_endian(&result.value)
}

#[test]
fn transfer_to_substrate_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		//let bob_evm= H160::from_slice(&[17u8;20][..]);
		let bob_evm: H160 = H160([
			0x05, 0xF9, 0xb8, 0xC7, 0x6E, 0x89, 0x87, 0xB8, 0x15, 0xC9, 0x3C, 0x27, 0xD1, 0x45,
			0x20, 0xb6, 0xeD, 0x57, 0x39, 0x02,
		]);
		log::info!("bob evm:{:?}", bob_evm);

		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000_000);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);

		let bob_evm_value_before: u128 = bob_evm_account.balance.as_u128();
		log::info!("bob evm value before:{:?}", bob_evm_value_before);

		let mint_amount: u128 = 1000_000_000_000_000_000;
		let transfer_token_amount: u128 = 800_000_000_000_000_000;

		let target: H160 = H160::from_low_u64_be(2049);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let token_addr = deploy_contract(bob_evm);
		log::info!("token addr:{:?}", token_addr);

		let asset_id = create_and_register_asset(token_addr);
		log::info!("asset id:{:?}", asset_id);

		let alice_token_amount_before = Assets::balance(asset_id, &ALICE);
		log::info!("alice token amount before mint:{:?}", alice_token_amount_before);

		let bob_evm_token_before_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("before mint, bob evm token balance:{:?}", bob_evm_token_before_mint);
		mint_erc20_tokens(token_addr, bob_evm, mint_amount, bob_evm);
		let bob_evm_token_after_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after mint, bob evm token balance:{:?}", bob_evm_token_after_mint);

		match burn_erc20_tokens(token_addr, bob_evm, transfer_token_amount, bob_evm) {
			Ok(_) => log::info!("Token burned."),
			Err(e) => {
				panic!("burn token execution reverted:{:?}.", e);
			},
		};

		let bob_evm_token_after_burn: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after burn, bob evm token balance:{:?}", bob_evm_token_after_burn);

		let selector_bytes: [u8; 4] =
			sp_io::hashing::keccak_256(b"transferToMagnet(address,uint256,string)")[0..4]
				.try_into()
				.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);

		let token_addr_encoded = token_addr.to_fixed_bytes();
		let transfer_value_encoded = solidity::encode_arguments(transfer_token_amount);
		let alice_ss58_address_encoded = solidity::encode_arguments(alice_ss58_bstring);
		let alice_ss58_address_len: u128 = alice_ss58_address_encoded.len().try_into().unwrap();
		let alice_ss58_address_len_encoded = solidity::encode_arguments(alice_ss58_address_len);
		log::info!("alice ss58 address encoded length:{:?}", alice_ss58_address_len);

		let mut call_data = selector.to_be_bytes().to_vec();
		call_data.extend_from_slice(&[0u8; 12][..]);
		call_data.extend_from_slice(&token_addr_encoded);
		call_data.extend_from_slice(&transfer_value_encoded);
		call_data.extend_from_slice(&alice_ss58_address_len_encoded);
		call_data.extend_from_slice(&alice_ss58_address_encoded);
		log::info!("transferToMagnet callData:{:?}", hex::encode(&call_data.clone()));

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			0.into(),
			3000000,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			None,
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
			CallInfo { exit_reason: ExitReason::Succeed(_), value: _, used_gas: gas, .. } => {
				log::info!("Transfer to magnet succeed. used gas:{:?}", gas);
			},
			CallInfo { exit_reason: reason, value: err_value, .. } => {
				log::error!("error : {:?}", std::str::from_utf8(&err_value));
				panic!("Call transferToMagnet failed!({:?})", reason);
			},
		};

		let alice_token_amount_after = Assets::balance(asset_id, &ALICE);
		log::info!("alice token amount after mint:{:?}", alice_token_amount_after);

		assert_eq!(alice_token_amount_before + transfer_token_amount, alice_token_amount_after);
		//let (bob_evm_account, _) = EVM::account_basic(&bob_evm);
		assert_eq!(bob_evm_token_after_mint, bob_evm_token_after_burn + transfer_token_amount);
	})
}

#[test]
fn gas_not_enough_error_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		//let bob_evm= H160::from_slice(&[17u8;20][..]);
		let bob_evm: H160 = H160([
			0x05, 0xF9, 0xb8, 0xC7, 0x6E, 0x89, 0x87, 0xB8, 0x15, 0xC9, 0x3C, 0x27, 0xD1, 0x45,
			0x20, 0xb6, 0xeD, 0x57, 0x39, 0x02,
		]);
		log::info!("bob evm:{:?}", bob_evm);

		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000_000);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);

		let bob_evm_value_before: u128 = bob_evm_account.balance.as_u128();
		log::info!("bob evm value before:{:?}", bob_evm_value_before);

		let mint_amount: u128 = 1000_000_000_000_000_000;
		let transfer_token_amount: u128 = 800_000_000_000_000_000;

		let target: H160 = H160::from_low_u64_be(2049);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let token_addr = deploy_contract(bob_evm);
		log::info!("token addr:{:?}", token_addr);

		let asset_id = create_and_register_asset(token_addr);
		log::info!("asset id:{:?}", asset_id);

		let alice_token_amount_before = Assets::balance(asset_id, &ALICE);
		log::info!("alice token amount before mint:{:?}", alice_token_amount_before);

		let bob_evm_token_before_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("before mint, bob evm token balance:{:?}", bob_evm_token_before_mint);
		mint_erc20_tokens(token_addr, bob_evm, mint_amount, bob_evm);
		let bob_evm_token_after_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after mint, bob evm token balance:{:?}", bob_evm_token_after_mint);

		match burn_erc20_tokens(token_addr, bob_evm, transfer_token_amount, bob_evm) {
			Ok(_) => log::info!("Token burned."),
			Err(e) => {
				panic!("burn token execution reverted:{:?}.", e);
			},
		}

		let bob_evm_token_after_burn: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after burn, bob evm token balance:{:?}", bob_evm_token_after_burn);

		let selector_bytes: [u8; 4] =
			sp_io::hashing::keccak_256(b"transferToMagnet(address,uint256,string)")[0..4]
				.try_into()
				.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);

		let token_addr_encoded = token_addr.to_fixed_bytes();
		let transfer_value_encoded = solidity::encode_arguments(transfer_token_amount);
		let alice_ss58_address_encoded = solidity::encode_arguments(alice_ss58_bstring);
		let alice_ss58_address_len: u128 = alice_ss58_address_encoded.len().try_into().unwrap();
		let alice_ss58_address_len_encoded = solidity::encode_arguments(alice_ss58_address_len);
		log::info!("alice ss58 address encoded length:{:?}", alice_ss58_address_len);

		let mut call_data = selector.to_be_bytes().to_vec();
		call_data.extend_from_slice(&[0u8; 12][..]);
		call_data.extend_from_slice(&token_addr_encoded);
		call_data.extend_from_slice(&transfer_value_encoded);
		call_data.extend_from_slice(&alice_ss58_address_len_encoded);
		call_data.extend_from_slice(&alice_ss58_address_encoded);
		log::info!("transferToMagnet callData:{:?}", hex::encode(&call_data.clone()));

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			0.into(),
			300,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			None,
			Vec::new(),
			is_transactional,
			validate,
			None,
			None,
			<Test as pallet_evm::Config>::config(),
		);
		assert!(call_result.is_err());
		let err = call_result.unwrap_err().error;
		log::info!("test gas limit too low err:{:?}", err);
		match err {
			Error::<Test>::GasLimitTooLow => assert!(true),
			_ => panic!("Not GasLimitTooLow but {:?}", err),
		}
	})
}

#[test]
fn gas_price_too_low_error_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		//let bob_evm= H160::from_slice(&[17u8;20][..]);
		let bob_evm: H160 = H160([
			0x05, 0xF9, 0xb8, 0xC7, 0x6E, 0x89, 0x87, 0xB8, 0x15, 0xC9, 0x3C, 0x27, 0xD1, 0x45,
			0x20, 0xb6, 0xeD, 0x57, 0x39, 0x02,
		]);
		log::info!("bob evm:{:?}", bob_evm);

		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000_000);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);

		let bob_evm_value_before: u128 = bob_evm_account.balance.as_u128();
		log::info!("bob evm value before:{:?}", bob_evm_value_before);

		let mint_amount: u128 = 1000_000_000_000_000_000;
		let transfer_token_amount: u128 = 800_000_000_000_000_000;

		let target: H160 = H160::from_low_u64_be(2049);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let token_addr = deploy_contract(bob_evm);
		log::info!("token addr:{:?}", token_addr);

		let asset_id = create_and_register_asset(token_addr);
		log::info!("asset id:{:?}", asset_id);

		let alice_token_amount_before = Assets::balance(asset_id, &ALICE);
		log::info!("alice token amount before mint:{:?}", alice_token_amount_before);

		let bob_evm_token_before_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("before mint, bob evm token balance:{:?}", bob_evm_token_before_mint);
		mint_erc20_tokens(token_addr, bob_evm, mint_amount, bob_evm);
		let bob_evm_token_after_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after mint, bob evm token balance:{:?}", bob_evm_token_after_mint);

		match burn_erc20_tokens(token_addr, bob_evm, transfer_token_amount, bob_evm) {
			Ok(_) => log::info!("Tokens were successfully burned."),
			Err(e) => {
				panic!("burn token execution reverted:{:?}.", e);
			},
		}
		let bob_evm_token_after_burn: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after burn, bob evm token balance:{:?}", bob_evm_token_after_burn);

		let selector_bytes: [u8; 4] =
			sp_io::hashing::keccak_256(b"transferToMagnet(address,uint256,string)")[0..4]
				.try_into()
				.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);

		let token_addr_encoded = token_addr.to_fixed_bytes();
		let transfer_value_encoded = solidity::encode_arguments(transfer_token_amount);
		let alice_ss58_address_encoded = solidity::encode_arguments(alice_ss58_bstring);
		let alice_ss58_address_len: u128 = alice_ss58_address_encoded.len().try_into().unwrap();
		let alice_ss58_address_len_encoded = solidity::encode_arguments(alice_ss58_address_len);
		log::info!("alice ss58 address encoded length:{:?}", alice_ss58_address_len);

		let mut call_data = selector.to_be_bytes().to_vec();
		call_data.extend_from_slice(&[0u8; 12][..]);
		call_data.extend_from_slice(&token_addr_encoded);
		call_data.extend_from_slice(&transfer_value_encoded);
		call_data.extend_from_slice(&alice_ss58_address_len_encoded);
		call_data.extend_from_slice(&alice_ss58_address_encoded);
		log::info!("transferToMagnet callData:{:?}", hex::encode(&call_data.clone()));

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			0.into(),
			3_000_000,
			Some(U256::from(1_000)),
			Some(U256::default()),
			None,
			Vec::new(),
			is_transactional,
			validate,
			None,
			None,
			<Test as pallet_evm::Config>::config(),
		);
		assert!(call_result.is_err());
		let err = call_result.unwrap_err().error;
		log::info!("test gas price too low err:{:?}", err);
		match err {
			Error::<Test>::GasPriceTooLow => assert!(true),
			_ => panic!("Not GasPriceTooLow but {:?}", err),
		}
	})
}

#[test]
fn balance_not_enough_error_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		//let bob_evm= H160::from_slice(&[17u8;20][..]);
		let bob_evm: H160 = H160([
			0x05, 0xF9, 0xb8, 0xC7, 0x6E, 0x89, 0x87, 0xB8, 0x15, 0xC9, 0x3C, 0x27, 0xD1, 0x45,
			0x20, 0xb6, 0xeD, 0x57, 0x39, 0x02,
		]);
		log::info!("bob evm:{:?}", bob_evm);

		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000_000);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);

		let bob_evm_value_before: u128 = bob_evm_account.balance.as_u128();
		log::info!("bob evm value before:{:?}", bob_evm_value_before);

		let mint_amount: u128 = 1000_000_000_000_000_000;
		let transfer_token_amount: u128 = 1800_000_000_000_000_000;

		let token_addr = deploy_contract(bob_evm);
		log::info!("token addr:{:?}", token_addr);

		let asset_id = create_and_register_asset(token_addr);
		log::info!("asset id:{:?}", asset_id);

		let alice_token_amount_before = Assets::balance(asset_id, &ALICE);
		log::info!("alice token amount before mint:{:?}", alice_token_amount_before);

		let bob_evm_token_before_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("before mint, bob evm token balance:{:?}", bob_evm_token_before_mint);
		mint_erc20_tokens(token_addr, bob_evm, mint_amount, bob_evm);
		let bob_evm_token_after_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after mint, bob evm token balance:{:?}", bob_evm_token_after_mint);

		match burn_erc20_tokens(token_addr, bob_evm, transfer_token_amount, bob_evm) {
			Ok(_) => panic!("transfer must revert."),
			Err(_) => {
				log::info!("burn token execution reverted.");
			},
		};
	})
}

#[test]
fn selector_error_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		//let bob_evm= H160::from_slice(&[17u8;20][..]);
		let bob_evm: H160 = H160([
			0x05, 0xF9, 0xb8, 0xC7, 0x6E, 0x89, 0x87, 0xB8, 0x15, 0xC9, 0x3C, 0x27, 0xD1, 0x45,
			0x20, 0xb6, 0xeD, 0x57, 0x39, 0x02,
		]);
		log::info!("bob evm:{:?}", bob_evm);

		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000_000);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);

		let bob_evm_value_before: u128 = bob_evm_account.balance.as_u128();
		log::info!("bob evm value before:{:?}", bob_evm_value_before);

		let mint_amount: u128 = 1000_000_000_000_000_000;
		let transfer_token_amount: u128 = 800_000_000_000_000_000;

		let target: H160 = H160::from_low_u64_be(2049);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let token_addr = deploy_contract(bob_evm);
		log::info!("token addr:{:?}", token_addr);

		let asset_id = create_and_register_asset(token_addr);
		log::info!("asset id:{:?}", asset_id);

		let alice_token_amount_before = Assets::balance(asset_id, &ALICE);
		log::info!("alice token amount before mint:{:?}", alice_token_amount_before);

		let bob_evm_token_before_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("before mint, bob evm token balance:{:?}", bob_evm_token_before_mint);
		mint_erc20_tokens(token_addr, bob_evm, mint_amount, bob_evm);
		let bob_evm_token_after_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after mint, bob evm token balance:{:?}", bob_evm_token_after_mint);

		match burn_erc20_tokens(token_addr, bob_evm, transfer_token_amount, bob_evm) {
			Ok(_) => log::info!("Token burned."),
			Err(e) => {
				panic!("burn token execution reverted:{:?}.", e);
			},
		}

		let bob_evm_token_after_burn: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after burn, bob evm token balance:{:?}", bob_evm_token_after_burn);

		let selector_bytes: [u8; 4] =
			sp_io::hashing::keccak_256(b"111transferToMagnet(address,uint256,string)")[0..4]
				.try_into()
				.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);

		let token_addr_encoded = token_addr.to_fixed_bytes();
		let transfer_value_encoded = solidity::encode_arguments(transfer_token_amount);
		let alice_ss58_address_encoded = solidity::encode_arguments(alice_ss58_bstring);
		let alice_ss58_address_len: u128 = alice_ss58_address_encoded.len().try_into().unwrap();
		let alice_ss58_address_len_encoded = solidity::encode_arguments(alice_ss58_address_len);
		log::info!("alice ss58 address encoded length:{:?}", alice_ss58_address_len);

		let mut call_data = selector.to_be_bytes().to_vec();
		call_data.extend_from_slice(&[0u8; 12][..]);
		call_data.extend_from_slice(&token_addr_encoded);
		call_data.extend_from_slice(&transfer_value_encoded);
		call_data.extend_from_slice(&alice_ss58_address_len_encoded);
		call_data.extend_from_slice(&alice_ss58_address_encoded);
		log::info!("transferToMagnet callData:{:?}", hex::encode(&call_data.clone()));

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			0.into(),
			3_000_000,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			None,
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
				let v = unsafe { String::from_utf8_unchecked(err_value.clone()) };
				log::info!("exit value:{:?}", v);
				assert_eq!(err_value, "Not find the selector error".as_bytes().to_owned());
			},
		};
	})
}

#[test]
fn ss58address_error_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		//let bob_evm= H160::from_slice(&[17u8;20][..]);
		let bob_evm: H160 = H160([
			0x05, 0xF9, 0xb8, 0xC7, 0x6E, 0x89, 0x87, 0xB8, 0x15, 0xC9, 0x3C, 0x27, 0xD1, 0x45,
			0x20, 0xb6, 0xeD, 0x57, 0x39, 0x02,
		]);
		log::info!("bob evm:{:?}", bob_evm);

		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000_000);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);

		let bob_evm_value_before: u128 = bob_evm_account.balance.as_u128();
		log::info!("bob evm value before:{:?}", bob_evm_value_before);

		let mint_amount: u128 = 1000_000_000_000_000_000;
		let transfer_token_amount: u128 = 800_000_000_000_000_000;

		let target: H160 = H160::from_low_u64_be(2049);

		//let alice_ss58_address = ALICE.to_ss58check();
		//let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from("1234567890");

		let token_addr = deploy_contract(bob_evm);
		log::info!("token addr:{:?}", token_addr);

		let asset_id = create_and_register_asset(token_addr);
		log::info!("asset id:{:?}", asset_id);

		let alice_token_amount_before = Assets::balance(asset_id, &ALICE);
		log::info!("alice token amount before mint:{:?}", alice_token_amount_before);

		let bob_evm_token_before_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("before mint, bob evm token balance:{:?}", bob_evm_token_before_mint);
		mint_erc20_tokens(token_addr, bob_evm, mint_amount, bob_evm);
		let bob_evm_token_after_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after mint, bob evm token balance:{:?}", bob_evm_token_after_mint);

		match burn_erc20_tokens(token_addr, bob_evm, transfer_token_amount, bob_evm) {
			Ok(_) => log::info!("Token burned."),
			Err(e) => {
				panic!("burn token execution reverted:{:?}.", e);
			},
		}

		let bob_evm_token_after_burn: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after burn, bob evm token balance:{:?}", bob_evm_token_after_burn);

		let selector_bytes: [u8; 4] =
			sp_io::hashing::keccak_256(b"transferToMagnet(address,uint256,string)")[0..4]
				.try_into()
				.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);

		let token_addr_encoded = token_addr.to_fixed_bytes();
		let transfer_value_encoded = solidity::encode_arguments(transfer_token_amount);
		let alice_ss58_address_encoded = solidity::encode_arguments(alice_ss58_bstring);
		let alice_ss58_address_len: u128 = alice_ss58_address_encoded.len().try_into().unwrap();
		let alice_ss58_address_len_encoded = solidity::encode_arguments(alice_ss58_address_len);
		log::info!("alice ss58 address encoded length:{:?}", alice_ss58_address_len);

		let mut call_data = selector.to_be_bytes().to_vec();
		call_data.extend_from_slice(&[0u8; 12][..]);
		call_data.extend_from_slice(&token_addr_encoded);
		call_data.extend_from_slice(&transfer_value_encoded);
		call_data.extend_from_slice(&alice_ss58_address_len_encoded);
		call_data.extend_from_slice(&alice_ss58_address_encoded);
		log::info!("transferToMagnet callData:{:?}", hex::encode(&call_data.clone()));

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			0.into(),
			3_000_000,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			None,
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

#[test]
fn not_evm_admin_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		let bob_evm = H160::from_slice(&[17u8; 20][..]);
		/*
		let bob_evm: H160 = H160([
			0x05, 0xF9, 0xb8, 0xC7, 0x6E, 0x89, 0x87, 0xB8, 0x15, 0xC9, 0x3C, 0x27, 0xD1, 0x45,
			0x20, 0xb6, 0xeD, 0x57, 0x39, 0x02,
		]);
		 */
		log::info!("bob evm:{:?}", bob_evm);

		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000_000);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);

		let bob_evm_value_before: u128 = bob_evm_account.balance.as_u128();
		log::info!("bob evm value before:{:?}", bob_evm_value_before);

		let mint_amount: u128 = 1000_000_000_000_000_000;
		let transfer_token_amount: u128 = 800_000_000_000_000_000;

		let target: H160 = H160::from_low_u64_be(2049);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let token_addr = deploy_contract(bob_evm);
		log::info!("token addr:{:?}", token_addr);

		let asset_id = create_and_register_asset(token_addr);
		log::info!("asset id:{:?}", asset_id);

		let alice_token_amount_before = Assets::balance(asset_id, &ALICE);
		log::info!("alice token amount before mint:{:?}", alice_token_amount_before);

		let bob_evm_token_before_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("before mint, bob evm token balance:{:?}", bob_evm_token_before_mint);
		mint_erc20_tokens(token_addr, bob_evm, mint_amount, bob_evm);
		let bob_evm_token_after_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after mint, bob evm token balance:{:?}", bob_evm_token_after_mint);

		match burn_erc20_tokens(token_addr, bob_evm, transfer_token_amount, bob_evm) {
			Ok(_) => log::info!("Token burned."),
			Err(e) => {
				panic!("burn token execution reverted:{:?}.", e);
			},
		}

		let bob_evm_token_after_burn: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after burn, bob evm token balance:{:?}", bob_evm_token_after_burn);

		let selector_bytes: [u8; 4] =
			sp_io::hashing::keccak_256(b"transferToMagnet(address,uint256,string)")[0..4]
				.try_into()
				.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);

		let token_addr_encoded = token_addr.to_fixed_bytes();
		let transfer_value_encoded = solidity::encode_arguments(transfer_token_amount);
		let alice_ss58_address_encoded = solidity::encode_arguments(alice_ss58_bstring);
		let alice_ss58_address_len: u128 = alice_ss58_address_encoded.len().try_into().unwrap();
		let alice_ss58_address_len_encoded = solidity::encode_arguments(alice_ss58_address_len);
		log::info!("alice ss58 address encoded length:{:?}", alice_ss58_address_len);

		let mut call_data = selector.to_be_bytes().to_vec();
		call_data.extend_from_slice(&[0u8; 12][..]);
		call_data.extend_from_slice(&token_addr_encoded);
		call_data.extend_from_slice(&transfer_value_encoded);
		call_data.extend_from_slice(&alice_ss58_address_len_encoded);
		call_data.extend_from_slice(&alice_ss58_address_encoded);
		log::info!("transferToMagnet callData:{:?}", hex::encode(&call_data.clone()));

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			0.into(),
			3_000_000,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			None,
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
						"Caller is not the admin"
					)))
				);
			},
		};
	})
}

#[test]
fn token_and_assets_not_bound_works() {
	ExtBuilder::default().existential_deposit(100).build().execute_with(|| {
		//let bob_evm= H160::from_slice(&[17u8;20][..]);
		let bob_evm: H160 = H160([
			0x05, 0xF9, 0xb8, 0xC7, 0x6E, 0x89, 0x87, 0xB8, 0x15, 0xC9, 0x3C, 0x27, 0xD1, 0x45,
			0x20, 0xb6, 0xeD, 0x57, 0x39, 0x02,
		]);
		log::info!("bob evm:{:?}", bob_evm);

		let bob = <Test as pallet_evm::Config>::AddressMapping::into_account_id(bob_evm);
		let _ = Balances::deposit_creating(&ALICE, 10_000_000_000_000_000_000);
		let _ = Balances::deposit_creating(&bob, 6_000_000_000_000_000_000);

		let (bob_evm_account, _) = EVM::account_basic(&bob_evm);

		let bob_evm_value_before: u128 = bob_evm_account.balance.as_u128();
		log::info!("bob evm value before:{:?}", bob_evm_value_before);

		let mint_amount: u128 = 1000_000_000_000_000_000;
		let transfer_token_amount: u128 = 800_000_000_000_000_000;

		let target: H160 = H160::from_low_u64_be(2049);

		let alice_ss58_address = ALICE.to_ss58check();
		let alice_ss58_bstring = <BoundedString<MaxSize>>::from(alice_ss58_address);

		let token_addr = deploy_contract(bob_evm);
		log::info!("token addr:{:?}", token_addr);

		let asset_id = create_without_register_asset();
		log::info!("asset id:{:?}", asset_id);

		let alice_token_amount_before = Assets::balance(asset_id, &ALICE);
		log::info!("alice token amount before mint:{:?}", alice_token_amount_before);

		let bob_evm_token_before_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("before mint, bob evm token balance:{:?}", bob_evm_token_before_mint);
		mint_erc20_tokens(token_addr, bob_evm, mint_amount, bob_evm);
		let bob_evm_token_after_mint: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after mint, bob evm token balance:{:?}", bob_evm_token_after_mint);

		match burn_erc20_tokens(token_addr, bob_evm, transfer_token_amount, bob_evm) {
			Ok(_) => log::info!("Token burned."),
			Err(e) => {
				panic!("burn token execution reverted:{:?}.", e);
			},
		}

		let bob_evm_token_after_burn: U256 = query_balance_of(token_addr, bob_evm, bob_evm);
		log::info!("after burn, bob evm token balance:{:?}", bob_evm_token_after_burn);

		let selector_bytes: [u8; 4] =
			sp_io::hashing::keccak_256(b"transferToMagnet(address,uint256,string)")[0..4]
				.try_into()
				.unwrap();
		let selector = u32::from_be_bytes(selector_bytes);

		let token_addr_encoded = token_addr.to_fixed_bytes();
		let transfer_value_encoded = solidity::encode_arguments(transfer_token_amount);
		let alice_ss58_address_encoded = solidity::encode_arguments(alice_ss58_bstring);
		let alice_ss58_address_len: u128 = alice_ss58_address_encoded.len().try_into().unwrap();
		let alice_ss58_address_len_encoded = solidity::encode_arguments(alice_ss58_address_len);
		log::info!("alice ss58 address encoded length:{:?}", alice_ss58_address_len);

		let mut call_data = selector.to_be_bytes().to_vec();
		call_data.extend_from_slice(&[0u8; 12][..]);
		call_data.extend_from_slice(&token_addr_encoded);
		call_data.extend_from_slice(&transfer_value_encoded);
		call_data.extend_from_slice(&alice_ss58_address_len_encoded);
		call_data.extend_from_slice(&alice_ss58_address_encoded);
		log::info!("transferToMagnet callData:{:?}", hex::encode(&call_data.clone()));

		let is_transactional = true;
		let validate = true;
		let call_result = <Test as pallet_evm::Config>::Runner::call(
			bob_evm,
			target,
			call_data,
			0.into(),
			3_000_000,
			Some(U256::from(1_000_000_000)),
			Some(U256::default()),
			None,
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
						"Failed to get asset_id from token_addr"
					)))
				);
			},
		};
	})
}
