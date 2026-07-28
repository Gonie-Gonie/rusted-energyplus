//! JSON serialization for CP332 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyTemperatureAssignmentLifecycleSummary,
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
        "supply_temperature_assignment_count": state.supply_temperature_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "zone_cooling_setpoint_load_read_count":
            state.zone_cooling_setpoint_load_read_count,
        "cp_air_read_count": state.cp_air_read_count,
        "supply_mass_flow_rate_read_count": state.supply_mass_flow_rate_read_count,
        "cp_air_times_supply_mass_flow_rate_calculation_count":
            state.cp_air_times_supply_mass_flow_rate_calculation_count,
        "zone_cooling_setpoint_load_over_denominator_calculation_count":
            state.zone_cooling_setpoint_load_over_denominator_calculation_count,
        "zone_node_temperature_read_count": state.zone_node_temperature_read_count,
        "supply_temperature_calculation_count":
            state.supply_temperature_calculation_count,
        "supply_temperature_assignment_write_count":
            state.supply_temperature_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
