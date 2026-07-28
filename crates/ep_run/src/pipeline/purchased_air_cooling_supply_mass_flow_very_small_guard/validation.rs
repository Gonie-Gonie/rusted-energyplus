//! Fail-closed validation helpers for CP327 evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState;

mod snapshot;

pub(super) use snapshot::snapshot_shape;

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardRuntimeState,
) -> Result<(), String> {
    let cooling_entries = state.cooling_body_entry_count;
    for (field, expected, actual) in [
        (
            "supply_mass_flow_rate_read_count",
            cooling_entries,
            state.supply_mass_flow_rate_read_count,
        ),
        (
            "hvac_very_small_mass_flow_read_count",
            cooling_entries,
            state.hvac_very_small_mass_flow_read_count,
        ),
        (
            "supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count",
            cooling_entries,
            state.supply_mass_flow_rate_at_or_below_very_small_mass_flow_comparison_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling very-small-flow guard invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}
