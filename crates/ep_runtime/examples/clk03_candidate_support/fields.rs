//! Read-only snapshots of selected actual Rust owners in native-shaped keys.
// Reuse the frozen CLK-02 DTO without changing its unused legacy call helpers.
#[allow(dead_code)]
#[path = "../clk02_probe_support/dto.rs"]
mod raw_dto;
use ep_runtime::weather::day::{
    DailyWeatherVariables, WeatherDayState, WeatherDayValues, WeatherEnvironmentState,
    WeatherGlobalState, WeatherOwnerState, WeatherVarCounts, WeatherVars,
};
use ep_runtime::weather::raw::{RawEpwCursorOwner, RawEpwInput, RawEpwStreamState};
use serde_json::{Value, json};
use std::path::Path;
fn day(v: &DailyWeatherVariables) -> Value {
    json!({
        "DayOfYear":v.day_of_year,
        "DayOfYear_Schedule":v.day_of_year_schedule,
        "Year":v.year,
        "Month":v.month,
        "DayOfMonth":v.day_of_month,
        "DayOfWeek":v.day_of_week,
        "DaylightSavingIndex":v.daylight_saving_index,
        "HolidayIndex":v.holiday_index,
        "SinSolarDeclinAngle":raw_dto::scalar(v.sin_solar_declin_angle),
        "CosSolarDeclinAngle":raw_dto::scalar(v.cos_solar_declin_angle),
        "EquationOfTime":raw_dto::scalar(v.equation_of_time),
    })
}

fn weather(v: &WeatherVars) -> Value {
    json!({
        "IsRain":v.is_rain,
        "IsSnow":v.is_snow,
        "OutDryBulbTemp":raw_dto::scalar(v.out_dry_bulb_temp),
        "OutDewPointTemp":raw_dto::scalar(v.out_dew_point_temp),
        "OutBaroPress":raw_dto::scalar(v.out_baro_press),
        "OutRelHum":raw_dto::scalar(v.out_rel_hum),
        "WindSpeed":raw_dto::scalar(v.wind_speed),
        "WindDir":raw_dto::scalar(v.wind_dir),
        "SkyTemp":raw_dto::scalar(v.sky_temp),
        "HorizIRSky":raw_dto::scalar(v.horiz_ir_sky),
        "BeamSolarRad":raw_dto::scalar(v.beam_solar_rad),
        "DifSolarRad":raw_dto::scalar(v.dif_solar_rad),
        "Albedo":raw_dto::scalar(v.albedo),
        "WaterPrecip":raw_dto::scalar(v.water_precip),
        "LiquidPrecip":raw_dto::scalar(v.liquid_precip),
        "TotalSkyCover":raw_dto::scalar(v.total_sky_cover),
        "OpaqueSkyCover":raw_dto::scalar(v.opaque_sky_cover),
    })
}

fn counts(v: &WeatherVarCounts) -> Value {
    json!({
        "OutDryBulbTemp":v.out_dry_bulb_temp,
        "OutDewPointTemp":v.out_dew_point_temp,
        "OutRelHum":v.out_rel_hum,
        "OutBaroPress":v.out_baro_press,
        "WindDir":v.wind_dir,
        "WindSpeed":v.wind_speed,
        "BeamSolarRad":v.beam_solar_rad,
        "DifSolarRad":v.dif_solar_rad,
        "TotalSkyCover":v.total_sky_cover,
        "OpaqueSkyCover":v.opaque_sky_cover,
        "Visibility":v.visibility,
        "Ceiling":v.ceiling,
        "LiquidPrecip":v.liquid_precip,
        "WaterPrecip":v.water_precip,
        "AerOptDepth":v.aer_opt_depth,
        "SnowDepth":v.snow_depth,
        "DaysLastSnow":v.days_last_snow,
        "WeathCodes":v.weath_codes,
        "Albedo":v.albedo,
    })
}

