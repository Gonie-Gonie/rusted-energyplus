//! Portable input-only regressions for actual cursor transport and day hooks.

use super::{ProductionWeatherTimestepSeries, WeatherDayPhase, WeatherEnvironmentConfiguration};
use crate::time_axis::build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps;
use crate::weather::EpwCalendarMetadata;
use ep_model::{
    DayOfWeek, FirstHourInterpolationStartingValues, NormalizedName, RunPeriod, RunPeriodId,
    SiteLocation,
};

pub(crate) const DECOY_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../energyplus_porting_plan/cases/CLK-03/decoy-three-days.epw"
));
const MISSING_BYTES: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../energyplus_porting_plan/cases/CLK-03/missing-target-complete-days.epw"
));

pub(crate) fn site() -> SiteLocation {
    SiteLocation {
        name: NormalizedName::new("CLK03"),
        latitude_deg: 39.74,
        longitude_deg: -105.18,
        time_zone_hours: -7.0,
        elevation_m: 1600.0,
    }
}

pub(crate) fn configuration(
    begin_month: u32,
    begin_day: u32,
    end_month: u32,
    end_day: u32,
    policy: FirstHourInterpolationStartingValues,
) -> Result<WeatherEnvironmentConfiguration, Box<dyn std::error::Error>> {
    let period = RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new("CLK03 Test"),
        begin_month,
        begin_day_of_month: begin_day,
        begin_year: Some(2013),
        end_month,
        end_day_of_month: end_day,
        end_year: Some(2013),
        day_of_week_for_start_day: Some(DayOfWeek::Sunday),
        first_hour_interpolation_starting_values: policy,
        use_weather_file_holidays_and_special_days: false,
        use_weather_file_daylight_saving_period: false,
        apply_weekend_holiday_rule: false,
        use_weather_file_rain_indicators: false,
        use_weather_file_snow_indicators: false,
        treat_weather_as_actual: false,
    };
    let axis = build_hourly_time_axis_for_run_period_with_weather_metadata_and_zone_timesteps(
        &period,
        &EpwCalendarMetadata::default(),
        4,
    )?;
    Ok(WeatherEnvironmentConfiguration::new(
        &axis,
        &period,
        &site(),
        0,
        0,
    )?)
}

fn consume_complete_day(
    series: &ProductionWeatherTimestepSeries,
    first_record: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    for hour_zero in 0..24 {
        for timestep in 1..=series.zone_steps_per_hour() {
            let current = series.current_for(first_record + hour_zero, timestep)?;
            assert_eq!(current.record.hour as usize, hour_zero + 1);
        }
    }
    let completed = series.snapshot();
    assert_eq!(completed.state.global.hour_of_day, 24);
    assert_eq!(completed.state.global.time_step, 4);
    Ok(())
}

#[test]
fn warmup_hooks_copy_current_first_day_without_reading_or_prefetching()
-> Result<(), Box<dyn std::error::Error>> {
    let series = ProductionWeatherTimestepSeries::from_bytes(
        DECOY_BYTES.to_vec(),
        configuration(6, 30, 7, 2, FirstHourInterpolationStartingValues::Hour24)?,
    )?;
    series.prepare_initial_phase(WeatherDayPhase::Warmup { day: 1 })?;
    let prepared = series.snapshot();
    series.begin_day(WeatherDayPhase::Warmup { day: 1 })?;
    let first_hook = series.snapshot();
    assert_eq!(first_hook.interpret_count, prepared.interpret_count);
    assert_eq!(first_hook.line_read_count, prepared.line_read_count);
    assert_eq!(first_hook.cursor_byte, prepared.cursor_byte);
    assert_eq!(first_hook.state.global.day_of_sim_chr, "0");
    consume_complete_day(&series, 0)?;
    for day in 2..=3 {
        series.begin_day(WeatherDayPhase::Warmup { day })?;
        let repeated = series.snapshot();
        assert_eq!(repeated.interpret_count, prepared.interpret_count);
        assert_eq!(repeated.line_read_count, prepared.line_read_count);
        assert_eq!(repeated.cursor_byte, prepared.cursor_byte);
        assert_eq!(repeated.state.today_values, prepared.state.today_values);
        assert_eq!(
            repeated.state.today_variables,
            prepared.state.today_variables
        );
        assert_eq!(repeated.state.global.day_of_sim, day as i32);
        assert_eq!(repeated.state.global.day_of_sim_chr, "0");
        series.current_for(0, 1)?;
        assert_eq!(series.snapshot().state.global.previous_hour, 24);
        consume_complete_day(&series, 0)?;
    }
    series.begin_day(WeatherDayPhase::Run { day: 1 })?;
    let run = series.snapshot();
    assert_eq!(run.interpret_count - prepared.interpret_count, 24);
    assert_eq!(run.state.today_variables.day_of_month, 30);
    assert_eq!(run.state.tomorrow_variables.day_of_month, 1);
    assert!(!run.state.global.warmup_flag);
    assert_eq!(run.state.global.day_of_sim_chr, "1");
    series.current_for(0, 1)?;
    assert_eq!(series.snapshot().state.global.previous_hour, 24);
    Ok(())
}

