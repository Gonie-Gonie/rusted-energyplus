//! Fail-closed validation helpers for CP326 evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState;

mod snapshot;

pub(super) use snapshot::snapshot_shape;

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingSupplyMassFlowLimitBodyRuntimeState,
) -> Result<(), String> {
    let body_entries = state.supply_mass_flow_limit_body_entry_count;
    for (field, expected, actual) in [
        (
            "supply_mass_flow_rate_for_minimum_read_count",
            body_entries,
            state.supply_mass_flow_rate_for_minimum_read_count,
        ),
        (
            "maximum_cooling_air_mass_flow_rate_for_minimum_read_count",
            body_entries,
            state.maximum_cooling_air_mass_flow_rate_for_minimum_read_count,
        ),
        (
            "source_shaped_two_argument_minimum_evaluation_count",
            body_entries,
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "supply_mass_flow_rate_assignment_count",
            body_entries,
            state.supply_mass_flow_rate_assignment_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads cooling flow-limit body invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}
