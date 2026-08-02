//! CP394 predecessor and compressed-snapshot corruption rejection.

use super::*;
use ep_model::DehumidificationControlType as D;

type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot;
type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntryRuntimeState;

fn active_case() -> (fixtures::Chain, Snapshot) {
    let chain = fixtures::chain(3, 1, true, Some(D::Humidistat), 1, 0.7, 18.0, 1.0);
    let mut state = State::new(chain.cp393.system);
    let snapshot = advance(&mut state, chain.cp393).expect("active CP394");
    (chain, snapshot)
}

#[test]
fn malformed_or_wrong_identity_cp393_predecessor_rejects_without_mutation() {
    let chain = active_case().0;
    let mut bad_source = chain.cp393;
    bad_source.source = "forged CP393 source";
    let mut bad_order = chain.cp393;
    bad_order.source_order = &["forged CP393 order"];
    let mut bad_enthalpy = chain.cp393;
    bad_enthalpy.resulting_supply_enthalpy_j_per_kg =
        flip(bad_enthalpy.resulting_supply_enthalpy_j_per_kg);
    let mut bad_system = chain.cp393;
    bad_system.system = ep_model::IdealLoadsAirSystemId(bad_system.system.0 + 1);

    for predecessor in [bad_source, bad_order, bad_enthalpy, bad_system] {
        let mut state = State::new(chain.cp393.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn entry_break_and_carrier_corruptions_are_rejected() {
    let snapshot = active_case().1;
    let mutations: &[fn(&mut Snapshot)] = &[
        |value| value.dehumidification_control_humidistat_case_entered = false,
        |value| {
            value.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break = true
        },
        |value| {
            value.predecessor_cp393_resulting_supply_enthalpy_j_per_kg =
                flip(value.predecessor_cp393_resulting_supply_enthalpy_j_per_kg)
        },
        |value| value.resulting_supply_temperature_c = flip(value.resulting_supply_temperature_c),
        |value| value.resulting_supply_humidity_ratio = Some(0.008),
    ];
    assert_corruptions_rejected(snapshot, mutations);
}

#[test]
fn constant_shr_break_never_falls_through_the_humidistat_label() {
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
    let mut state = State::new(chain.cp393.system);
    let snapshot = advance(&mut state, chain.cp393).expect("constant-SHR CP394 skip");
    assert!(snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break);
    assert!(!snapshot.dehumidification_control_humidistat_case_entered);
    assert_eq!(state.source_site_execution_count, 0);
}

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakSnapshot,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state(state, predecessor)
}

fn flip(value: Option<f64>) -> Option<f64> {
    value.map(|value| f64::from_bits(value.to_bits() ^ 1))
}

fn assert_corruptions_rejected(snapshot: Snapshot, mutations: &[fn(&mut Snapshot)]) {
    for mutate in mutations {
        let mut corrupted = snapshot;
        mutate(&mut corrupted);
        assert!(!cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_snapshot_is_exact(corrupted));
    }
}
