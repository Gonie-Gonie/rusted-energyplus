use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_lifecycle_summary,
};

#[test]
fn binding_places_cp405_after_cp404_before_unchanged_numerical_coupling() {
    let mut saw_assignment = false;
    let mut saw_skip = false;
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
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment;
        let assignment = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
            assignment,
        );
        assert_eq!(
            snapshot
                .preexisting_cooling_latent_output_w
                .map(f64::to_bits),
            predecessor
                .predecessor_cp402_cooling_latent_output_w
                .map(f64::to_bits),
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
            (
                snapshot.resulting_supply_temperature_c,
                predecessor.resulting_supply_temperature_c,
            ),
        ] {
            assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
        }

        if assignment {
            let maximum = predecessor
                .predecessor_maximum_total_cooling_capacity_w
                .expect("active CP405 must retain the CP402 maximum-capacity operand");
            assert!(snapshot.cp404_retained_maximum_total_cooling_capacity_owned_read);
            assert!(snapshot.maximum_total_cooling_capacity_read);
            assert!(snapshot.cooling_latent_output_assigned);
            for value in [
                snapshot.maximum_total_cooling_capacity_w,
                snapshot.assigned_cooling_latent_output_w,
                snapshot.resulting_cooling_latent_output_w,
            ] {
                assert_eq!(value.map(f64::to_bits), Some(maximum.to_bits()));
            }
        } else {
            assert!(snapshot.maximum_total_cooling_capacity_w.is_none());
            assert!(snapshot.assigned_cooling_latent_output_w.is_none());
            assert_eq!(
                snapshot.resulting_cooling_latent_output_w.map(f64::to_bits),
                snapshot
                    .preexisting_cooling_latent_output_w
                    .map(f64::to_bits),
            );
        }

        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP405 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            2 * usize::from(assignment),
        );
        saw_assignment |= assignment;
        saw_skip |= !assignment;
    }
    assert!(saw_assignment, "fixture set must execute CP405");
    assert!(
        saw_skip,
        "fixture set must preserve a CP405 zero-site route"
    );
}
