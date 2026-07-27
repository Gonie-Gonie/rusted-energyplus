//! Fail-closed validation helpers for CP319 direct-release evidence.

use ep_runtime::PurchasedAirCalcCoolingDehumidificationFlowRuntimeState;

mod snapshot;

pub(super) use snapshot::snapshot_shape;

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
) -> Result<(), String> {
    let cooling_on_partition = checked_add(
        state.cooling_on_body_entry_count,
        state.cooling_on_fallthrough_count,
        "cooling-on partition",
    )?;
    let dehumidification_control_partition = checked_add(
        state.dehumidification_control_type_humidistat_count,
        state.dehumidification_control_type_fallthrough_count,
        "dehumidification selector partition",
    )?;
    let delta_partition = checked_add(
        state.delta_humidity_ratio_comparison_satisfied_count,
        state.delta_humidity_ratio_fallthrough_count,
        "delta-humidity-ratio partition",
    )?;
    let load_partition = checked_add(
        state.zone_dehumidifying_setpoint_moisture_demand_comparison_satisfied_count,
        state.zone_dehumidifying_setpoint_moisture_demand_fallthrough_count,
        "dehumidifying-load partition",
    )?;
    for (field, expected, actual) in [
        (
            "supply_mass_flow_rate_for_dehumidification_reset_assignment_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_for_dehumidification_reset_assignment_count,
        ),
        (
            "cooling_on_read_count",
            state.cooling_body_entry_count,
            state.cooling_on_read_count,
        ),
        (
            "cooling_on_partition",
            state.cooling_on_read_count,
            cooling_on_partition,
        ),
        (
            "direct_cooling_on_body_entry_count",
            state.cooling_body_entry_count,
            state.cooling_on_body_entry_count,
        ),
        (
            "direct_cooling_on_fallthrough_count",
            0,
            state.cooling_on_fallthrough_count,
        ),
        (
            "dehumidification_control_type_read_count",
            state.cooling_on_body_entry_count,
            state.dehumidification_control_type_read_count,
        ),
        (
            "dehumidification_control_partition",
            state.dehumidification_control_type_read_count,
            dehumidification_control_partition,
        ),
        (
            "direct_dehumidification_control_type_humidistat_count",
            0,
            state.dehumidification_control_type_humidistat_count,
        ),
        (
            "dehumidification_control_body_entry_count",
            state.dehumidification_control_type_humidistat_count,
            state.dehumidification_control_body_entry_count,
        ),
        (
            "direct_dehumidification_control_type_fallthrough_count",
            state.cooling_body_entry_count,
            state.dehumidification_control_type_fallthrough_count,
        ),
        (
            "zone_dehumidifying_setpoint_moisture_demand_read_count",
            state.dehumidification_control_body_entry_count,
            state.zone_dehumidifying_setpoint_moisture_demand_read_count,
        ),
        (
            "zone_dehumidifying_setpoint_moisture_demand_assignment_count",
            state.dehumidification_control_body_entry_count,
            state.zone_dehumidifying_setpoint_moisture_demand_assignment_count,
        ),
        (
            "minimum_cooling_supply_air_humidity_ratio_read_count",
            state.dehumidification_control_body_entry_count,
            state.minimum_cooling_supply_air_humidity_ratio_read_count,
        ),
        (
            "zone_humidity_ratio_read_count",
            state.dehumidification_control_body_entry_count,
            state.zone_humidity_ratio_read_count,
        ),
        (
            "delta_humidity_ratio_calculation_count",
            state.dehumidification_control_body_entry_count,
            state.delta_humidity_ratio_calculation_count,
        ),
        (
            "delta_humidity_ratio_assignment_count",
            state.dehumidification_control_body_entry_count,
            state.delta_humidity_ratio_assignment_count,
        ),
        (
            "delta_humidity_ratio_for_gate_read_count",
            state.dehumidification_control_body_entry_count,
            state.delta_humidity_ratio_for_gate_read_count,
        ),
        (
            "delta_humidity_ratio_comparison_count",
            state.dehumidification_control_body_entry_count,
            state.delta_humidity_ratio_comparison_count,
        ),
        (
            "delta_humidity_ratio_partition",
            state.delta_humidity_ratio_comparison_count,
            delta_partition,
        ),
        (
            "zone_dehumidifying_setpoint_moisture_demand_for_gate_read_count",
            state.delta_humidity_ratio_comparison_satisfied_count,
            state.zone_dehumidifying_setpoint_moisture_demand_for_gate_read_count,
        ),
        (
            "zone_dehumidifying_setpoint_moisture_demand_comparison_count",
            state.delta_humidity_ratio_comparison_satisfied_count,
            state.zone_dehumidifying_setpoint_moisture_demand_comparison_count,
        ),
        (
            "zone_dehumidifying_setpoint_moisture_demand_partition",
            state.zone_dehumidifying_setpoint_moisture_demand_comparison_count,
            load_partition,
        ),
        (
            "dehumidification_flow_body_entry_count",
            state.zone_dehumidifying_setpoint_moisture_demand_comparison_satisfied_count,
            state.dehumidification_flow_body_entry_count,
        ),
        (
            "zone_dehumidifying_setpoint_moisture_demand_for_division_read_count",
            state.dehumidification_flow_body_entry_count,
            state.zone_dehumidifying_setpoint_moisture_demand_for_division_read_count,
        ),
        (
            "delta_humidity_ratio_for_division_read_count",
            state.dehumidification_flow_body_entry_count,
            state.delta_humidity_ratio_for_division_read_count,
        ),
        (
            "supply_mass_flow_rate_for_dehumidification_calculation_count",
            state.dehumidification_flow_body_entry_count,
            state.supply_mass_flow_rate_for_dehumidification_calculation_count,
        ),
        (
            "supply_mass_flow_rate_for_dehumidification_assignment_count",
            state.dehumidification_flow_body_entry_count,
            state.supply_mass_flow_rate_for_dehumidification_assignment_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling dehumidification-flow invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling dehumidification-flow {label} overflowed")
    })
}
