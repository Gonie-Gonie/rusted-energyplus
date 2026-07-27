//! JSON serialization for CP318 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSensibleFlowLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSensibleFlowLifecycleSummary,
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
        "supply_mass_flow_rate_for_cool_reset_assignment_count":
            state.supply_mass_flow_rate_for_cool_reset_assignment_count,
        "cooling_on_read_count": state.cooling_on_read_count,
        "cooling_on_body_entry_count": state.cooling_on_body_entry_count,
        "cooling_on_fallthrough_count": state.cooling_on_fallthrough_count,
        "zone_humidity_ratio_read_count": state.zone_humidity_ratio_read_count,
        "psychrometric_cp_air_evaluation_count":
            state.psychrometric_cp_air_evaluation_count,
        "cp_air_assignment_count": state.cp_air_assignment_count,
        "minimum_cooling_supply_air_temperature_read_count":
            state.minimum_cooling_supply_air_temperature_read_count,
        "zone_temperature_read_count": state.zone_temperature_read_count,
    });
    extend_object(
        &mut value,
        json!({
            "delta_temperature_calculation_count":
                state.delta_temperature_calculation_count,
            "delta_temperature_assignment_count":
                state.delta_temperature_assignment_count,
            "delta_temperature_for_gate_read_count":
                state.delta_temperature_for_gate_read_count,
            "delta_temperature_comparison_count":
                state.delta_temperature_comparison_count,
            "delta_temperature_comparison_satisfied_count":
                state.delta_temperature_comparison_satisfied_count,
            "delta_temperature_body_entry_count":
                state.delta_temperature_body_entry_count,
            "delta_temperature_fallthrough_count":
                state.delta_temperature_fallthrough_count,
            "zone_cooling_setpoint_load_read_count":
                state.zone_cooling_setpoint_load_read_count,
            "cp_air_for_first_division_read_count":
                state.cp_air_for_first_division_read_count,
            "zone_cooling_setpoint_load_over_cp_air_calculation_count":
                state.zone_cooling_setpoint_load_over_cp_air_calculation_count,
            "delta_temperature_for_second_division_read_count":
                state.delta_temperature_for_second_division_read_count,
            "supply_mass_flow_rate_for_cool_calculation_count":
                state.supply_mass_flow_rate_for_cool_calculation_count,
            "supply_mass_flow_rate_for_cool_assignment_count":
                state.supply_mass_flow_rate_for_cool_assignment_count,
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
