use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_lifecycle_summary,
};

#[test]
fn binding_places_cp402_after_cp401_before_unchanged_numerical_coupling() {
    let mut saw_active = false;
    let mut saw_inactive = false;
    for (limit, humidity_ratio, availability, capacity) in [
        (IdealLoadsLimit::NoLimit, 0.008, 0.0, 5_000.0),
        (IdealLoadsLimit::NoLimit, 0.008, 1.0, 5_000.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 500.0),
    ] {
        let (runtime, output) = super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
            limit,
            humidity_ratio,
            availability,
            capacity,
        );
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment;
        let capacity_owner = output.calculation_cooling_capacity_zero_flow_reset;
        let capacity_corroborator =
            output.calculation_cooling_positive_supply_capacity_limit_sensible_output_guard;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard;
        let active = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed,
            active,
        );
        for local in [
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluated,
            snapshot.cp401_retained_cooling_latent_output_owned_read,
            snapshot.cooling_latent_output_read,
            snapshot.cp321_maximum_total_cooling_capacity_owned_read,
            snapshot.cp340_same_call_maximum_total_cooling_capacity_bit_corroborated,
            snapshot.maximum_total_cooling_capacity_read,
            snapshot.cooling_latent_output_maximum_total_cooling_capacity_comparison_evaluated,
        ] {
            assert_eq!(local, active);
        }

        if active {
            let latent = predecessor
                .cooling_latent_output_w
                .expect("active CP402 must read CP401-owned latent output");
            let maximum = capacity_owner
                .maximum_total_cooling_capacity_w
                .expect("active CP402 must read CP321-owned maximum capacity");
            let corroborated = capacity_corroborator
                .maximum_total_cooling_capacity_w
                .expect("active CP402 must corroborate maximum capacity through CP340");
            assert_eq!(
                snapshot.cooling_latent_output_w.map(f64::to_bits),
                Some(latent.to_bits())
            );
            assert_eq!(
                snapshot.maximum_total_cooling_capacity_w.map(f64::to_bits),
                Some(maximum.to_bits())
            );
            assert_eq!(corroborated.to_bits(), maximum.to_bits());
            assert_eq!(
                snapshot
                    .cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity,
                Some(latent >= maximum),
            );
            assert_eq!(
                snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered,
                latent >= maximum,
            );
            assert_eq!(
                snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
                latent < maximum,
            );
        } else {
            assert!(snapshot.cooling_latent_output_w.is_none());
            assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
            assert!(
                snapshot
                    .cooling_latent_output_greater_than_or_equal_to_maximum_total_cooling_capacity
                    .is_none()
            );
        }
        for (left, right) in [
            (
                snapshot.predecessor_cp401_resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp401_resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp401_resulting_supply_temperature_c,
                predecessor.resulting_supply_temperature_c,
            ),
            (
                snapshot.resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.resulting_supply_temperature_c,
                predecessor.resulting_supply_temperature_c,
            ),
        ] {
            assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
        }

        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_guard_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP402 lifecycle");
        let evaluations = usize::from(active);
        let body = usize::from(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_body_entered,
        );
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1 - evaluations);
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_evaluation_count,
            evaluations,
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            3 * evaluations + body
        );
        saw_active |= active;
        saw_inactive |= !active;
    }
    assert!(saw_active, "fixture set must execute CP402");
    assert!(saw_inactive, "fixture set must also skip CP402");
}
