use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_state,
    completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent,
    release::next_transition_fits_for_test,
};
use super::public_release::completed_cp345_case;
use crate::ideal_loads::advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;
use ep_model::DehumidificationControlType;

fn assert_rejected_transactionally(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitHumidityRatioMixedAirAssignmentSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(&*runtime, &before);
}

fn assert_completed_state_rejected_without_mutation(
    runtime: &crate::ideal_loads::PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    snapshot: crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
) {
    let before = runtime.clone();
    let unit = runtime.units.get(&system.id).expect("known unit");
    let witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            system.id,
        );
    assert!(
        !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            witness,
        )
    );
    assert_eq!(runtime, &before);
}

#[test]
fn supplied_retained_and_private_cp345_drift_are_rejected_transactionally() {
    let (mut runtime, system, mut predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    predecessor.assigned_supply_humidity_ratio = predecessor
        .assigned_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest
        .as_mut()
        .expect("CP345")
        .mixed_air_humidity_ratio_read = false;
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    let mut drift = predecessor;
    drift.mixed_air_humidity_ratio = drift
        .mixed_air_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    runtime
        .set_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id,
            drift,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn active_cp319_selector_corruption_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_dehumidification_flow
        .latest
        .as_mut()
        .expect("CP319")
        .dehumidification_control_type = Some(DehumidificationControlType::Humidistat);
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    let mut drift = runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_dehumidification_flow
        .latest
        .expect("CP319");
    drift.dehumidification_control_type = Some(DehumidificationControlType::Humidistat);
    runtime.set_cooling_dehumidification_flow_latest_witness(system.id, drift);
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn typed_owner_mutation_is_rejected_transactionally() {
    let (mut runtime, mut system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    system.dehumidification_control_type = DehumidificationControlType::ConstantSensibleHeatRatio;
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn private_witnessed_counter_drift_is_rejected_transactionally() {
    macro_rules! drift {
        ($field:ident) => {{
            let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
            runtime
                .units
                .get_mut(&system.id)
                .expect("known unit")
                .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
                .$field = 1;
            assert_rejected_transactionally(&mut runtime, &system, predecessor);
        }};
    }

    drift!(witnessed_positive_guard_false_fallthrough_skip_count);
    drift!(witnessed_dehumidification_control_none_case_selection_count);
    drift!(witnessed_dehumidification_control_constant_sensible_heat_ratio_case_selection_count);
    drift!(witnessed_dehumidification_control_humidistat_case_selection_count);
    drift!(witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selection_count);
}

#[test]
fn private_latest_route_presence_drift_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
        .latest_route = Some(Route::DehumidificationControlNoneCaseSelected);

    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn latest_and_private_witness_presence_drift_is_rejected_transactionally() {
    let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    let mut forged_state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRuntimeState::new(
            system.id,
        );
    let forged_witness =
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_state(
            &mut forged_state,
            predecessor,
            Some(
                PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput {
                    dehumidification_control_type: DehumidificationControlType::None,
                },
            ),
        )
        .expect("valid forged CP346 witness seed");
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            system.id,
            forged_witness,
        );

    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn completed_latest_route_value_drift_fails_closed_without_mutation() {
    let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP346");
    runtime
        .units
        .get_mut(&system.id)
        .expect("known unit")
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
        .latest_route = Some(Route::DehumidificationControlHumidistatCaseSelected);

    assert_completed_state_rejected_without_mutation(&runtime, &system, snapshot);
}

#[test]
fn completed_latest_and_private_witness_bit_drift_fails_closed_without_mutation() {
    let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP346");
    let mut witness_drift = snapshot;
    witness_drift.predecessor_assigned_supply_humidity_ratio = witness_drift
        .predecessor_assigned_supply_humidity_ratio
        .map(|value| f64::from_bits(value.to_bits() ^ 1));
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            system.id,
            witness_drift,
        );

    assert_completed_state_rejected_without_mutation(&runtime, &system, snapshot);
}

#[test]
fn every_active_counter_increment_is_preflighted() {
    let (runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    let baseline = runtime.units.get(&system.id).expect("known unit");
    let active_input = Some(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput {
            dehumidification_control_type: DehumidificationControlType::None,
        },
    );
    macro_rules! overflow {
        ($field:ident) => {{
            let mut unit = baseline.clone();
            unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
                .$field = usize::MAX;
            assert!(!next_transition_fits_for_test(
                &unit,
                predecessor,
                active_input
            ));
        }};
    }
    overflow!(transition_count);
    overflow!(dehumidification_control_switch_count);
    overflow!(source_site_execution_count);
    overflow!(dehumidification_control_type_read_count);
    overflow!(dehumidification_control_switch_dispatch_count);
    overflow!(dehumidification_control_none_case_selection_count);
    overflow!(witnessed_dehumidification_control_none_case_selection_count);

    macro_rules! selector_overflow {
        ($selector:expr, $field:ident) => {{
            let mut unit = baseline.clone();
            unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
                .$field = usize::MAX;
            assert!(!next_transition_fits_for_test(
                &unit,
                predecessor,
                Some(
                    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchActiveInput {
                        dehumidification_control_type: $selector,
                    },
                ),
            ));
        }};
    }
    selector_overflow!(
        DehumidificationControlType::ConstantSensibleHeatRatio,
        dehumidification_control_constant_sensible_heat_ratio_case_selection_count
    );
    selector_overflow!(
        DehumidificationControlType::ConstantSensibleHeatRatio,
        witnessed_dehumidification_control_constant_sensible_heat_ratio_case_selection_count
    );
    selector_overflow!(
        DehumidificationControlType::Humidistat,
        dehumidification_control_humidistat_case_selection_count
    );
    selector_overflow!(
        DehumidificationControlType::Humidistat,
        witnessed_dehumidification_control_humidistat_case_selection_count
    );
    selector_overflow!(
        DehumidificationControlType::ConstantSupplyHumidityRatio,
        dehumidification_control_constant_supply_humidity_ratio_case_selection_count
    );
    selector_overflow!(
        DehumidificationControlType::ConstantSupplyHumidityRatio,
        witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selection_count
    );

    macro_rules! skip_overflow {
        ($demand:expr, $availability:expr, $field:ident) => {{
            let (runtime, system, predecessor) = completed_cp345_case($demand, $availability, true);
            let mut unit = runtime.units.get(&system.id).expect("known unit").clone();
            unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
                .$field = usize::MAX;
            assert!(!next_transition_fits_for_test(&unit, predecessor, None));
        }};
    }
    skip_overflow!(-1_000.0, 0.0, unit_off_skip_count);
    skip_overflow!(1.0, 1.0, non_cooling_skip_count);
    skip_overflow!(-1.0e-40, 1.0, positive_guard_false_fallthrough_skip_count);
    skip_overflow!(
        -1.0e-40,
        1.0,
        witnessed_positive_guard_false_fallthrough_skip_count
    );
}

#[test]
fn self_consistent_wrong_case_history_is_rejected_by_fixed_model_owner() {
    let (mut runtime, system, predecessor) = completed_cp345_case(-100_000.0, 1.0, true);
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP346");
    let mut drift = snapshot;
    drift.dehumidification_control_type = Some(DehumidificationControlType::Humidistat);
    {
        let state = &mut runtime
            .units
            .get_mut(&system.id)
            .expect("known unit")
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch;
        state.dehumidification_control_none_case_selection_count = 0;
        state.witnessed_dehumidification_control_none_case_selection_count = 0;
        state.dehumidification_control_humidistat_case_selection_count = 1;
        state.witnessed_dehumidification_control_humidistat_case_selection_count = 1;
        state.latest = Some(drift);
        state.latest_route = Some(
            super::super::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchRetainedRoute::DehumidificationControlHumidistatCaseSelected,
        );
    }
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            system.id,
            drift,
        );
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_is_consistent(
            &runtime,
            unit,
            &system,
            drift,
            Some(drift),
        )
    );
}
