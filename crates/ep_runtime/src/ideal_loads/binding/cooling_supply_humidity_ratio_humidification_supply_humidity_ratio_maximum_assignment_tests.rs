use super::*;
use crate::ideal_loads::{
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_links_to_direct_release,
    purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_lifecycle_summary,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::run_case;

#[test]
fn binding_orders_cp374_then_cp375_and_keeps_the_numerical_owner_unchanged() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let case = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0);
        assert!(case.is_some(), "CP375 binding fixture must succeed");
        let Some((runtime, output)) = case else {
            return;
        };
        let predecessor = output
            .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit;
        let snapshot = output
            .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment;

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
            cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        for flag in [
            snapshot.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed,
            snapshot.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed,
            snapshot.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read,
            snapshot.supply_humidity_ratio_for_humidification_for_supply_maximum_read,
            snapshot.source_shaped_two_argument_maximum_evaluated,
            snapshot.purchased_air_supply_humidity_ratio_assignment_performed,
        ] {
            assert!(!flag);
        }
        for value in [
            snapshot.predecessor_resulting_supply_humidity_ratio_for_humidification,
            snapshot.purchased_air_supply_humidity_ratio_before_humidification_supply_maximum,
            snapshot.supply_humidity_ratio_for_humidification_for_supply_maximum,
            snapshot.maximum_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(value, None);
        }

        let summary = purchased_air_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP375 lifecycle summary");
        assert_eq!(summary.state.transition_count, 1);
        assert_eq!(
            summary
                .state
                .humidification_control_guard_false_fallthrough_count,
            1,
        );
        for count in [
            summary.state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
            summary.state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
            summary.state.dehumidification_control_guard_false_fallthrough_count,
            summary.state.source_site_execution_count,
            summary.state.purchased_air_supply_humidity_ratio_for_humidification_supply_maximum_read_count,
            summary.state.supply_humidity_ratio_for_humidification_for_supply_maximum_read_count,
            summary.state.source_shaped_two_argument_maximum_evaluation_count,
            summary.state.purchased_air_supply_humidity_ratio_assignment_count,
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
fn private_selected_none_cp375_uses_strict_less_than_left_biased_source_maximum() {
    let case = run_case(IdealLoadsLimit::NoLimit, None, 3_000.0, 1.0);
    assert!(case.is_some(), "CP375 private binding fixture must succeed");
    let Some((runtime, output)) = case else {
        return;
    };
    let direct = output
        .calculation_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment;
    let system_id = output.initialization.system;
    let unit = runtime.units.get(&system_id).expect("known CP375 unit");
    let before = runtime.clone();

    for (demand, zone_humidity, cp374_limit) in [
        (0.0_f64, 0.010_f64, 0.008_f64),
        (0.0_f64, 0.007_f64, 0.008_f64),
        (-0.0_f64, -0.0_f64, 0.0_f64),
        (0.0_f64, 0.0_f64, -0.0_f64),
        (f64::from_bits(0x7ff8_0000_0000_0374), 0.0_f64, 0.008_f64),
    ] {
        let mut fixture_system = ideal_loads_system();
        fixture_system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
        fixture_system.maximum_heating_supply_air_humidity_ratio = cp374_limit;
        let private = private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_from_direct_release(
            &runtime,
            unit,
            &fixture_system,
            direct,
            demand,
            zone_humidity,
        )
        .expect("canonical private selected-None CP375");
        let left = private
            .purchased_air_supply_humidity_ratio_before_humidification_supply_maximum
            .expect("branch-owned purchased-air left operand");
        let right = private
            .predecessor_resulting_supply_humidity_ratio_for_humidification
            .expect("CP374 local right operand");
        let expected = if left < right { right } else { left };
        assert!(!private
            .dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_executed);
        assert!(private
            .dehumidification_control_none_supply_humidity_ratio_maximum_assignment_executed);
        assert_eq!(
            private
                .purchased_air_supply_humidity_ratio_before_humidification_supply_maximum
                .map(f64::to_bits),
            Some(left.to_bits()),
        );
        for value in [
            private.maximum_supply_humidity_ratio,
            private.assigned_supply_humidity_ratio,
            private.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
        }
        assert_eq!(
            private
                .supply_humidity_ratio_for_humidification_for_supply_maximum
                .map(f64::to_bits),
            Some(right.to_bits()),
        );
        assert!(
            private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_links_to_direct_release(
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

    let mut invalid_owner = ideal_loads_system();
    invalid_owner.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
    invalid_owner.maximum_heating_supply_air_humidity_ratio = f64::NAN;
    assert!(
        private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_maximum_assignment_counterfactual_from_direct_release(
            &runtime,
            unit,
            &invalid_owner,
            direct,
            0.0,
            0.008,
        )
        .is_none()
    );
    assert_eq!(runtime, before);
}
