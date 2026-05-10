use amd_ds_common::RequestHeaders;

use crate::selector::Selector;

const DEFAULT_DRIVER_PAGE_URL: &str =
    "https://www.amd.com/en/support/downloads/drivers.html/chipsets/am5/x870e.html";

const DEFAULT_DRIVER_DOWNLOAD_ANCHOR_SELECTOR: &str =
    ".accordion-item:first-of-type .driver-download-details:first-of-type .button a:first-of-type";
const DEFAULT_DRIVER_RELEASE_NOTES_ANCHOR_SELECTOR: &str =
    ".accordion-item:first-of-type .card-body:first-of-type a:first-of-type";
const DEFAULT_DRIVER_VERSION_ELEMENT_SELECTOR: &str =
    ".accordion-item:first-of-type .driver-download-details:first-of-type .row p:first-of-type";

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DriverPageConfig {
    pub url: Box<str>,
    pub request_headers: RequestHeaders,
}

impl Default for DriverPageConfig {
    fn default() -> Self {
        Self {
            url: Box::from(DEFAULT_DRIVER_PAGE_URL),
            request_headers: Default::default(),
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DriverVersionScraperConfig {
    pub element_selector: Selector,
}

impl Default for DriverVersionScraperConfig {
    fn default() -> Self {
        Self {
            element_selector: Selector::parse(DEFAULT_DRIVER_VERSION_ELEMENT_SELECTOR).unwrap(),
        }
    }
}

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

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct DriverReleaseNotesLinkScraperConfig {
    pub anchor_selector: Selector,
}

impl Default for DriverReleaseNotesLinkScraperConfig {
    fn default() -> Self {
        Self {
            anchor_selector: Selector::parse(DEFAULT_DRIVER_RELEASE_NOTES_ANCHOR_SELECTOR).unwrap(),
        }
    }
}
