use amd_ds_common::RequestHeaders;

const DEFAULT_DRIVER_PAGE_URL: &str =
    "https://www.amd.com/en/support/downloads/drivers.html/chipsets/am5/x870e.html";

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
