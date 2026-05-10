use std::path::Path;

use amd_ds::{
    driver_version::DriverVersion,
    request::driver_download_page::{RequestDriverDownloadPageError, request_driver_download_page},
};
use snafu::{ResultExt, Snafu};

mod compare_latest_driver_version;
mod download_latest_driver;
mod get_latest_driver_version;

use crate::{
    command::{
        compare_latest_driver_version::{
            CompareLatestDriverVersionError, compare_latest_driver_version,
        },
        download_latest_driver::{DownloadLatestDriverError, download_latest_driver},
        get_latest_driver_version::{GetLatestDriverVersionError, get_latest_driver_version},
    },
    config::CliConfig,
};

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    #[command(
        visible_alias = "gl",
        about = "Scrape and print the latest driver-version."
    )]
    GetLatestDriverVersion {
        #[arg(
            default_value_t = false,
            short,
            long,
            help = "Print the scraped driver-version formatted '1.2.3.4.n' to stdout with no trailing newline."
        )]
        scriptable_output: bool,
    },
    #[command(
        visible_alias = "cl",
        about = "Scrape and compare the latest driver-version to another version."
    )]
    CompareLatestDriverVersion {
        #[arg(short, long, help = "The driver-version to compare against.")]
        comparison_driver_version: DriverVersion,

        #[arg(
            default_value_t = false,
            short,
            long,
            help = "Print 'true' or 'false' to stdout with no trailing newline.",
            long_help = concat!(
                "Print 'true' or 'false' to stdout with no trailing newline.\n",
                '\n',
                "Compares the scraped driver version to the provided version:\n",
                "    - 'true'  if the scraped version is newer\n",
                "    - 'false' if the scraped version is equal or older",
            )
        )]
        scriptable_output: bool,
    },
    #[command(
        visible_alias = "dl",
        about = "Scrape and download the latest driver-download link."
    )]
    DownloadLatestDriver {
        #[arg(
            default_value = "./driver.exe",
            short,
            long,
            help = "Path to output the downloaded driver."
        )]
        driver_ouptut_path: Box<Path>,
    },
}

#[derive(Debug, Snafu)]
pub enum RunCommandError {
    #[snafu(display("failed to request the driver-download page"))]
    RequestDriverDownloadPage {
        source: RequestDriverDownloadPageError,
    },
    #[snafu(transparent)]
    GetLatestDriverVersion { source: GetLatestDriverVersionError },
    #[snafu(transparent)]
    CompareLatestDriverVersion {
        source: CompareLatestDriverVersionError,
    },
    #[snafu(transparent)]
    DownloadLatestDriver { source: DownloadLatestDriverError },
}

impl Command {
    pub fn run(&self, config: &CliConfig) -> Result<(), RunCommandError> {
        let driver_download_page = request_driver_download_page(&config.driver_page_config)
            .context(RequestDriverDownloadPageSnafu)?;

        Ok(match self {
            Self::GetLatestDriverVersion { scriptable_output } => get_latest_driver_version(
                &config.command_config.get_latest_driver_version,
                &config.scraper_config.driver_version,
                &driver_download_page,
                *scriptable_output,
            )?,
            Self::CompareLatestDriverVersion {
                comparison_driver_version,
                scriptable_output,
            } => compare_latest_driver_version(
                &config.command_config.compare_latest_driver_version,
                &config.scraper_config.driver_version,
                &driver_download_page,
                *scriptable_output,
                comparison_driver_version,
            )?,
            Self::DownloadLatestDriver { driver_ouptut_path } => download_latest_driver(
                &config.command_config.download_latest_driver,
                &config.scraper_config.driver_download_link,
                &driver_download_page,
                &driver_ouptut_path,
            )?,
        })
    }
}
