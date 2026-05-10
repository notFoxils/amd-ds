use snafu::{ResultExt, Snafu};

use crate::{
    config::DriverDownloadLinkScraperConfig,
    driver_download_page::DriverDownloadPage,
    util::{
        get_anchor_link::{GetAnchorLinkError, get_anchor_link},
        select_first::{SelectFirst, SelectFirstError},
    },
};

#[derive(Debug, Snafu)]
pub enum ScrapeDriverDownloadLinkError {
    #[snafu(display("failed to select a driver-download anchor"))]
    SelectDriverDownloadAnchor { source: SelectFirstError },
    #[snafu(display("failed to get the anchor-link from the driver-download anchor"))]
    GetAnchorLink { source: GetAnchorLinkError },
}

pub fn scrape_driver_download_link(
    config: &DriverDownloadLinkScraperConfig,
    driver_download_page: &DriverDownloadPage,
) -> Result<String, ScrapeDriverDownloadLinkError> {
    let selected_element = driver_download_page
        .select_first(&config.anchor_selector)
        .context(SelectDriverDownloadAnchorSnafu)?;

    get_anchor_link(selected_element.value())
        .map(str::to_string)
        .context(GetAnchorLinkSnafu)
}
