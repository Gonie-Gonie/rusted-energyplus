//! Fail-closed validation helpers for CP328 evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState;

mod snapshot;

pub(super) use snapshot::snapshot_shape;

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingSupplyMassFlowVerySmallGuardBodyRuntimeState,
) -> Result<(), String> {
    let expected = state.zero_flow_reset_body_entry_count;
    let actual = state.supply_mass_flow_rate_positive_zero_assignment_count;
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "direct-zone IdealLoads cooling positive-zero reset-body invariant supply_mass_flow_rate_positive_zero_assignment_count expected {expected}, got {actual}"
        ))
    }
}
