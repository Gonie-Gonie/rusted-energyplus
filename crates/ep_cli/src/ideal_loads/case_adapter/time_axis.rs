//! Time-axis adaptation for IdealLoads conformance traces.

use ep_model::TypedModel;
use ep_runtime::time_axis_timestep_profile;

const ESO_TIMESTAMP_RESOLUTION_SECONDS: f64 = 0.6;

/// Nominal timestep metadata derived from the shared runtime time axis.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads) struct IdealLoadsTimestepContext {
    pub(in crate::ideal_loads) nominal_system_timestep_substeps: f64,
    pub(in crate::ideal_loads) nominal_system_timestep_seconds: f64,
    pub(in crate::ideal_loads) zone_timestep_seconds: f64,
    pub(in crate::ideal_loads) source: &'static str,
    pub(in crate::ideal_loads) adaptive_system_timestep_claim: bool,
}

/// Builds IdealLoads nominal timestep metadata from `ep_runtime::TimeAxis`.
pub(in crate::ideal_loads) fn ideal_loads_timestep_context(
    model: &TypedModel,
) -> Result<IdealLoadsTimestepContext, String> {
    let timestep_profile = time_axis_timestep_profile(model);
    let zone_timestep_seconds = timestep_profile.zone_timestep.timestep_seconds;
    let nominal_system_timestep_seconds = timestep_profile.system_timestep.nominal_timestep_seconds;
    let nominal_system_timestep_substeps = zone_timestep_seconds / nominal_system_timestep_seconds;
    if !zone_timestep_seconds.is_finite()
        || zone_timestep_seconds <= 0.0
        || !nominal_system_timestep_seconds.is_finite()
        || nominal_system_timestep_seconds <= 0.0
        || !nominal_system_timestep_substeps.is_finite()
        || nominal_system_timestep_substeps <= 0.0
    {
        return Err(
            "IdealLoads runtime time axis has invalid nominal timestep metadata".to_string(),
        );
    }

    Ok(IdealLoadsTimestepContext {
        nominal_system_timestep_substeps,
        nominal_system_timestep_seconds,
        zone_timestep_seconds,
        source: "ep_runtime::TimeAxis",
        adaptive_system_timestep_claim: false,
    })
}

/// Resolves one trace sample duration in seconds, with a nominal fallback.
///
/// ESO start/end minutes are printed to two decimal places. When that rounded
/// duration is within the timestamp resolution of an integer subdivision of
/// the nominal zone timestep, the exact subdivision is restored. Other valid
/// durations at or below the nominal timestep remain exact (for example,
/// 0-10 minutes stays 600 seconds). Longer durations use the nominal fallback.
#[must_use]
pub(in crate::ideal_loads) fn ideal_loads_sample_timestep_seconds(
    timestamp: Option<&str>,
    nominal_timestep_seconds: f64,
) -> f64 {
    let Some(timestamp) = timestamp else {
        return nominal_timestep_seconds;
    };
    let Some(start_minute) = timestamp_numeric_field(timestamp, "start") else {
        return nominal_timestep_seconds;
    };
    let Some(end_minute) = timestamp_numeric_field(timestamp, "end") else {
        return nominal_timestep_seconds;
    };
    let duration_seconds = (end_minute - start_minute) * 60.0;
    if duration_seconds.is_finite() && duration_seconds > 0.0 {
        let normalized =
            normalize_eso_timestamp_duration_seconds(duration_seconds, nominal_timestep_seconds);
        if nominal_timestep_seconds.is_finite()
            && nominal_timestep_seconds > 0.0
            && normalized > nominal_timestep_seconds
        {
            nominal_timestep_seconds
        } else {
            normalized
        }
    } else {
        nominal_timestep_seconds
    }
}

/// Resolves one trace sample duration in hours, with a nominal fallback.
#[must_use]
pub(in crate::ideal_loads) fn ideal_loads_sample_timestep_hours(
    timestamp: Option<&str>,
    nominal_timestep_hours: f64,
) -> f64 {
    ideal_loads_sample_timestep_seconds(timestamp, nominal_timestep_hours * 3600.0) / 3600.0
}

fn timestamp_numeric_field(timestamp: &str, field_name: &str) -> Option<f64> {
    timestamp.split(';').find_map(|part| {
        let (key, value) = part.split_once('=')?;
        key.trim()
            .eq_ignore_ascii_case(field_name)
            .then(|| value.trim().parse::<f64>().ok())
            .flatten()
    })
}

fn normalize_eso_timestamp_duration_seconds(
    duration_seconds: f64,
    nominal_timestep_seconds: f64,
) -> f64 {
    if !nominal_timestep_seconds.is_finite() || nominal_timestep_seconds <= 0.0 {
        return duration_seconds;
    }
    let integer_substeps = (nominal_timestep_seconds / duration_seconds).round();
    if !integer_substeps.is_finite() || integer_substeps < 1.0 {
        return duration_seconds;
    }
    let exact_substep_seconds = nominal_timestep_seconds / integer_substeps;
    if (duration_seconds - exact_substep_seconds).abs() <= ESO_TIMESTAMP_RESOLUTION_SECONDS {
        exact_substep_seconds
    } else {
        duration_seconds
    }
}

#[cfg(test)]
#[path = "time_axis_tests.rs"]
mod time_axis_tests;
