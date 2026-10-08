// These regressions check runtime operand ownership and compatibility; they are
// not an independent original-engine comparison or a weather-physics gate.
use crate::weather::WeatherTimestepSeries;
use crate::weather::day::lifecycle_tests::{DECOY_BYTES, configuration, site};
use crate::weather::day::{
    ProductionSolarMetadata, ProductionWeatherContext, ProductionWeatherTimestepSeries, WeatherVars,
};

#[test]
fn owned_today_drives_solar_rain_and_sky_with_no_legacy_record_array()
-> Result<(), Box<dyn std::error::Error>> {
    let mut typed = cube_model_with_outward_horizontal_surfaces();
    let roof = typed
        .surfaces
        .iter_mut()
        .find(|surface| surface.surface_type == SurfaceType::Roof)
        .ok_or("missing roof")?;
    roof.sun_exposure = SunExposure::SunExposed;
    roof.wind_exposure = WindExposure::WindExposed;
    let records = parse_epw_records(std::str::from_utf8(DECOY_BYTES)?)?;
    let legacy = WeatherTimestepSeries::from_records(
        &records[..24],
        4,
        FirstHourInterpolationStartingValues::Hour24,
    );
    let mut sample = legacy.sample_for(11, 4).copied().ok_or("missing sample")?;
    sample.direct_normal_radiation_w_per_m2 = 0.0;
    sample.diffuse_horizontal_radiation_w_per_m2 = 0.0;
    sample.liquid_precipitation_depth_mm = 0.0;
    let (sin_declination, cos_declination, equation_of_time_hours) =
        energyplus_daily_solar_coefficients(181);
    let current = ProductionWeatherContext {
        record: records[11],
        sample,
        weather: WeatherVars {
            beam_solar_rad: 800.0,
            dif_solar_rad: 120.0,
            is_rain: true,
            sky_temp: -60.0,
            ..WeatherVars::default()
        },
        solar: ProductionSolarMetadata {
            sin_declination,
            cos_declination,
            equation_of_time_hours,
        },
        local_hour: 12.0,
    };
    let context = HeatBalanceWeatherContext {
        records: &[],
        sample: None,
        owned: Some(current),
        record_index: 11,
        zone_steps_per_hour: 4,
        zone_timestep: Some(4),
        first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
    };
    let roof = typed
        .surfaces
        .iter()
        .find(|surface| surface.surface_type == SurfaceType::Roof)
        .ok_or("missing roof")?;
    let tilt_rad =
        crate::geometry::surface_tilt_deg(roof.surface_type, &roof.vertices).to_radians();
    assert!(surface_sky_view_factor(roof, tilt_rad) > 0.0);
    assert_eq!(energyplus_exterior_wet_context_fraction(context, roof), 1.0);
    let incident = crate::heat_balance::solar::surface_incident_solar_radiation_for_current_weather_context_w_per_m2(
        roof, &site(), context);
    assert!(incident > 0.0);
    let dark = ProductionWeatherContext {
        weather: WeatherVars {
            beam_solar_rad: 0.0,
            dif_solar_rad: 0.0,
            ..current.weather
        },
        ..current
    };
    let dark_context = HeatBalanceWeatherContext {
        owned: Some(dark),
        ..context
    };
    assert_eq!(crate::heat_balance::solar::surface_incident_solar_radiation_for_current_weather_context_w_per_m2(
        roof, &site(), dark_context), 0.0);
    let model = SimulationModel::from_typed(typed);
    let state = initialize_heat_balance_state(&model, 23.0)?;
    let roof_state = state
        .surfaces
        .iter()
        .find(|surface| surface.surface_type == SurfaceType::Roof)
        .ok_or("missing roof state")?;
    let quick = Some(QuickOutsideConductionContext {
        reference_air_temperature_c: 23.0,
        inside_convection_coefficient_w_per_m2_k: 3.0,
        net_inside_source_w_per_m2: 0.0,
        exterior_coefficient_surface_temperature_c: Some(23.0),
        use_doe2_outside_convection: true,
    });
    let cold = crate::heat_balance::surface_balance::exterior_surface_boundary_balance(
        &model.typed,
        roof_state,
        sample.dry_bulb_c,
        23.0,
        Some(context),
        quick,
        true,
    );
    let warm = ProductionWeatherContext {
        weather: WeatherVars {
            sky_temp: 20.0,
            ..current.weather
        },
        ..current
    };
    let warm = crate::heat_balance::surface_balance::exterior_surface_boundary_balance(
        &model.typed,
        roof_state,
        sample.dry_bulb_c,
        23.0,
        Some(HeatBalanceWeatherContext {
            owned: Some(warm),
            ..context
        }),
        quick,
        true,
    );
    assert_ne!(
        cold.outside_balance_diagnostics
            .equivalent_radiant_temperature_c
            .to_bits(),
        warm.outside_balance_diagnostics
            .equivalent_radiant_temperature_c
            .to_bits()
    );
    Ok(())
}

#[test]
fn a_live_weather_entry_preserves_legacy_results_and_raw_initial_ctf_seed()
-> Result<(), Box<dyn std::error::Error>> {
    let config = configuration(6, 30, 6, 30, FirstHourInterpolationStartingValues::Hour24)?;
    let mut typed = cube_model();
    typed.timestep.number_of_timesteps_per_hour = 4;
    typed.run_periods = vec![config.run_period.clone()];
    typed.site = Some(config.site.clone());
    let model = SimulationModel::from_typed(typed);
    let records = parse_epw_records(std::str::from_utf8(DECOY_BYTES)?)?;
    let options = HeatBalanceSimulationOptions::hourly_samples(24);
    let legacy = crate::psychrometrics::with_fresh_psychrometric_state(|| {
        let series = WeatherTimestepSeries::from_records(
            &records[..24],
            4,
            FirstHourInterpolationStartingValues::Hour24,
        );
        super::simulate_heat_balance_zone_air_temperatures_with_weather_series(
            &model, &series, options,
        )
    })?;
    let live = ProductionWeatherTimestepSeries::from_bytes(DECOY_BYTES.to_vec(), config)?;
    let production = crate::psychrometrics::with_fresh_psychrometric_state(|| {
        super::simulate_heat_balance_zone_air_temperatures_with_production_weather(
            &model, &live, options,
        )
    })?;
    assert_eq!(legacy.results, production.results);
    assert_eq!(
        legacy.summary.run_period_initial_ctf_history_slots,
        production.summary.run_period_initial_ctf_history_slots
    );
    assert_eq!(
        legacy.summary.run_period_initial_zone_air_states,
        production.summary.run_period_initial_zone_air_states
    );
    assert_eq!(production.summary.run_period_timestep_count, 24 * 4);
    assert!(!production.summary.warmup.enabled);
    assert!(live.snapshot().state.global.end_envrn_flag);
    Ok(())
}
