//! Magnet Parachain Node CLI

#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod client;
mod command;
mod eth;
mod metadata;
mod on_demand_order;
mod rpc;
mod submit_order;

fn main() -> sc_cli::Result<()> {
	command::run()
}
