//! Output-only copies of actual selected compile owners and production caches.

use crate::{PortingScope, RunConfig, RunError, RunExitCode, clk02_trace::scalar};
use ep_compiler::CompileReport;
use ep_model::{ScheduleCompactDayProfile, TypedModel};
use ep_raw_model::{RawModel, RawValue};
use ep_runtime::schedules::{
    CachedScheduleSeries, CompiledScheduleInterval, ScheduleCacheProfile, ScheduleSampleStorage,
    ScheduleSeriesKind,
    production_trace::{ScheduleCacheOrigin, ScheduleProductionTrace},
};
use serde_json::{Value, json};
use std::io::{BufWriter, Write};

// Preserve actual lexical number storage; the typed owners below supply f64 DTOs.
fn raw_value(value: &RawValue) -> Value {
    match value {
        RawValue::Null => json!({"kind":"null"}),
        RawValue::Bool(value) => json!({"kind":"bool","value":value}),
        RawValue::String(value) => json!({"kind":"string","value":value}),
        RawValue::Number(text) => json!({"kind":"number_lexeme","value":text}),
        RawValue::Array(values) => json!({"kind":"array",
            "values":values.iter().map(raw_value).collect::<Vec<_>>()}),
        RawValue::Object(values) => json!({"kind":"object",
            "fields":values.iter().map(|(name,value)|json!({
                "name":name.0,"value":raw_value(value)})).collect::<Vec<_>>()}),
    }
}

fn raw_families(raw: &RawModel) -> Result<Value, String> {
    let mut families = Vec::new();
    for family in [
        "ScheduleTypeLimits",
        "Schedule:Constant",
        "Schedule:Compact",
    ] {
        let instances = raw
            .ordered_instances(family)
            .map_err(|error| error.to_string())?;
        families.push(json!({"object_type":family,
            "has_IDF_declaration_order_overlay":raw.has_idf_declaration_order(family),
            "effective_instances":instances.iter().enumerate().map(|(index,(name,object))|json!({
                "effective_instance_index":index,"name":name.0,
                "fields":object.fields.iter().map(|(field,value)|json!({
                    "name":field.0,"value":raw_value(value)})).collect::<Vec<_>>()
            })).collect::<Vec<_>>() }));
    }
    Ok(json!({"version":raw.version,"selected_families":families}))
}

fn typed_profile(profile: &ScheduleCompactDayProfile) -> Value {
    json!({"day_types":profile.day_types.iter().map(|day|json!({
            "name":format!("{day:?}"),"rust_enum_discriminant":*day as u32})).collect::<Vec<_>>(),
        "interpolation":format!("{:?}",profile.interpolation),
        "segments":profile.segments.iter().map(|segment|json!({
            "until_minute_of_day":segment.until_minute_of_day,
            "value":scalar(segment.value)})).collect::<Vec<_>>()})
}

fn typed(model: &TypedModel) -> Value {
    let registration = model
        .schedule_names
        .names()
        .iter()
        .enumerate()
        .map(|(position, name)| {
            let id = model.schedule_names.resolve(&name.0);
            json!({"registration_position":position,"normalized_name":name.0,
            "resolved_actual_ScheduleId":id.map(|value|value.0),
            "constant_vector_indices":model.schedules.iter().enumerate()
                .filter(|(_,value)|Some(value.id)==id).map(|(index,_)|index).collect::<Vec<_>>(),
            "compact_vector_indices":model.compact_schedules.iter().enumerate()
                .filter(|(_,value)|Some(value.id)==id).map(|(index,_)|index).collect::<Vec<_>>()})
        })
        .collect::<Vec<_>>();
    json!({"timestep_number_of_timesteps_per_hour":model.timestep.number_of_timesteps_per_hour,
        "schedule_registration":registration,
        "schedule_type_limit_registration":model.schedule_type_limit_names.names().iter().enumerate()
            .map(|(position,name)|json!({"registration_position":position,
                "normalized_name":name.0,"resolved_actual_ScheduleTypeLimitId":
                model.schedule_type_limit_names.resolve(&name.0).map(|id|id.0)})).collect::<Vec<_>>(),
        "schedule_type_limits":model.schedule_type_limits.iter().enumerate().map(|(index,value)|json!({
            "vector_index":index,"id":value.id.0,"name":value.name.0,
            "lower_limit":value.lower_limit.map(scalar),"upper_limit":value.upper_limit.map(scalar),
            "numeric_type":value.numeric_type.map(|kind|format!("{kind:?}")),
            "unit_type":format!("{:?}",value.unit_type),"unit_type_owner_available":true
        })).collect::<Vec<_>>(),
        "constant_schedules":model.schedules.iter().enumerate().map(|(index,value)|json!({
            "vector_index":index,"id":value.id.0,"name":value.name.0,
            "schedule_type_limits":value.schedule_type_limits.map(|id|id.0),
            "hourly_value":scalar(value.hourly_value)})).collect::<Vec<_>>(),
        "compact_schedules":model.compact_schedules.iter().enumerate().map(|(index,value)|json!({
            "vector_index":index,"id":value.id.0,"name":value.name.0,
            "schedule_type_limits":value.schedule_type_limits.map(|id|id.0),
            "periods":value.periods.iter().map(|period|json!({
                "through_schedule_day_of_year":period.through_schedule_day_of_year,
                "day_profiles":period.day_profiles.iter().map(typed_profile).collect::<Vec<_>>()
            })).collect::<Vec<_>>() })).collect::<Vec<_>>(),
        "Native_builtin_storage_owner_available":false,
        "Native_generated_week_day_ID_graph_owner_available":false,
        "Native_ScheduleInputProcessed_owner_available":false,
        "Native_min_max_cache_owner_available":false})
}

