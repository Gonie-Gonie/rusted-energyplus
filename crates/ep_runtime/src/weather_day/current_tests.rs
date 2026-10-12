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
    // Reuse this real session for authored solar branch coverage. The source
    // Today owner is deliberately nonzero at night; masking must not mutate it.
    session.state.today_values = WeatherDayValues::allocated(4)?;
    session.state.environment.sin_latitude = 0.0;
    session.state.environment.cos_latitude = 1.0;
    session.state.environment.time_zone_meridian = 0.0;
    session.state.environment.longitude = 0.0;
    session.state.today_variables.sin_solar_declin_angle = 0.0;
    session.state.today_variables.cos_solar_declin_angle = 1.0;
    session.state.today_variables.equation_of_time = 0.0;
    let slot = super::WeatherVars {
        out_dry_bulb_temp: 20.0,
        out_dew_point_temp: 10.0,
        out_baro_press: 101325.0,
        out_rel_hum: 50.0,
        beam_solar_rad: 800.0,
        dif_solar_rad: 120.0,
        sky_temp: -20.0,
        horiz_ir_sky: 300.0,
        total_sky_cover: 7.0,
        opaque_sky_cover: 6.0,
        ..super::WeatherVars::default()
    };
    for hour in [1_usize, 12] {
        session.state.today_values.hour_mut(hour)?.fill(slot);
    }
    session.state.global.hour_of_day = 12;
    session.state.global.time_step = 4;
    session.set_current_weather()?;
    let noon = session.state.environment.current_weather;
    assert!(noon.sun_is_up);
    assert_eq!(noon.solar_direction_cosines, [0.0, 0.0, 1.0]);
    assert_eq!(session.state.weather.solar_altitude_angle, 90.0);
    assert!(
        session.state.weather.solar_azimuth_angle.is_nan(),
        "ObjexxFCL min(1, NaN) retains the undefined azimuth at zenith"
    );
    assert_eq!(noon.gnd_solar_rad, 184.0);
    assert_eq!((noon.beam_solar_rad, noon.dif_solar_rad), (800.0, 120.0));
    assert_eq!(noon.sky_temp_kelvin, 253.14999999999998);
    assert_eq!(session.state.weather.horizontal_infrared_sky, 300.0);
    assert_eq!(
        (noon.total_cloud_cover, noon.opaque_cloud_cover),
        (7.0, 6.0)
    );
    session.state.global.hour_of_day = 1;
    session.state.global.time_step = 1;
    session.set_current_weather()?;
    let night = session.state.environment.current_weather;
    assert!(!night.sun_is_up);
    assert!(night.sun_is_up_previous_timestep);
    assert!(night.solar_direction_cosines[2] < 0.0);
    assert_eq!(&night.solar_direction_cosines[..2], &[0.0, 0.0]);
    assert_eq!(
        (
            night.beam_solar_rad,
            night.dif_solar_rad,
            night.gnd_solar_rad
        ),
        (0.0, 0.0, 0.0)
    );
    assert_eq!(session.state.today_values.hour(1)?[0], slot);
    session.set_current_weather()?;
    assert!(
        !session
            .state
            .environment
            .current_weather
            .sun_is_up_previous_timestep,
        "a second genuine call at the same clock still advances the retained previous flag"
    );
    session.state.global.hour_of_day = 12;
    session.state.global.time_step = 4;
    session.state.weather.use_snow_values = true;
    session.state.weather.snow_ground_reflectance_modifier = 10.0;
    session
        .state
        .weather
        .snow_ground_reflectance_for_daylighting_modifier = 0.5;
    session.state.today_values.hour_mut(12)?[3].is_snow = true;
    session.set_current_weather()?;
    let snow = session.state.environment.current_weather;
    assert_eq!(
        (snow.gnd_reflectance, snow.gnd_reflectance_for_daylighting),
        (1.0, 0.1)
    );
    assert_eq!(snow.gnd_solar_rad, 920.0);
    assert_eq!(session.state.weather.rpt_is_snow, 1);
    session.state.today_values.hour_mut(12)?[3].is_snow = false;
    session.set_current_weather()?;
    assert_eq!(
        session.state.environment.current_weather.gnd_reflectance, 0.2,
        "monthly reflectance resets before a new snow decision"
    );
    assert_eq!(session.state.weather.rpt_is_snow, 0);
    session.state.global.time_steps_in_hour = 1;
    session.state.global.time_step = 1;
    session.state.weather.time_step_fraction = 1.0;
    session.setup_interpolation_values()?;
    session.state.today_values = WeatherDayValues::allocated(1)?;
    session.state.today_values.hour_mut(12)?[0] = slot;
    session.set_current_weather()?;
    assert_eq!(
        session.state.weather.hour_angle, 7.5,
        "one timestep uses the real -0.5 hour offset"
    );
    assert!(session.state.environment.current_weather.sun_is_up);
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
    crate::psychrometrics::with_fresh_psychrometric_state(|| {
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
        assert_eq!(
            trace.consumers[0].completed_operation_count,
            trace.consumers[1].completed_operation_count
        );
        assert!(
            trace.consumers[2].completed_operation_count
                > trace.consumers[1].completed_operation_count
        );
        for consumer in &trace.consumers {
            let completed = trace
                .operations
                .iter()
                .find(|row| row.sequence == consumer.completed_operation_count)
                .unwrap();
            assert_eq!(completed.kind, "SetCurrentWeather");
            assert_eq!(
                completed.after.state.environment.current_weather,
                consumer.context.current_weather
            );
        }
        Ok(())
    })
}
