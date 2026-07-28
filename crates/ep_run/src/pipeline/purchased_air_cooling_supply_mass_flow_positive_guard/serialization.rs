//! JSON serialization for CP330 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowPositiveGuardLifecycleSummary,
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
        "source_site_execution_count": state.source_site_execution_count,
        "supply_mass_flow_rate_read_count": state.supply_mass_flow_rate_read_count,
        "supply_mass_flow_rate_strictly_positive_comparison_count":
            state.supply_mass_flow_rate_strictly_positive_comparison_count,
        "positive_supply_mass_flow_body_entry_count":
            state.positive_supply_mass_flow_body_entry_count,
        "active_guard_false_fallthrough_count": state.active_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}
