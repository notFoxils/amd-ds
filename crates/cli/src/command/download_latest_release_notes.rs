use std::{
    fs::File,
    io::{self, Read, Write},
    ops::Deref,
    path::Path,
};

use amd_ds::{
    config::DriverReleaseNotesLinkScraperConfig,
    driver_page::DriverPage,
    scrape::{ScrapeDriverReleaseNotesLinkError, scrape_driver_release_notes_link},
};
use amd_ds_common::{
    RequestError, request,
    select_first::{SelectFirst, SelectFirstError},
};
use snafu::{ResultExt, Snafu};

use crate::config::DownloadLatestReleaseNotesConfig;

#[derive(Debug, Snafu)]
pub enum DownloadLatestReleaseNotesError {
    #[snafu(display("failed to scrape the driver release-notes link"))]
    ScrapeReleaseNotesLink {
        source: ScrapeDriverReleaseNotesLinkError,
    },
    #[snafu(display("failed to request the release-notes page"))]
    RequestReleaseNotesPage { source: RequestError },
    #[snafu(display("failed to read the release-notes page's data"))]
    ReadReleaseNotesPageData { source: io::Error },
    #[snafu(display("failed to select the release-notes container-element"))]
    SelectReleaseNotesContainerElement { source: SelectFirstError },
    #[snafu(display("failed to create the release-notes output file"))]
    CreateOutputFile { source: io::Error },
    #[snafu(display("failed to write to the release-notes output file"))]
    WriteOutputFile { source: io::Error },
}

pub fn download_latest_release_notes(
    config: &DownloadLatestReleaseNotesConfig,
    driver_release_notes_link_scraper_config: &DriverReleaseNotesLinkScraperConfig,
    driver_page: &DriverPage,
    output_file_path: &Path,
) -> Result<(), DownloadLatestReleaseNotesError> {
    let driver_release_notes_link =
        scrape_driver_release_notes_link(driver_release_notes_link_scraper_config, driver_page)
            .context(ScrapeReleaseNotesLinkSnafu)?;

    let release_notes_page = {
        let mut release_notes_page_data = String::new();
        request(
            &driver_release_notes_link,
            &config.release_notes_page.request_headers,
        )
        .context(RequestReleaseNotesPageSnafu)?
        .deref()
        .read_to_string(&mut release_notes_page_data)
        .context(ReadReleaseNotesPageDataSnafu)?;

        scraper::Html::parse_document(&release_notes_page_data)
    };

    let release_notes_container_content = release_notes_page
        .select_first(&config.container_element_selector)
        .context(SelectReleaseNotesContainerElementSnafu)?
        .inner_html();

    let release_notes_markdown = html2md::parse_html(&release_notes_container_content);

    File::create(output_file_path)
        .context(CreateOutputFileSnafu)?
        .write_all(release_notes_markdown.as_bytes())
        .context(WriteOutputFileSnafu)?;

    Ok(())
}