/// Called once just after the real compiler returns, with its actual report/model.
pub(crate) fn write_compiled(
    config: &RunConfig,
    scope: PortingScope,
    raw: &RawModel,
    report: &CompileReport,
    model: Option<&TypedModel>,
) -> Result<(), RunError> {
    let artifact = json!({"schema":"sch01-production-compiled-schedules.v1",
        "scope":scope.id(),"input_path":config.input_path,"weather_path":config.weather_path,
        "actual_compile_result_count":1,"raw":raw_families(raw).map_err(output_error)?,
        "typed_model_available":model.is_some(),"typed":model.map(typed),
        "compiler_report":{"completed_stages":report.completed_stages.iter()
            .map(|stage|format!("{stage:?}")).collect::<Vec<_>>(),
            "diagnostics":report.diagnostics.iter().map(|value|json!({
                "severity":value.severity.to_string(),"code":value.code,
                "object_type":value.object_type,"object_name":value.object_name,
                "field":value.field,"message":value.message})).collect::<Vec<_>>(),
            "defaults_applied":report.defaults_applied.iter().map(|value|json!({
                "object_type":value.object_type,"object_name":value.object_name,
                "field":value.field,"value":value.value})).collect::<Vec<_>>()},
        "partial_failed_model_reconstructed":false,"schedule_IDs_or_order_normalized_away":false,
        "claim_boundary":"actual selected owners only; no annual production coverage, SCH02 arithmetic or SCH03 lookup certification"});
    write(config, "sch01_compiled_schedules.json", &artifact)
}

fn intervals(values: &[CompiledScheduleInterval]) -> Vec<Value> {
    values
        .iter()
        .map(|value| {
            json!({"start_minute_of_day":value.start_minute_of_day,
        "end_minute_of_day":value.end_minute_of_day,"value":scalar(value.value)})
        })
        .collect()
}

fn kind(kind: &ScheduleSeriesKind) -> Value {
    match kind {
        ScheduleSeriesKind::ConstantScalar { value } => {
            json!({"kind":"ConstantScalar","value":scalar(*value)})
        }
        ScheduleSeriesKind::ExternalInterfaceInitialValue { value } => {
            json!({"kind":"ExternalInterfaceInitialValue","value":scalar(*value)})
        }
        ScheduleSeriesKind::CompactIntervals { intervals: values } => {
            json!({"kind":"CompactIntervals","intervals":intervals(values)})
        }
        ScheduleSeriesKind::CompactCalendarProfiles { periods } => {
            json!({"kind":"CompactCalendarProfiles",
            "periods":periods.iter().map(|period|json!({
                "through_schedule_day_of_year":period.through_schedule_day_of_year,
                "day_profiles":period.day_profiles.iter().map(|profile|json!({
                    "day_types":profile.day_types.iter().map(|day|format!("{day:?}")).collect::<Vec<_>>(),
                    "interpolation":format!("{:?}",profile.interpolation),
                    "intervals":intervals(&profile.intervals),
                    "minutes_per_timestep":profile.minutes_per_timestep,
                    "zone_timestep_values":profile.zone_timestep_values.iter().copied().map(scalar).collect::<Vec<_>>()
                })).collect::<Vec<_>>() })).collect::<Vec<_>>() })
        }
        // Outside the selected three-family boundary; no alternative computation is run.
        other => {
            json!({"kind":"outside_selected_schedule_families","actual_variant":format!("{other:?}"),
            "selected_representation_available":false})
        }
    }
}

