//! Numeric series comparison summaries.

use std::collections::{BTreeMap, BTreeSet};

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

/// One numeric sample with an optional alignment timestamp.
#[derive(Clone, Debug, PartialEq)]
pub struct SeriesSample {
    /// Zero-based sample index in the source artifact.
    pub index: usize,
    /// Optional timestamp label used for cross-artifact alignment.
    pub timestamp: Option<String>,
    /// Numeric sample value.
    pub value: f64,
}

impl SeriesSample {
    /// Builds a sample with no timestamp.
    #[must_use]
    pub fn indexed(index: usize, value: f64) -> Self {
        Self {
            index,
            timestamp: None,
            value,
        }
    }

    /// Builds a sample with a timestamp label.
    #[must_use]
    pub fn timestamped(index: usize, timestamp: impl Into<String>, value: f64) -> Self {
        Self {
            index,
            timestamp: Some(timestamp.into()),
            value,
        }
    }
}

/// Alignment mode selected by the v2 compare engine.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeriesAlignment {
    /// Samples were compared by zero-based index.
    Index,
    /// Samples were aligned by timestamp label.
    Timestamp,
}

/// Status of a v2 series comparison.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeriesComparisonStatus {
    /// All aligned samples passed tolerance and no sample was missing.
    Pass,
    /// At least one aligned sample failed tolerance or a sample was missing.
    Fail,
}

/// Reason for the first v2 divergence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeriesDivergenceKind {
    /// Both samples were present but exceeded tolerance.
    Tolerance,
    /// The expected series had no sample for the aligned index or timestamp.
    MissingExpectedSample,
    /// The observed series had no sample for the aligned index or timestamp.
    MissingObservedSample,
}

/// First timestamp-aware divergence in a v2 comparison.
#[derive(Clone, Debug, PartialEq)]
pub struct SeriesDivergenceV2 {
    /// Zero-based aligned sample index where the divergence starts.
    pub index: usize,
    /// Optional timestamp label for timestamp-aligned comparisons.
    pub timestamp: Option<String>,
    /// Divergence reason.
    pub kind: SeriesDivergenceKind,
    /// Expected value, absent when the expected series is missing the sample.
    pub expected: Option<f64>,
    /// Observed value, absent when the observed series is missing the sample.
    pub observed: Option<f64>,
    /// Absolute delta, absent for missing-sample divergences.
    pub abs_delta: Option<f64>,
    /// Relative delta, absent for missing-sample divergences.
    pub rel_delta: Option<f64>,
}

/// v2 comparison summary with timestamp alignment and richer metrics.
#[derive(Clone, Debug, PartialEq)]
pub struct SeriesComparisonV2 {
    /// Alignment mode used for the comparison.
    pub alignment: SeriesAlignment,
    /// Number of samples in the expected series.
    pub expected_samples: usize,
    /// Number of samples in the observed series.
    pub observed_samples: usize,
    /// Number of sample pairs that were numerically compared.
    pub compared_samples: usize,
    /// Maximum absolute delta across compared samples.
    pub max_abs_delta: f64,
    /// Root mean square delta across compared samples.
    pub rmse_delta: f64,
    /// Maximum relative delta across compared samples.
    pub max_rel_delta: f64,
    /// First tolerance or missing-sample divergence.
    pub first_divergence: Option<SeriesDivergenceV2>,
    /// Final comparison status.
    pub status: SeriesComparisonStatus,
}

impl SeriesComparisonV2 {
    /// Returns true when the v2 comparison passed.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.status == SeriesComparisonStatus::Pass
    }
}

/// Compares raw numeric slices with the v2 metric engine using index alignment.
#[must_use]
pub fn compare_series_v2(
    expected: &[f64],
    observed: &[f64],
    tolerance: Tolerance,
) -> SeriesComparisonV2 {
    let expected_samples = expected
        .iter()
        .copied()
        .enumerate()
        .map(|(index, value)| SeriesSample::indexed(index, value))
        .collect::<Vec<_>>();
    let observed_samples = observed
        .iter()
        .copied()
        .enumerate()
        .map(|(index, value)| SeriesSample::indexed(index, value))
        .collect::<Vec<_>>();
    compare_series_samples_v2(&expected_samples, &observed_samples, tolerance)
}

/// Compares samples by timestamp when both series are fully timestamped,
/// otherwise by zero-based index.
#[must_use]
pub fn compare_series_samples_v2(
    expected: &[SeriesSample],
    observed: &[SeriesSample],
    tolerance: Tolerance,
) -> SeriesComparisonV2 {
    if all_samples_have_timestamps(expected) && all_samples_have_timestamps(observed) {
        compare_timestamped_samples(expected, observed, tolerance)
    } else {
        compare_indexed_samples(expected, observed, tolerance)
    }
}

fn all_samples_have_timestamps(samples: &[SeriesSample]) -> bool {
    !samples.is_empty() && samples.iter().all(|sample| sample.timestamp.is_some())
}

