//! Input-only observations of the existing eager weather/calendar pipeline.
//! Native daily-carrier, cursor replay, handoff and warmup owners are unavailable.

#[path = "clk02_probe_support/digest.rs"]
mod digest;

use ep_model::{
    DayOfWeek, FirstHourInterpolationStartingValues, NormalizedName, RunPeriod, RunPeriodId,
};
use ep_runtime::time_axis::{
    EnvironmentTimePoint, TimeAxis, TimePoint,
    build_environment_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps,
    build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps,
};
use ep_runtime::weather::raw::{RawEpwStreamState, load_parsed_epw_weather_file};
use ep_runtime::weather::{EpwRecord, select_epw_environment_weather};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let path = args
        .next()
        .ok_or("usage: clk03_legacy_probe <input-only-request.json>")?;
    require(
        args.next().is_none(),
        "exactly one request argument required",
    )?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("repository root unavailable")?
        .canonicalize()?;
    let path = Path::new(&path).canonicalize()?;
    require(path.starts_with(&root), "request must be inside repository")?;
    let bytes = std::fs::read(&path)?;
    let request: Value = serde_json::from_slice(&bytes)?;
    require(
        text(&request["schema"])? == "clk03-helper-cases.v1",
        "final frozen request schema required",
    )?;
    require(
        request["expected_values_supplied"] == false && request["expected_exits_supplied"] == false,
        "input-only request declarations required",
    )?;
    require(
        request["draft_not_frozen"] != true,
        "unused draft request forbidden",
    )?;
    let (_, scope) = bound_file(&root, &request["fixed_scope"])?;
    let mut contracts = serde_json::Map::new();
    for name in ["source", "cases", "tolerances"] {
        let relative = format!("energyplus_porting_plan/contracts/CLK-03-{name}.json");
        let file = root.join(&relative).canonicalize()?;
        require(
            file.starts_with(&root),
            "contract reference escapes repository",
        )?;
        contracts.insert(
            name.to_owned(),
            json!({"path":relative,"sha256":digest::sha256(&std::fs::read(file)?)}),
        );
    }
    let weather_inputs = array(&request["weather_sequences"])?;
    let handoff_inputs = array(&request["handoff_sequences"])?;
    require(
        weather_inputs.len() == 10 && handoff_inputs.len() == 4,
        "frozen sequence cardinality differs",
    )?;
    // Validate every declared file identity before any weather API executes.
    let prepared = weather_inputs
        .iter()
        .map(|input| {
            let (idf, _) = bound_file(&root, &input["input"])?;
            let (weather, _) = bound_file(&root, &input["weather"])?;
            Ok((idf, weather))
        })
        .collect::<Result<Vec<_>>>()?;
    let observations = weather_inputs
        .iter()
        .zip(&prepared)
        .map(|(input, (_, weather))| observe_weather(input, weather))
        .collect::<Result<Vec<_>>>()?;
    // Check that the same named input bytes remained present after the actual reads.
    for input in weather_inputs {
        bound_file(&root, &input["input"])?;
        bound_file(&root, &input["weather"])?;
    }
    require(
        std::fs::read(&path)? == bytes,
        "request changed during observations",
    )?;
    let handoffs = handoff_inputs.iter().map(|input| -> Result<Value> {
        Ok(json!({"id":text(&input["id"])?, "available":false,
            "operation_execution_available":false, "operations":unavailable_operations(input)?,
            "native_handoff_state":null, "reason":"existing pipeline has no whole native handoff owner"}))
    }).collect::<Result<Vec<_>>>()?;
    let result = json!({
        "schema":"clk03-legacy-probe-results.v1",
        "contracts":contracts,
        "actual_request":{"path":relative(&root,&path)?,"sha256":digest::sha256(&bytes)},
        "fixed_scope":{"path":request["fixed_scope"]["path"],"sha256":digest::sha256(&scope)},
        "handoff_sequences":handoffs,"weather_sequences":observations,
        "complete":true,"expected_answers_supplied":false,"native_reference_results_read":false,
        "probe_changes_production_sources":false,"gates_updated":false,
        "observation_boundary":"existing eager full-file raw parse and calendar/selector precomputation",
        "native_processed_weather_comparison_claimed":false,
        "unavailable_native_owners":["mutable Today/Tomorrow WeatherVars carriers","UpdateWeatherData whole-call writes",
            "live selection/read-day cursor and partial state","previous-hour lifecycle cache",
            "runtime warmup replay","native GetNextEnvironment admission and original fatal outcomes",
            "InitializeWeather end-environment rewind and report writes"]
    });
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn require(condition: bool, message: &str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}

