use std::ops::Deref;

#[derive(Debug)]
pub struct DriverDownloadPage(scraper::Html);

impl DriverDownloadPage {
    pub(crate) fn new(html: scraper::Html) -> Self {
        Self(html)
    }
}

impl Deref for DriverDownloadPage {
    type Target = scraper::Html;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
