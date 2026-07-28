//! JSON serialization for CP323 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
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
        "ems_supply_mass_flow_override_flag_read_count":
            state.ems_supply_mass_flow_override_flag_read_count,
        "ems_supply_mass_flow_override_guard_evaluation_count":
            state.ems_supply_mass_flow_override_guard_evaluation_count,
        "ems_supply_mass_flow_override_body_entry_count":
            state.ems_supply_mass_flow_override_body_entry_count,
        "ems_supply_mass_flow_override_guard_false_fallthrough_count":
            state.ems_supply_mass_flow_override_guard_false_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}
