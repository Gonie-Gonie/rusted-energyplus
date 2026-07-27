//! Exact retained CP310-through-CP313 release-prefix validation.

use ep_model::IdealLoadsAirSystem;

use super::predecessor_validation::cooling_gate_snapshot_is_exact_direct_release;
use crate::ideal_loads::{
    IdealLoadsSensibleMode, PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER, PURCHASED_AIR_CALC_ENTRY_SOURCE,
    PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER, PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE,
    PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE, PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER,
    PurchasedAirAvailabilityStatus, PurchasedAirCalcCoolingEntryGateRuntimeState,
    PurchasedAirCalcCoolingEntryGateSnapshot, PurchasedAirCalcEntryRuntimeState,
    PurchasedAirCalcEntrySnapshot, PurchasedAirCalcMinimumOaPrefixRuntimeState,
    PurchasedAirCalcMinimumOaPrefixSnapshot, PurchasedAirTemperatureControlType,
    PurchasedAirUnitRuntimeState,
};
use crate::zone_equipment::ZoneSensibleDemandInputKind;

pub(super) fn completed_cp310_through_cp313_prefix_is_consistent(
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
) -> bool {
    let (Some(entry), Some(minimum_oa), Some(cooling_entry), Some(cooling_gate)) = (
        unit.calc_entry.latest,
        unit.calc_minimum_oa_prefix.latest,
        unit.calc_cooling_entry_gate.latest,
        unit.calc_cooling_oa_max_flow_gate.latest,
    ) else {
        return false;
    };
    let count = unit.calc_entry.call_count;
    count > 0
        && entry.call_ordinal == count
        && minimum_oa.parent_call_ordinal == count
        && cooling_entry.parent_call_ordinal == count
        && cooling_gate.parent_call_ordinal == count
        && entry.system == unit.system
        && minimum_oa.system == unit.system
        && cooling_entry.system == unit.system
        && cooling_gate.system == unit.system
        && unit.controlled_zone == Some(entry.controlled_zone)
        && entry.controlled_zone == minimum_oa.controlled_zone
        && minimum_oa.controlled_zone == cooling_entry.controlled_zone
        && cooling_entry.controlled_zone == cooling_gate.controlled_zone
        && entry_snapshot_is_exact_direct_release(unit, entry)
        && minimum_oa_snapshot_is_exact_direct_release(minimum_oa)
        && cooling_entry_snapshot_is_exact_direct_release(cooling_entry, entry)
        && cooling_gate_snapshot_is_exact_direct_release(
            cooling_gate,
            system.cooling_limit,
            unit.maximum_cooling_air_mass_flow_rate_kg_per_s,
        )
        && entry_links_to_minimum_oa(entry, minimum_oa)
        && minimum_oa_links_to_cooling_entry(entry, minimum_oa, cooling_entry)
        && cooling_entry_links_to_gate(cooling_entry, cooling_gate)
        && entry_history_is_consistent(&unit.calc_entry)
        && minimum_oa_history_is_consistent(&unit.calc_minimum_oa_prefix, &unit.calc_entry)
        && cooling_entry_history_is_consistent(&unit.calc_cooling_entry_gate, &unit.calc_entry)
}

fn entry_snapshot_is_exact_direct_release(
    unit: &PurchasedAirUnitRuntimeState,
    snapshot: PurchasedAirCalcEntrySnapshot,
) -> bool {
    let sampled_values_are_finite = [
        snapshot.demand.remaining_output_req_to_heat_sp_w,
        snapshot.demand.remaining_output_req_to_cool_sp_w,
        snapshot.overall_availability,
        snapshot.heating_availability,
        snapshot.cooling_availability,
    ]
    .into_iter()
    .all(f64::is_finite);
    snapshot.source == PURCHASED_AIR_CALC_ENTRY_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_ENTRY_SOURCE_ORDER
        && unit.supply_node == Some(snapshot.supply_node)
        && unit.recirculation_node == Some(snapshot.recirculation_node)
        && snapshot.outdoor_air_node.is_none()
        && snapshot.demand.zone == snapshot.controlled_zone
        && snapshot.demand.sensible_input_kind
            == ZoneSensibleDemandInputKind::SourceSetpointThresholds
        && snapshot.reset.all_zero()
        && snapshot.unit_defaulted_on
        && !snapshot.economizer_defaulted_on
        && snapshot.availability_manager_read_site_visited
        && snapshot.availability_manager_zone_written
        && snapshot.copied_availability_status == Some(PurchasedAirAvailabilityStatus::NoAction)
        && !snapshot.force_off_applied
        && snapshot.overall_availability_read_site_visited
        && snapshot.heating_availability_read_site_visited
        && snapshot.cooling_availability_read_site_visited
        && sampled_values_are_finite
        && snapshot.heating_availability.to_bits() == 1.0_f64.to_bits()
        && snapshot.cooling_availability.to_bits() == 1.0_f64.to_bits()
        && snapshot.unit_on == (snapshot.overall_availability > 0.0)
        && snapshot.heating_on
        && snapshot.cooling_on
        && snapshot.unit_body_entered == snapshot.unit_on
}

