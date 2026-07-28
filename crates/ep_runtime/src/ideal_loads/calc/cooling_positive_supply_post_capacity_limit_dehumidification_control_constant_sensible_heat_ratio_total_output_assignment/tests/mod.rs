use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_state as advance,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_cp344_case;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

mod ieee;
mod public_release;
mod release_corruption;

pub(super) fn completed_cp350_case(
    cooling_demand_w: f64,
    overall_availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    Predecessor,
)> {
    let (mut runtime, system, cp344) =
        completed_cp344_case(cooling_demand_w, overall_availability, capacity_limit);
    let cp345 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment(
            &mut runtime,
            &system,
            cp344,
        )
        .ok()?;
    let cp346 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch(
            &mut runtime,
            &system,
            cp345,
        )
        .ok()?;
    let cp347 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case(
            &mut runtime,
            &system,
            cp346,
        )
        .ok()?;
    let cp348 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry(
            &mut runtime,
            &system,
            cp347,
        )
        .ok()?;
    let cp349 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment(
            &mut runtime,
            &system,
            cp348,
        )
        .ok()?;
    let cp350 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment(
            &mut runtime,
            &system,
            cp349,
        )
        .ok()?;
    Some((runtime, system, cp350))
}

pub(super) fn predecessor(route: Route, ordinal: usize) -> Predecessor {
    let active = matches!(
        route,
        Route::DehumidificationControlNoneCaseCompletedSkip
            | Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned
            | Route::DehumidificationControlHumidistatCaseSelectedSkip
            | Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip
    );
    let constant =
        route == Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned;
    let selector = match route {
        Route::DehumidificationControlNoneCaseCompletedSkip => {
            Some(DehumidificationControlType::None)
        }
        Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned => {
            Some(DehumidificationControlType::ConstantSensibleHeatRatio)
        }
        Route::DehumidificationControlHumidistatCaseSelectedSkip => {
            Some(DehumidificationControlType::Humidistat)
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => {
            Some(DehumidificationControlType::ConstantSupplyHumidityRatio)
        }
        _ => None,
    };
    let flow = 1.5;
    let cp_air = energyplus_psy_cp_air_fn_w(0.008);
    let mixed = 25.0;
    let supply = 15.0;
    let first = flow * cp_air;
    let difference = mixed - supply;
    let sensible = first * difference;
    Predecessor {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(7),
        parent_call_ordinal: ordinal,
        controlled_zone: ZoneId(11),
        unit_body_entered: route != Route::UnitOff,
        predecessor_cooling_body_entered: !matches!(route, Route::UnitOff | Route::NonCooling),
        predecessor_no_outdoor_air_fallback_entered: !matches!(
            route,
            Route::UnitOff | Route::NonCooling
        ),
        predecessor_positive_supply_mass_flow_body_entered: active,
        unit_off_skipped: route == Route::UnitOff,
        non_cooling_skipped: route == Route::NonCooling,
        positive_guard_false_fallthrough_skipped:
            route == Route::PositiveGuardFalseFallthrough,
        predecessor_dehumidification_control_type: selector,
        predecessor_dehumidification_control_none_case_completed_skip:
            route == Route::DehumidificationControlNoneCaseCompletedSkip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed:
            constant,
        predecessor_dehumidification_control_humidistat_case_selected_skip:
            route == Route::DehumidificationControlHumidistatCaseSelectedSkip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
        dehumidification_control_none_case_completed_skip:
            route == Route::DehumidificationControlNoneCaseCompletedSkip,
        dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed:
            constant,
        dehumidification_control_humidistat_case_selected_skip:
            route == Route::DehumidificationControlHumidistatCaseSelectedSkip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
        supply_mass_flow_rate_read: constant,
        supply_mass_flow_rate_kg_per_s: constant.then_some(flow),
        cp_air_read: constant,
        cp_air_j_per_kg_k: constant.then_some(cp_air),
        supply_mass_flow_rate_times_cp_air_calculated: constant,
        supply_mass_flow_rate_times_cp_air_w_per_k: constant.then_some(first),
        mixed_air_temperature_read: constant,
        mixed_air_temperature_c: constant.then_some(mixed),
        supply_temperature_read: constant,
        supply_temperature_c: constant.then_some(supply),
        mixed_air_minus_supply_temperature_calculated: constant,
        mixed_air_minus_supply_temperature_k: constant.then_some(difference),
        cooling_sensible_output_calculated: constant,
        calculated_cooling_sensible_output_w: constant.then_some(sensible),
        cooling_sensible_output_assigned: constant,
        cooling_sensible_output_w: constant.then_some(sensible),
    }
}

pub(super) const fn active_input(ratio: f64) -> ActiveInput {
    ActiveInput {
        cooling_sensible_heat_ratio: ratio,
    }
}

pub(super) fn private_active_predecessor(
    mut direct: Predecessor,
    runtime: &PurchasedAirRuntimeState,
) -> Option<Predecessor> {
    let unit = runtime.units.get(&direct.system)?;
    let flow = unit
        .calc_cooling_supply_mass_flow_positive_guard
        .latest?
        .supply_mass_flow_rate_kg_per_s?;
    let mixed_owner = unit.calc_cooling_mixed_air_call.latest?;
    let humidity = mixed_owner.mixed_air_humidity_ratio?;
    let mixed = mixed_owner.mixed_air_temperature_c?;
    let cp_air = energyplus_psy_cp_air_fn_w(humidity);
    let provenance = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest?;
    let supply = if provenance.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed {
        unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .latest?
            .resulting_supply_temperature_c?
    } else {
        unit.calc_cooling_positive_supply_temperature_mixed_air_limit
            .latest?
            .assigned_supply_temperature_c?
    };
    let first = flow * cp_air;
    let difference = mixed - supply;
    let sensible = first * difference;

    direct.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::ConstantSensibleHeatRatio);
    direct.predecessor_dehumidification_control_none_case_completed_skip = false;
    direct
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed =
        true;
    direct.dehumidification_control_none_case_completed_skip = false;
    direct
        .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed =
        true;
    direct.supply_mass_flow_rate_read = true;
    direct.supply_mass_flow_rate_kg_per_s = Some(flow);
    direct.cp_air_read = true;
    direct.cp_air_j_per_kg_k = Some(cp_air);
    direct.supply_mass_flow_rate_times_cp_air_calculated = true;
    direct.supply_mass_flow_rate_times_cp_air_w_per_k = Some(first);
    direct.mixed_air_temperature_read = true;
    direct.mixed_air_temperature_c = Some(mixed);
    direct.supply_temperature_read = true;
    direct.supply_temperature_c = Some(supply);
    direct.mixed_air_minus_supply_temperature_calculated = true;
    direct.mixed_air_minus_supply_temperature_k = Some(difference);
    direct.cooling_sensible_output_calculated = true;
    direct.calculated_cooling_sensible_output_w = Some(sensible);
    direct.cooling_sensible_output_assigned = true;
    direct.cooling_sensible_output_w = Some(sensible);
    Some(direct)
}

