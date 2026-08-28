//! Run-summary evidence for the bounded PurchasedAir cooling-entry gate.

use ep_runtime::{
    IdealLoadsSensibleMode, PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
    PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER,
    PurchasedAirCalcCoolingEntryGateLifecycleSummary, PurchasedAirCalcCoolingEntryGateSnapshot,
    PurchasedAirCalcEntryLifecycleSummary, PurchasedAirCalcMinimumOaPrefixLifecycleSummary,
    PurchasedAirInitLifecycleSummary, PurchasedAirTemperatureControlType,
};
use serde_json::{Value, json};

pub(super) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingEntryGateLifecycleSummary,
) -> Value {
    let latest = lifecycle.state.latest.map(snapshot_json);
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": lifecycle.state.system.0,
        "transition_count": lifecycle.state.transition_count,
        "source_execution_count": lifecycle.state.source_execution_count,
        "unit_off_skip_count": lifecycle.state.unit_off_skip_count,
        "sensible_comparison_count": lifecycle.state.sensible_comparison_count,
        "sensible_comparison_satisfied_count":
            lifecycle.state.sensible_comparison_satisfied_count,
        "temperature_control_type_read_count":
            lifecycle.state.temperature_control_type_read_count,
        "single_heat_block_count": lifecycle.state.single_heat_block_count,
        "cooling_body_entry_count": lifecycle.state.cooling_body_entry_count,
        "operating_mode_assignment_count": lifecycle.state.operating_mode_assignment_count,
        "active_fallthrough_count": lifecycle.state.active_fallthrough_count,
        "latest": latest,
    })
}

pub(super) fn validate_direct_lifecycle(
    lifecycle: Option<&PurchasedAirCalcCoolingEntryGateLifecycleSummary>,
    minimum_oa_lifecycle: Option<&PurchasedAirCalcMinimumOaPrefixLifecycleSummary>,
    calculation_entry_lifecycle: Option<&PurchasedAirCalcEntryLifecycleSummary>,
    init_lifecycle: Option<&PurchasedAirInitLifecycleSummary>,
    coupling_call_count: Option<usize>,
) -> Result<(), String> {
    let lifecycle = lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads runtime did not expose cooling-entry gate evidence".to_string()
    })?;
    let minimum_oa_lifecycle = minimum_oa_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling-entry gate has no minimum-OA evidence".to_string()
    })?;
    let calculation_entry_lifecycle = calculation_entry_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling-entry gate has no Calc-entry evidence".to_string()
    })?;
    let init_lifecycle = init_lifecycle.ok_or_else(|| {
        "direct-zone IdealLoads cooling-entry gate has no initialization evidence".to_string()
    })?;
    let coupling_call_count = coupling_call_count.ok_or_else(|| {
        "direct-zone IdealLoads cooling-entry gate has no coupling call count".to_string()
    })?;
    let state = &lifecycle.state;
    let minimum_oa_state = &minimum_oa_lifecycle.state;
    let entry_state = &calculation_entry_lifecycle.state;
    let source_skip_partition = checked_partition(
        state.source_execution_count,
        state.unit_off_skip_count,
        "source/skip",
    )?;
    let cooling_fallthrough_partition = checked_partition(
        state.cooling_body_entry_count,
        state.active_fallthrough_count,
        "cooling/fallthrough",
    )?;
    if coupling_call_count == 0
        || lifecycle.source != PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
        || state.transition_count != coupling_call_count
    {
        return Err(
            "direct-zone IdealLoads cooling-entry gate provenance or call count is invalid"
                .to_string(),
        );
    }
    for (field, expected, actual) in [
        (
            "minimum_oa_transition_count",
            minimum_oa_state.transition_count,
            state.transition_count,
        ),
        (
            "calculation_entry_call_count",
            entry_state.call_count,
            state.transition_count,
        ),
        (
            "source_execution_count",
            minimum_oa_state.source_execution_count,
            state.source_execution_count,
        ),
        (
            "unit_off_skip_count",
            minimum_oa_state.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "sensible_comparison_count",
            state.source_execution_count,
            state.sensible_comparison_count,
        ),
        (
            "satisfied_comparison_read_count",
            state.sensible_comparison_satisfied_count,
            state.temperature_control_type_read_count,
        ),
        (
            "thermostat_read_cooling_entry_count",
            state.temperature_control_type_read_count,
            state.cooling_body_entry_count,
        ),
        ("single_heat_block_count", 0, state.single_heat_block_count),
        (
            "operating_mode_assignment_count",
            state.cooling_body_entry_count,
            state.operating_mode_assignment_count,
        ),
        (
            "source_skip_partition",
            coupling_call_count,
            source_skip_partition,
        ),
        (
            "cooling_fallthrough_partition",
            state.source_execution_count,
            cooling_fallthrough_partition,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling-entry invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    let latest = state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling-entry gate has no latest snapshot".to_string()
    })?;
    let latest_minimum_oa = minimum_oa_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling-entry gate has no latest minimum-OA snapshot".to_string()
    })?;
    let latest_entry = entry_state.latest.as_ref().ok_or_else(|| {
        "direct-zone IdealLoads cooling-entry gate has no latest Calc-entry snapshot".to_string()
    })?;
    let expected_system = init_lifecycle
        .declared_system_order
        .first()
        .copied()
        .ok_or_else(|| {
            "direct-zone IdealLoads cooling-entry gate has no declared system".to_string()
        })?;
    if state.system != expected_system
        || minimum_oa_state.system != expected_system
        || entry_state.system != expected_system
        || !latest_matches_release(latest, latest_minimum_oa, latest_entry, coupling_call_count)
    {
        return Err(
            "direct-zone IdealLoads cooling-entry gate latest state is not release-ready"
                .to_string(),
        );
    }
    Ok(())
}

