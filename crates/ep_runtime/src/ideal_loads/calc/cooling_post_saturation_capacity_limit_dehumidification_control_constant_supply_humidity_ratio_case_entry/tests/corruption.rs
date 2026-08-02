//! CP398 predecessor, route, case-entry, and carrier corruption rejection.

use super::*;
use ep_model::DehumidificationControlType as D;

type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot;
type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntryRuntimeState;

fn active_case() -> (fixtures::Chain, Snapshot) {
    let chain = fixtures::chain(3, 1, true, Some(D::None), 1, 0.7, 18.0, 1.0);
    let mut state = State::new(chain.cp397.system);
    let snapshot = advance(&mut state, chain.cp397).expect("active CP398");
    (chain, snapshot)
}

#[test]
fn malformed_or_wrong_identity_cp397_predecessor_rejects_without_mutation() {
    let chain = active_case().0;
    let mut bad_source = chain.cp397;
    bad_source.source = "forged CP397 source";
    let mut bad_excluded = chain.cp397;
    bad_excluded.first_excluded_source = "forged CP397 excluded source";
    let mut bad_order = chain.cp397;
    bad_order.source_order = &["forged CP397 source order"];
    let mut bad_carrier = chain.cp397;
    bad_carrier.resulting_supply_enthalpy_j_per_kg =
        flip(bad_carrier.resulting_supply_enthalpy_j_per_kg);
    let mut bad_system = chain.cp397;
    bad_system.system = ep_model::IdealLoadsAirSystemId(bad_system.system.0 + 1);

    for predecessor in [bad_source, bad_excluded, bad_order, bad_carrier, bad_system] {
        let mut state = State::new(chain.cp397.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn active_control_entry_and_carrier_corruptions_are_rejected() {
    let snapshot = active_case().1;
    let mutations: &[fn(&mut Snapshot)] = &[
        |value| {
            value.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered =
                true
        },
        |value| {
            value
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break = true
        },
        |value| value.predecessor_dehumidification_control_humidistat_case_entered = true,
        |value| {
            value
                .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed = true
        },
        |value| {
            value.predecessor_dehumidification_control_humidistat_case_exited_via_break = true
        },
        |value| value.predecessor_dehumidification_control_none_case_entered = false,
        |value| {
            value
                .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered = false
        },
        |value| {
            value.predecessor_cp397_resulting_supply_humidity_ratio = Some(0.008)
        },
        |value| {
            value.predecessor_cp397_resulting_supply_enthalpy_j_per_kg =
                flip(value.predecessor_cp397_resulting_supply_enthalpy_j_per_kg)
        },
        |value| {
            value.predecessor_cp397_resulting_supply_temperature_c =
                flip(value.predecessor_cp397_resulting_supply_temperature_c)
        },
        |value| value.resulting_supply_humidity_ratio = Some(0.008),
        |value| {
            value.resulting_supply_enthalpy_j_per_kg =
                flip(value.resulting_supply_enthalpy_j_per_kg)
        },
        |value| value.resulting_supply_temperature_c = flip(value.resulting_supply_temperature_c),
    ];
    assert_corruptions_rejected(snapshot, mutations);
}

#[test]
fn inactive_routes_reject_forged_entries_and_carrier_presence() {
    let retained = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    );
    let mut retained_state = State::new(retained.cp397.system);
    let retained_snapshot = advance(&mut retained_state, retained.cp397).expect("retained-W CP398");
    assert_corruptions_rejected(
        retained_snapshot,
        &[
            |value| value.predecessor_dehumidification_control_none_case_entered = true,
            |value| {
                value
                    .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered = true
            },
        ],
    );

    let empty = fixtures::chain(3, 1, true, Some(D::None), 1, 0.7, 18.0, 1.0);
    let mut empty_state = State::new(empty.cp397.system);
    let empty_snapshot = advance(&mut empty_state, empty.cp397).expect("empty-W CP398");
    assert_corruptions_rejected(
        empty_snapshot,
        &[
            |value| value.predecessor_cp397_resulting_supply_humidity_ratio = Some(0.008),
            |value| value.resulting_supply_humidity_ratio = Some(0.008),
        ],
    );
}

#[test]
fn constant_supply_selector_enters_shared_case_without_none_fallthrough() {
    let chain = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSupplyHumidityRatio),
        1,
        0.7,
        18.0,
        1.0,
    );
    let mut state = State::new(chain.cp397.system);
    let snapshot = advance(&mut state, chain.cp397).expect("constant-supply CP398");
    assert!(!snapshot.predecessor_dehumidification_control_none_case_entered);
    assert!(
        snapshot
            .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered
    );
    assert_eq!(state.inactive_transition_count, 0);
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_case_entry_count,
        1
    );
}

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlNoneCaseEntrySnapshot,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_state(
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
        assert!(
            !cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_snapshot_is_exact(
                corrupted,
            )
        );
    }
}
