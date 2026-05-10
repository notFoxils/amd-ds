mod driver_download_link;
mod driver_version;

pub use driver_download_link::{ScrapeDriverDownloadLinkError, scrape_driver_download_link};
pub use driver_version::{ScrapeDriverVersionError, scrape_driver_version};
