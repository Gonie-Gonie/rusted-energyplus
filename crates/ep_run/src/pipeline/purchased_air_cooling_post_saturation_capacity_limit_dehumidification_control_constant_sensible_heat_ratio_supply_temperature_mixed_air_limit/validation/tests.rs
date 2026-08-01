use ep_model::IdealLoadsAirSystemId;
use ep_runtime::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState,
};

use super::*;

type DirectLifecycleValidator = fn(
    Option<&Lifecycle>,
    Option<&PredecessorLifecycle>,
    Option<&PurchasedAirInitLifecycleSummary>,
    Option<usize>,
) -> Result<(), String>;

#[test]
fn public_cp390_validator_has_no_cp329_dependency_and_requires_all_routes_inactive() {
    let validator: DirectLifecycleValidator = validate_direct_lifecycle;
    let _ = validator;

    let system = IdealLoadsAirSystemId(0);
    let mut state = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitRuntimeState::new(system);
    let mut predecessor = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureAssignmentRuntimeState::new(system);
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_ok());

    state.supply_temperature_owned_read_count = 1;
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_err());
    state.supply_temperature_owned_read_count = 0;
    state.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count = 1;
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_err());
    state.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count = 0;
    predecessor.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_count = 1;
    assert!(validate_all_public_inactive_contract(&state, &predecessor).is_err());
}

#[test]
fn ep_run_cp390_rejects_cp389_source_order_corruption() {
    let snapshot = super::super::test_snapshot(Some(-0.0), false);
    let predecessor = crate::pipeline::purchased_air_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment::test_snapshot(
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
    assert_eq!(cp389_flags(snapshot), predecessor_flags(predecessor));
    assert_eq!(
        snapshot.predecessor_dehumidification_control_type,
        predecessor.predecessor_dehumidification_control_type
    );
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
    corrupted.source_order = &["forged-cp389-source-order"];
    assert!(!links_to_predecessor(snapshot, corrupted));
}
