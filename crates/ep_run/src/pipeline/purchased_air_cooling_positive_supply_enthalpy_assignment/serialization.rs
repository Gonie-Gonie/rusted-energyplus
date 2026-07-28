//! JSON serialization for CP336 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentLifecycleSummary,
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
        "supply_enthalpy_assignment_count": state.supply_enthalpy_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "supply_temperature_for_enthalpy_read_count":
            state.supply_temperature_for_enthalpy_read_count,
        "supply_humidity_ratio_for_enthalpy_read_count":
            state.supply_humidity_ratio_for_enthalpy_read_count,
        "psychrometric_supply_enthalpy_evaluation_count":
            state.psychrometric_supply_enthalpy_evaluation_count,
        "supply_enthalpy_assignment_write_count":
            state.supply_enthalpy_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
