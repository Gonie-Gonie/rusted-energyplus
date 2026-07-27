//! Fail-closed validation helpers for CP320 direct-release evidence.

use ep_runtime::PurchasedAirCalcCoolingHumidificationFlowRuntimeState;

mod snapshot;

pub(super) use snapshot::snapshot_shape;

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingHumidificationFlowRuntimeState,
) -> Result<(), String> {
    let heating_partition = checked_add(
        state.heating_on_body_entry_count,
        state.heating_on_fallthrough_count,
        "heating-on partition",
    )?;
    let humidification_partition = checked_add(
        state.humidification_control_type_humidistat_count,
        state.humidification_control_type_fallthrough_count,
        "humidification selector partition",
    )?;
    for (field, expected, actual) in [
        (
            "reset_assignment_count",
            state.cooling_body_entry_count,
            state.reset_assignment_count,
        ),
        (
            "heating_on_read_count",
            state.cooling_body_entry_count,
            state.heating_on_read_count,
        ),
        (
            "heating_on_partition",
            state.heating_on_read_count,
            heating_partition,
        ),
        (
            "direct_heating_on_body_entry_count",
            state.cooling_body_entry_count,
            state.heating_on_body_entry_count,
        ),
        (
            "direct_heating_on_fallthrough_count",
            0,
            state.heating_on_fallthrough_count,
        ),
        (
            "humidification_control_type_read_count",
            state.heating_on_body_entry_count,
            state.humidification_control_type_read_count,
        ),
        (
            "humidification_control_partition",
            state.humidification_control_type_read_count,
            humidification_partition,
        ),
        (
            "direct_humidification_control_type_humidistat_count",
            0,
            state.humidification_control_type_humidistat_count,
        ),
        (
            "direct_humidification_control_type_fallthrough_count",
            state.cooling_body_entry_count,
            state.humidification_control_type_fallthrough_count,
        ),
        (
            "humidification_control_body_entry_count",
            0,
            state.humidification_control_body_entry_count,
        ),
        (
            "dehumidification_control_type_first_read_count",
            0,
            state.dehumidification_control_type_first_read_count,
        ),
        (
            "dehumidification_control_type_humidistat_count",
            0,
            state.dehumidification_control_type_humidistat_count,
        ),
        (
            "dehumidification_control_type_second_read_count",
            0,
            state.dehumidification_control_type_second_read_count,
        ),
        (
            "dehumidification_control_type_none_count",
            0,
            state.dehumidification_control_type_none_count,
        ),
        (
            "dehumidification_control_type_rejected_count",
            0,
            state.dehumidification_control_type_rejected_count,
        ),
        (
            "admitted_control_body_entry_count",
            0,
            state.admitted_control_body_entry_count,
        ),
        (
            "moisture_demand_read_count",
            0,
            state.moisture_demand_read_count,
        ),
        (
            "moisture_demand_assignment_count",
            0,
            state.moisture_demand_assignment_count,
        ),
        (
            "maximum_heating_supply_humidity_ratio_read_count",
            0,
            state.maximum_heating_supply_humidity_ratio_read_count,
        ),
        (
            "zone_humidity_ratio_read_count",
            0,
            state.zone_humidity_ratio_read_count,
        ),
        ("delta_calculation_count", 0, state.delta_calculation_count),
        ("delta_assignment_count", 0, state.delta_assignment_count),
        ("delta_gate_read_count", 0, state.delta_gate_read_count),
        ("delta_comparison_count", 0, state.delta_comparison_count),
        (
            "delta_comparison_satisfied_count",
            0,
            state.delta_comparison_satisfied_count,
        ),
        ("delta_fallthrough_count", 0, state.delta_fallthrough_count),
        (
            "moisture_demand_gate_read_count",
            0,
            state.moisture_demand_gate_read_count,
        ),
        (
            "moisture_demand_comparison_count",
            0,
            state.moisture_demand_comparison_count,
        ),
        (
            "moisture_demand_comparison_satisfied_count",
            0,
            state.moisture_demand_comparison_satisfied_count,
        ),
        (
            "moisture_demand_fallthrough_count",
            0,
            state.moisture_demand_fallthrough_count,
        ),
        (
            "humidification_flow_body_entry_count",
            0,
            state.humidification_flow_body_entry_count,
        ),
        (
            "moisture_demand_division_read_count",
            0,
            state.moisture_demand_division_read_count,
        ),
        (
            "delta_division_read_count",
            0,
            state.delta_division_read_count,
        ),
        ("calculation_count", 0, state.calculation_count),
        ("assignment_count", 0, state.assignment_count),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling humidification-flow invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}

fn checked_add(left: usize, right: usize, label: &str) -> Result<usize, String> {
    left.checked_add(right).ok_or_else(|| {
        format!("direct-zone IdealLoads cooling humidification-flow {label} overflowed")
    })
}
