use super::{C0, CSH, H, N, P, Q, U, operands, predecessor};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentActiveOperands as Operands,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState as State,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_state as advance,
};
use ep_model::IdealLoadsAirSystemId;

#[test]
fn source_boundary_six_sites_and_seven_routes_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2230"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2231"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-local-zone-dehumidifying-setpoint-moisture-demand-for-supply-humidity-ratio-division",
            "read-retained-supply-mass-flow-rate-for-supply-humidity-ratio-division",
            "calculate-zone-dehumidifying-setpoint-moisture-demand-divided-by-supply-mass-flow-rate",
            "read-zone-node-humidity-ratio-for-dehumidification-supply-humidity-ratio",
            "add-zone-node-humidity-ratio-to-moisture-demand-derived-supply-humidity-ratio",
            "assign-local-supply-humidity-ratio-for-dehumidification",
        ]
    );

    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in [U, N, P, C0, Q, H, CSH].into_iter().enumerate() {
        let snapshot = advance(
            &mut state,
            predecessor(route, index + 1, -0.002),
            operands(route, 0.5, 0.008),
        )
        .expect("CP360 route");
        let active = route == H;
        assert_eq!(
            snapshot.dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed,
            active
        );
        for flag in [
            snapshot.zone_dehumidifying_setpoint_moisture_demand_read,
            snapshot.supply_mass_flow_rate_read,
            snapshot.moisture_demand_derived_supply_humidity_ratio_calculated,
            snapshot.zone_node_humidity_ratio_read,
            snapshot.supply_humidity_ratio_for_dehumidification_calculated,
            snapshot.supply_humidity_ratio_for_dehumidification_assigned,
        ] {
            assert_eq!(flag, active);
        }
        for value in [
            snapshot.zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.supply_mass_flow_rate_kg_per_s,
            snapshot.moisture_demand_derived_supply_humidity_ratio,
            snapshot.zone_node_humidity_ratio,
            snapshot.calculated_supply_humidity_ratio_for_dehumidification,
            snapshot.assigned_supply_humidity_ratio_for_dehumidification,
            snapshot.resulting_supply_humidity_ratio_for_dehumidification,
        ] {
            assert_eq!(value.is_some(), active);
        }
        if active {
            assert_eq!(
                snapshot
                    .moisture_demand_derived_supply_humidity_ratio
                    .expect("quotient")
                    .to_bits(),
                (-0.002_f64 / 0.5).to_bits()
            );
            assert_eq!(
                snapshot
                    .resulting_supply_humidity_ratio_for_dehumidification
                    .expect("result")
                    .to_bits(),
                ((-0.002_f64 / 0.5) + 0.008).to_bits()
            );
        }
        if route == Q {
            assert!(!snapshot.zone_node_humidity_ratio_read);
        }
    }
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.source_site_execution_count, 6);
    assert_eq!(
        [
            state.zone_dehumidifying_setpoint_moisture_demand_read_count,
            state.supply_mass_flow_rate_read_count,
            state.moisture_demand_derived_supply_humidity_ratio_calculation_count,
            state.zone_node_humidity_ratio_read_count,
            state.supply_humidity_ratio_for_dehumidification_calculation_count,
            state.supply_humidity_ratio_for_dehumidification_assignment_count,
        ],
        [1; 6]
    );
}

#[test]
fn active_operand_contract_and_predecessor_shape_are_transactional() {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let before = state.clone();
    assert!(advance(&mut state, predecessor(H, 1, 1.0), None).is_none());
    assert_eq!(state, before);

    let supplied_on_skip = Some(Operands {
        supply_mass_flow_rate_kg_per_s: 1.0,
        zone_node_humidity_ratio: 0.008,
    });
    assert!(advance(&mut state, predecessor(C0, 1, 1.0), supplied_on_skip).is_none());
    assert_eq!(state, before);

    let mut forged = predecessor(H, 1, -0.0);
    forged.assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s = Some(0.0);
    assert!(advance(&mut state, forged, operands(H, 1.0, 0.008)).is_none());
    assert_eq!(state, before);
}
