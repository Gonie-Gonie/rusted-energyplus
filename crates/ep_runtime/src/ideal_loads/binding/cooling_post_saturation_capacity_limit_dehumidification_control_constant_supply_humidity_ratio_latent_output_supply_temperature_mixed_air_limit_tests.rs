use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_lifecycle_summary,
};

#[test]
fn binding_places_cp408_after_cp407_before_unchanged_numerical_coupling() {
    let mut saw_limit = false;
    let mut saw_cp405_sibling = false;
    let mut saw_inherited_inactive = false;
    for (limit, humidity_ratio, availability, capacity) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 500.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 1.0e-100),
        (IdealLoadsLimit::NoLimit, 0.008, 0.0, 5_000.0),
    ] {
        let (runtime, output) = super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
            limit,
            humidity_ratio,
            availability,
            capacity,
        );
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment;
        let mixed_air_owner = output.calculation_cooling_mixed_air_call;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit;
        let executed = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed,
            executed,
        );
        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_executed,
            executed,
        );
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(snapshot.system, predecessor.system);
        assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);

        if executed {
            let supply = predecessor
                .resulting_supply_temperature_c
                .expect("active CP408 CP407 supply-temperature owner");
            let mixed = mixed_air_owner
                .mixed_air_temperature_c
                .expect("active CP408 CP329 mixed-air-temperature owner");
            let expected = if supply < mixed { supply } else { mixed };
            for flag in [
                snapshot.cp407_retained_supply_temperature_state_owned,
                snapshot.cp407_retained_supply_temperature_owned_read,
                snapshot.supply_temperature_for_minimum_read,
                snapshot.cp329_retained_mixed_air_temperature_owned_read,
                snapshot.mixed_air_temperature_for_minimum_read,
                snapshot.source_shaped_two_argument_minimum_evaluated,
                snapshot.supply_temperature_assignment_performed,
            ] {
                assert!(flag);
            }
            for (actual, expected) in [
                (snapshot.preexisting_supply_temperature_c, supply),
                (snapshot.supply_temperature_before_mixed_air_limit_c, supply),
                (snapshot.mixed_air_temperature_c, mixed),
                (snapshot.minimum_supply_temperature_c, expected),
                (snapshot.assigned_supply_temperature_c, expected),
                (snapshot.resulting_supply_temperature_c, expected),
            ] {
                assert_eq!(actual.map(f64::to_bits), Some(expected.to_bits()));
            }
            assert_eq!(
                snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
                predecessor
                    .resulting_supply_humidity_ratio
                    .map(f64::to_bits)
            );
            assert_eq!(
                snapshot
                    .resulting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits),
                predecessor
                    .resulting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits)
            );
        } else {
            for flag in [
                snapshot.cp407_retained_supply_temperature_owned_read,
                snapshot.supply_temperature_for_minimum_read,
                snapshot.cp329_retained_mixed_air_temperature_owned_read,
                snapshot.mixed_air_temperature_for_minimum_read,
                snapshot.source_shaped_two_argument_minimum_evaluated,
                snapshot.supply_temperature_assignment_performed,
            ] {
                assert!(!flag);
            }
            for value in [
                snapshot.supply_temperature_before_mixed_air_limit_c,
                snapshot.mixed_air_temperature_c,
                snapshot.minimum_supply_temperature_c,
                snapshot.assigned_supply_temperature_c,
            ] {
                assert!(value.is_none());
            }
            assert_eq!(
                snapshot.resulting_supply_temperature_c.map(f64::to_bits),
                predecessor.resulting_supply_temperature_c.map(f64::to_bits)
            );
        }

        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP408 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.inactive_transition_count,
            usize::from(!executed)
        );
        assert_eq!(
            lifecycle.state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_count,
            usize::from(executed),
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            4 * usize::from(executed)
        );

        saw_limit |= executed;
        saw_cp405_sibling |= predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;
        saw_inherited_inactive |= !executed
            && !predecessor
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;
    }
    assert!(saw_limit, "fixture set must execute CP408");
    assert!(
        saw_cp405_sibling,
        "fixture set must preserve the CP405 sibling"
    );
    assert!(
        saw_inherited_inactive,
        "fixture set must preserve inherited inactivity"
    );
}
