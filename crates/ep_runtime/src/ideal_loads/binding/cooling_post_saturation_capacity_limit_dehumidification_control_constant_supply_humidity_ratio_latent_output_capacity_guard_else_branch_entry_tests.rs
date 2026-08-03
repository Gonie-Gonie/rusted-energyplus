use super::*;
use crate::ideal_loads::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot_is_exact_direct_release,
    purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_lifecycle_summary,
};

#[test]
fn binding_places_cp406_after_cp405_before_unchanged_numerical_coupling() {
    let mut saw_else_entry = false;
    let mut saw_true_sibling_skip = false;
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
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry;
        let guard_false = predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough;
        let assignment = predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;
        let else_entry = guard_false && !assignment;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(
            snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
            guard_false,
        );
        assert_eq!(
            snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
            assignment,
        );
        assert_eq!(
            snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered,
            else_entry,
        );
        for (left, right) in [
            (
                snapshot.predecessor_cp405_resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp405_resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp405_resulting_supply_temperature_c,
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

        let lifecycle = purchased_air_calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_lifecycle_summary(
            &runtime,
            output.initialization.system,
        )
        .expect("CP406 lifecycle");
        assert_eq!(lifecycle.state.transition_count, 1);
        assert_eq!(
            lifecycle.state.source_site_execution_count,
            usize::from(else_entry),
        );

        saw_else_entry |= else_entry;
        saw_true_sibling_skip |= assignment;
        saw_inherited_inactive |= !guard_false && !assignment;
    }
    assert!(
        saw_else_entry,
        "fixture set must enter the CP406 else branch"
    );
    assert!(
        saw_true_sibling_skip,
        "fixture set must skip CP406 through the CP405 true body"
    );
    assert!(
        saw_inherited_inactive,
        "fixture set must preserve an inherited inactive route"
    );
}
