use super::*;
use crate::ideal_loads::{
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn scheduled_binding_places_cp360_after_cp359_as_an_exact_null_c0_skip() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let Some((runtime, output)) = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0)
        else {
            return;
        };
        let predecessor = output.calculation_cooling_humidistat_moisture_demand_assignment;
        let snapshot = output
            .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment;

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
            cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(snapshot.dehumidification_control_none_case_completed_skip);
        assert!(
            !snapshot
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed
        );
        assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
        assert!(
            snapshot
                .zone_dehumidifying_setpoint_moisture_demand_kg_per_s
                .is_none()
        );
        assert!(!snapshot.supply_mass_flow_rate_read);
        assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
        assert!(!snapshot.moisture_demand_derived_supply_humidity_ratio_calculated);
        assert!(
            snapshot
                .moisture_demand_derived_supply_humidity_ratio
                .is_none()
        );
        assert!(!snapshot.zone_node_humidity_ratio_read);
        assert!(snapshot.zone_node_humidity_ratio.is_none());
        assert!(!snapshot.supply_humidity_ratio_for_dehumidification_calculated);
        assert!(
            snapshot
                .calculated_supply_humidity_ratio_for_dehumidification
                .is_none()
        );
        assert!(!snapshot.supply_humidity_ratio_for_dehumidification_assigned);
        assert!(
            snapshot
                .assigned_supply_humidity_ratio_for_dehumidification
                .is_none()
        );
        assert!(
            snapshot
                .resulting_supply_humidity_ratio_for_dehumidification
                .is_none()
        );

        let summary =
            purchased_air_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_lifecycle_summary(
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
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count,
            0
        );
        assert_eq!(state.source_site_execution_count, 0);
        assert_eq!(
            (
                state.zone_dehumidifying_setpoint_moisture_demand_read_count,
                state.supply_mass_flow_rate_read_count,
                state.moisture_demand_derived_supply_humidity_ratio_calculation_count,
                state.zone_node_humidity_ratio_read_count,
                state.supply_humidity_ratio_for_dehumidification_calculation_count,
                state.supply_humidity_ratio_for_dehumidification_assignment_count,
            ),
            (0, 0, 0, 0, 0, 0)
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
fn scheduled_binding_preserves_cp360_u_n_and_p_skips() {
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
            .calculation_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            route
        );
        assert!(
            cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(!snapshot.supply_humidity_ratio_for_dehumidification_assigned);
        assert!(
            snapshot
                .resulting_supply_humidity_ratio_for_dehumidification
                .is_none()
        );
    }
}
