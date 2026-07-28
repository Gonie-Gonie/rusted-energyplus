//! Fail-closed validation helpers for CP324 exact direct evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState;

mod snapshot;

pub(super) use snapshot::snapshot_shape;

pub(super) fn validate_source_counters(
    state: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRuntimeState,
) -> Result<(), String> {
    for (field, expected, actual) in [
        ("body_entry_count", 0, state.body_entry_count),
        (
            "body_skip_count",
            state.transition_count,
            state.body_skip_count,
        ),
        (
            "ems_disabled_fallthrough_count",
            state.cooling_body_entry_count,
            state.ems_disabled_fallthrough_count,
        ),
        (
            "ems_supply_mass_flow_override_value_read_count",
            0,
            state.ems_supply_mass_flow_override_value_read_count,
        ),
        (
            "supply_mass_flow_rate_override_assignment_count",
            0,
            state.supply_mass_flow_rate_override_assignment_count,
        ),
        (
            "outdoor_air_mass_flow_rate_for_minimum_read_count",
            0,
            state.outdoor_air_mass_flow_rate_for_minimum_read_count,
        ),
        (
            "supply_mass_flow_rate_for_minimum_read_count",
            0,
            state.supply_mass_flow_rate_for_minimum_read_count,
        ),
        (
            "source_shaped_two_argument_minimum_evaluation_count",
            0,
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "outdoor_air_mass_flow_rate_assignment_count",
            0,
            state.outdoor_air_mass_flow_rate_assignment_count,
        ),
    ] {
        if actual != expected {
            return Err(format!(
                "direct-zone IdealLoads EMS mass-flow override body invariant {field} expected {expected}, got {actual}"
            ));
        }
    }
    Ok(())
}
