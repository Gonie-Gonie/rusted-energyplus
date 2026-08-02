use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_lifecycle_summary,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_lifecycle_summary,
};
use ep_model::DehumidificationControlType;

#[test]
fn binding_places_cp397_after_cp396_before_unchanged_numerical_coupling() {
    let mut saw_active_none_entry = false;
    let mut saw_inactive_none_entry = false;
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
        let predecessor = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break;
        let snapshot = output.calculation_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_snapshot_is_exact_direct_release(snapshot)
        );
        let expected_active = predecessor.predecessor_dehumidification_control_switch_dispatched
            && predecessor.predecessor_dehumidification_control_type
                == Some(DehumidificationControlType::None);
        assert_eq!(
            snapshot.dehumidification_control_none_case_entered,
            expected_active
        );
        saw_active_none_entry |= expected_active;
        saw_inactive_none_entry |= !expected_active;
        assert_carriers(snapshot, predecessor);

        let system = output.initialization.system;
        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_none_case_entry_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP397 lifecycle");
        let predecessor_lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_lifecycle_summary(
            &runtime,
            system,
        )
        .expect("CP396 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.inactive_transition_count,
            usize::from(!expected_active)
        );
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_none_case_entry_count,
            usize::from(expected_active)
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            usize::from(expected_active)
        );
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
                .is_finite(),
            "CP397 evidence must not replace the numerical PurchasedAir output"
        );
    }
    assert!(saw_active_none_entry, "fixture set must enter CP397");
    assert!(saw_inactive_none_entry, "fixture set must also skip CP397");
}

fn assert_carriers(
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntrySnapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakSnapshot,
) {
    for (left, right) in [
        (
            snapshot.predecessor_cp396_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        ),
        (
            snapshot.predecessor_cp396_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.predecessor_cp396_resulting_supply_temperature_c,
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
