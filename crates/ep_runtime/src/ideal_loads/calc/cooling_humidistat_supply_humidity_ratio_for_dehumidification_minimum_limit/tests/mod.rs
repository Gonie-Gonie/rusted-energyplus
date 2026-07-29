use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId, ZoneId};

use super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitActiveOperands as Operands,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitRetainedRoute as Route,
};
use crate::ideal_loads::calc::cooling_constant_shr_supply_humidity_ratio_mixed_air_limit::completed_cp355_case;
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentActiveOperands as Cp360Operands,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState as Cp360State,
    advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_state as advance_cp360,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot as Cp359,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, advance_direct_no_oa_calc_cooling_constant_shr_case_break,
    advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit,
    advance_direct_no_oa_calc_cooling_humidistat_case_entry,
    advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment,
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment,
};

mod ieee;
mod overflow;
mod release;
mod release_corruption;
mod routes;

pub(super) const U: Route = Route::UnitOff;
pub(super) const N: Route = Route::NonCooling;
pub(super) const P: Route = Route::PositiveGuardFalseFallthrough;
pub(super) const C0: Route = Route::DehumidificationControlNoneCaseCompletedSkip;
pub(super) const Q: Route =
    Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip;
pub(super) const H: Route =
    Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitExecuted;
pub(super) const CSH: Route =
    Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip;

pub(super) fn operands(route: Route, minimum: f64) -> Option<Operands> {
    (route == H).then_some(Operands {
        minimum_cooling_supply_air_humidity_ratio: minimum,
    })
}

pub(super) fn predecessor(route: Route, ordinal: usize, requested_left: f64) -> Predecessor {
    let cp359 = cp359_predecessor(route, ordinal, requested_left);
    let active = route == H;
    let zone_humidity = if requested_left == 0.0 {
        requested_left
    } else {
        -0.0
    };
    advance_cp360(
        &mut Cp360State::new(cp359.system),
        cp359,
        active.then_some(Cp360Operands {
            supply_mass_flow_rate_kg_per_s: 1.0,
            zone_node_humidity_ratio: zone_humidity,
        }),
    )
    .expect("canonical CP360 predecessor")
}

fn cp359_predecessor(route: Route, ordinal: usize, demand: f64) -> Cp359 {
    let active = matches!(route, C0 | Q | H | CSH);
    let humidistat = route == H;
    let selector = match route {
        C0 => Some(DehumidificationControlType::None),
        Q => Some(DehumidificationControlType::ConstantSensibleHeatRatio),
        H => Some(DehumidificationControlType::Humidistat),
        CSH => Some(DehumidificationControlType::ConstantSupplyHumidityRatio),
        _ => None,
    };
    Cp359 {
        source: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_MOISTURE_DEMAND_ASSIGNMENT_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(7),
        parent_call_ordinal: ordinal,
        controlled_zone: ZoneId(9),
        unit_body_entered: route != U,
        predecessor_cooling_body_entered: matches!(route, P | C0 | Q | H | CSH),
        predecessor_no_outdoor_air_fallback_entered: matches!(route, P | C0 | Q | H | CSH),
        predecessor_positive_supply_mass_flow_body_entered: active,
        unit_off_skipped: route == U,
        non_cooling_skipped: route == N,
        positive_guard_false_fallthrough_skipped: route == P,
        predecessor_dehumidification_control_type: selector,
        predecessor_dehumidification_control_none_case_completed_skip: route == C0,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route
            == Q,
        predecessor_dehumidification_control_humidistat_case_entered: humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            route == CSH,
        dehumidification_control_none_case_completed_skip: route == C0,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route == Q,
        dehumidification_control_humidistat_moisture_demand_assignment_executed: humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: route == CSH,
        zone_dehumidifying_setpoint_moisture_demand_read: humidistat,
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s: humidistat.then_some(demand),
        zone_dehumidifying_setpoint_moisture_demand_assigned: humidistat,
        assigned_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: humidistat.then_some(demand),
        resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: humidistat
            .then_some(demand),
    }
}

pub(super) fn completed_cp360_case(
    cooling_demand_w: f64,
    availability: f64,
    capacity_limit: bool,
) -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    Predecessor,
)> {
    let (mut runtime, system, cp355) =
        completed_cp355_case(cooling_demand_w, availability, capacity_limit)?;
    let cp356 =
        advance_direct_no_oa_calc_cooling_constant_shr_supply_humidity_ratio_mixed_air_limit(
            &mut runtime,
            &system,
            cp355,
        )
        .ok()?;
    let cp357 =
        advance_direct_no_oa_calc_cooling_constant_shr_case_break(&mut runtime, &system, cp356)
            .ok()?;
    let cp358 =
        advance_direct_no_oa_calc_cooling_humidistat_case_entry(&mut runtime, &system, cp357)
            .ok()?;
    let cp359 = advance_direct_no_oa_calc_cooling_humidistat_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp358,
    )
    .ok()?;
    let cp360 =
        advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment(
            &mut runtime,
            &system,
            cp359,
        )
        .ok()?;
    Some((runtime, system, cp360))
}
