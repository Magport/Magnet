//! Substrate Parachain Node Template CLI

#![warn(missing_docs)]

mod chain_spec;
#[macro_use]
mod service;
mod cli;
mod client;
mod command;
mod eth;
mod rpc;
mod submit_order;
mod on_demand_order;

fn main() -> sc_cli::Result<()> {
	command::run()
}
