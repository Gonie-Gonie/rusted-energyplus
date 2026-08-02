use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_lifecycle_summary,
};

#[test]
fn binding_places_cp401_after_cp400_before_unchanged_numerical_coupling() {
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
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_sensible_output_assignment;
        let cooling_total_output_owner = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment;
        let cooling_total_output_corroborator = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment;
        let active = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_sensible_output_assignment_executed,
            active
        );
        assert_eq!(
            snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_assignment_executed,
            active
        );
        assert_eq!(
            snapshot.cp400_retained_supply_humidity_ratio_state_owned,
            predecessor.resulting_supply_humidity_ratio.is_some()
        );
        assert_eq!(
            snapshot.cp400_retained_supply_enthalpy_state_owned,
            predecessor.resulting_supply_enthalpy_j_per_kg.is_some()
        );
        assert_eq!(
            snapshot.cp400_retained_supply_temperature_state_owned,
            predecessor.resulting_supply_temperature_c.is_some()
        );
        for local in [
            snapshot.cp384_retained_cooling_total_output_owned_read,
            snapshot.cp385_cooling_total_output_bit_corroborated,
            snapshot.cooling_total_output_read,
            snapshot.cp400_retained_cooling_sensible_output_owned_read,
            snapshot.cooling_sensible_output_read,
            snapshot.cooling_latent_output_calculated,
            snapshot.cooling_latent_output_assigned,
        ] {
            assert_eq!(local, active);
        }

        if active {
            let cooling_total_output = cooling_total_output_owner
                .resulting_cooling_total_output_w
                .expect("active CP401 must read the CP384-owned cooling total output");
            let corroborating_total_output = cooling_total_output_corroborator
                .cooling_total_output_w
                .expect("active CP401 must corroborate cooling total output through CP385");
            let cooling_sensible_output = predecessor
                .cooling_sensible_output_w
                .expect("active CP401 must read the CP400-owned cooling sensible output");
            let cooling_latent_output = cooling_total_output - cooling_sensible_output;
            for (left, right) in [
                (snapshot.cooling_total_output_w, Some(cooling_total_output)),
                (Some(corroborating_total_output), Some(cooling_total_output)),
                (
                    snapshot.cooling_sensible_output_w,
                    Some(cooling_sensible_output),
                ),
                (
                    snapshot.calculated_cooling_latent_output_w,
                    Some(cooling_latent_output),
                ),
                (
                    snapshot.cooling_latent_output_w,
                    Some(cooling_latent_output),
                ),
            ] {
                assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
            }
        } else {
            for value in [
                snapshot.cooling_total_output_w,
                snapshot.cooling_sensible_output_w,
                snapshot.calculated_cooling_latent_output_w,
                snapshot.cooling_latent_output_w,
            ] {
                assert!(value.is_none());
            }
        }
        for (left, right) in [
            (
                snapshot.predecessor_cp400_resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp400_resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp400_resulting_supply_temperature_c,
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

        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP401 lifecycle");
        let assignments = usize::from(active);
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(lifecycle.state.inactive_transition_count, 1 - assignments);
        assert_eq!(
            lifecycle
                .state
                .dehumidification_control_constant_supply_humidity_ratio_latent_output_assignment_count,
            assignments
        );
        assert_eq!(lifecycle.state.source_site_execution_count, 4 * assignments);
        for count in [
            lifecycle.state.cooling_total_output_owned_read_count,
            lifecycle.state.cooling_total_output_bit_corroboration_count,
            lifecycle.state.cooling_total_output_read_count,
            lifecycle.state.cooling_sensible_output_owned_read_count,
            lifecycle.state.cooling_sensible_output_read_count,
            lifecycle.state.cooling_latent_output_calculation_count,
            lifecycle.state.cooling_latent_output_assignment_write_count,
        ] {
            assert_eq!(count, assignments);
        }
        saw_active |= active;
        saw_inactive |= !active;
    }
    assert!(saw_active, "fixture set must execute CP401");
    assert!(saw_inactive, "fixture set must also skip CP401");
}
