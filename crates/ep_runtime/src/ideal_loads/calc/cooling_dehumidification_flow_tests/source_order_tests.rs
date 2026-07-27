use super::{active_predecessor, assert_bits, base_input, characterize};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER,
};

#[test]
fn source_boundary_and_all_twenty_one_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2119-2128"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2133"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER.len(),
        21
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER.first(),
        Some(&"assign-supply-mass-flow-rate-for-dehumidification-zero")
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_DEHUMIDIFICATION_FLOW_SOURCE_ORDER.last(),
        Some(&"assign-supply-mass-flow-rate-for-dehumidification")
    );
}

#[test]
fn humidistat_route_preserves_source_reads_and_single_division() {
    let input = base_input();
    let (snapshot, state) = characterize(active_predecessor(), input);
    let delta = input.minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air
        - input.zone_humidity_ratio_kg_water_per_kg_dry_air;
    let expected = input.zone_dehumidifying_setpoint_moisture_demand_kg_per_s / delta;

    assert!(snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
    assert!(snapshot.zone_dehumidifying_setpoint_moisture_demand_assigned);
    assert_bits(
        snapshot.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        input.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
    );
    assert_bits(
        snapshot.assigned_delta_humidity_ratio_kg_water_per_kg_dry_air,
        delta,
    );
    assert!(snapshot.zone_dehumidifying_setpoint_moisture_demand_for_gate_read);
    assert!(snapshot.dehumidification_flow_body_entered);
    assert_bits(
        snapshot.calculated_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        expected,
    );
    assert_bits(
        snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
        expected,
    );
    assert_eq!(
        state.supply_mass_flow_rate_for_dehumidification_calculation_count,
        1
    );
    assert_eq!(
        state.supply_mass_flow_rate_for_dehumidification_assignment_count,
        1
    );
}
