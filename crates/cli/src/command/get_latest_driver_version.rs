use amd_ds::{
    config::DriverVersionScraperConfig,
    driver_page::DriverPage,
    scrape::{ScrapeDriverVersionError, scrape_driver_version},
};
use snafu::{ResultExt, Snafu};

use crate::config::GetLatestDriverVersionConfig;

#[derive(Debug, Snafu)]
pub enum GetLatestDriverVersionError {
    #[snafu(display("failed to scrape the current driver version"))]
    ScrapeDriverVersion { source: ScrapeDriverVersionError },
}

pub fn get_latest_driver_version(
    _config: &GetLatestDriverVersionConfig,
    driver_version_scraper_config: &DriverVersionScraperConfig,
    driver_page: &DriverPage,
    scriptable_output: bool,
) -> Result<(), GetLatestDriverVersionError> {
    let driver_version = scrape_driver_version(driver_version_scraper_config, driver_page)
        .context(ScrapeDriverVersionSnafu)?;

    if scriptable_output {
        print!("{driver_version}");
    } else {
        println!("The latest driver version is {driver_version}.");
    }

    Ok(())
}
