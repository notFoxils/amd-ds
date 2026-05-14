use std::str::FromStr;

use snafu::{ResultExt, Snafu};

use crate::{
    config::DriverVersionScraperConfig,
    driver_page::DriverPage,
    driver_version::{DriverVersion, from_str::ParseDriverVersionError},
    util::select_first::{SelectFirst, SelectFirstError},
};

#[derive(Debug, Snafu)]
pub enum ScrapeDriverVersionError {
    #[snafu(display("failed to select a driver-version element"))]
    SelectDriverVersionElement { source: SelectFirstError },
    #[snafu(display("failed to parse a driver-version"))]
    ParseDriverVersion { source: ParseDriverVersionError },
}

pub fn scrape_driver_version(
    config: &DriverVersionScraperConfig,
    driver_page: &DriverPage,
) -> Result<DriverVersion, ScrapeDriverVersionError> {
    let selected_element = driver_page
        .select_first(&config.element_selector)
        .context(SelectDriverVersionElementSnafu)?;

    DriverVersion::from_str(&selected_element.text().collect::<Box<str>>())
        .context(ParseDriverVersionSnafu)
}
