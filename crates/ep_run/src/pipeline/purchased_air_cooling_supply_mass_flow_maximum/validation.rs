//! Fail-closed validation helpers for CP322 direct-release evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState;

mod snapshot;

pub(super) use snapshot::{same_option, snapshot_shape};

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingSupplyMassFlowMaximumRuntimeState,
) -> Result<(), String> {
    for (field, expected, actual) in [
        (
            "outdoor_air_mass_flow_rate_read_count",
            state.cooling_body_entry_count,
            state.outdoor_air_mass_flow_rate_read_count,
        ),
        (
            "supply_mass_flow_rate_for_cool_read_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_for_cool_read_count,
        ),
        (
            "supply_mass_flow_rate_for_dehumidification_read_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_for_dehumidification_read_count,
        ),
        (
            "supply_mass_flow_rate_for_humidification_read_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_for_humidification_read_count,
        ),
        (
            "positive_zero_vs_outdoor_air_comparison_count",
            state.cooling_body_entry_count,
            state.positive_zero_vs_outdoor_air_comparison_count,
        ),
        (
            "cooling_vs_dehumidification_comparison_count",
            state.cooling_body_entry_count,
            state.cooling_vs_dehumidification_comparison_count,
        ),
        (
            "leading_vs_candidate_pair_comparison_count",
            state.cooling_body_entry_count,
            state.leading_vs_candidate_pair_comparison_count,
        ),
        (
            "leading_vs_humidification_comparison_count",
            state.cooling_body_entry_count,
            state.leading_vs_humidification_comparison_count,
        ),
        (
            "maximum_evaluation_count",
            state.cooling_body_entry_count,
            state.maximum_evaluation_count,
        ),
        (
            "supply_mass_flow_rate_assignment_count",
            state.cooling_body_entry_count,
            state.supply_mass_flow_rate_assignment_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling supply-flow maximum invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}