fn global(v: &WeatherGlobalState) -> Value {
    json!({
        "BeginSimFlag":v.begin_sim_flag,
        "BeginEnvrnFlag":v.begin_envrn_flag,
        "BeginDayFlag":v.begin_day_flag,
        "BeginHourFlag":v.begin_hour_flag,
        "BeginTimeStepFlag":v.begin_time_step_flag,
        "EndDayFlag":v.end_day_flag,
        "EndHourFlag":v.end_hour_flag,
        "EndEnvrnFlag":v.end_envrn_flag,
        "EndDesignDayEnvrnsFlag":v.end_design_day_envrns_flag,
        "WarmupFlag":v.warmup_flag,
        "DayOfSim":v.day_of_sim,
        "CalendarYear":v.calendar_year,
        "PreviousHour":v.previous_hour,
        "HourOfDay":v.hour_of_day,
        "NumOfDayInEnvrn":v.num_of_day_in_envrn,
        "TimeStepsInHour":v.time_steps_in_hour,
        "TimeStep":v.time_step,
        "KindOfSim":v.kind_of_sim,
        "DoWeathSim":v.do_weath_sim,
        "DoDesDaySim":v.do_des_day_sim,
        "DoOutputReporting":v.do_output_reporting,
        "TimeStepZone":raw_dto::scalar(v.time_step_zone),
        "DayOfSimChr":v.day_of_sim_chr,
        "CalendarYearChr":v.calendar_year_chr,
    })
}

fn environment(v: &WeatherEnvironmentState) -> Value {
    json!({
        "DayOfYear":v.day_of_year,
        "DayOfYear_Schedule":v.day_of_year_schedule,
        "Year":v.year,
        "Month":v.month,
        "DayOfMonth":v.day_of_month,
        "DayOfWeek":v.day_of_week,
        "HolidayIndex":v.holiday_index,
        "DSTIndicator":v.dst_indicator,
        "YearTomorrow":v.year_tomorrow,
        "MonthTomorrow":v.month_tomorrow,
        "DayOfMonthTomorrow":v.day_of_month_tomorrow,
        "DayOfWeekTomorrow":v.day_of_week_tomorrow,
        "HolidayIndexTomorrow":v.holiday_index_tomorrow,
        "SinSolarDeclinAngle":raw_dto::scalar(v.sin_solar_declin_angle),
        "CosSolarDeclinAngle":raw_dto::scalar(v.cos_solar_declin_angle),
        "EquationOfTime":raw_dto::scalar(v.equation_of_time),
        "TotDesDays":v.tot_des_days,
        "CurEnvirNum":v.cur_envir_num,
        "RunPeriodStartDayOfWeek":v.run_period_start_day_of_week,
        "Latitude":raw_dto::scalar(v.latitude),
        "Longitude":raw_dto::scalar(v.longitude),
        "Elevation":raw_dto::scalar(v.elevation),
        "StdBaroPress":raw_dto::scalar(v.std_baro_press),
        "EnvironmentName":v.environment_name,
        "EndMonthFlag":v.end_month_flag,
        "EndYearFlag":v.end_year_flag,
    })
}

fn owner(v: &WeatherOwnerState) -> Value {
    json!({
        "Envrn":v.envrn,
        "NumOfEnvrn":v.num_of_envrn,
        "TotRunPers":v.tot_run_pers,
        "TotRunDesPers":v.tot_run_des_pers,
        "NumDataPeriods":v.num_data_periods,
        "NumIntervalsPerHour":v.num_intervals_per_hour,
        "NumSpecialDays":v.num_special_days,
        "LeapYearAdd":v.leap_year_add,
        "RptDayType":v.rpt_day_type,
        "CurDayOfWeek":v.cur_day_of_week,
        "curSimDayForEndOfRunPeriod":v.cur_sim_day_for_end_of_run_period,
        "GetBranchInputOneTimeFlag":v.get_branch_input_one_time_flag,
        "GetEnvironmentFirstCall":v.get_environment_first_call,
        "FirstCall":v.first_call,
        "WaterMainsParameterReport":v.water_mains_parameter_report,
        "LastHourSet":v.last_hour_set,
        "WeatherFileExists":v.weather_file_exists,
        "DatesShouldBeReset":v.dates_should_be_reset,
        "StartDatesCycleShouldBeReset":v.start_dates_cycle_should_be_reset,
        "Jan1DatesShouldBeReset":v.jan1_dates_should_be_reset,
        "RPReadAllWeatherData":v.rp_read_all_weather_data,
        "UseDaylightSaving":v.use_daylight_saving,
        "UseSpecialDays":v.use_special_days,
        "DaylightSavingIsActive":v.daylight_saving_is_active,
        "ReadEPlusWeatherCurTime":raw_dto::scalar(v.read_e_plus_weather_cur_time),
        "TimeStepFraction":raw_dto::scalar(v.time_step_fraction),
        "IsRainThreshold":raw_dto::scalar(v.is_rain_threshold),
    })
}

