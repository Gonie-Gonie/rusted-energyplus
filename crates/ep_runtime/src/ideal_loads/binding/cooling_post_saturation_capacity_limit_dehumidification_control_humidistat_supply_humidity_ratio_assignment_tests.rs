use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_lifecycle_summary,
};

#[test]
fn binding_places_cp395_after_cp394_before_unchanged_numerical_coupling() {
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
        let predecessor = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry;
        let snapshot = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert!(
            !snapshot.dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed
        );
        for flag in [
            snapshot.cp394_retained_supply_temperature_owned_read,
            snapshot.supply_temperature_for_humidity_ratio_inversion_read,
            snapshot.cp394_retained_supply_enthalpy_owned_read,
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
        ] {
            assert!(value.is_none());
        }
        assert_recursive_carriers(snapshot, predecessor);

        let system = output.initialization.system;
        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_lifecycle_summary(&runtime, system)
            .expect("CP395 lifecycle");
        let predecessor_lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_lifecycle_summary(&runtime, system)
            .expect("CP394 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1);
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_humidistat_supply_humidity_ratio_assignment_count,
            0
        );
        assert_eq!(lifecycle.state.source_site_execution_count, 0);
        assert_eq!(
            lifecycle.state.predecessor_route_counts,
            predecessor_lifecycle.state.predecessor_route_counts
        );
        assert!(
            output
                .coupling
                .purchased_air
                .supply_node_update
                .humidity_ratio
                .is_finite()
        );
    }
}

fn assert_recursive_carriers(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot,
) {
    for (left, right) in [
        (
            snapshot.predecessor_cp393_resulting_supply_humidity_ratio,
            predecessor.predecessor_cp393_resulting_supply_humidity_ratio,
        ),
        (
            snapshot.predecessor_cp393_resulting_supply_enthalpy_j_per_kg,
            predecessor.predecessor_cp393_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.predecessor_cp393_resulting_supply_temperature_c,
            predecessor.predecessor_cp393_resulting_supply_temperature_c,
        ),
        (
            snapshot.predecessor_cp394_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        ),
        (
            snapshot.predecessor_cp394_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.predecessor_cp394_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        ),
        (
            snapshot.resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        ),
        (
            snapshot.resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        ),
    ] {
        assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
    }
}
