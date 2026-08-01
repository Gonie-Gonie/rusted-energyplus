use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_lifecycle_summary,
};

#[test]
fn binding_releases_cp390_after_cp389_without_mutating_direct_numerical_state() {
    for (limit, humidity_ratio, availability, capacity) in [
        (IdealLoadsLimit::NoLimit, 0.008, 0.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.008, 1.0, 5_000.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 500.0),
    ] {
        let (runtime, output) = super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
            limit,
            humidity_ratio,
            availability,
            capacity,
        );
        let predecessor = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment;
        let snapshot = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot
                .predecessor_resulting_supply_temperature_c
                .map(f64::to_bits),
            predecessor.resulting_supply_temperature_c.map(f64::to_bits)
        );
        assert_eq!(
            snapshot.preexisting_supply_temperature_c.map(f64::to_bits),
            predecessor.resulting_supply_temperature_c.map(f64::to_bits)
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            predecessor.resulting_supply_temperature_c.map(f64::to_bits)
        );
        assert_source_local_skip(snapshot);

        let system = output.initialization.system;
        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP390 lifecycle");
        let predecessor_lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP389 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1);
        assert_eq!(lifecycle.state.source_site_execution_count, 0);
        assert_eq!(
            lifecycle.state.predecessor_route_counts,
            predecessor_lifecycle.state.predecessor_route_counts
        );
        assert_eq!(
            lifecycle.state.cp389_supply_temperature_state_owner_count,
            predecessor_lifecycle
                .state
                .cp379_supply_temperature_state_owner_count
        );
        assert_eq!(
            lifecycle
                .state
                .unchanged_supply_temperature_preservation_count,
            lifecycle.state.cp389_supply_temperature_state_owner_count
        );
    }
}

fn assert_source_local_skip(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyTemperatureMixedAirLimitSnapshot,
) {
    for flag in [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_executed,
        snapshot.cp389_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_read,
        snapshot.cp329_retained_mixed_air_temperature_owned_read,
        snapshot.cp389_mixed_air_temperature_bit_corroborated,
        snapshot.mixed_air_temperature_for_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.supply_temperature_assignment_performed,
    ] {
        assert!(!flag);
    }
    for value in [
        snapshot.supply_temperature_before_mixed_air_limit_c,
        snapshot.mixed_air_temperature_c,
        snapshot.minimum_supply_temperature_c,
        snapshot.assigned_supply_temperature_c,
    ] {
        assert!(value.is_none());
    }
}
