//! JSON serialization for CP372 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "positive_guard_false_fallthrough_skip_count":
            state.positive_guard_false_fallthrough_skip_count,
        "heating_availability_guard_false_fallthrough_count":
            state.heating_availability_guard_false_fallthrough_count,
        "humidification_control_guard_false_fallthrough_count":
            state.humidification_control_guard_false_fallthrough_count,
        "dehumidification_control_humidistat_moisture_demand_assignment_count":
            state.dehumidification_control_humidistat_moisture_demand_assignment_count,
        "dehumidification_control_none_moisture_demand_assignment_count":
            state.dehumidification_control_none_moisture_demand_assignment_count,
        "dehumidification_control_guard_false_fallthrough_count":
            state.dehumidification_control_guard_false_fallthrough_count,
        "humidification_moisture_demand_assignment_count":
            state.humidification_moisture_demand_assignment_count,
        "zone_humidifying_setpoint_moisture_demand_read_count":
            state.zone_humidifying_setpoint_moisture_demand_read_count,
        "zone_humidifying_setpoint_moisture_demand_assignment_count":
            state.zone_humidifying_setpoint_moisture_demand_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "latest": state.latest.map(snapshot_json),
    })
}
