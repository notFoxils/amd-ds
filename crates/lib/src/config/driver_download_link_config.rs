use crate::selector::Selector;

const DEFAULT_DRIVER_DOWNLOAD_ANCHOR_SELECTOR: &str =
    ".accordion-item:first-of-type .driver-download-details:first-of-type .button a:first-of-type";

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DriverDownloadLinkScraperConfig {
    pub anchor_selector: Selector,
}

impl Default for DriverDownloadLinkScraperConfig {
    fn default() -> Self {
        Self {
            anchor_selector: Selector::parse(DEFAULT_DRIVER_DOWNLOAD_ANCHOR_SELECTOR).unwrap(),
        }
    }
}
