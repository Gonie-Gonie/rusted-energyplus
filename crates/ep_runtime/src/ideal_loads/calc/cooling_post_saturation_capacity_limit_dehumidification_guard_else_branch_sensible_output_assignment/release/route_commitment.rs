//! Bounded CP420 route-to-snapshot commitment.

use ep_model::DehumidificationControlType as D;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentCommittedRoute as Route,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardElseBranchSensibleOutputAssignmentSnapshot as Snapshot,
};

pub(in crate::ideal_loads::calc) fn cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_committed_route_matches_snapshot(
    snapshot: Snapshot,
    route: Route,
) -> bool {
    super::snapshot::snapshot_shape_is_exact(snapshot)
        && bounded_logical_index(snapshot) == Some(route.logical_index)
        && route.predecessor_guard_false_fallthrough
            == snapshot.saturation_supply_humidity_ratio_guard_false_fallthrough
        && route.predecessor_guard_body_entered
            == snapshot.saturation_supply_humidity_ratio_guard_body_entered
        && route.predecessor_saturation_temperature_assignment_executed
            == snapshot.post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_assignment_executed
        && route.predecessor_saturation_temperature_mixed_air_limit_executed
            == snapshot.post_saturation_capacity_limit_dehumidification_supply_temperature_saturation_mixed_air_limit_executed
        && route.predecessor_supply_humidity_ratio_assignment_executed
            == snapshot.post_saturation_capacity_limit_dehumidification_supply_humidity_ratio_assignment_executed
        && route.predecessor_supply_enthalpy_assignment_executed
            == snapshot.post_saturation_capacity_limit_dehumidification_supply_enthalpy_assignment_executed
        && route.active
            == snapshot.post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment_executed
        && route.active == matches!(route.logical_index, 4 | 7 | 10 | 13 | 16)
}

fn bounded_logical_index(snapshot: Snapshot) -> Option<usize> {
    let predecessor_index = if snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.positive_guard_false_fallthrough_skipped
    {
        0
    } else if !snapshot.unit_off_skipped
        && snapshot.non_cooling_skipped
        && !snapshot.positive_guard_false_fallthrough_skipped
    {
        1
    } else if !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.positive_guard_false_fallthrough_skipped
    {
        2
    } else if !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && !snapshot.positive_guard_false_fallthrough_skipped
    {
        active_predecessor_index(snapshot)?
    } else {
        return None;
    };
    if !control_markers_match_predecessor_index(snapshot, predecessor_index) {
        return None;
    }
    let maximum = snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_maximum_capacity_assignment_executed;
    let guard_false = snapshot
        .predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_capacity_guard_false_fallthrough;
    let split = matches!(predecessor_index, 20 | 21 | 24 | 25 | 27 | 29);
    if (split && guard_false == maximum) || (!split && (guard_false || maximum)) {
        return None;
    }
    Some(logical_index(predecessor_index, maximum))
}

fn active_predecessor_index(snapshot: Snapshot) -> Option<usize> {
    let lineages = [
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ];
    if lineages.into_iter().filter(|value| *value).count() != 1 {
        return None;
    }
    let lineage = lineages.into_iter().position(|value| value)?;
    let stages = [
        snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        snapshot.predecessor_dehumidification_guard_false_fallthrough,
        snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
    ];
    if stages.into_iter().filter(|value| *value).count() != 1 {
        return None;
    }
    let stage = stages.into_iter().position(|value| value)?;
    if stage < 3 {
        return Some(3 + lineage * 3 + stage);
    }
    let selector = snapshot.predecessor_dehumidification_control_type?;
    match (lineage, selector) {
        (0, D::ConstantSensibleHeatRatio) => Some(18),
        (0, D::Humidistat) => Some(19),
        (0, D::None) => Some(20),
        (0, D::ConstantSupplyHumidityRatio) => Some(21),
        (1, D::ConstantSensibleHeatRatio) => Some(22),
        (1, D::Humidistat) => Some(23),
        (1, D::None) => Some(24),
        (1, D::ConstantSupplyHumidityRatio) => Some(25),
        (2, D::Humidistat) => Some(26),
        (3, D::None) => Some(27),
        (4, D::ConstantSensibleHeatRatio) => Some(28),
        (4, D::ConstantSupplyHumidityRatio) => Some(29),
        _ => None,
    }
}

fn control_markers_match_predecessor_index(snapshot: Snapshot, index: usize) -> bool {
    let selected = matches!(index, 18..=29);
    let constant_shr = matches!(index, 18 | 22 | 28);
    let humidistat = matches!(index, 19 | 23 | 26);
    let shared = matches!(index, 20 | 21 | 24 | 25 | 27 | 29);
    let no_active_prefix_markers = index >= 3
        || (!snapshot.heating_availability_guard_false_fallthrough
            && !snapshot.humidification_control_guard_false_fallthrough
            && !snapshot.dehumidification_control_humidistat_maximum_assignment_executed
            && !snapshot.dehumidification_control_none_maximum_assignment_executed
            && !snapshot.dehumidification_control_guard_false_fallthrough
            && !snapshot.predecessor_active_capacity_limit_guard_false_fallthrough
            && !snapshot.predecessor_dehumidification_guard_false_fallthrough
            && !snapshot.dehumidification_total_output_capacity_guard_false_fallthrough
            && !snapshot.dehumidification_total_output_maximum_capacity_assignment_executed);
    no_active_prefix_markers
        && snapshot.predecessor_supply_enthalpy_assignment_executed == selected
        && snapshot.predecessor_dehumidification_control_type_read == selected
        && snapshot.predecessor_dehumidification_control_switch_dispatched == selected
        && snapshot.predecessor_dehumidification_control_type.is_some() == selected
        && snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered
            == constant_shr
        && snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_exited_via_break
            == constant_shr
        && snapshot.predecessor_dehumidification_control_humidistat_case_entered == humidistat
        && snapshot.predecessor_dehumidification_control_humidistat_supply_humidity_ratio_assignment_executed
            == humidistat
        && snapshot.predecessor_dehumidification_control_humidistat_case_exited_via_break
            == humidistat
        && snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_entered
            == shared
        && snapshot.predecessor_dehumidification_control_none_or_constant_supply_humidity_ratio_shared_case_exited_via_break
            == shared
        && !snapshot.predecessor_dehumidification_control_default_case_exited_via_break
}

fn logical_index(predecessor_index: usize, maximum: bool) -> usize {
    let prior_splits = [20, 21, 24, 25, 27, 29]
        .into_iter()
        .filter(|index| *index < predecessor_index)
        .count();
    predecessor_index + prior_splits + usize::from(maximum)
}
