// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use clap::*;
//use derivative::Derivative;
use regex::Regex;
use std::{fmt, path::PathBuf};
#[derive(Parser, Clone, ValueEnum, Debug)]
pub enum Env {
    Devnet,
    Staging,
    Ci,
    CiNomad,
    Testnet,
    CustomRemote,
    NewLocal,
}

// #[derive(Derivative, Parser)]
// #[derivative(Debug)]
#[derive(Debug, Clone, Parser)]
#[clap(name = "", rename_all = "kebab-case")]
pub struct LocalClusterConfig {
    #[clap(value_enum)]
    pub env: Env,
    #[clap(long)]
    pub faucet_address: Option<String>,
    #[clap(long)]
    pub consensus_grpc_port: Option<u16>,
    #[clap(long)]
    pub fullnode_address: Option<String>,
    #[clap(long)]
    pub epoch_duration_ms: Option<u64>,
    /// URL for the indexer RPC server
    #[clap(long)]
    pub indexer_address: Option<String>,
    /// Use new version of indexer or not
    #[clap(long)]
    pub use_indexer_v2: bool,
    // /// URL for the Indexer Postgres DB
    #[clap(long)]
    // #[derivative(Debug(format_with = "obfuscated_pg_address"))]
    pub pg_address: Option<String>,
    /// TODO(gegao): remove this after indexer migration is complete.
    #[clap(long)]
    pub use_indexer_experimental_methods: bool,
    #[clap(long)]
    pub config_dir: Option<PathBuf>,
    #[clap(long)]
    pub cluster_size: Option<usize>,
    /// URL for the indexer RPC server
    #[clap(long)]
    pub graphql_address: Option<String>,
}

fn obfuscated_pg_address(val: &Option<String>, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match val {
        None => write!(f, "None"),
        Some(val) => {
            write!(
                f,
                "{}",
                Regex::new(r":.*@")
                    .unwrap()
                    .replace_all(val.as_str(), ":*****@")
            )
        }
    }
}

impl LocalClusterConfig {
    pub fn new_local() -> Self {
        Self {
            env: Env::NewLocal,
            faucet_address: None,
            consensus_grpc_port: None,
            fullnode_address: None,
            epoch_duration_ms: None,
            indexer_address: None,
            pg_address: None,
            use_indexer_experimental_methods: false,
            config_dir: None,
            cluster_size: None,
            graphql_address: None,
            use_indexer_v2: false,
        }
    }
}
