//! CP395 exhaustive route, ownership, and carrier tests.

use super::*;
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState;
type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentSnapshot;

#[test]
fn cp395_preserves_thirty_routes_and_assigns_exactly_three_humidistat_routes() {
    let chains = fixtures::all_chains();
    let mut state = State::new(chains[0].cp394.system);
    let snapshots: Vec<_> = chains
        .iter()
        .map(|chain| advance(&mut state, chain.cp394).expect("CP395 route"))
        .collect();

    assert_eq!(snapshots.len(), 30);
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 27);
    assert_eq!(
        state.dehumidification_control_humidistat_supply_humidity_ratio_assignment_count,
        3
    );
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 12);
    assert_eq!(state.cp394_supply_humidity_ratio_state_owner_count, 3);
    assert_eq!(state.unchanged_supply_humidity_ratio_preservation_count, 3);
    assert_eq!(state.cp394_supply_temperature_state_owner_count, 27);
    assert_eq!(state.unchanged_supply_temperature_preservation_count, 27);
    assert_eq!(state.cp394_supply_enthalpy_state_owner_count, 17);
    assert_eq!(state.unchanged_supply_enthalpy_preservation_count, 17);
    for count in active_counters(&state) {
        assert_eq!(count, 3);
    }
    assert_eq!(
        snapshots
            .iter()
            .filter(|value| exact_direct(**value))
            .count(),
        11
    );

    for (index, (chain, snapshot)) in chains.into_iter().zip(snapshots).enumerate() {
        assert!(exact(snapshot));
        assert_eq!(exact_direct(snapshot), matches!(index, 0..=8 | 20 | 24));
        let active = matches!(index, 19 | 23 | 26);
        assert_eq!(
            snapshot.dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
            active,
        );
        assert_eq!(
            snapshot.cp394_retained_supply_humidity_ratio_state_owned,
            matches!(index, 18 | 22 | 28),
        );
        assert_eq!(
            snapshot.cp394_retained_supply_temperature_state_owned,
            index >= 3
        );
        assert_eq!(
            snapshot.cp394_retained_supply_enthalpy_state_owned,
            matches!(index, 5 | 8 | 11 | 14 | 17..=29),
        );
        assert_predecessor_carriers(snapshot, chain.cp394);
        assert_bits_eq(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            chain.cp394.resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits_eq(
            snapshot.resulting_supply_temperature_c,
            chain.cp394.resulting_supply_temperature_c,
        );
        assert_eq!(
            snapshot.resulting_supply_humidity_ratio.is_some(),
            matches!(index, 18 | 19 | 22 | 23 | 26 | 28),
        );
        if active {
            let temperature = chain
                .cp394
                .resulting_supply_temperature_c
                .expect("temperature");
            let enthalpy = chain
                .cp394
                .resulting_supply_enthalpy_j_per_kg
                .expect("enthalpy");
            let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);
            for value in [
                snapshot.psychrometric_supply_humidity_ratio,
                snapshot.assigned_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
            ] {
                assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
            }
        } else {
            assert_bits_eq(
                snapshot.resulting_supply_humidity_ratio,
                chain.cp394.resulting_supply_humidity_ratio,
            );
            assert_eq!(
                [
                    snapshot.supply_temperature_c,
                    snapshot.supply_enthalpy_j_per_kg,
                    snapshot.psychrometric_supply_humidity_ratio,
                    snapshot.assigned_supply_humidity_ratio,
                ],
                [None; 4],
            );
        }
    }
}

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state(state, predecessor)
}

fn exact(snapshot: Snapshot) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact(snapshot)
}

fn exact_direct(snapshot: Snapshot) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact_direct_release(snapshot)
}

fn active_counters(state: &State) -> [usize; 6] {
    [
        state.supply_temperature_owned_read_count,
        state.supply_temperature_for_humidity_ratio_inversion_read_count,
        state.supply_enthalpy_owned_read_count,
        state.supply_enthalpy_for_humidity_ratio_inversion_read_count,
        state.psychrometric_supply_humidity_ratio_evaluation_count,
        state.supply_humidity_ratio_assignment_write_count,
    ]
}

fn assert_predecessor_carriers(
    snapshot: Snapshot,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot,
) {
    for (left, right) in [
        (
            snapshot.predecessor_cp393_resulting_supply_humidity_ratio,
            predecessor.predecessor_cp393_resulting_supply_humidity_ratio,
        ),
        (
            snapshot.predecessor_cp393_resulting_supply_enthalpy_j_per_kg,
            predecessor.predecessor_cp393_resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.predecessor_cp393_resulting_supply_temperature_c,
            predecessor.predecessor_cp393_resulting_supply_temperature_c,
        ),
        (
            snapshot.predecessor_cp394_resulting_supply_humidity_ratio,
            predecessor.resulting_supply_humidity_ratio,
        ),
        (
            snapshot.predecessor_cp394_resulting_supply_enthalpy_j_per_kg,
            predecessor.resulting_supply_enthalpy_j_per_kg,
        ),
        (
            snapshot.predecessor_cp394_resulting_supply_temperature_c,
            predecessor.resulting_supply_temperature_c,
        ),
    ] {
        assert_bits_eq(left, right);
    }
}

fn assert_bits_eq(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}
