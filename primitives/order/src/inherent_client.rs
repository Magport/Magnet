use sp_consensus_aura::sr25519::AuthorityId;
use {
	crate::OrderInherentData,
	cumulus_primitives_core::{ParaId, PersistedValidationData},
	cumulus_relay_chain_interface::{PHash, RelayChainInterface},
};

async fn collect_relay_storage_proof(
	relay_chain_interface: &impl RelayChainInterface,
	relay_parent: PHash,
) -> Option<sp_state_machine::StorageProof> {
	let mut relevant_keys = Vec::new();
	//System Events
	relevant_keys.push(
		hex_literal::hex!["26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7"]
			.to_vec(),
	);

	relay_chain_interface.prove_read(relay_parent, &relevant_keys).await.ok()
}

impl OrderInherentData<AuthorityId> {
	pub async fn create_at(
		relay_parent: PHash,
		relay_chain_interface: &impl RelayChainInterface,
		validation_data: &Option<PersistedValidationData>,
		para_id: ParaId,
		sequence_number: u64,
		author_pub: &Option<AuthorityId>,
	) -> Option<OrderInherentData<AuthorityId>> {
		let relay_storage_proof =
			collect_relay_storage_proof(relay_chain_interface, relay_parent).await?;

		Some(OrderInherentData {
			relay_storage_proof,
			validation_data: validation_data.clone(),
			para_id,
			sequence_number,
			author_pub: author_pub.clone(),
		})
	}
}

#[async_trait::async_trait]
impl sp_inherents::InherentDataProvider for OrderInherentData<AuthorityId> {
	async fn provide_inherent_data(
		&self,
		inherent_data: &mut sp_inherents::InherentData,
	) -> Result<(), sp_inherents::Error> {
		inherent_data.put_data(crate::INHERENT_IDENTIFIER, &self)
	}

	async fn try_handle_error(
		&self,
		_: &sp_inherents::InherentIdentifier,
		_: &[u8],
	) -> Option<Result<(), sp_inherents::Error>> {
		None
	}
}
