//! Runtime-owned normalized timestamp labels used at comparison boundaries.

use crate::time_axis::{EnvironmentTimeAxis, EnvironmentTimePoint, TimeAxis, TimePoint};

/// Formats the normalized hourly timestamp label used by `ep_compare` series.
///
/// This is a projection of runtime-owned output timestamp fields, not a raw
/// ESO/MTR row serializer and not a `WriteTimeStampFormatData` conformance
/// claim. Callers must not reconstruct calendar or day-type state.
#[must_use]
pub fn normalized_hourly_timestamp_label(time_axis: &TimeAxis, point: &TimePoint) -> String {
    format!(
        "env={};day={};month={};date={};dst={};hour={};start={:.2};end={:.2};day_type={}",
        time_axis.run_period_name.to_ascii_uppercase(),
        point.day_of_sim,
        point.month,
        point.day_of_month,
        u8::from(point.dst),
        point.hour,
        point.start_minute,
        point.end_minute,
        point.day_type.label()
    )
}

/// Formats a normalized timestamp label for one environment zone timestep.
///
/// This projection uses the already-resolved environment point, including its
/// subhourly minute bounds. It is not a raw ESO/MTR serializer or a broad
/// `WriteTimeStampFormatData` conformance claim.
#[must_use]
pub fn normalized_environment_timestep_timestamp_label(
    time_axis: &EnvironmentTimeAxis,
    point: &EnvironmentTimePoint,
) -> String {
    format!(
        "env={};day={};month={};date={};dst={};hour={};start={:.2};end={:.2};day_type={}",
        time_axis.environment_name.to_ascii_uppercase(),
        point.day_of_sim,
        point.month,
        point.day_of_month,
        u8::from(point.dst),
        point.hour,
        point.start_minute,
        point.end_minute,
        point.day_type.label()
    )
}
