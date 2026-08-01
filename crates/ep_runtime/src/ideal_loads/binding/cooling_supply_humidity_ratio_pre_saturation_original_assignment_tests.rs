use super::*;
use crate::ideal_loads::{
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn binding_orders_cp375_then_cp376_and_keeps_the_numerical_owner_unchanged() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let case = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        assert!(case.is_some(), "CP376 binding fixture must succeed");
        let Some((runtime, output)) = case else {
            return;
        };
        let predecessor = output
            .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment;
        let owner = output
            .calculation_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
        let snapshot = output
            .calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment;

        assert_eq!(
            (
                snapshot.system,
                snapshot.parent_call_ordinal,
                snapshot.controlled_zone,
            ),
            (
                predecessor.system,
                predecessor.parent_call_ordinal,
                predecessor.controlled_zone,
            ),
        );
        assert!(
            cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.humidification_control_guard_false_fallthrough);
        assert!(!snapshot.predecessor_purchased_air_supply_humidity_ratio_assignment_performed);
        assert_eq!(snapshot.predecessor_resulting_supply_humidity_ratio, None);
        assert!(!snapshot.cp375_maximum_assignment_owned_read);
        assert!(snapshot.cp347_none_case_owned_read);
        for flag in [
            snapshot.cp356_constant_shr_owned_read,
            snapshot.cp362_humidistat_owned_read,
            snapshot.cp365_constant_supply_humidity_ratio_owned_read,
        ] {
            assert!(!flag);
        }
        assert!(snapshot.purchased_air_supply_humidity_ratio_read);
        assert!(snapshot.local_supply_humidity_ratio_original_assignment_performed);
        let owner_bits = owner
            .resulting_supply_humidity_ratio
            .map(f64::to_bits)
            .expect("same-call CP347 direct owner");
        for value in [
            snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
            snapshot.assigned_supply_humidity_ratio_original,
            snapshot.resulting_supply_humidity_ratio_original,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(owner_bits));
        }

        let summary = purchased_air_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP376 lifecycle summary");
        assert_eq!(summary.state.transition_count, 1);
        assert_eq!(summary.state.source_site_execution_count, 2);

        let numerical_owner = output
            .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .assigned_supply_humidity_ratio;
        let numerical_result = output
            .coupling
            .purchased_air
            .supply_node_update
            .humidity_ratio;
        assert_eq!(
            numerical_owner.map(f64::to_bits),
            Some(numerical_result.to_bits()),
        );
    }
}

#[test]
fn binding_cp376_preserves_u_n_and_p_as_zero_site_owner_nonreads() {
    for (cooling_limit, maximum_capacity_w, independent_load_w, availability, route) in [
        (
            IdealLoadsLimit::NoLimit,
            None,
            3_000.0,
            0.0,
            (true, false, false),
        ),
        (
            IdealLoadsLimit::NoLimit,
            None,
            0.0,
            1.0,
            (false, true, false),
        ),
        (
            IdealLoadsLimit::LimitCapacity,
            Some(0.0),
            3_000.0,
            1.0,
            (false, false, true),
        ),
    ] {
        let case = run_case(
            cooling_limit,
            maximum_capacity_w,
            independent_load_w,
            availability,
        );
        assert!(case.is_some(), "CP376 skipped binding fixture must succeed");
        let Some((runtime, output)) = case else {
            return;
        };
        let snapshot = output
            .calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
        assert!(
            cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            route,
        );
        for flag in [
            snapshot.heating_availability_guard_false_fallthrough,
            snapshot.humidification_control_guard_false_fallthrough,
            snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
            snapshot.dehumidification_control_none_maximum_assignment_executed,
            snapshot.dehumidification_control_guard_false_fallthrough,
            snapshot.cp375_maximum_assignment_owned_read,
            snapshot.cp347_none_case_owned_read,
            snapshot.cp356_constant_shr_owned_read,
            snapshot.cp362_humidistat_owned_read,
            snapshot.cp365_constant_supply_humidity_ratio_owned_read,
            snapshot.purchased_air_supply_humidity_ratio_read,
            snapshot.local_supply_humidity_ratio_original_assignment_performed,
        ] {
            assert!(!flag);
        }
        for value in [
            snapshot.predecessor_resulting_supply_humidity_ratio,
            snapshot.purchased_air_supply_humidity_ratio_before_saturation_check,
            snapshot.assigned_supply_humidity_ratio_original,
            snapshot.resulting_supply_humidity_ratio_original,
        ] {
            assert!(value.is_none());
        }

        let state =
            purchased_air_calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP376 skipped lifecycle")
            .state;
        assert_eq!(state.transition_count, 1);
        for count in [
            state.source_site_execution_count,
            state.purchased_air_supply_humidity_ratio_before_saturation_limit_read_count,
            state.local_original_supply_humidity_ratio_before_saturation_limit_assignment_count,
            state.cp375_maximum_assignment_owner_count,
            state.cp347_none_case_owner_count,
            state.cp356_constant_shr_owner_count,
            state.cp362_humidistat_owner_count,
            state.cp365_constant_supply_humidity_ratio_owner_count,
        ] {
            assert_eq!(count, 0);
        }
    }
}
