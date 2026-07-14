use super::*;
use ep_model::{
    FirstHourInterpolationStartingValues, NormalizedName, RunPeriod, RunPeriodId, TypedModel,
};

#[test]
fn ideal_loads_timestep_context_uses_runtime_time_axis_nominal_values() {
    let mut model = TypedModel::default();
    model.timestep.number_of_timesteps_per_hour = 4;

    let context = ideal_loads_timestep_context(&model).expect("time axis should be valid");

    assert_eq!(context.zone_timestep_seconds, 900.0);
    assert_eq!(context.nominal_system_timestep_seconds, 900.0);
    assert_eq!(context.nominal_system_timestep_substeps, 1.0);
    assert_eq!(context.source, "ep_runtime::TimeAxis");
    assert!(!context.adaptive_system_timestep_claim);
}

#[test]
fn ideal_loads_timestep_context_does_not_validate_unrelated_run_period_dates() {
    let mut model = TypedModel::default();
    model.timestep.number_of_timesteps_per_hour = 4;
    model.run_periods.push(RunPeriod {
        id: RunPeriodId(0),
        name: NormalizedName::new("Yearless Winter"),
        begin_month: 11,
        begin_day_of_month: 1,
        begin_year: None,
        end_month: 3,
        end_day_of_month: 31,
        end_year: None,
        day_of_week_for_start_day: None,
        first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        use_weather_file_holidays_and_special_days: true,
        use_weather_file_daylight_saving_period: true,
        apply_weekend_holiday_rule: true,
        use_weather_file_rain_indicators: true,
        use_weather_file_snow_indicators: true,
        treat_weather_as_actual: false,
    });

    let context = ideal_loads_timestep_context(&model).expect("timestep profile should be valid");

    assert_eq!(context.zone_timestep_seconds, 900.0);
    assert_eq!(context.nominal_system_timestep_seconds, 900.0);
}

#[test]
fn ideal_loads_sample_timestep_uses_valid_duration_and_timestamp_precision_normalization() {
    assert_eq!(
        ideal_loads_sample_timestep_seconds(Some("env=RUN PERIOD;start=0;end=15"), 900.0),
        900.0
    );
    assert_eq!(
        ideal_loads_sample_timestep_seconds(Some("ENV=RUN PERIOD; START=0; END=10"), 900.0),
        600.0
    );
    assert_eq!(
        ideal_loads_sample_timestep_hours(Some("start=0;end=10"), 0.25),
        1.0 / 6.0
    );
    assert_eq!(
        ideal_loads_sample_timestep_seconds(Some("start=0;end=1.88"), 900.0),
        112.5
    );
}

#[test]
fn ideal_loads_sample_timestep_falls_back_for_missing_or_invalid_timestamp() {
    for timestamp in [
        None,
        Some("env=RUN PERIOD"),
        Some("start=bad;end=15"),
        Some("start=0;end=NaN"),
        Some("start=0;end=0"),
        Some("start=15;end=0"),
        Some("start=0;end=20"),
    ] {
        assert_eq!(ideal_loads_sample_timestep_seconds(timestamp, 900.0), 900.0);
    }
}
