//! CP398 exhaustive route, case-entry, and carrier tests.

use super::*;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntryRuntimeState;
type Snapshot = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseEntrySnapshot;

#[test]
fn cp398_preserves_thirty_routes_and_enters_shared_case_exactly_six_times() {
    let chains = fixtures::all_chains();
    let mut state = State::new(chains[0].cp397.system);
    let snapshots: Vec<_> = chains
        .iter()
        .map(|chain| advance(&mut state, chain.cp397).expect("CP398 route"))
        .collect();

    assert_eq!(snapshots.len(), 30);
    assert_eq!(state.transition_count, 30);
    assert_eq!(state.inactive_transition_count, 24);
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_case_entry_count,
        6
    );
    assert_eq!(state.predecessor_route_counts, [1; 30]);
    assert_eq!(state.source_site_execution_count, 6);
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
        let constant_shr = matches!(index, 18 | 22 | 28);
        let humidistat = matches!(index, 19 | 23 | 26);
        let none = matches!(index, 20 | 24 | 27);
        let shared = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
        assert_eq!(
            snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
            constant_shr,
        );
        assert_eq!(
            snapshot
                .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
            constant_shr,
        );
        assert_eq!(
            snapshot.predecessor_dehumidification_control_humidistat_case_entered,
            humidistat,
        );
        assert_eq!(
            snapshot
                .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
            humidistat,
        );
        assert_eq!(
            snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break,
            humidistat,
        );
        assert_eq!(snapshot.predecessor_dehumidification_control_none_case_entered, none);
        assert_eq!(
            snapshot
                .dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
            shared,
        );
        assert_eq!(
            snapshot.resulting_supply_humidity_ratio.is_some(),
            matches!(index, 18 | 19 | 22 | 23 | 26 | 28),
        );
        assert_eq!(
            snapshot.resulting_supply_enthalpy_j_per_kg.is_some(),
            matches!(index, 5 | 8 | 11 | 14 | 17..=29),
        );
        assert_eq!(
            snapshot.resulting_supply_temperature_c.is_some(),
            index >= 3
        );
        for (predecessor, resulting, expected) in [
            (
                snapshot.predecessor_cp397_resulting_supply_humidity_ratio,
                snapshot.resulting_supply_humidity_ratio,
                chain.cp397.resulting_supply_humidity_ratio,
            ),
            (
                snapshot.predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
                snapshot.resulting_supply_enthalpy_j_per_kg,
                chain.cp397.resulting_supply_enthalpy_j_per_kg,
            ),
            (
                snapshot.predecessor_cp397_resulting_supply_temperature_c,
                snapshot.resulting_supply_temperature_c,
                chain.cp397.resulting_supply_temperature_c,
            ),
        ] {
            assert_bits_eq(predecessor, expected);
            assert_bits_eq(resulting, expected);
        }
    }
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

fn exact(snapshot: Snapshot) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_snapshot_is_exact(
        snapshot,
    )
}

fn exact_direct(snapshot: Snapshot) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_entry_snapshot_is_exact_direct_release(
        snapshot,
    )
}

fn assert_bits_eq(left: Option<f64>, right: Option<f64>) {
    assert_eq!(left.map(f64::to_bits), right.map(f64::to_bits));
}
