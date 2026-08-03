//! Exact CP409 route preservation and compressed CP410 route reconstruction.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCaseBreakSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlDefaultCaseBreakSnapshot as Snapshot,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub predecessor_guard_false_fallthrough: bool,
    pub predecessor_maximum_capacity_assignment_executed: bool,
    pub predecessor_shared_case_break_executed: bool,
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<RetainedRoute> {
    let route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_case_break_snapshot_route(predecessor)?;
    Some(RetainedRoute {
        predecessor_index: route.predecessor_index,
        predecessor_guard_false_fallthrough: route.predecessor_guard_false_fallthrough,
        predecessor_maximum_capacity_assignment_executed: route
            .predecessor_maximum_capacity_assignment_executed,
        predecessor_shared_case_break_executed: route.active,
    })
}

pub(in crate::ideal_loads::calc) fn compressed_snapshot_route(
    snapshot: Snapshot,
) -> Option<RetainedRoute> {
    if snapshot.dehumidification_control_default_case_exited_via_break {
        return None;
    }
    let predecessor = cp409_shape(snapshot);
    let route = predecessor_route(predecessor)?;
    if snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break
        != route.predecessor_shared_case_break_executed
    {
        return None;
    }
    Some(route)
}

pub(in crate::ideal_loads::calc) fn cp409_shape(snapshot: Snapshot) -> Predecessor {
    use crate::ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_FIRST_EXCLUDED_SOURCE as EXCLUDED,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE as SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CASE_BREAK_SOURCE_ORDER as ORDER,
    };
    Predecessor {
        source: SOURCE,
        first_excluded_source: EXCLUDED,
        source_order: ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot
            .heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot
            .humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot
            .dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot
            .dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot
            .dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot
            .predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot
            .predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot
            .predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot
            .predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot
            .predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot
            .predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot
            .predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot
            .predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot
            .predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot
            .dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot
            .dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: snapshot
            .predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: snapshot
            .predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: snapshot
            .predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: snapshot
            .predecessor_dehumidification_control_switch_dispatched,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered: snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break: snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break,
        predecessor_dehumidification_control_humidistat_case_entered: snapshot
            .predecessor_dehumidification_control_humidistat_case_entered,
        predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed: snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed,
        predecessor_dehumidification_control_humidistat_case_exited_via_break: snapshot
            .predecessor_dehumidification_control_humidistat_case_exited_via_break,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed,
        predecessor_cp408_resulting_supply_humidity_ratio: snapshot
            .predecessor_cp409_resulting_supply_humidity_ratio,
        predecessor_cp408_resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp408_resulting_supply_temperature_c: snapshot
            .predecessor_cp409_resulting_supply_temperature_c,
        dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break,
        resulting_supply_humidity_ratio: snapshot.predecessor_cp409_resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp409_resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: snapshot.predecessor_cp409_resulting_supply_temperature_c,
    }
}

/// Stable flattened ordering of the 36 CP409/CP410 logical routes.
#[cfg(test)]
pub(in crate::ideal_loads::calc) const fn logical_route_index(route: RetainedRoute) -> usize {
    let mut extra = 0;
    let mut index = 0;
    while index < route.predecessor_index {
        if predecessor_index_is_split(index) {
            extra += 1;
        }
        index += 1;
    }
    route.predecessor_index
        + extra
        + if route.predecessor_maximum_capacity_assignment_executed {
            1
        } else {
            0
        }
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_split(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_public(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 24)
}
