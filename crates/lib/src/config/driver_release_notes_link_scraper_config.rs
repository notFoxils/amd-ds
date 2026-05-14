use crate::selector::Selector;

const DEFAULT_DRIVER_RELEASE_NOTES_ANCHOR_SELECTOR: &str =
    ".accordion-item:first-of-type .card-body:first-of-type a:first-of-type";

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
