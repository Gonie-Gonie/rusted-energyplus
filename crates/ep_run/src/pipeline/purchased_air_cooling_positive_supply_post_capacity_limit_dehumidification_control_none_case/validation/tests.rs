use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState;

use super::*;

#[test]
fn lifecycle_partition_overflow_fails_closed() {
    let error = checked_sum(&[usize::MAX, 1], "test partition")
        .expect_err("partition overflow must fail closed");
    assert!(error.contains("overflowed"));
}

#[test]
fn bit_comparison_distinguishes_signed_zero() {
    assert!(option_bits_equal(Some(-0.0), Some(-0.0)));
    assert!(!option_bits_equal(Some(-0.0), Some(0.0)));
}

#[test]
fn lifecycle_route_partition_corruption_fails_closed() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(0),
        );
    state.transition_count = 1;
    state.unit_off_skip_count = 1;
    assert!(validate_route_partition(&state).is_ok());

    state.non_cooling_skip_count = 1;
    let error = validate_route_partition(&state)
        .expect_err("two retained routes for one transition must fail closed");
    assert!(error.contains("transition_partition"));
}

#[test]
fn missing_cp346_predecessor_and_cp329_owner_evidence_fail_closed() {
    let system = ep_model::IdealLoadsAirSystemId(0);
    let lifecycle =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_NONE_CASE_FIRST_EXCLUDED_SOURCE,
            state:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState::new(
                    system,
                ),
        };
    let error = validate_direct_lifecycle(
        Some(&lifecycle),
        DirectLifecyclePredecessors {
            control_switch_cp346: None,
            mixed_air_cp329: None,
        },
        None,
        None,
    )
    .expect_err("missing CP346 predecessor must fail closed");
    assert!(error.contains("no CP346 evidence"));

    let predecessor =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_SWITCH_FIRST_EXCLUDED_SOURCE,
            state:
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState::new(
                    system,
                ),
        };
    let error = validate_direct_lifecycle(
        Some(&lifecycle),
        DirectLifecyclePredecessors {
            control_switch_cp346: Some(&predecessor),
            mixed_air_cp329: None,
        },
        None,
        None,
    )
    .expect_err("missing CP329 owner must fail closed");
    assert!(error.contains("no CP329 evidence"));
}
