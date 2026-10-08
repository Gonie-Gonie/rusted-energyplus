//! Regressions for actual storage admission and repeated thermal consumers.

use super::lifecycle_tests::{DECOY_BYTES, configuration};
use super::{
    ProductionWeatherTimestepSeries, WeatherDayPhase, WeatherDayState, WeatherDayValues,
    WeatherSession,
};
use ep_model::FirstHourInterpolationStartingValues;

#[test]
fn current_rejects_clock_that_exceeds_actual_today_storage()
-> Result<(), Box<dyn std::error::Error>> {
    let mut session = WeatherSession::new(
        DECOY_BYTES.to_vec(),
        configuration(6, 30, 7, 2, FirstHourInterpolationStartingValues::Hour1)?,
    )?;
    session.set_phase(WeatherDayPhase::Run { day: 1 }, true);
    session.get_next_environment()?;
    session.initialize_weather()?;
    session.state.today_values = WeatherDayValues::allocated(1)?;
    session.state.global.hour_of_day = 1;
    session.state.global.time_step = 4;
    let error = session.set_current_weather().unwrap_err();
    assert!(!error.is_source_fatal());
    assert!(error.to_string().contains("current Today slot unavailable"));
    Ok(())
}

#[test]
fn setup_rejects_zero_steps_without_allocating_owners() {
    let mut state = WeatherDayState::default();
    assert!(state.setup_interpolation_values().is_err());
    assert!(state.weather.interpolation.is_none());
    assert!(state.weather.solar_interpolation.is_none());
}

#[test]
fn repeated_initial_consumers_share_current_without_deduplicating_hooks()
-> Result<(), Box<dyn std::error::Error>> {
    let series = ProductionWeatherTimestepSeries::from_bytes(
        DECOY_BYTES.to_vec(),
        configuration(6, 30, 7, 2, FirstHourInterpolationStartingValues::Hour24)?,
    )?;
    let (result, trace) = super::production_trace::capture(true, || {
        series.prepare_initial_phase(WeatherDayPhase::Warmup { day: 1 })?;
        let seed = series.current_for(0, 1)?;
        series.begin_day(WeatherDayPhase::Warmup { day: 1 })?;
        let first = series.current_for(0, 1)?;
        assert_eq!(seed.current_weather, first.current_weather);
        assert_eq!(seed.sample, first.sample);
        assert!(!first.current_weather.is_rain);
        assert_eq!(
            first.current_weather.out_dry_bulb_temp.to_bits(),
            first.sample.dry_bulb_c.to_bits()
        );
        assert_eq!(
            first.current_weather.out_hum_rat.to_bits(),
            first.sample.outdoor_humidity_ratio.to_bits()
        );
        series.begin_day(WeatherDayPhase::Warmup { day: 2 })?;
        series.current_for(0, 1)?;
        Ok::<_, super::WeatherDayError>(())
    });
    result?;
    let trace = trace.unwrap();
    assert_eq!(trace.total_consumer_count, 3);
    assert_eq!(trace.consumers.len(), 3);
    assert_eq!(
        trace
            .operations
            .iter()
            .filter(|row| row.kind == "SetCurrentWeather")
            .count(),
        2
    );
    assert_eq!(trace.consumers[0].phase, WeatherDayPhase::Warmup { day: 1 });
    assert_eq!(trace.consumers[1].phase, WeatherDayPhase::Warmup { day: 1 });
    assert_eq!(trace.consumers[2].phase, WeatherDayPhase::Warmup { day: 2 });
    Ok(())
}
