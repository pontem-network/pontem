use clap::Parser as Clap;
use std::path::PathBuf;
use sc_cli::SubstrateCli;

#[derive(Debug)]
pub enum Sealing {
    /// Blocks are produced for each incoming transaction.
    Instant,
    /// Blocks are produced once per N milliseconds
    Interval(u64),
}

impl std::str::FromStr for Sealing {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "instant" => Ok(Self::Instant),
            number => {
                let millis = number
                    .parse()
                    .map_err(|_| "unable to decode sealing param")?;
                Ok(Self::Interval(millis))
            }
        }
    }
}

#[derive(Debug, Clap)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Option<Subcommand>,

    #[clap(flatten)]
    pub run: cumulus_client_cli::RunCmd,

    /// Sealing mode for --dev-service
    #[clap(long, default_value = "instant")]
    pub sealing: Sealing,

    /// Whether to run node in development node (single node, no consensus)
    #[clap(long)]
    pub dev_service: bool,

    /// Relaychain arguments
    #[clap(raw = true)]
    pub relaychain_args: Vec<String>,
}

#[derive(Debug, Clap)]
pub enum Subcommand {
    /// Export the genesis state of the parachain.
    #[clap(name = "export-genesis-state")]
    ExportGenesisState(ExportGenesisStateCommand),

    /// Export the genesis wasm of the parachain.
    #[clap(name = "export-genesis-wasm")]
    ExportGenesisWasm(ExportGenesisWasmCommand),

    /// Key management cli utilities
    #[clap(subcommand)]
    Key(sc_cli::KeySubcommand),
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

    /// The custom benchmark subcommmand benchmarking runtime pallets.
    #[clap(name = "benchmark", about = "Benchmark runtime pallets.")]
    Benchmark(frame_benchmarking_cli::BenchmarkCmd),
}

/// Command for exporting the genesis state of the parachain
#[derive(Debug, Clap)]
pub struct ExportGenesisStateCommand {
    /// Output file name or stdout if unspecified.
    #[clap(parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// Write output in binary. Default is to write in hex.
    #[clap(short, long)]
    pub raw: bool,

    /// The name of the chain for that the genesis state should be exported.
    #[clap(long)]
    pub chain: Option<String>,
}

/// Command for exporting the genesis wasm file.
#[derive(Debug, Clap)]
pub struct ExportGenesisWasmCommand {
    /// Output file name or stdout if unspecified.
    #[clap(parse(from_os_str))]
    pub output: Option<PathBuf>,

    /// Write output in binary. Default is to write in hex.
    #[clap(short, long)]
    pub raw: bool,

    /// The name of the chain for that the genesis wasm file should be exported.
    #[clap(long)]
    pub chain: Option<String>,
}

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
        relay_chain_args: impl Iterator<Item = String>,
    ) -> Self {
        let extension = crate::chain_spec::Extensions::try_get(&*para_config.chain_spec);
        let chain_id = extension.map(|e| e.relay_chain.clone());
        let base_path = para_config
            .base_path
            .as_ref()
            .map(|x| x.path().join("polkadot"));
        let args = std::iter::once(Self::executable_name().to_string()).chain(relay_chain_args);
        Self {
            base_path,
            chain_id,
            base: polkadot_cli::RunCmd::parse_from(args),
        }
    }
}
