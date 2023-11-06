pub mod collators;
pub mod collator;

const LOG_TARGET: &str = "on_demand_aura::magnet";

use sp_core::crypto::{ByteArray, Pair};
use sp_keystore::KeystorePtr;

type AuthorityId<P> = <P as Pair>::Public;


pub async fn order_slot<P: Pair>(
	idx: u32,
	authorities: &[AuthorityId<P>],
	keystore: &KeystorePtr,
) -> Option<P::Public> {

    if authorities.is_empty() {
		return None
	}

	let expected_author = authorities.get(idx as usize).expect(
		"authorities not empty; index constrained to list length;this is a valid index; qed",
	);

	if keystore.has_keys(&[(expected_author.to_raw_vec(), sp_application_crypto::key_types::AURA)]) {
		Some(expected_author.clone())
	} else {
		None
	}
}