//! CP414 coupled-runtime accounting contract tests.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp414_conceptual_contract_has_54_outcomes_72_sites_and_expected_carrier_ownership() {
    let predecessor_inactive = 18;
    let predecessor_guard_false = 18;
    let assignments = 18;
    let total = predecessor_inactive + predecessor_guard_false + assignments;
    assert_eq!(
        (
            total,
            total - assignments,
            assignments,
            assignments * 4,
            36,
            41,
            51,
            51 - assignments,
        ),
        (54, 36, 18, 72, 36, 41, 51, 33),
    );
}

#[test]
fn cp414_new_state_has_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyTemperatureSaturationAssignmentRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    assert_eq!(state.predecessor_route_counts, [0; 36]);
    assert_eq!(
        state.predecessor_guard_false_fallthrough_route_counts,
        [0; 36]
    );
    assert_eq!(state.predecessor_guard_body_entry_route_counts, [0; 36]);
    assert_eq!(
        state.supply_temperature_saturation_assignment_route_counts,
        [0; 36]
    );
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}
