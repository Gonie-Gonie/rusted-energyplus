use super::*;
use crate::{
    ideal_loads::{
        cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release,
        purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_lifecycle_summary,
    },
    psychrometrics::energyplus_psy_w_fn_tdb_h,
};

#[test]
fn binding_places_cp404_after_cp403_before_unchanged_numerical_coupling() {
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
        let cp385_owner = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment;
        let assignment = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_temperature_mixed_air_assignment_executed;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_adjustment_supply_humidity_ratio_assignment_executed,
            assignment,
        );
        for (left, right) in [
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
            let temperature = predecessor
                .resulting_supply_temperature_c
                .expect("active CP404 must consume CP403 supply temperature");
            let enthalpy = cp385_owner
                .resulting_supply_enthalpy_j_per_kg
                .expect("active CP404 must consume CP385-owned supply enthalpy");
            assert!(cp385_owner.supply_enthalpy_assignment_executed);
            assert_eq!(
                predecessor
                    .resulting_supply_enthalpy_j_per_kg
                    .map(f64::to_bits),
                Some(enthalpy.to_bits()),
            );
            let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);
            assert_eq!(
                snapshot.supply_temperature_c.map(f64::to_bits),
                Some(temperature.to_bits()),
            );
            assert_eq!(
                snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
                Some(enthalpy.to_bits()),
            );
            for value in [
                snapshot.psychrometric_supply_humidity_ratio,
                snapshot.assigned_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ] {
                assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
            }
            for flag in [
                snapshot.cp403_retained_supply_temperature_owned_read,
                snapshot.supply_temperature_for_humidity_ratio_inversion_read,
                snapshot.cp403_retained_supply_enthalpy_owned_read,
                snapshot.supply_enthalpy_for_humidity_ratio_inversion_read,
                snapshot.psychrometric_supply_humidity_ratio_evaluated,
                snapshot.supply_humidity_ratio_assignment_performed,
            ] {
                assert!(flag);
            }
        } else {
            for value in [
                snapshot.supply_temperature_c,
                snapshot.supply_enthalpy_j_per_kg,
                snapshot.psychrometric_supply_humidity_ratio,
                snapshot.assigned_supply_humidity_ratio,
            ] {
                assert!(value.is_none());
            }
            assert_eq!(
                snapshot.resulting_supply_humidity_ratio.map(f64::to_bits),
                predecessor
                    .resulting_supply_humidity_ratio
                    .map(f64::to_bits),
            );
        }

        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_humidity_ratio_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP404 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            4 * usize::from(assignment),
        );
        saw_assignment |= assignment;
        saw_inactive |= !assignment;
    }
    assert!(saw_assignment, "fixture set must execute CP404");
    assert!(saw_inactive, "fixture set must preserve a CP404 skip route");
}
