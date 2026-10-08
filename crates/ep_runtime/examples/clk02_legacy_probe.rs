//! Input-only observation of the existing public rich EPW loader.
//!
//! This baseline exposes actual legacy fields, not a source raw-parser owner.
//! The external recorded launcher must verify the request and weather SHA256.

use ep_runtime::weather::{
    EpwCalendarDateRule, EpwDataPeriodDate, EpwRecord, EpwWeatherFile, load_epw_weather_file,
};
use serde_json::{Value, json};
use std::{error::Error, path::Path};

fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value == f64::INFINITY {
        "positive_infinity"
    } else if value == f64::NEG_INFINITY {
        "negative_infinity"
    } else if value == 0.0 {
        if value.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value":if value.is_finite(){Some(value)}else{None},
        "value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}

fn date_rule(rule: EpwCalendarDateRule) -> Value {
    match rule {
        EpwCalendarDateRule::MonthDay {
            month,
            day_of_month,
        } => json!({"kind":"MonthDay","month":month,"day_of_month":day_of_month}),
        EpwCalendarDateRule::NthWeekdayInMonth {
            nth,
            weekday,
            month,
        } => {
            json!({"kind":"NthWeekdayInMonth","nth":nth,"weekday":format!("{weekday:?}"),"month":month})
        }
        EpwCalendarDateRule::LastWeekdayInMonth { weekday, month } => {
            json!({"kind":"LastWeekdayInMonth","weekday":format!("{weekday:?}"),"month":month})
        }
    }
}

fn period_date(date: EpwDataPeriodDate) -> Value {
    json!({"year":date.year,"month":date.month,"day":date.day})
}

fn metadata(weather: &EpwWeatherFile) -> (Value, Value) {
    let calendar = &weather.calendar_metadata;
    let periods = &weather.data_periods;
    let calendar = json!({
        "leap_year_observed":calendar.leap_year_observed,
        "daylight_saving_period":calendar.daylight_saving_period.map(|period|
            json!({"start":date_rule(period.start),"end":date_rule(period.end)})),
        "holidays":calendar.holidays.iter().map(|holiday|
            json!({"name":holiday.name,"date":date_rule(holiday.date)})).collect::<Vec<_>>(),
    });
    let periods = json!({
        "records_per_hour":periods.records_per_hour,
        "periods":periods.periods.iter().map(|period|json!({
            "name":period.name,"start_day_of_week":format!("{:?}",period.start_day_of_week),
            "start_date":period_date(period.start_date),"end_date":period_date(period.end_date),
        })).collect::<Vec<_>>(),
    });
    (calendar, periods)
}

fn record(record_index: usize, row: &EpwRecord) -> Value {
    json!({
        "record_index":record_index,
        "source_date_fields":{"year":row.year,"month":row.month,"day":row.day,
            "hour":row.hour,"minute":row.minute},
        "legacy_fields":{
            "dry_bulb_c":scalar(row.dry_bulb_c),
            "dew_point_c":scalar(row.dew_point_c),
            "relative_humidity_percent":scalar(row.relative_humidity_percent),
            "atmospheric_pressure_pa":scalar(row.atmospheric_pressure_pa),
            "horizontal_infrared_radiation_wh_per_m2":scalar(row.horizontal_infrared_radiation_wh_per_m2),
            "global_horizontal_radiation_wh_per_m2":scalar(row.global_horizontal_radiation_wh_per_m2),
            "direct_normal_radiation_wh_per_m2":scalar(row.direct_normal_radiation_wh_per_m2),
            "diffuse_horizontal_radiation_wh_per_m2":scalar(row.diffuse_horizontal_radiation_wh_per_m2),
            "wind_direction_deg":scalar(row.wind_direction_deg),
            "wind_speed_m_per_s":scalar(row.wind_speed_m_per_s),
            "liquid_precipitation_depth_mm":scalar(row.liquid_precipitation_depth_mm),
        },
    })
}

fn ids(request: &Value, group: &str, key: &str) -> Result<Vec<String>, String> {
    request[group]
        .as_array()
        .ok_or_else(|| format!("expected array {group}"))?
        .iter()
        .map(|row| {
            row[key]
                .as_str()
                .map(str::to_owned)
                .ok_or_else(|| format!("expected string {group}.{key}"))
        })
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() != 2 {
        return Err("usage: clk02_legacy_probe INPUT-ONLY-HELPER-REQUEST.json".into());
    }
    let request: Value = serde_json::from_str(&std::fs::read_to_string(&args[1])?)?;
    if request["schema"] != "clk02-helper-cases.v1"
        || request["expected_values_supplied"] != false
        || request["expected_exits_supplied"] != false
    {
        return Err("expected input-only clk02-helper-cases.v1".into());
    }
    let fixed = &request["fixed_epw"];
    let relative_path = fixed["file"]["path"]
        .as_str()
        .ok_or("missing fixed_epw.file.path")?;
    let declared_sha = fixed["file"]["sha256"]
        .as_str()
        .ok_or("missing fixed_epw.file.sha256")?;
    if declared_sha.len() != 64 || !declared_sha.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err("invalid declared weather SHA256".into());
    }
    let repo = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("missing repository root")?
        .canonicalize()?;
    let weather_path = repo.join(relative_path).canonicalize()?;
    if !weather_path.starts_with(&repo) || fixed["physics_executed"] != false {
        return Err("expected contained parser-only fixed EPW input".into());
    }
    let weather = load_epw_weather_file(&weather_path)?;
    let expected_count = fixed["record_count"]
        .as_u64()
        .ok_or("missing declared record_count")?;
    if u64::try_from(weather.records.len())? != expected_count {
        return Err("actual rich-loader record count differs from input-only request".into());
    }
    let (calendar, periods) = metadata(&weather);
    let result = json!({
        "schema":"clk02-legacy-probe-results.v1",
        "implementation_stage":"legacy_public_rich_loader",
        "route":"ep_runtime::weather::load_epw_weather_file",
        "physics_executed":false,"environment_selection_executed":false,
        "weather_interpolation_executed":false,"source_raw_parser_certified":false,
        "request_sha256_verification":"external recorded launcher required",
        "weather_sha256_verification":"external recorded launcher required",
        "fixed_epw":{
            "case_id":fixed["case_id"],"request_declared_input_ref":fixed["file"],
            "actual_loaded_path":weather_path,"record_count":weather.records.len(),
            "source_date_integer_type":"Rust u32; no civil calendar supplied or inferred",
            "records":weather.records.iter().enumerate().map(|(index,row)|record(index,row)).collect::<Vec<_>>(),
            "calendar_metadata":calendar,"data_periods":periods,
        },
        "available_record_fields":{"date_integers":5,"legacy_reals":11},
        "unavailable_original_mandatory_real_outputs":["ETHoriz","ETDirect","GLBHorizIllum",
            "DirectNrmIllum","DiffuseHorizIllum","ZenLum","TotalSkyCover","OpaqueSkyCover","Visibility","CeilHeight"],
        "unavailable_original_integer_outputs":["WObs"],
        "unavailable_original_weather_codes":9,
        "unavailable_original_optional_real_outputs":["PrecipWater","AerosolOptDepth","SnowDepth","DaysSinceLastSnow","Albedo"],
        "liquid_precipitation_semantics":"actual legacy field is observed; blank/invalid/absent and >=99 map to zero, negative values clamp; raw sentinel/error retention is not owned",
        "unavailable_header_ownership":["LOCATION and weather-file location numeric owner",
            "original header Line/stream cursor and caller token admission","remaining ProcessEPWHeader state and error availability"],
        "unavailable_record_state":["ErrorFound/fatal output availability","declared original output canary retention",
            "source weather-code missed counter and original mutable state"],
        "diagnostic_record_sequences_executed":0,
        "unsupported_record_sequence_ids":ids(&request,"record_sequences","sequence_id")?,
        "diagnostic_header_cases_executed":0,
        "unsupported_header_case_ids":ids(&request,"header_cases","case_id")?,
        "unconsumed_request_context":["record_sequences","header_cases","fixed_epw.initial_outputs",
            "fixed_epw.initial_ErrorsFound","fixed_epw.reset_declared_output_canaries_before_each_call"],
        "source_or_reference_outputs_supplied":false,"gates_updated":false,
    });
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}
