//! JSON serialization for CP325 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowLimitGuardLifecycleSummary,
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
        "first_cooling_limit_read_count": state.first_cooling_limit_read_count,
        "cooling_limit_flow_rate_comparison_count":
            state.cooling_limit_flow_rate_comparison_count,
        "cooling_limit_flow_rate_match_count": state.cooling_limit_flow_rate_match_count,
        "second_cooling_limit_read_count": state.second_cooling_limit_read_count,
        "cooling_limit_flow_rate_and_capacity_comparison_count":
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
        "cooling_limit_flow_rate_and_capacity_match_count":
            state.cooling_limit_flow_rate_and_capacity_match_count,
        "cooling_limit_rejected_count": state.cooling_limit_rejected_count,
        "maximum_cooling_air_mass_flow_rate_read_count":
            state.maximum_cooling_air_mass_flow_rate_read_count,
        "maximum_cooling_air_mass_flow_rate_positive_comparison_count":
            state.maximum_cooling_air_mass_flow_rate_positive_comparison_count,
        "maximum_cooling_air_mass_flow_rate_strictly_positive_count":
            state.maximum_cooling_air_mass_flow_rate_strictly_positive_count,
        "maximum_cooling_air_mass_flow_rate_not_positive_count":
            state.maximum_cooling_air_mass_flow_rate_not_positive_count,
        "supply_mass_flow_limit_body_entry_count":
            state.supply_mass_flow_limit_body_entry_count,
        "active_guard_false_fallthrough_count": state.active_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}
