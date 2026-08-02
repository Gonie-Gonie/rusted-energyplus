//! CP392 predecessor and snapshot corruption rejection.

use super::*;

type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot;
type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentRuntimeState;

fn active_case() -> (fixtures::Chain, Snapshot) {
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
    let mut state = State::new(chain.cp391.system);
    let snapshot = advance(&mut state, chain.cp391).expect("active CP392");
    (chain, snapshot)
}

#[test]
fn malformed_or_wrong_identity_cp391_predecessor_rejects_without_mutation() {
    let chain = active_case().0;
    for predecessor in {
        let mut bad_source = chain.cp391;
        bad_source.source = "forged CP391 source";
        let mut bad_excluded = chain.cp391;
        bad_excluded.first_excluded_source = "forged CP391 excluded source";
        let mut bad_order = chain.cp391;
        bad_order.source_order = &["forged CP391 source order"];
        let mut bad_enthalpy = chain.cp391;
        bad_enthalpy.resulting_supply_enthalpy_j_per_kg =
            flip(bad_enthalpy.resulting_supply_enthalpy_j_per_kg);
        let mut bad_temperature = chain.cp391;
        bad_temperature.resulting_supply_temperature_c =
            flip(bad_temperature.resulting_supply_temperature_c);
        let mut bad_system = chain.cp391;
        bad_system.system = ep_model::IdealLoadsAirSystemId(bad_system.system.0 + 1);
        [
            bad_source,
            bad_excluded,
            bad_order,
            bad_enthalpy,
            bad_temperature,
            bad_system,
        ]
    } {
        let mut state = State::new(chain.cp391.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn every_local_active_flag_is_exactly_route_shaped() {
    let snapshot = active_case().1;
    let mutations: &[fn(&mut Snapshot)] = &[
        |value| {
            value.dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed = false
        },
        |value| value.cp391_retained_supply_temperature_state_owned = false,
        |value| value.cp391_retained_supply_enthalpy_state_owned = false,
        |value| value.cp391_retained_supply_temperature_owned_read = false,
        |value| value.supply_temperature_for_humidity_ratio_inversion_read = false,
        |value| value.cp391_retained_supply_enthalpy_owned_read = false,
        |value| value.supply_enthalpy_for_humidity_ratio_inversion_read = false,
        |value| value.psychrometric_supply_humidity_ratio_evaluated = false,
        |value| value.supply_humidity_ratio_assignment_performed = false,
    ];
    assert_corruptions_rejected(snapshot, mutations);
}

#[test]
fn active_arithmetic_and_unchanged_carrier_corruption_reject() {
    let snapshot = active_case().1;
    let mutations: &[fn(&mut Snapshot)] = &[
        |value| value.supply_temperature_c = flip(value.supply_temperature_c),
        |value| value.supply_enthalpy_j_per_kg = flip(value.supply_enthalpy_j_per_kg),
        |value| {
            value.psychrometric_supply_humidity_ratio =
                flip(value.psychrometric_supply_humidity_ratio)
        },
        |value| value.assigned_supply_humidity_ratio = flip(value.assigned_supply_humidity_ratio),
        |value| value.resulting_supply_humidity_ratio = flip(value.resulting_supply_humidity_ratio),
        |value| {
            value.resulting_supply_enthalpy_j_per_kg =
                flip(value.resulting_supply_enthalpy_j_per_kg)
        },
        |value| value.resulting_supply_temperature_c = flip(value.resulting_supply_temperature_c),
        |value| {
            value.predecessor_cp391_resulting_supply_enthalpy_j_per_kg =
                flip(value.predecessor_cp391_resulting_supply_enthalpy_j_per_kg)
        },
        |value| {
            value.predecessor_cp391_resulting_supply_temperature_c =
                flip(value.predecessor_cp391_resulting_supply_temperature_c)
        },
    ];
    assert_corruptions_rejected(snapshot, mutations);
}

#[test]
fn inactive_routes_reject_local_payloads_and_bad_ownership() {
    let chain = fixtures::chain(3, 1, false, None, 1, 0.7, 18.0, 1.0);
    let mut state = State::new(chain.cp391.system);
    let snapshot = advance(&mut state, chain.cp391).expect("inactive CP392");
    let mutations: &[fn(&mut Snapshot)] = &[
        |value| value.supply_temperature_c = Some(18.0),
        |value| value.supply_enthalpy_j_per_kg = Some(42_000.0),
        |value| value.psychrometric_supply_humidity_ratio = Some(0.008),
        |value| value.assigned_supply_humidity_ratio = Some(0.008),
        |value| value.resulting_supply_humidity_ratio = Some(0.008),
        |value| value.cp391_retained_supply_temperature_state_owned = false,
        |value| value.cp391_retained_supply_enthalpy_state_owned = false,
        |value| value.resulting_supply_temperature_c = flip(value.resulting_supply_temperature_c),
        |value| {
            value.resulting_supply_enthalpy_j_per_kg =
                flip(value.resulting_supply_enthalpy_j_per_kg)
        },
    ];
    assert_corruptions_rejected(snapshot, mutations);
}

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_state(state, predecessor)
}

fn flip(value: Option<f64>) -> Option<f64> {
    value.map(|value| f64::from_bits(value.to_bits() ^ 1))
}

fn assert_corruptions_rejected(snapshot: Snapshot, mutations: &[fn(&mut Snapshot)]) {
    for mutate in mutations {
        let mut corrupted = snapshot;
        mutate(&mut corrupted);
        assert!(!cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_snapshot_is_exact(corrupted));
    }
}