fn minimum_oa_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcMinimumOaPrefixSnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE
        && snapshot.minimum_oa_child_source == PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER;
    if !provenance || snapshot.ems_override_applied || snapshot.psychrometric_call_count != 0 {
        return false;
    }
    if snapshot.unit_body_entered {
        snapshot.zone_heat_balance_reference_bound
            && snapshot.minimum_oa_child_called
            && snapshot.minimum_oa_child_no_outdoor_air_route
            && option_f64_has_bits(
                snapshot.retained_minimum_outdoor_air_mass_flow_rate_kg_per_s,
                0.0,
            )
            && snapshot.retained_minimum_outdoor_air_write_performed
            && snapshot.ems_override_flag_read
            && snapshot.ems_override_enabled == Some(false)
            && option_f64_has_bits(snapshot.working_outdoor_air_mass_flow_rate_kg_per_s, 0.0)
            && snapshot.outdoor_air_flag_read
            && snapshot.outdoor_air_enabled == Some(false)
            && snapshot.no_outdoor_air_zero_branch_entered
            && option_f64_has_bits(snapshot.minimum_outdoor_air_sensible_output_w, 0.0)
            && option_f64_has_bits(snapshot.minimum_outdoor_air_moisture_output_kg_per_s, 0.0)
    } else {
        !snapshot.zone_heat_balance_reference_bound
            && !snapshot.minimum_oa_child_called
            && !snapshot.minimum_oa_child_no_outdoor_air_route
            && snapshot
                .retained_minimum_outdoor_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.retained_minimum_outdoor_air_write_performed
            && !snapshot.ems_override_flag_read
            && snapshot.ems_override_enabled.is_none()
            && snapshot
                .working_outdoor_air_mass_flow_rate_kg_per_s
                .is_none()
            && !snapshot.outdoor_air_flag_read
            && snapshot.outdoor_air_enabled.is_none()
            && !snapshot.no_outdoor_air_zero_branch_entered
            && snapshot.minimum_outdoor_air_sensible_output_w.is_none()
            && snapshot
                .minimum_outdoor_air_moisture_output_kg_per_s
                .is_none()
    }
}

fn cooling_entry_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingEntryGateSnapshot,
    entry: PurchasedAirCalcEntrySnapshot,
) -> bool {
    let provenance = snapshot.source == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER;
    if !provenance {
        return false;
    }
    if !snapshot.unit_body_entered {
        return snapshot.minimum_outdoor_air_sensible_output_w.is_none()
            && snapshot.cooling_setpoint_demand_w.is_none()
            && !snapshot.sensible_comparison_evaluated
            && snapshot.sensible_comparison_satisfied.is_none()
            && !snapshot.temperature_control_type_read
            && snapshot.temperature_control_type.is_none()
            && snapshot.temperature_control_type_permits_cooling.is_none()
            && !snapshot.single_heat_blocked
            && !snapshot.cooling_body_entered
            && snapshot.assigned_operating_mode.is_none();
    }
    let cooling_demand = entry.demand.remaining_output_req_to_cool_sp_w;
    if !cooling_demand.is_finite()
        || !option_f64_has_bits(snapshot.minimum_outdoor_air_sensible_output_w, 0.0)
        || !option_f64_has_bits(snapshot.cooling_setpoint_demand_w, cooling_demand)
        || !snapshot.sensible_comparison_evaluated
    {
        return false;
    }
    let admitted = 0.0 >= cooling_demand;
    snapshot.sensible_comparison_satisfied == Some(admitted)
        && snapshot.temperature_control_type_read == admitted
        && snapshot.temperature_control_type
            == admitted.then_some(PurchasedAirTemperatureControlType::DualHeatCool)
        && snapshot.temperature_control_type_permits_cooling == admitted.then_some(true)
        && !snapshot.single_heat_blocked
        && snapshot.cooling_body_entered == admitted
        && snapshot.assigned_operating_mode == admitted.then_some(IdealLoadsSensibleMode::Cooling)
}

fn entry_links_to_minimum_oa(
    entry: PurchasedAirCalcEntrySnapshot,
    minimum_oa: PurchasedAirCalcMinimumOaPrefixSnapshot,
) -> bool {
    minimum_oa.system == entry.system
        && minimum_oa.parent_call_ordinal == entry.call_ordinal
        && minimum_oa.controlled_zone == entry.controlled_zone
        && minimum_oa.unit_body_entered == entry.unit_body_entered
}

