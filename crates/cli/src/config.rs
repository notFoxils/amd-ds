use amd_ds::config::{
    DriverDownloadLinkScraperConfig, DriverPageConfig, DriverReleaseNotesLinkScraperConfig,
    DriverVersionScraperConfig,
};
use amd_ds_common::RequestHeaders;
use snafu::{ResultExt, Snafu};
use std::{collections::HashMap, fs, io, path::Path};

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CliConfig {
    pub driver_page: DriverPageConfig,
    pub scraper: ScraperConfig,
    pub command: CommandConfig,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct ScraperConfig {
    pub version: DriverVersionScraperConfig,
    pub download_link: DriverDownloadLinkScraperConfig,
    pub driver_release_notes_link: DriverReleaseNotesLinkScraperConfig,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CommandConfig {
    pub download_latest_driver: DownloadLatestDriverConfig,
    pub get_latest_driver_version: GetLatestDriverVersionConfig,
    pub compare_latest_driver_version: CompareLatestDriverVersionConfig,
}

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct DownloadLatestDriverConfig {
    pub download_request_headers: RequestHeaders,
}

impl Default for DownloadLatestDriverConfig {
    fn default() -> Self {
        Self {
            download_request_headers: HashMap::from([
                (
                    "User-Agent",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.3",
                ),
                ("Referer", "https://www.amd.com/"),
            ].map(|(lhs, rhs)| (Box::from(lhs), Box::from(rhs)))),
        }
    }
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLatestDriverVersionConfig {}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CompareLatestDriverVersionConfig {}

#[derive(Debug, Snafu)]
pub enum InitConfigError {
    #[snafu(display("failed to read the config-contents of: '{path}'", path = path.display()))]
    Read { path: Box<Path>, source: io::Error },
    #[snafu(display("failed to parse the config-contents of: '{path}'", path = path.display()))]
    Parse {
        path: Box<Path>,
        source: toml::de::Error,
    },
}

pub fn init_config(path: &Path) -> Result<CliConfig, InitConfigError> {
    let config_contents = fs::read(path).context(ReadSnafu { path })?;

    toml::from_slice(&config_contents).context(ParseSnafu { path })
}
