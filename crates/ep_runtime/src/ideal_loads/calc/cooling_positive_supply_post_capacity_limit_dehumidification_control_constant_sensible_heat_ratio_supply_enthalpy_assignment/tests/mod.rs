use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_state as advance,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_cp344_case;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioTotalOutputAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

mod ieee;
mod public_release;
mod release_corruption;

pub(super) fn completed_cp351_case(
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
    let cp351 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment(
            &mut runtime,
            &system,
            cp350,
        )
        .ok()?;
    Some((runtime, system, cp351))
}

pub(super) fn predecessor(route: Route, ordinal: usize) -> Predecessor {
    let active = matches!(
        route,
        Route::DehumidificationControlNoneCaseCompletedSkip
            | Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned
            | Route::DehumidificationControlHumidistatCaseSelectedSkip
            | Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip
    );
    let constant =
        route == Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned;
    let selector = match route {
        Route::DehumidificationControlNoneCaseCompletedSkip => {
            Some(DehumidificationControlType::None)
        }
        Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned => {
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
    let sensible = 10_000.0;
    let ratio = 0.8;
    let total = sensible / ratio;
    Predecessor {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_TOTAL_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed:
            constant,
        predecessor_dehumidification_control_humidistat_case_selected_skip:
            route == Route::DehumidificationControlHumidistatCaseSelectedSkip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
        dehumidification_control_none_case_completed_skip:
            route == Route::DehumidificationControlNoneCaseCompletedSkip,
        dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed:
            constant,
        dehumidification_control_humidistat_case_selected_skip:
            route == Route::DehumidificationControlHumidistatCaseSelectedSkip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
        cooling_sensible_output_read: constant,
        cooling_sensible_output_w: constant.then_some(sensible),
        cooling_sensible_heat_ratio_read: constant,
        cooling_sensible_heat_ratio: constant.then_some(ratio),
        cooling_total_output_calculated: constant,
        calculated_cooling_total_output_w: constant.then_some(total),
        cooling_total_output_assigned: constant,
        cooling_total_output_w: constant.then_some(total),
    }
}

pub(super) const fn active_operands(
    mixed: f64,
    total: f64,
    flow: f64,
) -> ActiveOperands {
    ActiveOperands {
        mixed_air_enthalpy_j_per_kg: mixed,
        cooling_total_output_w: total,
        supply_mass_flow_rate_kg_per_s: flow,
    }
}

pub(super) fn private_active_predecessor(
    mut direct: Predecessor,
    runtime: &PurchasedAirRuntimeState,
    system: &ep_model::IdealLoadsAirSystem,
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
    let supply = if provenance
        .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
    {
        unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .latest?
            .resulting_supply_temperature_c?
    } else {
        unit.calc_cooling_positive_supply_temperature_mixed_air_limit
            .latest?
            .assigned_supply_temperature_c?
    };
    let sensible = (flow * cp_air) * (mixed - supply);
    let ratio = system.cooling_sensible_heat_ratio;
    let total = sensible / ratio;

    direct.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::ConstantSensibleHeatRatio);
    direct.predecessor_dehumidification_control_none_case_completed_skip = false;
    direct
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_executed =
        true;
    direct.dehumidification_control_none_case_completed_skip = false;
    direct
        .dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed =
        true;
    direct.cooling_sensible_output_read = true;
    direct.cooling_sensible_output_w = Some(sensible);
    direct.cooling_sensible_heat_ratio_read = true;
    direct.cooling_sensible_heat_ratio = Some(ratio);
    direct.cooling_total_output_calculated = true;
    direct.calculated_cooling_total_output_w = Some(total);
    direct.cooling_total_output_assigned = true;
    direct.cooling_total_output_w = Some(total);
    Some(direct)
}

#[test]
fn source_boundary_six_sites_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2219"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2221"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER.len(),
        6
    );
    let routes = [
        Route::UnitOff,
        Route::NonCooling,
        Route::PositiveGuardFalseFallthrough,
        Route::DehumidificationControlNoneCaseCompletedSkip,
        Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned,
        Route::DehumidificationControlHumidistatCaseSelectedSkip,
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
    ];
    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in routes.into_iter().enumerate() {
        let operands = (route
            == Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned)
            .then_some(active_operands(50_000.0, 10_000.0, 2.0));
        assert!(
            advance(&mut state, predecessor(route, index + 1), operands).is_some(),
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
            .dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_count,
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
    assert_eq!(state.source_site_execution_count, 6);
    for count in [
        state.mixed_air_enthalpy_read_count,
        state.cooling_total_output_read_count,
        state.supply_mass_flow_rate_read_count,
        state.specific_cooling_output_calculation_count,
        state.supply_enthalpy_calculation_count,
        state.supply_enthalpy_assignment_write_count,
    ] {
        assert_eq!(count, 1);
    }
}

#[test]
fn active_assignment_preserves_exact_grouping_and_result_owner_bits() {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let operands = active_operands(50_000.0, 12_345.0, 1.5);
    let snapshot = advance(
        &mut state,
        predecessor(
            Route::DehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssigned,
            1,
        ),
        Some(operands),
    );
    assert!(snapshot.is_some());
    let Some(snapshot) = snapshot else {
        return;
    };
    let specific = operands.cooling_total_output_w / operands.supply_mass_flow_rate_kg_per_s;
    let expected = operands.mixed_air_enthalpy_j_per_kg - specific;
    assert_eq!(
        snapshot.specific_cooling_output_j_per_kg.map(f64::to_bits),
        Some(specific.to_bits())
    );
    for value in [
        snapshot.calculated_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ] {
        assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
    }
}
