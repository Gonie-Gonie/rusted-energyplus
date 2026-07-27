//! JSON serialization for CP320 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingHumidificationFlowLifecycleSummary,
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
        "reset_assignment_count": state.reset_assignment_count,
        "heating_on_read_count": state.heating_on_read_count,
        "heating_on_body_entry_count": state.heating_on_body_entry_count,
        "heating_on_fallthrough_count": state.heating_on_fallthrough_count,
        "humidification_control_type_read_count":
            state.humidification_control_type_read_count,
        "humidification_control_type_humidistat_count":
            state.humidification_control_type_humidistat_count,
        "humidification_control_type_fallthrough_count":
            state.humidification_control_type_fallthrough_count,
        "humidification_control_body_entry_count":
            state.humidification_control_body_entry_count,
        "dehumidification_control_type_first_read_count":
            state.dehumidification_control_type_first_read_count,
        "dehumidification_control_type_humidistat_count":
            state.dehumidification_control_type_humidistat_count,
        "dehumidification_control_type_second_read_count":
            state.dehumidification_control_type_second_read_count,
        "dehumidification_control_type_none_count":
            state.dehumidification_control_type_none_count,
    });
    extend_object(
        &mut value,
        json!({
            "dehumidification_control_type_rejected_count":
                state.dehumidification_control_type_rejected_count,
            "admitted_control_body_entry_count": state.admitted_control_body_entry_count,
            "moisture_demand_read_count": state.moisture_demand_read_count,
            "moisture_demand_assignment_count": state.moisture_demand_assignment_count,
            "maximum_heating_supply_humidity_ratio_read_count":
                state.maximum_heating_supply_humidity_ratio_read_count,
            "zone_humidity_ratio_read_count": state.zone_humidity_ratio_read_count,
            "delta_calculation_count": state.delta_calculation_count,
            "delta_assignment_count": state.delta_assignment_count,
            "delta_gate_read_count": state.delta_gate_read_count,
            "delta_comparison_count": state.delta_comparison_count,
            "delta_comparison_satisfied_count": state.delta_comparison_satisfied_count,
            "delta_fallthrough_count": state.delta_fallthrough_count,
            "moisture_demand_gate_read_count": state.moisture_demand_gate_read_count,
            "moisture_demand_comparison_count": state.moisture_demand_comparison_count,
            "moisture_demand_comparison_satisfied_count":
                state.moisture_demand_comparison_satisfied_count,
            "moisture_demand_fallthrough_count": state.moisture_demand_fallthrough_count,
            "humidification_flow_body_entry_count":
                state.humidification_flow_body_entry_count,
            "moisture_demand_division_read_count":
                state.moisture_demand_division_read_count,
            "delta_division_read_count": state.delta_division_read_count,
            "calculation_count": state.calculation_count,
            "assignment_count": state.assignment_count,
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
