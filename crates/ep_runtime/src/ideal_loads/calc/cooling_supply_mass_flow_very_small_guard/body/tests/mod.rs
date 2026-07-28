use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot,
    advance_cooling_supply_mass_flow_very_small_guard_body_state,
    cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release,
};
use crate::ideal_loads::{
    ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
};

mod release_corruption;

fn active_predecessor(
    supply_mass_flow_rate_kg_per_s: f64,
    body_entered: bool,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_body_entered: true,
        predecessor_cooling_body_entered: true,
        predecessor_ems_supply_mass_flow_override_body_entered: false,
        predecessor_ems_supply_mass_flow_override_body_skipped: true,
        predecessor_ems_disabled_fallthrough: true,
        predecessor_supply_mass_flow_limit_body_entered: false,
        predecessor_supply_mass_flow_limit_body_skipped: true,
        predecessor_supply_mass_flow_limit_active_guard_false_fallthrough: true,
        unit_off_skipped: false,
        non_cooling_skipped: false,
        cooling_body_entered: true,
        supply_mass_flow_rate_read: true,
        supply_mass_flow_rate_kg_per_s: Some(supply_mass_flow_rate_kg_per_s),
        hvac_very_small_mass_flow_read: true,
        hvac_very_small_mass_flow_source: Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE),
        hvac_very_small_mass_flow_kg_per_s: Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S),
        supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated: true,
        supply_mass_flow_rate_at_or_below_very_small_mass_flow: Some(body_entered),
        zero_flow_reset_body_entered: body_entered,
        active_guard_false_fallthrough: !body_entered,
    }
}

fn skipped_predecessor(
    unit_off: bool,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
    let mut predecessor = active_predecessor(0.0, true);
    predecessor.unit_body_entered = !unit_off;
    predecessor.predecessor_cooling_body_entered = false;
    predecessor.predecessor_ems_disabled_fallthrough = false;
    predecessor.predecessor_supply_mass_flow_limit_active_guard_false_fallthrough = false;
    predecessor.unit_off_skipped = unit_off;
    predecessor.non_cooling_skipped = !unit_off;
    predecessor.cooling_body_entered = false;
    predecessor.supply_mass_flow_rate_read = false;
    predecessor.supply_mass_flow_rate_kg_per_s = None;
    predecessor.hvac_very_small_mass_flow_read = false;
    predecessor.hvac_very_small_mass_flow_source = None;
    predecessor.hvac_very_small_mass_flow_kg_per_s = None;
    predecessor.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated = false;
    predecessor.supply_mass_flow_rate_at_or_below_very_small_mass_flow = None;
    predecessor.zero_flow_reset_body_entered = false;
    predecessor.active_guard_false_fallthrough = false;
    predecessor
}

fn run(
    state: &mut PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodySnapshot {
    advance_cooling_supply_mass_flow_very_small_guard_body_state(state, predecessor)
}

#[test]
fn source_boundary_and_exact_single_assignment_site_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2167"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2171"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_BODY_SOURCE_ORDER,
        ["assign-supply-mass-flow-rate-positive-zero"]
    );
}

#[test]
fn entered_body_assigns_exact_positive_zero_without_retesting_the_guard() {
    for supply in [
        f64::NEG_INFINITY,
        -0.0,
        0.0,
        ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S,
    ] {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        let snapshot = run(&mut state, active_predecessor(supply, true));
        assert!(snapshot.zero_flow_reset_body_entered);
        assert!(snapshot.supply_mass_flow_rate_positive_zero_assignment_performed);
        assert_eq!(
            snapshot
                .predecessor_supply_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            Some(supply.to_bits())
        );
        assert_eq!(
            snapshot
                .assigned_supply_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            Some(0)
        );
        assert_eq!(
            snapshot
                .resulting_supply_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            Some(0)
        );
        assert!(
            cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(
                snapshot
            )
        );
    }
}

#[test]
fn active_guard_false_preserves_predecessor_bits_and_skips_the_site() {
    let threshold = ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S;
    for supply in [
        f64::from_bits(threshold.to_bits() + 1),
        f64::INFINITY,
        f64::from_bits(0x7ff8_0000_0000_00a1),
        f64::from_bits(0xfff8_0000_0000_00b2),
    ] {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        let snapshot = run(&mut state, active_predecessor(supply, false));
        assert!(snapshot.body_skipped);
        assert!(snapshot.active_guard_false_fallthrough);
        assert!(!snapshot.supply_mass_flow_rate_positive_zero_assignment_performed);
        assert!(snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none());
        assert_eq!(
            snapshot
                .resulting_supply_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            Some(supply.to_bits())
        );
        assert!(
            cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(
                snapshot
            )
        );
    }
}

#[test]
fn unit_off_and_non_cooling_skip_the_site_and_retain_no_flow() {
    for predecessor in [skipped_predecessor(true), skipped_predecessor(false)] {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState::new(
            predecessor.system,
        );
        let snapshot = run(&mut state, predecessor);
        assert!(snapshot.body_skipped);
        assert!(!snapshot.zero_flow_reset_body_entered);
        assert!(!snapshot.active_guard_false_fallthrough);
        assert!(
            snapshot
                .predecessor_supply_mass_flow_rate_kg_per_s
                .is_none()
        );
        assert!(!snapshot.supply_mass_flow_rate_positive_zero_assignment_performed);
        assert!(snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none());
        assert!(snapshot.resulting_supply_mass_flow_rate_kg_per_s.is_none());
        assert!(
            cooling_supply_mass_flow_very_small_guard_body_snapshot_is_exact_direct_release(
                snapshot
            )
        );
    }
}

#[test]
fn counters_partition_assignment_fallthrough_and_skip_routes() {
    let mut state = PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    run(&mut state, active_predecessor(0.0, true));
    run(&mut state, active_predecessor(1.0, false));
    run(&mut state, skipped_predecessor(true));
    run(&mut state, skipped_predecessor(false));

    assert_eq!(state.transition_count, 4);
    assert_eq!(state.cooling_body_entry_count, 2);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.zero_flow_reset_body_entry_count, 1);
    assert_eq!(state.body_skip_count, 3);
    assert_eq!(state.active_guard_false_fallthrough_count, 1);
    assert_eq!(
        state.supply_mass_flow_rate_positive_zero_assignment_count,
        1
    );
}

#[test]
fn bit_exact_snapshot_comparison_rejects_one_sided_signed_zero_corruption() {
    let mut state = PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    let positive = run(&mut state, active_predecessor(0.0, true));
    let mut negative = positive;
    negative.resulting_supply_mass_flow_rate_kg_per_s = Some(-0.0);

    assert_eq!(positive, negative);
    assert!(!super::release::snapshots_match_bit_exact(
        positive, negative
    ));
}
