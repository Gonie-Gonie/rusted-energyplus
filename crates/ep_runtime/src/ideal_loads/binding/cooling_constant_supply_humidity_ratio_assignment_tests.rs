use super::*;
use crate::ideal_loads::{
    cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn binding_orders_cp365_after_cp364_as_an_exact_null_c0_skip() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let case = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        assert!(case.is_some(), "CP365 binding fixture must succeed");
        let Some((runtime, output)) = case else {
            return;
        };
        let predecessor = output.calculation_cooling_constant_supply_humidity_ratio_case_entry;
        let snapshot = output.calculation_cooling_constant_supply_humidity_ratio_assignment;
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
            )
        );
        assert!(
            cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(
            !snapshot.dehumidification_control_constant_supply_humidity_ratio_assignment_executed
        );
        assert!(!snapshot.minimum_cooling_supply_air_humidity_ratio_read);
        assert!(snapshot.minimum_cooling_supply_air_humidity_ratio.is_none());
        assert!(!snapshot.supply_humidity_ratio_assigned);
        assert!(snapshot.assigned_supply_humidity_ratio.is_none());
        assert!(snapshot.resulting_supply_humidity_ratio.is_none());

        let summary =
            purchased_air_calc_cooling_constant_supply_humidity_ratio_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            );
        assert!(summary.is_ok(), "CP365 lifecycle summary must be available");
        let Ok(summary) = summary else {
            return;
        };
        assert_eq!(summary.state.transition_count, 1);
        assert_eq!(
            summary
                .state
                .dehumidification_control_none_case_completed_skip_count,
            1
        );
        assert_eq!(
            summary
                .state
                .dehumidification_control_constant_supply_humidity_ratio_assignment_count,
            0
        );
        assert_eq!(summary.state.source_site_execution_count, 0);
        assert_eq!(
            summary
                .state
                .minimum_cooling_supply_air_humidity_ratio_read_count,
            0
        );
        assert_eq!(summary.state.supply_humidity_ratio_assignment_count, 0);

        let numerical_owner = output
            .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .assigned_supply_humidity_ratio;
        let numerical = output
            .coupling
            .purchased_air
            .supply_node_update
            .humidity_ratio;
        assert_eq!(numerical_owner.map(f64::to_bits), Some(numerical.to_bits()));
    }
}

#[test]
fn binding_cp365_preserves_u_n_p_and_c0_complete_null_routes() {
    for (load, availability, capacity, expected) in [
        (3_000.0, 0.0, None, (true, false, false, false)),
        (0.0, 1.0, None, (false, true, false, false)),
        (3_000.0, 1.0, Some(-0.0), (false, false, true, false)),
        (3_000.0, 1.0, Some(1.0), (false, false, false, true)),
    ] {
        let limit = if capacity.is_some() {
            IdealLoadsLimit::LimitCapacity
        } else {
            IdealLoadsLimit::NoLimit
        };
        let case = run_case(limit, capacity, load, availability);
        assert!(case.is_some(), "CP365 route fixture must succeed");
        let Some((_, output)) = case else {
            return;
        };
        let snapshot = output.calculation_cooling_constant_supply_humidity_ratio_assignment;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
                snapshot.dehumidification_control_none_case_completed_skip,
            ),
            expected
        );
        assert!(
            cooling_constant_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(!snapshot.minimum_cooling_supply_air_humidity_ratio_read);
        assert!(!snapshot.supply_humidity_ratio_assigned);
        assert!(snapshot.resulting_supply_humidity_ratio.is_none());
    }
}
