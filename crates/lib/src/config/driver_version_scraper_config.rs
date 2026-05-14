use crate::selector::Selector;

const DEFAULT_DRIVER_VERSION_ELEMENT_SELECTOR: &str =
    ".accordion-item:first-of-type .driver-download-details:first-of-type .row p:first-of-type";

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