fn text(value: &Value) -> Result<&str> {
    value
        .as_str()
        .ok_or_else(|| "typed string input required".into())
}

fn integer(value: &Value) -> Result<u32> {
    Ok(u32::try_from(
        value.as_u64().ok_or("nonnegative integer input required")?,
    )?)
}

fn flag(value: &Value) -> Result<bool> {
    value
        .as_bool()
        .ok_or_else(|| "typed boolean input required".into())
}

fn array(value: &Value) -> Result<&[Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "typed array input required".into())
}

fn relative(root: &Path, path: &Path) -> Result<String> {
    Ok(path
        .strip_prefix(root)?
        .to_str()
        .ok_or("UTF-8 repository path required")?
        .replace('\\', "/"))
}

fn bound_file(root: &Path, binding: &Value) -> Result<(PathBuf, Vec<u8>)> {
    let path = root.join(text(&binding["path"])?).canonicalize()?;
    require(path.starts_with(root), "input reference escapes repository")?;
    let bytes = std::fs::read(&path)?;
    require(
        digest::sha256(&bytes) == text(&binding["sha256"])?,
        "actual input byte hash differs",
    )?;
    Ok((path, bytes))
}

fn weekday(value: u32) -> Result<DayOfWeek> {
    match value {
        1 => Ok(DayOfWeek::Sunday),
        2 => Ok(DayOfWeek::Monday),
        3 => Ok(DayOfWeek::Tuesday),
        4 => Ok(DayOfWeek::Wednesday),
        5 => Ok(DayOfWeek::Thursday),
        6 => Ok(DayOfWeek::Friday),
        7 => Ok(DayOfWeek::Saturday),
        _ => Err("native weekday input outside 1..7".into()),
    }
}

fn weekday_number(value: DayOfWeek) -> u32 {
    match value {
        DayOfWeek::Sunday => 1,
        DayOfWeek::Monday => 2,
        DayOfWeek::Tuesday => 3,
        DayOfWeek::Wednesday => 4,
        DayOfWeek::Thursday => 5,
        DayOfWeek::Friday => 6,
        DayOfWeek::Saturday => 7,
    }
}

fn period(input: &Value) -> Result<RunPeriod> {
    let policy = match text(&input["first_hour_interpolation_starting_values"])? {
        "Hour1" => FirstHourInterpolationStartingValues::Hour1,
        "Hour24" => FirstHourInterpolationStartingValues::Hour24,
        _ => return Err("declared interpolation policy invalid".into()),
    };
    let year = |value: &Value| -> Result<Option<u32>> {
        if value.is_null() {
            Ok(None)
        } else {
            Ok(Some(integer(value)?))
        }
    };
    Ok(RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new(text(&input["name"])?),
        begin_month: integer(&input["begin_month"])?,
        begin_day_of_month: integer(&input["begin_day_of_month"])?,
        begin_year: year(&input["begin_year"])?,
        end_month: integer(&input["end_month"])?,
        end_day_of_month: integer(&input["end_day_of_month"])?,
        end_year: year(&input["end_year"])?,
        day_of_week_for_start_day: Some(weekday(integer(&input["start_weekday"])?)?),
        first_hour_interpolation_starting_values: policy,
        use_weather_file_holidays_and_special_days: flag(&input["use_weather_holidays"])?,
        use_weather_file_daylight_saving_period: flag(&input["use_weather_dst"])?,
        apply_weekend_holiday_rule: flag(&input["apply_weekend_holiday_rule"])?,
        // All admitted IDFs explicitly request Yes for rain and snow indicators.
        // These fields are not calculation operands of the selected axis/selector APIs.
        use_weather_file_rain_indicators: true,
        use_weather_file_snow_indicators: true,
        treat_weather_as_actual: flag(&input["treat_weather_as_actual"])?,
    })
}

