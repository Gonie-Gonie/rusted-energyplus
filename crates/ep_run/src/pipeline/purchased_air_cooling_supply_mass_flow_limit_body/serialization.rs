//! JSON serialization for CP326 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodyLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "cooling_body_entry_count": state.cooling_body_entry_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "supply_mass_flow_limit_body_entry_count":
            state.supply_mass_flow_limit_body_entry_count,
        "body_skip_count": state.body_skip_count,
        "active_guard_false_fallthrough_count": state.active_guard_false_fallthrough_count,
        "supply_mass_flow_rate_for_minimum_read_count":
            state.supply_mass_flow_rate_for_minimum_read_count,
        "maximum_cooling_air_mass_flow_rate_for_minimum_read_count":
            state.maximum_cooling_air_mass_flow_rate_for_minimum_read_count,
        "source_shaped_two_argument_minimum_evaluation_count":
            state.source_shaped_two_argument_minimum_evaluation_count,
        "supply_mass_flow_rate_assignment_count": state.supply_mass_flow_rate_assignment_count,
        "latest": state.latest.map(snapshot_json),
    })
}
