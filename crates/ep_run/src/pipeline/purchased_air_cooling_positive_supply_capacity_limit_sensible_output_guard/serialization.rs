//! JSON serialization for CP340 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputGuardLifecycleSummary,
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
        "capacity_limit_sensible_output_guard_evaluation_count":
            state.capacity_limit_sensible_output_guard_evaluation_count,
        "source_site_execution_count": state.source_site_execution_count,
        "cooling_sensible_output_read_count": state.cooling_sensible_output_read_count,
        "maximum_total_cooling_capacity_read_count":
            state.maximum_total_cooling_capacity_read_count,
        "cooling_sensible_output_maximum_capacity_comparison_count":
            state.cooling_sensible_output_maximum_capacity_comparison_count,
        "capacity_limit_sensible_output_guard_false_fallthrough_count":
            state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        "capacity_limit_sensible_output_adjustment_body_entry_count":
            state.capacity_limit_sensible_output_adjustment_body_entry_count,
        "latest": state.latest.map(snapshot_json),
    })
}
