//! Pure CP406-to-CP407 supply-temperature assignment transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_SUPPLY_TEMPERATURE_ASSIGNMENT_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Snapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputSupplyEnthalpyAssignmentSnapshot as EnthalpyOwner,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as HumidityOwner,
};
use crate::psychrometrics::energyplus_psy_tdb_fn_h_w;

mod accounting;
mod owners;
pub(super) mod routes;

use accounting::{increment_counts, next_transition_fits};
use owners::prepare_exact_input;
pub(in crate::ideal_loads::calc) use routes::RetainedRoute;
use routes::predecessor_route;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use routes::{
    logical_route_index, predecessor_index_is_active, predecessor_index_is_public,
};

/// Exact CP378/CP385 owner bundle required only on active CP407 routes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(in crate::ideal_loads::calc) struct PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentActiveOwners
{
    pub supply_humidity_ratio_owner: HumidityOwner,
    pub supply_enthalpy_owner: EnthalpyOwner,
}

use PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentActiveOwners as ActiveOwners;

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_state(
    state: &mut State,
    predecessor: Predecessor,
    active_owners: Option<ActiveOwners>,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    let prepared = prepare_exact_input(predecessor, route, active_owners)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    let evaluated = prepared.active.map(|active| {
        let result = source_assignment(
            active.supply_enthalpy_j_per_kg,
            active.supply_humidity_ratio,
        );
        (active, result)
    });
    let supply_enthalpy_j_per_kg = evaluated.map(|(active, _)| active.supply_enthalpy_j_per_kg);
    let supply_humidity_ratio = evaluated.map(|(active, _)| active.supply_humidity_ratio);
    let psychrometric_supply_temperature_result_c = evaluated.map(|(_, result)| result);
    let assigned_supply_temperature_c = psychrometric_supply_temperature_result_c;
    let resulting_supply_humidity_ratio =
        supply_humidity_ratio.or(prepared.predecessor_supply_humidity_ratio);
    let resulting_supply_enthalpy_j_per_kg =
        supply_enthalpy_j_per_kg.or(prepared.predecessor_supply_enthalpy_j_per_kg);
    let resulting_supply_temperature_c =
        assigned_supply_temperature_c.or(prepared.predecessor_supply_temperature_c);
    let transition_ordinal = state.transition_count + 1;

    let snapshot = Snapshot {
        source: SOURCE,
        first_excluded_source: EXCLUDED,
        source_order: ORDER,
        system: state.system,
        parent_call_ordinal: predecessor.parent_call_ordinal,
        controlled_zone: predecessor.controlled_zone,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: predecessor
            .positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: predecessor
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: predecessor
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: predecessor
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: predecessor
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: predecessor
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: predecessor
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: predecessor
            .predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: predecessor
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: predecessor
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: predecessor
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: predecessor
            .predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: predecessor
            .predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: predecessor
            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: predecessor
            .dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: predecessor
            .dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: predecessor
            .predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: predecessor
            .predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: predecessor
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: predecessor
            .predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: predecessor
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: predecessor
            .predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: predecessor
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: predecessor
            .predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_dehumidification_control_none_case_entered: predecessor
            .predecessor_dehumidification_control_none_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: predecessor
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered: predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered,
        predecessor_cp406_resulting_supply_humidity_ratio: prepared
            .predecessor_supply_humidity_ratio,
        predecessor_cp406_resulting_supply_enthalpy_j_per_kg: prepared
            .predecessor_supply_enthalpy_j_per_kg,
        predecessor_cp406_resulting_supply_temperature_c: prepared
            .predecessor_supply_temperature_c,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_executed: route.assignment_executed,
        cp385_retained_supply_enthalpy_owned_read: route.assignment_executed,
        cp406_same_call_supply_enthalpy_bit_corroborated: route.assignment_executed,
        supply_enthalpy_for_dry_bulb_inversion_read: route.assignment_executed,
        supply_enthalpy_j_per_kg,
        cp378_retained_supply_humidity_ratio_owned_read: route.assignment_executed,
        supply_humidity_ratio_for_dry_bulb_inversion_read: route.assignment_executed,
        supply_humidity_ratio,
        cp406_retained_supply_temperature_state_owned: prepared
            .predecessor_supply_temperature_c
            .is_some(),
        preexisting_supply_temperature_c: prepared.predecessor_supply_temperature_c,
        psychrometric_supply_temperature_evaluated: route.assignment_executed,
        psychrometric_supply_temperature_result_c,
        supply_temperature_assigned: route.assignment_executed,
        assigned_supply_temperature_c,
        resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c,
    };
    increment_counts(state, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(transition_ordinal);
    Some(snapshot)
}

/// Evaluates the pinned EnergyPlus `PsyTdbFnHW` expression without extra gates.
pub(in crate::ideal_loads::calc) fn source_assignment(
    supply_enthalpy_j_per_kg: f64,
    supply_humidity_ratio: f64,
) -> f64 {
    energyplus_psy_tdb_fn_h_w(supply_enthalpy_j_per_kg, supply_humidity_ratio)
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn test_increment_counts(state: &mut State, route: RetainedRoute) {
    accounting::increment_counts(state, route);
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn test_next_transition_fits(
    state: &State,
    route: RetainedRoute,
) -> bool {
    accounting::next_transition_fits(state, route)
}
