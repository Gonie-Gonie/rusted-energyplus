//! Pure CP405-to-CP406 latent-output capacity-guard else-entry transition.

use super::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_FIRST_EXCLUDED_SOURCE as EXCLUDED,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE as SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_LATENT_OUTPUT_CAPACITY_GUARD_ELSE_BRANCH_ENTRY_SOURCE_ORDER as ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntryRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Snapshot,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot as Predecessor;

mod accounting;
pub(super) mod routes;

use accounting::{increment_counts, next_transition_fits};
pub(in crate::ideal_loads::calc) use routes::RetainedRoute;
#[cfg(test)]
pub(in crate::ideal_loads::calc) use routes::{
    logical_route_index, predecessor_index_is_active, predecessor_index_is_public,
};
use routes::predecessor_route;

pub(in crate::ideal_loads::calc) fn advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entry_state(
    state: &mut State,
    predecessor: Predecessor,
) -> Option<Snapshot> {
    if state.system != predecessor.system {
        return None;
    }
    let route = predecessor_route(predecessor)?;
    if !next_transition_fits(state, route) {
        return None;
    }

    let predecessor_cp405_resulting_supply_humidity_ratio =
        predecessor.resulting_supply_humidity_ratio;
    let predecessor_cp405_resulting_supply_enthalpy_j_per_kg =
        predecessor.resulting_supply_enthalpy_j_per_kg;
    let predecessor_cp405_resulting_supply_temperature_c =
        predecessor.resulting_supply_temperature_c;
    let transition_ordinal = state.transition_count + 1;
    let else_entered = route.guard_evaluated && !route.assignment_executed;
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
            .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_cp405_resulting_supply_humidity_ratio,
        predecessor_cp405_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp405_resulting_supply_temperature_c,
        dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered: else_entered,
        resulting_supply_humidity_ratio: predecessor_cp405_resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: predecessor_cp405_resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: predecessor_cp405_resulting_supply_temperature_c,
    };
    increment_counts(state, route);
    state.latest = Some(snapshot);
    state.latest_route = Some(route);
    state.latest_transition_ordinal = Some(transition_ordinal);
    Some(snapshot)
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn test_increment_counts(
    state: &mut State,
    route: RetainedRoute,
) {
    accounting::increment_counts(state, route);
}

#[cfg(test)]
pub(in crate::ideal_loads::calc) fn test_next_transition_fits(
    state: &State,
    route: RetainedRoute,
) -> bool {
    accounting::next_transition_fits(state, route)
}
