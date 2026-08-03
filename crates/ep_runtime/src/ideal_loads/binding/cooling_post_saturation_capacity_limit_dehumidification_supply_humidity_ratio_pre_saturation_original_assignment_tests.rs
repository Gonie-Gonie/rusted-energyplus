use super::*;
use crate::ideal_loads::cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release;

#[test]
fn binding_places_cp411_after_cp410_before_unchanged_numerical_coupling() {
    let mut saw_assignment = false;
    let mut saw_skip = false;

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
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_control_default_case_break;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(snapshot.system, predecessor.system);
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);
        assert_eq!(
            snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed,
            predecessor.predecessor_dehumidification_control_switch_dispatched,
        );
        assert_eq!(
            snapshot.local_supply_humidity_ratio_original_assignment_performed,
            snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed,
        );

        for (left, right) in [
            (
                snapshot.predecessor_cp410_resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp410_resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp410_resulting_supply_temperature_c,
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

        if snapshot
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_pre_saturation_original_assignment_executed
        {
            assert!(snapshot.cp410_retained_supply_humidity_ratio_owned_read);
            assert!(snapshot.purchased_air_supply_humidity_ratio_read);
            assert_eq!(
                snapshot
                    .purchased_air_supply_humidity_ratio_before_saturation_check
                    .map(f64::to_bits),
                predecessor.resulting_supply_humidity_ratio.map(f64::to_bits),
            );
            assert_eq!(
                snapshot.assigned_supply_humidity_ratio_original.map(f64::to_bits),
                predecessor.resulting_supply_humidity_ratio.map(f64::to_bits),
            );
            assert_eq!(
                snapshot.resulting_supply_humidity_ratio_original.map(f64::to_bits),
                predecessor.resulting_supply_humidity_ratio.map(f64::to_bits),
            );
            saw_assignment = true;
        } else {
            assert!(!snapshot.cp410_retained_supply_humidity_ratio_owned_read);
            assert!(!snapshot.purchased_air_supply_humidity_ratio_read);
            assert!(snapshot.purchased_air_supply_humidity_ratio_before_saturation_check.is_none());
            assert!(snapshot.assigned_supply_humidity_ratio_original.is_none());
            assert!(snapshot.resulting_supply_humidity_ratio_original.is_none());
            saw_skip = true;
        }

        assert!(
            output
                .coupling
                .purchased_air
                .supply_node_update
                .humidity_ratio
                .is_finite(),
            "CP411 evidence must not replace the numerical PurchasedAir output",
        );
    }

    assert!(saw_assignment);
    assert!(saw_skip);
}