fn unavailable_operations(input: &Value) -> Result<Vec<Value>> {
    array(&input["operations"])?
        .iter()
        .map(|op| -> Result<Value> {
            Ok(json!({"id":text(&op["id"])?,"kind":text(&op["kind"])?,
            "available":false,"executed":false,"native_before":null,"native_after":null,
            "actual_native_fatal":null,"native_returned":null}))
        })
        .collect()
}

fn observe_weather(input: &Value, weather: &Path) -> Result<Value> {
    let run_period = period(&input["run_period"])?;
    let steps = integer(&input["time_steps_per_hour"])?;
    require(steps == 4, "bounded four-step caller required")?;
    let mut result = json!({"id":text(&input["id"])?,"input":input["input"],"weather":input["weather"],
        "declared_run_period":input["run_period"],"requested_operations":unavailable_operations(input)?,
        "operation_execution_available":false,"actual_api_calls":["load_parsed_epw_weather_file"],
        "status":"raw_loader_error","error":null,"raw_loader":null,"selection":null,
        "clock_axis":null,"environment_clock_axis":null,
        "native_handoff_state":null,"native_live_selection_cursor":null,
        "native_runtime_warmup_state":null,"raw_records_are_processed_Tomorrow":false});
    let parsed = match load_parsed_epw_weather_file(weather) {
        Ok(parsed) => parsed,
        Err(error) => {
            result["error"] = json!({"display":error.to_string(),"debug":format!("{error:?}")});
            return Ok(result);
        }
    };
    let header = &parsed.raw_header;
    result["raw_loader"] = json!({"raw_record_count":parsed.raw_records.len(),
        "physical_record_count":parsed.physical_projection.records.len(),"input_byte_length":parsed.input_byte_length,
        "stream_after_header":stream(parsed.stream_after_header),"final_stream":stream(parsed.final_stream),
        "weather_code_missed_count":header.weather_code_missed_count,
        "raw_header_snapshot_fields":"selected-public-fields-only",
        "raw_header":{"header_title":header.header_title,"weather_location_title":header.weather_location_title,
            "latitude":bits(header.latitude),"longitude":bits(header.longitude),"time_zone":bits(header.time_zone),"elevation":bits(header.elevation),
            "allows_leap_years":header.allows_leap_years,"epw_daylight_saving":header.epw_daylight_saving,
            "number_special_days":header.number_special_days,"special_days_allocated":header.special_days.allocated,
            "number_data_periods":header.number_data_periods,"intervals_per_hour":header.intervals_per_hour,
            "data_periods_allocated":header.data_periods.allocated,"leap_year_add":header.leap_year_add,"month_ends":header.month_ends},
        "records_per_hour":parsed.physical_projection.data_periods.records_per_hour,
        "calendar_metadata":{"leap_year_observed":parsed.physical_projection.calendar_metadata.leap_year_observed,
            "daylight_saving_period":format!("{:?}",parsed.physical_projection.calendar_metadata.daylight_saving_period),
            "holidays":format!("{:?}",parsed.physical_projection.calendar_metadata.holidays)}});
    result["actual_api_calls"]
        .as_array_mut()
        .ok_or("API list owner unavailable")?
        .push(json!(
            "build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps"
        ));
    let axis = match build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
        &run_period,
        &parsed.physical_projection.calendar_metadata,
        steps,
    ) {
        Ok(axis) => axis,
        Err(error) => {
            result["status"] = json!("hourly_axis_error");
            result["error"] = json!({"display":error.to_string(),"debug":format!("{error:?}")});
            return Ok(result);
        }
    };
    result["clock_axis"] = axis_value(&axis);
    result["actual_api_calls"]
        .as_array_mut()
        .ok_or("API list owner unavailable")?
        .push(json!(
            "build_environment_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps"
        ));
    let environment_axis =
        build_environment_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
            &run_period,
            &parsed.physical_projection.calendar_metadata,
            1,
            steps,
        );
    result["environment_clock_axis"] = match environment_axis {
        Ok(axis) => json!({"status":"returned","environment_index":axis.environment_index,
            "environment_name":axis.environment_name,"kind":format!("{:?}",axis.environment_kind),
            "native_environment_ordinal_available":false,"runtime_clock_executed":false,
            "zone_timesteps_per_hour":axis.zone_timestep.timesteps_per_hour,
            "zone_timestep_seconds":bits(axis.zone_timestep.timestep_seconds),
            "points":axis.points.iter().map(environment_point).collect::<Vec<_>>()}),
        Err(error) => {
            json!({"status":"error","display":error.to_string(),"debug":format!("{error:?}")})
        }
    };
    result["actual_api_calls"]
        .as_array_mut()
        .ok_or("API list owner unavailable")?
        .push(json!("select_epw_environment_weather"));
    match select_epw_environment_weather(&parsed.physical_projection, &axis) {
        Ok(selected) => {
            result["status"] = json!("selection_returned");
            result["selection"] = json!({"data_period_index":selected.data_period_index,
                "source_start_record_index":selected.source_start_record_index,
                "initial_tomorrow_source_record_start":selected.initial_tomorrow_source_record_start,
                "selected_source_record_indices":selected.selected_source_record_indices,
                "skipped_february_29_source_record_starts":selected.skipped_february_29_source_record_starts,
                "day_buffer_transitions":selected.day_buffer_transitions.iter().map(|day| json!({
                    "day_of_sim":day.day_of_sim,"today_source_record_start":day.today_source_record_start,
                    "tomorrow_source_record_start":day.tomorrow_source_record_start,
                    "prefetched_next_day":day.prefetched_next_day})).collect::<Vec<_>>(),
                "hourly_records":selected.hourly_records().iter().map(record).collect::<Vec<_>>(),
                "indices_and_transitions_are_precomputed":true,"mutable_native_day_buffer_observed":false});
        }
        Err(error) => {
            result["status"] = json!("selection_error");
            result["error"] = json!({"display":error.to_string(),"debug":format!("{error:?}")});
        }
    }
    Ok(result)
}