fn minimum_oa_links_to_cooling_entry(
    entry: PurchasedAirCalcEntrySnapshot,
    minimum_oa: PurchasedAirCalcMinimumOaPrefixSnapshot,
    cooling_entry: PurchasedAirCalcCoolingEntryGateSnapshot,
) -> bool {
    cooling_entry.system == minimum_oa.system
        && cooling_entry.parent_call_ordinal == minimum_oa.parent_call_ordinal
        && cooling_entry.controlled_zone == minimum_oa.controlled_zone
        && cooling_entry.unit_body_entered == minimum_oa.unit_body_entered
        && option_f64_bits_equal(
            cooling_entry.minimum_outdoor_air_sensible_output_w,
            minimum_oa.minimum_outdoor_air_sensible_output_w,
        )
        && if entry.unit_body_entered {
            option_f64_has_bits(
                cooling_entry.cooling_setpoint_demand_w,
                entry.demand.remaining_output_req_to_cool_sp_w,
            )
        } else {
            cooling_entry.cooling_setpoint_demand_w.is_none()
        }
}

fn cooling_entry_links_to_gate(
    cooling_entry: PurchasedAirCalcCoolingEntryGateSnapshot,
    gate: crate::ideal_loads::PurchasedAirCalcCoolingOaMaxFlowGateSnapshot,
) -> bool {
    gate.system == cooling_entry.system
        && gate.parent_call_ordinal == cooling_entry.parent_call_ordinal
        && gate.controlled_zone == cooling_entry.controlled_zone
        && gate.unit_body_entered == cooling_entry.unit_body_entered
        && gate.predecessor_cooling_body_entered == cooling_entry.cooling_body_entered
        && gate.unit_off_skipped != cooling_entry.unit_body_entered
        && gate.non_cooling_skipped
            == (cooling_entry.unit_body_entered && !cooling_entry.cooling_body_entered)
}

fn entry_history_is_consistent(state: &PurchasedAirCalcEntryRuntimeState) -> bool {
    state.reset_count == state.call_count
        && state.demand_read_count == state.call_count
        && state.availability_manager_read_count == state.call_count
        && state.availability_manager_zone_write_count == state.call_count
        && state.availability_status_copy_count == state.call_count
        && state.overall_availability_read_count == state.call_count
        && state.heating_availability_read_count == state.call_count
        && state.cooling_availability_read_count == state.call_count
        && state.force_off_count == 0
        && state.overall_schedule_off_count == state.unit_off_count
        && state
            .unit_body_entry_count
            .checked_add(state.unit_off_count)
            == Some(state.call_count)
        && state.heating_on_count == state.call_count
        && state.cooling_on_count == state.call_count
        && state.availability_manager_zone == state.latest.map(|snapshot| snapshot.controlled_zone)
        && state.availability_status == PurchasedAirAvailabilityStatus::NoAction
        && state.minimum_outdoor_air_mass_flow_rate_kg_per_s.to_bits() == 0.0_f64.to_bits()
        && state.economizer_active_time_hours.to_bits() == 0.0_f64.to_bits()
        && state.heat_recovery_active_time_hours.to_bits() == 0.0_f64.to_bits()
}

fn minimum_oa_history_is_consistent(
    state: &PurchasedAirCalcMinimumOaPrefixRuntimeState,
    entry: &PurchasedAirCalcEntryRuntimeState,
) -> bool {
    state
        .source_execution_count
        .checked_add(state.unit_off_skip_count)
        == Some(state.transition_count)
        && state.source_execution_count == entry.unit_body_entry_count
        && state.unit_off_skip_count == entry.unit_off_count
        && state.zone_heat_balance_reference_count == state.source_execution_count
        && state.minimum_oa_child_call_count == state.source_execution_count
        && state.minimum_oa_child_no_outdoor_air_count == state.source_execution_count
        && state.retained_minimum_outdoor_air_write_count == state.source_execution_count
        && state.ems_override_flag_read_count == state.source_execution_count
        && state.ems_override_apply_count == 0
        && state.outdoor_air_flag_read_count == state.source_execution_count
        && state.outdoor_air_effect_count == 0
        && state.no_outdoor_air_zero_branch_count == state.source_execution_count
        && state.psychrometric_call_count == 0
}

fn cooling_entry_history_is_consistent(
    state: &PurchasedAirCalcCoolingEntryGateRuntimeState,
    entry: &PurchasedAirCalcEntryRuntimeState,
) -> bool {
    state
        .source_execution_count
        .checked_add(state.unit_off_skip_count)
        == Some(state.transition_count)
        && state.source_execution_count == entry.unit_body_entry_count
        && state.unit_off_skip_count == entry.unit_off_count
        && state.sensible_comparison_count == state.source_execution_count
        && state.sensible_comparison_satisfied_count == state.temperature_control_type_read_count
        && state.temperature_control_type_read_count == state.cooling_body_entry_count
        && state.single_heat_block_count == 0
        && state.cooling_body_entry_count == state.operating_mode_assignment_count
        && state
            .cooling_body_entry_count
            .checked_add(state.active_fallthrough_count)
            == Some(state.source_execution_count)
}

fn option_f64_has_bits(value: Option<f64>, expected: f64) -> bool {
    value.is_some_and(|value| value.to_bits() == expected.to_bits())
}

fn option_f64_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