fn compare_indexed_samples(
    expected: &[SeriesSample],
    observed: &[SeriesSample],
    tolerance: Tolerance,
) -> SeriesComparisonV2 {
    let mut accumulator = SeriesMetricAccumulator::default();
    let mut first_divergence = None;
    let compared_samples = expected.len().min(observed.len());

    for index in 0..compared_samples {
        let left = expected[index].value;
        let right = observed[index].value;
        let (abs_delta, rel_delta) = accumulator.record(left, right);
        if !tolerance.accepts(left, right) && first_divergence.is_none() {
            first_divergence = Some(SeriesDivergenceV2 {
                index,
                timestamp: None,
                kind: SeriesDivergenceKind::Tolerance,
                expected: Some(left),
                observed: Some(right),
                abs_delta: Some(abs_delta),
                rel_delta: Some(rel_delta),
            });
        }
    }

    if expected.len() != observed.len() && first_divergence.is_none() {
        first_divergence = Some(SeriesDivergenceV2 {
            index: compared_samples,
            timestamp: None,
            kind: if expected.len() < observed.len() {
                SeriesDivergenceKind::MissingExpectedSample
            } else {
                SeriesDivergenceKind::MissingObservedSample
            },
            expected: expected.get(compared_samples).map(|sample| sample.value),
            observed: observed.get(compared_samples).map(|sample| sample.value),
            abs_delta: None,
            rel_delta: None,
        });
    }

    accumulator.finish(
        SeriesAlignment::Index,
        expected.len(),
        observed.len(),
        first_divergence,
    )
}

fn compare_timestamped_samples(
    expected: &[SeriesSample],
    observed: &[SeriesSample],
    tolerance: Tolerance,
) -> SeriesComparisonV2 {
    let expected_by_timestamp = timestamp_map(expected);
    let observed_by_timestamp = timestamp_map(observed);
    let timestamps = expected_by_timestamp
        .keys()
        .chain(observed_by_timestamp.keys())
        .cloned()
        .collect::<BTreeSet<_>>();

    let mut accumulator = SeriesMetricAccumulator::default();
    let mut first_divergence = None;

    for (index, timestamp) in timestamps.iter().enumerate() {
        match (
            expected_by_timestamp.get(timestamp),
            observed_by_timestamp.get(timestamp),
        ) {
            (Some(left), Some(right)) => {
                let (abs_delta, rel_delta) = accumulator.record(left.value, right.value);
                if !tolerance.accepts(left.value, right.value) && first_divergence.is_none() {
                    first_divergence = Some(SeriesDivergenceV2 {
                        index,
                        timestamp: Some(timestamp.clone()),
                        kind: SeriesDivergenceKind::Tolerance,
                        expected: Some(left.value),
                        observed: Some(right.value),
                        abs_delta: Some(abs_delta),
                        rel_delta: Some(rel_delta),
                    });
                }
            }
            (None, Some(right)) if first_divergence.is_none() => {
                first_divergence = Some(SeriesDivergenceV2 {
                    index,
                    timestamp: Some(timestamp.clone()),
                    kind: SeriesDivergenceKind::MissingExpectedSample,
                    expected: None,
                    observed: Some(right.value),
                    abs_delta: None,
                    rel_delta: None,
                });
            }
            (Some(left), None) if first_divergence.is_none() => {
                first_divergence = Some(SeriesDivergenceV2 {
                    index,
                    timestamp: Some(timestamp.clone()),
                    kind: SeriesDivergenceKind::MissingObservedSample,
                    expected: Some(left.value),
                    observed: None,
                    abs_delta: None,
                    rel_delta: None,
                });
            }
            (None, None) | (None, Some(_)) | (Some(_), None) => {}
        }
    }

    accumulator.finish(
        SeriesAlignment::Timestamp,
        expected.len(),
        observed.len(),
        first_divergence,
    )
}

fn timestamp_map(samples: &[SeriesSample]) -> BTreeMap<String, &SeriesSample> {
    samples
        .iter()
        .filter_map(|sample| {
            sample
                .timestamp
                .as_ref()
                .map(|timestamp| (timestamp.clone(), sample))
        })
        .collect()
}

#[derive(Default)]
struct SeriesMetricAccumulator {
    compared_samples: usize,
    max_abs_delta: f64,
    max_rel_delta: f64,
    sum_squared_delta: f64,
}

impl SeriesMetricAccumulator {
    fn record(&mut self, expected: f64, observed: f64) -> (f64, f64) {
        let abs_delta = (expected - observed).abs();
        let rel_delta = relative_delta(expected, observed, abs_delta);
        self.compared_samples += 1;
        self.max_abs_delta = self.max_abs_delta.max(abs_delta);
        self.max_rel_delta = self.max_rel_delta.max(rel_delta);
        self.sum_squared_delta += abs_delta * abs_delta;
        (abs_delta, rel_delta)
    }

    fn finish(
        self,
        alignment: SeriesAlignment,
        expected_samples: usize,
        observed_samples: usize,
        first_divergence: Option<SeriesDivergenceV2>,
    ) -> SeriesComparisonV2 {
        let rmse_delta = if self.compared_samples == 0 {
            0.0
        } else {
            (self.sum_squared_delta / self.compared_samples as f64).sqrt()
        };
        let status = if first_divergence.is_none() {
            SeriesComparisonStatus::Pass
        } else {
            SeriesComparisonStatus::Fail
        };

        SeriesComparisonV2 {
            alignment,
            expected_samples,
            observed_samples,
            compared_samples: self.compared_samples,
            max_abs_delta: self.max_abs_delta,
            rmse_delta,
            max_rel_delta: self.max_rel_delta,
            first_divergence,
            status,
        }
    }
}

fn relative_delta(expected: f64, observed: f64, abs_delta: f64) -> f64 {
    let scale = expected.abs().max(observed.abs());
    if scale == 0.0 { 0.0 } else { abs_delta / scale }
}
