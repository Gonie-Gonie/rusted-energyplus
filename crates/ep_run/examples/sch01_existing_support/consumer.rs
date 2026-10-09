//! Existing runtime consumer calls; their arithmetic is not certified by SCH-01.
use super::{Result, fields};
use ep_model::TypedModel;
use ep_runtime::schedules::{
    precompile_compact_schedule_periods, precompute_constant_schedule_cache,
};
use serde_json::{Value, json};

pub(super) fn handoff(model: &TypedModel) -> Result<Value> {
    let steps = model.timestep.number_of_timesteps_per_hour;
    let samples = usize::try_from(steps)?
        .checked_mul(24)
        .ok_or("daily sample count overflow")?;
    let cache = precompute_constant_schedule_cache(model, samples);
    let cache_profile = cache.profile();
    let constant_rows = cache
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            json!({
                "cache_entry_index":index,"schedule_id":entry.schedule_id.0,
                "schedule_name":entry.schedule_name,"logical_sample_count":entry.len(),
                "values":entry.values().map(fields::scalar).collect::<Vec<_>>()
            })
        })
        .collect::<Vec<_>>();
    let mut compact_rows = Vec::new();
    for (index, schedule) in model.compact_schedules.iter().enumerate() {
        let periods = precompile_compact_schedule_periods(schedule, steps);
        compact_rows.push(json!({"model_vector_index":index,"schedule_id":schedule.id.0,
            "schedule_name":schedule.name.0,"actual_timesteps_per_hour_argument":steps,
            "periods":periods.iter().map(|period|json!({
                "through_schedule_day_of_year":period.through_schedule_day_of_year,
                "day_profiles":period.day_profiles.iter().map(|profile|json!({
                    "day_types":profile.day_types.iter().map(|value|format!("{value:?}")).collect::<Vec<_>>(),
                    "interpolation":format!("{:?}",profile.interpolation),
                    "intervals":profile.intervals.iter().map(|interval|json!({
                        "start_minute_of_day":interval.start_minute_of_day,
                        "end_minute_of_day":interval.end_minute_of_day,"value":fields::scalar(interval.value)
                    })).collect::<Vec<_>>(),
                    "minutes_per_timestep":profile.minutes_per_timestep,
                    "zone_timestep_values":profile.zone_timestep_values.iter().copied().map(fields::scalar).collect::<Vec<_>>()
                })).collect::<Vec<_>>()
            })).collect::<Vec<_>>() }));
    }
    Ok(
        json!({"available":true,"actual_model_timesteps_per_hour":steps,
        "constant_cache":constant_rows,"compact_compiled_periods":compact_rows,
        "constant_cache_profile":{"scalar_series_count":cache_profile.scalar_series_count,
            "dense_series_count":cache_profile.dense_series_count,
            "logical_sample_count":cache_profile.logical_sample_count,
            "allocated_dense_sample_count":cache_profile.allocated_dense_sample_count,
            "index_kind":format!("{:?}",cache_profile.index_kind),
            "ambiguous_id_count":cache_profile.ambiguous_id_count},
        "actual_public_API_invocations":{
            "ep_runtime::schedules::precompute_constant_schedule_cache":1,
            "ep_runtime::schedules::precompile_compact_schedule_periods":model.compact_schedules.len()},
        "normalized_model_borrowed_without_mutation":true,
        "source_expected_or_Native_values_used":false,
        "SCH02_arithmetic_certified":false,"SCH03_lookup_certified":false}),
    )
}
