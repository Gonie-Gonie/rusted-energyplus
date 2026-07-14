use std::collections::BTreeSet;

use ep_conformance::{
    OutputFrequency, OutputRequest, SourceArtifact, TimestampContract, VariableClass,
};
use ep_model::{NormalizedName, ScheduleCompact, ScheduleCompactSegment, ScheduleId, TypedModel};

use super::{
    build_hourly_time_axis, precompute_schedule_value_series_for_time_axis, schedule_samples,
};

#[test]
fn schedule_samples_resolves_compact_trace_from_shared_name_registry()
-> Result<(), Box<dyn std::error::Error>> {
    let mut model = TypedModel::default();
    let schedule_id = ScheduleId(0);
    assert!(
        model
            .schedule_names
            .insert("Calendar Hourly", schedule_id)
            .is_none()
    );
    model.compact_schedules.push(ScheduleCompact {
        id: schedule_id,
        name: NormalizedName::new("Calendar Hourly"),
        schedule_type_limits: None,
        segments: (1..=24)
            .map(|hour| ScheduleCompactSegment {
                until_minute_of_day: hour * 60,
                value: f64::from(hour),
            })
            .collect(),
    });
    let time_axis = build_hourly_time_axis(&model).map_err(std::io::Error::other)?;
    let schedule_series = precompute_schedule_value_series_for_time_axis(&model, &time_axis);
    let output = OutputRequest {
        key: "CALENDAR HOURLY".to_string(),
        variable: "Schedule Value".to_string(),
        frequency: OutputFrequency::Hourly,
        class: VariableClass::Schedule,
        source: SourceArtifact::Eso,
        timestamp_contract: Some(TimestampContract::OrderedExactUnique),
        domain: None,
        level: None,
        abs_tol: None,
        rmse_tol: None,
        rel_tol: None,
    };

    let samples = schedule_samples(&output, &model, &time_axis, &schedule_series)
        .map_err(std::io::Error::other)?;
    let timestamps = samples
        .iter()
        .filter_map(|sample| sample.timestamp.as_deref())
        .collect::<BTreeSet<_>>();

    assert_eq!(samples.len(), 24);
    assert_eq!(timestamps.len(), 24);
    assert_eq!(samples.first().map(|sample| sample.value), Some(1.0));
    assert_eq!(samples.last().map(|sample| sample.value), Some(24.0));
    Ok(())
}
