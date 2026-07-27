use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

use super::{active_predecessor, assert_bits, base_input, characterize};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER,
};

#[test]
fn source_boundary_and_all_nineteen_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2109-2116"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2119"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER.len(),
        19
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER.first(),
        Some(&"assign-supply-mass-flow-rate-for-cool-zero")
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SENSIBLE_FLOW_SOURCE_ORDER.last(),
        Some(&"assign-supply-mass-flow-rate-for-cool")
    );
}

#[test]
fn active_route_uses_canonical_psychrometrics_and_left_associated_divisions() {
    let input = base_input();
    let (snapshot, state) = characterize(active_predecessor(), input);
    let cp_air = energyplus_psy_cp_air_fn_w(input.zone_humidity_ratio);
    let delta = input.minimum_cooling_supply_air_temperature_c - input.zone_temperature_c;
    let first_division = input.zone_cooling_setpoint_load_w / cp_air;
    let expected = first_division / delta;

    assert_bits(snapshot.psychrometric_cp_air_result_j_per_kg_k, cp_air);
    assert_bits(snapshot.cp_air_j_per_kg_k, cp_air);
    assert_bits(snapshot.delta_temperature_c, delta);
    assert_bits(
        snapshot.zone_cooling_setpoint_load_over_cp_air_kg_k_per_s,
        first_division,
    );
    assert_bits(
        snapshot.calculated_supply_mass_flow_rate_for_cool_kg_per_s,
        expected,
    );
    assert_bits(
        snapshot.assigned_supply_mass_flow_rate_for_cool_kg_per_s,
        expected,
    );
    assert_bits(
        snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
        expected,
    );
    assert_eq!(state.supply_mass_flow_rate_for_cool_calculation_count, 1);
    assert_eq!(state.supply_mass_flow_rate_for_cool_assignment_count, 1);
}
