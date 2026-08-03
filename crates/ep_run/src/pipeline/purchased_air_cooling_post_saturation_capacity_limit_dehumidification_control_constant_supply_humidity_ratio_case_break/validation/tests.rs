use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp409_validator_depends_only_on_cp408() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;
}

#[test]
fn ep_run_cp409_rejects_missing_cp408_predecessor_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let result = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1));
    assert!(
        result
            .as_ref()
            .is_err_and(|error| error.contains("CP408 evidence is missing"))
    );
}

#[test]
fn compact_snapshot_exposes_active_and_inactive_break_shapes() {
    let active = super::super::test_snapshot(Some(-0.0), true);
    let inactive = super::super::test_snapshot(Some(-0.0), false);
    assert!(
        active
            .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break
    );
    assert!(
        !inactive
            .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break
    );
}

#[test]
fn route_evidence_is_partitioned_per_predecessor_route() {
    let mut routes = [0; 30];
    let mut guard_false = [0; 30];
    let maximum = [0; 30];
    routes[20] = 1;
    guard_false[20] = 1;
    assert!(validate_route_evidence(&routes, &guard_false, &maximum).is_ok());

    guard_false[20] = 0;
    assert!(validate_route_evidence(&routes, &guard_false, &maximum).is_err());

    routes[20] = 0;
    routes[0] = 1;
    guard_false[0] = 1;
    assert!(validate_route_evidence(&routes, &guard_false, &maximum).is_err());
}
