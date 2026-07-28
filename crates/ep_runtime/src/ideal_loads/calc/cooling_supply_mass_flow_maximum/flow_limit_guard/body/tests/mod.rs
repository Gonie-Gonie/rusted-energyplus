use ep_model::{IdealLoadsAirSystemId, IdealLoadsLimit, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyInput,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot,
    advance_cooling_supply_mass_flow_limit_body_state,
    cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
};

mod release_corruption;

fn active_predecessor(
    body_entered: bool,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
    let limit = if body_entered {
        IdealLoadsLimit::LimitFlowRate
    } else {
        IdealLoadsLimit::NoLimit
    };
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_GUARD_SOURCE_ORDER,
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
        first_cooling_limit_read: true,
        first_cooling_limit: Some(limit),
        cooling_limit_flow_rate_comparison_evaluated: true,
        cooling_limit_flow_rate: Some(body_entered),
        second_cooling_limit_read: !body_entered,
        second_cooling_limit: (!body_entered).then_some(limit),
        cooling_limit_flow_rate_and_capacity_comparison_evaluated: !body_entered,
        cooling_limit_flow_rate_and_capacity: (!body_entered).then_some(false),
        cooling_limit_condition_satisfied: Some(body_entered),
        maximum_cooling_air_mass_flow_rate_read: body_entered,
        maximum_cooling_air_mass_flow_rate_kg_per_s: body_entered.then_some(1.0),
        maximum_cooling_air_mass_flow_rate_positive_comparison_evaluated: body_entered,
        maximum_cooling_air_mass_flow_rate_strictly_positive: body_entered.then_some(true),
        supply_mass_flow_limit_body_entered: body_entered,
        active_guard_false_fallthrough: !body_entered,
    }
}

fn skipped_predecessor(unit_off: bool) -> PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot {
    let mut predecessor = active_predecessor(false);
    predecessor.unit_body_entered = !unit_off;
    predecessor.predecessor_cooling_body_entered = false;
    predecessor.predecessor_ems_disabled_fallthrough = false;
    predecessor.unit_off_skipped = unit_off;
    predecessor.non_cooling_skipped = !unit_off;
    predecessor.cooling_body_entered = false;
    predecessor.first_cooling_limit_read = false;
    predecessor.first_cooling_limit = None;
    predecessor.cooling_limit_flow_rate_comparison_evaluated = false;
    predecessor.cooling_limit_flow_rate = None;
    predecessor.second_cooling_limit_read = false;
    predecessor.second_cooling_limit = None;
    predecessor.cooling_limit_flow_rate_and_capacity_comparison_evaluated = false;
    predecessor.cooling_limit_flow_rate_and_capacity = None;
    predecessor.cooling_limit_condition_satisfied = None;
    predecessor.active_guard_false_fallthrough = false;
    predecessor
}

fn run(
    state: &mut PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    supply: Option<f64>,
    maximum: f64,
) -> PurchasedAirCalcCoolingSupplyMassFlowLimitBodySnapshot {
    advance_cooling_supply_mass_flow_limit_body_state(
        state,
        predecessor,
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyInput {
            supply_mass_flow_rate_before_limit_kg_per_s: supply,
            maximum_cooling_air_mass_flow_rate_kg_per_s: maximum,
        },
    )
}

#[test]
fn source_boundary_and_exact_four_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2163"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2166"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_LIMIT_BODY_SOURCE_ORDER,
        [
            "read-supply-mass-flow-rate-for-minimum",
            "reread-maximum-cooling-air-mass-flow-rate-for-minimum",
            "apply-source-shaped-two-argument-minimum",
            "assign-supply-mass-flow-rate",
        ]
    );
}

