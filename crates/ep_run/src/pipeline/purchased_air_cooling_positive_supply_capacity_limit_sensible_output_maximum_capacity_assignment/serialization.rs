//! JSON serialization for CP341 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputMaximumCapacityAssignmentLifecycleSummary,
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
        "capacity_limit_guard_false_fallthrough_skip_count":
            state.capacity_limit_guard_false_fallthrough_skip_count,
        "capacity_limit_sensible_output_guard_false_fallthrough_count":
            state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        "capacity_limit_sensible_output_maximum_capacity_assignment_count":
            state.capacity_limit_sensible_output_maximum_capacity_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "maximum_total_cooling_capacity_read_count":
            state.maximum_total_cooling_capacity_read_count,
        "cooling_sensible_output_assignment_write_count":
            state.cooling_sensible_output_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
