use super::*;
use crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshot_is_exact_direct_release;

#[test]
fn binding_places_cp410_after_cp409_before_unchanged_numerical_coupling() {
    let mut saw_cp409_shared_break = false;
    let mut saw_cp409_shared_break_skip = false;

    for (limit, humidity_ratio, availability, capacity) in [
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 500.0),
        (IdealLoadsLimit::LimitCapacity, 0.020, 1.0, 1.0e-100),
        (IdealLoadsLimit::NoLimit, 0.008, 0.0, 5_000.0),
    ] {
        let (_runtime, output) = super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
            limit,
            humidity_ratio,
            availability,
            capacity,
        );
        let predecessor = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break_snapshot_is_exact_direct_release(snapshot)
        );
        assert!(!snapshot.dehumidification_control_default_case_exited_via_break);
        assert_eq!(
            snapshot
                .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
            predecessor
                .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        );
        assert_eq!(snapshot.system, predecessor.system);
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);

        for (left, right) in [
            (
                snapshot.predecessor_cp409_resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp409_resulting_supply_temperature_c,
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

        saw_cp409_shared_break |= predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break;
        saw_cp409_shared_break_skip |= !predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break;

        assert!(
            output
                .coupling
                .purchased_air
                .supply_node_update
                .humidity_ratio
                .is_finite(),
            "CP410 evidence must not replace the numerical PurchasedAir output",
        );
    }

    assert!(saw_cp409_shared_break);
    assert!(saw_cp409_shared_break_skip);
}
