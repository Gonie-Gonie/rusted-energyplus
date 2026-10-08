//! Compatibility and error-propagation checks for the existing B thermal path.
use super::*;
use crate::weather::day::ProductionWeatherTimestepSeries;
use crate::weather::day::lifecycle_tests::{DECOY_BYTES, configuration};

#[test]
fn b_live_weather_preserves_legacy_results_without_warmup() -> Result<(), Box<dyn std::error::Error>>
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
    let records = crate::weather::parse_epw_records(std::str::from_utf8(DECOY_BYTES)?)?;
    let options = DirectZonePurchasedAirCoupledOptions::hourly_samples(24);
    let legacy = crate::psychrometrics::with_fresh_psychrometric_state(|| {
        let weather = WeatherTimestepSeries::from_records(
            &records[..24],
            4,
            ep_model::FirstHourInterpolationStartingValues::Hour24,
        );
        simulate_direct_zone_purchased_air_coupled_heat_balance(
            &model, &weather, &schedules, options,
        )
    })?;
    let live = ProductionWeatherTimestepSeries::from_bytes(DECOY_BYTES.to_vec(), config)?;
    let (production, trace) = crate::weather::day::production_trace::capture(true, || {
        crate::psychrometrics::with_fresh_psychrometric_state(|| {
            simulate_direct_zone_purchased_air_coupled_heat_balance_with_production_weather(
                &model, &live, &schedules, options,
            )
        })
    });
    let production = production?;
    assert_eq!(legacy.results, production.results);
    let trace = trace.ok_or("missing production trace")?;
    assert!(trace.consumers.iter().all(|consumer| {
        matches!(
            consumer.phase,
            crate::weather::day::WeatherDayPhase::Run { .. }
        )
    }));
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
