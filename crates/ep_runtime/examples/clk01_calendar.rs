//! Run the existing calendar APIs with input-only CLK-01 unit cases.
//!
//! Reads a JSON path or stdin. Prepared rows are not production invocations.

use ep_model::{
    DayOfWeek, FirstHourInterpolationStartingValues, NormalizedName, RunPeriod, RunPeriodId,
};
use ep_runtime::time_axis::{
    build_environment_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps,
    build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps,
    clock_trace::CalendarFrame, resolve_run_period_calendar, resolve_weather_environment_calendar,
};
use ep_runtime::weather::EpwCalendarMetadata;
use serde_json::{Map, Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let text = if let Some(path) = args.next() {
        std::fs::read_to_string(path)?
    } else {
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text)?;
        text
    };
    if args.next().is_some() {
        return Err("usage: clk01_calendar [INPUT.json] (results on stdout)".into());
    }
    println!(
        "{}",
        serde_json::to_string(&execute(&serde_json::from_str(&text)?)?)?
    );
    Ok(())
}

fn object<'a>(
    value: &'a Value,
    required: &[&str],
    optional: &[&str],
) -> Result<&'a Map<String, Value>, String> {
    let object = value.as_object().ok_or("expected object")?;
    if required.iter().any(|name| !object.contains_key(*name))
        || object
            .keys()
            .any(|name| !required.contains(&name.as_str()) && !optional.contains(&name.as_str()))
    {
        return Err(format!(
            "expected required fields {required:?}, optional {optional:?}"
        ));
    }
    Ok(object)
}

fn integer(value: &Value) -> Result<u32, String> {
    value
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .ok_or("expected u32".into())
}

fn optional_integer(value: &Value) -> Result<Option<u32>, String> {
    if value.is_null() {
        Ok(None)
    } else {
        integer(value).map(Some)
    }
}

fn boolean(value: &Value) -> Result<bool, String> {
    value.as_bool().ok_or("expected boolean".into())
}

fn weekday(value: &Value) -> Result<Option<DayOfWeek>, String> {
    if value.is_null() {
        return Ok(None);
    }
    match integer(value)? {
        1 => Ok(Some(DayOfWeek::Sunday)),
        2 => Ok(Some(DayOfWeek::Monday)),
        3 => Ok(Some(DayOfWeek::Tuesday)),
        4 => Ok(Some(DayOfWeek::Wednesday)),
        5 => Ok(Some(DayOfWeek::Thursday)),
        6 => Ok(Some(DayOfWeek::Friday)),
        7 => Ok(Some(DayOfWeek::Saturday)),
        _ => Err("weekday must be null or Sunday=1..Saturday=7".into()),
    }
}

fn run_period(value: &Value, case_id: &str) -> Result<RunPeriod, String> {
    let v = object(
        value,
        &[
            "begin_month",
            "begin_day_of_month",
            "begin_year",
            "end_month",
            "end_day_of_month",
            "end_year",
            "start_weekday",
            "first_hour_interpolation_starting_values",
            "use_weather_holidays",
            "use_weather_dst",
            "apply_weekend_holiday_rule",
            "treat_weather_as_actual",
        ],
        &["name"],
    )?;
    let name = v.get("name").map_or(Ok(case_id), |name| {
        name.as_str().ok_or("name must be string")
    })?;
    let interpolation = match v["first_hour_interpolation_starting_values"].as_str() {
        Some("Hour1") => FirstHourInterpolationStartingValues::Hour1,
        Some("Hour24") => FirstHourInterpolationStartingValues::Hour24,
        _ => return Err("first-hour policy must be Hour1 or Hour24".into()),
    };
    Ok(RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new(name),
        begin_month: integer(&v["begin_month"])?,
        begin_day_of_month: integer(&v["begin_day_of_month"])?,
        begin_year: optional_integer(&v["begin_year"])?,
        end_month: integer(&v["end_month"])?,
        end_day_of_month: integer(&v["end_day_of_month"])?,
        end_year: optional_integer(&v["end_year"])?,
        day_of_week_for_start_day: weekday(&v["start_weekday"])?,
        first_hour_interpolation_starting_values: interpolation,
        use_weather_file_holidays_and_special_days: boolean(&v["use_weather_holidays"])?,
        use_weather_file_daylight_saving_period: boolean(&v["use_weather_dst"])?,
        apply_weekend_holiday_rule: boolean(&v["apply_weekend_holiday_rule"])?,
        use_weather_file_rain_indicators: true,
        use_weather_file_snow_indicators: true,
        treat_weather_as_actual: boolean(&v["treat_weather_as_actual"])?,
    })
}

