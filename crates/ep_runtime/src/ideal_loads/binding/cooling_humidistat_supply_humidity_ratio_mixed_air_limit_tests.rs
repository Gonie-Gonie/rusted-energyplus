use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor,
    purchased_air_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn scheduled_binding_places_cp362_after_cp361_and_keeps_cp345_owner() {
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
            .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit;
        let snapshot = output.calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit;

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
            cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(
            cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor(
                snapshot,
                predecessor,
            )
        );
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert_complete_null(snapshot);

        let summary =
            purchased_air_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP362 lifecycle");
        let state = &summary.state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(
            state.dehumidification_control_none_case_completed_skip_count,
            1
        );
        assert_eq!(
            state.dehumidification_control_humidistat_supply_humidity_ratio_mixed_air_limit_count,
            0
        );
        assert_eq!(
            (
                state.source_site_execution_count,
                state.mixed_air_humidity_ratio_for_minimum_read_count,
                state
                    .supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read_count,
                state.source_shaped_two_argument_minimum_evaluation_count,
                state.supply_humidity_ratio_assignment_count,
            ),
            (0, 0, 0, 0, 0)
        );

        let cp345 = output
            .calculation_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment;
        let numerical = output
            .coupling
            .purchased_air
            .supply_node_update
            .humidity_ratio;
        assert_eq!(
            cp345.assigned_supply_humidity_ratio.map(f64::to_bits),
            Some(numerical.to_bits())
        );
        assert!(
            snapshot.resulting_supply_humidity_ratio.is_none(),
            "complete-null CP362 evidence must not feed the numerical result"
        );
    }
}

#[test]
fn scheduled_binding_preserves_cp362_u_n_p_as_complete_null_operand_nonreads() {
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
        let predecessor = output
            .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit;
        let snapshot = output.calculation_cooling_humidistat_supply_humidity_ratio_mixed_air_limit;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            route
        );
        assert!(
            cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(
            cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor(
                snapshot,
                predecessor,
            )
        );
        assert_complete_null(snapshot);
    }
}

fn assert_complete_null(
    snapshot: PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot,
) {
    for flag in [
        snapshot.mixed_air_humidity_ratio_for_minimum_read,
        snapshot.supply_humidity_ratio_for_dehumidification_for_mixed_air_limit_minimum_read,
        snapshot.source_shaped_two_argument_minimum_evaluated,
        snapshot.supply_humidity_ratio_assignment_performed,
    ] {
        assert!(!flag);
    }
    assert_eq!(
        [
            snapshot.predecessor_resulting_supply_humidity_ratio_for_dehumidification,
            snapshot.mixed_air_humidity_ratio,
            snapshot.supply_humidity_ratio_for_dehumidification_before_mixed_air_limit,
            snapshot.minimum_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ],
        [None; 6]
    );
}
