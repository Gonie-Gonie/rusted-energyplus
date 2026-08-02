use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp393_validator_depends_only_on_cp392_and_requires_all_routes_inactive() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;

    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = PredecessorState::new(system);
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_ok());

    state.dehumidification_control_constant_sensible_heat_ratio_case_break_count = 1;
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_err());
    state.dehumidification_control_constant_sensible_heat_ratio_case_break_count = 0;
    predecessor.dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count = 1;
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_err());
}

#[test]
fn ep_run_cp393_rejects_missing_cp392_predecessor_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_BREAK_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1))
        .expect_err("CP393 must require CP392 evidence");
    assert!(error.contains("CP392 evidence is missing"));
}

#[test]
fn ep_run_cp393_links_exactly_to_cp392_and_rejects_corruption() {
    let snapshot = super::super::test_snapshot(Some(-0.0), false);
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment::test_snapshot(
        Some(-0.0),
        false,
    );
    assert_eq!(snapshot.system, predecessor.system);
    assert_eq!(
        snapshot.parent_call_ordinal,
        predecessor.parent_call_ordinal
    );
    assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);
    assert!(links_to_predecessor(snapshot, predecessor));
    assert!(carriers_are_preserved(snapshot, predecessor));

    let mut corrupted = predecessor;
    corrupted.source_order = &["forged-cp392-source-order"];
    assert!(!links_to_predecessor(snapshot, corrupted));

    let mut corrupted = predecessor;
    corrupted.resulting_supply_enthalpy_j_per_kg = Some(f64::from_bits(
        corrupted
            .resulting_supply_enthalpy_j_per_kg
            .expect("retained enthalpy")
            .to_bits()
            ^ 1,
    ));
    assert!(!links_to_predecessor(snapshot, corrupted));
}
