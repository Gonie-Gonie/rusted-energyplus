use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_lifecycle_summary,
};

#[test]
fn binding_releases_cp392_after_cp391_without_mutating_direct_numerical_state() {
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
        let predecessor = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit;
        let snapshot = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot
                .predecessor_cp391_resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits),
            predecessor
                .resulting_supply_enthalpy_j_per_kg
                .map(f64::to_bits)
        );
        assert_eq!(
            snapshot
                .predecessor_cp391_resulting_supply_temperature_c
                .map(f64::to_bits),
            predecessor.resulting_supply_temperature_c.map(f64::to_bits)
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
        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP392 lifecycle");
        let predecessor_lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP391 lifecycle");
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
                .dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_count,
            predecessor_lifecycle
                .state
                .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count
        );
        assert_eq!(
            lifecycle
                .state
                .unchanged_supply_temperature_preservation_count,
            lifecycle.state.cp391_supply_temperature_state_owner_count
        );
        assert_eq!(
            lifecycle.state.unchanged_supply_enthalpy_preservation_count,
            lifecycle.state.cp391_supply_enthalpy_state_owner_count
        );
    }
}

fn assert_source_local_skip(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot,
) {
    assert_eq!(
        snapshot.cp391_retained_supply_temperature_state_owned,
        snapshot
            .predecessor_cp391_resulting_supply_temperature_c
            .is_some()
    );
    assert_eq!(
        snapshot.cp391_retained_supply_enthalpy_state_owned,
        snapshot
            .predecessor_cp391_resulting_supply_enthalpy_j_per_kg
            .is_some()
    );
    for flag in [
        snapshot.dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed,
        snapshot.cp391_retained_supply_temperature_owned_read,
        snapshot.supply_temperature_for_humidity_ratio_inversion_read,
        snapshot.cp391_retained_supply_enthalpy_owned_read,
        snapshot.supply_enthalpy_for_humidity_ratio_inversion_read,
        snapshot.psychrometric_supply_humidity_ratio_evaluated,
        snapshot.supply_humidity_ratio_assignment_performed,
    ] {
        assert!(!flag);
    }
    for value in [
        snapshot.supply_temperature_c,
        snapshot.supply_enthalpy_j_per_kg,
        snapshot.psychrometric_supply_humidity_ratio,
        snapshot.assigned_supply_humidity_ratio,
        snapshot.resulting_supply_humidity_ratio,
    ] {
        assert!(value.is_none());
    }
}
