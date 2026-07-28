use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::{
    ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S, ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardInput,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot,
    advance_cooling_supply_mass_flow_very_small_guard_state,
    cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
};

mod release_corruption;

fn active_predecessor(
    supply_mass_flow_rate_kg_per_s: f64,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_body_entered: true,
        predecessor_cooling_body_entered: true,
        predecessor_ems_supply_mass_flow_override_body_entered: false,
        predecessor_ems_supply_mass_flow_override_body_skipped: true,
        predecessor_ems_disabled_fallthrough: true,
        unit_off_skipped: false,
        non_cooling_skipped: false,
        cooling_body_entered: true,
        supply_mass_flow_limit_body_entered: false,
        body_skipped: true,
        active_guard_false_fallthrough: true,
        supply_mass_flow_rate_for_minimum_read: false,
        supply_mass_flow_rate_before_limit_kg_per_s: None,
        maximum_cooling_air_mass_flow_rate_for_minimum_read: false,
        maximum_cooling_air_mass_flow_rate_kg_per_s: None,
        source_shaped_two_argument_minimum_evaluated: false,
        minimum_supply_mass_flow_rate_kg_per_s: None,
        supply_mass_flow_rate_assignment_performed: false,
        assigned_supply_mass_flow_rate_kg_per_s: None,
        resulting_supply_mass_flow_rate_kg_per_s: Some(supply_mass_flow_rate_kg_per_s),
    }
}

fn skipped_predecessor(unit_off: bool) -> PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
    let mut predecessor = active_predecessor(0.0);
    predecessor.unit_body_entered = !unit_off;
    predecessor.predecessor_cooling_body_entered = false;
    predecessor.predecessor_ems_disabled_fallthrough = false;
    predecessor.unit_off_skipped = unit_off;
    predecessor.non_cooling_skipped = !unit_off;
    predecessor.cooling_body_entered = false;
    predecessor.active_guard_false_fallthrough = false;
    predecessor.resulting_supply_mass_flow_rate_kg_per_s = None;
    predecessor
}

fn run(
    state: &mut PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
) -> PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardSnapshot {
    advance_cooling_supply_mass_flow_very_small_guard_state(
        state,
        predecessor,
        PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardInput {
            supply_mass_flow_rate_kg_per_s: predecessor.resulting_supply_mass_flow_rate_kg_per_s,
        },
    )
}

#[test]
fn source_boundary_constant_provenance_and_exact_four_textual_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2166"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2167"
    );
    assert_eq!(
        ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE,
        "EnergyPlus 26.1 DataHVACGlobals.hh:89"
    );
    assert_eq!(
        ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S.to_bits(),
        0x39b4_484b_feeb_c2a0
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_VERY_SMALL_GUARD_SOURCE_ORDER,
        [
            "read-retained-supply-mass-flow-rate",
            "read-hvac-very-small-mass-flow",
            "compare-supply-mass-flow-rate-less-than-or-equal-to-hvac-very-small-mass-flow",
            "enter-zero-flow-reset-body-if-at-or-below-threshold",
        ]
    );
    // The inventory above is textual; it deliberately does not assert a C++
    // left-before-right operand evaluation order.
}

#[test]
fn source_less_than_or_equal_preserves_ieee_nan_signed_zero_and_infinities() {
    let threshold = ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_KG_PER_S;
    let immediately_below = f64::from_bits(threshold.to_bits() - 1);
    let immediately_above = f64::from_bits(threshold.to_bits() + 1);
    let positive_nan = f64::from_bits(0x7ff8_0000_0000_00a1);
    let negative_nan = f64::from_bits(0xfff8_0000_0000_00b2);
    let cases = [
        (f64::NEG_INFINITY, true),
        (-0.0, true),
        (0.0, true),
        (immediately_below, true),
        (threshold, true),
        (immediately_above, false),
        (f64::INFINITY, false),
        (positive_nan, false),
        (negative_nan, false),
    ];

    for (supply, expected) in cases {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        let snapshot = run(&mut state, active_predecessor(supply));
        assert!(snapshot.supply_mass_flow_rate_read);
        assert_eq!(
            snapshot.supply_mass_flow_rate_kg_per_s.map(f64::to_bits),
            Some(supply.to_bits())
        );
        assert!(snapshot.hvac_very_small_mass_flow_read);
        assert_eq!(
            snapshot.hvac_very_small_mass_flow_source,
            Some(ENERGYPLUS_HVAC_VERY_SMALL_MASS_FLOW_SOURCE)
        );
        assert_eq!(
            snapshot
                .hvac_very_small_mass_flow_kg_per_s
                .map(f64::to_bits),
            Some(threshold.to_bits())
        );
        assert!(
            snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated
        );
        assert_eq!(
            snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow,
            Some(expected)
        );
        assert_eq!(snapshot.zero_flow_reset_body_entered, expected);
        assert_eq!(snapshot.active_guard_false_fallthrough, !expected);
        assert!(
            cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(snapshot)
        );
    }
}

#[test]
fn unit_off_and_non_cooling_skip_both_operands_comparison_and_body_entry() {
    for predecessor in [skipped_predecessor(true), skipped_predecessor(false)] {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState::new(
            predecessor.system,
        );
        let snapshot = run(&mut state, predecessor);
        assert!(!snapshot.supply_mass_flow_rate_read);
        assert!(snapshot.supply_mass_flow_rate_kg_per_s.is_none());
        assert!(!snapshot.hvac_very_small_mass_flow_read);
        assert!(snapshot.hvac_very_small_mass_flow_source.is_none());
        assert!(snapshot.hvac_very_small_mass_flow_kg_per_s.is_none());
        assert!(
            !snapshot.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_evaluated
        );
        assert!(
            snapshot
                .supply_mass_flow_rate_at_or_below_very_small_mass_flow
                .is_none()
        );
        assert!(!snapshot.zero_flow_reset_body_entered);
        assert!(!snapshot.active_guard_false_fallthrough);
        assert!(
            cooling_supply_mass_flow_very_small_guard_snapshot_is_exact_direct_release(snapshot)
        );
    }
}

#[test]
fn counters_partition_body_entry_fallthrough_and_skip_routes() {
    let mut state = PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    run(&mut state, active_predecessor(0.0));
    run(&mut state, active_predecessor(1.0));
    run(&mut state, skipped_predecessor(true));
    run(&mut state, skipped_predecessor(false));

    assert_eq!(state.transition_count, 4);
    assert_eq!(state.cooling_body_entry_count, 2);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.supply_mass_flow_rate_read_count, 2);
    assert_eq!(state.hvac_very_small_mass_flow_read_count, 2);
    assert_eq!(
        state.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count,
        2
    );
    assert_eq!(state.zero_flow_reset_body_entry_count, 1);
    assert_eq!(state.active_guard_false_fallthrough_count, 1);
}

#[test]
fn bit_exact_snapshot_comparison_rejects_one_sided_signed_zero_corruption() {
    let mut state = PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState::new(
        IdealLoadsAirSystemId(0),
    );
    let positive = run(&mut state, active_predecessor(0.0));
    let mut negative = positive;
    negative.supply_mass_flow_rate_kg_per_s = Some(-0.0);

    assert_eq!(positive, negative);
    assert!(!super::release::snapshots_match_bit_exact(
        positive, negative
    ));
}
