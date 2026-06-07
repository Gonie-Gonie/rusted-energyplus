//! Tolerance policies for numeric comparisons.

/// Absolute and relative tolerance policy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tolerance {
    /// Absolute tolerance.
    pub absolute: f64,
    /// Relative tolerance.
    pub relative: f64,
}

impl Tolerance {
    /// Returns true when two values are within tolerance.
    #[must_use]
    pub fn accepts(self, expected: f64, observed: f64) -> bool {
        let delta = (expected - observed).abs();
        if delta <= self.absolute {
            return true;
        }

        let scale = expected.abs().max(observed.abs());
        delta <= self.relative * scale
    }
}

impl Default for Tolerance {
    fn default() -> Self {
        Self {
            absolute: 1.0e-9,
            relative: 1.0e-6,
        }
    }
}
