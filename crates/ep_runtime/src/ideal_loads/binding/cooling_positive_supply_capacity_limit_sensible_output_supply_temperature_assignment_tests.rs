use super::*;
use crate::ideal_loads::{
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle_summary,
};

fn run_case(
    cooling_limit: IdealLoadsLimit,
    maximum_capacity_w: Option<f64>,
    independent_load_w: f64,
    availability: f64,
) -> (
    PurchasedAirRuntimeState,
    DirectZonePurchasedAirScheduledCouplingOutput,
) {
    let (model, cache) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = cooling_limit;
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
        system.maximum_total_cooling_capacity_w = maximum_capacity_w.map(AutosizeOrNumber::Value);
        schedule_mut(typed, ScheduleId(3)).hourly_value = availability;
    });
    let binding = bind_direct_zone_purchased_air_model(&model).expect("bounded model binding");
    let mut zone_state = zone_state_for_temp_independent_load(independent_load_w);
    let mut runtime = PurchasedAirRuntimeState::default();
    let output = couple_model_bound_direct_zone_purchased_air(
        DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut zone_state,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        },
    )
    .expect("source-ordered CP343 coupling");
    (runtime, output)
}

#[test]
fn scheduled_binding_preserves_false_true_and_inherited_supply_temperature_routes() {
    for (
        maximum_capacity_w,
        independent_load_w,
        availability,
        expected_guard_false,
        expected_assignment,
        expected_unit_off,
    ) in [
        (1.0e9, 3_000.0, 1.0, true, false, false),
        (1.0, 3_000.0, 1.0, false, true, false),
        (1.0, 3_000.0, 0.0, false, false, true),
    ] {
        let (runtime, output) = run_case(
            IdealLoadsLimit::LimitCapacity,
            Some(maximum_capacity_w),
            independent_load_w,
            availability,
        );
        let cp334 = output.calculation_cooling_positive_supply_temperature_mixed_air_limit;
        let cp335 = output.calculation_cooling_positive_supply_humidity_ratio_mixed_air_assignment;
        let cp336 = output.calculation_cooling_positive_supply_enthalpy_assignment;
        let predecessor = output
            .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_enthalpy_assignment;
        let assignment = output
            .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;

        assert!(
            cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_snapshot_is_exact_direct_release(
                assignment,
            )
        );
        assert_eq!(
            assignment.capacity_limit_sensible_output_guard_false_fallthrough,
            expected_guard_false
        );
        assert_eq!(
            assignment.capacity_limit_sensible_output_supply_temperature_assignment_executed,
            expected_assignment
        );
        assert_eq!(assignment.unit_off_skipped, expected_unit_off);

        if expected_assignment {
            let preexisting = cp334
                .assigned_supply_temperature_c
                .expect("CP334 owned supply temperature");
            let enthalpy = predecessor
                .resulting_supply_enthalpy_j_per_kg
                .expect("CP342 resulting supply enthalpy");
            let humidity = cp335
                .assigned_supply_humidity_ratio
                .expect("CP335 owned supply humidity ratio");
            let expected = crate::psychrometrics::energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
            assert_eq!(
                cp336.supply_temperature_c.map(f64::to_bits),
                Some(preexisting.to_bits())
            );
            assert_eq!(
                cp336.supply_humidity_ratio.map(f64::to_bits),
                Some(humidity.to_bits())
            );
            assert_eq!(
                assignment
                    .preexisting_supply_temperature_c
                    .map(f64::to_bits),
                Some(preexisting.to_bits())
            );
            assert_eq!(
                assignment.supply_enthalpy_j_per_kg.map(f64::to_bits),
                Some(enthalpy.to_bits())
            );
            assert_eq!(
                assignment.supply_humidity_ratio.map(f64::to_bits),
                Some(humidity.to_bits())
            );
            assert_eq!(
                assignment
                    .psychrometric_supply_temperature_result_c
                    .map(f64::to_bits),
                Some(expected.to_bits())
            );
            assert_eq!(
                assignment.assigned_supply_temperature_c.map(f64::to_bits),
                Some(expected.to_bits())
            );
            assert_eq!(
                assignment.resulting_supply_temperature_c.map(f64::to_bits),
                Some(expected.to_bits())
            );
        } else if expected_guard_false {
            let preexisting = cp334
                .assigned_supply_temperature_c
                .expect("CP334 owned supply temperature");
            assert_eq!(
                assignment
                    .preexisting_supply_temperature_c
                    .map(f64::to_bits),
                Some(preexisting.to_bits())
            );
            assert_eq!(
                assignment.resulting_supply_temperature_c.map(f64::to_bits),
                Some(preexisting.to_bits())
            );
            assert!(assignment.supply_enthalpy_j_per_kg.is_none());
            assert!(assignment.supply_humidity_ratio.is_none());
            assert!(
                assignment
                    .psychrometric_supply_temperature_result_c
                    .is_none()
            );
            assert!(assignment.assigned_supply_temperature_c.is_none());
        } else {
            assert!(assignment.preexisting_supply_temperature_c.is_none());
            assert!(assignment.resulting_supply_temperature_c.is_none());
        }

        let lifecycle =
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP343 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle
                .state
                .capacity_limit_sensible_output_supply_temperature_assignment_count,
            usize::from(expected_assignment)
        );
    }
}

#[test]
fn scheduled_binding_preserves_complete_null_inherited_routes() {
    for (
        cooling_limit,
        maximum_capacity_w,
        independent_load_w,
        availability,
        unit_off,
        non_cooling,
        positive_guard_false,
        capacity_guard_false,
    ) in [
        (
            IdealLoadsLimit::NoLimit,
            None,
            3_000.0,
            0.0,
            true,
            false,
            false,
            false,
        ),
        (
            IdealLoadsLimit::NoLimit,
            None,
            0.0,
            1.0,
            false,
            true,
            false,
            false,
        ),
        (
            IdealLoadsLimit::LimitCapacity,
            Some(-0.0),
            3_000.0,
            1.0,
            false,
            false,
            true,
            false,
        ),
        (
            IdealLoadsLimit::NoLimit,
            None,
            3_000.0,
            1.0,
            false,
            false,
            false,
            true,
        ),
    ] {
        let (runtime, output) = run_case(
            cooling_limit,
            maximum_capacity_w,
            independent_load_w,
            availability,
        );
        let snapshot = output
            .calculation_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment;
        assert!(!snapshot.capacity_limit_sensible_output_guard_false_fallthrough);
        assert!(!snapshot.capacity_limit_sensible_output_supply_temperature_assignment_executed);
        for value in [
            snapshot.preexisting_supply_temperature_c,
            snapshot.supply_enthalpy_j_per_kg,
            snapshot.supply_humidity_ratio,
            snapshot.psychrometric_supply_temperature_result_c,
            snapshot.assigned_supply_temperature_c,
            snapshot.resulting_supply_temperature_c,
        ] {
            assert!(value.is_none());
        }

        let state =
            purchased_air_calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_assignment_lifecycle_summary(
                &runtime,
                output.initialization.system,
            )
            .expect("CP343 inherited-skip lifecycle")
            .state;
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(
            state.positive_guard_false_fallthrough_skip_count,
            usize::from(positive_guard_false)
        );
        assert_eq!(
            state.capacity_limit_guard_false_fallthrough_skip_count,
            usize::from(capacity_guard_false)
        );
        assert_eq!(
            state.capacity_limit_sensible_output_guard_false_fallthrough_count,
            0
        );
        assert_eq!(
            state.capacity_limit_sensible_output_supply_temperature_assignment_count,
            0
        );
        assert_eq!(state.source_site_execution_count, 0);
    }
}
