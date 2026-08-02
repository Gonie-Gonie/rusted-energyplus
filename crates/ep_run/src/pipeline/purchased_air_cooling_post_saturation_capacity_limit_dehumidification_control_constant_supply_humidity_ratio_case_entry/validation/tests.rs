use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp398_validator_depends_only_on_cp397_and_accepts_direct_active_routes() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;

    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = PredecessorState::new(system);
    state.transition_count = 1;
    state.dehumidification_control_constant_supply_humidity_ratio_case_entry_count = 1;
    state.predecessor_route_counts[20] = 1;
    state.source_site_execution_count = 1;
    predecessor.transition_count = 1;
    predecessor.predecessor_route_counts[20] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_ok());

    state.predecessor_route_counts[27] = 1;
    predecessor.predecessor_route_counts[27] = 1;
    assert!(validate_public_route_contract(&state, &predecessor).is_err());
}

#[test]
fn ep_run_cp398_rejects_missing_cp397_predecessor_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1))
        .expect_err("CP398 must require CP397 evidence");
    assert!(error.contains("CP397 evidence is missing"));
}

#[test]
fn ep_run_cp398_links_exactly_to_cp397_and_rejects_corruption() {
    let snapshot = super::super::test_snapshot(Some(-0.0), true);
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry::test_snapshot(
        Some(-0.0),
        true,
    );
    assert_eq!(snapshot.system, predecessor.system);
    assert_eq!(
        snapshot.parent_call_ordinal,
        predecessor.parent_call_ordinal
    );
    assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);
    assert!(links_to_predecessor(snapshot, predecessor));
    assert!(carriers_are_preserved(snapshot, predecessor));
    assert!(shared_case_shape_is_exact(snapshot, predecessor));

    let mut corrupted = predecessor;
    corrupted.source_order = &["forged-cp397-source-order"];
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
