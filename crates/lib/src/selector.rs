use std::ops::{Deref, DerefMut};

use snafu::Snafu;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Selector(scraper::Selector);

#[derive(Debug, Snafu)]
#[snafu(display("could not parse a selector"), whatever)]
pub struct SelectorError {
    message: String,
}

impl From<scraper::error::SelectorErrorKind<'_>> for SelectorError {
    fn from(value: scraper::error::SelectorErrorKind<'_>) -> Self {
        Self {
            message: format!("{}", value),
        }
    }
}

impl Selector {
    pub fn parse(selectors: &str) -> Result<Selector, SelectorError> {
        Ok(Self(scraper::Selector::parse(selectors)?))
    }
}

impl Deref for Selector {
    type Target = scraper::Selector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Selector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