fn calendar_frame(frame: &CalendarFrame) -> Value {
    json!({
        "hourly_sample_index": frame.hourly_sample_index, "day_of_sim": frame.day_of_sim,
        "year": frame.year, "month": frame.month, "day_of_month": frame.day_of_month,
        "gregorian_day_of_year": frame.gregorian_day_of_year, "weather_day_of_year": frame.weather_day_of_year,
        "schedule_day_of_year": frame.schedule_day_of_year, "gregorian_day_of_week": frame.gregorian_day_of_week,
        "day_of_week": frame.day_of_week, "day_type": frame.day_type, "day_type_label": frame.day_type_label,
        "gregorian_year_is_leap_year": frame.gregorian_year_is_leap_year,
        "weather_effective_year_is_leap_year": frame.weather_effective_year_is_leap_year,
        "leap_year_add": frame.leap_year_add, "dst": frame.dst, "special_day_type": frame.special_day_type,
        "hour_ending": frame.hour_ending,
    })
}

fn prepare(case: &Map<String, Value>, case_id: &str) -> Result<Value, String> {
    let run_period = run_period(&case["run_period"], case_id)?;
    let weather = object(&case["weather_calendar"], &["leap_year_observed"], &[])?;
    let metadata = EpwCalendarMetadata {
        leap_year_observed: boolean(&weather["leap_year_observed"])?,
        daylight_saving_period: None,
        holidays: Vec::new(),
    };
    let steps = integer(&case["zone_timesteps_per_hour"])?;
    if steps == 0 {
        return Err("zone_timesteps_per_hour must be positive".into());
    }
    let calendar = resolve_run_period_calendar(&run_period).map_err(|error| error.to_string())?;
    let weather_calendar = resolve_weather_environment_calendar(&run_period, &metadata)
        .map_err(|error| error.to_string())?;
    let hourly = build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
        &run_period,
        &metadata,
        steps,
    )
    .map_err(|error| error.to_string())?;
    let environment =
        build_environment_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
            &run_period,
            &metadata,
            1,
            steps,
        )
        .map_err(|error| error.to_string())?;
    let daily: Vec<_> = hourly
        .points
        .iter()
        .filter(|point| point.hour == 1)
        .map(CalendarFrame::from)
        .collect();
    Ok(json!({
        "case_id": case_id, "status": "resolved", "preparation_only": true, "physics_executed": false,
        "run_period_name": run_period.name.0,
        "resolved_calendar": {
            "start_year": calendar.start_year, "start_month": calendar.start_month, "start_day_of_month": calendar.start_day_of_month,
            "start_year_is_leap_year": calendar.start_year_is_leap_year,
            "end_year": calendar.end_year, "end_month": calendar.end_month, "end_day_of_month": calendar.end_day_of_month,
            "end_year_is_leap_year": calendar.end_year_is_leap_year, "total_days": calendar.total_days,
        },
        "weather_calendar": {
            "weather_file_allows_leap_years": weather_calendar.weather_file_allows_leap_years,
            "start_year_is_weather_effective_leap_year": weather_calendar.start_year_is_weather_effective_leap_year,
            "end_year_is_weather_effective_leap_year": weather_calendar.end_year_is_weather_effective_leap_year,
            "leap_days_skipped": weather_calendar.leap_days_skipped, "total_days": weather_calendar.total_days,
        },
        "hourly_sample_count": hourly.sample_count(), "zone_timestep_sample_count": environment.sample_count(),
        "zone_timesteps_per_hour": steps,
        "daily_frames": daily.iter().map(calendar_frame).collect::<Vec<_>>(),
        "month_start_frames": daily.iter().filter(|frame| frame.day_of_month == 1).map(calendar_frame).collect::<Vec<_>>(),
        "prepared_zone_points": environment.points.iter().map(|point| json!({
            "sample_index": point.sample_index, "day_of_sim": point.day_of_sim, "hour": point.hour,
            "zone_timestep": point.zone_timestep, "simulation_timestep": point.simulation_timestep,
            "start_minute_bits": format!("{:016x}", point.start_minute.to_bits()),
            "end_minute_bits": format!("{:016x}", point.end_minute.to_bits()),
            "current_time_hours_bits": format!("{:016x}", point.current_time_hours.to_bits()),
            "begin_environment": point.begin_environment, "end_environment": point.end_environment,
            "begin_day": point.begin_day, "end_day": point.end_day, "begin_hour": point.begin_hour, "end_hour": point.end_hour,
        })).collect::<Vec<_>>(),
        "state_boundary": "existing-prepared-axes; no-EP-global-cursor/Tomorrow/WeekDayTypes366-state-model",
    }))
}

