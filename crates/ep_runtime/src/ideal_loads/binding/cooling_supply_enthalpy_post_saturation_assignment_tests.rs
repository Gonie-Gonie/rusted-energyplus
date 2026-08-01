use super::*;
use crate::ideal_loads::{
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle_summary,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case_with_pressure;

#[test]
fn binding_orders_cp378_then_cp379_and_records_the_four_source_sites() {
    let Some((runtime, output)) = run_case_with_pressure(
        IdealLoadsLimit::NoLimit,
        None,
        3_000.0,
        1.0,
        Some(101_325.0),
    ) else {
        return;
    };
    let humidity = output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
    let temperature = output.calculation_cooling_supply_humidity_ratio_saturation_assignment;
    let snapshot = output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
    assert_eq!(
        (
            snapshot.system,
            snapshot.parent_call_ordinal,
            snapshot.controlled_zone,
        ),
        (
            humidity.system,
            humidity.parent_call_ordinal,
            humidity.controlled_zone,
        ),
    );
    assert!(
        cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    assert_eq!(
        snapshot.supply_temperature_c.map(f64::to_bits),
        temperature
            .supply_temperature_for_saturation_humidity_ratio_c
            .map(f64::to_bits),
    );
    assert_eq!(
        snapshot.supply_humidity_ratio.map(f64::to_bits),
        humidity.resulting_supply_humidity_ratio.map(f64::to_bits),
    );
    let state =
        purchased_air_calc_cooling_supply_enthalpy_post_saturation_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP379 lifecycle")
        .state;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.source_site_execution_count, 4);
    assert_eq!(
        state.local_supply_enthalpy_after_saturation_limit_assignment_count,
        1
    );
    assert_eq!(
        state.cp334_supply_temperature_mixed_air_limit_owner_count
            + state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
        1
    );
    assert_eq!(
        state.cp378_supply_humidity_ratio_saturation_limit_owner_count,
        1
    );
}

#[test]
fn binding_keeps_cp379_canonical_enthalpy_out_of_the_unchanged_numerical_coupling() {
    let output = run_zero_humidity_case();
    let snapshot = output.calculation_cooling_supply_enthalpy_post_saturation_assignment;
    let temperature = snapshot
        .supply_temperature_c
        .expect("active CP379 temperature");
    let humidity_ratio = snapshot
        .supply_humidity_ratio
        .expect("active CP379 humidity ratio");
    assert!(
        humidity_ratio < 1.0e-5,
        "fixture must exercise the EnergyPlus humidity floor"
    );
    let canonical = energyplus_psy_h_fn_tdb_w(temperature, humidity_ratio);
    assert_eq!(
        snapshot
            .resulting_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(canonical.to_bits()),
    );
    assert_ne!(
        canonical.to_bits(),
        output
            .coupling
            .purchased_air
            .calculation
            .supply_enthalpy_j_per_kg
            .to_bits(),
        "CP379 evidence must not feed or reconcile the unchanged numerical calculation"
    );
}

fn run_zero_humidity_case() -> DirectZonePurchasedAirScheduledCouplingOutput {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = IdealLoadsLimit::NoLimit;
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
        system.maximum_total_cooling_capacity_w = None;
        system.dehumidification_control_type = DehumidificationControlType::None;
        system.humidification_control_type = HumidificationControlType::None;
        system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
        schedule_mut(typed, ScheduleId(3)).hourly_value = 1.0;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("zero-humidity binding");
    let mut zone_state = zone_state_for_temp_independent_load(3_000.0);
    zone_state.air_humidity_ratio = 0.0;
    zone_state.zone_timestep_average_air_humidity_ratio = 0.0;
    zone_state.previous_air_humidity_ratios = [0.0; 3];
    zone_state.previous_system_air_humidity_ratios = [0.0; 3];
    let mut runtime = PurchasedAirRuntimeState::default();
    couple_model_bound_direct_zone_purchased_air(DirectZonePurchasedAirScheduledCouplingInput {
        binding: &binding,
        schedule_cache: &cache,
        schedule_sample_index: 0,
        zone_state: &mut zone_state,
        purchased_air_runtime_state: &mut runtime,
        begin_environment: true,
        barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
        system_timestep_seconds: binding.nominal_system_timestep_seconds,
    })
    .expect("zero-humidity coupling")
}
