//! CP393 predecessor and compressed-snapshot corruption rejection.

use super::*;
use ep_model::DehumidificationControlType as D;

type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakSnapshot;
type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakRuntimeState;

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
    let mut state = State::new(chain.cp392.system);
    let snapshot = advance(&mut state, chain.cp392).expect("active CP393");
    (chain, snapshot)
}

#[test]
fn malformed_or_wrong_identity_cp392_predecessor_rejects_without_mutation() {
    let chain = active_case().0;
    let mut bad_source = chain.cp392;
    bad_source.source = "forged CP392 source";
    let mut bad_excluded = chain.cp392;
    bad_excluded.first_excluded_source = "forged CP392 exclusion";
    let mut bad_order = chain.cp392;
    bad_order.source_order = &["forged CP392 order"];
    let mut bad_humidity = chain.cp392;
    bad_humidity.resulting_supply_humidity_ratio =
        flip(bad_humidity.resulting_supply_humidity_ratio);
    let mut bad_system = chain.cp392;
    bad_system.system = ep_model::IdealLoadsAirSystemId(bad_system.system.0 + 1);

    for predecessor in [
        bad_source,
        bad_excluded,
        bad_order,
        bad_humidity,
        bad_system,
    ] {
        let mut state = State::new(chain.cp392.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn active_control_and_carrier_corruptions_are_rejected() {
    let snapshot = active_case().1;
    let mutations: &[fn(&mut Snapshot)] = &[
        |value| {
            value.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered =
                false
        },
        |value| {
            value.predecessor_dehumidification_control_constant_sensible_heat_ratio_supply_humidity_ratio_assignment_executed = false
        },
        |value| {
            value.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break =
                false
        },
        |value| {
            value.predecessor_cp392_resulting_supply_humidity_ratio =
                flip(value.predecessor_cp392_resulting_supply_humidity_ratio)
        },
        |value| {
            value.resulting_supply_enthalpy_j_per_kg =
                flip(value.resulting_supply_enthalpy_j_per_kg)
        },
        |value| value.resulting_supply_temperature_c = flip(value.resulting_supply_temperature_c),
    ];
    assert_corruptions_rejected(snapshot, mutations);
}

#[test]
fn inactive_selector_and_presence_corruptions_are_rejected() {
    let chain = fixtures::chain(3, 1, true, Some(D::None), 1, 0.7, 18.0, 1.0);
    let mut state = State::new(chain.cp392.system);
    let snapshot = advance(&mut state, chain.cp392).expect("inactive CP393");
    let mutations: &[fn(&mut Snapshot)] = &[
        |value| {
            value.dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break = true
        },
        |value| {
            value.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered =
                true
        },
        |value| value.resulting_supply_humidity_ratio = Some(0.008),
        |value| value.predecessor_cp392_resulting_supply_enthalpy_j_per_kg = None,
        |value| value.resulting_supply_temperature_c = None,
    ];
    assert_corruptions_rejected(snapshot, mutations);
}

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyHumidityRatioAssignmentSnapshot,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_state(
        state,
        predecessor,
    )
}

fn flip(value: Option<f64>) -> Option<f64> {
    value.map(|value| f64::from_bits(value.to_bits() ^ 1))
}

fn assert_corruptions_rejected(snapshot: Snapshot, mutations: &[fn(&mut Snapshot)]) {
    for mutate in mutations {
        let mut corrupted = snapshot;
        mutate(&mut corrupted);
        assert!(!cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_break_snapshot_is_exact(corrupted));
    }
}
