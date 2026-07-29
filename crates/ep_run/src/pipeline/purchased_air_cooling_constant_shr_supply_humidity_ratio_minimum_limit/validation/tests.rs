use ep_model::IdealLoadsAirSystemId;

use super::*;

#[test]
fn route_partition_overflow_fails_closed() {
    let mut state =
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    state.unit_off_skip_count = usize::MAX;
    state.non_cooling_skip_count = 1;
    assert!(validate_route_partition(&state).is_err());
}

#[test]
fn source_counter_mismatch_fails_closed() {
    let mut state =
        PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioMinimumLimitRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
    state.dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_minimum_limit_count =
        1;
    state.source_site_execution_count = 4;
    state.supply_humidity_ratio_for_minimum_limit_maximum_read_count = 1;
    assert!(validate_source_counters(&state).is_err());
}
