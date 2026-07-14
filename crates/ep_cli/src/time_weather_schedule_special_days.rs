use ep_runtime::TimeAxis;

use crate::json_string;

pub(super) fn append_special_days_markdown(report: &mut String, time_axis: &TimeAxis) {
    report.push_str(&format!(
        "weather_file_holidays_declared: {}\n",
        time_axis.special_days.weather_file_holidays_declared
    ));
    report.push_str(&format!(
        "run_period_uses_weather_file_holidays: {}\n",
        time_axis.special_days.run_period_uses_weather_file_holidays
    ));
    report.push_str(&format!(
        "weather_file_holidays_resolved: {}\n",
        time_axis.special_days.weather_file_holidays_resolved
    ));
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
            "special_day_resolved: {} {}/{} duration={} day_type={} weekend_shift_days={} source={}\n",
            special_day.name,
            special_day.start.month,
            special_day.start.day_of_month,
            special_day.duration_days,
            special_day.day_type.label(),
            special_day.weekend_shift_days,
            special_day.source.label()
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
                "{{\"name\": {}, \"source\": {}, \"start_month\": {}, \"start_day\": {}, \"start_day_of_year\": {}, \"duration_days\": {}, \"day_type\": {}, \"day_type_index\": {}, \"weekend_shift_days\": {}}}",
                json_string(&special_day.name),
                json_string(special_day.source.label()),
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
        "{{\"weather_file_declared\": {}, \"run_period_uses_weather_file\": {}, \"weather_file_resolved\": {}, \"input_file_declared\": {}, \"apply_weekend_rule\": {}, \"resolved_count\": {}, \"hourly_samples\": {}, \"resolved\": [{}]}}",
        time_axis.special_days.weather_file_holidays_declared,
        time_axis.special_days.run_period_uses_weather_file_holidays,
        time_axis.special_days.weather_file_holidays_resolved,
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
