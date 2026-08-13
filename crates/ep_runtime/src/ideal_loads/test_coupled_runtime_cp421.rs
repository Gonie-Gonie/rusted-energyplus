//! CP421 coupled-runtime accounting and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp421_conceptual_contract_has_59_outcomes_49_inactive_5_false_5_body_and_four_sites() {
    assert_eq!(
        (59 - 10, 5, 5, 10 * 3 + 5, 36, 41, 56),
        (49, 5, 5, 35, 36, 41, 56)
    );
}

#[test]
fn cp421_new_state_has_three_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputGuardRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    for values in [
        state.predecessor_route_counts,
        state.guard_false_fallthrough_route_counts,
        state.adjustment_body_entry_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp421_binding_and_pipeline_keep_numerical_dto_unchanged() {
    let binding = include_str!("binding.rs");
    let pipeline = include_str!("../../../ep_run/src/pipeline.rs");
    let marker = "guard_else_branch_sensible_output_guard";
    assert!(binding.contains(marker));
    assert!(pipeline.contains("guard_else_branch_sensible_output_guard_lifecycle"));
    for forbidden in [
        "coupling.zone_sensible_cooling_rate_w = calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard",
        "DirectZonePurchasedAirCouplingInput { calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard",
    ] {
        assert!(!binding.contains(forbidden), "{forbidden}");
    }
}
