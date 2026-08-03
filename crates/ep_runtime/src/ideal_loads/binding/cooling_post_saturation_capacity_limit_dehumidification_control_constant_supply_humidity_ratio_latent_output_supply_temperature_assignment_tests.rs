use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_lifecycle_summary,
};

#[test]
fn binding_places_cp407_after_cp406_before_unchanged_numerical_coupling() {
    let mut saw_assignment = false;
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
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry;
        let humidity_owner =
            output.calculation_cooling_supply_humidity_ratio_saturation_limit_assignment;
        let enthalpy_owner = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment;
        let assignment = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered,
            assignment,
        );
        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed,
            assignment,
        );
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(snapshot.system, predecessor.system);
        assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);

        if assignment {
            let enthalpy = enthalpy_owner
                .resulting_supply_enthalpy_j_per_kg
                .expect("active CP407 CP385 enthalpy owner");
            let humidity = humidity_owner
                .resulting_supply_humidity_ratio
                .expect("active CP407 CP378 humidity owner");
            let preexisting = predecessor
                .resulting_supply_temperature_c
                .expect("active CP407 CP406 temperature owner");
            let expected = crate::psychrometrics::energyplus_psy_tdb_fn_h_w(enthalpy, humidity);
            assert!(snapshot.cp385_retained_supply_enthalpy_owned_read);
            assert!(snapshot.cp406_same_call_supply_enthalpy_bit_corroborated);
            assert!(snapshot.supply_enthalpy_for_dry_bulb_inversion_read);
            assert!(snapshot.cp378_retained_supply_humidity_ratio_owned_read);
            assert!(snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read);
            assert!(snapshot.cp406_retained_supply_temperature_state_owned);
            assert!(snapshot.psychrometric_supply_temperature_evaluated);
            assert!(snapshot.supply_temperature_assigned);
            for (actual, expected) in [
                (snapshot.supply_enthalpy_j_per_kg, enthalpy),
                (snapshot.supply_humidity_ratio, humidity),
                (snapshot.preexisting_supply_temperature_c, preexisting),
                (snapshot.psychrometric_supply_temperature_result_c, expected),
                (snapshot.assigned_supply_temperature_c, expected),
                (snapshot.resulting_supply_humidity_ratio, humidity),
                (snapshot.resulting_supply_enthalpy_j_per_kg, enthalpy),
                (snapshot.resulting_supply_temperature_c, expected),
            ] {
                assert_eq!(actual.map(f64::to_bits), Some(expected.to_bits()));
            }
        } else {
            for flag in [
                snapshot.cp385_retained_supply_enthalpy_owned_read,
                snapshot.cp406_same_call_supply_enthalpy_bit_corroborated,
                snapshot.supply_enthalpy_for_dry_bulb_inversion_read,
                snapshot.cp378_retained_supply_humidity_ratio_owned_read,
                snapshot.supply_humidity_ratio_for_dry_bulb_inversion_read,
                snapshot.psychrometric_supply_temperature_evaluated,
                snapshot.supply_temperature_assigned,
            ] {
                assert!(!flag);
            }
            for value in [
                snapshot.supply_enthalpy_j_per_kg,
                snapshot.supply_humidity_ratio,
                snapshot.psychrometric_supply_temperature_result_c,
                snapshot.assigned_supply_temperature_c,
            ] {
                assert!(value.is_none());
            }
            for (actual, expected) in [
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
                assert_eq!(actual.map(f64::to_bits), expected.map(f64::to_bits));
            }
        }

        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP407 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.inactive_transition_count,
            usize::from(!assignment)
        );
        assert_eq!(
            lifecycle.state.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_count,
            usize::from(assignment),
        );
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            4 * usize::from(assignment)
        );

        saw_assignment |= assignment;
        saw_cp405_sibling |= predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;
        saw_inherited_inactive |= !assignment
            && !predecessor
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;
    }
    assert!(saw_assignment, "fixture set must execute CP407");
    assert!(
        saw_cp405_sibling,
        "fixture set must preserve the CP405 sibling"
    );
    assert!(
        saw_inherited_inactive,
        "fixture set must preserve inherited inactivity"
    );
}
