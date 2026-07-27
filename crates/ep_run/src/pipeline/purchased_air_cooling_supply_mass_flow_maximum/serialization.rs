//! JSON serialization for CP322 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowMaximumLifecycleSummary,
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
        "outdoor_air_mass_flow_rate_read_count":
            state.outdoor_air_mass_flow_rate_read_count,
        "supply_mass_flow_rate_for_cool_read_count":
            state.supply_mass_flow_rate_for_cool_read_count,
        "supply_mass_flow_rate_for_dehumidification_read_count":
            state.supply_mass_flow_rate_for_dehumidification_read_count,
        "supply_mass_flow_rate_for_humidification_read_count":
            state.supply_mass_flow_rate_for_humidification_read_count,
        "positive_zero_vs_outdoor_air_comparison_count":
            state.positive_zero_vs_outdoor_air_comparison_count,
        "cooling_vs_dehumidification_comparison_count":
            state.cooling_vs_dehumidification_comparison_count,
        "leading_vs_candidate_pair_comparison_count":
            state.leading_vs_candidate_pair_comparison_count,
        "leading_vs_humidification_comparison_count":
            state.leading_vs_humidification_comparison_count,
        "maximum_evaluation_count": state.maximum_evaluation_count,
        "supply_mass_flow_rate_assignment_count":
            state.supply_mass_flow_rate_assignment_count,
        "latest": state.latest.map(snapshot_json),
    })
}
