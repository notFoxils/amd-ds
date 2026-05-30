use snafu::{ResultExt, Snafu};

use amd_ds_common::select_first::{SelectFirst, SelectFirstError};

use crate::{
    config::DriverReleaseNotesLinkScraperConfig,
    driver_page::DriverPage,
    util::get_anchor_link::{GetAnchorLinkError, get_anchor_link},
};

#[derive(Debug, Snafu)]
pub enum ScrapeDriverReleaseNotesLinkError {
    #[snafu(display("failed to select a driver-download anchor"))]
    SelectDriverDownloadAnchor { source: SelectFirstError },
    #[snafu(display("failed to get the anchor-link from the driver-download anchor"))]
    GetAnchorLink { source: GetAnchorLinkError },
}

pub fn scrape_driver_release_notes_link(
    config: &DriverReleaseNotesLinkScraperConfig,
    driver_page: &DriverPage,
) -> Result<String, ScrapeDriverReleaseNotesLinkError> {
    let selected_element = driver_page
        .select_first(&config.anchor_selector)
        .context(SelectDriverDownloadAnchorSnafu)?;

    get_anchor_link(selected_element.value())
        .map(str::to_string)
        .context(GetAnchorLinkSnafu)
}
