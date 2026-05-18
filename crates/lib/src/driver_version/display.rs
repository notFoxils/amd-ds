use std::fmt::{self, Display};

use super::DriverVersion;

impl Display for DriverVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut version_elements = self.0.iter();
        if let Some(first_element) = version_elements.next() {
            write!(f, "{first_element}")?;
        } else {
            return Ok(());
        }

        for version_element in version_elements {
            write!(f, ".{version_element}")?;
        }

        Ok(())
    }
}
