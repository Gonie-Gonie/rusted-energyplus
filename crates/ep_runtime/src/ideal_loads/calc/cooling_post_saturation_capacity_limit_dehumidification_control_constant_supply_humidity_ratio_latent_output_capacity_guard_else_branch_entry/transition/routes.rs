//! Exact CP405 route preservation and compact CP406 route reconstruction.

use ep_model::DehumidificationControlType as D;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Snapshot,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputMaximumCapacityAssignmentSnapshot as Predecessor,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::ideal_loads::calc) struct RetainedRoute {
    pub predecessor_index: usize,
    pub guard_evaluated: bool,
    pub assignment_executed: bool,
}

pub(super) fn predecessor_route(predecessor: Predecessor) -> Option<RetainedRoute> {
    let route = crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_snapshot_route(predecessor)?;
    Some(RetainedRoute {
        predecessor_index: route.predecessor_index,
        guard_evaluated: route.guard_evaluated,
        assignment_executed: route.assignment_executed,
    })
}

pub(in crate::ideal_loads::calc) fn compressed_snapshot_route(
    snapshot: Snapshot,
) -> Option<RetainedRoute> {
    if !control_shape_is_exact(snapshot) {
        return None;
    }
    let predecessor_index = base_predecessor_index(snapshot)?;
    let active = predecessor_index_is_active(predecessor_index);
    let guard_false = snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough;
    let assignment = snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;
    let else_entered = snapshot
        .dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_else_branch_entered;
    if active {
        if guard_false == assignment || else_entered != guard_false {
            return None;
        }
    } else if guard_false || assignment || else_entered {
        return None;
    }
    Some(RetainedRoute {
        predecessor_index,
        guard_evaluated: active,
        assignment_executed: assignment,
    })
}

fn base_predecessor_index(snapshot: Snapshot) -> Option<usize> {
    if snapshot.unit_off_skipped {
        return inactive_route(snapshot, 0);
    }
    if snapshot.non_cooling_skipped {
        return inactive_route(snapshot, 1);
    }
    if snapshot.positive_guard_false_fallthrough_skipped {
        return inactive_route(snapshot, 2);
    }
    let lineage = if snapshot.heating_availability_guard_false_fallthrough {
        0
    } else if snapshot.humidification_control_guard_false_fallthrough {
        1
    } else if snapshot.dehumidification_control_humidistat_maximum_assignment_executed {
        2
    } else if snapshot.dehumidification_control_none_maximum_assignment_executed {
        3
    } else if snapshot.dehumidification_control_guard_false_fallthrough {
        4
    } else {
        return None;
    };
    let stage = if snapshot.predecessor_active_capacity_limit_guard_false_fallthrough {
        0
    } else if snapshot.predecessor_dehumidification_guard_false_fallthrough {
        1
    } else if snapshot.dehumidification_total_output_capacity_guard_false_fallthrough {
        2
    } else if snapshot.dehumidification_total_output_maximum_capacity_assignment_executed {
        3
    } else {
        return None;
    };
    if stage < 3 {
        return inactive_route(snapshot, 3 + lineage * 3 + stage);
    }
    if !snapshot.predecessor_supply_enthalpy_assignment_executed
        || !snapshot.predecessor_dehumidification_control_type_read
        || !snapshot.predecessor_dehumidification_control_switch_dispatched
    {
        return None;
    }
    let selector = snapshot.predecessor_dehumidification_control_type?;
    let index = match (lineage, selector) {
        (0, D::ConstantSensibleHeatRatio) => 18,
        (0, D::Humidistat) => 19,
        (0, D::None) => 20,
        (0, D::ConstantSupplyHumidityRatio) => 21,
        (1, D::ConstantSensibleHeatRatio) => 22,
        (1, D::Humidistat) => 23,
        (1, D::None) => 24,
        (1, D::ConstantSupplyHumidityRatio) => 25,
        (2, D::Humidistat) => 26,
        (3, D::None) => 27,
        (4, D::ConstantSensibleHeatRatio) => 28,
        (4, D::ConstantSupplyHumidityRatio) => 29,
        _ => return None,
    };
    local_case_shape_is_exact(snapshot, index).then_some(index)
}

