use ep_model::DehumidificationControlType;

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_cp344_case;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
};

mod public_release;
mod release_corruption;

fn completed_cp346_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlSwitchSnapshot,
)> {
    let (mut runtime, system, predecessor) =
        completed_cp344_case(cooling_demand_w, overall_availability, capacity_limit);
    let cp345 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            predecessor,
        )
        .ok()?;
    let cp346 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
            &mut runtime,
            &system,
            cp345,
        )
        .ok()?;
    Some((runtime, system, cp346))
}

fn completed_cp347_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
)> {
    let (mut runtime, system, cp346) =
        completed_cp346_case(cooling_demand_w, overall_availability, capacity_limit)?;
    let cp347 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
            &mut runtime,
            &system,
            cp346,
        )
        .ok()?;
    Some((runtime, system, cp347))
}

fn private_cp347_case(
    selector: DehumidificationControlType,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseSnapshot,
> {
    let (_, system, mut cp346) = completed_cp346_case(-1_000.0, 1.0, false)?;
    cp346.dehumidification_control_type = Some(selector);
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlNoneCaseRuntimeState::new(
            system.id,
        );
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case_state(
        &mut state, cp346, None,
    )
}

#[test]
fn source_boundary_and_exact_one_site_are_stable() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2213"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2216"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CASE_ENTRY_SOURCE_ORDER,
        ["enter-purchased-air-dehumidification-control-constant-sensible-heat-ratio-case"]
    );
}

#[test]
fn pure_transition_partitions_all_seven_routes_and_only_e_executes_the_site() {
    let mut cases = Vec::new();
    for (demand, availability) in [(-1_000.0, 0.0), (1.0, 1.0), (-1.0e-40, 1.0)] {
        let completed = completed_cp347_case(demand, availability, true);
        assert!(completed.is_some());
        let Some((_, _, snapshot)) = completed else {
            return;
        };
        cases.push((snapshot, false));
    }
    let completed = completed_cp347_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((_, _, none)) = completed else {
        return;
    };
    cases.push((none, false));
    for selector in [
        DehumidificationControlType::ConstantSensibleHeatRatio,
        DehumidificationControlType::Humidistat,
        DehumidificationControlType::ConstantSupplyHumidityRatio,
    ] {
        let predecessor = private_cp347_case(selector);
        assert!(predecessor.is_some());
        let Some(predecessor) = predecessor else {
            return;
        };
        cases.push((
            predecessor,
            selector == DehumidificationControlType::ConstantSensibleHeatRatio,
        ));
    }

    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState::new(
            none.system,
        );
    for (predecessor, entered) in cases {
        let snapshot =
            advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_state(
                &mut state,
                predecessor,
            );
        assert!(snapshot.is_some());
        let Some(snapshot) = snapshot else {
            return;
        };
        assert_eq!(
            snapshot.dehumidification_control_constant_sensible_heat_ratio_case_entered,
            entered
        );
    }
    assert_eq!(state.transition_count, 7);
    assert_eq!(state.unit_off_skip_count, 1);
    assert_eq!(state.non_cooling_skip_count, 1);
    assert_eq!(state.positive_guard_false_fallthrough_skip_count, 1);
    assert_eq!(
        state.dehumidification_control_none_case_completed_skip_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_case_entry_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_humidistat_case_selected_skip_count,
        1
    );
    assert_eq!(
        state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        1
    );
    assert_eq!(state.source_site_execution_count, 1);
    assert_eq!(
        state.dehumidification_control_constant_sensible_heat_ratio_case_entry_site_count,
        1
    );
}

#[test]
fn pure_transition_rejects_malformed_cp347_without_mutation() {
    let completed = completed_cp347_case(-1_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((_, system, mut predecessor)) = completed else {
        return;
    };
    let valid_predecessor = predecessor;
    predecessor.dehumidification_control_none_case_exited_via_break = false;
    let mut state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState::new(
            system.id,
        );
    let before = state.clone();
    assert!(
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_state(
            &mut state,
            predecessor,
        )
        .is_none()
    );
    assert_eq!(state, before);

    let mut mismatched_state =
        PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCaseEntryRuntimeState::new(
            ep_model::IdealLoadsAirSystemId(system.id.0 + 1),
        );
    let before = mismatched_state.clone();
    assert!(
        advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry_state(
            &mut mismatched_state,
            valid_predecessor,
        )
        .is_none()
    );
    assert_eq!(mismatched_state, before);
}
