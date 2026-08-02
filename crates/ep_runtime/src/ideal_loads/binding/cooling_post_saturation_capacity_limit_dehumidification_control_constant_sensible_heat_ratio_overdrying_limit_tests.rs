use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_lifecycle_summary,
};

#[test]
fn binding_releases_cp391_after_cp390_without_mutating_direct_numerical_state() {
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
        let predecessor = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit;
        let snapshot = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot
                .predecessor_cp390_resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            predecessor
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits)
        );
        assert_eq!(
            snapshot
                .predecessor_cp390_resulting_supply_temperature_c
                .map(f64::to_bits),
            predecessor.resulting_supply_temperature_c.map(f64::to_bits)
        );
        assert_eq!(
            snapshot
                .preexisting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            predecessor
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits)
        );
        assert_eq!(
            snapshot
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            predecessor
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits)
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.map(f64::to_bits),
            predecessor.resulting_supply_temperature_c.map(f64::to_bits)
        );
        assert_source_local_skip(snapshot);

        let system = output.initialization.system;
        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP391 lifecycle");
        let predecessor_lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP390 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1);
        assert_eq!(lifecycle.state.source_site_execution_count, 0);
        assert_eq!(
            lifecycle.state.predecessor_route_counts,
            predecessor_lifecycle.state.predecessor_route_counts
        );
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count,
            predecessor_lifecycle
                .state
                .dehumidification_control_constant_sensible_heat_ratio_supply_temperature_mixed_air_limit_count
        );
        assert_eq!(
            lifecycle.state.unchanged_supply_enthalpy_preservation_count,
            lifecycle.state.cp390_supply_enthalpy_state_owner_count
        );
    }
}

fn assert_source_local_skip(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
) {
    for flag in [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed,
        snapshot.cp390_retained_supply_enthalpy_owned_read,
        snapshot.supply_enthalpy_for_overdrying_limit_maximum_read,
        snapshot.cp390_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_minimum_humidity_ratio_enthalpy_read,
        snapshot.psychrometric_minimum_supply_enthalpy_evaluated,
        snapshot.source_shaped_two_argument_maximum_evaluated,
        snapshot.supply_enthalpy_assignment_performed,
    ] {
        assert!(!flag);
    }
    for value in [
        snapshot.supply_enthalpy_before_overdrying_limit_j_per_kg,
        snapshot.supply_temperature_c,
        snapshot.psychrometric_minimum_supply_enthalpy_j_per_kg,
        snapshot.maximum_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
    ] {
        assert!(value.is_none());
    }
}
