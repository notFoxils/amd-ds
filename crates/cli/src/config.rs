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
    pub release_notes_link: DriverReleaseNotesLinkScraperConfig,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CommandConfig {
    pub get_latest_driver_version: GetLatestDriverVersionConfig,
    pub compare_latest_driver_version: CompareLatestDriverVersionConfig,
    pub download_latest_driver: DownloadLatestDriverConfig,
    pub download_latest_release_notes: DownloadLatestReleaseNotesConfig,
}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct GetLatestDriverVersionConfig {}

#[derive(Debug, Default, serde::Deserialize)]
#[serde(default)]
pub struct CompareLatestDriverVersionConfig {}

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

const DEFAULT_RELEASE_NOTES_CONTAINER_ELEMENT_SELECTOR: &str = ".content-wrapper:first-of-type .cmp-container__content:first-of-type .text:first-of-type .cmp-text:first-of-type";

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct DownloadLatestReleaseNotesConfig {
    pub release_notes_page: ReleaseNotesPageConfig,
    pub container_element_selector: scraper::Selector,
}

impl Default for DownloadLatestReleaseNotesConfig {
    fn default() -> Self {
        Self {
            release_notes_page: ReleaseNotesPageConfig::default(),
            container_element_selector: scraper::Selector::parse(
                DEFAULT_RELEASE_NOTES_CONTAINER_ELEMENT_SELECTOR,
            )
            .unwrap(),
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(default)]
pub struct ReleaseNotesPageConfig {
    pub request_headers: RequestHeaders,
}

impl Default for ReleaseNotesPageConfig {
    fn default() -> Self {
        Self {
            request_headers: HashMap::from([
                (
                    "User-Agent",
                    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.3",
                ),
            ].map(|(lhs, rhs)| (Box::from(lhs), Box::from(rhs)))),
        }
    }
}

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
