use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp395_validator_depends_only_on_cp394_and_requires_active_sites_inactive() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;

    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    assert!(validate_all_public_inactive_contract(&state).is_ok());
    state.dehumidification_control_humidistat_supply_humidity_ratio_assignment_count = 1;
    assert!(validate_all_public_inactive_contract(&state).is_err());
    state.dehumidification_control_humidistat_supply_humidity_ratio_assignment_count = 0;
    state.psychrometric_supply_humidity_ratio_evaluation_count = 1;
    assert!(validate_all_public_inactive_contract(&state).is_err());
}

#[test]
fn ep_run_cp395_rejects_missing_cp394_predecessor_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1))
        .expect_err("CP395 must require CP394 evidence");
    assert!(error.contains("CP394 evidence is missing"));
}

#[test]
fn ep_run_cp395_links_recursive_cp393_and_terminal_cp394_carriers() {
    let snapshot = super::super::test_snapshot(Some(-0.0));
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry::test_snapshot(
        Some(-0.0),
        true,
    );
    assert!(links_to_predecessor(snapshot, predecessor));
    assert!(carriers_are_preserved(snapshot, predecessor));

    let mut corrupted = predecessor;
    corrupted.predecessor_cp393_resulting_supply_enthalpy_j_per_kg = Some(f64::from_bits(
        corrupted
            .predecessor_cp393_resulting_supply_enthalpy_j_per_kg
            .expect("recursive enthalpy")
            .to_bits()
            ^ 1,
    ));
    assert!(!links_to_predecessor(snapshot, corrupted));

    let direct_skip = super::super::test_snapshot(None);
    assert!(direct_skip_shape(direct_skip));
}
