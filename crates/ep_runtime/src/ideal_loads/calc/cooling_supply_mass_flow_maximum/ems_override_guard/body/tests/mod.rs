use super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyInput,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
    advance_cooling_supply_mass_flow_ems_override_body_state,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
};

mod release_corruption;

fn guard(
    cooling_demand_w: f64,
    enabled: bool,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot {
    let (_, _, reset) = super::super::super::tests::release_case(cooling_demand_w);
    let maximum = super::super::super::tests::run(reset, 0.0);
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(maximum.system);
    super::super::advance_cooling_supply_mass_flow_ems_override_guard_state(
        &mut state, maximum, enabled,
    )
}

#[test]
fn source_boundary_and_exact_six_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2158-2159"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2161"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
        [
            "read-ems-supply-mass-flow-override-value",
            "assign-supply-mass-flow-rate-from-ems-override",
            "read-outdoor-air-mass-flow-rate-for-minimum",
            "read-supply-mass-flow-rate-for-minimum",
            "apply-source-shaped-two-argument-minimum",
            "assign-outdoor-air-mass-flow-rate",
        ]
    );
}

#[test]
fn active_false_guard_skips_every_body_site() {
    let predecessor = guard(-1_000.0, false);
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(predecessor.system);
    let snapshot =
        advance_cooling_supply_mass_flow_ems_override_body_state(&mut state, predecessor, None);
    assert!(snapshot.body_skipped);
    assert!(snapshot.ems_disabled_fallthrough);
    assert!(!snapshot.ems_supply_mass_flow_override_value_read);
    assert!(!snapshot.supply_mass_flow_rate_override_assignment_performed);
    assert!(!snapshot.outdoor_air_mass_flow_rate_for_minimum_read);
    assert!(!snapshot.supply_mass_flow_rate_for_minimum_read);
    assert!(!snapshot.source_shaped_two_argument_minimum_evaluated);
    assert!(!snapshot.outdoor_air_mass_flow_rate_assignment_performed);
    assert_eq!(state.body_skip_count, 1);
    assert_eq!(state.ems_disabled_fallthrough_count, 1);
}

#[test]
fn active_true_guard_executes_all_six_sites() {
    let predecessor = guard(-1_000.0, true);
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(predecessor.system);
    let snapshot = advance_cooling_supply_mass_flow_ems_override_body_state(
        &mut state,
        predecessor,
        Some(PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyInput {
            ems_supply_mass_flow_override_value_kg_per_s: 1.0,
            outdoor_air_mass_flow_rate_before_override_kg_per_s: 2.0,
        }),
    );
    assert!(!snapshot.body_skipped);
    assert!(!snapshot.ems_disabled_fallthrough);
    assert!(snapshot.ems_supply_mass_flow_override_value_read);
    assert!(snapshot.supply_mass_flow_rate_override_assignment_performed);
    assert!(snapshot.outdoor_air_mass_flow_rate_for_minimum_read);
    assert!(snapshot.supply_mass_flow_rate_for_minimum_read);
    assert!(snapshot.source_shaped_two_argument_minimum_evaluated);
    assert!(snapshot.outdoor_air_mass_flow_rate_assignment_performed);
    assert_eq!(snapshot.assigned_supply_mass_flow_rate_kg_per_s, Some(1.0));
    assert_eq!(
        snapshot.assigned_outdoor_air_mass_flow_rate_kg_per_s,
        Some(1.0)
    );
    assert_eq!(state.body_entry_count, 1);
    assert_eq!(state.ems_supply_mass_flow_override_value_read_count, 1);
    assert_eq!(state.supply_mass_flow_rate_override_assignment_count, 1);
    assert_eq!(state.outdoor_air_mass_flow_rate_for_minimum_read_count, 1);
    assert_eq!(state.supply_mass_flow_rate_for_minimum_read_count, 1);
    assert_eq!(state.source_shaped_two_argument_minimum_evaluation_count, 1);
    assert_eq!(state.outdoor_air_mass_flow_rate_assignment_count, 1);
}

#[test]
fn source_minimum_preserves_right_choice_for_ties_and_unordered_values() {
    let predecessor = guard(-1_000.0, true);
    let cases = [
        (-0.0, 0.0, 0.0),
        (0.0, -0.0, -0.0),
        (f64::NAN, 3.0, 3.0),
        (3.0, f64::NAN, f64::NAN),
    ];
    for (outdoor_air, supply, expected) in cases {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(
            predecessor.system,
        );
        let snapshot = advance_cooling_supply_mass_flow_ems_override_body_state(
            &mut state,
            predecessor,
            Some(PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyInput {
                ems_supply_mass_flow_override_value_kg_per_s: supply,
                outdoor_air_mass_flow_rate_before_override_kg_per_s: outdoor_air,
            }),
        );
        assert_eq!(
            snapshot
                .assigned_outdoor_air_mass_flow_rate_kg_per_s
                .expect("entered body")
                .to_bits(),
            expected.to_bits()
        );
    }
}

#[test]
fn missing_internal_input_is_rejected_by_snapshot_validation_without_panicking() {
    let predecessor = guard(-1_000.0, true);
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(predecessor.system);
    let snapshot =
        advance_cooling_supply_mass_flow_ems_override_body_state(&mut state, predecessor, None);
    assert!(
        !super::cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release(
            snapshot
        )
    );
}

#[test]
fn unit_off_and_non_cooling_skip_every_body_site() {
    let non_cooling = guard(1.0, true);
    let (_, _, reset) = super::super::super::tests::release_case(1.0);
    let mut maximum = super::super::super::tests::run(reset, 0.0);
    maximum.unit_body_entered = false;
    maximum.unit_off_skipped = true;
    maximum.non_cooling_skipped = false;
    let mut guard_state =
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(maximum.system);
    let unit_off = super::super::advance_cooling_supply_mass_flow_ems_override_guard_state(
        &mut guard_state,
        maximum,
        true,
    );

    for predecessor in [unit_off, non_cooling] {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(
            predecessor.system,
        );
        let snapshot = advance_cooling_supply_mass_flow_ems_override_body_state(
            &mut state,
            predecessor,
            Some(PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyInput {
                ems_supply_mass_flow_override_value_kg_per_s: f64::NAN,
                outdoor_air_mass_flow_rate_before_override_kg_per_s: f64::NAN,
            }),
        );
        assert!(snapshot.body_skipped);
        assert!(!snapshot.ems_disabled_fallthrough);
        assert!(!snapshot.ems_supply_mass_flow_override_value_read);
    }
}

#[test]
fn state_counters_partition_all_characterized_routes() {
    let cooling_false = guard(-1_000.0, false);
    let cooling_true = guard(-1_000.0, true);
    let non_cooling = guard(1.0, false);
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState::new(cooling_false.system);
    advance_cooling_supply_mass_flow_ems_override_body_state(&mut state, non_cooling, None);
    advance_cooling_supply_mass_flow_ems_override_body_state(&mut state, cooling_false, None);
    advance_cooling_supply_mass_flow_ems_override_body_state(
        &mut state,
        cooling_true,
        Some(PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyInput {
            ems_supply_mass_flow_override_value_kg_per_s: 1.0,
            outdoor_air_mass_flow_rate_before_override_kg_per_s: 2.0,
        }),
    );
    assert_eq!(state.transition_count, 3);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.cooling_body_entry_count, 2);
    assert_eq!(state.body_entry_count, 1);
    assert_eq!(state.body_skip_count, 2);
    assert_eq!(state.ems_disabled_fallthrough_count, 1);
}
