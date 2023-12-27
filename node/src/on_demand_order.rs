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

use crate::submit_order::{build_rpc_for_submit_order, SubmitOrderError};
use codec::{Codec, Decode};
use cumulus_primitives_core::relay_chain::vstaging::Assignment;
use cumulus_primitives_core::{
	relay_chain::BlockNumber as RelayBlockNumber, ParaId, PersistedValidationData,
};
use cumulus_relay_chain_interface::{RelayChainInterface, RelayChainResult};
use frame_system::{self, AccountInfo};
use futures::{lock::Mutex, pin_mut, select, FutureExt, Stream, StreamExt};
use magnet_primitives_order::{
	self,
	well_known_keys::paras_para_lifecycles,
	well_known_keys::{
		ACTIVE_CONFIG, ON_DEMAND_QUEUE, SPOT_TRAFFIC, SYSTEM_ACCOUNT, SYSTEM_BLOCKHASH,
		SYSTEM_EVENTS,
	},
	OrderRecord, OrderRuntimeApi, OrderStatus,
};
use mp_system::OnRelayChainApi;
pub use pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi;
use polkadot_primitives::OccupiedCoreAssumption;
use rococo_runtime::{Runtime, RuntimeCall, SignedExtra, SignedPayload, UncheckedExtrinsic};
use runtime_parachains::{
	assigner_on_demand as parachains_assigner_on_demand, configuration::HostConfiguration,
	paras::ParaLifecycle,
};
use sc_client_api::UsageProvider;
use sc_service::TaskManager;
use sc_transaction_pool_api::{InPoolTransaction, MaintainedTransactionPool};
use sp_api::ProvideRuntimeApi;
use sp_application_crypto::{AppCrypto, AppPublic};
use sp_consensus_aura::AuraApi;
use sp_core::{crypto::Pair, H256};
use sp_io::hashing::blake2_128;
use sp_keystore::KeystorePtr;
use sp_runtime::{
	codec::Encode,
	generic,
	traits::{
		AtLeast32BitUnsigned, Block as BlockT, Header as HeaderT, MaybeDisplay, Member,
		SaturatedConversion,
	},
	FixedPointNumber, FixedU128, OpaqueExtrinsic,
};
use std::cmp::Ordering;
use std::{convert::TryFrom, error::Error, fmt::Debug, sync::Arc};

#[derive(Clone, PartialEq)]
pub enum OrderType {
	Normal,
	Force,
	XCMEvent,
}

async fn get_spot_price<Balance>(
	relay_chain: impl RelayChainInterface + Clone,
	hash: H256,
) -> Option<Balance>
where
	Balance: Codec + MaybeDisplay + 'static + Debug + From<u128>,
{
	let spot_traffic_storage = relay_chain.get_storage_by_key(hash, SPOT_TRAFFIC).await.ok()?;
	let p_spot_traffic = spot_traffic_storage
		.map(|raw| <FixedU128>::decode(&mut &raw[..]))
		.transpose()
		.ok()?;
	let active_config_storage = relay_chain.get_storage_by_key(hash, ACTIVE_CONFIG).await.ok()?;
	let p_active_config = active_config_storage
		.map(|raw| <HostConfiguration<u32>>::decode(&mut &raw[..]))
		.transpose()
		.ok()?;
	if p_spot_traffic.is_some() && p_active_config.is_some() {
		let spot_traffic = p_spot_traffic.unwrap();
		let active_config = p_active_config.unwrap();
		let spot_price = spot_traffic
			.saturating_mul_int(active_config.on_demand_base_fee.saturated_into::<u128>());
		Some(Balance::from(spot_price))
	} else {
		None
	}
}

async fn get_relay_chain_nonce<Balance>(
	relay_chain: impl RelayChainInterface + Clone,
	hash: H256,
	keystore: KeystorePtr,
) -> Option<u32>
where
	Balance: Codec + MaybeDisplay + 'static + Debug,
{
	let pubkey = keystore.sr25519_public_keys(sp_application_crypto::key_types::AURA)[0];
	//System Account
	let public_key: Vec<u8> = pubkey.using_encoded(|key: &[u8]| {
		SYSTEM_ACCOUNT
			.iter()
			.chain(blake2_128(key).iter())
			.chain(key.iter())
			.cloned()
			.collect()
	});
	let system_account_storage =
		relay_chain.get_storage_by_key(hash, public_key.as_slice()).await.ok()?;
	let system_account = system_account_storage.map(|raw| AccountInfo::<parachain_magnet_runtime::Nonce,pallet_balances::AccountData<Balance>>::decode(&mut &raw[..])).transpose().ok()?;
	match system_account {
		Some(account) => Some(account.nonce),
		None => Some(0),
	}
}

