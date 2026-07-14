use super::*;
use crate::time_axis::build_hourly_time_axis_for_run_period_with_weather_metadata;
use crate::weather::{
    EpwCalendarMetadata, EpwDataPeriodDate, EpwDataPeriods, EpwWeatherFile,
    precompute_weather_timestep_series,
};
use ep_model::{
    DayOfWeek, FirstHourInterpolationStartingValues, NormalizedName, RunPeriod, RunPeriodId,
};

fn run_period(
    name: &str,
    begin_month: u32,
    begin_day: u32,
    end_month: u32,
    end_day: u32,
) -> RunPeriod {
    RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new(name),
        begin_month,
        begin_day_of_month: begin_day,
        begin_year: Some(2016),
        end_month,
        end_day_of_month: end_day,
        end_year: Some(2016),
        day_of_week_for_start_day: Some(DayOfWeek::Sunday),
        first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        use_weather_file_holidays_and_special_days: false,
        use_weather_file_daylight_saving_period: false,
        apply_weekend_holiday_rule: false,
        use_weather_file_rain_indicators: false,
        use_weather_file_snow_indicators: false,
        treat_weather_as_actual: false,
    }
}

fn data_period(start_month: u32, start_day: u32, end_month: u32, end_day: u32) -> EpwDataPeriod {
    EpwDataPeriod {
        name: "DATA".to_string(),
        start_day_of_week: DayOfWeek::Sunday,
        start_date: EpwDataPeriodDate {
            year: None,
            month: start_month,
            day: start_day,
        },
        end_date: EpwDataPeriodDate {
            year: None,
            month: end_month,
            day: end_day,
        },
    }
}

fn weather_file(
    leap_year_observed: bool,
    periods: Vec<EpwDataPeriod>,
    records: Vec<EpwRecord>,
) -> EpwWeatherFile {
    EpwWeatherFile {
        calendar_metadata: EpwCalendarMetadata { leap_year_observed },
        data_periods: EpwDataPeriods {
            records_per_hour: 1,
            periods,
        },
        records,
    }
}

fn source_day(year: u32, month: u32, day: u32, dry_bulb_base: f64) -> Vec<EpwRecord> {
    (1..=24)
        .map(|hour| EpwRecord {
            year,
            month,
            day,
            hour,
            minute: 60,
            dry_bulb_c: dry_bulb_base + f64::from(hour),
            dew_point_c: 0.0,
            relative_humidity_percent: 50.0,
            atmospheric_pressure_pa: 101_325.0,
            horizontal_infrared_radiation_wh_per_m2: 0.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 180.0,
            wind_speed_m_per_s: 2.0,
            liquid_precipitation_depth_mm: 0.0,
        })
        .collect()
}

fn axis_for(
    run_period: &RunPeriod,
    leap_year_observed: bool,
) -> Result<TimeAxis, Box<dyn std::error::Error>> {
    Ok(build_hourly_time_axis_for_run_period_with_weather_metadata(
        run_period,
        &EpwCalendarMetadata { leap_year_observed },
    )?)
}

#[test]
fn selects_nonactual_start_offset_and_materializes_day_buffers()
-> Result<(), Box<dyn std::error::Error>> {
    let mut records = source_day(1999, 6, 30, 0.0);
    records.extend(source_day(2004, 7, 1, 100.0));
    records.extend(source_day(2007, 7, 2, 200.0));
    let weather = weather_file(false, vec![data_period(6, 30, 7, 2)], records);
    let run_period = run_period("Offset Start", 7, 1, 7, 2);
    let axis = axis_for(&run_period, false)?;

    let selected = select_epw_environment_weather(&weather, &axis)?;

    assert_eq!(selected.source_start_record_index, 24);
    assert_eq!(selected.initial_tomorrow_source_record_start, 24);
    assert_eq!(selected.hourly_records().len(), 48);
    assert_eq!(selected.selected_source_record_indices[0], 24);
    assert_eq!(selected.selected_source_record_indices[47], 71);
    assert_eq!(selected.hourly_records()[0].year, 2004);
    assert_eq!(selected.hourly_records()[0].dry_bulb_c, 101.0);
    assert_eq!(
        selected.day_buffer_transitions,
        vec![
            EpwWeatherDayBufferTransition {
                day_of_sim: 1,
                today_source_record_start: 24,
                tomorrow_source_record_start: 48,
                prefetched_next_day: true,
            },
            EpwWeatherDayBufferTransition {
                day_of_sim: 2,
                today_source_record_start: 48,
                tomorrow_source_record_start: 48,
                prefetched_next_day: false,
            },
        ]
    );

    Ok(())
}

