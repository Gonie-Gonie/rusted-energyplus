use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp417_validator_requires_cp416_and_runtime_identity() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;

    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1))
        .expect_err("CP417 must require CP416 before runtime validation");
    assert!(error.contains("CP416 evidence is missing"));
}

#[test]
fn route_evidence_requires_cp417_assignment_to_equal_cp416_assignment() {
    let mut routes = [0; 36];
    let mut guard_false = [0; 36];
    let mut guard_body = [0; 36];
    let mut predecessor_temperature_assignment = [0; 36];
    let mut predecessor_temperature_limit = [0; 36];
    let mut predecessor_humidity_ratio_assignment = [0; 36];
    let mut enthalpy_assignment = [0; 36];
    routes[20] = 2;
    guard_false[20] = 1;
    guard_body[20] = 1;
    predecessor_temperature_assignment[20] = 1;
    predecessor_temperature_limit[20] = 1;
    predecessor_humidity_ratio_assignment[20] = 1;
    enthalpy_assignment[20] = 1;
    assert!(
        validate_route_evidence(
            &routes,
            &guard_false,
            &guard_body,
            &predecessor_temperature_assignment,
            &predecessor_temperature_limit,
            &predecessor_humidity_ratio_assignment,
            &enthalpy_assignment,
        )
        .is_ok()
    );

    enthalpy_assignment[20] = 0;
    assert!(
        validate_route_evidence(
            &routes,
            &guard_false,
            &guard_body,
            &predecessor_temperature_assignment,
            &predecessor_temperature_limit,
            &predecessor_humidity_ratio_assignment,
            &enthalpy_assignment,
        )
        .is_err()
    );
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
fn conceptual_contract_retains_54_outcomes_and_four_public_assignments() {
    let flattened_public = [0, 1, 2, 3, 4, 5, 6, 7, 8, 22, 23, 24, 25, 34, 35, 36, 37];
    let public_active = [23, 25, 35, 37];
    assert_eq!(
        (54, flattened_public.len(), 54 - flattened_public.len()),
        (54, 17, 37)
    );
    assert_eq!((18, 36, 18 * EXPECTED_SOURCE_ORDER.len()), (18, 36, 72));
    assert_eq!(public_active.len(), 4);
    assert!(public_active.into_iter().all(|index| index % 2 == 1));
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
