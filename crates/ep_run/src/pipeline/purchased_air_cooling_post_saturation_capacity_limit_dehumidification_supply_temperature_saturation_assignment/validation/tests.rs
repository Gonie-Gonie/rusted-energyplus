use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp414_validator_depends_only_on_cp413() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;
}

#[test]
fn ep_run_cp414_rejects_missing_cp413_predecessor_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_TEMPERATURE_SATURATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let result = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1));
    assert!(
        result
            .as_ref()
            .is_err_and(|error| error.contains("CP413 evidence is missing"))
    );
}

#[test]
fn route_evidence_requires_assignment_to_equal_cp413_body_entry() {
    let mut routes = [0; 36];
    let mut guard_false = [0; 36];
    let mut guard_body = [0; 36];
    let mut assignment = [0; 36];
    routes[20] = 2;
    guard_false[20] = 1;
    guard_body[20] = 1;
    assignment[20] = 1;
    assert!(validate_route_evidence(&routes, &guard_false, &guard_body, &assignment).is_ok());

    assignment[20] = 0;
    assert!(validate_route_evidence(&routes, &guard_false, &guard_body, &assignment).is_err());

    routes[20] = 1;
    assert!(validate_route_evidence(&routes, &guard_false, &guard_body, &assignment).is_err());
}

#[test]
fn public_route_firewall_rejects_private_base_routes() {
    let mut routes = [0; 36];
    routes[22] = 1;
    assert!(ensure_public_routes_only(&routes).is_err());
    routes[22] = 0;
    routes[20] = 1;
    assert!(ensure_public_routes_only(&routes).is_ok());
}

#[test]
fn conceptual_contract_retains_54_outcomes_and_four_public_body_assignments() {
    let flattened_public = [0, 1, 2, 3, 4, 5, 6, 7, 8, 22, 23, 24, 25, 34, 35, 36, 37];
    let public_body = [23, 25, 35, 37];
    assert_eq!(
        (54, flattened_public.len(), 54 - flattened_public.len()),
        (54, 17, 37)
    );
    assert_eq!((18, 36, 18 * EXPECTED_SOURCE_ORDER.len()), (18, 36, 72));
    assert_eq!(public_body.len(), 4);
    assert!(public_body.into_iter().all(|index| index % 2 == 1));
}

#[test]
fn overflow_helpers_fail_closed() {
    assert!(checked_sum(&[usize::MAX, 1], "test partition").is_err());
    assert!(
        usize::MAX
            .checked_mul(EXPECTED_SOURCE_ORDER.len())
            .is_none()
    );
}
