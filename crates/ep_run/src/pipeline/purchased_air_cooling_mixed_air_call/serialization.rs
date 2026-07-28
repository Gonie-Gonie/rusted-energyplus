//! JSON serialization for CP329 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingMixedAirCallLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingMixedAirCallLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "child_source": lifecycle.child_source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "cooling_call_count": state.cooling_call_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "caller_source_site_execution_count": state.caller_source_site_execution_count,
        "child_source_site_execution_count": state.child_source_site_execution_count,
        "state_reference_bind_count": state.state_reference_bind_count,
        "purchased_air_number_read_count": state.purchased_air_number_read_count,
        "outdoor_air_mass_flow_rate_read_count": state.outdoor_air_mass_flow_rate_read_count,
        "supply_mass_flow_rate_read_count": state.supply_mass_flow_rate_read_count,
        "mixed_air_output_reference_bind_count": state.mixed_air_output_reference_bind_count,
        "operating_mode_read_count": state.operating_mode_read_count,
        "mixed_air_child_call_count": state.mixed_air_child_call_count,
        "no_outdoor_air_fallback_count": state.no_outdoor_air_fallback_count,
        "recirculation_enthalpy_projection_count": state.recirculation_enthalpy_projection_count,
        "mixed_air_output_assignment_count": state.mixed_air_output_assignment_count,
        "heat_recovery_output_positive_zero_assignment_count":
            state.heat_recovery_output_positive_zero_assignment_count,
        "latest": state.latest.map(snapshot_json),
    })
}
