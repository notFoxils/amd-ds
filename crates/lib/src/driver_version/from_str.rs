use std::{num::ParseIntError, str::FromStr};

use snafu::{ResultExt, Snafu};

use super::DriverVersion;

#[derive(Debug, Snafu)]
pub enum ParseDriverVersionError {
    #[snafu(display("the provided input '{input}' has no version elements"))]
    NoVersionElements { input: Box<str> },
    #[snafu(display(
        "the provided input '{input}' has an invalid version element '{invalid_element}'"
    ))]
    InvalidVersionElement {
        input: Box<str>,
        invalid_element: Box<str>,
        source: ParseIntError,
    },
}

impl FromStr for DriverVersion {
    type Err = ParseDriverVersionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return NoVersionElementsSnafu { input: s }.fail();
        }

        let version_elements = s
            .split('.')
            .map(|version_element_str| {
                u32::from_str(version_element_str).context(InvalidVersionElementSnafu {
                    input: s,
                    invalid_element: version_element_str,
                })
            })
            .collect::<Result<Box<[u32]>, ParseDriverVersionError>>()?;

        Ok(Self::new(&version_elements).expect("parsed an empty version_element"))
    }
}
