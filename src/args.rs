use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub(crate) struct MeshtasticGateArgs {
    #[clap(subcommand)]
    pub(crate) command: CommandType,
}

#[derive(Debug, Subcommand)]
pub(crate) enum CommandType {
    /// Starts API server
    Start(StartCommand),
}

#[derive(Debug, Args)]
pub(crate) struct StartCommand {
}