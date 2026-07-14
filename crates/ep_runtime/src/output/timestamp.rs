//! Runtime-owned normalized timestamp labels used at comparison boundaries.

use crate::time_axis::{TimeAxis, TimePoint};

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
