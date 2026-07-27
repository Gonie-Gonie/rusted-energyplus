use ep_model::OutputHandle;

use crate::{OutputSeries, ResultStore};

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

pub(super) fn sentinel_results(handle: OutputHandle) -> ResultStore {
    let mut results = ResultStore::new();
    results.add_series(OutputSeries {
        handle,
        key: "EXISTING".to_string(),
        variable_name: "Existing Variable".to_string(),
        units: "W".to_string(),
        values: vec![1.0],
    });
    results
}
