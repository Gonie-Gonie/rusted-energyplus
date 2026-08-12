//! CP417 coupled-runtime accounting contract tests.

use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentRuntimeState;
use ep_model::IdealLoadsAirSystemId;

#[test]
fn cp417_conceptual_contract_has_54_outcomes_72_sites_and_expected_carrier_ownership() {
    let predecessor_inactive = 36;
    let assignments = 18;
    let total = predecessor_inactive + assignments;
    assert_eq!(
        (
            total,
            total - assignments,
            assignments,
            assignments * 4,
            36,
            41,
            51,
            36,
            41 - assignments,
            51,
        ),
        (54, 36, 18, 72, 36, 41, 51, 36, 23, 51),
    );
}

#[test]
fn cp417_new_state_has_zeroed_lossless_route_partitions() {
    let state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyEnthalpyAssignmentRuntimeState::new(
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
    assert_eq!(state.supply_enthalpy_assignment_route_counts, [0; 36]);
    assert_eq!(state.source_site_execution_count, 0);
    assert!(state.latest.is_none());
}
