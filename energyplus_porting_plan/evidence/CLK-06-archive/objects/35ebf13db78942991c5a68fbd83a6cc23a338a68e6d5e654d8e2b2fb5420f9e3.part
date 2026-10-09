//! Live B weather transport and source-failure propagation checks.
use super::*;
use crate::weather::day::ProductionWeatherTimestepSeries;
use crate::weather::day::lifecycle_tests::{DECOY_BYTES, configuration};

#[test]
fn b_live_weather_reaches_hourly_reports_without_warmup() -> Result<(), Box<dyn std::error::Error>>
{
    let config = configuration(
        6,
        30,
        6,
        30,
        ep_model::FirstHourInterpolationStartingValues::Hour24,
    )?;
    let mut typed = exact_model(4).typed;
    typed.run_periods = vec![config.run_period.clone()];
    typed.site = Some(config.site.clone());
    let model = SimulationModel::from_typed(typed);
    let schedules = precompute_schedule_cache(&model.typed, 24 * 4)?;
    let options = DirectZonePurchasedAirCoupledOptions::hourly_samples(24);
    let live = ProductionWeatherTimestepSeries::from_bytes(DECOY_BYTES.to_vec(), config)?;
    let (production, trace) = crate::weather::day::production_trace::capture(true, || {
        crate::psychrometrics::with_fresh_psychrometric_state(|| {
            simulate_direct_zone_purchased_air_coupled_heat_balance_with_production_weather(
                &model, &live, &schedules, options,
            )
        })
    });
    let production = production?;
    let trace = trace.ok_or("missing production trace")?;
    assert_eq!(production.summary.samples, 24);
    assert_eq!(production.summary.timestep_count, 24 * 4);
    assert_eq!(production.summary.coupling_call_count, 24 * 4);
    assert_eq!(trace.total_operation_count as usize, trace.operations.len());
    assert_eq!(trace.total_consumer_count as usize, trace.consumers.len());
    assert_eq!(
        trace
            .operations
            .iter()
            .filter(|op| op.kind == "SetCurrentWeather")
            .count(),
        24 * 4
    );
    assert_eq!(trace.consumers.len(), 1 + 24 * 4);
    // The actual humidity seed and first thermal call reuse one accepted Current.
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
            .ok_or("consumer has no actual completed operation")?;
        assert_eq!(current.kind, "SetCurrentWeather");
        assert!(current.error.is_none());
        assert_eq!(
            current.after.state.global.hour_of_day as usize,
            consumer.record_index + 1
        );
        assert_eq!(
            current.after.state.global.time_step as u32,
            consumer.timestep
        );
        let slot = current
            .after
            .state
            .today_values
            .hour(consumer.record_index + 1)?
            .get(consumer.timestep as usize - 1)
            .ok_or("actual Today slot unavailable")?;
        assert_eq!(consumer.context.weather, *slot);
        assert_eq!(
            consumer.context.current_weather,
            current.after.state.environment.current_weather
        );
        assert_eq!(
            consumer.context.sample.dry_bulb_c.to_bits(),
            consumer.context.current_weather.out_dry_bulb_temp.to_bits()
        );
        // Requires the independently reviewed production Today-IR copy stage.
        assert_eq!(
            consumer
                .context
                .sample
                .horizontal_infrared_radiation_w_per_m2
                .to_bits(),
            slot.horiz_ir_sky.to_bits()
        );
    }
    let sky = production
        .results
        .find_series("Environment", "Site Sky Temperature")
        .ok_or("missing actual sky report")?;
    let infrared = production
        .results
        .find_series(
            "Environment",
            "Site Horizontal Infrared Radiation Rate per Area",
        )
        .ok_or("missing actual infrared report")?;
    assert_eq!((&*sky.units, &*infrared.units), ("C", "W/m2"));
    assert_eq!((sky.values.len(), infrared.values.len()), (24, 24));
    // Skip only the explicitly verified initial seed; retain every thermal call.
    for (hour, consumers) in trace.consumers[1..].chunks_exact(4).enumerate() {
        for (step, consumer) in consumers.iter().enumerate() {
            assert_eq!(
                (consumer.record_index, consumer.timestep),
                (hour, step as u32 + 1)
            );
        }
        let sky_mean = consumers
            .iter()
            .fold(0.0, |sum, consumer| sum + consumer.context.weather.sky_temp)
            / 4.0;
        let ir_mean = consumers.iter().fold(0.0, |sum, consumer| {
            sum + consumer
                .context
                .sample
                .horizontal_infrared_radiation_w_per_m2
        }) / 4.0;
        assert!(sky.values[hour].is_finite() && infrared.values[hour].is_finite());
        assert_eq!(sky.values[hour].to_bits(), sky_mean.to_bits());
        assert_eq!(infrared.values[hour].to_bits(), ir_mean.to_bits());
    }
    assert!(live.snapshot().state.global.end_envrn_flag);
    Ok(())
}

#[test]
fn b_reports_weather_source_fatal_without_thermal_or_end_environment_fallback()
-> Result<(), Box<dyn std::error::Error>> {
    let config = configuration(
        7,
        1,
        7,
        1,
        ep_model::FirstHourInterpolationStartingValues::Hour24,
    )?;
    let mut typed = exact_model(4).typed;
    typed.run_periods = vec![config.run_period.clone()];
    typed.site = Some(config.site.clone());
    let model = SimulationModel::from_typed(typed);
    let schedules = precompute_schedule_cache(&model.typed, 24 * 4)?;
    let bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../energyplus_porting_plan/cases/CLK-03/missing-target-complete-days.epw"
    ));
    let live = ProductionWeatherTimestepSeries::from_bytes(bytes.to_vec(), config)?;
    let result = simulate_direct_zone_purchased_air_coupled_heat_balance_with_production_weather(
        &model,
        &live,
        &schedules,
        DirectZonePurchasedAirCoupledOptions::hourly_samples(24),
    );
    assert!(matches!(
        result,
        Err(DirectZonePurchasedAirCoupledRuntimeError::HeatBalance(
            RuntimeError::WeatherDay {
                source_fatal: true,
                ..
            }
        ))
    ));
    assert!(live.snapshot().stream.fail);
    assert!(!live.snapshot().state.global.end_envrn_flag);
    Ok(())
}
