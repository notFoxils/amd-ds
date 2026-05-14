use std::io::{self, Read};

use amd_ds_common::{RequestError, request};
use snafu::{ResultExt, Snafu};

use crate::{config::DriverPageConfig, driver_page::DriverPage};

#[derive(Debug, Snafu)]
pub enum RequestDriverPageError {
    #[snafu(display("failed to request the document"))]
    Request { source: RequestError },
    #[snafu(display("failed to read the document"))]
    Read { source: io::Error },
}

pub fn request_driver_page(
    config: &DriverPageConfig,
) -> Result<DriverPage, RequestDriverPageError> {
    let mut page_content = String::new();
    request(&config.url, &config.request_headers)
        .context(RequestSnafu)?
        .read_to_string(&mut page_content)
        .context(ReadSnafu)?;

    Ok(DriverPage::new(scraper::Html::parse_document(
        &page_content,
    )))
}
