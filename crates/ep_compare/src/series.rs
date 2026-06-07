//! Numeric series comparison summaries.

use crate::Tolerance;

/// Comparison summary for two numeric series.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SeriesComparison {
    /// Number of compared samples.
    pub samples: usize,
    /// Maximum absolute difference.
    pub max_abs_delta: f64,
    /// First tolerance or length divergence, if any.
    pub first_divergence: Option<SeriesDivergence>,
    /// True when every sample is within tolerance.
    pub passed: bool,
}

/// First point where two numeric series diverged.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SeriesDivergence {
    /// Zero-based sample index where the divergence starts.
    pub index: usize,
    /// Expected value, absent when the expected series ended first.
    pub expected: Option<f64>,
    /// Observed value, absent when the observed series ended first.
    pub observed: Option<f64>,
    /// Absolute delta, absent for length-only divergence.
    pub abs_delta: Option<f64>,
}

/// Compares two equally-sized numeric series.
#[must_use]
pub fn compare_series(
    expected: &[f64],
    observed: &[f64],
    tolerance: Tolerance,
) -> SeriesComparison {
    let mut max_abs_delta: f64 = 0.0;
    let mut passed = expected.len() == observed.len();
    let mut first_divergence = None;

    for (index, (left, right)) in expected.iter().zip(observed).enumerate() {
        let delta = (left - right).abs();
        max_abs_delta = max_abs_delta.max(delta);
        if !tolerance.accepts(*left, *right) {
            passed = false;
            if first_divergence.is_none() {
                first_divergence = Some(SeriesDivergence {
                    index,
                    expected: Some(*left),
                    observed: Some(*right),
                    abs_delta: Some(delta),
                });
            }
        }
    }

    let samples = expected.len().min(observed.len());
    if expected.len() != observed.len() && first_divergence.is_none() {
        first_divergence = Some(SeriesDivergence {
            index: samples,
            expected: expected.get(samples).copied(),
            observed: observed.get(samples).copied(),
            abs_delta: None,
        });
    }

    SeriesComparison {
        samples,
        max_abs_delta,
        first_divergence,
        passed,
    }
}
