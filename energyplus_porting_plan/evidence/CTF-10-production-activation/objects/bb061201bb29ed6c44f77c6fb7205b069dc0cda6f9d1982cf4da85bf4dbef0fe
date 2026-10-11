// These regressions check runtime operand ownership and compatibility; they are
// not an independent original-engine comparison or a weather-physics gate.
use crate::weather::WeatherTimestepSeries;
use crate::weather::day::lifecycle_tests::{DECOY_BYTES, configuration, site};
use crate::weather::day::{
    CurrentWeatherState, ProductionSolarMetadata, ProductionWeatherContext,
    ProductionWeatherTimestepSeries, WeatherVars,
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
        current_weather: CurrentWeatherState {
            is_rain: true,
            ..CurrentWeatherState::default()
        },
        solar: ProductionSolarMetadata {
            sin_declination,
            cos_declination,
            equation_of_time_hours,
        },
        local_hour: 12.0,
        sky_transport_stamp: None,
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
fn a_live_weather_reaches_reports_and_preserves_raw_initial_ctf_seed()
-> Result<(), Box<dyn std::error::Error>> {
    use crate::weather::day::sky_transport_trace::{
        self, SkyTransportKind as Kind, SkyTransportSeries as Series, SkyTransportValues as Values,
    };
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
    let ((production, trace), sky_trace) = sky_transport_trace::capture(true, || {
        crate::weather::day::production_trace::capture(true, || {
            crate::psychrometrics::with_fresh_psychrometric_state(|| {
                super::simulate_heat_balance_zone_air_temperatures_with_production_weather(
                    &model, &live, options,
                )
            })
        })
    });
    let production = production?;
    let trace = trace.ok_or("missing actual weather trace")?;
    let sky_trace = sky_trace.ok_or("missing actual sky transport trace")?;
    // The original raw-weather CTF and initial zone-state invariants remain exact.
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
    assert_eq!(trace.total_operation_count as usize, trace.operations.len());
    assert_eq!(trace.total_consumer_count as usize, trace.consumers.len());
    assert_eq!(
        trace
            .operations
            .iter()
            .filter(|op| op.kind == "SetCurrentWeather")
            .count(),
        96
    );
    assert_eq!(trace.consumers.len(), 97);
    for consumer in &trace.consumers[..2] {
        assert_eq!((consumer.record_index, consumer.timestep), (0, 1));
        assert_eq!(
            consumer.completed_operation_count,
            trace.consumers[0].completed_operation_count
        );
    }
    for consumer in &trace.consumers {
        assert_eq!(
            consumer.phase,
            crate::weather::day::WeatherDayPhase::Run { day: 1 }
        );
        assert!(!consumer.caller_state.warmup_flag);
        let current = trace
            .operations
            .iter()
            .find(|op| op.sequence == consumer.completed_operation_count)
            .ok_or("consumer has no actual completed weather operation")?;
        assert_eq!(current.kind, "SetCurrentWeather");
        assert!(current.error.is_none());
        let slot = current
            .after
            .state
            .today_values
            .hour(consumer.record_index + 1)?
            .get(consumer.timestep as usize - 1)
            .ok_or("actual Today unavailable")?;
        assert_eq!(consumer.context.weather, *slot);
        assert_eq!(
            consumer.context.current_weather,
            current.after.state.environment.current_weather
        );
        assert_eq!(
            consumer
                .context
                .sample
                .horizontal_infrared_radiation_w_per_m2
                .to_bits(),
            slot.horiz_ir_sky.to_bits()
        );
        let stamp = consumer
            .context
            .sky_transport_stamp
            .ok_or("actual context stamp unavailable")?;
        assert_eq!(
            (stamp.record_index, stamp.hour, stamp.timestep),
            (
                consumer.record_index,
                consumer.record_index as i32 + 1,
                consumer.timestep as i32
            )
        );
        assert_eq!(
            stamp.completed_operation_count,
            consumer.completed_operation_count
        );
    }
    let sky = production
        .results
        .find_series("Environment", "Site Sky Temperature")
        .ok_or("missing actual sky output series")?;
    let infrared = production
        .results
        .find_series(
            "Environment",
            "Site Horizontal Infrared Radiation Rate per Area",
        )
        .ok_or("missing actual infrared output series")?;
    assert_eq!((&*sky.units, &*infrared.units), ("C", "W/m2"));
    assert_eq!((sky.values.len(), infrared.values.len()), (24, 24));
    let mut events = sky_trace
        .retained_by_kind
        .iter()
        .flatten()
        .collect::<Vec<_>>();
    events.sort_unstable_by_key(|event| event.sequence);
    assert_eq!(sky_trace.total_event_count as usize, events.len());
    assert_eq!(
        sky_trace.completed_operation_count,
        trace.total_operation_count
    );
    for (total, retained) in sky_trace
        .total_by_kind
        .iter()
        .zip(&sky_trace.retained_by_kind)
    {
        assert_eq!(*total as usize, retained.len());
    }
    let mut contexts = 0;
    let mut samplers = Vec::new();
    let mut accumulators = 0;
    let mut hourly = 0;
    let mut handoffs = [0, 0];
    let mut sums = [[0.0_f64; 2]; 24];
    let mut pushed = [[0.0_f64; 2]; 24];
    for (index, event) in events.iter().enumerate() {
        assert_eq!(event.sequence as usize, index + 1);
        if !matches!(
            event.kind,
            Kind::OwnedContextReceipt
                | Kind::SamplerReturn
                | Kind::ReportAccumulator
                | Kind::HourlyOutput
                | Kind::SeriesHandoff
        ) {
            continue;
        }
        let stamp = event
            .stamp
            .ok_or("actual report operand stamp unavailable")?;
        assert_eq!(
            stamp.phase,
            crate::weather::day::WeatherDayPhase::Run { day: 1 }
        );
        match event.values {
            Values::OwnedContext { today, sample_ir } => {
                let consumer = &trace.consumers[contexts];
                assert_eq!(Some(stamp), consumer.context.sky_transport_stamp);
                let owner = consumer.context.weather;
                assert_eq!(
                    today.map(f64::to_bits),
                    [
                        owner.sky_temp,
                        owner.horiz_ir_sky,
                        owner.total_sky_cover,
                        owner.opaque_sky_cover
                    ]
                    .map(f64::to_bits)
                );
                assert_eq!(
                    sample_ir.to_bits(),
                    consumer
                        .context
                        .sample
                        .horizontal_infrared_radiation_w_per_m2
                        .to_bits()
                );
                contexts += 1;
            }
            Values::Sampler { values, owned } => {
                let consumer = &trace.consumers[samplers.len() + 1];
                assert!(owned);
                assert_eq!(Some(stamp), consumer.context.sky_transport_stamp);
                assert_eq!(
                    values.map(f64::to_bits),
                    [
                        consumer.context.weather.sky_temp,
                        consumer
                            .context
                            .sample
                            .horizontal_infrared_radiation_w_per_m2
                    ]
                    .map(f64::to_bits)
                );
                samplers.push((stamp, values));
            }
            Values::Accumulator {
                hour_index,
                substep,
                received,
                before,
                after,
            } => {
                let (sample_stamp, values) = samplers
                    .get(accumulators)
                    .ok_or("accumulator precedes sampler")?;
                assert_eq!(stamp, *sample_stamp);
                assert_eq!(
                    (hour_index, substep),
                    (accumulators / 4, accumulators as u32 % 4 + 1)
                );
                assert_eq!(received.map(f64::to_bits), values.map(f64::to_bits));
                assert_eq!(before.map(f64::to_bits), sums[hour_index].map(f64::to_bits));
                assert_eq!(
                    after.map(f64::to_bits),
                    [before[0] + received[0], before[1] + received[1]].map(f64::to_bits)
                );
                assert!(after.iter().all(|value| value.is_finite()));
                sums[hour_index] = after;
                accumulators += 1;
            }
            Values::Hourly {
                hour_index,
                divisor,
                pushed: values,
            } => {
                assert_eq!(hour_index, hourly);
                assert_eq!(accumulators, (hour_index + 1) * 4);
                assert_eq!((stamp.record_index, stamp.timestep), (hour_index, 4));
                assert_eq!(divisor.to_bits(), 4.0_f64.to_bits());
                // These are actual report operands, not a reconstructed native oracle.
                assert_eq!(
                    values.map(f64::to_bits),
                    sums[hour_index].map(|value| (value / divisor).to_bits())
                );
                assert_eq!(
                    values.map(f64::to_bits),
                    [sky.values[hour_index], infrared.values[hour_index]].map(f64::to_bits)
                );
                pushed[hour_index] = values;
                hourly += 1;
            }
            Values::Series {
                hour_index,
                handle,
                series,
                value,
            } => {
                let (field, output) = match series {
                    Series::SkyTemperature => (0, sky),
                    Series::HorizontalInfrared => (1, infrared),
                };
                assert_eq!(hour_index, handoffs[field]);
                assert_eq!((stamp.record_index, stamp.timestep), (hour_index, 4));
                assert_eq!(handle, output.handle.0);
                assert_eq!(value.to_bits(), pushed[hour_index][field].to_bits());
                assert_eq!(value.to_bits(), output.values[hour_index].to_bits());
                handoffs[field] += 1;
            }
            _ => unreachable!("selected report kind has the wrong actual payload"),
        }
    }
    assert_eq!(
        (contexts, samplers.len(), accumulators, hourly, handoffs),
        (97, 96, 96, 24, [24, 24])
    );
    assert!(live.snapshot().state.global.end_envrn_flag);
    Ok(())
}