fn bits(value: f64) -> Value {
    json!({"bits":format!("{:016x}",value.to_bits())})
}

fn stream(value: RawEpwStreamState) -> Value {
    json!({"is_open":value.is_open,"good":value.good,"eof":value.eof,"fail":value.fail,
        "bad":value.bad,"position_byte":value.position_byte,"native_rdstate_bits_available":false})
}

fn record(value: &EpwRecord) -> Value {
    json!({"dates":[value.year,value.month,value.day,value.hour,value.minute],
        "dry_bulb_c":bits(value.dry_bulb_c),"dew_point_c":bits(value.dew_point_c),
        "relative_humidity_percent":bits(value.relative_humidity_percent),"atmospheric_pressure_pa":bits(value.atmospheric_pressure_pa),
        "horizontal_infrared_radiation_wh_per_m2":bits(value.horizontal_infrared_radiation_wh_per_m2),
        "global_horizontal_radiation_wh_per_m2":bits(value.global_horizontal_radiation_wh_per_m2),
        "direct_normal_radiation_wh_per_m2":bits(value.direct_normal_radiation_wh_per_m2),
        "diffuse_horizontal_radiation_wh_per_m2":bits(value.diffuse_horizontal_radiation_wh_per_m2),
        "wind_direction_deg":bits(value.wind_direction_deg),"wind_speed_m_per_s":bits(value.wind_speed_m_per_s),
        "liquid_precipitation_depth_mm":bits(value.liquid_precipitation_depth_mm)})
}

