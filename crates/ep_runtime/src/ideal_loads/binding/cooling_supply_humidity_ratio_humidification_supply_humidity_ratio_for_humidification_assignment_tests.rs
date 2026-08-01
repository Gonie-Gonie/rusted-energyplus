use super::*;
use crate::ideal_loads::{
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_links_to_direct_release,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn binding_orders_cp372_then_cp373_and_keeps_the_direct_assignment_numeric_free() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let case = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        assert!(case.is_some(), "CP373 binding fixture must succeed");
        let Some((runtime, output)) = case else {
            return;
        };
        let predecessor = output
            .calculation_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment;
        let snapshot = output
            .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment;

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
            cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert!(!snapshot
            .dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed);
        assert!(!snapshot
            .dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed);
        for flag in [
            snapshot.zone_humidifying_setpoint_moisture_demand_read,
            snapshot.supply_mass_flow_rate_read,
            snapshot.moisture_demand_derived_supply_humidity_ratio_calculated,
            snapshot.zone_node_humidity_ratio_read,
            snapshot.supply_humidity_ratio_for_humidification_calculated,
            snapshot.supply_humidity_ratio_for_humidification_assigned,
        ] {
            assert!(!flag);
        }
        for value in [
            snapshot.predecessor_resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.zone_humidifying_setpoint_moisture_demand_kg_per_s,
            snapshot.supply_mass_flow_rate_kg_per_s,
            snapshot.moisture_demand_derived_supply_humidity_ratio,
            snapshot.zone_node_humidity_ratio,
            snapshot.calculated_supply_humidity_ratio_for_humidification,
            snapshot.assigned_supply_humidity_ratio_for_humidification,
            snapshot.resulting_supply_humidity_ratio_for_humidification,
        ] {
            assert_eq!(value, None);
        }

        let summary = purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP373 lifecycle summary");
        assert_eq!(summary.state.transition_count, 1);
        assert_eq!(
            summary
                .state
                .humidification_control_guard_false_fallthrough_count,
            1,
        );
        for count in [
            summary.state.dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_count,
            summary.state.dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_count,
            summary.state.dehumidification_control_guard_false_fallthrough_count,
            summary.state.source_site_execution_count,
            summary.state.zone_humidifying_setpoint_moisture_demand_read_count,
            summary.state.supply_mass_flow_rate_read_count,
            summary.state.moisture_demand_derived_supply_humidity_ratio_calculation_count,
            summary.state.zone_node_humidity_ratio_read_count,
            summary.state.supply_humidity_ratio_for_humidification_calculation_count,
            summary.state.supply_humidity_ratio_for_humidification_assignment_count,
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
fn private_selected_none_cp373_preserves_divide_then_add_ieee_bits_without_mutating_release() {
    let case = run_case(IdealLoadsLimit::NoLimit, None, 3_000.0, 1.0);
    assert!(case.is_some(), "CP373 private binding fixture must succeed");
    let Some((runtime, output)) = case else {
        return;
    };
    let direct = output
        .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment;
    let system_id = output.initialization.system;
    let mut fixture_system = ideal_loads_system();
    fixture_system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    let unit = runtime.units.get(&system_id).expect("known CP373 unit");
    let flow = output
        .calculation_cooling_supply_mass_flow_positive_guard
        .supply_mass_flow_rate_kg_per_s
        .expect("CP330 positive flow");
    let before = runtime.clone();

    for (demand, zone_humidity) in [
        (0.001_f64, 0.008_f64),
        (-0.0_f64, 0.0_f64),
        (f64::from_bits(0x7ff8_0000_0000_0373), 0.008_f64),
    ] {
        let private = private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release(
            &runtime,
            unit,
            &fixture_system,
            direct,
            demand,
            zone_humidity,
        )
        .expect("canonical private selected-None CP373");
        let quotient = demand / flow;
        let expected = quotient + zone_humidity;
        assert!(!private
            .dehumidification_control_humidistat_supply_humidity_ratio_for_humidification_assignment_executed);
        assert!(private
            .dehumidification_control_none_supply_humidity_ratio_for_humidification_assignment_executed);
        assert_eq!(
            private
                .moisture_demand_derived_supply_humidity_ratio
                .expect("CP373 quotient")
                .to_bits(),
            quotient.to_bits(),
        );
        assert_eq!(
            private
                .resulting_supply_humidity_ratio_for_humidification
                .expect("CP373 result")
                .to_bits(),
            expected.to_bits(),
        );
        assert!(
            private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_links_to_direct_release(
                &runtime,
                unit,
                &fixture_system,
                direct,
                private,
                demand,
                zone_humidity,
            )
        );
    }
    assert_eq!(runtime, before);
}
