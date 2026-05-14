use std::ops::Deref;

#[derive(Debug)]
pub struct DriverPage(scraper::Html);

impl DriverPage {
    pub(crate) fn new(html: scraper::Html) -> Self {
        Self(html)
    }
}

impl Deref for DriverPage {
    type Target = scraper::Html;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
