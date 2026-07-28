use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state,
    completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent,
    release::next_transition_fits_for_test,
};
use super::completed_cp346_case;
use crate::ideal_loads::advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;

fn flipped(value: f64) -> f64 {
    f64::from_bits(value.to_bits() ^ 1)
}

fn assert_rejected_transactionally(
    runtime: &mut crate::ideal_loads::PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
    predecessor: crate::ideal_loads::PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
) {
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
            runtime,
            system,
            predecessor,
        )
        .is_err()
    );
    assert_eq!(&*runtime, &before);
}

fn active_input(
    runtime: &crate::ideal_loads::PurchasedAirRuntimeState,
    system: ep_model::IdealLoadsAirSystemId,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput,
>{
    Some(
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseActiveInput {
            mixed_air_humidity_ratio: runtime
                .units
                .get(&system)?
                .calc_cooling_mixed_air_call
                .latest?
                .mixed_air_humidity_ratio?,
        },
    )
}

#[test]
fn supplied_retained_and_private_cp346_drift_are_rejected_transactionally() {
    let (mut runtime, system, mut predecessor) =
        completed_cp346_case(-100_000.0, 1.0, true).expect("CP346 corruption test prefix");
    predecessor.predecessor_assigned_supply_humidity_ratio = predecessor
        .predecessor_assigned_supply_humidity_ratio
        .map(flipped);
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let (mut runtime, system, predecessor) =
        completed_cp346_case(-100_000.0, 1.0, true).expect("CP346 corruption test prefix");
    if let Some(unit) = runtime.units.get_mut(&system.id)
        && let Some(latest) = unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch
            .latest
            .as_mut()
    {
        latest.dehumidification_control_switch_dispatched = false;
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let (mut runtime, system, predecessor) =
        completed_cp346_case(-100_000.0, 1.0, true).expect("CP346 corruption test prefix");
    let mut drift = predecessor;
    drift.predecessor_assigned_supply_humidity_ratio = drift
        .predecessor_assigned_supply_humidity_ratio
        .map(flipped);
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch_latest_witness(
            system.id,
            drift,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn cp329_owner_and_cp345_corroboration_corruption_are_rejected_transactionally() {
    let (mut runtime, system, predecessor) =
        completed_cp346_case(-100_000.0, 1.0, true).expect("CP346 owner test prefix");
    if let Some(unit) = runtime.units.get_mut(&system.id)
        && let Some(owner) = unit.calc_cooling_mixed_air_call.latest.as_mut()
    {
        owner.mixed_air_humidity_ratio = owner.mixed_air_humidity_ratio.map(flipped);
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);

    let (mut runtime, system, predecessor) =
        completed_cp346_case(-100_000.0, 1.0, true).expect("CP346 corroboration test prefix");
    if let Some(unit) = runtime.units.get_mut(&system.id)
        && let Some(cp345) = unit
            .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .latest
            .as_mut()
    {
        cp345.assigned_supply_humidity_ratio = cp345.assigned_supply_humidity_ratio.map(flipped);
    }
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn private_latest_and_witness_presence_drift_are_rejected_transactionally() {
    let (mut runtime, system, predecessor) =
        completed_cp346_case(-100_000.0, 1.0, true).expect("CP346 witness test prefix");
    let mut forged_state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState::new(
            system.id,
        );
    let forged_witness =
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state(
            &mut forged_state,
            predecessor,
            active_input(&runtime, system.id),
        )
        .expect("valid forged CP347 witness seed");
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
            system.id,
            forged_witness,
        );
    assert_rejected_transactionally(&mut runtime, &system, predecessor);
}

#[test]
fn completed_private_route_and_witness_bit_drift_fail_closed() {
    let (mut runtime, system, predecessor) =
        completed_cp346_case(-100_000.0, 1.0, true).expect("CP346 route test prefix");
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP347 must complete");
    if let Some(unit) = runtime.units.get_mut(&system.id) {
        unit.calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case
            .latest_route = Some(Route::DehumidificationControlHumidistatCaseSelected);
    }
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            runtime
                .cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
                    system.id,
                ),
        )
    );

    let (mut runtime, system, predecessor) =
        completed_cp346_case(-100_000.0, 1.0, true).expect("CP346 witness-bit test prefix");
    let snapshot =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
            &mut runtime,
            &system,
            predecessor,
        )
        .expect("CP347 must complete");
    let mut drift = snapshot;
    drift.resulting_supply_humidity_ratio = drift.resulting_supply_humidity_ratio.map(flipped);
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_latest_witness(
            system.id,
            drift,
        );
    let unit = runtime.units.get(&system.id).expect("known unit");
    assert!(
        !completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_is_consistent(
            &runtime,
            unit,
            &system,
            snapshot,
            Some(drift),
        )
    );
}

#[test]
fn every_none_case_counter_increment_is_preflighted() {
    let (runtime, system, predecessor) =
        completed_cp346_case(-100_000.0, 1.0, true).expect("CP346 overflow test prefix");
    let baseline = runtime.units.get(&system.id).expect("known unit");
    let input = active_input(&runtime, system.id);
    macro_rules! overflow {
        ($field:ident) => {{
            let mut unit = baseline.clone();
            let state = &mut unit
                .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case;
            state.$field = usize::MAX;
            assert!(!next_transition_fits_for_test(&unit, predecessor, input));
        }};
    }
    overflow!(transition_count);
    overflow!(dehumidification_control_none_case_completion_count);
    overflow!(source_site_execution_count);
    overflow!(dehumidification_control_none_case_entry_count);
    overflow!(mixed_air_humidity_ratio_read_count);
    overflow!(supply_humidity_ratio_assignment_count);
    overflow!(dehumidification_control_none_case_break_count);
    overflow!(witnessed_dehumidification_control_none_case_completion_count);
}
