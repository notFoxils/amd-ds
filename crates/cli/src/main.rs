use std::path::Path;

use clap::Parser;
use snafu::{ResultExt, Snafu};

use crate::{
    command::{Command, RunCommandError},
    config::{InitConfigError, init_config},
};

mod command;
mod config;

#[derive(Debug, clap::Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,

    #[arg(
        default_value = "./amd_ds_config.toml",
        short,
        long,
        help = "Path to the scraping configuration file."
    )]
    pub config_path: Box<Path>,
}

#[derive(Debug, Snafu)]
enum CliError {
    #[snafu(display("failed to parse the provided config"))]
    InitConfig { source: InitConfigError },
    #[snafu(display("failed to run the provided command"))]
    RunCommand { source: RunCommandError },
}

#[snafu::report]
fn main() -> Result<(), CliError> {
    let args = Args::parse();
    let config = init_config(&args.config_path).context(InitConfigSnafu)?;

    args.command.run(&config).context(RunCommandSnafu)
}
