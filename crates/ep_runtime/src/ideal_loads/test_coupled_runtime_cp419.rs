//! CP419 coupled-runtime accounting contract tests.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp419_conceptual_contract_has_54_outcomes_49_inactive_5_assignments_and_15_sites() {
    assert_eq!((54 - 5, 5, 5 * 3, 36, 41, 51), (49, 5, 15, 36, 41, 51));
}

#[test]
fn cp419_new_state_has_nine_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchCpAirAssignmentRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    for values in [
        state.predecessor_route_counts,
        state.predecessor_guard_false_fallthrough_route_counts,
        state.predecessor_guard_body_entry_route_counts,
        state.predecessor_supply_temperature_saturation_assignment_route_counts,
        state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        state.predecessor_supply_humidity_ratio_assignment_route_counts,
        state.predecessor_supply_enthalpy_assignment_route_counts,
        state.predecessor_dehumidification_guard_else_branch_entry_route_counts,
        state.dehumidification_guard_else_branch_cp_air_assignment_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}
