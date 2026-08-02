//! CP391 predecessor and snapshot corruption rejection.

use super::*;

fn active_case() -> (
    fixtures::Chain,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
){
    let chain = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    );
    let mut state =
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState::new(
            chain.cp390.system,
        );
    let snapshot =
        advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state(
            &mut state,
            chain.cp390,
        )
        .expect("active CP391");
    (chain, snapshot)
}

#[test]
fn malformed_or_wrong_identity_cp390_predecessor_rejects_without_mutation() {
    let chain = active_case().0;
    for predecessor in {
        let mut bad_source = chain.cp390;
        bad_source.source = "forged CP390 source";
        let mut bad_first_excluded_source = chain.cp390;
        bad_first_excluded_source.first_excluded_source = "forged CP390 excluded source";
        let mut bad_source_order = chain.cp390;
        bad_source_order.source_order = &["forged CP390 source order"];
        let mut bad_resulting_supply_enthalpy = chain.cp390;
        bad_resulting_supply_enthalpy.resulting_supply_enthalpy_j_per_kg =
            bad_resulting_supply_enthalpy
                .resulting_supply_enthalpy_j_per_kg
                .map(|value| f64::from_bits(value.to_bits() ^ 1));
        let mut bad_resulting_supply_temperature = chain.cp390;
        bad_resulting_supply_temperature.resulting_supply_temperature_c =
            bad_resulting_supply_temperature
                .resulting_supply_temperature_c
                .map(|value| f64::from_bits(value.to_bits() ^ 1));
        let mut bad_system = chain.cp390;
        bad_system.system = ep_model::IdealLoadsAirSystemId(bad_system.system.0 + 1);
        [
            bad_source,
            bad_first_excluded_source,
            bad_source_order,
            bad_resulting_supply_enthalpy,
            bad_resulting_supply_temperature,
            bad_system,
        ]
    } {
        let mut state =
            PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState::new(
                chain.cp390.system,
            );
        let before = state.clone();
        assert!(
            advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state(
                &mut state,
                predecessor,
            )
            .is_none()
        );
        assert_eq!(state, before);
    }
}

#[test]
fn every_local_active_flag_is_exactly_route_shaped() {
    let (_, snapshot) = active_case();
    let mutations: &[fn(
        &mut PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    )] = &[
        |value| {
            value.dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_executed =
                false
        },
        |value| value.cp390_retained_supply_enthalpy_state_owned = false,
        |value| value.cp390_retained_supply_enthalpy_owned_read = false,
        |value| value.supply_enthalpy_for_overdrying_limit_maximum_read = false,
        |value| value.cp390_retained_supply_temperature_owned_read = false,
        |value| value.supply_temperature_for_minimum_humidity_ratio_enthalpy_read = false,
        |value| value.psychrometric_minimum_supply_enthalpy_evaluated = false,
        |value| value.source_shaped_two_argument_maximum_evaluated = false,
        |value| value.supply_enthalpy_assignment_performed = false,
    ];
    for mutate in mutations {
        let mut corrupted = snapshot;
        mutate(&mut corrupted);
        assert!(
            !cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact(
                corrupted,
            )
        );
    }
}

#[test]
fn active_arithmetic_and_temperature_carrier_corruption_reject() {
    let (_, snapshot) = active_case();
    let mutations: &[fn(
        &mut PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
    )] = &[
        |value| {
            value.preexisting_supply_enthalpy_j_per_kg = value
                .preexisting_supply_enthalpy_j_per_kg
                .map(|v| f64::from_bits(v.to_bits() ^ 1))
        },
        |value| value.preexisting_supply_enthalpy_j_per_kg = None,
        |value| {
            value.supply_enthalpy_before_overdrying_limit_j_per_kg =
                value.supply_enthalpy_before_overdrying_limit_j_per_kg.map(|v| v + 1.0)
        },
        |value| value.supply_temperature_c = value.supply_temperature_c.map(|v| v + 1.0),
        |value| {
            value.psychrometric_minimum_supply_enthalpy_j_per_kg = value
                .psychrometric_minimum_supply_enthalpy_j_per_kg
                .map(|v| v + 1.0)
        },
        |value| {
            value.maximum_supply_enthalpy_j_per_kg =
                value.maximum_supply_enthalpy_j_per_kg.map(|v| v + 1.0)
        },
        |value| {
            value.assigned_supply_enthalpy_j_per_kg =
                value.assigned_supply_enthalpy_j_per_kg.map(|v| v + 1.0)
        },
        |value| {
            value.resulting_supply_enthalpy_j_per_kg =
                value.resulting_supply_enthalpy_j_per_kg.map(|v| v + 1.0)
        },
        |value| {
            value.resulting_supply_temperature_c =
                value.resulting_supply_temperature_c.map(|v| v + 1.0)
        },
    ];
    for mutate in mutations {
        let mut corrupted = snapshot;
        mutate(&mut corrupted);
        assert!(
            !cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact(
                corrupted,
            )
        );
    }
}

#[test]
fn inactive_routes_reject_source_local_payloads_and_bad_carriers() {
    let chain = fixtures::chain(3, 1, false, None, 1, 0.7, 18.0, 1.0);
    let mut state =
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState::new(
            chain.cp390.system,
        );
    let snapshot =
        advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state(
            &mut state,
            chain.cp390,
        )
        .expect("inactive CP391");

    let mut payload = snapshot;
    payload.supply_temperature_c = Some(18.0);
    assert!(
        !cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact(
            payload,
        )
    );

    let mut bad_owner = snapshot;
    bad_owner.cp390_retained_supply_enthalpy_state_owned = false;
    assert!(
        !cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact(
            bad_owner,
        )
    );

    let mut bad_result = snapshot;
    bad_result.resulting_supply_enthalpy_j_per_kg = bad_result
        .resulting_supply_enthalpy_j_per_kg
        .map(|v| v + 1.0);
    assert!(
        !cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_snapshot_is_exact(
            bad_result,
        )
    );
}
