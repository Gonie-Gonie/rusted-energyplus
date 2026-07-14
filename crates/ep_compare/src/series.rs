//! Numeric series comparison summaries.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

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

/// Reason the ordered timestamp contract first diverged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OrderedTimestampDivergenceReason {
    /// An expected sample exists but has no timestamp label.
    MissingExpectedTimestamp,
    /// An observed sample exists but has no timestamp label.
    MissingObservedTimestamp,
    /// A timestamp label occurs more than once in the expected slice.
    DuplicateExpectedTimestamp,
    /// A timestamp label occurs more than once in the observed slice.
    DuplicateObservedTimestamp,
    /// The observed slice has a sample after the expected slice ended.
    MissingExpectedSample,
    /// The expected slice has a sample after the observed slice ended.
    MissingObservedSample,
    /// The timestamp strings at the same slice index are not exactly equal.
    TimestampMismatch,
}

impl OrderedTimestampDivergenceReason {
    /// Returns a stable label suitable for machine-readable reports.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingExpectedTimestamp => "missing_expected_timestamp",
            Self::MissingObservedTimestamp => "missing_observed_timestamp",
            Self::DuplicateExpectedTimestamp => "duplicate_expected_timestamp",
            Self::DuplicateObservedTimestamp => "duplicate_observed_timestamp",
            Self::MissingExpectedSample => "missing_expected_sample",
            Self::MissingObservedSample => "missing_observed_sample",
            Self::TimestampMismatch => "timestamp_mismatch",
        }
    }
}

impl fmt::Display for OrderedTimestampDivergenceReason {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// First violation of the ordered, exact, unique timestamp contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OrderedTimestampDivergence {
    /// Zero-based slice index where the timestamp contract first diverged.
    pub index: usize,
    /// Expected timestamp at the divergent index, if present.
    pub expected: Option<String>,
    /// Observed timestamp at the divergent index, if present.
    pub observed: Option<String>,
    /// Timestamp contract violation at this index.
    pub reason: OrderedTimestampDivergenceReason,
}

/// Ordered timestamp contract result paired with same-index numeric metrics.
///
/// This is intentionally separate from [`compare_series_samples_v2`]. The
/// existing comparator remains timestamp-set aligned and order-insensitive,
/// while this result requires complete, unique timestamp slices whose strings
/// match exactly at every slice index.
#[derive(Clone, Debug, PartialEq)]
pub struct OrderedTimestampComparison {
    /// Same-index value comparison, reported as timestamp alignment.
    pub comparison: SeriesComparisonV2,
    /// Status of the timestamp-only contract.
    pub contract_status: SeriesComparisonStatus,
    /// Whether all present expected timestamp labels are unique.
    pub expected_unique_timestamps: bool,
    /// Whether all present observed timestamp labels are unique.
    pub observed_unique_timestamps: bool,
    /// Whether lengths match and every same-index timestamp string is present and exact.
    pub timestamp_order_match: bool,
    /// First timestamp contract divergence, independent of numeric tolerance.
    pub first_timestamp_divergence: Option<OrderedTimestampDivergence>,
}

impl OrderedTimestampComparison {
    /// Returns true when both the timestamp contract and numeric tolerance pass.
    #[must_use]
    pub fn passed(&self) -> bool {
        self.contract_status == SeriesComparisonStatus::Pass && self.comparison.passed()
    }

