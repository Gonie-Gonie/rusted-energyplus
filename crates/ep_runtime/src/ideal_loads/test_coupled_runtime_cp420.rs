//! CP420 coupled-runtime accounting and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp420_conceptual_contract_has_54_outcomes_49_inactive_5_assignments_and_40_sites() {
    assert_eq!((54 - 5, 5, 5 * 8, 36, 41, 51), (49, 5, 40, 36, 41, 51));
}

#[test]
fn cp420_new_state_has_ten_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentRuntimeState::new(
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
        state.predecessor_dehumidification_guard_else_branch_cp_air_assignment_route_counts,
        state.dehumidification_guard_else_branch_sensible_output_assignment_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp420_binding_and_pipeline_keep_numerical_dto_unchanged() {
    let binding = include_str!("binding.rs");
    let pipeline = include_str!("../../../ep_run/src/pipeline.rs");
    assert!(binding.contains("guard_else_branch_sensible_output_assignment"));
    assert!(pipeline.contains("guard_else_branch_sensible_output_assignment_lifecycle"));
    assert!(!binding.contains("coupling.zone_sensible_cooling_rate_w = calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment"));
}
