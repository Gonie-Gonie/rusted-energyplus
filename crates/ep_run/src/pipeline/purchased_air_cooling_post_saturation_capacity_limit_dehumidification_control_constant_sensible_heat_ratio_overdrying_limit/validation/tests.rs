use ep_model::IdealLoadsAirSystemId;

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp391_validator_depends_only_on_cp390_and_requires_all_routes_inactive() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;

    let system = IdealLoadsAirSystemId(0);
    let mut state = State::new(system);
    let mut predecessor = PredecessorState::new(system);
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_ok());

    state.supply_enthalpy_owned_read_count = 1;
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_err());
    state.supply_enthalpy_owned_read_count = 0;
    state.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count = 1;
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_err());
    state.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count = 0;
    predecessor.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count = 1;
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_err());
}

#[test]
fn ep_run_cp391_rejects_missing_cp390_predecessor_evidence() {
    let lifecycle = Lifecycle {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
        state: State::new(IdealLoadsAirSystemId(0)),
    };
    let error = validate_direct_lifecycle(Some(&lifecycle), None, None, Some(1))
        .expect_err("CP391 must require CP390 evidence");
    assert!(error.contains("CP390 evidence is missing"));
}

#[test]
fn ep_run_cp391_links_exactly_to_cp390_and_rejects_corruption() {
    let snapshot = super::super::test_snapshot(Some(-0.0), false);
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit::test_snapshot(
        Some(-0.0),
        false,
    );
    assert_eq!(snapshot.system, predecessor.system);
    assert_eq!(
        snapshot.parent_call_ordinal,
        predecessor.parent_call_ordinal
    );
    assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);
    assert_eq!(
        inherited_flags(snapshot),
        inherited_predecessor_flags(predecessor)
    );
    assert_eq!(cp389_flags(snapshot), predecessor_cp389_flags(predecessor));
    assert_eq!(cp390_flags(snapshot), predecessor_cp390_flags(predecessor));
    for (index, (left, right)) in predecessor_values(snapshot)
        .into_iter()
        .zip(predecessor_snapshot_values(predecessor))
        .enumerate()
    {
        assert!(
            option_bits_equal(left, right),
            "numeric lineage mismatch at {index}: {left:?} != {right:?}"
        );
    }
    assert!(links_to_predecessor(snapshot, predecessor));

    let mut corrupted = predecessor;
    corrupted.source_order = &["forged-cp390-source-order"];
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
