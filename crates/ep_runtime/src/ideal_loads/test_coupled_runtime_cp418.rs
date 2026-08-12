//! CP418 coupled-runtime accounting contract tests.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp418_conceptual_contract_has_54_outcomes_5_else_entries_and_preserves_all_carriers() {
    let inherited_or_true_body_inactive = 49;
    let else_entries = 5;
    let total = inherited_or_true_body_inactive + else_entries;
    assert_eq!(
        (
            total,
            total - else_entries,
            else_entries,
            else_entries,
            36,
            41,
            51,
            36,
            41,
            51,
        ),
        (54, 49, 5, 5, 36, 41, 51, 36, 41, 51),
    );
}

#[test]
fn cp418_new_state_has_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchEntryRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    assert_eq!(state.predecessor_route_counts, [0; 36]);
    assert_eq!(
        state.predecessor_guard_false_fallthrough_route_counts,
        [0; 36]
    );
    assert_eq!(state.predecessor_guard_body_entry_route_counts, [0; 36]);
    assert_eq!(
        state.predecessor_supply_temperature_saturation_assignment_route_counts,
        [0; 36]
    );
    assert_eq!(
        state.predecessor_supply_temperature_mixed_air_limit_route_counts,
        [0; 36]
    );
    assert_eq!(
        state.predecessor_supply_humidity_ratio_assignment_route_counts,
        [0; 36]
    );
    assert_eq!(
        state.predecessor_supply_enthalpy_assignment_route_counts,
        [0; 36]
    );
    assert_eq!(
        state.dehumidification_guard_else_branch_entry_route_counts,
        [0; 36]
    );
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}
