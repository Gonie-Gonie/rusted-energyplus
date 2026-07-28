//! JSON serialization for CP342 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle:
        &PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "positive_guard_false_fallthrough_skip_count":
            state.positive_guard_false_fallthrough_skip_count,
        "capacity_limit_guard_false_fallthrough_skip_count":
            state.capacity_limit_guard_false_fallthrough_skip_count,
        "capacity_limit_sensible_output_guard_false_fallthrough_count":
            state.capacity_limit_sensible_output_guard_false_fallthrough_count,
        "capacity_limit_sensible_output_supply_enthalpy_assignment_count":
            state.capacity_limit_sensible_output_supply_enthalpy_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "mixed_air_enthalpy_read_count": state.mixed_air_enthalpy_read_count,
        "cooling_sensible_output_read_count": state.cooling_sensible_output_read_count,
        "supply_mass_flow_rate_read_count": state.supply_mass_flow_rate_read_count,
        "specific_cooling_output_calculation_count":
            state.specific_cooling_output_calculation_count,
        "supply_enthalpy_calculation_count": state.supply_enthalpy_calculation_count,
        "supply_enthalpy_assignment_write_count":
            state.supply_enthalpy_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
