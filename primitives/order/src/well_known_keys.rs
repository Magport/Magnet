#![cfg_attr(not(feature = "std"), no_std)]

use {
    cumulus_primitives_core::ParaId, sp_core::Encode, sp_io::hashing::twox_64, sp_std::vec::Vec,
};

pub const PARAS_PARA_LIFECYCLES: &[u8] =
&hex_literal::hex!["cd710b30bd2eab0352ddcc26417aa194281e0bfde17b36573208a06cb5cfba6b"];

pub fn paras_para_lifecycles(para_id: ParaId) -> Vec<u8> {
    para_id.using_encoded(|para_id: &[u8]| {
        PARAS_PARA_LIFECYCLES
            .iter()
            .chain(twox_64(para_id).iter())
            .chain(para_id.iter())
            .cloned()
            .collect()
    })
}

pub const SYSTEM_BLOCKHASH: &[u8] =
&hex_literal::hex!["26aa394eea5630e07c48ae0c9558cef7a44704b568d21667356a5a050c118746b4def25cfda6ef3a00000000"];


pub const SYSTEM_ACCOUNT: &[u8] =
&hex_literal::hex!["26aa394eea5630e07c48ae0c9558cef7b99d880ec681799c0cf30e8886371da9"];

pub const SYSTEM_EVENTS: &[u8] =
&hex_literal::hex!["26aa394eea5630e07c48ae0c9558cef780d41e5e16056765bc8461851072c9d7"];

//OnDemandAssignmentProvider OnDemandQueue
pub const ON_DEMAND_QUEUE: &[u8] =
&hex_literal::hex!["8f32430b49607f8d60bfd3a003ddf4b53f35b69d817556cf6b886e5b4f01fbdc"];