fn inactive_route(snapshot: Snapshot, index: usize) -> Option<usize> {
    if snapshot.predecessor_supply_enthalpy_assignment_executed
        != matches!(index, 18..=29)
        || snapshot.predecessor_dehumidification_control_type_read
            != matches!(index, 18..=29)
        || snapshot.predecessor_dehumidification_control_switch_dispatched
            != matches!(index, 18..=29)
        || snapshot.predecessor_dehumidification_control_type.is_some()
            != matches!(index, 18..=29)
        || !local_case_shape_is_exact(snapshot, index)
    {
        return None;
    }
    Some(index)
}

fn local_case_shape_is_exact(snapshot: Snapshot, index: usize) -> bool {
    let constant_shr = matches!(index, 18 | 22 | 28);
    let humidistat = matches!(index, 19 | 23 | 26);
    let none_case = matches!(index, 20 | 24 | 27);
    let shared = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
    snapshot
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
        == constant_shr
        && snapshot
            .predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break
            == constant_shr
        && snapshot.predecessor_dehumidification_control_humidistat_case_entered == humidistat
        && snapshot
            .predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed
            == humidistat
        && snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break
            == humidistat
        && snapshot.predecessor_dehumidification_control_none_case_entered == none_case
        && snapshot
            .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered
            == shared
}

fn control_shape_is_exact(snapshot: Snapshot) -> bool {
    crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment::cooling_post_saturation_capacity_limit_dehumidification_total_output_maximum_capacity_assignment_control_flow_shape_is_exact(
        control_shape(snapshot),
    )
}

fn control_shape(snapshot: Snapshot) -> crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot {
    use crate::ideal_loads::{
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_FIRST_EXCLUDED_SOURCE as EXCLUDED,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE as SOURCE,
        PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_TOTAL_OUTPUT_MAXIMUM_CAPACITY_ASSIGNMENT_SOURCE_ORDER as ORDER,
        PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputMaximumCapacityAssignmentSnapshot as Control,
    };
    Control {
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
        predecessor_capacity_limit_body_entered: snapshot
            .predecessor_capacity_limit_body_entered,
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
        preexisting_cooling_total_output_w: None,
        cp383_retained_maximum_total_cooling_capacity_owned_read: false,
        maximum_total_cooling_capacity_read: false,
        maximum_total_cooling_capacity_w: None,
        cooling_total_output_assigned: false,
        assigned_cooling_total_output_w: None,
        resulting_cooling_total_output_w: None,
    }
}

/// Stable flattened ordering of the 36 CP405/CP406 logical routes.
#[cfg(test)]
pub(in crate::ideal_loads::calc) const fn logical_route_index(route: RetainedRoute) -> usize {
    let mut extra = 0;
    let mut index = 0;
    while index < route.predecessor_index {
        if predecessor_index_is_active(index) {
            extra += 1;
        }
        index += 1;
    }
    route.predecessor_index + extra + if route.assignment_executed { 1 } else { 0 }
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_active(index: usize) -> bool {
    matches!(index, 20 | 21 | 24 | 25 | 27 | 29)
}

pub(in crate::ideal_loads::calc) const fn predecessor_index_is_public(index: usize) -> bool {
    matches!(index, 0..=8 | 20 | 24)
}

pub(in crate::ideal_loads::calc) const fn predecessor_has_supply_humidity_ratio(
    route: RetainedRoute,
) -> bool {
    route.assignment_executed
        || matches!(route.predecessor_index, 18 | 19 | 22 | 23 | 26 | 28)
}

pub(in crate::ideal_loads::calc) const fn predecessor_has_supply_enthalpy(index: usize) -> bool {
    matches!(index, 5 | 8 | 11 | 14 | 17..=29)
}

pub(in crate::ideal_loads::calc) const fn predecessor_has_supply_temperature(index: usize) -> bool {
    index >= 3
}
