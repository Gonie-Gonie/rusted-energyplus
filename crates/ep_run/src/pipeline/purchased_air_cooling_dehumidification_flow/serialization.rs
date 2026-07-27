//! JSON serialization for CP319 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingDehumidificationFlowLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    let mut value = json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "cooling_body_entry_count": state.cooling_body_entry_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "supply_mass_flow_rate_for_dehumidification_reset_assignment_count":
            state.supply_mass_flow_rate_for_dehumidification_reset_assignment_count,
        "cooling_on_read_count": state.cooling_on_read_count,
        "cooling_on_body_entry_count": state.cooling_on_body_entry_count,
        "cooling_on_fallthrough_count": state.cooling_on_fallthrough_count,
        "dehumidification_control_type_read_count":
            state.dehumidification_control_type_read_count,
        "dehumidification_control_type_humidistat_count":
            state.dehumidification_control_type_humidistat_count,
        "dehumidification_control_type_fallthrough_count":
            state.dehumidification_control_type_fallthrough_count,
        "dehumidification_control_body_entry_count":
            state.dehumidification_control_body_entry_count,
        "zone_dehumidifying_setpoint_moisture_demand_read_count":
            state.zone_dehumidifying_setpoint_moisture_demand_read_count,
        "zone_dehumidifying_setpoint_moisture_demand_assignment_count":
            state.zone_dehumidifying_setpoint_moisture_demand_assignment_count,
        "minimum_cooling_supply_air_humidity_ratio_read_count":
            state.minimum_cooling_supply_air_humidity_ratio_read_count,
        "zone_humidity_ratio_read_count": state.zone_humidity_ratio_read_count,
    });
    extend_object(
        &mut value,
        json!({
            "delta_humidity_ratio_calculation_count":
                state.delta_humidity_ratio_calculation_count,
            "delta_humidity_ratio_assignment_count":
                state.delta_humidity_ratio_assignment_count,
            "delta_humidity_ratio_for_gate_read_count":
                state.delta_humidity_ratio_for_gate_read_count,
            "delta_humidity_ratio_comparison_count":
                state.delta_humidity_ratio_comparison_count,
            "delta_humidity_ratio_comparison_satisfied_count":
                state.delta_humidity_ratio_comparison_satisfied_count,
            "delta_humidity_ratio_fallthrough_count":
                state.delta_humidity_ratio_fallthrough_count,
            "zone_dehumidifying_setpoint_moisture_demand_for_gate_read_count":
                state.zone_dehumidifying_setpoint_moisture_demand_for_gate_read_count,
            "zone_dehumidifying_setpoint_moisture_demand_comparison_count":
                state.zone_dehumidifying_setpoint_moisture_demand_comparison_count,
            "zone_dehumidifying_setpoint_moisture_demand_comparison_satisfied_count":
                state.zone_dehumidifying_setpoint_moisture_demand_comparison_satisfied_count,
            "zone_dehumidifying_setpoint_moisture_demand_fallthrough_count":
                state.zone_dehumidifying_setpoint_moisture_demand_fallthrough_count,
            "dehumidification_flow_body_entry_count":
                state.dehumidification_flow_body_entry_count,
            "zone_dehumidifying_setpoint_moisture_demand_for_division_read_count":
                state.zone_dehumidifying_setpoint_moisture_demand_for_division_read_count,
            "delta_humidity_ratio_for_division_read_count":
                state.delta_humidity_ratio_for_division_read_count,
            "supply_mass_flow_rate_for_dehumidification_calculation_count":
                state.supply_mass_flow_rate_for_dehumidification_calculation_count,
            "supply_mass_flow_rate_for_dehumidification_assignment_count":
                state.supply_mass_flow_rate_for_dehumidification_assignment_count,
            "latest": state.latest.map(snapshot_json),
        }),
    );
    value
}

fn extend_object(target: &mut Value, extension: Value) {
    let Value::Object(extension) = extension else {
        return;
    };
    if let Value::Object(target) = target {
        target.extend(extension);
    }
}
