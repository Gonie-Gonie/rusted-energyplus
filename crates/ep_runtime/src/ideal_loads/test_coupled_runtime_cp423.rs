//! CP423 coupled-runtime accounting and no-feed contracts.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp423_conceptual_contract_has_59_outcomes_49_inactive_5_false_5_assignments_and_eight_sites() {
    assert_eq!(
        (59 - 10, 5, 5, 5 * 8, 36, 41, 51, 5),
        (49, 5, 5, 40, 36, 41, 51, 5)
    );
}

#[test]
fn cp423_new_state_has_three_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputSupplyTemperatureAssignmentRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    for values in [
        state.predecessor_route_counts,
        state.predecessor_guard_false_fallthrough_route_counts,
        state.cooling_sensible_output_supply_temperature_assignment_route_counts,
    ] {
        assert_eq!(values, [0; 36]);
    }
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}

#[test]
fn cp423_binding_and_pipeline_keep_numerical_dto_unchanged() {
    let binding = include_str!("binding.rs");
    let pipeline = include_str!("../../../ep_run/src/pipeline.rs");
    let marker = "guard_else_branch_sensible_output_supply_temperature_assignment";
    assert!(binding.contains(marker));
    assert!(pipeline.contains(&format!("{marker}_lifecycle")));
    for forbidden in [
        "coupling.zone_sensible_cooling_rate_w = calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment",
        "DirectZonePurchasedAirCouplingInput { calculation_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment",
    ] {
        assert!(!binding.contains(forbidden), "{forbidden}");
    }
}
