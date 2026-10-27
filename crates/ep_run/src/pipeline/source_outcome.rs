//! Separate own-file Source outcome and ancillary CLI scope validation.
//! No legacy profile/state is constructed; Native simulation/control is untouched.
use super::{RuntimeClass, RustRuntimeResult, SourceOrderGateSummary};
use crate::source_reported_results::{SourceReportedStream, SourceReportedStreams};
use ep_runtime::ResultStore;
use ep_runtime::heat_balance::source_reported_catalog::SourceReportedRunScope;
use serde_json::{Value, json};
use std::path::Path;

pub(super) struct SourceRuntimeResult {
    /// Optional output-only copy made while the final Source owners are live.
    pub geometry_snapshot: Option<Value>,
    pub reports: SourceReportedStreams,
    pub scope: SourceReportedRunScope,
    pub native_hour_scope: usize,
    pub requested_hours: Option<usize>,
    pub runtime_class: RuntimeClass,
    pub source_order_gate: SourceOrderGateSummary,
}
pub(super) enum RuntimeExecutionOutcome {
    Legacy(Box<RustRuntimeResult>),
    Source(Box<SourceRuntimeResult>),
}
impl RuntimeExecutionOutcome {
    pub fn legacy_results(&self) -> Option<&ResultStore> {
        match self {
            Self::Legacy(r) => Some(&r.results),
            Self::Source(_) => None,
        }
    }
    pub fn source_order_gate(&self) -> &SourceOrderGateSummary {
        match self {
            Self::Legacy(r) => &r.source_order_gate,
            Self::Source(r) => &r.source_order_gate,
        }
    }
    pub fn execution_description(&self) -> String {
        match self {
            Self::Legacy(r) => format!(
                "execute {} for {} hourly samples",
                r.runtime_class.id(),
                r.sample_count
            ),
            Self::Source(r) => format!(
                "execute actual Source ManageSimulation through620; full native environment scope {} hours; ESO {} and MTR {} reported numeric records",
                r.native_hour_scope, r.reports.eso.record_count, r.reports.mtr.record_count
            ),
        }
    }
    pub fn export_description(&self) -> &'static str {
        match self {
            Self::Legacy(_) => "write result-store.json, selected-outputs.csv, and meters.csv",
            Self::Source(_) => {
                "write complete source-reported-results.json, source-eso.csv, and source-mtr.csv with exact report identity, raw operands and native timestamp tuples"
            }
        }
    }
}

#[derive(Debug)]
pub(super) enum SourceScopeError {
    UnknownEnvironmentKind { ordinal: usize, kind: i32 },
    InvalidDays { ordinal: usize, days: i32 },
    HoursOverflow,
    NoSelectedEnvironments,
    RequestedHoursMismatch { requested: usize, native: usize },
}
impl std::fmt::Display for SourceScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownEnvironmentKind { ordinal, kind } => write!(
                f,
                "unsupported actual Source environment {ordinal} kind {kind} for CLI scope validation"
            ),
            Self::InvalidDays { ordinal, days } => write!(
                f,
                "invalid actual Source environment {ordinal} TotalDays {days} for CLI scope validation"
            ),
            Self::HoursOverflow => {
                f.write_str("actual Source environment hour scope overflows usize")
            }
            Self::NoSelectedEnvironments => {
                f.write_str("actual Source primary environment scope is empty")
            }
            Self::RequestedHoursMismatch { requested, native } => write!(
                f,
                "requested --hours {requested} differs from actual full Source environment scope {native}; truncation is unsupported"
            ),
        }
    }
}
impl std::error::Error for SourceScopeError {}

