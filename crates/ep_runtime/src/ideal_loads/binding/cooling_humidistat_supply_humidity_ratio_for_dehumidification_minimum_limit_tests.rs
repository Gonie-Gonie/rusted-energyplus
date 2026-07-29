use super::*;
use crate::ideal_loads::{
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn scheduled_binding_places_cp361_after_cp360_as_an_exact_null_c0_skip() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let Some((runtime, output)) = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0)
        else {
            return;
        };
        let predecessor = output
            .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment;
        let snapshot = output
            .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit;
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
            cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(
            !snapshot
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_executed
        );
        for flag in [
            snapshot.supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read,
            snapshot.minimum_cooling_supply_air_humidity_ratio_for_maximum_read,
            snapshot.source_shaped_two_argument_maximum_evaluated,
            snapshot.supply_humidity_ratio_for_dehumidification_assignment_performed,
        ] {
            assert!(!flag);
        }
        for value in [
            snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
            snapshot.supply_humidity_ratio_for_dehumidification_before_minimum_limit,
            snapshot.minimum_cooling_supply_air_humidity_ratio,
            snapshot.maximum_supply_humidity_ratio_for_dehumidification,
            snapshot.assigned_supply_humidity_ratio_for_dehumidification,
            snapshot.resulting_supply_humidity_ratio_for_dehumidification,
        ] {
            assert!(value.is_none());
        }

        let summary =
            purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_lifecycle_summary(
                &runtime,
                output.initialization.system,
            );
        assert!(summary.is_ok());
        let Ok(summary) = summary else {
            return;
        };
        let state = &summary.state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(
            state.dehumidification_control_none_case_completed_skip_count,
            1
        );
        assert_eq!(
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_count,
            0
        );
        assert_eq!(
            (
                state.source_site_execution_count,
                state
                    .supply_humidity_ratio_for_dehumidification_for_minimum_limit_maximum_read_count,
                state.minimum_cooling_supply_air_humidity_ratio_for_maximum_read_count,
                state.source_shaped_two_argument_maximum_evaluation_count,
                state.supply_humidity_ratio_for_dehumidification_assignment_count,
            ),
            (0, 0, 0, 0, 0)
        );

        let owner = output
            .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .assigned_supply_humidity_ratio;
        assert_eq!(
            owner.map(f64::to_bits),
            Some(
                output
                    .coupling
                    .purchased_air
                    .supply_node_update
                    .humidity_ratio
                    .to_bits()
            )
        );
    }
}

#[test]
fn scheduled_binding_preserves_cp361_u_n_and_p_skips() {
    for (load, availability, capacity, route) in [
        (3_000.0, 0.0, None, (true, false, false)),
        (0.0, 1.0, None, (false, true, false)),
        (3_000.0, 1.0, Some(-0.0), (false, false, true)),
    ] {
        let limit = if capacity.is_some() {
            IdealLoadsLimit::LimitCapacity
        } else {
            IdealLoadsLimit::NoLimit
        };
        let Some((_, output)) = run_case(limit, capacity, load, availability) else {
            return;
        };
        let snapshot = output
            .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            route
        );
        assert!(
            cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(!snapshot.supply_humidity_ratio_for_dehumidification_assignment_performed);
        assert!(
            snapshot
                .resulting_supply_humidity_ratio_for_dehumidification
                .is_none()
        );
    }
}