async fn try_place_order<Balance>(
	relay_chain: impl RelayChainInterface + Clone,
	hash: H256,
	number: u32,
	keystore: KeystorePtr,
	para_id: ParaId,
	url: String,
	max_amount: Balance,
) -> Result<(), SubmitOrderError>
where
	Balance: Codec + MaybeDisplay + 'static + Debug + Into<u128>,
{
	let nonce = get_relay_chain_nonce::<Balance>(relay_chain.clone(), hash, keystore.clone())
		.await
		.ok_or_else(|| SubmitOrderError::NonceGetError)?;
	// key:System BlockHash 0x00000000(Twox64Concat)
	let genesis_hash_storage = relay_chain
		.get_storage_by_key(hash, SYSTEM_BLOCKHASH)
		.await
		.map_err(|_e| SubmitOrderError::GenesisHashGetError)?;
	let genesis_hash = genesis_hash_storage
		.map(|raw| <H256>::decode(&mut &raw[..]))
		.transpose()
		.map_err(|_e| SubmitOrderError::GenesisHashGetError)?
		.unwrap_or_default();
	let max_amount_128 = max_amount.into();
	let output = place_order_extrinsic(
		hash,
		u64::from(number),
		genesis_hash,
		nonce,
		keystore,
		para_id,
		max_amount_128,
	);
	build_rpc_for_submit_order(&url, output).await
}

async fn reach_txpool_threshold<P, Block, ExPool, Balance, PB>(
	parachain: &P,
	transaction_pool: Arc<ExPool>,
	height: RelayBlockNumber,
	snap_txs: Vec<H256>,
) -> Option<(bool, OrderType)>
where
	Block: BlockT,
	P: ProvideRuntimeApi<Block> + UsageProvider<Block>,
	Balance: Codec + MaybeDisplay + 'static + Debug + AtLeast32BitUnsigned + Copy,
	P::Api: TransactionPaymentApi<Block, Balance>
		+ OrderRuntimeApi<Block, Balance, PB::Public>
		+ OnRelayChainApi<Block>,
	ExPool: MaintainedTransactionPool<Block = Block, Hash = <Block as BlockT>::Hash> + 'static,
	PB: Pair,
	PB::Public: AppPublic + Member + Codec,
	PB::Signature: TryFrom<Vec<u8>> + Member + Codec,
{
	let mut pending_iterator = transaction_pool.ready();
	let mut is_place_order = false;
	let mut all_gas_value = Balance::from(0u32);
	let block_hash = parachain.usage_info().chain.best_hash;
	let mut back_txs: Vec<H256> = vec![];
	loop {
		let pending_tx =
			if let Some(pending_tx) = pending_iterator.next() { pending_tx } else { break };
		let pending_tx_data = pending_tx.data().clone();
		let utx_length = pending_tx_data.encode().len() as u32;
		let query_fee = parachain
			.runtime_api()
			.query_fee_details(block_hash, pending_tx_data, utx_length)
			.ok()?;
		all_gas_value = query_fee.final_fee().add(all_gas_value);
		if transaction_pool.status().ready != 0 {
			is_place_order =
				parachain.runtime_api().reach_txpool_threshold(block_hash, all_gas_value).ok()?;
		}
		log::info!(
			"tx_fee:{:?},all_fee:{:?},can_order:{:?},status:{:?}",
			query_fee.final_fee(),
			all_gas_value,
			is_place_order,
			transaction_pool.status()
		);
		back_txs.push(H256::from_slice(pending_tx.hash().as_ref()));
	}
	let mut order_type = OrderType::Normal;
	if !is_place_order {
		//check is need force bid coretime
		let force_bid = parachain.runtime_api().on_relaychain(block_hash, height).ok()?;
		if all_gas_value.cmp(&Balance::from(0u32)) == Ordering::Greater && force_bid == 1 {
			is_place_order = true;
			order_type = OrderType::Force;
		}
	}
	if is_place_order {
		if back_txs == snap_txs {
			is_place_order = false;
		}
	}
	log::info!("back_txs:{:?}", back_txs);
	log::info!("snap_txs:{:?}", snap_txs);
	Some((is_place_order, order_type))
}