fn axis_value(axis: &TimeAxis) -> Value {
    json!({"run_period_name":axis.run_period_name,
        "first_hour_interpolation_starting_values":format!("{:?}",axis.first_hour_interpolation_starting_values),
        "zone_timesteps_per_hour":axis.zone_timestep.timesteps_per_hour,
        "zone_timestep_seconds":bits(axis.zone_timestep.timestep_seconds),
        "system_nominal_timestep_seconds":bits(axis.system_timestep.nominal_timestep_seconds),
        "variable_system_timestep_support":axis.system_timestep.variable_system_timestep_support,
        "shorten_timestep_sys_state":axis.system_timestep.shorten_timestep_sys_state,
        "use_zone_timestep_history_state":axis.system_timestep.use_zone_timestep_history_state,
        "warmup_reported_samples":axis.sample_partitions.warmup_reported_samples,
        "run_period_reported_samples":axis.sample_partitions.run_period_reported_samples,
        "design_day_reported_samples":axis.sample_partitions.design_day_reported_samples,
        "points":axis.points.iter().map(hourly_point).collect::<Vec<_>>(),
        "runtime_clock_executed":false})
}

fn hourly_point(point: &TimePoint) -> Value {
    json!({"sample_index":point.sample_index,"day_of_sim":point.day_of_sim,"year":point.year,
        "gregorian_year_is_leap_year":point.gregorian_year_is_leap_year,
        "weather_effective_year_is_leap_year":point.weather_effective_year_is_leap_year,"leap_year_add":point.leap_year_add,
        "month":point.month,"day_of_month":point.day_of_month,"gregorian_day_of_year":point.gregorian_day_of_year,
        "day_of_year":point.day_of_year,"schedule_day_of_year":point.schedule_day_of_year,
        "gregorian_day_of_week":weekday_number(point.gregorian_day_of_week),"day_of_week":weekday_number(point.day_of_week),
        "day_type":point.day_type.energyplus_index(),"dst":point.dst,
        "special_day_type":point.special_day_type.map(|value|value.energyplus_index()),
        "tomorrow_day_of_week":weekday_number(point.tomorrow_day_of_week),"tomorrow_day_type":point.tomorrow_day_type.energyplus_index(),
        "tomorrow_special_day_type":point.tomorrow_special_day_type.map(|value|value.energyplus_index()),
        "hour":point.hour,"start_minute":bits(point.start_minute),"end_minute":bits(point.end_minute)})
}

fn environment_point(point: &EnvironmentTimePoint) -> Value {
    json!({"sample_index":point.sample_index,"environment_index":point.environment_index,
        "environment_kind":format!("{:?}",point.environment_kind),"day_of_sim":point.day_of_sim,"year":point.year,
        "gregorian_year_is_leap_year":point.gregorian_year_is_leap_year,
        "weather_effective_year_is_leap_year":point.weather_effective_year_is_leap_year,"leap_year_add":point.leap_year_add,
        "month":point.month,"day_of_month":point.day_of_month,"gregorian_day_of_year":point.gregorian_day_of_year,
        "day_of_year":point.day_of_year,"schedule_day_of_year":point.schedule_day_of_year,
        "gregorian_day_of_week":weekday_number(point.gregorian_day_of_week),"day_of_week":weekday_number(point.day_of_week),
        "day_type":point.day_type.energyplus_index(),"dst":point.dst,
        "special_day_type":point.special_day_type.map(|value|value.energyplus_index()),
        "tomorrow_day_of_week":weekday_number(point.tomorrow_day_of_week),"tomorrow_day_type":point.tomorrow_day_type.energyplus_index(),
        "tomorrow_special_day_type":point.tomorrow_special_day_type.map(|value|value.energyplus_index()),
        "hour":point.hour,"zone_timestep":point.zone_timestep,"start_minute":bits(point.start_minute),
        "end_minute":bits(point.end_minute),"current_time_hours":bits(point.current_time_hours),
        "simulation_timestep":point.simulation_timestep,"begin_environment":point.begin_environment,
        "end_environment":point.end_environment,"begin_day":point.begin_day,"end_day":point.end_day,
        "begin_hour":point.begin_hour,"end_hour":point.end_hour})
}