fn entry(index: usize, value: &CachedScheduleSeries) -> Value {
    let storage = match &value.samples {
        ScheduleSampleStorage::Scalar { value, len } => json!({"kind":"Scalar",
            "value":scalar(*value),"logical_len":len}),
        ScheduleSampleStorage::Dense(values) => json!({"kind":"Dense",
            "values":values.iter().copied().map(scalar).collect::<Vec<_>>()}),
    };
    json!({"cache_entry_index":index,"schedule_id":value.schedule_id.0,
        "schedule_name":value.schedule_name,"logical_sample_count":value.len(),
        "storage":storage,"representation":kind(&value.kind)})
}

fn profile(value: ScheduleCacheProfile) -> Value {
    json!({"scalar_series_count":value.scalar_series_count,"dense_series_count":value.dense_series_count,
        "logical_sample_count":value.logical_sample_count,
        "allocated_dense_sample_count":value.allocated_dense_sample_count,
        "index_kind":format!("{:?}",value.index_kind),"ambiguous_id_count":value.ambiguous_id_count})
}

/// Serializes only retained real owners; no producer, precompile or lookup is called.
pub(crate) fn write_runtime(
    config: &RunConfig,
    trace: &ScheduleProductionTrace,
) -> Result<(), RunError> {
    let observations = trace.observations.iter().enumerate().map(|(index,observation)| {
        let role = match observation.origin {
            ScheduleCacheOrigin::PipelineHourlyPrepared => "prepared_only_not_a_runtime_cache_argument",
            ScheduleCacheOrigin::PipelineEnvironmentPrepared => "prepared_environment_cache",
            ScheduleCacheOrigin::CoupledProductionArgument => "received_actual_coupled_runtime_argument",
            ScheduleCacheOrigin::HeatBalanceReferencedInitialization
            | ScheduleCacheOrigin::CoupledReferencedInitialization => "referenced_only_cache_just_constructed_for_initialization",
        };
        json!({"observation_index":index,"origin":observation.origin.id(),"role":role,
            "sample_count":observation.cache.sample_count(),"entry_count":observation.cache.len(),
            "profile":profile(observation.cache.profile()),
            "entries":observation.cache.iter().enumerate().map(|(index,value)|entry(index,value)).collect::<Vec<_>>()})
    }).collect::<Vec<_>>();
    let artifact = json!({"schema":"sch01-production-schedule-cache-trace.v1",
        "scope":trace.scope,"input_path":config.input_path,"weather_path":config.weather_path,
        "total_observation_count":trace.observations.len(),"retained_observation_count":observations.len(),
        "omitted_observation_count":0,"observation_cap":null,"observations":observations,
        "cache_producer_or_precompile_or_schedule_lookup_called_by_observer":false,
        "identities_inferred_from_latest_context":false,"per_lookup_observations_available":false,
        "claim_boundary":"actual initialization and prepared/received cache owners only; no annual production coverage, SCH02 arithmetic or SCH03 lookup certification"});
    write(config, "sch01_schedule_cache_trace.json", &artifact)
}

fn output_error(message: String) -> RunError {
    RunError {
        exit_code: RunExitCode::OutputExport,
        message,
    }
}

fn write(config: &RunConfig, filename: &str, artifact: &Value) -> Result<(), RunError> {
    let path = config.output_dir.join(filename);
    let result = || -> Result<(), String> {
        let file = std::fs::File::create(&path)
            .map_err(|error| format!("failed to create {}: {error}", path.display()))?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, artifact)
            .map_err(|error| format!("failed to serialize {}: {error}", path.display()))?;
        writer
            .write_all(b"\n")
            .and_then(|()| writer.flush())
            .map_err(|error| format!("failed to write {}: {error}", path.display()))
    };
    result().map_err(output_error)
}
