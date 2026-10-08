//! Output-only serialization of actual raw preparation and weather consumers.

use crate::{RunConfig, RunError, RunExitCode};
use ep_runtime::weather::raw::production_trace::{OBSERVATION_LIMIT, WeatherProductionTrace};
use ep_runtime::weather::raw::{
    DATE_KEYS, MANDATORY_KEYS, OPTIONAL_KEYS, RawEpwHeaderState, RawEpwOutputs, RawEpwStreamState,
};
use ep_runtime::weather::{EpwRecord, WeatherTimestepSample};
use serde_json::{Map, Value, json};
use std::io::{BufWriter, Write};

pub(crate) fn scalar(value: f64) -> Value {
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

fn named<const N: usize>(keys: [&str; N], values: [f64; N]) -> Value {
    Value::Object(
        keys.into_iter()
            .zip(values)
            .map(|(key, value)| (key.to_owned(), scalar(value)))
            .collect::<Map<_, _>>(),
    )
}

pub(crate) fn raw(value: &RawEpwOutputs) -> Value {
    json!({
        "ErrorFound":value.error_found,"WObs":value.observation_indicator,
        "date_fields":DATE_KEYS.into_iter().zip(value.dates).map(|(key, number)|(key.to_owned(),json!(number))).collect::<Map<_,_>>(),
        "mandatory_reals":named(MANDATORY_KEYS,value.mandatory_reals),
        "optional_reals":named(OPTIONAL_KEYS,value.optional_reals),
        "weather_codes":value.weather_codes,"private_RField21_observed":false,
    })
}

pub(crate) fn projected(value: &EpwRecord) -> Value {
    json!({
        "source_date_fields":{"year":value.year,"month":value.month,"day":value.day,"hour":value.hour,"minute":value.minute},
        "legacy_fields":{
            "dry_bulb_c":scalar(value.dry_bulb_c),"dew_point_c":scalar(value.dew_point_c),
            "relative_humidity_percent":scalar(value.relative_humidity_percent),
            "atmospheric_pressure_pa":scalar(value.atmospheric_pressure_pa),
            "horizontal_infrared_radiation_wh_per_m2":scalar(value.horizontal_infrared_radiation_wh_per_m2),
            "global_horizontal_radiation_wh_per_m2":scalar(value.global_horizontal_radiation_wh_per_m2),
            "direct_normal_radiation_wh_per_m2":scalar(value.direct_normal_radiation_wh_per_m2),
            "diffuse_horizontal_radiation_wh_per_m2":scalar(value.diffuse_horizontal_radiation_wh_per_m2),
            "wind_direction_deg":scalar(value.wind_direction_deg),"wind_speed_m_per_s":scalar(value.wind_speed_m_per_s),
            "liquid_precipitation_depth_mm":scalar(value.liquid_precipitation_depth_mm),
        },
    })
}

pub(crate) fn sample(value: &WeatherTimestepSample) -> Value {
    json!({
        "record_index":value.record_index,"zone_timestep":value.timestep,
        "dry_bulb_c":scalar(value.dry_bulb_c),"wet_bulb_c":scalar(value.wet_bulb_c),
        "relative_humidity_percent":scalar(value.relative_humidity_percent),
        "outdoor_humidity_ratio":scalar(value.outdoor_humidity_ratio),
        "atmospheric_pressure_pa":scalar(value.atmospheric_pressure_pa),
        "horizontal_infrared_radiation_w_per_m2":scalar(value.horizontal_infrared_radiation_w_per_m2),
        "global_horizontal_radiation_w_per_m2":scalar(value.global_horizontal_radiation_w_per_m2),
        "direct_normal_radiation_w_per_m2":scalar(value.direct_normal_radiation_w_per_m2),
        "diffuse_horizontal_radiation_w_per_m2":scalar(value.diffuse_horizontal_radiation_w_per_m2),
        "wind_speed_m_per_s":scalar(value.wind_speed_m_per_s),"wind_direction_deg":scalar(value.wind_direction_deg),
        "liquid_precipitation_depth_mm":scalar(value.liquid_precipitation_depth_mm),
    })
}

pub(crate) fn stream(value: RawEpwStreamState) -> Value {
    json!({"is_open":value.is_open,"good":value.good,"eof":value.eof,"fail":value.fail,"bad":value.bad,
        "position_byte":value.position_byte,"position_available":value.position_byte.is_some(),
        "native_rdstate_bits_claimed":false})
}

pub(crate) fn header(value: &RawEpwHeaderState) -> Value {
    let dst = |value: &ep_runtime::weather::raw::DstPeriod| {
        json!({
            "StDateType":value.start_date_type as i32,"StWeekDay":value.start_weekday,"StMon":value.start_month,"StDay":value.start_day,
            "EnDateType":value.end_date_type as i32,"EnWeekDay":value.end_weekday,"EnMon":value.end_month,"EnDay":value.end_day,
        })
    };
    json!({
        "EPWHeaderTitle":value.header_title,"WeatherFileLocationTitle":value.weather_location_title,
        "WeatherFileLatitude":scalar(value.latitude),"WeatherFileLongitude":scalar(value.longitude),
        "WeatherFileTimeZone":scalar(value.time_zone),"WeatherFileElevation":scalar(value.elevation),
        "WFAllowsLeapYears":value.allows_leap_years,"EPWDaylightSaving":value.epw_daylight_saving,
        "EPWDST":dst(&value.epw_dst),"DST":dst(&value.dst),
        "NumSpecialDays":value.number_special_days,"SpecialDays_allocated":value.special_days.allocated,
        "SpecialDays":value.special_days.values.iter().map(|day|json!({
            "Name":day.name,"dateType":day.date_type as i32,"Month":day.month,"Day":day.day,"WeekDay":day.weekday,
            "CompDate":day.compressed_date,"WthrFile":day.weather_file,"Duration":day.duration,"DayType":day.day_type,
            "ActStMon":day.actual_start_month,"ActStDay":day.actual_start_day,"Used":day.used,
        })).collect::<Vec<_>>(),
        "NumDataPeriods":value.number_data_periods,"NumIntervalsPerHour":value.intervals_per_hour,
        "DataPeriods_allocated":value.data_periods.allocated,
        "DataPeriods":value.data_periods.values.iter().map(|period|json!({
            "Name":period.name,"DayOfWeek":period.day_of_week,"NumYearsData":period.number_years_data,"WeekDay":period.weekday,
            "StMon":period.start_month,"StDay":period.start_day,"StYear":period.start_year,
            "EnMon":period.end_month,"EnDay":period.end_day,"EnYear":period.end_year,"NumDays":period.number_days,
            "MonWeekDay":period.month_weekdays,"DataStJDay":period.start_julian_day,"DataEnJDay":period.end_julian_day,
            "HasYearData":period.has_year_data,
        })).collect::<Vec<_>>(),
        "LeapYearAdd":value.leap_year_add,"EndDayOfMonth":value.month_ends,
        "wvarsMissedCounts.WeathCodes":value.weather_code_missed_count,
    })
}

pub(crate) fn write_trace(
    config: &RunConfig,
    trace: &WeatherProductionTrace,
) -> Result<(), RunError> {
    let omitted_prepared = trace.total_prepared_count - trace.prepared.len() as u64;
    let omitted_records =
        trace.total_selected_record_count - trace.retained_selected_record_count as u64;
    let omitted_consumers = trace.total_consumer_count - trace.consumers.len() as u64;
    let artifact = json!({
        "schema":"clk02-weather-production-trace.v1",
        "capture_source":"actual-Rust-runtime-raw-weather-preparation-and-existing-consumers",
        "prepared_lane":"eager-input-preview-only; live CLK-03 owner supplies production weather",
        "prepared_rows_supply_production_weather":false,
        "thread_coverage":"collecting-thread-only","observer_supplies_inputs":false,"observer_recomputes_weather":false,
        "observation_limit_per_series":OBSERVATION_LIMIT,"selected_record_pool_limit":OBSERVATION_LIMIT,
        "total_prepared_count":trace.total_prepared_count,"retained_prepared_count":trace.prepared.len(),"omitted_prepared_count":omitted_prepared,
        "total_selected_record_count":trace.total_selected_record_count,"retained_selected_record_count":trace.retained_selected_record_count,
        "omitted_selected_record_count":omitted_records,
        "total_consumer_count":trace.total_consumer_count,"retained_consumer_count":trace.consumers.len(),"omitted_consumer_count":omitted_consumers,
        "complete_on_collecting_thread":omitted_prepared==0 && omitted_records==0 && omitted_consumers==0,
        "truncation_reason":if omitted_prepared==0 && omitted_records==0 && omitted_consumers==0 {None} else {Some("observation_limit")},
        "prepared":trace.prepared.iter().map(|row|json!({
            "sequence":row.sequence,"caller":{"file":row.caller.file(),"line":row.caller.line(),"column":row.caller.column()},
            "phase":row.phase,"context":row.context.map(crate::psychrometrics_trace::execution_context),
            "weather_path":row.path.to_string_lossy(),"weather_byte_length":row.byte_length,
            "raw_record_count":row.raw_record_count,"selected_record_count":row.selected_record_count,
            "retained_selected_record_count":row.selected_records.len(),"omitted_selected_record_count":row.selected_record_count-row.selected_records.len(),
            "header_after_load":header(&row.header),"stream_after_header":stream(row.stream_after_header),"final_stream":stream(row.final_stream),
            "selected_records":row.selected_records.iter().enumerate().map(|(index,record)|json!({
                "selected_record_index":index,"source_record_index":record.source_record_index,
                "raw":raw(&record.raw),"projected":projected(&record.projected),
            })).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "consumers":trace.consumers.iter().map(|row|json!({
            "sequence":row.sequence,"caller":{"file":row.caller.file(),"line":row.caller.line(),"column":row.caller.column()},
            "phase":row.phase,"context":row.context.map(crate::psychrometrics_trace::execution_context),
            "record_index":row.record_index,"zone_timestep":row.zone_timestep,
            "received_hourly_record":projected(&row.record),"received_precomputed_sample":row.sample.as_ref().map(sample),
        })).collect::<Vec<_>>(),
        "prepared_rows_are_executed_zone_events":false,"consumer_repeated_identities_deduplicated":false,
        "input_sha256_verification":"external recorded launcher required",
        "claim_boundary":"raw owner and actual projected-record handoff plus literal existing consumer operands; source phase/calendar/interpolation/missing-data physics remains unpaired",
        "raw_source_year_equals_civil_calendar_year_claimed":false,"native_weather_callback_phase_claimed":false,
        "computed_precomputed_sample_values_paired_to_native":false,
    });
    let path = config
        .output_dir
        .join("clk02-weather-production-trace.json");
    let write = || -> Result<(), String> {
        let file = std::fs::File::create(&path).map_err(|error| error.to_string())?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &artifact).map_err(|error| error.to_string())?;
        writer
            .write_all(b"\n")
            .and_then(|()| writer.flush())
            .map_err(|error| error.to_string())
    };
    write().map_err(|message| RunError {
        exit_code: RunExitCode::OutputExport,
        message,
    })
}
