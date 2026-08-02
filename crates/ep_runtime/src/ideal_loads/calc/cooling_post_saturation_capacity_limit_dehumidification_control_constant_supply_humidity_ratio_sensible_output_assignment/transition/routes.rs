//! Exact CP399 route preservation and compressed CP400 reconstruction.

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioSensibleOutputAssignmentSnapshot as Snapshot,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub active: bool,
}

pub(in crate::ideal_loads::calc) fn predecessor_route(
    predecessor: Predecessor,
) -> Option<RetainedRoute> {
    let route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_cp_air_assignment_snapshot_route(predecessor)?;
    Some(RetainedRoute {
        predecessor_index: route.predecessor_index,
        active: predecessor_index_is_active(route.predecessor_index),
    })
}

pub(in crate::ideal_loads::calc) fn compressed_snapshot_route(
    snapshot: Snapshot,
) -> Option<RetainedRoute> {
    predecessor_route(predecessor_snapshot(snapshot))
}

pub(in crate::ideal_loads::calc) fn predecessor_snapshot(snapshot: Snapshot) -> Predecessor {
    use crate::ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE as SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SUPPLY_HUMIDITY_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER as ORDER,
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
        positive_guard_false_fallthrough_skipped: snapshot
            .positive_guard_false_fallthrough_skipped,
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
        predecessor_cp397_resulting_supply_humidity_ratio: snapshot
            .predecessor_cp397_resulting_supply_humidity_ratio,
        predecessor_cp397_resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp397_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp397_resulting_supply_temperature_c: snapshot
            .predecessor_cp397_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_case_entered: snapshot
            .predecessor_dehumidification_control_none_case_entered,
        predecessor_cp398_resulting_supply_humidity_ratio: snapshot
            .predecessor_cp398_resulting_supply_humidity_ratio,
        predecessor_cp398_resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp398_resulting_supply_enthalpy_j_per_kg,
        predecessor_cp398_resulting_supply_temperature_c: snapshot
            .predecessor_cp398_resulting_supply_temperature_c,
        predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered,
        dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed: snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed,
        mixed_air_humidity_ratio_read: snapshot.predecessor_mixed_air_humidity_ratio_read,
        mixed_air_humidity_ratio: snapshot.predecessor_mixed_air_humidity_ratio,
        psychrometric_cp_air_evaluated: snapshot.predecessor_psychrometric_cp_air_evaluated,
        psychrometric_cp_air_result_j_per_kg_k: snapshot
            .predecessor_psychrometric_cp_air_result_j_per_kg_k,
        cp_air_assigned: snapshot.predecessor_cp_air_assigned,
        cp_air_j_per_kg_k: snapshot.predecessor_cp_air_j_per_kg_k,
        resulting_supply_humidity_ratio: snapshot
            .predecessor_cp399_resulting_supply_humidity_ratio,
        resulting_supply_enthalpy_j_per_kg: snapshot
            .predecessor_cp399_resulting_supply_enthalpy_j_per_kg,
        resulting_supply_temperature_c: snapshot
            .predecessor_cp399_resulting_supply_temperature_c,
    }
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
}

pub(in crate::ideal_loads::calc) const fn predecessor_has_supply_humidity_ratio(
    index: usize,
) -> bool {
    matches!(index, 18 | 19 | 22 | 23 | 26 | 28)
}

pub(in crate::ideal_loads::calc) const fn predecessor_has_supply_enthalpy(index: usize) -> bool {
    matches!(index, 5 | 8 | 11 | 14 | 17..=29)
}

pub(in crate::ideal_loads::calc) const fn predecessor_has_supply_temperature(index: usize) -> bool {
    index >= 3
}
