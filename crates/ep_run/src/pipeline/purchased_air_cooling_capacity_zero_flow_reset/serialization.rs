//! JSON serialization for CP321 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingCapacityZeroFlowResetLifecycleSummary,
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
        "cooling_limit_capacity_count": state.cooling_limit_capacity_count,
        "second_cooling_limit_read_count": state.second_cooling_limit_read_count,
        "cooling_limit_flow_rate_and_capacity_count":
            state.cooling_limit_flow_rate_and_capacity_count,
        "cooling_limit_rejected_count": state.cooling_limit_rejected_count,
        "maximum_total_cooling_capacity_read_count":
            state.maximum_total_cooling_capacity_read_count,
        "maximum_total_cooling_capacity_comparison_count":
            state.maximum_total_cooling_capacity_comparison_count,
        "maximum_total_cooling_capacity_zero_count":
            state.maximum_total_cooling_capacity_zero_count,
        "maximum_total_cooling_capacity_nonzero_count":
            state.maximum_total_cooling_capacity_nonzero_count,
        "zero_cooling_capacity_body_entry_count":
            state.zero_cooling_capacity_body_entry_count,
        "supply_mass_flow_rate_for_cool_zero_assignment_count":
            state.supply_mass_flow_rate_for_cool_zero_assignment_count,
        "supply_mass_flow_rate_for_dehumidification_zero_assignment_count":
            state.supply_mass_flow_rate_for_dehumidification_zero_assignment_count,
        "supply_mass_flow_rate_for_humidification_zero_assignment_count":
            state.supply_mass_flow_rate_for_humidification_zero_assignment_count,
        "latest": state.latest.map(snapshot_json),
    })
}
