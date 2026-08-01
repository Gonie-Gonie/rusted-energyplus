use super::*;
use crate::ideal_loads::{
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn binding_orders_cp371_then_cp372_and_keeps_the_direct_assignment_numeric_free() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let case = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        assert!(case.is_some(), "CP372 binding fixture must succeed");
        let Some((runtime, output)) = case else {
            return;
        };
        let predecessor = output
            .calculation_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard;
        let snapshot = output
            .calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment;

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
            cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.predecessor_humidification_control_type_read);
        assert!(snapshot.predecessor_humidification_control_guard_false_fallthrough);
        assert!(!snapshot.predecessor_dehumidification_control_body_entered);
        assert!(!snapshot.humidification_moisture_demand_assignment_executed);
        assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_read);
        assert_eq!(
            snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s,
            None,
        );
        assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_assigned);
        assert_eq!(
            snapshot.assigned_zone_humidifying_setpoint_moisture_demand_kg_per_s,
            None,
        );
        assert_eq!(
            snapshot.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
            None,
        );

        let summary = purchased_air_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP372 lifecycle summary");
        assert_eq!(summary.state.transition_count, 1);
        assert_eq!(
            summary
                .state
                .humidification_control_guard_false_fallthrough_count,
            1,
        );
        for count in [
            summary
                .state
                .dehumidification_control_humidistat_moisture_demand_assignment_count,
            summary
                .state
                .dehumidification_control_none_moisture_demand_assignment_count,
            summary
                .state
                .dehumidification_control_guard_false_fallthrough_count,
            summary
                .state
                .humidification_moisture_demand_assignment_count,
            summary.state.source_site_execution_count,
            summary
                .state
                .zone_humidifying_setpoint_moisture_demand_read_count,
            summary
                .state
                .zone_humidifying_setpoint_moisture_demand_assignment_count,
        ] {
            assert_eq!(count, 0);
        }

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
fn binding_cp372_preserves_u_n_and_p_skips_without_reading_humidity_demand() {
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
        assert!(case.is_some(), "CP372 U/N/P binding fixture must succeed");
        let Some((_, output)) = case else {
            return;
        };
        let snapshot = output
            .calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            expected,
        );
        assert!(
            cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(!snapshot.humidification_moisture_demand_assignment_executed);
        assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_read);
        assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_assigned);
        assert_eq!(
            snapshot.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
            None,
        );
    }
}
