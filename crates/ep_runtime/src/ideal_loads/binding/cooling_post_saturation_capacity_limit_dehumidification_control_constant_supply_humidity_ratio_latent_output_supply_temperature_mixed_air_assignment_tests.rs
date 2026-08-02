use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_lifecycle_summary,
};

#[test]
fn binding_places_cp403_after_cp402_before_unchanged_numerical_coupling() {
    let mut saw_assignment = false;
    let mut saw_inactive = false;
    for (limit, humidity_ratio, availability, capacity) in [
        (
            IdealLoadsLimit::LimitCapacity,
            0.020,
            1.0,
            f64::MIN_POSITIVE,
        ),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 1.0e9),
        (IdealLoadsLimit::NoLimit, 0.008, 0.0, 5_000.0),
    ] {
        let (runtime, output) = super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
            limit,
            humidity_ratio,
            availability,
            capacity,
        );
        let mixed_air_owner = output.calculation_cooling_mixed_air_call;
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment;
        let assignment = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered;
        let guard_false = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed,
            assignment,
        );
        assert_eq!(
            snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
            guard_false,
        );

        for (left, right) in [
            (
                snapshot.resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            ),
        ] {
            assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
        }

        if assignment {
            let mixed_air_temperature_c = mixed_air_owner
                .mixed_air_temperature_c
                .expect("active CP403 must read CP329 mixed-air temperature");
            assert!(mixed_air_owner.cooling_call_executed);
            assert!(mixed_air_owner.no_outdoor_air_fallback_entered);
            assert!(mixed_air_owner.mixed_air_temperature_assigned);
            for value in [
                predecessor.predecessor_mixed_air_temperature_c,
                snapshot.predecessor_mixed_air_temperature_c,
                snapshot.mixed_air_temperature_c,
                snapshot.assigned_supply_temperature_c,
                snapshot.resulting_supply_temperature_c,
            ] {
                assert_eq!(
                    value.map(f64::to_bits),
                    Some(mixed_air_temperature_c.to_bits())
                );
            }
            for flag in [
                snapshot.cp329_retained_mixed_air_temperature_owned_read,
                snapshot.cp402_same_call_mixed_air_temperature_bit_corroborated,
                snapshot.mixed_air_temperature_read,
                snapshot.supply_temperature_assigned,
            ] {
                assert!(flag);
            }
        } else {
            for value in [
                snapshot.mixed_air_temperature_c,
                snapshot.assigned_supply_temperature_c,
            ] {
                assert!(value.is_none());
            }
            assert_eq!(
                snapshot.resulting_supply_temperature_c.map(f64::to_bits),
                predecessor.resulting_supply_temperature_c.map(f64::to_bits),
            );
        }

        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP403 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle
                .state
                .supply_temperature_mixed_air_assignment_count,
            usize::from(assignment),
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            2 * usize::from(assignment),
        );
        saw_assignment |= assignment;
        saw_inactive |= !assignment && !guard_false;
    }
    assert!(saw_assignment, "fixture set must execute CP403");
    assert!(saw_inactive, "fixture set must preserve an inactive route");
}
