use ep_model::IdealLoadsAirSystemId;

use super::*;

#[test]
fn route_partition_overflow_fails_closed() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    state.unit_off_skip_count = usize::MAX;
    state.non_cooling_skip_count = 1;
    assert!(validate_route_partition(&state).is_err());
}

#[test]
fn source_counter_mismatch_fails_closed() {
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    state.dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_count = 1;
    state.source_site_execution_count = 4;
    state.cooling_sensible_output_read_count = 1;
    assert!(validate_source_counters(&state).is_err());
}