fn execute(value: &Value) -> Result<Value, String> {
    if value["schema"] == "clk01-helper-tuples.v1" {
        return execute_helpers(value);
    }
    let request = object(value, &["schema", "cases"], &["source_metadata"])?;
    if request["schema"] != "clk01-calendar-cases.v1" {
        return Err("expected clk01-calendar-cases.v1".into());
    }
    let cases = request["cases"].as_array().ok_or("cases must be array")?;
    if cases.is_empty() {
        return Err("cases must be nonempty".into());
    }
    let mut ids = BTreeSet::new();
    let mut output = Vec::new();
    for value in cases {
        let case = object(
            value,
            &[
                "case_id",
                "run_period",
                "weather_calendar",
                "zone_timesteps_per_hour",
            ],
            &["source_metadata", "scope", "duration"],
        )?;
        let id = case["case_id"]
            .as_str()
            .filter(|id| !id.is_empty())
            .ok_or("case_id must be nonempty string")?;
        if !ids.insert(id) {
            return Err(format!("duplicate case_id {id}"));
        }
        output.push(match prepare(case, id) {
            Ok(value) => value,
            Err(error) => json!({"case_id":id, "status":"rejected", "error":error, "preparation_only":true, "physics_executed":false}),
        });
    }
    Ok(json!({"schema":"clk01-calendar-results.v1", "cases":output}))
}

fn execute_helpers(value: &Value) -> Result<Value, String> {
    use ep_runtime::time_axis::clock_trace::{existing_day_of_year, existing_is_leap_year};
    let request = object(value, &["schema", "calls"], &["source_metadata"])?;
    let calls = request["calls"].as_array().ok_or("calls must be array")?;
    let mut previous = BTreeMap::<u64, Value>::new();
    let mut indices = BTreeSet::new();
    let mut output = Vec::new();
    for value in calls {
        let call = object(value, &["call_index", "function", "inputs", "context"], &[])?;
        let index = call["call_index"]
            .as_u64()
            .ok_or("call_index must be u64")?;
        if !indices.insert(index) {
            return Err(format!("duplicate call_index {index}"));
        }
        let function = call["function"].as_str().ok_or("function must be string")?;
        if !call["context"].is_object() {
            return Err("context must be object".into());
        }
        let names: Option<&[&str]> = match function {
            "isLeapYear" => Some(&["year"]),
            "calculateDayOfYear" => Some(&["month", "day", "leap_year"]),
            "General::OrdinalDay" => Some(&["month", "day", "leap_year_add"]),
            _ => None,
        };
        let mut row = call.clone();
        let Some(names) = names else {
            row.insert("supported".into(), json!(false));
            row.insert(
                "boundary".into(),
                json!("no-existing-paired-Rust-helper-route; no-cloned-source-state"),
            );
            output.push(Value::Object(row));
            continue;
        };
        let raw_inputs = object(&call["inputs"], names, &[])?;
        let mut inputs = Map::new();
        for name in names {
            let value = &raw_inputs[*name];
            let resolved = if value.is_object() {
                let origin = object(value, &["from_call"], &[])?["from_call"]
                    .as_u64()
                    .ok_or("from_call must be u64")?;
                previous
                    .get(&origin)
                    .ok_or("from_call must refer to an earlier supported bool/integer result")?
                    .clone()
            } else {
                value.clone()
            };
            inputs.insert((*name).into(), resolved);
        }
        let result = match function {
            "isLeapYear" => Some(json!(existing_is_leap_year(integer(&inputs["year"])?))),
            "calculateDayOfYear" => existing_day_of_year(
                integer(&inputs["month"])?,
                integer(&inputs["day"])?,
                boolean(&inputs["leap_year"])?,
            )
            .map(|value| json!(value)),
            "General::OrdinalDay" => {
                let leap = integer(&inputs["leap_year_add"])?;
                if leap > 1 {
                    return Err(
                        "leap_year_add must be 0 or 1 for the existing Rust helper route".into(),
                    );
                }
                existing_day_of_year(
                    integer(&inputs["month"])?,
                    integer(&inputs["day"])?,
                    leap == 1,
                )
                .map(|value| json!(value))
            }
            _ => return Err("unsupported paired helper".into()),
        };
        row.insert("resolved_inputs".into(), Value::Object(inputs));
        row.insert("supported".into(), json!(result.is_some()));
        if let Some(result) = result {
            previous.insert(index, result.clone());
            row.insert("value".into(), result);
        } else {
            row.insert(
                "boundary".into(),
                json!("invalid-date-outside-paired-helper-domain; no-source-sentinel-model"),
            );
        }
        output.push(Value::Object(row));
    }
    Ok(json!({"schema":"clk01-helper-results.v1", "calls":output}))
}