/// Ancillary CLI compatibility only. These are precisely Native386-398 primary
/// environment exclusions, not a second simulator or selection-state owner.
fn selected(scope: &SourceReportedRunScope, kind: i32) -> bool {
    if !scope.do_des_day_sim && kind != 3 {
        return false;
    }
    if !scope.do_weath_sim && kind == 3 {
        return false;
    }
    if kind == 4 {
        return false;
    }
    if kind == 5 {
        return false;
    }
    true
}
// Retain the existing nested input/trace guards and their error or diagnostic order.
#[allow(clippy::collapsible_if)]
pub(super) fn validate_requested_hours(
    scope: &SourceReportedRunScope,
    requested: Option<usize>,
) -> Result<usize, SourceScopeError> {
    let mut hours = 0usize;
    for row in &scope.environments {
        if !matches!(row.kind_of_environment, 1..=5) {
            return Err(SourceScopeError::UnknownEnvironmentKind {
                ordinal: row.ordinal,
                kind: row.kind_of_environment,
            });
        }
        if !selected(scope, row.kind_of_environment) {
            continue;
        }
        let days = usize::try_from(row.total_days)
            .ok()
            .filter(|days| *days > 0)
            .ok_or(SourceScopeError::InvalidDays {
                ordinal: row.ordinal,
                days: row.total_days,
            })?;
        // Native primary435 iterates actual NumOfDayInEnvrn=Environment.TotalDays;
        // Native485 always iterates24 hours per non-warmup day. No report sample
        // count, legacy TimeAxis, reconstructed date or hardcoded case is used.
        let row_hours = days
            .checked_mul(24)
            .ok_or(SourceScopeError::HoursOverflow)?;
        hours = hours
            .checked_add(row_hours)
            .ok_or(SourceScopeError::HoursOverflow)?;
    }
    if hours == 0 {
        return Err(SourceScopeError::NoSelectedEnvironments);
    }
    if let Some(requested) = requested {
        if requested != hours {
            return Err(SourceScopeError::RequestedHoursMismatch {
                requested,
                native: hours,
            });
        }
    }
    Ok(hours)
}
fn stream_summary(stream: &SourceReportedStream) -> Value {
    let mut summary = json!({"stream":stream.stream.id(),"owned_path":stream.path,"deleted_empty":stream.deleted_empty,
        "reported_numeric_records":stream.record_count,"series":stream.series.len(),
        "series_samples":stream.series.iter().map(|s| {
            let mut row = json!({"report_id":s.declaration.report_id,
            "key":s.declaration.key,"name":s.declaration.name,"units":s.declaration.units,
            "frequency":s.declaration.frequency.id(),"store_type":s.declaration.store_type.id(),
            "kind":s.declaration.kind.id(),"samples":s.samples.len()});
            if let Some(schedule) = &s.declaration.reporting_schedule {
                row["reporting_schedule"] = json!(schedule);
            }
            row
        }).collect::<Vec<_>>()});
    if !stream.deleted_empty_declarations.is_empty() {
        summary["deleted_empty_declarations"] =
            json!(stream.deleted_empty_declarations.iter().map(|d| json!({
            "report_id":d.report_id,"key":d.key,"name":d.name,"units":d.units,
            "frequency":d.frequency.id(),"store_type":d.store_type.id(),"kind":d.kind.id(),
            "reporting_schedule":d.reporting_schedule
        })).collect::<Vec<_>>());
    }
    summary
}
pub(super) fn source_summary(r: &SourceRuntimeResult) -> Value {
    json!({"schema":"rusted-energyplus.source-runtime.v1","runtime_class":r.runtime_class.id(),
        "execution":"actual Source ManageSimulation successful through620",
        "value_boundary":"own completed ESO/MTR reported decimal tokens; no internal history reconstruction",
        "timestamp_boundary":"native physical tuple; no civil year assigned where absent",
        "native_environment_hour_scope":r.native_hour_scope,"requested_hours":r.requested_hours,
        "native_options":{"DoDesDaySim":r.scope.do_des_day_sim,"DoWeathSim":r.scope.do_weath_sim,
            "DDOnly":r.scope.dd_only,"FullAnnualRun":r.scope.full_annual_run},
        "native_environment_scope":r.scope.environments.iter().map(|e|json!({"ordinal":e.ordinal,
            "title":e.title,"kind_of_environment":e.kind_of_environment,"total_days":e.total_days,
            "selected_by_primary_flags":selected(&r.scope,e.kind_of_environment)})).collect::<Vec<_>>(),
        "reported_streams":[stream_summary(&r.reports.eso),stream_summary(&r.reports.mtr)]})
}
#[derive(Debug)]
pub(super) enum SourceArtifactMapError {
    ExpectedArtifactObject,
}
impl std::fmt::Display for SourceArtifactMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExpectedArtifactObject => f.write_str("existing artifact map is not an object; completed Source artifact paths cannot be published"),
        }
    }
}
impl std::error::Error for SourceArtifactMapError {}
pub(super) fn artifact_map(
    output_dir: &Path,
    outcome: Option<&RuntimeExecutionOutcome>,
) -> Result<Value, SourceArtifactMapError> {
    let mut artifacts = super::artifact_map(output_dir);
    if matches!(outcome, Some(RuntimeExecutionOutcome::Source(_))) {
        let object = artifacts
            .as_object_mut()
            .ok_or(SourceArtifactMapError::ExpectedArtifactObject)?;
        object.remove("result_store_json");
        object.remove("selected_outputs_csv");
        object.remove("meters_csv");
        for (key, name) in [
            (
                "source_reported_results_json",
                "source-reported-results.json",
            ),
            ("source_eso_csv", "source-eso.csv"),
            ("source_mtr_csv", "source-mtr.csv"),
        ] {
            object.insert(
                key.to_owned(),
                Value::String(output_dir.join("results").join(name).display().to_string()),
            );
        }
    }
    Ok(artifacts)
}
