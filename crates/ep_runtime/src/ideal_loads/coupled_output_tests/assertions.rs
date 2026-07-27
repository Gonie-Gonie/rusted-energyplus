use crate::OutputSeries;

const ABS_TOLERANCE: f64 = 1.0e-9;

pub(super) fn assert_values(series: &OutputSeries, expected: &[f64]) {
    assert_eq!(series.values.len(), expected.len());
    for (actual, expected) in series.values.iter().zip(expected) {
        assert!(
            (actual - expected).abs() <= ABS_TOLERANCE,
            "expected {expected}, got {actual} for {}",
            series.variable_name
        );
    }
}
