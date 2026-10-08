//! Output-only serialization of actual live weather calls and thermal operands.

use crate::clk02_trace::{header, projected, raw, sample, scalar, stream};
use crate::{RunConfig, RunError, RunExitCode};
use ep_runtime::weather::day::production_trace::{
    OBSERVATION_LIMIT, WeatherDayProductionTrace, WeatherSessionSnapshot,
};
use ep_runtime::weather::day::{
    CurrentWeatherState, DailyWeatherVariables, WeatherDayState, WeatherGlobalState, WeatherVars,
};
use serde_json::{Value, json};
use std::io::{BufWriter, Write};

fn daily(value: DailyWeatherVariables) -> Value {
    json!({"DayOfYear":value.day_of_year,"DayOfYear_Schedule":value.day_of_year_schedule,
        "Year":value.year,"Month":value.month,"DayOfMonth":value.day_of_month,
        "DayOfWeek":value.day_of_week,"DaylightSavingIndex":value.daylight_saving_index,
        "HolidayIndex":value.holiday_index,"SinSolarDeclinAngle":scalar(value.sin_solar_declin_angle),
        "CosSolarDeclinAngle":scalar(value.cos_solar_declin_angle),"EquationOfTime":scalar(value.equation_of_time)})
}

fn weather(value: WeatherVars) -> Value {
    json!({"IsRain":value.is_rain,"IsSnow":value.is_snow,
        "OutDryBulbTemp":scalar(value.out_dry_bulb_temp),"OutDewPointTemp":scalar(value.out_dew_point_temp),
        "OutBaroPress":scalar(value.out_baro_press),"OutRelHum":scalar(value.out_rel_hum),
        "WindSpeed":scalar(value.wind_speed),"WindDir":scalar(value.wind_dir),"SkyTemp":scalar(value.sky_temp),
        "HorizIRSky":scalar(value.horiz_ir_sky),"BeamSolarRad":scalar(value.beam_solar_rad),
        "DifSolarRad":scalar(value.dif_solar_rad),"Albedo":scalar(value.albedo),
        "WaterPrecip":scalar(value.water_precip),"LiquidPrecip":scalar(value.liquid_precip),
        "TotalSkyCover":scalar(value.total_sky_cover),"OpaqueSkyCover":scalar(value.opaque_sky_cover)})
}

// Observe the actual current owner; selecting/deriving weather is never done here.
fn current_weather(value: CurrentWeatherState) -> Value {
    json!({"IsRain":value.is_rain,
        "OutDryBulbTemp":scalar(value.out_dry_bulb_temp),"OutDewPointTemp":scalar(value.out_dew_point_temp),
        "OutBaroPress":scalar(value.out_baro_press),"OutRelHum":scalar(value.out_rel_hum),
        "OutRelHumValue":scalar(value.out_rel_hum_value),"OutHumRat":scalar(value.out_hum_rat),
        "OutWetBulbTemp":scalar(value.out_wet_bulb_temp),"WindSpeed":scalar(value.wind_speed),
        "WindDir":scalar(value.wind_dir),"LiquidPrecipitation":scalar(value.liquid_precipitation),
        "OutEnthalpy":scalar(value.out_enthalpy),"OutAirDensity":scalar(value.out_air_density)})
}

fn current_observation(state: &WeatherDayState) -> Value {
    let global = &state.global;
    let weather_owner = &state.weather;
    let selected = usize::try_from(global.hour_of_day).ok().and_then(|hour| {
        usize::try_from(global.time_step).ok().and_then(|step| {
            state.today_values.hour(hour).ok().and_then(|values| {
                step.checked_sub(1)
                    .and_then(|index| values.get(index))
                    .copied()
                    .map(|value| json!({"hour":hour,"time_step":step,"value":weather(value)}))
            })
        })
    });
    json!({"current_non_solar_weather":current_weather(state.environment.current_weather),
        "selected_today_slot":selected,
        "weights":{"WeightNow":scalar(global.weight_now),"WeightPreviousHour":scalar(global.weight_previous_hour)},
        "clock":{"CurrentTime":scalar(global.current_time),"SimTimeSteps":global.sim_time_steps,
            "TimeStepZoneSec":scalar(global.time_step_zone_sec),"MinutesInTimeStep":global.minutes_in_time_step},
        "selected_weather_control":{"NextHour":weather_owner.next_hour,"RptIsRain":weather_owner.rpt_is_rain,
            "UseRainValues":weather_owner.use_rain_values,"UseSnowValues":weather_owner.use_snow_values},
        "native_internal_record_index_observed":false,"error_text_parity_claimed":false,
        "observation_scope":"actual owned non-solar current state and selected Today; EMS/ground/water/sky/solar/daylight unpaired"})
}

