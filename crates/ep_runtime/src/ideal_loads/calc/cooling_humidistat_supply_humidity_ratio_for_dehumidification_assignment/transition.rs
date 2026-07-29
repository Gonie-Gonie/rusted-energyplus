//! Pure CP359-to-CP360 Humidistat local humidity-ratio assignment transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRetainedRoute as Route,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingHumidistatMoistureDemandAssignmentSnapshot as Predecessor;

mod predecessor;
pub(in crate::ideal_loads::calc) use predecessor::predecessor_route;

/// Same-call active values needed after the CP359-owned numerator is retained.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentActiveOperands
{
    /// Same-call CP330-owned `SupplyMassFlowRate`.
    pub supply_mass_flow_rate_kg_per_s: f64,
    /// Explicit pre-sampled `Node(ZoneNodeNum).HumRat`; this is not owner proof.
    pub zone_node_humidity_ratio: f64,
}

struct PreparedValues {
    demand: Option<f64>,
    flow: Option<f64>,
    quotient: Option<f64>,
    zone_humidity: Option<f64>,
    calculated: Option<f64>,
}

pub(in crate::ideal_loads::calc) fn advance_cooling_humidistat_supply_humidity_ratio_for_dehumidification_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    active_operands: Option<
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentActiveOperands,
    >,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let values = prepare_values(route, predecessor, active_operands)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    state.transition_count += 1;
    match route {
        Route::UnitOff => state.unit_off_skip_count += 1,
        Route::NonCooling => state.non_cooling_skip_count += 1,
        Route::PositiveGuardFalseFallthrough => {
            state.positive_guard_false_fallthrough_skip_count += 1;
            state.witnessed_positive_guard_false_fallthrough_skip_count += 1;
        }
        Route::DehumidificationControlNoneCaseCompletedSkip => {
            state.dehumidification_control_none_case_completed_skip_count += 1;
            state.witnessed_dehumidification_control_none_case_completed_skip_count += 1;
        }
        Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip => {
            state
                .dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count += 1;
        }
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted => {
            state
                .dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count += 1;
            state.source_site_execution_count +=
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER.len();
            state.zone_dehumidifying_setpoint_moisture_demand_read_count += 1;
            state.supply_mass_flow_rate_read_count += 1;
            state.moisture_demand_derived_supply_humidity_ratio_calculation_count += 1;
            state.zone_node_humidity_ratio_read_count += 1;
            state.supply_humidity_ratio_for_dehumidification_calculation_count += 1;
            state.supply_humidity_ratio_for_dehumidification_assignment_count += 1;
            state
                .witnessed_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count += 1;
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => {
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count +=
                1;
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count += 1;
        }
    }

    let active = route
        == Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted;
    let snapshot = Snapshot {
        source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order:
            PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.predecessor_cooling_body_entered,
        predecessor_no_outdoor_air_fallback_entered: predecessor
            .predecessor_no_outdoor_air_fallback_entered,
        predecessor_positive_supply_mass_flow_body_entered: predecessor
            .predecessor_positive_supply_mass_flow_body_entered,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_none_case_completed_skip: predecessor
            .dehumidification_control_none_case_completed_skip,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            predecessor.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip,
        predecessor_dehumidification_control_humidistat_moisture_demand_assignment_executed:
            predecessor
                .dehumidification_control_humidistat_moisture_demand_assignment_executed,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip:
            predecessor.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip,
        predecessor_resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s: predecessor
            .resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s,
        dehumidification_control_none_case_completed_skip: route
            == Route::DehumidificationControlNoneCaseCompletedSkip,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip: route
            == Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip,
        dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_executed:
            active,
        dehumidification_control_constant_supply_humidity_ratio_case_selected_skip: route
            == Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip,
        zone_dehumidifying_setpoint_moisture_demand_read: active,
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s: values.demand,
        supply_mass_flow_rate_read: active,
        supply_mass_flow_rate_kg_per_s: values.flow,
        moisture_demand_derived_supply_humidity_ratio_calculated: active,
        moisture_demand_derived_supply_humidity_ratio: values.quotient,
        zone_node_humidity_ratio_read: active,
        zone_node_humidity_ratio: values.zone_humidity,
        supply_humidity_ratio_for_dehumidification_calculated: active,
        calculated_supply_humidity_ratio_for_dehumidification: values.calculated,
        supply_humidity_ratio_for_dehumidification_assigned: active,
        assigned_supply_humidity_ratio_for_dehumidification: values.calculated,
        resulting_supply_humidity_ratio_for_dehumidification: values.calculated,
    };
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(state.transition_count);
    Some(snapshot)
}

