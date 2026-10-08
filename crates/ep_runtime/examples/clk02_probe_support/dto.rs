//! Read-only defined storage with the selected source-token field names.

use ep_runtime::weather::raw::{
    DATE_KEYS, DataPeriod, DstPeriod, MANDATORY_KEYS, OPTIONAL_KEYS, RawEpwHeaderState,
    RawEpwInput, RawEpwOutputs, SpecialDay,
};
use serde_json::{Map, Value, json};
use std::path::Path;

pub(super) fn scalar(value: f64) -> Value {
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

fn named_reals(keys: &[&str], values: &[f64]) -> Value {
    keys.iter()
        .zip(values)
        .map(|(key, value)| ((*key).to_owned(), scalar(*value)))
        .collect::<Map<String, Value>>()
        .into()
}

pub(super) fn raw(value: &RawEpwOutputs) -> Value {
    let dates: Map<String, Value> = DATE_KEYS
        .iter()
        .zip(value.dates)
        .map(|(key, value)| ((*key).to_owned(), json!(value)))
        .collect();
    json!({"ErrorFound":value.error_found,"date_fields":dates,
        "mandatory_reals":named_reals(&MANDATORY_KEYS,&value.mandatory_reals),
        "WObs":value.observation_indicator,"weather_codes":value.weather_codes,
        "optional_reals":named_reals(&OPTIONAL_KEYS,&value.optional_reals),
        "private_RField21_observed":false})
}

fn dst(value: &DstPeriod) -> Value {
    json!({"StDateType":value.start_date_type as i32,"StWeekDay":value.start_weekday,
        "StMon":value.start_month,"StDay":value.start_day,
        "EnDateType":value.end_date_type as i32,"EnMon":value.end_month,
        "EnDay":value.end_day,"EnWeekDay":value.end_weekday})
}

fn period(value: &DataPeriod) -> Value {
    json!({"Name":value.name,"DayOfWeek":value.day_of_week,
        "NumYearsData":value.number_years_data,"WeekDay":value.weekday,
        "StMon":value.start_month,"StDay":value.start_day,"StYear":value.start_year,
        "EnMon":value.end_month,"EnDay":value.end_day,"EnYear":value.end_year,
        "NumDays":value.number_days,"MonWeekDay":value.month_weekdays,
        "DataStJDay":value.start_julian_day,"DataEnJDay":value.end_julian_day,
        "HasYearData":value.has_year_data})
}

fn special(value: &SpecialDay) -> Value {
    json!({"Name":value.name,"dateType":value.date_type as i32,"Month":value.month,
        "Day":value.day,"WeekDay":value.weekday,"CompDate":value.compressed_date,
        "WthrFile":value.weather_file,"Duration":value.duration,"DayType":value.day_type,
        "ActStMon":value.actual_start_month,"ActStDay":value.actual_start_day,"Used":value.used})
}

pub(super) fn header(value: &RawEpwHeaderState) -> Value {
    json!({"EPWHeaderTitle":value.header_title,"WeatherFileLocationTitle":value.weather_location_title,
        "WeatherFileLatitude":scalar(value.latitude),"WeatherFileLongitude":scalar(value.longitude),
        "WeatherFileTimeZone":scalar(value.time_zone),"WeatherFileElevation":scalar(value.elevation),
        "WFAllowsLeapYears":value.allows_leap_years,"EPWDaylightSaving":value.epw_daylight_saving,
        "EPWDST":dst(&value.epw_dst),"DST":dst(&value.dst),"NumSpecialDays":value.number_special_days,
        "SpecialDays":value.special_days.values.iter().map(special).collect::<Vec<_>>(),
        "SpecialDays_allocated":value.special_days.allocated,"NumDataPeriods":value.number_data_periods,
        "NumIntervalsPerHour":value.intervals_per_hour,
        "DataPeriods":value.data_periods.values.iter().map(period).collect::<Vec<_>>(),
        "DataPeriods_allocated":value.data_periods.allocated,"LeapYearAdd":value.leap_year_add,
        "EndDayOfMonth":value.month_ends,"wvarsMissedCounts.WeathCodes":value.weather_code_missed_count})
}

pub(super) fn stream(input: &RawEpwInput, file_path: &Path) -> Value {
    let state = input.snapshot();
    json!({"file_path":file_path,"is_open":state.is_open,"good":state.good,
        "eof":state.eof,"fail":state.fail,"bad":state.bad,
        "position_byte":state.position_byte,"position_available":state.position_byte.is_some()})
}

pub(super) fn returned() -> Value {
    json!({"status":"source_returned","source_fatal":false,"exception_message":null})
}

pub(super) fn fatal(message: &str) -> Value {
    json!({"status":"source_fatal","source_fatal":true,"exception_message":message})
}