    /// Returns true when only the ordered timestamp contract passed.
    #[must_use]
    pub fn timestamp_contract_passed(&self) -> bool {
        self.contract_status == SeriesComparisonStatus::Pass
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

/// Compares complete, unique timestamps in exact slice order and values by the
/// same slice index.
///
/// Unlike [`compare_series_samples_v2`], this contract never sorts timestamps
/// and never coalesces duplicate labels. Numeric metrics, including RMSE, are
/// returned through [`OrderedTimestampComparison::comparison`].
#[must_use]
pub fn compare_ordered_timestamp_samples_v2(
    expected: &[SeriesSample],
    observed: &[SeriesSample],
    tolerance: Tolerance,
) -> OrderedTimestampComparison {
    let expected_unique_timestamps = timestamps_are_unique(expected);
    let observed_unique_timestamps = timestamps_are_unique(observed);
    let timestamp_order_match = timestamps_match_in_exact_order(expected, observed);
    let first_timestamp_divergence = first_ordered_timestamp_divergence(expected, observed);
    let contract_status = if first_timestamp_divergence.is_none() {
        SeriesComparisonStatus::Pass
    } else {
        SeriesComparisonStatus::Fail
    };

    let mut comparison = compare_indexed_samples(expected, observed, tolerance);
    comparison.alignment = SeriesAlignment::Timestamp;
    if let Some(divergence) = comparison.first_divergence.as_mut() {
        divergence.timestamp = expected
            .get(divergence.index)
            .and_then(|sample| sample.timestamp.clone())
            .or_else(|| {
                observed
                    .get(divergence.index)
                    .and_then(|sample| sample.timestamp.clone())
            });
    }

    OrderedTimestampComparison {
        comparison,
        contract_status,
        expected_unique_timestamps,
        observed_unique_timestamps,
        timestamp_order_match,
        first_timestamp_divergence,
    }
}

fn timestamps_are_unique(samples: &[SeriesSample]) -> bool {
    let mut timestamps = BTreeSet::new();
    samples.iter().all(|sample| {
        sample
            .timestamp
            .as_deref()
            .is_none_or(|timestamp| timestamps.insert(timestamp))
    })
}

fn timestamps_match_in_exact_order(expected: &[SeriesSample], observed: &[SeriesSample]) -> bool {
    expected.len() == observed.len()
        && expected.iter().zip(observed).all(|(left, right)| {
            matches!(
                (left.timestamp.as_deref(), right.timestamp.as_deref()),
                (Some(expected_timestamp), Some(observed_timestamp))
                    if expected_timestamp == observed_timestamp
            )
        })
}

fn first_ordered_timestamp_divergence(
    expected: &[SeriesSample],
    observed: &[SeriesSample],
) -> Option<OrderedTimestampDivergence> {
    let mut expected_timestamps = BTreeSet::new();
    let mut observed_timestamps = BTreeSet::new();

    for index in 0..expected.len().max(observed.len()) {
        let expected_sample = expected.get(index);
        let observed_sample = observed.get(index);
        let expected_timestamp = expected_sample.and_then(|sample| sample.timestamp.as_deref());
        let observed_timestamp = observed_sample.and_then(|sample| sample.timestamp.as_deref());

        let duplicate_expected = expected_timestamp
            .map(|timestamp| !expected_timestamps.insert(timestamp))
            .unwrap_or(false);
        let duplicate_observed = observed_timestamp
            .map(|timestamp| !observed_timestamps.insert(timestamp))
            .unwrap_or(false);

        let reason = if duplicate_expected {
            Some(OrderedTimestampDivergenceReason::DuplicateExpectedTimestamp)
        } else if duplicate_observed {
            Some(OrderedTimestampDivergenceReason::DuplicateObservedTimestamp)
        } else if expected_sample.is_some() && expected_timestamp.is_none() {
            Some(OrderedTimestampDivergenceReason::MissingExpectedTimestamp)
        } else if observed_sample.is_some() && observed_timestamp.is_none() {
            Some(OrderedTimestampDivergenceReason::MissingObservedTimestamp)
        } else if expected_sample.is_none() {
            Some(OrderedTimestampDivergenceReason::MissingExpectedSample)
        } else if observed_sample.is_none() {
            Some(OrderedTimestampDivergenceReason::MissingObservedSample)
        } else if expected_timestamp != observed_timestamp {
            Some(OrderedTimestampDivergenceReason::TimestampMismatch)
        } else {
            None
        };

        if let Some(reason) = reason {
            return Some(OrderedTimestampDivergence {
                index,
                expected: expected_timestamp.map(str::to_owned),
                observed: observed_timestamp.map(str::to_owned),
                reason,
            });
        }
    }

    None
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
