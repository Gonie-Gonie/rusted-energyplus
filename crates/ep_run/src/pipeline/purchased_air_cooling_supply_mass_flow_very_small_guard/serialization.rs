//! JSON serialization for CP327 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardLifecycleSummary,
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
        "supply_mass_flow_rate_read_count": state.supply_mass_flow_rate_read_count,
        "hvac_very_small_mass_flow_read_count": state.hvac_very_small_mass_flow_read_count,
        "supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count":
            state.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count,
        "zero_flow_reset_body_entry_count": state.zero_flow_reset_body_entry_count,
        "active_guard_false_fallthrough_count": state.active_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}
