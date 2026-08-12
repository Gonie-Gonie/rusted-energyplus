use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&OwnerLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp419_validator_requires_cp418_and_cp329_evidence() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_GUARD_ELSE_BRANCH_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, None, Some(1))
        .expect_err("CP419 must require CP418 evidence");
    assert!(error.contains("CP418 evidence is missing"));
}

#[test]
fn conceptual_cp419_contract_is_54_routes_49_inactive_five_assignments_and_15_sites() {
    assert_eq!(
        (
            54 - ASSIGNMENT_LOGICAL_INDICES.len(),
            ASSIGNMENT_LOGICAL_INDICES.len(),
            ASSIGNMENT_LOGICAL_INDICES.len() * EXPECTED_SOURCE_ORDER.len(),
        ),
        (49, 5, 15),
    );
}

#[test]
fn cp419_direct_validator_uses_local_lineage_without_recursive_exact_characterization() {
    let source = include_str!("../validation.rs");
    let lineage = include_str!("lineage.rs");
    for forbidden in [
        "private_cooling_post_saturation",
        "snapshot_is_exact_direct_release",
        "completed_direct_cooling_post_saturation",
    ] {
        assert!(!source.contains(forbidden), "{forbidden}");
        assert!(!lineage.contains(forbidden), "{forbidden}");
    }
}
