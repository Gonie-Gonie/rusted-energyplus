mod committed;
mod release_corruption;

use super::*;
use crate::ideal_loads::calc::cooling_mixed_air_call::advance_cooling_mixed_air_call_state;
use crate::ideal_loads::calc::cooling_mixed_air_call::tests::{
    Route as MixedAirRoute, active_input as mixed_air_active_input,
    predecessor as mixed_air_predecessor,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallRuntimeState, PurchasedAirCalcCoolingMixedAirCallSnapshot,
};

fn active_predecessor(
    supply_mass_flow_rate_kg_per_s: f64,
    ordinal: usize,
) -> PurchasedAirCalcCoolingMixedAirCallSnapshot {
    let mut predecessor = mixed_air_predecessor(MixedAirRoute::CoolingFallthrough);
    predecessor.parent_call_ordinal = ordinal;
    let mut state = PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(predecessor.system);
    advance_cooling_mixed_air_call_state(
        &mut state,
        predecessor,
        Some(mixed_air_active_input(supply_mass_flow_rate_kg_per_s)),
    )
}

fn skipped_predecessor(
    route: MixedAirRoute,
    ordinal: usize,
) -> PurchasedAirCalcCoolingMixedAirCallSnapshot {
    let mut predecessor = mixed_air_predecessor(route);
    predecessor.parent_call_ordinal = ordinal;
    let mut state = PurchasedAirCalcCoolingMixedAirCallRuntimeState::new(predecessor.system);
    advance_cooling_mixed_air_call_state(&mut state, predecessor, None)
}

#[test]
fn strict_positive_guard_preserves_source_double_comparison_semantics() {
    for (value, expected_positive) in [
        (f64::NEG_INFINITY, false),
        (-1.0, false),
        (-0.0, false),
        (0.0, false),
        (f64::from_bits(1), true),
        (1.0, true),
        (f64::INFINITY, true),
        (f64::from_bits(0x7ff8_0000_0000_00a1), false),
        (f64::from_bits(0xfff8_0000_0000_00b2), false),
    ] {
        let predecessor = active_predecessor(value, 1);
        let mut state =
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(predecessor.system);
        let snapshot =
            advance_cooling_supply_mass_flow_positive_guard_state(&mut state, predecessor);

        assert!(cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(snapshot));
        assert!(snapshot.supply_mass_flow_rate_read);
        assert_eq!(
            snapshot.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
            Some(value.to_bits())
        );
        assert!(snapshot.supply_mass_flow_rate_strictly_positive_comparison_evaluated);
        assert_eq!(
            snapshot.supply_mass_flow_rate_strictly_positive,
            Some(expected_positive)
        );
        assert_eq!(
            snapshot.positive_supply_mass_flow_body_entered,
            expected_positive
        );
        assert_eq!(snapshot.active_guard_false_fallthrough, !expected_positive);
        assert_eq!(
            state.source_site_execution_count,
            2 + usize::from(expected_positive)
        );
    }
}

#[test]
fn skipped_routes_execute_no_cp330_source_sites() {
    for (route, unit_off, non_cooling) in [
        (MixedAirRoute::UnitOff, true, false),
        (MixedAirRoute::NonCooling, false, true),
    ] {
        let predecessor = skipped_predecessor(route, 1);
        let mut state =
            PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(predecessor.system);
        let snapshot =
            advance_cooling_supply_mass_flow_positive_guard_state(&mut state, predecessor);

        assert!(cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(snapshot));
        assert_eq!(snapshot.unit_off_skipped, unit_off);
        assert_eq!(snapshot.non_cooling_skipped, non_cooling);
        assert!(!snapshot.cooling_body_entered);
        assert!(!snapshot.supply_mass_flow_rate_read);
        assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
        assert_eq!(state.transition_count, 1);
        assert_eq!(state.cooling_body_entry_count, 0);
        assert_eq!(state.unit_off_skip_count, usize::from(unit_off));
        assert_eq!(state.non_cooling_skip_count, usize::from(non_cooling));
        assert_eq!(state.source_site_execution_count, 0);
    }
}

#[test]
fn counters_partition_routes_and_count_only_executed_source_sites() {
    let mut state = PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(
        ep_model::IdealLoadsAirSystemId(3),
    );
    for predecessor in [
        skipped_predecessor(MixedAirRoute::UnitOff, 1),
        skipped_predecessor(MixedAirRoute::NonCooling, 2),
        active_predecessor(0.25, 3),
        active_predecessor(0.0, 4),
    ] {
        advance_cooling_supply_mass_flow_positive_guard_state(&mut state, predecessor);
    }

    assert_eq!(state.transition_count, 4);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.cooling_body_entry_count, 2);
    assert_eq!(state.positive_supply_mass_flow_body_entry_count, 1);
    assert_eq!(state.active_guard_false_fallthrough_count, 1);
    assert_eq!(state.source_site_execution_count, 5);
    assert_eq!(state.supply_mass_flow_rate_read_count, 2);
    assert_eq!(
        state.supply_mass_flow_rate_strictly_positive_comparison_count,
        2
    );
}

#[test]
fn exact_predicate_rejects_forged_provenance_and_comparison_results() {
    let predecessor = active_predecessor(0.25, 1);
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(predecessor.system);
    let snapshot = advance_cooling_supply_mass_flow_positive_guard_state(&mut state, predecessor);

    let mut forged_source = snapshot;
    forged_source.source = "forged";
    assert!(
        !cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(forged_source)
    );

    let mut forged_comparison = snapshot;
    forged_comparison.supply_mass_flow_rate_strictly_positive = Some(false);
    assert!(
        !cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(
            forged_comparison
        )
    );

    let predecessor = active_predecessor(0.0, 1);
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardRuntimeState::new(predecessor.system);
    let positive_zero =
        advance_cooling_supply_mass_flow_positive_guard_state(&mut state, predecessor);
    let mut negative_zero = positive_zero;
    negative_zero.supply_mass_flow_rate_kg_per_s = Some(-0.0);

    assert_eq!(positive_zero, negative_zero);
    assert!(
        cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(negative_zero)
    );
    assert!(!super::release::snapshots_match_bit_exact(
        positive_zero,
        negative_zero
    ));
}
