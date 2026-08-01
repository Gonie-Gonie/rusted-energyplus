//! CP385 23-route refinement and exact accounting.

use super::{predecessor_for_route, retained_input};
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentRuntimeState as State,
    advance_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_state as advance,
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
};

#[test]
fn cp385_maps_twenty_three_routes_with_thirteen_null_five_preserved_five_assigned() {
    let system = predecessor_for_route(0, 0, false, 1).system;
    let mut state = State::new(system);
    let mut snapshots = Vec::new();
    let mut ordinal = 1;
    for inherited in 0..3 {
        let predecessor = predecessor_for_route(inherited, 0, false, ordinal);
        snapshots.push(advance(&mut state, predecessor, None).expect("complete skip"));
        ordinal += 1;
    }
    for inherited in 3..8 {
        for (outcome, assignment) in [(0, false), (2, false), (1, false), (1, true)] {
            let predecessor = predecessor_for_route(inherited, outcome, assignment, ordinal);
            let input = retained_input(predecessor, 41_000.0, 50_000.0, 2.0);
            snapshots.push(advance(&mut state, predecessor, input).expect("route"));
            ordinal += 1;
        }
    }
    assert_eq!(snapshots.len(), 23);
    let null = snapshots.iter().filter(|s| s.preexisting_supply_enthalpy_j_per_kg.is_none()).count();
    let preserved = snapshots.iter().filter(|s| {
        !s.supply_enthalpy_assignment_executed
            && s.preexisting_supply_enthalpy_j_per_kg.is_some()
    }).count();
    let assigned = snapshots.iter().filter(|s| s.supply_enthalpy_assignment_executed).count();
    assert_eq!((null, preserved, assigned), (13, 5, 5));
    for snapshot in snapshots.iter().filter(|s| !s.supply_enthalpy_assignment_executed && s.preexisting_supply_enthalpy_j_per_kg.is_some()) {
        assert_eq!(
            snapshot.preexisting_supply_enthalpy_j_per_kg.map(f64::to_bits),
            snapshot.resulting_supply_enthalpy_j_per_kg.map(f64::to_bits),
        );
        assert!(!snapshot.mixed_air_enthalpy_read);
    }
    assert_eq!(state.transition_count, 23);
    assert_eq!(state.dehumidification_total_output_capacity_guard_evaluation_count, 10);
    assert_eq!(state.dehumidification_total_output_capacity_guard_false_fallthrough_count, 5);
    assert_eq!(state.dehumidification_total_output_maximum_capacity_assignment_count, 5);
    assert_eq!(state.post_saturation_capacity_limited_dehumidification_supply_enthalpy_assignment_count, 5);
    assert_eq!(state.cp379_retained_supply_enthalpy_owned_read_count, 10);
    assert_eq!(state.source_site_execution_count, 30);
    for count in [
        state.cp329_retained_mixed_air_enthalpy_owned_read_count,
        state.mixed_air_enthalpy_read_count,
        state.cp384_retained_cooling_total_output_owned_read_count,
        state.cooling_total_output_read_count,
        state.cp330_retained_supply_mass_flow_rate_owned_read_count,
        state.supply_mass_flow_rate_read_count,
        state.specific_cooling_output_calculation_count,
        state.supply_enthalpy_difference_calculation_count,
        state.supply_enthalpy_assignment_write_count,
    ] {
        assert_eq!(count, 5);
    }
    assert_eq!(
        snapshots.iter().filter(|snapshot| {
            cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact_direct_release(**snapshot)
        }).count(),
        11,
    );
}
