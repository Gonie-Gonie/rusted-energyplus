use super::{C0, CSH, H, N, P, Q, U, operands, predecessor};
use crate::ideal_loads::calc::cooling_humidistat_moisture_demand_assignment::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentActiveOperands as Operands,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentRuntimeState as State,
    advance_cooling_humidistat_moisture_demand_assignment_state as advance,
    cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release,
};
use ep_model::IdealLoadsAirSystemId;

#[test]
fn source_boundary_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2229"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2230"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
        &[
            "read-zone-dehumidifying-setpoint-moisture-demand",
            "assign-local-zone-dehumidifying-setpoint-moisture-demand",
        ]
    );

    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in [U, N, P, C0, Q, H, CSH].into_iter().enumerate() {
        let snapshot = advance(
            &mut state,
            predecessor(route, index + 1),
            operands(route, -0.0),
        )
        .expect("CP359 route");
        assert_eq!(
            snapshot.dehumidification_control_humidistat_moisture_demand_assignment_executed,
            route == H
        );
        assert_eq!(
            snapshot.zone_dehumidifying_setpoint_moisture_demand_read,
            route == H
        );
        assert_eq!(
            snapshot
                .zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_some(),
            route == H
        );
        assert_eq!(
            cooling_humidistat_moisture_demand_assignment_snapshot_is_exact_direct_release(
                snapshot
            ),
            matches!(route, U | N | P | C0)
        );
        if route == Q {
            assert!(
                !snapshot.dehumidification_control_humidistat_moisture_demand_assignment_executed
            );
        }
    }
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.source_site_execution_count, 2);
    assert_eq!(
        state.zone_dehumidifying_setpoint_moisture_demand_read_count,
        1
    );
    assert_eq!(
        state.zone_dehumidifying_setpoint_moisture_demand_assignment_count,
        1
    );
}

#[test]
fn active_operand_contract_is_transactional_and_route_exact() {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let before = state.clone();
    assert!(advance(&mut state, predecessor(H, 1), None).is_none());
    assert_eq!(state, before);

    let supplied_on_skip = Some(Operands {
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s: 1.0,
    });
    assert!(advance(&mut state, predecessor(C0, 1), supplied_on_skip).is_none());
    assert_eq!(state, before);

    let mut forged = predecessor(H, 1);
    forged.dehumidification_control_humidistat_case_entered = false;
    assert!(advance(&mut state, forged, operands(H, 1.0)).is_none());
    assert_eq!(state, before);
}
