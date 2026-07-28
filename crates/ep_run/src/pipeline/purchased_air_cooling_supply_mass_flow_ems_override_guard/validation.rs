//! Fail-closed validation helpers for CP323 exact direct evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState;

mod snapshot;

pub(super) use snapshot::snapshot_shape;

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRuntimeState,
) -> Result<(), String> {
    for (field, expected, actual) in [
        (
            "ems_supply_mass_flow_override_flag_read_count",
            state.cooling_body_entry_count,
            state.ems_supply_mass_flow_override_flag_read_count,
        ),
        (
            "ems_supply_mass_flow_override_guard_evaluation_count",
            state.cooling_body_entry_count,
            state.ems_supply_mass_flow_override_guard_evaluation_count,
        ),
        (
            "ems_supply_mass_flow_override_body_entry_count",
            0,
            state.ems_supply_mass_flow_override_body_entry_count,
        ),
        (
            "ems_supply_mass_flow_override_guard_false_fallthrough_count",
            state.cooling_body_entry_count,
            state.ems_supply_mass_flow_override_guard_false_fallthrough_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads EMS mass-flow override guard invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}