fn global(value: &WeatherGlobalState) -> Value {
    json!({"BeginSimFlag":value.begin_sim_flag,"BeginEnvrnFlag":value.begin_envrn_flag,
        "BeginDayFlag":value.begin_day_flag,"BeginHourFlag":value.begin_hour_flag,
        "BeginTimeStepFlag":value.begin_time_step_flag,"EndDayFlag":value.end_day_flag,
        "EndHourFlag":value.end_hour_flag,"EndEnvrnFlag":value.end_envrn_flag,
        "EndDesignDayEnvrnsFlag":value.end_design_day_envrns_flag,"WarmupFlag":value.warmup_flag,
        "DayOfSim":value.day_of_sim,"CalendarYear":value.calendar_year,"PreviousHour":value.previous_hour,
        "HourOfDay":value.hour_of_day,"NumOfDayInEnvrn":value.num_of_day_in_envrn,
        "TimeStepsInHour":value.time_steps_in_hour,"TimeStep":value.time_step,"KindOfSim":value.kind_of_sim,
        "DoWeathSim":value.do_weath_sim,"DoDesDaySim":value.do_des_day_sim,
        "DoOutputReporting":value.do_output_reporting,"TimeStepZone":scalar(value.time_step_zone),
        "DayOfSimChr":value.day_of_sim_chr,"CalendarYearChr":value.calendar_year_chr})
}

fn snapshot(value: &WeatherSessionSnapshot) -> Value {
    let state = &value.state;
    let env = &state.environment;
    let owner = &state.weather;
    let slots = |day: &ep_runtime::weather::day::WeatherDayValues| {
        json!({
        "allocated":day.is_allocated(),"time_steps":day.time_steps(),"hours":day.hours(),
        "slots":day.slots().iter().copied().map(weather).collect::<Vec<_>>()})
    };
    let counts = |counts: &ep_runtime::weather::day::WeatherVarCounts| {
        json!({
        "declaration_order_integer_values":counts.integer_values()})
    };
    json!({"global":global(&state.global),"environment":{
            "DayOfYear":env.day_of_year,"DayOfYear_Schedule":env.day_of_year_schedule,
            "Year":env.year,"Month":env.month,"DayOfMonth":env.day_of_month,"DayOfWeek":env.day_of_week,
            "HolidayIndex":env.holiday_index,"DSTIndicator":env.dst_indicator,"YearTomorrow":env.year_tomorrow,
            "MonthTomorrow":env.month_tomorrow,"DayOfMonthTomorrow":env.day_of_month_tomorrow,
            "DayOfWeekTomorrow":env.day_of_week_tomorrow,"HolidayIndexTomorrow":env.holiday_index_tomorrow,
            "TotDesDays":env.tot_des_days,"CurEnvirNum":env.cur_envir_num,
            "RunPeriodStartDayOfWeek":env.run_period_start_day_of_week,
            "SinSolarDeclinAngle":scalar(env.sin_solar_declin_angle),"CosSolarDeclinAngle":scalar(env.cos_solar_declin_angle),
            "EquationOfTime":scalar(env.equation_of_time),"Latitude":scalar(env.latitude),"Longitude":scalar(env.longitude),
            "Elevation":scalar(env.elevation),"StdBaroPress":scalar(env.std_baro_press),
            "EnvironmentName":env.environment_name,"EndMonthFlag":env.end_month_flag,"EndYearFlag":env.end_year_flag},
        "weather":{"Envrn":owner.envrn,"NumOfEnvrn":owner.num_of_envrn,"TotRunPers":owner.tot_run_pers,
            "TotRunDesPers":owner.tot_run_des_pers,"NumDataPeriods":owner.num_data_periods,
            "NumIntervalsPerHour":owner.num_intervals_per_hour,"NumSpecialDays":owner.num_special_days,
            "LeapYearAdd":owner.leap_year_add,"RptDayType":owner.rpt_day_type,"CurDayOfWeek":owner.cur_day_of_week,
            "curSimDayForEndOfRunPeriod":owner.cur_sim_day_for_end_of_run_period,
            "GetBranchInputOneTimeFlag":owner.get_branch_input_one_time_flag,
            "GetEnvironmentFirstCall":owner.get_environment_first_call,"FirstCall":owner.first_call,
            "WaterMainsParameterReport":owner.water_mains_parameter_report,"LastHourSet":owner.last_hour_set,
            "WeatherFileExists":owner.weather_file_exists,"DatesShouldBeReset":owner.dates_should_be_reset,
            "StartDatesCycleShouldBeReset":owner.start_dates_cycle_should_be_reset,
            "Jan1DatesShouldBeReset":owner.jan1_dates_should_be_reset,"RPReadAllWeatherData":owner.rp_read_all_weather_data,
            "UseDaylightSaving":owner.use_daylight_saving,"UseSpecialDays":owner.use_special_days,
            "DaylightSavingIsActive":owner.daylight_saving_is_active,
            "ReadEPlusWeatherCurTime":scalar(owner.read_e_plus_weather_cur_time),
            "TimeStepFraction":scalar(owner.time_step_fraction),"IsRainThreshold":scalar(owner.is_rain_threshold)},
        "today_variables":daily(state.today_variables),"tomorrow_variables":daily(state.tomorrow_variables),
        "today_values":slots(&state.today_values),"tomorrow_values":slots(&state.tomorrow_values),
        "last_hour":weather(state.last_hour),"next_hour":weather(state.next_hour),
        "missing_values":{"base":weather(state.missing_values.base),
            "extra_reals":state.missing_values.extra_real_values().into_iter().map(scalar).collect::<Vec<_>>(),
            "DaysLastSnow":state.missing_values.days_last_snow},
        "missed_counts":counts(&state.missed_counts),"out_of_range_counts":counts(&state.out_of_range_counts),
        "header":header(&value.header),"stream":stream(value.stream),"raw_arguments":raw(&value.raw_arguments),
        "actual_cursor_byte":value.cursor_byte,"actual_line_read_count":value.line_read_count,
        "actual_Rust_interpret_count":value.interpret_count,"native_internal_record_index_claimed":false,
        "available":value.available,"errors_found":value.errors_found,
        "print_environment_stamp":value.print_environment_stamp,
        "current_cycle":value.current_cycle,"set_week_days":value.set_week_days,
        "Interpolation_allocated":owner.interpolation.is_some(),
        "Interpolation_length":owner.interpolation.as_ref().map(Vec::len),
        "Interpolation":owner.interpolation.as_ref().map(|values|values.iter().copied().map(scalar).collect::<Vec<_>>()).unwrap_or_default(),
        "SolarInterpolation_allocated":owner.solar_interpolation.is_some(),
        "SolarInterpolation_length":owner.solar_interpolation.as_ref().map(Vec::len),
        "SolarInterpolation":owner.solar_interpolation.as_ref().map(|values|values.iter().copied().map(scalar).collect::<Vec<_>>()).unwrap_or_default(),
        "solar_interpolation_numerical_parity_claimed":false,
        "clk04_observation":current_observation(state),
        "clk05_observation":{
            "solar_controls":{"DisplayWeatherMissingDataWarnings":env.display_weather_missing_data_warnings,
                "IgnoreSolarRadiation":env.ignore_solar_radiation,
                "IgnoreBeamRadiation":env.ignore_beam_radiation,
                "IgnoreDiffuseRadiation":env.ignore_diffuse_radiation},
            "values_copied_from_actual_stored_owner":true,
            "current_solar_night_physics_paired":false}})
}

