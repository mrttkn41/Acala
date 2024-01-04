// This file is part of Acala.

// Copyright (C) 2020-2024 Acala Foundation.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Acala CLI library.

#![allow(clippy::large_enum_variant)]

use clap::Parser;
use sc_cli::{KeySubcommand, SignCmd, VanityCmd, VerifyCmd};
use std::path::PathBuf;

use acala_service::chain_spec;

/// Possible subcommands of the main binary.
#[derive(Debug, Parser)]
pub enum Subcommand {
	/// Export the genesis state of the parachain.
	ExportGenesisState(cumulus_client_cli::ExportGenesisStateCommand),

	/// Export the genesis wasm of the parachain.
	ExportGenesisWasm(cumulus_client_cli::ExportGenesisWasmCommand),

	/// Key management cli utilities
	#[command(subcommand)]
	Key(KeySubcommand),

	/// The custom benchmark subcommmand benchmarking runtime modules.
	#[command(subcommand)]
	Benchmark(frame_benchmarking_cli::BenchmarkCmd),

	/// Try some experimental command on the runtime. This includes migration and runtime-upgrade
	/// testing.
	#[cfg(feature = "try-runtime")]
	TryRuntime(try_runtime_cli::TryRuntimeCmd),

	/// Verify a signature for a message, provided on STDIN, with a given
	/// (public or secret) key.
	Verify(VerifyCmd),

	/// Generate a seed that provides a vanity address.
	Vanity(VanityCmd),

	/// Sign a message, with a given (secret) key.
	Sign(SignCmd),

	/// Build a chain specification.
	BuildSpec(sc_cli::BuildSpecCmd),

	/// Validate blocks.
	CheckBlock(sc_cli::CheckBlockCmd),

	/// Export blocks.
	ExportBlocks(sc_cli::ExportBlocksCmd),

	/// Export the state of a given block into a chain spec.
	ExportState(sc_cli::ExportStateCmd),

	/// Import blocks.
	ImportBlocks(sc_cli::ImportBlocksCmd),

	/// Remove the whole chain.
	PurgeChain(cumulus_client_cli::PurgeChainCmd),

	/// Revert the chain to a previous state.
	Revert(sc_cli::RevertCmd),
}

/// An overarching CLI command definition.
#[derive(Debug, Parser)]
#[clap(
	propagate_version = true,
	args_conflicts_with_subcommands = true,
	subcommand_negates_reqs = true
)]
pub struct Cli {
	/// Possible subcommand with parameters.
	#[command(subcommand)]
	pub subcommand: Option<Subcommand>,

	#[allow(missing_docs)]
	#[clap(flatten)]
	pub run: cumulus_client_cli::RunCmd,

	/// Relay chain arguments
	#[clap(raw = true)]
	pub relay_chain_args: Vec<String>,

	/// Instant block sealing
	///
	/// Can only be used with `--dev`
	#[clap(long = "instant-sealing", requires = "dev")]
	pub instant_sealing: bool,

	/// Mnemonic for evm development.
	/// This will derive 10 funded accounts in the genesis
	///
	/// Can only be used with `--dev`
	#[cfg(feature = "with-mandala-runtime")]
	#[clap(long = "mnemonic", requires = "dev")]
	pub mnemonic: Option<String>,
}

/// Relay chain CLI.
#[derive(Debug)]
pub struct RelayChainCli {
	/// The actual relay chain cli object.
	pub base: polkadot_cli::RunCmd,

	/// Optional chain id that should be passed to the relay chain.
	pub chain_id: Option<String>,

	/// The base path that should be used by the relay chain.
	pub base_path: Option<PathBuf>,
}

impl RelayChainCli {
	/// Parse the relay chain CLI parameters using the para chain `Configuration`.
	pub fn new<'a>(
		para_config: &sc_service::Configuration,
		relay_chain_args: impl Iterator<Item = &'a String>,
	) -> Self {
		let extension = chain_spec::Extensions::try_get(&*para_config.chain_spec);
		let chain_id = extension.map(|e| e.relay_chain.clone());
		let base_path = para_config.base_path.path().join("polkadot");
		Self {
			base_path: Some(base_path),
			chain_id,
			base: clap::Parser::parse_from(relay_chain_args),
		}
	}
}
