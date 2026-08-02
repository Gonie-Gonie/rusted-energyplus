//! CP394 exhaustive route and terminal-carrier tests.

use super::*;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntryRuntimeState;
type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot;

#[test]
fn cp394_preserves_thirty_routes_and_enters_exactly_three_humidistat_routes() {
    let chains = fixtures::all_chains();
    let mut state = State::new(chains[0].cp393.system);
    let snapshots: Vec<_> = chains
        .iter()
        .map(|chain| advance(&mut state, chain.cp393).expect("CP394 route"))
        .collect();

    assert_eq!(snapshots.len(), 30);
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 27);
    assert_eq!(
        state.dehumidification_control_humidistat_case_entry_count,
        3
    );
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 3);
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
        assert_eq!(
            snapshot.dehumidification_control_humidistat_case_entered,
            matches!(index, 19 | 23 | 26),
        );
        assert_eq!(
            snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
            matches!(index, 18 | 22 | 28),
        );
        assert_bits_eq(
            snapshot.predecessor_cp393_resulting_supply_humidity_ratio,
            chain.cp393.resulting_supply_humidity_ratio,
        );
        assert_bits_eq(
            snapshot.resulting_supply_humidity_ratio,
            chain.cp393.resulting_supply_humidity_ratio,
        );
        assert_bits_eq(
            snapshot.predecessor_cp393_resulting_supply_enthalpy_j_per_kg,
            chain.cp393.resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits_eq(
            snapshot.resulting_supply_enthalpy_j_per_kg,
            chain.cp393.resulting_supply_enthalpy_j_per_kg,
        );
        assert_bits_eq(
            snapshot.predecessor_cp393_resulting_supply_temperature_c,
            chain.cp393.resulting_supply_temperature_c,
        );
        assert_bits_eq(
            snapshot.resulting_supply_temperature_c,
            chain.cp393.resulting_supply_temperature_c,
        );
        assert_eq!(
            snapshot.resulting_supply_humidity_ratio.is_some(),
            matches!(index, 18 | 22 | 28)
        );
        assert_eq!(
            snapshot.resulting_supply_enthalpy_j_per_kg.is_some(),
            matches!(index, 5 | 8 | 11 | 14 | 17..=29)
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.is_some(),
            index >= 3
        );
        if matches!(index, 19 | 23 | 26) {
            assert!(snapshot.resulting_supply_humidity_ratio.is_none());
            assert!(snapshot.resulting_supply_enthalpy_j_per_kg.is_some());
            assert!(snapshot.resulting_supply_temperature_c.is_some());
        }
    }
}

fn advance(
    state: &mut State,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseBreakSnapshot,
) -> Option<Snapshot> {
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state(state, predecessor)
}

fn exact(snapshot: Snapshot) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_snapshot_is_exact(snapshot)
}

fn exact_direct(snapshot: Snapshot) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_snapshot_is_exact_direct_release(snapshot)
}

fn assert_bits_eq(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}
