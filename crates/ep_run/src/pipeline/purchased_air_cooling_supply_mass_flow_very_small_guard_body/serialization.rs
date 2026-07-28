//! JSON serialization for CP328 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyLifecycleSummary,
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
        "zero_flow_reset_body_entry_count": state.zero_flow_reset_body_entry_count,
        "body_skip_count": state.body_skip_count,
        "active_guard_false_fallthrough_count": state.active_guard_false_fallthrough_count,
        "supply_mass_flow_rate_positive_zero_assignment_count":
            state.supply_mass_flow_rate_positive_zero_assignment_count,
        "latest": state.latest.map(snapshot_json),
    })
}