#[test]
fn source_boundary_four_sites_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2218"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2219"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
        4
    );
    let routes = [
        Route::UnitOff,
        Route::NonCooling,
        Route::PositiveGuardFalseFallthrough,
        Route::DehumidificationControlNoneCaseCompletedSkip,
        Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned,
        Route::DehumidificationControlHumidistatCaseSelectedSkip,
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
    ];
    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in routes.into_iter().enumerate() {
        let input =
            (route == Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned)
                .then_some(active_input(0.7));
        assert!(
            advance(&mut state, predecessor(route, index + 1), input).is_some(),
            "{route:?}"
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
        state
            .dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_count,
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
    assert_eq!(state.source_site_execution_count, 4);
    for count in [
        state.cooling_sensible_output_read_count,
        state.cooling_sensible_heat_ratio_read_count,
        state.cooling_total_output_calculation_count,
        state.cooling_total_output_assignment_write_count,
    ] {
        assert_eq!(count, 1);
    }
}

#[test]
fn private_constant_shr_division_reads_cp350_owner_and_system_ratio_once() {
    let predecessor = predecessor(
        Route::DehumidificationControlConstantSensibleHeatRatioTotalOutputAssigned,
        1,
    );
    let sensible = predecessor.cooling_sensible_output_w;
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let snapshot = advance(&mut state, predecessor, Some(active_input(0.7)));
    assert!(snapshot.is_some());
    let Some(snapshot) = snapshot else {
        return;
    };
    assert_eq!(
        snapshot.cooling_sensible_output_w.map(f64::to_bits),
        sensible.map(f64::to_bits)
    );
    assert_eq!(
        snapshot.cooling_sensible_heat_ratio.map(f64::to_bits),
        Some(0.7f64.to_bits())
    );
    assert_eq!(
        snapshot.cooling_total_output_w.map(f64::to_bits),
        sensible.map(|value| (value / 0.7).to_bits())
    );
    assert_eq!(state.source_site_execution_count, 4);
}
