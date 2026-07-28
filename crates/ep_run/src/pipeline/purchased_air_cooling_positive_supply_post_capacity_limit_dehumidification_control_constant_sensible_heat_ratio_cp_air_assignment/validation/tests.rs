use ep_model::IdealLoadsAirSystemId;

use super::*;

fn lifecycle() -> PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary
{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentLifecycleSummary {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        state:
            PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
                IdealLoadsAirSystemId(0),
            ),
    }
}

#[test]
fn lifecycle_partition_overflow_fails_closed() {
    let error = checked_sum(&[usize::MAX, 1], "test partition")
        .expect_err("partition overflow must fail closed");
    assert!(error.contains("overflowed"));
}

#[test]
fn lifecycle_route_and_source_counter_corruption_fail_closed() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    state.transition_count = 1;
    state.dehumidification_control_none_case_completed_skip_count = 1;
    assert!(validate_route_partition(&state).is_ok());
    assert!(validate_source_counters(&state).is_ok());

    state.dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_count = 1;
    assert!(
        validate_route_partition(&state)
            .expect_err("two retained routes must fail closed")
            .contains("transition_partition")
    );
    assert!(
        validate_source_counters(&state)
            .expect_err("missing source counters must fail closed")
            .contains("source_site_execution_count")
    );
}

#[test]
fn missing_cp348_and_cp329_predecessors_fail_closed() {
    let lifecycle = lifecycle();
    let error = validate_direct_lifecycle(
        Some(&lifecycle),
        DirectLifecyclePredecessors {
            case_entry_cp348: None,
            mixed_air_cp329: None,
        },
        None,
        None,
    )
    .expect_err("missing CP348 predecessor must fail closed");
    assert!(error.contains("no CP348 evidence"));

    let cp348 =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryLifecycleSummary {
            source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE,
            first_excluded_source:
                PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
            state:
                ep_runtime::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState::new(
                    IdealLoadsAirSystemId(0),
                ),
        };
    let error = validate_direct_lifecycle(
        Some(&lifecycle),
        DirectLifecyclePredecessors {
            case_entry_cp348: Some(&cp348),
            mixed_air_cp329: None,
        },
        None,
        None,
    )
    .expect_err("missing CP329 owner must fail closed");
    assert!(error.contains("no CP329 evidence"));
}