#[test]
fn later_days_advance_actual_cursor_and_final_day_waits_for_end_environment()
-> Result<(), Box<dyn std::error::Error>> {
    let series = ProductionWeatherTimestepSeries::from_bytes(
        DECOY_BYTES.to_vec(),
        configuration(6, 30, 7, 2, FirstHourInterpolationStartingValues::Hour24)?,
    )?;
    series.prepare_initial_phase(WeatherDayPhase::Run { day: 1 })?;
    let prepared = series.snapshot();
    series.begin_day(WeatherDayPhase::Run { day: 1 })?;
    assert_eq!(series.snapshot().interpret_count, prepared.interpret_count);
    let raw_first = series.current_for(0, 1)?;
    assert_eq!(raw_first.record.year, 1999);
    assert_eq!(series.snapshot().state.global.calendar_year, 2013);
    consume_complete_day(&series, 0)?;
    series.begin_day(WeatherDayPhase::Run { day: 2 })?;
    let second = series.snapshot();
    assert_eq!(second.interpret_count - prepared.interpret_count, 24);
    assert_eq!(second.state.today_variables.day_of_month, 1);
    assert_eq!(second.state.tomorrow_variables.day_of_month, 2);
    assert_eq!(second.state.global.day_of_sim_chr, "2");
    series.current_for(24, 1)?;
    assert_eq!(series.snapshot().state.global.previous_hour, 24);
    consume_complete_day(&series, 24)?;
    series.begin_day(WeatherDayPhase::Run { day: 3 })?;
    let last_day = series.snapshot();
    assert_eq!(last_day.interpret_count, second.interpret_count);
    assert_eq!(last_day.cursor_byte, second.cursor_byte);
    assert_eq!(last_day.state.today_variables.day_of_month, 2);
    assert_eq!(series.current_for(71, 4)?.record.year, 2007);
    series.finish_environment()?;
    let finished = series.snapshot();
    assert_eq!(finished.interpret_count, last_day.interpret_count);
    assert!(finished.cursor_byte < last_day.cursor_byte);
    assert!(finished.stream.good);
    assert!(!finished.stream.eof);
    assert!(finished.state.global.end_envrn_flag);
    Ok(())
}

#[test]
fn initial_seed_is_hour_one_raw_dry_bulb_for_either_interpolation_policy()
-> Result<(), Box<dyn std::error::Error>> {
    let mut seeds = Vec::new();
    let mut samples = Vec::new();
    for policy in [
        FirstHourInterpolationStartingValues::Hour1,
        FirstHourInterpolationStartingValues::Hour24,
    ] {
        let series = ProductionWeatherTimestepSeries::from_bytes(
            DECOY_BYTES.to_vec(),
            configuration(6, 30, 6, 30, policy)?,
        )?;
        series.prepare_initial_phase(WeatherDayPhase::Run { day: 1 })?;
        let current = series.current_for(0, 1)?;
        let seed = series.initial_hourly_dry_bulb_c()?;
        assert_eq!(seed.to_bits(), current.record.dry_bulb_c.to_bits());
        seeds.push(seed);
        samples.push(current.sample.dry_bulb_c);
    }
    assert_eq!(seeds[0].to_bits(), seeds[1].to_bits());
    assert_eq!(seeds[0].to_bits(), samples[0].to_bits());
    assert_ne!(seeds[1].to_bits(), samples[1].to_bits());
    Ok(())
}

#[test]
fn missing_target_retains_actual_failed_stream_and_unwritten_today()
-> Result<(), Box<dyn std::error::Error>> {
    let series = ProductionWeatherTimestepSeries::from_bytes(
        MISSING_BYTES.to_vec(),
        configuration(7, 1, 7, 1, FirstHourInterpolationStartingValues::Hour24)?,
    )?;
    let before = series.snapshot();
    let error = match series.prepare_initial_phase(WeatherDayPhase::Run { day: 1 }) {
        Ok(()) => return Err("missing date unexpectedly admitted".into()),
        Err(error) => error,
    };
    assert!(error.is_source_fatal());
    let after = series.snapshot();
    assert!(after.stream.eof);
    assert!(after.stream.fail);
    assert!(!after.stream.good);
    assert!(after.interpret_count > before.interpret_count);
    assert_eq!(after.state.today_values, before.state.today_values);
    assert_eq!(after.state.today_variables, before.state.today_variables);
    assert!(series.current_for(0, 1).is_err());
    Ok(())
}