async fn relay_chain_xcm_event(
	relay_chain_interface: impl RelayChainInterface + Clone,
	para_id: ParaId,
	relay_parent: H256,
) -> Option<(bool, OrderType)> {
	let downward_messages =
		relay_chain_interface.retrieve_dmq_contents(para_id, relay_parent).await.ok()?;
	let horizontal_messages = relay_chain_interface
		.retrieve_all_inbound_hrmp_channel_contents(para_id, relay_parent)
		.await
		.ok()?;
	let can_order = downward_messages.len() > 0 || horizontal_messages.len() > 0;
	return Some((can_order, OrderType::XCMEvent));
}

async fn get_txs<Block, ExPool>(transaction_pool: Arc<ExPool>) -> Vec<H256>
where
	Block: BlockT,
	ExPool: MaintainedTransactionPool<Block = Block, Hash = <Block as BlockT>::Hash> + 'static,
{
	let mut pending_iterator = transaction_pool.ready();
	let mut back_txs: Vec<H256> = vec![];
	loop {
		let pending_tx =
			if let Some(pending_tx) = pending_iterator.next() { pending_tx } else { break };
		back_txs.push(H256::from_slice(pending_tx.hash().as_ref()));
	}
	return back_txs;
}

async fn handle_new_best_parachain_head<P, Block, PB, ExPool, Balance>(
	validation_data: PersistedValidationData,
	height: RelayBlockNumber,
	parachain: &P,
	keystore: KeystorePtr,
	relay_chain: impl RelayChainInterface + Clone,
	p_hash: H256,
	para_id: ParaId,
	order_record: Arc<Mutex<OrderRecord<PB::Public>>>,
	transaction_pool: Arc<ExPool>,
	url: String,
) -> Result<(), Box<dyn Error>>
where
	Block: BlockT,
	P: ProvideRuntimeApi<Block> + UsageProvider<Block>,
	Balance: Codec
		+ MaybeDisplay
		+ 'static
		+ Debug
		+ Into<u128>
		+ AtLeast32BitUnsigned
		+ Copy
		+ From<u128>,
	P::Api: AuraApi<Block, PB::Public>
		+ OrderRuntimeApi<Block, Balance, PB::Public>
		+ TransactionPaymentApi<Block, Balance>
		+ OnRelayChainApi<Block>,
	PB: Pair,
	PB::Public: AppPublic + Member + Codec,
	PB::Signature: TryFrom<Vec<u8>> + Member + Codec,
	ExPool: MaintainedTransactionPool<Block = Block, Hash = <Block as BlockT>::Hash> + 'static,
{
	let para_lifecycles_storage = relay_chain
		.get_storage_by_key(p_hash, paras_para_lifecycles(para_id).as_slice())
		.await?;
	let para_lifecycles = para_lifecycles_storage
		.map(|raw| <ParaLifecycle>::decode(&mut &raw[..]))
		.transpose()?;
	let is_parathread = match para_lifecycles {
		Some(lifecycles) => matches!(
			lifecycles,
			ParaLifecycle::Parathread
				| ParaLifecycle::UpgradingParathread
				| ParaLifecycle::OffboardingParathread
		),
		None => false,
	};
	if !is_parathread {
		let mut order_record_local = order_record.lock().await;
		order_record_local.validation_data = None;
		order_record_local.author_pub = None;
		order_record_local.relay_parent = None;
		return Ok(());
	} else {
		let order_record_local = order_record.lock().await;
		if order_record_local.relay_height == height {
			return Ok(());
		}
	}
	let head = validation_data.clone().parent_head.0;
	let parachain_head = match <<Block as BlockT>::Header>::decode(&mut &head[..]) {
		Ok(header) => header,
		Err(err) => return Err(format!("get parachain head error:{:?}", err).into()),
	};

	let hash = parachain_head.hash();
	let authorities = parachain.runtime_api().authorities(hash).map_err(Box::new)?;
	let slot_width = parachain.runtime_api().slot_width(hash)?;
	let auth_len = authorities.len() as u32;
	let idx = (height >> slot_width) % auth_len;
	let collator_public =
		magnet_client_consensus_aura::order_slot::<PB>(idx, &authorities, &keystore).await;
	let base = 2 as u32;
	let slot_block = base.pow(slot_width);
	if height % slot_block == 0 {
		let mut order_record_local = order_record.lock().await;
		order_record_local.relay_base = p_hash;
		order_record_local.relay_base_height = height;
		order_record_local.order_status = OrderStatus::Init;
	}
	let mut relevant_keys = Vec::new();
	//System Events
	relevant_keys.push(SYSTEM_EVENTS.to_vec());
	let storage_proof = relay_chain.prove_read(p_hash, &relevant_keys).await?;
	let order_placed = parachain.runtime_api().order_placed(
		hash,
		storage_proof,
		validation_data.clone(),
		para_id,
	)?;
	match order_placed {
		Some(author) => {
			let mut order_record_local = order_record.lock().await;
			order_record_local.relay_parent = Some(p_hash);
			order_record_local.relay_height = height;
			order_record_local.validation_data = Some(validation_data);
			order_record_local.author_pub = Some(author);
			order_record_local.para_id = para_id;
			let sequence_number = parachain.runtime_api().sequence_number(hash)?;
			order_record_local.sequence_number = sequence_number;
			order_record_local.txs = get_txs(transaction_pool.clone()).await;
		},
		None => {
			let sequence_number = parachain.runtime_api().sequence_number(hash)?;
			let order_executed = parachain.runtime_api().order_executed(hash, sequence_number)?;
			if order_executed {
				return Ok(());
			}
			let mut order_record_local = order_record.lock().await;
			if collator_public.is_some() {
				//your turn
				let reached = reach_txpool_threshold::<_, _, _, _, PB>(
					parachain,
					transaction_pool,
					height,
					order_record_local.txs.clone(),
				)
				.await;
				let mut can_order = false;
				let mut order_type = OrderType::Normal;
				if let Some((reach, o_t)) = reached {
					if reach {
						order_type = o_t;
						let mut exist_order = false;
						// key = OnDemandAssignmentProvider OnDemandQueue
						let on_demand_queue_storage =
							relay_chain.get_storage_by_key(p_hash, ON_DEMAND_QUEUE).await?;
						let on_demand_queue = on_demand_queue_storage
							.map(|raw| <Vec<Assignment>>::decode(&mut &raw[..]))
							.transpose()?;
						if let Some(vvs) = on_demand_queue.clone() {
							for vv in vvs.into_iter() {
								if vv.para_id == para_id {
									exist_order = true;
									break;
								}
							}
						}
						if !exist_order {
							can_order = true;
						}
					} else {
						let trig_xcm_event =
							relay_chain_xcm_event(relay_chain.clone(), para_id, p_hash).await;
						if let Some((trig_flag, o_t)) = trig_xcm_event {
							can_order = trig_flag;
							order_type = o_t;
						}
					}
				}
				if can_order {
					if height - order_record_local.relay_height > slot_block {
						if order_record_local.order_status == OrderStatus::Init {
							let max_amount = parachain.runtime_api().order_max_amount(hash)?;
							let p_spot_price =
								get_spot_price::<Balance>(relay_chain.clone(), p_hash).await;
							let spot_price;
							if p_spot_price.is_some() {
								spot_price = p_spot_price.unwrap();
							} else {
								spot_price = max_amount;
							}
							match order_type {
								OrderType::Normal => {
									log::info!(
										"============normal place order======================"
									);
								},
								OrderType::Force => {
									log::info!(
										"============force place order======================"
									);
								},
								OrderType::XCMEvent => {
									log::info!("============xcm place order======================");
								},
							}
							let order_result = try_place_order::<Balance>(
								relay_chain,
								order_record_local.relay_base,
								order_record_local.relay_base_height,
								keystore,
								para_id,
								url,
								spot_price,
							)
							.await;
							log::info!("===========place order completed==============",);
							order_record_local.order_status = OrderStatus::Order;
							if order_result.is_err() {
								log::info!(
									"===========place_order error:=============={:?}",
									order_result
								);
							}
						}
					}
				}
			}
		},
	}
	Ok(())
}