pub(in crate::ideal_loads::calc) fn next_transition_fits(state: &State, route: Route) -> bool {
    if state.transition_count.checked_add(1).is_none() {
        return false;
    }
    match route {
        Route::UnitOff => state.unit_off_skip_count.checked_add(1).is_some(),
        Route::NonCooling => state.non_cooling_skip_count.checked_add(1).is_some(),
        Route::PositiveGuardFalseFallthrough => checked_pair(
            state.positive_guard_false_fallthrough_skip_count,
            state.witnessed_positive_guard_false_fallthrough_skip_count,
        ),
        Route::DehumidificationControlNoneCaseCompletedSkip => checked_pair(
            state.dehumidification_control_none_case_completed_skip_count,
            state.witnessed_dehumidification_control_none_case_completed_skip_count,
        ),
        Route::DehumidificationControlConstantSensibleHeatRatioCaseCompletedSkip => checked_pair(
            state.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
            state
                .witnessed_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip_count,
        ),
        Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted => {
            checked_pair(
                state.dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count,
                state.witnessed_dehumidification_control_humidistat_supply_humidity_ratio_for_dehumidification_assignment_count,
            ) && state.source_site_execution_count.checked_add(
                PURCHASED_AIR_CALC_COOLING_HUMIDISTAT_SUPPLY_HUMIDITY_RATIO_FOR_DEHUMIDIFICATION_ASSIGNMENT_SOURCE_ORDER.len(),
            ).is_some()
                && all_site_counters_fit(state)
        }
        Route::DehumidificationControlConstantSupplyHumidityRatioCaseSelectedSkip => checked_pair(
            state.dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
            state
                .witnessed_dehumidification_control_constant_supply_humidity_ratio_case_selected_skip_count,
        ),
    }
}

fn prepare_values(
    route: Route,
    predecessor: Predecessor,
    operands: Option<
        PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationAssignmentActiveOperands,
    >,
) -> Option<PreparedValues> {
    if route
        != Route::DehumidificationControlHumidistatSupplyHumidityRatioForDehumidificationAssignmentExecuted
    {
        return operands.is_none().then_some(PreparedValues {
            demand: None,
            flow: None,
            quotient: None,
            zone_humidity: None,
            calculated: None,
        });
    }
    let demand = predecessor.resulting_zone_dehumidifying_setpoint_moisture_demand_kg_per_s?;
    let operands = operands?;
    let quotient = demand / operands.supply_mass_flow_rate_kg_per_s;
    let calculated = quotient + operands.zone_node_humidity_ratio;
    Some(PreparedValues {
        demand: Some(demand),
        flow: Some(operands.supply_mass_flow_rate_kg_per_s),
        quotient: Some(quotient),
        zone_humidity: Some(operands.zone_node_humidity_ratio),
        calculated: Some(calculated),
    })
}

fn all_site_counters_fit(state: &State) -> bool {
    [
        state.zone_dehumidifying_setpoint_moisture_demand_read_count,
        state.supply_mass_flow_rate_read_count,
        state.moisture_demand_derived_supply_humidity_ratio_calculation_count,
        state.zone_node_humidity_ratio_read_count,
        state.supply_humidity_ratio_for_dehumidification_calculation_count,
        state.supply_humidity_ratio_for_dehumidification_assignment_count,
    ]
    .into_iter()
    .all(|count| count.checked_add(1).is_some())
}

fn checked_pair(left: usize, right: usize) -> bool {
    left.checked_add(1).is_some() && right.checked_add(1).is_some()
}