fn slots(value: &WeatherDayValues) -> Value {
    let rows=value.slots().iter().enumerate().map(|(index,v)|json!({"hour":index/value.time_steps()+1,"time_step":index%value.time_steps()+1,"value":weather(v)})).collect::<Vec<_>>();
    json!({"allocated":value.is_allocated(),"time_steps":value.time_steps(),"hours":value.hours(),"ordering":"hour-major,timestep-minor","slots":rows})
}
fn stream(v: RawEpwStreamState, path: Option<&Path>) -> Value {
    json!({"file_path":path.map(|p|p.to_string_lossy().into_owned()).unwrap_or_default(),"is_open":v.is_open,"good":v.good,"eof":v.eof,"fail":v.fail,"bad":v.bad,"position_byte":v.position_byte,"position_available":v.position_byte.is_some()})
}
pub(super) fn snapshot(
    state: &WeatherDayState,
    cursor: Option<&RawEpwCursorOwner>,
    file: Option<&Path>,
) -> Value {
    let mut missing = weather(&state.missing_values.base);
    for (key, value) in [
        ("Visibility", state.missing_values.visibility),
        ("Ceiling", state.missing_values.ceiling),
        ("AerOptDepth", state.missing_values.aer_opt_depth),
        ("SnowDepth", state.missing_values.snow_depth),
    ] {
        missing[key] = raw_dto::scalar(value);
    }
    missing["DaysLastSnow"] = json!(state.missing_values.days_last_snow);
    let mut header = cursor.map(|c| c.header().clone()).unwrap_or_default();
    if cursor.is_none() {
        // Same source member viewed through its header DTO, not an unrelated zero counter.
        header.weather_code_missed_count = state.missed_counts.weath_codes;
    }
    let status = cursor
        .map(|c| c.stream_state())
        .unwrap_or_else(|| RawEpwInput::new_unopened(Vec::new()).snapshot());
    json!({"global":global(&state.global),"environment":environment(&state.environment),"weather":owner(&state.weather),
        "today_variables":day(&state.today_variables),"tomorrow_variables":day(&state.tomorrow_variables),
        "today_values":slots(&state.today_values),"tomorrow_values":slots(&state.tomorrow_values),
        "last_hour":weather(&state.last_hour),"next_hour":weather(&state.next_hour),"missing_values":missing,
        "missed_counts":counts(&state.missed_counts),"out_of_range_counts":counts(&state.out_of_range_counts),
        "header":raw_dto::header(&header),"stream":stream(status,file),
        "cursor_observations":cursor.map(|c|json!({"byte_cursor":c.byte_cursor(),"line_read_count":c.line_reads(),"interpret_count":c.interpret_count(),"last_raw_arguments":raw_dto::raw(c.last_arguments())})),
        "native_internal_record_index_observed":false,"native_source_local_variables_observed":false,
        "snapshot_scope":"selected typed Rust owners; no whole original constructor or native private state claim",
        "unowned_native_fields":["EnvironmentData records","Interpolation arrays","SolarInterpolation arrays","DSTIndex/SpecialDayTypes/WeekDayTypes source-only arrays"],
        "processed_weather_numerical_fields_paired":false})
}