async fn new_best_heads(
	relay_chain: impl RelayChainInterface + Clone,
	para_id: ParaId,
) -> RelayChainResult<impl Stream<Item = (u32, PersistedValidationData, H256)>> {
	let new_best_notification_stream =
		relay_chain.new_best_notification_stream().await?.filter_map(move |n| {
			let relay_chain = relay_chain.clone();
			async move {
				let relay_head: PersistedValidationData = relay_chain
					.persisted_validation_data(n.hash(), para_id, OccupiedCoreAssumption::TimedOut)
					.await
					.map(|s| s.map(|s| s))
					.ok()
					.flatten()?;
				Some((n.number, relay_head, n.hash()))
			}
		});

	Ok(new_best_notification_stream)
}
async fn relay_chain_notification<P, R, Block, PB, ExPool, Balance>(
	para_id: ParaId,
	parachain: Arc<P>,
	relay_chain: R,
	keystore: KeystorePtr,
	order_record: Arc<Mutex<OrderRecord<PB::Public>>>,
	transaction_pool: Arc<ExPool>,
	url: String,
) where
	R: RelayChainInterface + Clone,
	Block: BlockT,
	Balance: Codec
		+ MaybeDisplay
		+ 'static
		+ Debug
		+ Into<u128>
		+ AtLeast32BitUnsigned
		+ Copy
		+ From<u128>,
	P: ProvideRuntimeApi<Block> + UsageProvider<Block>,
	P::Api: AuraApi<Block, PB::Public>
		+ OrderRuntimeApi<Block, Balance, PB::Public>
		+ TransactionPaymentApi<Block, Balance>
		+ OnRelayChainApi<Block>,
	PB: Pair,
	PB::Public: AppPublic + Member + Codec,
	PB::Signature: TryFrom<Vec<u8>> + Member + Codec,
	ExPool: MaintainedTransactionPool<Block = Block, Hash = <Block as BlockT>::Hash> + 'static,
{
	let new_best_heads = match new_best_heads(relay_chain.clone(), para_id).await {
		Ok(best_heads_stream) => best_heads_stream.fuse(),
		Err(_err) => {
			return;
		},
	};
	pin_mut!(new_best_heads);
	loop {
		select! {
			h = new_best_heads.next() => {
				match h {
					Some((height, head, hash)) => {
						let _ = handle_new_best_parachain_head::<_,_,PB,_,_>(head,height, &*parachain,keystore.clone(), relay_chain.clone(), hash, para_id, order_record.clone(),transaction_pool.clone(), url.clone()).await;
					},
					None => {
						return;
					}
				}
			},
		}
	}
}
pub async fn run_on_demand_task<P, R, Block, PB, ExPool, Balance>(
	para_id: ParaId,
	parachain: Arc<P>,
	relay_chain: R,
	keystore: KeystorePtr,
	order_record: Arc<Mutex<OrderRecord<PB::Public>>>,
	transaction_pool: Arc<ExPool>,
	url: String,
) where
	R: RelayChainInterface + Clone,
	Block: BlockT,
	P: ProvideRuntimeApi<Block> + UsageProvider<Block>,
	Balance: Codec
		+ MaybeDisplay
		+ 'static
		+ Debug
		+ Into<u128>
		+ AtLeast32BitUnsigned
		+ Copy
		+ From<u128>,
	P::Api: AuraApi<Block, PB::Public>
		+ OrderRuntimeApi<Block, Balance, PB::Public>
		+ TransactionPaymentApi<Block, Balance>
		+ OnRelayChainApi<Block>,
	PB: Pair,
	PB::Public: AppPublic + Member + Codec,
	PB::Signature: TryFrom<Vec<u8>> + Member + Codec,
	ExPool: MaintainedTransactionPool<Block = Block, Hash = <Block as BlockT>::Hash> + 'static,
{
	let relay_chain_notification = relay_chain_notification::<_, _, _, PB, _, _>(
		para_id,
		parachain.clone(),
		relay_chain,
		keystore,
		order_record,
		transaction_pool,
		url,
	);
	select! {
		_ = relay_chain_notification.fuse() => {},
	}
}

