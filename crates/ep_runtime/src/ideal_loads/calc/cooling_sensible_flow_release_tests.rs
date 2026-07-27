//! CP318 exact direct-release tests.

use ep_model::{IdealLoadsAirSystem, ZoneId};

use crate::heat_balance::state::{ZoneAirTemperatureCoefficients, ZoneHeatBalanceState};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingEconomizerBodySnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_economizer_body,
    advance_direct_no_oa_calc_cooling_sensible_flow,
    purchased_air_calc_cooling_sensible_flow_lifecycle_summary,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

use super::cooling_economizer_body_release_tests::{
    advance_subsequent_body_predecessor, body_release_fixture_with_cooling_demand,
};

mod corruption_tests;

#[test]
fn public_active_cooling_executes_exact_left_associated_source_route() {
    let (mut runtime, system, predecessor, zone_state) = release_case(-1_000.0);
    let snapshot = advance_direct_no_oa_calc_cooling_sensible_flow(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("exact active CP318 release transition");
    let cp_air = energyplus_psy_cp_air_fn_w(zone_state.air_humidity_ratio);
    let delta = system.minimum_cooling_supply_air_temperature_c - zone_state.mean_air_temperature_c;
    let expected = (-1_000.0 / cp_air) / delta;

    assert!(super::cooling_sensible_flow_snapshot_is_exact_direct_release(snapshot));
    assert!(snapshot.cooling_body_entered);
    assert!(snapshot.supply_mass_flow_rate_for_cool_reset_assigned);
    assert_eq!(snapshot.cooling_on, Some(true));
    assert!(snapshot.delta_temperature_body_entered);
    assert_eq!(
        snapshot
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s
            .expect("assigned cooling flow")
            .to_bits(),
        expected.to_bits()
    );

    let lifecycle = purchased_air_calc_cooling_sensible_flow_lifecycle_summary(&runtime, system.id)
        .expect("CP318 lifecycle");
    assert_eq!(lifecycle.state.transition_count, 1);
    assert_eq!(lifecycle.state.cooling_body_entry_count, 1);
    assert_eq!(
        lifecycle
            .state
            .supply_mass_flow_rate_for_cool_assignment_count,
        1
    );
    assert_eq!(lifecycle.state.latest, Some(snapshot));
}

#[test]
fn public_non_cooling_route_skips_poisoned_source_inputs() {
    let (mut runtime, system, predecessor, mut zone_state) = release_case(1.0);
    zone_state.air_humidity_ratio = f64::NAN;
    zone_state.mean_air_temperature_c = f64::NAN;
    let snapshot = advance_direct_no_oa_calc_cooling_sensible_flow(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("exact non-cooling CP318 release transition");

    assert!(super::cooling_sensible_flow_snapshot_is_exact_direct_release(snapshot));
    assert!(snapshot.non_cooling_skipped);
    assert!(!snapshot.cooling_body_entered);
    assert!(!snapshot.supply_mass_flow_rate_for_cool_reset_assigned);
    assert!(!snapshot.cooling_on_read);
    assert!(!snapshot.zone_humidity_ratio_read);
    assert!(!snapshot.zone_temperature_read);
    assert!(!snapshot.zone_cooling_setpoint_load_read);
    assert!(
        snapshot
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s
            .is_none()
    );
}

#[test]
fn public_active_delta_fallthrough_retains_the_positive_zero_reset() {
    let (mut runtime, system, predecessor, mut zone_state) = release_case(-1_000.0);
    zone_state.mean_air_temperature_c = system.minimum_cooling_supply_air_temperature_c;
    let snapshot = advance_direct_no_oa_calc_cooling_sensible_flow(
        &mut runtime,
        &system,
        predecessor,
        &zone_state,
    )
    .expect("exact CP318 delta fallthrough");

    assert_eq!(
        snapshot.delta_temperature_below_negative_small_temp_diff,
        Some(false)
    );
    assert!(!snapshot.delta_temperature_body_entered);
    assert!(!snapshot.zone_cooling_setpoint_load_read);
    assert_eq!(
        snapshot
            .resulting_supply_mass_flow_rate_for_cool_kg_per_s
            .expect("reset result")
            .to_bits(),
        0.0_f64.to_bits()
    );
    assert!(super::cooling_sensible_flow_snapshot_is_exact_direct_release(snapshot));
}

#[test]
fn repeated_release_calls_preserve_one_for_one_history_across_route_changes() {
    let (mut runtime, system, first_predecessor, zone_state) = release_case(-1_000.0);
    advance_direct_no_oa_calc_cooling_sensible_flow(
        &mut runtime,
        &system,
        first_predecessor,
        &zone_state,
    )
    .expect("first CP318 call");

    let condition = advance_subsequent_body_predecessor(&mut runtime, &system, 1.0);
    let body = advance_direct_no_oa_calc_cooling_economizer_body(&mut runtime, &system, condition)
        .expect("second CP317 predecessor");
    let second =
        advance_direct_no_oa_calc_cooling_sensible_flow(&mut runtime, &system, body, &zone_state)
            .expect("second CP318 call");

    assert!(second.non_cooling_skipped);
    let state = &runtime
        .units
        .get(&system.id)
        .expect("selected unit")
        .calc_cooling_sensible_flow;
    assert_eq!(state.transition_count, 2);
    assert_eq!(state.cooling_body_entry_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.supply_mass_flow_rate_for_cool_assignment_count, 1);
}

pub(super) fn release_case(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    PurchasedAirCalcCoolingEconomizerBodySnapshot,
    ZoneHeatBalanceState,
) {
    let (mut runtime, system, condition) =
        body_release_fixture_with_cooling_demand(cooling_demand_w);
    let predecessor =
        advance_direct_no_oa_calc_cooling_economizer_body(&mut runtime, &system, condition)
            .expect("exact CP317 predecessor");
    let zone_state = zone_state(predecessor.controlled_zone);
    (runtime, system, predecessor, zone_state)
}

pub(super) fn zone_state(zone_id: ZoneId) -> ZoneHeatBalanceState {
    ZoneHeatBalanceState {
        zone_id,
        zone_name: "ZONE ONE".to_string(),
        mean_air_temperature_c: 22.0,
        zone_timestep_average_air_temperature_c: 22.0,
        previous_mean_air_temperatures_c: [0.0; 3],
        previous_system_mean_air_temperatures_c: [0.0; 3],
        previous_system_timestep_count: 1,
        air_humidity_ratio: 0.008,
        zone_timestep_average_air_humidity_ratio: 0.008,
        previous_air_humidity_ratios: [0.008; 3],
        previous_system_air_humidity_ratios: [0.008; 3],
        use_zone_timestep_history: false,
        shorten_timestep_sys: false,
        prior_timestep_seconds: 600.0,
        volume_m3: 100.0,
        air_heat_capacity_j_per_k: 0.0,
        convective_internal_gain_w: 0.0,
        opaque_surface_conductance_w_per_k: 100.0,
        opaque_surface_heat_gain_w: 0.0,
        opaque_surface_outside_conduction_w: 0.0,
        sum_ha_w_per_k: 100.0,
        sum_hat_surf_w: 0.0,
        sum_hat_ref_w: 0.0,
        sum_mcp_w_per_k: 0.0,
        sum_mcp_t_w: 0.0,
        sum_sys_mcp_w_per_k: 0.0,
        sum_sys_mcp_t_w: 0.0,
        system_dependent_zone_loads_lagged_w: 0.0,
        zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
        system_timestep_average_surface_convection_report_w: None,
        system_timestep_average_air_storage_report_w: None,
    }
}
