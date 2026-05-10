mod display;
pub mod from_str;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DriverVersion(Box<[u32]>);

impl DriverVersion {
    /// Creates a new [`DriverVersion`] from a slice of [`u32`].
    ///
    /// Returns [`None`] if `elements` is empty.
    pub fn new(elements: &[u32]) -> Option<Self> {
        if elements.is_empty() {
            return None;
        }

        let elements = Self::strip_leading_zeroes(elements);

        Some(Self(Box::from(elements)))
    }

    fn strip_leading_zeroes(elements: &[u32]) -> &[u32] {
        let Some(first_nonzero_index) = elements.iter().position(|&element| element != 0) else {
            return elements;
        };

        elements.split_at(first_nonzero_index).1
    }
}

impl Default for DriverVersion {
    fn default() -> Self {
        Self(Box::new([0]))
    }
}