pub(crate) fn write_trace(
    config: &RunConfig,
    trace: &WeatherDayProductionTrace,
) -> Result<(), RunError> {
    let complete = trace.total_operation_count == trace.operations.len() as u64
        && trace.total_consumer_count == trace.consumers.len() as u64;
    let artifact = json!({"schema":"clk03-weather-day-production-trace.v1",
        "capture_source":"actual-live-Rust-weather-cursor-daily-owner-and-owned-thermal-operands",
        "observer_supplies_inputs":false,"observer_recomputes_weather":false,
        "eager_preview_values_supply_production_weather":false,
        "thread_coverage":"collecting-thread-only","observation_limit_per_series":OBSERVATION_LIMIT,
        "total_operation_count":trace.total_operation_count,"retained_operation_count":trace.operations.len(),
        "total_consumer_count":trace.total_consumer_count,"retained_consumer_count":trace.consumers.len(),
        "complete_on_collecting_thread":complete,
        "truncation_reason":if complete {None} else {Some("observation_limit")},
        "operations_completion_order":trace.operations.iter().map(|row|json!({
            "sequence":row.sequence,"kind":row.kind,"before":snapshot(&row.before),"after":snapshot(&row.after),
            "outcome":{"returned":row.error.is_none(),"source_fatal":row.error.as_ref().is_some_and(|error|error.is_source_fatal()),
                "error":row.error.as_ref().map(ToString::to_string)}})).collect::<Vec<_>>(),
        "consumers":trace.consumers.iter().map(|row|json!({
            "sequence":row.sequence,"completed_operation_count":row.completed_operation_count,
            "caller":{"file":row.caller.file(),"line":row.caller.line(),"column":row.caller.column()},
            "record_index":row.record_index,"timestep":row.timestep,"phase":format!("{:?}",row.phase),
            "caller_state":global(&row.caller_state),"received_raw":raw(&row.raw),
            "actual_input_byte_range":{"start_byte":row.provenance.start_byte,"end_byte":row.provenance.end_byte,
                "line_read_attempt":row.provenance.line_read_attempt},
            "received_hourly_record":projected(&row.context.record),"received_sample":sample(&row.context.sample),
            "received_Today_slot":weather(row.context.weather),"local_hour":scalar(row.context.local_hour),
            "received_current_weather":current_weather(row.context.current_weather),
            "shadowing_metadata":{"sin_declination":scalar(row.context.solar.sin_declination),
                "cos_declination":scalar(row.context.solar.cos_declination),
                "equation_of_time_hours":scalar(row.context.solar.equation_of_time_hours)} })).collect::<Vec<_>>(),
        "additional_observation_scope":"CLK-04 actual current13, source-owned arrays and clock/control state; original CLK-03 schema and fields retained",
        "added_fields_supply_inputs_or_recompute_weather":false,
        "physical_producer_numerical_parity_claimed":false,
        "raw_source_year_equals_civil_calendar_year_claimed":false,
        "consumer_repeated_identities_deduplicated":false,
        "input_sha256_verification":"external recorded launcher required"});
    let path = config
        .output_dir
        .join("clk03-weather-day-production-trace.json");
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