#[test]
fn source_min_preserves_ties_unordered_payloads_and_infinities() {
    let left_nan = f64::from_bits(0x7ff8_0000_0000_00a1);
    let right_nan = f64::from_bits(0x7ff8_0000_0000_00b2);
    let cases = [
        (1.0, 2.0, 1.0_f64.to_bits()),
        (2.0, 1.0, 1.0_f64.to_bits()),
        (0.0, -0.0, (-0.0_f64).to_bits()),
        (-0.0, 0.0, 0.0_f64.to_bits()),
        (left_nan, 3.0, 3.0_f64.to_bits()),
        (3.0, right_nan, right_nan.to_bits()),
        (left_nan, right_nan, right_nan.to_bits()),
        (
            f64::NEG_INFINITY,
            f64::INFINITY,
            f64::NEG_INFINITY.to_bits(),
        ),
        (
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::NEG_INFINITY.to_bits(),
        ),
    ];

    for (supply, maximum, expected_bits) in cases {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState::new(
            IdealLoadsAirSystemId(0),
        );
        let snapshot = run(&mut state, active_predecessor(true), Some(supply), maximum);
        assert!(snapshot.supply_mass_flow_rate_for_minimum_read);
        assert!(snapshot.maximum_cooling_air_mass_flow_rate_for_minimum_read);
        assert!(snapshot.source_shaped_two_argument_minimum_evaluated);
        assert!(snapshot.supply_mass_flow_rate_assignment_performed);
        for value in [
            snapshot.minimum_supply_mass_flow_rate_kg_per_s,
            snapshot.assigned_supply_mass_flow_rate_kg_per_s,
            snapshot.resulting_supply_mass_flow_rate_kg_per_s,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(expected_bits));
        }
    }
}

#[test]
fn unit_off_non_cooling_and_active_guard_false_skip_every_lexical_site() {
    let carried = f64::from_bits(0x8000_0000_0000_0000);
    for (predecessor, expected_result) in [
        (skipped_predecessor(true), None),
        (skipped_predecessor(false), None),
        (active_predecessor(false), Some(carried.to_bits())),
    ] {
        let mut state =
            PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState::new(predecessor.system);
        let snapshot = run(&mut state, predecessor, Some(carried), f64::NAN);
        assert!(snapshot.body_skipped);
        assert!(!snapshot.supply_mass_flow_rate_for_minimum_read);
        assert!(
            snapshot
                .supply_mass_flow_rate_before_limit_kg_per_s
                .is_none()
        );
        assert!(!snapshot.maximum_cooling_air_mass_flow_rate_for_minimum_read);
        assert!(
            snapshot
                .maximum_cooling_air_mass_flow_rate_kg_per_s
                .is_none()
        );
        assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
        assert!(snapshot.minimum_supply_mass_flow_rate_kg_per_s.is_none());
        assert!(!snapshot.supply_mass_flow_rate_assignment_performed);
        assert!(snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none());
        assert_eq!(
            snapshot
                .resulting_supply_mass_flow_rate_kg_per_s
                .map(f64::to_bits),
            expected_result
        );
        assert!(cooling_supply_mass_flow_limit_body_snapshot_is_exact_direct_release(snapshot));
    }
}

#[test]
fn counters_partition_applied_fallthrough_and_skip_routes() {
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState::new(IdealLoadsAirSystemId(0));
    run(&mut state, active_predecessor(true), Some(2.0), 1.0);
    run(&mut state, active_predecessor(false), Some(2.0), 1.0);
    run(&mut state, skipped_predecessor(true), None, 1.0);
    run(&mut state, skipped_predecessor(false), None, 1.0);

    assert_eq!(state.transition_count, 4);
    assert_eq!(state.cooling_body_entry_count, 2);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.supply_mass_flow_limit_body_entry_count, 1);
    assert_eq!(state.body_skip_count, 3);
    assert_eq!(state.active_guard_false_fallthrough_count, 1);
    assert_eq!(state.supply_mass_flow_rate_for_minimum_read_count, 1);
    assert_eq!(
        state.maximum_cooling_air_mass_flow_rate_for_minimum_read_count,
        1
    );
    assert_eq!(state.source_shaped_two_argument_minimum_evaluation_count, 1);
    assert_eq!(state.supply_mass_flow_rate_assignment_count, 1);
}

#[test]
fn bit_exact_snapshot_comparison_rejects_one_sided_signed_zero_corruption() {
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState::new(IdealLoadsAirSystemId(0));
    let positive = run(&mut state, active_predecessor(false), Some(0.0), 1.0);
    let mut negative = positive;
    negative.resulting_supply_mass_flow_rate_kg_per_s = Some(-0.0);

    assert_eq!(positive, negative);
    assert!(!super::release::snapshots_match_bit_exact(
        positive, negative
    ));
}
