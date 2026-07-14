use ep_runtime::TimeAxis;

use crate::json_string;

pub(super) fn append_special_days_markdown(report: &mut String, time_axis: &TimeAxis) {
    report.push_str(&format!(
        "input_file_special_days_declared: {}\n",
        time_axis.special_days.input_file_special_days_declared
    ));
    report.push_str(&format!(
        "special_day_weekend_rule: {}\n",
        time_axis.special_days.apply_weekend_rule
    ));
    report.push_str(&format!(
        "special_days_resolved: {}\n",
        time_axis.special_days.resolved_days.len()
    ));
    report.push_str(&format!(
        "special_day_hourly_samples: {}\n",
        time_axis
            .points
            .iter()
            .filter(|point| point.special_day_type.is_some())
            .count()
    ));
    for special_day in &time_axis.special_days.resolved_days {
        report.push_str(&format!(
            "special_day_resolved: {} {}/{} duration={} day_type={} weekend_shift_days={}\n",
            special_day.name,
            special_day.start.month,
            special_day.start.day_of_month,
            special_day.duration_days,
            special_day.day_type.label(),
            special_day.weekend_shift_days
        ));
    }
}

pub(super) fn special_days_json(time_axis: &TimeAxis) -> String {
    let resolved = time_axis
        .special_days
        .resolved_days
        .iter()
        .map(|special_day| {
            format!(
                "{{\"name\": {}, \"start_month\": {}, \"start_day\": {}, \"start_day_of_year\": {}, \"duration_days\": {}, \"day_type\": {}, \"day_type_index\": {}, \"weekend_shift_days\": {}}}",
                json_string(&special_day.name),
                special_day.start.month,
                special_day.start.day_of_month,
                special_day.start.day_of_year,
                special_day.duration_days,
                json_string(special_day.day_type.label()),
                special_day.day_type.energyplus_index(),
                special_day.weekend_shift_days,
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "{{\"input_file_declared\": {}, \"apply_weekend_rule\": {}, \"resolved_count\": {}, \"hourly_samples\": {}, \"resolved\": [{}]}}",
        time_axis.special_days.input_file_special_days_declared,
        time_axis.special_days.apply_weekend_rule,
        time_axis.special_days.resolved_days.len(),
        time_axis
            .points
            .iter()
            .filter(|point| point.special_day_type.is_some())
            .count(),
        resolved,
    )
}
