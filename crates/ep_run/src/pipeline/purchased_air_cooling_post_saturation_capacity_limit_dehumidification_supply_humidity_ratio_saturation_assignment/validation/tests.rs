use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp412_validator_depends_only_on_cp411() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;
}

#[test]
fn ep_run_cp412_rejects_missing_cp411_predecessor_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let result = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1));
    assert!(
        result
            .as_ref()
            .is_err_and(|error| error.contains("CP411 evidence is missing"))
    );
}

#[test]
fn route_evidence_partitions_predecessor_splits_and_both_assignment_arrays() {
    let mut routes = [0; 30];
    let mut guard_false = [0; 30];
    let maximum = [0; 30];
    let mut predecessor_assignment = [0; 30];
    let mut assignment = [0; 30];
    routes[20] = 1;
    guard_false[20] = 1;
    predecessor_assignment[20] = 1;
    assignment[20] = 1;
    assert!(
        validate_route_evidence(
            &routes,
            &guard_false,
            &maximum,
            &predecessor_assignment,
            &assignment,
        )
        .is_ok()
    );

    assignment[20] = 0;
    assert!(
        validate_route_evidence(
            &routes,
            &guard_false,
            &maximum,
            &predecessor_assignment,
            &assignment,
        )
        .is_err()
    );

    routes[20] = 0;
    guard_false[20] = 0;
    predecessor_assignment[20] = 0;
    routes[0] = 1;
    predecessor_assignment[0] = 1;
    assert!(
        validate_route_evidence(
            &routes,
            &guard_false,
            &maximum,
            &predecessor_assignment,
            &assignment,
        )
        .is_err()
    );
}

#[test]
fn public_route_firewall_rejects_private_active_routes() {
    let mut routes = [0; 30];
    routes[21] = 1;
    assert!(ensure_public_routes_only(&routes).is_err());
    routes[21] = 0;
    routes[20] = 1;
    assert!(ensure_public_routes_only(&routes).is_ok());
}
