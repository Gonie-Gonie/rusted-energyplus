//! Remaining actual constructor cells consumed at normal program completion.
#![allow(dead_code)]
/// DataErrorTracking.hh185-202: exactly13 actual class members/defaults.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceRecurringErrorData {
    pub(crate) message: String,
    pub(crate) count: i32,
    pub(crate) warmup_count: i32,
    pub(crate) sizing_count: i32,
    pub(crate) max_value: f64,
    pub(crate) min_value: f64,
    pub(crate) sum_value: f64,
    pub(crate) max_units: String,
    pub(crate) min_units: String,
    pub(crate) sum_units: String,
    pub(crate) report_max: bool,
    pub(crate) report_min: bool,
    pub(crate) report_sum: bool,
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceRecurringErrorState {
    pub(crate) number: i32,
    pub(crate) rows: Option<Vec<SourceRecurringErrorData>>,
}
/// Native ResultsFramework::SimInfo's real string members, no reporting snapshots.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SourceSimulationInformation {
    pub(crate) program_version: String,
    pub(crate) simulation_environment: String,
    pub(crate) input_model_uri: String,
    pub(crate) start_date_time_stamp: String,
    pub(crate) run_time: String,
    pub(crate) warnings_warmup: String,
    pub(crate) severe_warmup: String,
    pub(crate) warnings_sizing: String,
    pub(crate) severe_sizing: String,
    pub(crate) warnings: String,
    pub(crate) severe: String,
}
