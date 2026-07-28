//! JSON serialization for CP337 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitGuardLifecycleSummary,
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
        "capacity_limit_guard_evaluation_count":
            state.capacity_limit_guard_evaluation_count,
        "source_site_execution_count": state.source_site_execution_count,
        "first_cooling_limit_read_count": state.first_cooling_limit_read_count,
        "cooling_limit_capacity_comparison_count":
            state.cooling_limit_capacity_comparison_count,
        "cooling_limit_capacity_match_count": state.cooling_limit_capacity_match_count,
        "second_cooling_limit_read_count": state.second_cooling_limit_read_count,
        "cooling_limit_flow_rate_and_capacity_comparison_count":
            state.cooling_limit_flow_rate_and_capacity_comparison_count,
        "cooling_limit_flow_rate_and_capacity_match_count":
            state.cooling_limit_flow_rate_and_capacity_match_count,
        "cooling_limit_rejected_count": state.cooling_limit_rejected_count,
        "capacity_limit_body_entry_count": state.capacity_limit_body_entry_count,
        "active_guard_false_fallthrough_count":
            state.active_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}