pub fn spawn_on_demand_order<T, R, ExPool, Block, PB, Balance>(
	parachain: Arc<T>,
	para_id: ParaId,
	relay_chain: R,
	transaction_pool: Arc<ExPool>,
	task_manager: &TaskManager,
	keystore: KeystorePtr,
	order_record: Arc<Mutex<OrderRecord<PB::Public>>>,
	url: String,
) -> sc_service::error::Result<()>
where
	Block: BlockT,
	R: RelayChainInterface + Clone + 'static,
	Balance: Codec
		+ MaybeDisplay
		+ 'static
		+ Debug
		+ Send
		+ Into<u128>
		+ AtLeast32BitUnsigned
		+ Copy
		+ From<u128>,
	T: Send + Sync + 'static + ProvideRuntimeApi<Block> + UsageProvider<Block>,
	ExPool: MaintainedTransactionPool<Block = Block, Hash = <Block as BlockT>::Hash> + 'static,
	T::Api: AuraApi<Block, PB::Public>
		+ OrderRuntimeApi<Block, Balance, PB::Public>
		+ TransactionPaymentApi<Block, Balance>
		+ OnRelayChainApi<Block>,
	PB: Pair + 'static,
	PB::Public: AppPublic + Member + Codec,
	PB::Signature: TryFrom<Vec<u8>> + Member + Codec,
{
	let on_demand_order_task = run_on_demand_task::<_, _, _, PB, _, _>(
		para_id,
		parachain.clone(),
		relay_chain.clone(),
		keystore,
		order_record,
		transaction_pool.clone(),
		url,
	);
	task_manager.spawn_essential_handle().spawn_blocking(
		"on_demand_order_task",
		None,
		on_demand_order_task,
	);
	Ok(())
}

