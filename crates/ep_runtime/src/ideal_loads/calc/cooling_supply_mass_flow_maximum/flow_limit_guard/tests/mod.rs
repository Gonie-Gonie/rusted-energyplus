use ep_model::IdealLoadsLimit;

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardInput,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    advance_cooling_supply_mass_flow_limit_guard_state,
    cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release,
};
use crate::ideal_loads::calc::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyInput;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
};

mod release_corruption;

fn body(
    cooling_demand_w: f64,
    ems_override_enabled: bool,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
    let (_, _, reset) = super::super::tests::release_case(cooling_demand_w);
    let maximum = super::super::tests::run(reset, 0.0);
    body_from_maximum(maximum, ems_override_enabled)
}

fn body_from_maximum(
    maximum: crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot,
    ems_override_enabled: bool,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
    let mut guard_state =
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(maximum.system);
    let guard = super::super::advance_cooling_supply_mass_flow_ems_override_guard_state(
        &mut guard_state,
        maximum,
        ems_override_enabled,
    );
    let mut body_state =
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(maximum.system);
    let input =
        ems_override_enabled.then_some(PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyInput {
            ems_supply_mass_flow_override_value_kg_per_s: 1.0,
            outdoor_air_mass_flow_rate_before_override_kg_per_s: 2.0,
        });
    super::super::advance_cooling_supply_mass_flow_ems_override_body_state(
        &mut body_state,
        guard,
        input,
    )
}

fn run(
    state: &mut PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    cooling_limit: IdealLoadsLimit,
    maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
    advance_cooling_supply_mass_flow_limit_guard_state(
        state,
        predecessor,
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardInput {
            cooling_limit,
            maximum_cooling_air_mass_flow_rate_kg_per_s,
        },
    )
}

#[test]
fn source_boundary_and_exact_seven_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2161-2162"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2163"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
        [
            "read-cooling-limit-for-flow-rate-comparison",
            "compare-cooling-limit-equal-to-flow-rate",
            "read-cooling-limit-for-flow-rate-and-capacity-comparison-after-first-false",
            "compare-cooling-limit-equal-to-flow-rate-and-capacity",
            "read-maximum-cooling-air-mass-flow-rate-after-limit-condition-true",
            "compare-maximum-cooling-air-mass-flow-rate-strictly-above-zero",
            "enter-supply-mass-flow-limit-body-if-compound-condition-satisfied",
        ]
    );
}

#[test]
fn selector_short_circuit_matches_all_four_limits_and_allows_true_body_entry() {
    let predecessor = body(-1_000.0, false);
    let cases = [
        (IdealLoadsLimit::NoLimit, false, true, false, false),
        (IdealLoadsLimit::LimitFlowRate, true, false, false, true),
        (IdealLoadsLimit::LimitCapacity, false, true, false, false),
        (
            IdealLoadsLimit::LimitFlowRateAndCapacity,
            false,
            true,
            true,
            true,
        ),
    ];

    for (limit, first_match, second_read, second_match, body_entered) in cases {
        let mut state =
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState::new(predecessor.system);
        let snapshot = run(&mut state, predecessor, limit, 0.25);
        assert_eq!(snapshot.cooling_limit_flow_rate, Some(first_match));
        assert_eq!(snapshot.second_cooling_limit_read, second_read);
        assert_eq!(
            snapshot.cooling_limit_flow_rate_and_capacity,
            second_read.then_some(second_match)
        );
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_read,
            body_entered
        );
        assert_eq!(snapshot.supply_mass_flow_limit_body_entered, body_entered);
        assert_eq!(snapshot.active_guard_false_fallthrough, !body_entered);
        assert!(cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(snapshot));
    }
}

#[test]
fn strict_positive_comparison_preserves_ieee_boundary_behavior() {
    let predecessor = body(-1_000.0, false);
    let cases = [
        (0.0, false),
        (-0.0, false),
        (f64::NAN, false),
        (-1.0, false),
        (f64::from_bits(1), true),
        (f64::INFINITY, true),
    ];

    for (maximum, expected) in cases {
        let mut state =
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState::new(predecessor.system);
        let snapshot = run(
            &mut state,
            predecessor,
            IdealLoadsLimit::LimitFlowRate,
            maximum,
        );
        assert_eq!(
            snapshot.maximum_cooling_air_mass_flow_rate_strictly_positive,
            Some(expected)
        );
        assert_eq!(snapshot.supply_mass_flow_limit_body_entered, expected);
        assert!(cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(snapshot));
    }
}

#[test]
fn unit_off_and_non_cooling_skip_all_guard_reads_even_with_poisoned_input() {
    let non_cooling = body(1.0, false);
    let (_, _, reset) = super::super::tests::release_case(1.0);
    let mut maximum = super::super::tests::run(reset, 0.0);
    maximum.unit_body_entered = false;
    maximum.unit_off_skipped = true;
    maximum.non_cooling_skipped = false;
    let unit_off = body_from_maximum(maximum, false);

    for predecessor in [unit_off, non_cooling] {
        let mut state =
            PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState::new(predecessor.system);
        let snapshot = run(
            &mut state,
            predecessor,
            IdealLoadsLimit::LimitFlowRate,
            f64::NAN,
        );
        assert!(!snapshot.first_cooling_limit_read);
        assert!(!snapshot.second_cooling_limit_read);
        assert!(!snapshot.maximum_cooling_air_mass_flow_rate_read);
        assert!(!snapshot.supply_mass_flow_limit_body_entered);
        assert!(!snapshot.active_guard_false_fallthrough);
        assert!(cooling_supply_mass_flow_limit_guard_snapshot_is_exact_direct_release(snapshot));
    }
}

#[test]
fn counters_partition_rejected_nonpositive_and_entered_routes() {
    let predecessor = body(-1_000.0, false);
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowLimitGuardRuntimeState::new(predecessor.system);
    for (limit, maximum) in [
        (IdealLoadsLimit::NoLimit, f64::NAN),
        (IdealLoadsLimit::LimitCapacity, f64::NAN),
        (IdealLoadsLimit::LimitFlowRate, 0.0),
        (IdealLoadsLimit::LimitFlowRate, 0.25),
        (IdealLoadsLimit::LimitFlowRateAndCapacity, 0.25),
    ] {
        run(&mut state, predecessor, limit, maximum);
    }

    assert_eq!(state.transition_count, 5);
    assert_eq!(state.cooling_body_entry_count, 5);
    assert_eq!(state.first_cooling_limit_read_count, 5);
    assert_eq!(state.cooling_limit_flow_rate_match_count, 2);
    assert_eq!(state.second_cooling_limit_read_count, 3);
    assert_eq!(state.cooling_limit_flow_rate_and_capacity_match_count, 1);
    assert_eq!(state.cooling_limit_rejected_count, 2);
    assert_eq!(state.maximum_cooling_air_mass_flow_rate_read_count, 3);
    assert_eq!(
        state.maximum_cooling_air_mass_flow_rate_not_positive_count,
        1
    );
    assert_eq!(
        state.maximum_cooling_air_mass_flow_rate_strictly_positive_count,
        2
    );
    assert_eq!(state.supply_mass_flow_limit_body_entry_count, 2);
    assert_eq!(state.active_guard_false_fallthrough_count, 3);
}
