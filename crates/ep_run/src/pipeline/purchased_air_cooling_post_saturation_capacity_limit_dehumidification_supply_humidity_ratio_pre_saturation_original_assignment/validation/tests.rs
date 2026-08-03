use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp411_validator_depends_only_on_cp410() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;
}

#[test]
fn ep_run_cp411_rejects_missing_cp410_predecessor_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_PRE_SATURATION_ORIGINAL_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let result = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1));
    assert!(
        result
            .as_ref()
            .is_err_and(|error| error.contains("CP410 evidence is missing"))
    );
}

#[test]
fn compact_snapshot_copies_active_humidity_ratio_and_skips_inactive_local_values() {
    let active = super::super::test_snapshot(Some(-0.0), true);
    let inactive = super::super::test_snapshot(Some(-0.0), false);
    assert!(
        active
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed
    );
    assert!(active.assigned_supply_humidity_ratio_original.is_some());
    assert!(
        !inactive
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed
    );
    assert!(inactive.assigned_supply_humidity_ratio_original.is_none());
    assert_eq!(
        inactive.resulting_supply_humidity_ratio.map(f64::to_bits),
        inactive
            .predecessor_cp410_resulting_supply_humidity_ratio
            .map(f64::to_bits)
    );
}

#[test]
fn route_evidence_partitions_predecessor_splits_and_assignment_routes() {
    let mut routes = [0; 30];
    let mut guard_false = [0; 30];
    let maximum = [0; 30];
    let mut assignment = [0; 30];
    routes[20] = 1;
    guard_false[20] = 1;
    assignment[20] = 1;
    assert!(validate_route_evidence(&routes, &guard_false, &maximum, &assignment).is_ok());

    assignment[20] = 0;
    assert!(validate_route_evidence(&routes, &guard_false, &maximum, &assignment).is_err());

    routes[20] = 0;
    guard_false[20] = 0;
    routes[0] = 1;
    assignment[0] = 1;
    assert!(validate_route_evidence(&routes, &guard_false, &maximum, &assignment).is_err());
}
