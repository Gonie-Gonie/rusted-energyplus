use super::*;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationSupplyHumidityRatioSaturationGuardError as Cp413Error,
    advance_direct_no_oa_calc_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard as advance_cp413,
    cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release,
};

#[test]
fn binding_places_cp413_after_cp412_before_unchanged_numerical_coupling() {
    let mut saw_body = false;
    let mut saw_false_fallthrough = false;
    let mut saw_inactive = false;

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
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;
        let snapshot = output
            .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard;
        let active = predecessor
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed;

        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_snapshot_is_exact_direct_release(snapshot)
        );
        assert_eq!(snapshot.system, predecessor.system);
        assert_eq!(
            snapshot.parent_call_ordinal,
            predecessor.parent_call_ordinal
        );
        assert_eq!(snapshot.controlled_zone, predecessor.controlled_zone);
        for flag in [
            snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_guard_evaluated,
            snapshot.cp412_saturation_supply_humidity_ratio_owned_read,
            snapshot.saturation_supply_humidity_ratio_for_guard_read,
            snapshot.cp411_original_supply_humidity_ratio_owned_read,
            snapshot.cp412_same_call_original_supply_humidity_ratio_bit_corroborated,
            snapshot.original_supply_humidity_ratio_for_guard_read,
            snapshot.saturation_original_supply_humidity_ratio_comparison_evaluated,
        ] {
            assert_eq!(flag, active);
        }

        for (left, right) in [
            (
                snapshot.predecessor_cp412_resulting_supply_humidity_ratio,
                predecessor.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp412_resulting_supply_enthalpy_j_per_kg,
                predecessor.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp412_resulting_supply_temperature_c,
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

        if active {
            let saturation = predecessor
                .resulting_saturation_supply_humidity_ratio
                .expect("active CP413 requires the CP412 saturation humidity ratio");
            let original = predecessor.resulting_supply_humidity_ratio_original.expect(
                "active CP413 requires the recursively retained CP411 original humidity ratio",
            );
            assert_eq!(
                snapshot
                    .saturation_supply_humidity_ratio_for_guard
                    .map(f64::to_bits),
                Some(saturation.to_bits()),
            );
            assert_eq!(
                snapshot
                    .original_supply_humidity_ratio_for_guard
                    .map(f64::to_bits),
                Some(original.to_bits()),
            );
            assert_eq!(
                snapshot.saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio,
                Some(saturation < original),
            );
            assert_eq!(
                snapshot.saturation_supply_humidity_ratio_guard_body_entered,
                saturation < original,
            );
            assert_eq!(
                snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough,
                saturation >= original,
            );
            saw_body |= saturation < original;
            saw_false_fallthrough |= saturation >= original;
        } else {
            assert!(
                snapshot
                    .saturation_supply_humidity_ratio_for_guard
                    .is_none()
            );
            assert!(snapshot.original_supply_humidity_ratio_for_guard.is_none());
            assert!(
                snapshot
                    .saturation_supply_humidity_ratio_strictly_less_than_original_supply_humidity_ratio
                    .is_none()
            );
            assert!(!snapshot.saturation_supply_humidity_ratio_guard_body_entered);
            assert!(!snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough);
            saw_inactive = true;
        }

        assert!(
            output
                .coupling
                .purchased_air
                .supply_node_update
                .humidity_ratio
                .is_finite(),
            "CP413 evidence must not replace the numerical PurchasedAir output",
        );
    }

    assert!(saw_body);
    assert!(saw_false_fallthrough);
    assert!(saw_inactive);
}

#[test]
fn binding_cp413_active_nonfinite_operands_are_transactional_and_fail_closed() {
    let (runtime, output) = super::cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_temperature_assignment_tests::run_case(
        IdealLoadsLimit::LimitCapacity,
        0.020,
        1.0,
        500.0,
    );
    let predecessor = output
        .calculation_cooling_post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment;
    assert!(
        predecessor
            .post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_saturation_assignment_executed
    );

    let (model, _) = fixture(|typed| {
        let system = &mut typed.ideal_loads_air_systems[0];
        system.cooling_limit = IdealLoadsLimit::LimitCapacity;
        system.maximum_cooling_air_flow_rate_m3_per_s = None;
        system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(500.0));
        system.dehumidification_control_type = DehumidificationControlType::None;
        system.humidification_control_type = HumidificationControlType::None;
        system.minimum_cooling_supply_air_humidity_ratio = f64::NAN;
        schedule_mut(typed, ScheduleId(3)).hourly_value = 1.0;
    });
    let system = &model.typed.ideal_loads_air_systems[0];

    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut invalid = predecessor;
        invalid.resulting_saturation_supply_humidity_ratio = Some(value);
        let mut rejected = runtime.clone();
        let before = rejected.clone();
        assert_eq!(
            advance_cp413(&mut rejected, system, invalid),
            Err(Cp413Error::SaturationHumidityRatioOutsideDirectSubset {
                system: system.id,
                bits: value.to_bits(),
            })
        );
        assert_eq!(rejected, before);

        let mut invalid = predecessor;
        invalid.resulting_supply_humidity_ratio_original = Some(value);
        invalid.predecessor_cp411_resulting_supply_humidity_ratio = Some(value);
        let mut rejected = runtime.clone();
        let before = rejected.clone();
        assert_eq!(
            advance_cp413(&mut rejected, system, invalid),
            Err(Cp413Error::OriginalHumidityRatioOutsideDirectSubset {
                system: system.id,
                bits: value.to_bits(),
            })
        );
        assert_eq!(rejected, before);
    }
}
