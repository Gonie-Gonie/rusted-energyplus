//! CP396 predecessor, route, break, and carrier corruption rejection.

use super::*;
use ep_model::DehumidificationControlType as D;

type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakSnapshot;
type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseBreakRuntimeState;

fn active_case() -> (fixtures::Chain, Snapshot) {
    let chain = fixtures::chain(3, 1, true, Some(D::Humidistat), 1, 0.7, 18.0, 1.0);
    let mut state = State::new(chain.cp395.system);
    let snapshot = advance(&mut state, chain.cp395).expect("active CP396");
    (chain, snapshot)
}

#[test]
fn malformed_or_wrong_identity_cp395_predecessor_rejects_without_mutation() {
    let chain = active_case().0;
    let mut bad_source = chain.cp395;
    bad_source.source = "forged CP395 source";
    let mut bad_excluded = chain.cp395;
    bad_excluded.first_excluded_source = "forged CP395 excluded source";
    let mut bad_order = chain.cp395;
    bad_order.source_order = &["forged CP395 source order"];
    let mut bad_carrier = chain.cp395;
    bad_carrier.resulting_supply_enthalpy_j_per_kg =
        flip(bad_carrier.resulting_supply_enthalpy_j_per_kg);
    let mut bad_system = chain.cp395;
    bad_system.system = ep_model::IdealLoadsAirSystemId(bad_system.system.0 + 1);

    for predecessor in [bad_source, bad_excluded, bad_order, bad_carrier, bad_system] {
        let mut state = State::new(chain.cp395.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn active_control_break_and_carrier_corruptions_are_rejected() {
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
        |value| value.predecessor_dehumidification_control_humidistat_case_entered = false,
        |value| {
            value
                .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed = false
        },
        |value| value.dehumidification_control_humidistat_case_exited_via_break = false,
        |value| {
            value.predecessor_cp395_resulting_supply_humidity_ratio =
                flip(value.predecessor_cp395_resulting_supply_humidity_ratio)
        },
        |value| {
            value.predecessor_cp395_resulting_supply_enthalpy_j_per_kg =
                flip(value.predecessor_cp395_resulting_supply_enthalpy_j_per_kg)
        },
        |value| {
            value.predecessor_cp395_resulting_supply_temperature_c =
                flip(value.predecessor_cp395_resulting_supply_temperature_c)
        },
        |value| value.resulting_supply_humidity_ratio = flip(value.resulting_supply_humidity_ratio),
        |value| {
            value.resulting_supply_enthalpy_j_per_kg =
                flip(value.resulting_supply_enthalpy_j_per_kg)
        },
        |value| value.resulting_supply_temperature_c = flip(value.resulting_supply_temperature_c),
    ];
    assert_corruptions_rejected(snapshot, mutations);
}

#[test]
fn inactive_routes_reject_forged_breaks_and_carrier_presence() {
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
    let mut retained_state = State::new(retained.cp395.system);
    let retained_snapshot = advance(&mut retained_state, retained.cp395).expect("retained-W CP396");
    assert_corruptions_rejected(
        retained_snapshot,
        &[|value| value.dehumidification_control_humidistat_case_exited_via_break = true],
    );

    let empty = fixtures::chain(3, 1, true, Some(D::None), 1, 0.7, 18.0, 1.0);
    let mut empty_state = State::new(empty.cp395.system);
    let empty_snapshot = advance(&mut empty_state, empty.cp395).expect("empty-W CP396");
    assert_corruptions_rejected(
        empty_snapshot,
        &[
            |value| value.predecessor_cp395_resulting_supply_humidity_ratio = Some(0.008),
            |value| value.resulting_supply_humidity_ratio = Some(0.008),
        ],
    );
}

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_state(
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
            !cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_break_snapshot_is_exact(
                corrupted,
            )
        );
    }
}
