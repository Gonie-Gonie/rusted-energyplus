//! CP395 predecessor, assignment, and carrier corruption rejection.

use super::*;
use ep_model::DehumidificationControlType as D;

type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot;
type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState;

fn active_case() -> (fixtures::Chain, Snapshot) {
    let chain = fixtures::chain(3, 1, true, Some(D::Humidistat), 1, 0.7, 18.0, 1.0);
    let mut state = State::new(chain.cp394.system);
    let snapshot = advance(&mut state, chain.cp394).expect("active CP395");
    (chain, snapshot)
}

#[test]
fn malformed_or_wrong_identity_cp394_predecessor_rejects_without_mutation() {
    let chain = active_case().0;
    let mut bad_source = chain.cp394;
    bad_source.source = "forged CP394 source";
    let mut bad_excluded = chain.cp394;
    bad_excluded.first_excluded_source = "forged CP394 excluded source";
    let mut bad_order = chain.cp394;
    bad_order.source_order = &["forged CP394 source order"];
    let mut bad_cp393_carrier = chain.cp394;
    bad_cp393_carrier.predecessor_cp393_resulting_supply_enthalpy_j_per_kg =
        flip(bad_cp393_carrier.predecessor_cp393_resulting_supply_enthalpy_j_per_kg);
    let mut bad_cp394_carrier = chain.cp394;
    bad_cp394_carrier.resulting_supply_temperature_c =
        flip(bad_cp394_carrier.resulting_supply_temperature_c);
    let mut bad_system = chain.cp394;
    bad_system.system = ep_model::IdealLoadsAirSystemId(bad_system.system.0 + 1);

    for predecessor in [
        bad_source,
        bad_excluded,
        bad_order,
        bad_cp393_carrier,
        bad_cp394_carrier,
        bad_system,
    ] {
        let mut state = State::new(chain.cp394.system);
        let before = state.clone();
        assert!(advance(&mut state, predecessor).is_none());
        assert_eq!(state, before);
    }
}

#[test]
fn every_active_flag_operand_result_and_preserved_carrier_is_exact() {
    let snapshot = active_case().1;
    let mutations: &[fn(&mut Snapshot)] = &[
        |value| {
            value.dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed =
                false
        },
        |value| value.cp394_retained_supply_humidity_ratio_state_owned = true,
        |value| value.cp394_retained_supply_temperature_state_owned = false,
        |value| value.cp394_retained_supply_enthalpy_state_owned = false,
        |value| value.cp394_retained_supply_temperature_owned_read = false,
        |value| value.supply_temperature_for_humidity_ratio_inversion_read = false,
        |value| value.cp394_retained_supply_enthalpy_owned_read = false,
        |value| value.supply_enthalpy_for_humidity_ratio_inversion_read = false,
        |value| value.psychrometric_supply_humidity_ratio_evaluated = false,
        |value| value.supply_humidity_ratio_assignment_performed = false,
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
            value.predecessor_cp394_resulting_supply_enthalpy_j_per_kg =
                flip(value.predecessor_cp394_resulting_supply_enthalpy_j_per_kg)
        },
        |value| {
            value.predecessor_cp394_resulting_supply_temperature_c =
                flip(value.predecessor_cp394_resulting_supply_temperature_c)
        },
    ];
    assert_corruptions_rejected(snapshot, mutations);
}

#[test]
fn inactive_routes_preserve_existing_humidity_ratio_and_reject_local_payloads() {
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
    let mut retained_state = State::new(retained.cp394.system);
    let retained_snapshot =
        advance(&mut retained_state, retained.cp394).expect("inactive retained-W CP395");
    assert_eq!(
        retained_snapshot
            .resulting_supply_humidity_ratio
            .map(f64::to_bits),
        retained
            .cp394
            .resulting_supply_humidity_ratio
            .map(f64::to_bits),
    );
    assert_corruptions_rejected(
        retained_snapshot,
        &[
            |value| value.cp394_retained_supply_humidity_ratio_state_owned = false,
            |value| {
                value.resulting_supply_humidity_ratio = flip(value.resulting_supply_humidity_ratio)
            },
        ],
    );

    let empty = fixtures::chain(3, 1, true, Some(D::None), 1, 0.7, 18.0, 1.0);
    let mut empty_state = State::new(empty.cp394.system);
    let empty_snapshot = advance(&mut empty_state, empty.cp394).expect("inactive empty-W CP395");
    assert_corruptions_rejected(
        empty_snapshot,
        &[
            |value| value.supply_temperature_c = Some(18.0),
            |value| value.supply_enthalpy_j_per_kg = Some(42_000.0),
            |value| value.psychrometric_supply_humidity_ratio = Some(0.008),
            |value| value.assigned_supply_humidity_ratio = Some(0.008),
            |value| value.resulting_supply_humidity_ratio = Some(0.008),
        ],
    );
}

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state(
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
            !cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact(
                corrupted,
            )
        );
    }
}
