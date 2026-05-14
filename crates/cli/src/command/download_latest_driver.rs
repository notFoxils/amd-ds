use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

use amd_ds::{
    config::DriverDownloadLinkScraperConfig,
    driver_page::DriverPage,
    scrape::{ScrapeDriverDownloadLinkError, scrape_driver_download_link},
};
use amd_ds_common::{RequestError, request};
use snafu::{ResultExt, Snafu};

use crate::config::DownloadLatestDriverConfig;

#[derive(Debug, Snafu)]
pub enum DownloadLatestDriverError {
    #[snafu(display("failed to scrape the driver-download link"))]
    ScrapeDriverDownloadLink {
        source: ScrapeDriverDownloadLinkError,
    },
    #[snafu(display("failed to request the driver-download"))]
    RequestDriverDownload { source: RequestError },
    #[snafu(display("failed to get the driver-download data from the request"))]
    GetDriverDownloadRequestData { source: io::Error },
    #[snafu(display("failed to create the driver-download output file"))]
    CreateOutputFile { source: io::Error },
    #[snafu(display("failed to write to the driver-download output file"))]
    WriteOutputFile { source: io::Error },
}

pub fn download_latest_driver(
    config: &DownloadLatestDriverConfig,
    driver_download_link_scraper_config: &DriverDownloadLinkScraperConfig,
    driver_page: &DriverPage,
    output_file_path: &Path,
) -> Result<(), DownloadLatestDriverError> {
    let driver_download_link =
        scrape_driver_download_link(driver_download_link_scraper_config, driver_page)
            .context(ScrapeDriverDownloadLinkSnafu)?;

    let driver_download_request_data =
        request(&driver_download_link, &config.download_request_headers)
            .context(RequestDriverDownloadSnafu)?
            .bytes()
            .collect::<Result<Vec<_>, io::Error>>()
            .context(GetDriverDownloadRequestDataSnafu)?;

    File::create(output_file_path)
        .context(CreateOutputFileSnafu)?
        .write_all(&driver_download_request_data)
        .context(WriteOutputFileSnafu)?;

    Ok(())
}
