//! Remaining actual constructor cells consumed at normal program completion.
#![allow(dead_code)]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SourceRecurringErrorState {
    pub(crate) number: i32,
    pub(crate) rows: Option<Vec<std::convert::Infallible>>,
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
