//! Direct copies of available public fields; no Native graph reconstruction.
use super::Result;
use ep_compiler::CompileReport;
use ep_model::{ScheduleCompactDayProfile, TypedModel};
use ep_raw_model::RawModel;
use serde_json::{Value, json};

pub(super) fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value == f64::INFINITY {
        "positive_infinity"
    } else if value == f64::NEG_INFINITY {
        "negative_infinity"
    } else if value == 0.0 && value.is_sign_negative() {
        "negative_zero"
    } else if value == 0.0 {
        "positive_zero"
    } else {
        "finite"
    };
    json!({"value":if value.is_finite(){Some(value)}else{None},
        "value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}

pub(super) fn raw(model: &RawModel) -> Result<Value> {
    let mut families = Vec::new();
    for object_type in [
        "ScheduleTypeLimits",
        "Schedule:Constant",
        "Schedule:Compact",
    ] {
        families.push(json!({"object_type":object_type,
            "has_IDF_declaration_order_overlay":model.has_idf_declaration_order(object_type),
            "effective_instance_names":model.ordered_instances(object_type)?.iter()
                .map(|(name, _)|name.0.as_str()).collect::<Vec<_>>() }));
    }
    Ok(
        json!({"version":model.version,"raw_object_count":model.object_count(),
        "raw_object_type_counts":model.object_type_counts(),"selected_family_order":families}),
    )
}

pub(super) fn report(report: &CompileReport) -> Value {
    json!({"completed_stages":report.completed_stages.iter().map(|value|format!("{value:?}")).collect::<Vec<_>>(),
        "raw_object_count":report.raw_object_count,"typed_object_count":report.typed_object_count,
        "diagnostics":report.diagnostics.iter().map(|value|json!({
            "severity":value.severity.to_string(),"code":value.code,"object_type":value.object_type,
            "object_name":value.object_name,"field":value.field,"message":value.message
        })).collect::<Vec<_>>(),
        "defaults_applied":report.defaults_applied.iter().map(|value|json!({
            "object_type":value.object_type,"object_name":value.object_name,
            "field":value.field,"value":value.value
        })).collect::<Vec<_>>(),
        "coverage":report.coverage.iter().map(|value|json!({"object_type":value.object_type,
            "object_count":value.object_count,"status":value.status.to_string()})).collect::<Vec<_>>(),
        "compiler_stage_labels_are_reported_metadata_not_Native_IDD_validation_proof":true})
}

fn profile(profile: &ScheduleCompactDayProfile) -> Value {
    json!({"day_types":profile.day_types.iter().map(|value|json!({"name":format!("{value:?}"),
        "rust_enum_discriminant":*value as u32})).collect::<Vec<_>>(),
        "interpolation":format!("{:?}",profile.interpolation),
        "segments":profile.segments.iter().map(|segment|json!({
            "until_minute_of_day":segment.until_minute_of_day,"value":scalar(segment.value)
        })).collect::<Vec<_>>()})
}

pub(super) fn model(model: &TypedModel) -> Value {
    let registration = model
        .schedule_names
        .names()
        .iter()
        .enumerate()
        .map(|(position, name)| {
            let actual_id = model.schedule_names.resolve(&name.0);
            let constant_owners = model
                .schedules
                .iter()
                .enumerate()
                .filter(|(_, schedule)| Some(schedule.id) == actual_id)
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            let compact_owners = model
                .compact_schedules
                .iter()
                .enumerate()
                .filter(|(_, schedule)| Some(schedule.id) == actual_id)
                .map(|(index, _)| index)
                .collect::<Vec<_>>();
            json!({"registration_position":position,"normalized_name":name.0,
            "resolved_actual_ScheduleId":actual_id.map(|id|id.0),
            "constant_vector_indices":constant_owners,"compact_vector_indices":compact_owners})
        })
        .collect::<Vec<_>>();
    json!({"typed_object_count":model.object_count(),
        "timestep_number_of_timesteps_per_hour":model.timestep.number_of_timesteps_per_hour,
        "schedule_registration":registration,
        "schedule_type_limit_registration":model.schedule_type_limit_names.names().iter()
            .map(|name|json!({"normalized_name":name.0,
                "resolved_actual_ScheduleTypeLimitId":model.schedule_type_limit_names.resolve(&name.0).map(|id|id.0)}))
            .collect::<Vec<_>>(),
        "schedule_type_limits":model.schedule_type_limits.iter().enumerate().map(|(index, value)|json!({
            "vector_index":index,"id":value.id.0,"name":value.name.0,
            "lower_limit":value.lower_limit.map(scalar),"upper_limit":value.upper_limit.map(scalar),
            "numeric_type":value.numeric_type.map(|kind|format!("{kind:?}")),
            "unit_type_owner_available":false
        })).collect::<Vec<_>>(),
        "constant_schedules":model.schedules.iter().enumerate().map(|(index, value)|json!({
            "vector_index":index,"id":value.id.0,"name":value.name.0,
            "schedule_type_limits":value.schedule_type_limits.map(|id|id.0),
            "hourly_value":scalar(value.hourly_value)
        })).collect::<Vec<_>>(),
        "compact_schedules":model.compact_schedules.iter().enumerate().map(|(index, value)|json!({
            "vector_index":index,"id":value.id.0,"name":value.name.0,
            "schedule_type_limits":value.schedule_type_limits.map(|id|id.0),
            "periods":value.periods.iter().map(|period|json!({
                "through_schedule_day_of_year":period.through_schedule_day_of_year,
                "day_profiles":period.day_profiles.iter().map(profile).collect::<Vec<_>>()
            })).collect::<Vec<_>>()
        })).collect::<Vec<_>>(),
        "Native_builtin_storage_owner_available":false,
        "Native_generated_week_day_ID_graph_owner_available":false,
        "Native_ScheduleInputProcessed_owner_available":false,
        "Native_min_max_cache_owner_available":false,
        "partial_failed_compiler_model_observed":false,
        "actual_IDs_or_registration_order_normalized_away":false})
}
