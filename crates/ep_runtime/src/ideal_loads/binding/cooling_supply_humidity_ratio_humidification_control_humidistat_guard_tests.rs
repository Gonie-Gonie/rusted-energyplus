use super::*;
use ep_model::HumidificationControlType;
use crate::ideal_loads::{
    cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn binding_orders_cp369_then_cp370_before_numerical_and_does_not_feed_result() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let case = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        assert!(case.is_some(), "CP370 binding fixture must succeed");
        let Some((runtime, output)) = case else {
            return;
        };
        let predecessor = output.calculation_cooling_supply_humidity_ratio_humidification_heating_availability_guard;
        let snapshot = output
            .calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard;
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
            cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(snapshot.predecessor_heating_on_read);
        assert_eq!(snapshot.predecessor_heating_on, Some(true));
        assert!(
            snapshot.predecessor_cooling_supply_humidity_ratio_humidification_body_entered
        );
        assert!(!snapshot.predecessor_heating_on_guard_false_fallthrough);
        assert!(snapshot.humidification_control_type_read);
        assert_eq!(
            snapshot.humidification_control_type,
            Some(HumidificationControlType::None)
        );
        assert_eq!(snapshot.humidification_control_type_humidistat, Some(false));
        assert!(!snapshot.humidification_control_body_entered);
        assert!(snapshot.humidification_control_guard_false_fallthrough);

        let summary = purchased_air_calc_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP370 lifecycle summary");
        assert_eq!(summary.state.transition_count, 1);
        assert_eq!(
            summary
                .state
                .dehumidification_control_none_case_completed_skip_count,
            1
        );
        assert_eq!(summary.state.heating_on_read_count, 1);
        assert_eq!(summary.state.heating_on_body_entry_count, 1);
        assert_eq!(summary.state.heating_on_guard_false_fallthrough_count, 0);
        assert_eq!(summary.state.humidification_control_type_read_count, 1);
        assert_eq!(
            summary
                .state
                .humidification_control_type_humidistat_comparison_count,
            1
        );
        assert_eq!(summary.state.humidification_control_body_entry_count, 0);
        assert_eq!(
            summary.state.humidification_control_guard_false_fallthrough_count,
            1
        );
        assert_eq!(summary.state.source_site_execution_count, 2);

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
fn binding_cp370_skips_control_sites_for_u_n_and_p_routes() {
    for (load, availability, capacity, expected) in [
        (3_000.0, 0.0, None, (true, false, false)),
        (0.0, 1.0, None, (false, true, false)),
        (3_000.0, 1.0, Some(-0.0), (false, false, true)),
    ] {
        let limit = if capacity.is_some() {
            IdealLoadsLimit::LimitCapacity
        } else {
            IdealLoadsLimit::NoLimit
        };
        let case = run_case(limit, capacity, load, availability);
        assert!(
            case.is_some(),
            "CP370 U/N/P binding fixture must succeed"
        );
        let Some((_, output)) = case else {
            return;
        };
        let snapshot = output
            .calculation_cooling_supply_humidity_ratio_humidification_control_humidistat_guard;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            expected
        );
        assert!(
            cooling_supply_humidity_ratio_humidification_control_humidistat_guard_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(!snapshot.predecessor_heating_on_read);
        assert_eq!(snapshot.predecessor_heating_on, None);
        assert!(
            !snapshot.predecessor_cooling_supply_humidity_ratio_humidification_body_entered
        );
        assert!(!snapshot.predecessor_heating_on_guard_false_fallthrough);
        assert!(!snapshot.humidification_control_type_read);
        assert_eq!(snapshot.humidification_control_type, None);
        assert_eq!(snapshot.humidification_control_type_humidistat, None);
        assert!(!snapshot.humidification_control_body_entered);
        assert!(!snapshot.humidification_control_guard_false_fallthrough);
    }
}
