use super::*;
use crate::{
    ideal_loads::{
        cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
        purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle_summary,
    },
    psychrometrics::energyplus_psy_w_fn_tdb_rh_pb,
};

use super::cooling_humidistat_moisture_demand_assignment_tests::{
    run_case, run_case_with_pressure,
};

#[test]
fn binding_orders_cp376_then_cp377_and_keeps_the_numerical_owner_unchanged() {
    for (cooling_limit, maximum_capacity_w) in [
        (IdealLoadsLimit::NoLimit, None),
        (IdealLoadsLimit::LimitCapacity, Some(1.0e9)),
        (IdealLoadsLimit::LimitCapacity, Some(1.0)),
    ] {
        let Some((runtime, output)) = run_case(cooling_limit, maximum_capacity_w, 3_000.0, 1.0)
        else {
            return;
        };
        let predecessor =
            output.calculation_cooling_supply_humidity_ratio_pre_saturation_original_assignment;
        let snapshot = output.calculation_cooling_supply_humidity_ratio_saturation_assignment;

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
            cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
                snapshot,
            )
        );
        assert_eq!(
            snapshot.predecessor_local_supply_humidity_ratio_original_assignment_performed,
            predecessor.local_supply_humidity_ratio_original_assignment_performed,
        );
        assert_eq!(
            snapshot
                .predecessor_resulting_supply_humidity_ratio_original
                .map(f64::to_bits),
            predecessor
                .resulting_supply_humidity_ratio_original
                .map(f64::to_bits),
        );
        assert!(snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read);
        assert!(
            snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read
        );
        assert!(snapshot.environment_outdoor_barometric_pressure_owned_read);
        assert!(snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated);
        assert!(snapshot.local_saturation_supply_humidity_ratio_assignment_performed);

        let temperature = snapshot
            .supply_temperature_for_saturation_humidity_ratio_c
            .expect("active CP377 supply temperature");
        let pressure = snapshot
            .outdoor_barometric_pressure_pa
            .expect("active CP377 weather pressure");
        let owner_temperature = if snapshot
            .cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read
        {
            assert!(!snapshot.cp334_supply_temperature_mixed_air_limit_owned_read);
            output
                .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
                .resulting_supply_temperature_c
                .expect("CP344 temperature owner")
        } else {
            assert!(snapshot.cp334_supply_temperature_mixed_air_limit_owned_read);
            output
                .calculation_cooling_positive_supply_temperature_mixed_air_limit
                .assigned_supply_temperature_c
                .expect("CP334 temperature owner")
        };
        assert_eq!(temperature.to_bits(), owner_temperature.to_bits());
        let saturation = energyplus_psy_w_fn_tdb_rh_pb(temperature, 1.0, pressure);
        for value in [
            snapshot.saturation_supply_humidity_ratio,
            snapshot.assigned_saturation_supply_humidity_ratio,
            snapshot.resulting_saturation_supply_humidity_ratio,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(saturation.to_bits()));
        }

        let summary =
            purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP377 lifecycle summary");
        assert_eq!(summary.state.transition_count, 1);
        assert_eq!(summary.state.source_site_execution_count, 4);

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
fn binding_cp377_preserves_u_n_and_p_as_complete_null_zero_site_routes() {
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
        let Some((runtime, output)) = run_case(
            cooling_limit,
            maximum_capacity_w,
            independent_load_w,
            availability,
        ) else {
            return;
        };
        let snapshot = output.calculation_cooling_supply_humidity_ratio_saturation_assignment;
        assert_eq!(
            (
                snapshot.unit_off_skipped,
                snapshot.non_cooling_skipped,
                snapshot.positive_guard_false_fallthrough_skipped,
            ),
            route,
        );
        for flag in [
            snapshot.cp334_supply_temperature_mixed_air_limit_owned_read,
            snapshot.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read,
            snapshot.environment_outdoor_barometric_pressure_owned_read,
            snapshot.purchased_air_supply_temperature_for_saturation_humidity_ratio_read,
            snapshot.environment_outdoor_barometric_pressure_for_saturation_humidity_ratio_read,
            snapshot.psy_w_fn_tdb_rh_pb_at_unity_relative_humidity_evaluated,
            snapshot.local_saturation_supply_humidity_ratio_assignment_performed,
        ] {
            assert!(!flag);
        }
        for value in [
            snapshot.predecessor_resulting_supply_humidity_ratio_original,
            snapshot.supply_temperature_for_saturation_humidity_ratio_c,
            snapshot.outdoor_barometric_pressure_pa,
            snapshot.saturation_supply_humidity_ratio,
            snapshot.assigned_saturation_supply_humidity_ratio,
            snapshot.resulting_saturation_supply_humidity_ratio,
        ] {
            assert!(value.is_none());
        }
        let state =
            purchased_air_calc_cooling_supply_humidity_ratio_saturation_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP377 inactive lifecycle")
            .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.source_site_execution_count, 0);
        assert_eq!(state.environment_outdoor_barometric_pressure_owner_count, 0);
    }
}

#[test]
fn binding_cp377_u_n_and_p_do_not_gate_on_barometric_pressure() {
    for (cooling_limit, maximum_capacity_w, independent_load_w, availability) in [
        (IdealLoadsLimit::NoLimit, None, 3_000.0, 0.0),
        (IdealLoadsLimit::NoLimit, None, 0.0, 1.0),
        (IdealLoadsLimit::LimitCapacity, Some(0.0), 3_000.0, 1.0),
    ] {
        for pressure in [f64::NAN, 0.0, -1.0, f64::INFINITY] {
            let case = run_case_with_pressure(
                cooling_limit,
                maximum_capacity_w,
                independent_load_w,
                availability,
                Some(pressure),
            );
            assert!(
                case.is_some(),
                "inactive CP377 route must not validate pressure bits 0x{:016x}",
                pressure.to_bits(),
            );
            let Some((_runtime, output)) = case else {
                return;
            };
            let snapshot = output.calculation_cooling_supply_humidity_ratio_saturation_assignment;
            assert!(!snapshot.environment_outdoor_barometric_pressure_owned_read);
            assert!(snapshot.outdoor_barometric_pressure_pa.is_none());
        }
    }
}
