use amd_ds::{
    config::DriverVersionScraperConfig,
    driver_page::DriverPage,
    driver_version::DriverVersion,
    scrape::{ScrapeDriverVersionError, scrape_driver_version},
};
use snafu::{ResultExt, Snafu};

use crate::config::CompareLatestDriverVersionConfig;

#[derive(Debug, Snafu)]
pub enum CompareLatestDriverVersionError {
    #[snafu(display("failed to scrape the current driver version"))]
    ScrapeDriverVersion { source: ScrapeDriverVersionError },
}

pub fn compare_latest_driver_version(
    _config: &CompareLatestDriverVersionConfig,
    driver_version_scraper_config: &DriverVersionScraperConfig,
    driver_page: &DriverPage,
    scriptable_output: bool,
    comparison_driver_version: &DriverVersion,
) -> Result<(), CompareLatestDriverVersionError> {
    let driver_version = scrape_driver_version(driver_version_scraper_config, driver_page)
        .context(ScrapeDriverVersionSnafu)?;

    if !scriptable_output {
        if driver_version > *comparison_driver_version {
            println!("There is a new driver version ({driver_version}) available.");
        } else {
            println!(
                "The available driver ({driver_version}) is {phrase} the comparison version ({comparison_driver_version}).",
                phrase = if driver_version < *comparison_driver_version {
                    "older than"
                } else {
                    "equal to"
                }
            );
        }
    } else {
        print!(
            "{is_newer}",
            is_newer = driver_version > *comparison_driver_version
        );
    }

    Ok(())
}