pub fn construct_extrinsic(
	current_block_hash: H256,
	current_block: u64,
	genesis_block: H256,
	function: impl Into<RuntimeCall>,
	keystore: KeystorePtr,
	nonce: u32,
) -> UncheckedExtrinsic {
	let function = function.into();
	let period = 4;
	let tip = 0;
	let extra: SignedExtra = (
		frame_system::CheckNonZeroSender::<Runtime>::new(),
		frame_system::CheckSpecVersion::<Runtime>::new(),
		frame_system::CheckTxVersion::<Runtime>::new(),
		frame_system::CheckGenesis::<Runtime>::new(),
		frame_system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
		frame_system::CheckNonce::<Runtime>::from(nonce),
		frame_system::CheckWeight::<Runtime>::new(),
		pallet_transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
	);
	let raw_payload = SignedPayload::from_raw(
		function.clone(),
		extra.clone(),
		(
			(),
			rococo_runtime::VERSION.spec_version,
			rococo_runtime::VERSION.transaction_version,
			genesis_block,
			current_block_hash,
			(),
			(),
			(),
		),
	);
	let pub_key = keystore.sr25519_public_keys(sp_consensus_aura::sr25519::AuthorityPair::ID)[0];
	let signature = raw_payload
		.using_encoded(|e| {
			keystore.sr25519_sign(sp_consensus_aura::sr25519::AuthorityPair::ID, &pub_key, e)
		})
		.unwrap()
		.unwrap();
	UncheckedExtrinsic::new_signed(
		function.clone(),
		rococo_runtime::Address::Id(pub_key.into()),
		polkadot_primitives::Signature::Sr25519(signature.clone()),
		extra.clone(),
	)
}

pub fn place_order_extrinsic(
	current_block_hash: H256,
	current_block: u64,
	genesis_block: H256,
	nonce: u32,
	keystore: KeystorePtr,
	para_id: ParaId,
	max_amount: u128,
) -> String {
	let function = rococo_runtime::RuntimeCall::OnDemandAssignmentProvider(
		parachains_assigner_on_demand::Call::place_order_allow_death { max_amount, para_id },
	);
	let extrinsic: OpaqueExtrinsic = construct_extrinsic(
		current_block_hash,
		current_block,
		genesis_block,
		function,
		keystore,
		nonce,
	)
	.into();
	let output = array_bytes::bytes2hex("", &extrinsic.encode());
	output
}