#[test]
fn selected_stream_resets_first_hour_seed_and_carries_previous_day_hour_24()
-> Result<(), Box<dyn std::error::Error>> {
    let mut records = source_day(2016, 6, 30, -1_000.0);
    records.extend(source_day(2016, 7, 1, 100.0));
    records.extend(source_day(2016, 7, 2, 200.0));
    let weather = weather_file(false, vec![data_period(6, 30, 7, 2)], records);
    let run_period = run_period("Interpolation Handoff", 7, 1, 7, 2);
    let axis = axis_for(&run_period, false)?;
    let selected = select_epw_environment_weather(&weather, &axis)?;

    let hour_24_seed = precompute_weather_timestep_series(
        selected.hourly_records(),
        4,
        FirstHourInterpolationStartingValues::Hour24,
    );
    assert_eq!(hour_24_seed.timestep_samples()[0].dry_bulb_c, 118.25);
    assert_eq!(hour_24_seed.timestep_samples()[3].dry_bulb_c, 101.0);
    assert_eq!(hour_24_seed.timestep_samples()[24 * 4].dry_bulb_c, 143.25);
    assert_eq!(
        hour_24_seed.timestep_samples()[24 * 4 + 3].dry_bulb_c,
        201.0
    );

    let hour_1_seed = precompute_weather_timestep_series(
        selected.hourly_records(),
        4,
        FirstHourInterpolationStartingValues::Hour1,
    );
    assert_eq!(hour_1_seed.timestep_samples()[0].dry_bulb_c, 101.0);

    Ok(())
}

#[test]
fn skips_raw_february_29_only_after_literal_start_search() -> Result<(), Box<dyn std::error::Error>>
{
    let mut records = source_day(2016, 2, 28, -100.0);
    records.extend(source_day(2016, 2, 29, 2_900.0));
    records.extend(source_day(2016, 3, 1, 300.0));
    let weather = weather_file(false, vec![data_period(2, 28, 3, 1)], records);

    let through_leap_day = run_period("Through Leap Day", 2, 28, 3, 1);
    let through_axis = axis_for(&through_leap_day, false)?;
    let through = select_epw_environment_weather(&weather, &through_axis)?;
    assert_eq!(through.source_start_record_index, 0);
    assert_eq!(through.skipped_february_29_source_record_starts, vec![24]);
    assert_eq!(through.hourly_records().len(), 48);
    assert_eq!(through.selected_source_record_indices[24], 48);
    assert!(
        through
            .hourly_records()
            .iter()
            .all(|record| !(record.month == 2 && record.day == 29))
    );

    let leap_day_only = run_period("Leap Day Only", 2, 29, 2, 29);
    let endpoint_axis = axis_for(&leap_day_only, false)?;
    let endpoint = select_epw_environment_weather(&weather, &endpoint_axis)?;
    assert_eq!(endpoint.source_start_record_index, 24);
    assert_eq!(endpoint.initial_tomorrow_source_record_start, 48);
    assert_eq!(endpoint.skipped_february_29_source_record_starts, vec![24]);
    assert_eq!(endpoint.hourly_records().len(), 24);
    assert_eq!(
        (
            endpoint.hourly_records()[0].month,
            endpoint.hourly_records()[0].day
        ),
        (3, 1)
    );
    assert_eq!(
        endpoint.day_buffer_transitions[0],
        EpwWeatherDayBufferTransition {
            day_of_sim: 1,
            today_source_record_start: 48,
            tomorrow_source_record_start: 48,
            prefetched_next_day: false,
        }
    );

    Ok(())
}

#[test]
fn rejects_missing_literal_february_29_start_record() -> Result<(), Box<dyn std::error::Error>> {
    let weather = weather_file(
        false,
        vec![data_period(2, 29, 3, 1)],
        source_day(2016, 3, 1, 300.0),
    );
    let run_period = run_period("Missing Leap Start", 2, 29, 2, 29);
    let axis = axis_for(&run_period, false)?;

    assert!(matches!(
        select_epw_environment_weather(&weather, &axis),
        Err(EpwEnvironmentWeatherError::StartRecordNotFound {
            month: 2,
            day: 29,
            ..
        })
    ));

    Ok(())
}

#[test]
fn rejects_an_empty_public_weather_axis_without_panicking() -> Result<(), Box<dyn std::error::Error>>
{
    let weather = weather_file(
        false,
        vec![data_period(7, 1, 7, 1)],
        source_day(2016, 7, 1, 0.0),
    );
    let run_period = run_period("Empty Axis", 7, 1, 7, 1);
    let mut axis = axis_for(&run_period, false)?;
    axis.points.clear();
    axis.weather_calendar
        .as_mut()
        .expect("metadata-aware test axis")
        .total_days = 0;

    assert!(matches!(
        select_epw_environment_weather(&weather, &axis),
        Err(EpwEnvironmentWeatherError::EmptyEnvironment { .. })
    ));

    Ok(())
}

