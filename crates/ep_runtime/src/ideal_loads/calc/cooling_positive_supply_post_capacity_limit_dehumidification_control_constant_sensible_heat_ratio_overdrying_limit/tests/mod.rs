use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state as advance,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_cp344_case;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSupplyEnthalpyAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
};
use crate::psychrometrics::{energyplus_psy_cp_air_fn_w, energyplus_psy_h_fn_tdb_w};

mod ieee;
mod public_release;
mod release_corruption;

pub(in crate::ideal_loads::calc) fn completed_cp352_case(
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
    let cp352 =
        advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment(
            &mut runtime,
            &system,
            cp351,
        )
        .ok()?;
    Some((runtime, system, cp352))
}

pub(super) fn predecessor(route: Route, ordinal: usize) -> Predecessor {
    let active = matches!(
        route,
        Route::DehumidificationControlNoneCaseCompletedSkip
            | Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted
            | Route::DehumidificationControlHumidistatCaseSelectedSkip
            | Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip
    );
    let constant =
        route == Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted;
    let selector = match route {
        Route::DehumidificationControlNoneCaseCompletedSkip => {
            Some(DehumidificationControlType::None)
        }
        Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted => {
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
    let mixed = 50_000.0;
    let total = 10_000.0;
    let flow = 2.0;
    let specific = total / flow;
    let enthalpy = mixed - specific;
    Predecessor {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SUPPLY_ENTHALPY_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed:
            constant,
        predecessor_dehumidification_control_humidistat_case_selected_skip:
            route == Route::DehumidificationControlHumidistatCaseSelectedSkip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
        dehumidification_control_none_case_completed_skip:
            route == Route::DehumidificationControlNoneCaseCompletedSkip,
        dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed:
            constant,
        dehumidification_control_humidistat_case_selected_skip:
            route == Route::DehumidificationControlHumidistatCaseSelectedSkip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
        mixed_air_enthalpy_read: constant,
        mixed_air_enthalpy_j_per_kg: constant.then_some(mixed),
        cooling_total_output_read: constant,
        cooling_total_output_w: constant.then_some(total),
        supply_mass_flow_rate_read: constant,
        supply_mass_flow_rate_kg_per_s: constant.then_some(flow),
        specific_cooling_output_calculated: constant,
        specific_cooling_output_j_per_kg: constant.then_some(specific),
        supply_enthalpy_calculated: constant,
        calculated_supply_enthalpy_j_per_kg: constant.then_some(enthalpy),
        supply_enthalpy_assigned: constant,
        assigned_supply_enthalpy_j_per_kg: constant.then_some(enthalpy),
        resulting_supply_enthalpy_j_per_kg: constant.then_some(enthalpy),
    }
}

pub(super) const fn active_operands(pre_limit: f64, temperature: f64) -> ActiveOperands {
    ActiveOperands {
        supply_enthalpy_before_overdrying_limit_j_per_kg: pre_limit,
        supply_temperature_c: temperature,
    }
}

pub(super) fn retained_supply_temperature(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Option<f64> {
    let unit = runtime.units.get(&system)?;
    let provenance = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest?;
    if provenance.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed {
        unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .latest?
            .resulting_supply_temperature_c
    } else {
        unit.calc_cooling_positive_supply_temperature_mixed_air_limit
            .latest?
            .assigned_supply_temperature_c
    }
}

pub(in crate::ideal_loads::calc) fn private_active_predecessor(
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
    let mixed_temperature = mixed_owner.mixed_air_temperature_c?;
    let mixed_enthalpy = mixed_owner.mixed_air_enthalpy_projection_j_per_kg?;
    let supply_temperature = retained_supply_temperature(runtime, direct.system)?;
    let cp_air = energyplus_psy_cp_air_fn_w(humidity);
    let sensible = (flow * cp_air) * (mixed_temperature - supply_temperature);
    let total = sensible / system.cooling_sensible_heat_ratio;
    let specific = total / flow;
    let supply_enthalpy = mixed_enthalpy - specific;

    direct.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::ConstantSensibleHeatRatio);
    direct.predecessor_dehumidification_control_none_case_completed_skip = false;
    direct
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed =
        true;
    direct.dehumidification_control_none_case_completed_skip = false;
    direct
        .dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed =
        true;
    direct.mixed_air_enthalpy_read = true;
    direct.mixed_air_enthalpy_j_per_kg = Some(mixed_enthalpy);
    direct.cooling_total_output_read = true;
    direct.cooling_total_output_w = Some(total);
    direct.supply_mass_flow_rate_read = true;
    direct.supply_mass_flow_rate_kg_per_s = Some(flow);
    direct.specific_cooling_output_calculated = true;
    direct.specific_cooling_output_j_per_kg = Some(specific);
    direct.supply_enthalpy_calculated = true;
    direct.calculated_supply_enthalpy_j_per_kg = Some(supply_enthalpy);
    direct.supply_enthalpy_assigned = true;
    direct.assigned_supply_enthalpy_j_per_kg = Some(supply_enthalpy);
    direct.resulting_supply_enthalpy_j_per_kg = Some(supply_enthalpy);
    Some(direct)
}

#[test]
fn source_boundary_five_sites_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2221"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2222"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_OVERDRYING_LIMIT_SOURCE_ORDER,
        &[
            "read-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit-maximum",
            "read-purchased-air-supply-temperature-for-constant-sensible-heat-ratio-overdrying-limit-enthalpy",
            "evaluate-psy-h-fn-tdb-w-at-minimum-humidity-ratio-for-constant-sensible-heat-ratio-overdrying-limit",
            "apply-source-shaped-two-argument-maximum-for-constant-sensible-heat-ratio-overdrying-limit",
            "assign-local-supply-enthalpy-for-constant-sensible-heat-ratio-overdrying-limit",
        ]
    );
    let routes = [
        Route::UnitOff,
        Route::NonCooling,
        Route::PositiveGuardFalseFallthrough,
        Route::DehumidificationControlNoneCaseCompletedSkip,
        Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted,
        Route::DehumidificationControlHumidistatCaseSelectedSkip,
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
    ];
    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in routes.into_iter().enumerate() {
        let operands = (route
            == Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted)
            .then_some(active_operands(40_000.0, 12.0));
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
            .dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_count,
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
    assert_eq!(state.source_site_execution_count, 5);
    for count in [
        state.supply_enthalpy_for_overdrying_limit_maximum_read_count,
        state.supply_temperature_for_minimum_humidity_ratio_enthalpy_read_count,
        state.psychrometric_minimum_supply_enthalpy_evaluation_count,
        state.source_shaped_two_argument_maximum_evaluation_count,
        state.supply_enthalpy_assignment_write_count,
    ] {
        assert_eq!(count, 1);
    }
}

#[test]
fn active_limit_uses_canonical_psychrometrics_and_source_shaped_maximum() {
    let mut state = State::new(IdealLoadsAirSystemId(7));
    let operands = active_operands(10_000.0, 12.345_678_9);
    let snapshot = advance(&mut state, predecessor(Route::DehumidificationControlConstantSensibleHeatRatioOverdryingLimitExecuted, 1), Some(operands));
    assert!(snapshot.is_some());
    let Some(snapshot) = snapshot else {
        return;
    };
    let minimum = energyplus_psy_h_fn_tdb_w(operands.supply_temperature_c, 1.0e-5);
    let expected = if operands.supply_enthalpy_before_overdrying_limit_j_per_kg < minimum {
        minimum
    } else {
        operands.supply_enthalpy_before_overdrying_limit_j_per_kg
    };
    assert_eq!(
        snapshot
            .psychrometric_minimum_supply_enthalpy_j_per_kg
            .map(f64::to_bits),
        Some(minimum.to_bits())
    );
    for value in [
        snapshot.maximum_supply_enthalpy_j_per_kg,
        snapshot.assigned_supply_enthalpy_j_per_kg,
        snapshot.resulting_supply_enthalpy_j_per_kg,
    ] {
        assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
    }
}
