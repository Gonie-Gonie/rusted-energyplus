use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
    advance_cooling_supply_mass_flow_ems_override_guard_state,
};
use crate::ideal_loads::PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot;

mod release_corruption;

fn maximum(cooling_demand_w: f64) -> PurchasedAirCalcCoolingSupplyMassFlowMaximumSnapshot {
    let (_, _, reset) = super::super::tests::release_case(cooling_demand_w);
    super::super::tests::run(reset, 0.0)
}

#[test]
fn source_boundary_and_exact_three_sites_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2157"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2158"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
        [
            "read-ems-supply-mass-flow-override-flag",
            "evaluate-ems-supply-mass-flow-override-guard",
            "enter-ems-supply-mass-flow-override-body-if-enabled",
        ]
    );
}

#[test]
fn active_false_and_true_guards_stop_before_line_2158() {
    let predecessor = maximum(-1_000.0);
    for (enabled, body_entered, false_fallthrough) in [(false, false, true), (true, true, false)] {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(
            predecessor.system,
        );
        let snapshot = advance_cooling_supply_mass_flow_ems_override_guard_state(
            &mut state,
            predecessor,
            enabled,
        );
        assert!(snapshot.ems_supply_mass_flow_override_flag_read);
        assert!(snapshot.ems_supply_mass_flow_override_guard_evaluated);
        assert_eq!(
            snapshot.ems_supply_mass_flow_override_enabled,
            Some(enabled)
        );
        assert_eq!(
            snapshot.ems_supply_mass_flow_override_body_entered,
            body_entered
        );
        assert_eq!(
            snapshot.ems_supply_mass_flow_override_guard_false_fallthrough,
            false_fallthrough
        );
        assert_eq!(state.ems_supply_mass_flow_override_flag_read_count, 1);
        assert_eq!(
            state.ems_supply_mass_flow_override_guard_evaluation_count,
            1
        );
        assert_eq!(
            state.ems_supply_mass_flow_override_body_entry_count,
            usize::from(enabled)
        );
        assert_eq!(
            state.ems_supply_mass_flow_override_guard_false_fallthrough_count,
            usize::from(!enabled)
        );
    }
}

#[test]
fn unit_off_and_non_cooling_skip_every_guard_site() {
    let non_cooling = maximum(1.0);
    let mut unit_off = non_cooling;
    unit_off.unit_body_entered = false;
    unit_off.unit_off_skipped = true;
    unit_off.non_cooling_skipped = false;
    for predecessor in [unit_off, non_cooling] {
        let mut state = PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(
            predecessor.system,
        );
        let snapshot = advance_cooling_supply_mass_flow_ems_override_guard_state(
            &mut state,
            predecessor,
            true,
        );
        assert!(!snapshot.ems_supply_mass_flow_override_flag_read);
        assert!(!snapshot.ems_supply_mass_flow_override_guard_evaluated);
        assert_eq!(snapshot.ems_supply_mass_flow_override_enabled, None);
        assert!(!snapshot.ems_supply_mass_flow_override_body_entered);
        assert!(!snapshot.ems_supply_mass_flow_override_guard_false_fallthrough);
    }
}

#[test]
fn state_counters_partition_all_four_characterized_routes() {
    let cooling = maximum(-1_000.0);
    let non_cooling = maximum(1.0);
    let mut unit_off = non_cooling;
    unit_off.unit_body_entered = false;
    unit_off.unit_off_skipped = true;
    unit_off.non_cooling_skipped = false;
    let mut state =
        PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState::new(cooling.system);
    for (predecessor, enabled) in [
        (unit_off, false),
        (non_cooling, false),
        (cooling, false),
        (cooling, true),
    ] {
        advance_cooling_supply_mass_flow_ems_override_guard_state(&mut state, predecessor, enabled);
    }
    assert_eq!(state.transition_count, 4);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.cooling_body_entry_count, 2);
    assert_eq!(state.ems_supply_mass_flow_override_flag_read_count, 2);
    assert_eq!(
        state.ems_supply_mass_flow_override_guard_evaluation_count,
        2
    );
    assert_eq!(state.ems_supply_mass_flow_override_body_entry_count, 1);
    assert_eq!(
        state.ems_supply_mass_flow_override_guard_false_fallthrough_count,
        1
    );
}