fn snapshot_json(snapshot: PurchasedAirCalcCoolingEntryGateSnapshot) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "source_order": snapshot.source_order,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "minimum_outdoor_air_sensible_output_w":
            snapshot.minimum_outdoor_air_sensible_output_w,
        "cooling_setpoint_demand_w": snapshot.cooling_setpoint_demand_w,
        "sensible_comparison_evaluated": snapshot.sensible_comparison_evaluated,
        "sensible_comparison_satisfied": snapshot.sensible_comparison_satisfied,
        "temperature_control_type_read": snapshot.temperature_control_type_read,
        "temperature_control_type": snapshot
            .temperature_control_type
            .map(temperature_control_type_name),
        "temperature_control_type_permits_cooling":
            snapshot.temperature_control_type_permits_cooling,
        "single_heat_blocked": snapshot.single_heat_blocked,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "assigned_operating_mode": snapshot
            .assigned_operating_mode
            .map(operating_mode_name),
    })
}

fn latest_matches_release(
    gate: &PurchasedAirCalcCoolingEntryGateSnapshot,
    minimum_oa: &ep_runtime::PurchasedAirCalcMinimumOaPrefixSnapshot,
    entry: &ep_runtime::PurchasedAirCalcEntrySnapshot,
    call_count: usize,
) -> bool {
    let common = gate.source == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE
        && gate.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE
        && gate.source_order == PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER
        && gate.system == minimum_oa.system
        && gate.system == entry.system
        && gate.parent_call_ordinal == call_count
        && gate.parent_call_ordinal == minimum_oa.parent_call_ordinal
        && gate.parent_call_ordinal == entry.call_ordinal
        && gate.controlled_zone == minimum_oa.controlled_zone
        && gate.controlled_zone == entry.controlled_zone
        && gate.unit_body_entered == minimum_oa.unit_body_entered
        && gate.unit_body_entered == entry.unit_body_entered
        && !gate.single_heat_blocked;
    if !common {
        return false;
    }
    if gate.unit_body_entered {
        let cooling_demand_w = entry.demand.remaining_output_req_to_cool_sp_w;
        if !cooling_demand_w.is_finite() {
            return false;
        }
        let expected_cooling = 0.0 >= cooling_demand_w;
        gate.minimum_outdoor_air_sensible_output_w == Some(0.0)
            && minimum_oa.minimum_outdoor_air_sensible_output_w == Some(0.0)
            && gate.cooling_setpoint_demand_w == Some(cooling_demand_w)
            && gate.sensible_comparison_evaluated
            && gate.sensible_comparison_satisfied == Some(expected_cooling)
            && gate.temperature_control_type_read == expected_cooling
            && gate.temperature_control_type
                == expected_cooling.then_some(PurchasedAirTemperatureControlType::DualHeatCool)
            && gate.temperature_control_type_permits_cooling == expected_cooling.then_some(true)
            && gate.cooling_body_entered == expected_cooling
            && gate.assigned_operating_mode
                == expected_cooling.then_some(IdealLoadsSensibleMode::Cooling)
    } else {
        gate.minimum_outdoor_air_sensible_output_w.is_none()
            && minimum_oa.minimum_outdoor_air_sensible_output_w.is_none()
            && gate.cooling_setpoint_demand_w.is_none()
            && !gate.sensible_comparison_evaluated
            && gate.sensible_comparison_satisfied.is_none()
            && !gate.temperature_control_type_read
            && gate.temperature_control_type.is_none()
            && gate.temperature_control_type_permits_cooling.is_none()
            && !gate.cooling_body_entered
            && gate.assigned_operating_mode.is_none()
    }
}

fn checked_partition(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right)
        .ok_or_else(|| format!("direct-zone IdealLoads cooling-entry {label} partition overflowed"))
}

pub(in crate::pipeline) fn temperature_control_type_name(
    value: PurchasedAirTemperatureControlType,
) -> &'static str {
    match value {
        PurchasedAirTemperatureControlType::Invalid => "Invalid",
        PurchasedAirTemperatureControlType::Uncontrolled => "Uncontrolled",
        PurchasedAirTemperatureControlType::SingleHeat => "SingleHeat",
        PurchasedAirTemperatureControlType::SingleCool => "SingleCool",
        PurchasedAirTemperatureControlType::SingleHeatCool => "SingleHeatCool",
        PurchasedAirTemperatureControlType::DualHeatCool => "DualHeatCool",
    }
}

pub(in crate::pipeline) fn operating_mode_name(value: IdealLoadsSensibleMode) -> &'static str {
    match value {
        IdealLoadsSensibleMode::Off => "Off",
        IdealLoadsSensibleMode::Deadband => "Deadband",
        IdealLoadsSensibleMode::Cooling => "Cooling",
        IdealLoadsSensibleMode::Heating => "Heating",
    }
}