#[test]
fn rejects_a_filtered_source_day_cycle_instead_of_looping() -> Result<(), Box<dyn std::error::Error>>
{
    let weather = weather_file(
        false,
        vec![data_period(1, 1, 12, 31)],
        source_day(2016, 2, 29, 0.0),
    );
    let run_period = run_period("Filtered Cycle", 2, 29, 2, 29);
    let axis = axis_for(&run_period, false)?;

    assert!(matches!(
        select_epw_environment_weather(&weather, &axis),
        Err(EpwEnvironmentWeatherError::NoUsableSourceDay {
            day_of_sim: 1,
            scanned_source_days: 1,
        })
    ));

    Ok(())
}

#[test]
fn requires_one_data_period_to_cover_both_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let mut records = source_day(2016, 2, 28, 0.0);
    records.extend(source_day(2016, 3, 1, 100.0));
    let weather = weather_file(
        false,
        vec![data_period(2, 28, 2, 28), data_period(3, 1, 3, 1)],
        records,
    );
    let run_period = run_period("Split Coverage", 2, 28, 3, 1);
    let axis = axis_for(&run_period, false)?;

    assert!(matches!(
        select_epw_environment_weather(&weather, &axis),
        Err(EpwEnvironmentWeatherError::RunPeriodOutsideDataPeriods { .. })
    ));

    Ok(())
}

#[test]
fn rejects_subhourly_epw_rows_and_malformed_source_days() -> Result<(), Box<dyn std::error::Error>>
{
    let run_period = run_period("Malformed Source", 7, 1, 7, 1);
    let axis = axis_for(&run_period, false)?;
    let mut subhourly = weather_file(
        false,
        vec![data_period(7, 1, 7, 1)],
        source_day(2016, 7, 1, 0.0),
    );
    subhourly.data_periods.records_per_hour = 2;
    assert!(matches!(
        select_epw_environment_weather(&subhourly, &axis),
        Err(EpwEnvironmentWeatherError::RecordsPerHourUnsupported {
            records_per_hour: 2
        })
    ));

    let mut malformed = weather_file(
        false,
        vec![data_period(7, 1, 7, 1)],
        source_day(2016, 7, 1, 0.0),
    );
    malformed.records[10].hour = 12;
    assert!(matches!(
        select_epw_environment_weather(&malformed, &axis),
        Err(EpwEnvironmentWeatherError::SourceHourOutOfOrder {
            source_record_index: 10,
            expected_hour: 11,
            actual_hour: 12,
        })
    ));

    Ok(())
}

#[test]
fn last_environment_day_does_not_require_or_prefetch_another_source_day()
-> Result<(), Box<dyn std::error::Error>> {
    let weather = weather_file(
        false,
        vec![data_period(7, 1, 7, 1)],
        source_day(2016, 7, 1, 0.0),
    );
    let one_day = run_period("One Day", 7, 1, 7, 1);
    let one_day_axis = axis_for(&one_day, false)?;

    let selected = select_epw_environment_weather(&weather, &one_day_axis)?;
    assert_eq!(selected.hourly_records().len(), 24);
    assert!(!selected.day_buffer_transitions[0].prefetched_next_day);
    assert_eq!(
        selected.day_buffer_transitions[0].today_source_record_start,
        selected.day_buffer_transitions[0].tomorrow_source_record_start
    );

    let missing_second_day_weather = weather_file(
        false,
        vec![data_period(7, 1, 7, 2)],
        source_day(2016, 7, 1, 0.0),
    );
    let two_days = run_period("Two Days", 7, 1, 7, 2);
    let two_day_axis = axis_for(&two_days, false)?;
    assert!(matches!(
        select_epw_environment_weather(&missing_second_day_weather, &two_day_axis),
        Err(EpwEnvironmentWeatherError::SourceEndedBeforeEnvironment {
            completed_days: 1,
            required_days: 2,
        })
    ));

    Ok(())
}

#[test]
fn full_cycle_rewind_requires_one_complete_data_period() {
    let full_year = data_period(1, 1, 12, 31);
    assert!(data_period_is_full_cycle(&full_year, false));
    assert!(data_period_is_full_cycle(&full_year, true));
    assert!(data_period_is_full_cycle(&data_period(7, 1, 6, 30), false));
    assert!(!data_period_is_full_cycle(&data_period(7, 1, 6, 29), false));
    assert_eq!(advance_source_day(24, 48, true, 2, 3), Ok(0));
    assert!(matches!(
        advance_source_day(24, 48, false, 2, 3),
        Err(EpwEnvironmentWeatherError::SourceEndedBeforeEnvironment { .. })
    ));
}
