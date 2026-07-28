use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentRuntimeState as State,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_state as advance,
};
use crate::ideal_loads::calc::cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment::completed_cp344_case;
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_case_entry,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_none_case,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_switch,
    advance_direct_no_oa_calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment,
};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

mod ieee;
mod public_release;
mod release_corruption;

pub(super) fn completed_cp349_case(
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
    Some((runtime, system, cp349))
}

pub(super) fn predecessor(route: Route, ordinal: usize) -> Predecessor {
    let active = matches!(
        route,
        Route::DehumidificationControlNoneCaseCompletedSkip
            | Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned
            | Route::DehumidificationControlHumidistatCaseSelectedSkip
            | Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip
    );
    let constant =
        route == Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned;
    let humidity = 0.008;
    let cp_air = energyplus_psy_cp_air_fn_w(humidity);
    let selector = match route {
        Route::DehumidificationControlNoneCaseCompletedSkip => {
            Some(DehumidificationControlType::None)
        }
        Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned => {
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
    Predecessor {
        source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
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
        predecessor_dehumidification_control_none_case_completed:
            route == Route::DehumidificationControlNoneCaseCompletedSkip,
        predecessor_dehumidification_control_none_case_completed_skip:
            route == Route::DehumidificationControlNoneCaseCompletedSkip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: constant,
        predecessor_dehumidification_control_humidistat_case_selected_skip:
            route == Route::DehumidificationControlHumidistatCaseSelectedSkip,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
        dehumidification_control_none_case_completed_skip:
            route == Route::DehumidificationControlNoneCaseCompletedSkip,
        dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed: constant,
        dehumidification_control_humidistat_case_selected_skip:
            route == Route::DehumidificationControlHumidistatCaseSelectedSkip,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
        mixed_air_humidity_ratio_read: constant,
        mixed_air_humidity_ratio: constant.then_some(humidity),
        psychrometric_cp_air_evaluated: constant,
        psychrometric_cp_air_result_j_per_kg_k: constant.then_some(cp_air),
        cp_air_assigned: constant,
        cp_air_j_per_kg_k: constant.then_some(cp_air),
    }
}

pub(super) const fn active_input(flow: f64, mixed: f64, supply: f64) -> ActiveInput {
    ActiveInput {
        supply_mass_flow_rate_kg_per_s: flow,
        mixed_air_temperature_c: mixed,
        supply_temperature_c: supply,
    }
}

#[test]
fn source_boundary_eight_sites_and_seven_route_algebra_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2217"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2218"
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_POSITIVE_SUPPLY_POST_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_SENSIBLE_OUTPUT_ASSIGNMENT_SOURCE_ORDER.len(),
        8
    );

    let routes = [
        Route::UnitOff,
        Route::NonCooling,
        Route::PositiveGuardFalseFallthrough,
        Route::DehumidificationControlNoneCaseCompletedSkip,
        Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned,
        Route::DehumidificationControlHumidistatCaseSelectedSkip,
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
    ];
    let mut state = State::new(IdealLoadsAirSystemId(7));
    for (index, route) in routes.into_iter().enumerate() {
        let input = (route
            == Route::DehumidificationControlConstantSensibleHeatRatioSensibleOutputAssigned)
            .then_some(active_input(1.5, 25.0, 15.0));
        let snapshot = advance(&mut state, predecessor(route, index + 1), input);
        assert!(snapshot.is_some(), "{route:?}");
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
            .dehumidification_control_constant_sensible_heat_ratio_sensible_output_assignment_count,
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
    assert_eq!(state.source_site_execution_count, 8);
    for count in [
        state.supply_mass_flow_rate_read_count,
        state.cp_air_read_count,
        state.supply_mass_flow_rate_times_cp_air_calculation_count,
        state.mixed_air_temperature_read_count,
        state.supply_temperature_read_count,
        state.mixed_air_minus_supply_temperature_calculation_count,
        state.cooling_sensible_output_calculation_count,
        state.cooling_sensible_output_assignment_write_count,
    ] {
        assert_eq!(count, 1);
    }
}
