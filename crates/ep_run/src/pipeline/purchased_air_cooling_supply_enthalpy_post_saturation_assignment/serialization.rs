//! JSON serialization for CP379 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentLifecycleSummary,
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
        "heating_availability_guard_false_fallthrough_count":
            state.heating_availability_guard_false_fallthrough_count,
        "humidification_control_guard_false_fallthrough_count":
            state.humidification_control_guard_false_fallthrough_count,
        "dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count":
            state.dehumidification_control_humidistat_supply_humidity_ratio_maximum_assignment_count,
        "dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count":
            state.dehumidification_control_none_supply_humidity_ratio_maximum_assignment_count,
        "dehumidification_control_guard_false_fallthrough_count":
            state.dehumidification_control_guard_false_fallthrough_count,
        "source_site_execution_count": state.source_site_execution_count,
        "purchased_air_supply_temperature_for_post_saturation_enthalpy_read_count":
            state.purchased_air_supply_temperature_for_post_saturation_enthalpy_read_count,
        "purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read_count":
            state.purchased_air_supply_humidity_ratio_for_post_saturation_enthalpy_read_count,
        "psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluation_count":
            state.psy_h_fn_tdb_w_for_post_saturation_enthalpy_evaluation_count,
        "local_supply_enthalpy_after_saturation_limit_assignment_count":
            state.local_supply_enthalpy_after_saturation_limit_assignment_count,
        "cp334_supply_temperature_mixed_air_limit_owner_count":
            state.cp334_supply_temperature_mixed_air_limit_owner_count,
        "cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count":
            state.cp344_capacity_limit_supply_temperature_mixed_air_limit_owner_count,
        "cp378_supply_humidity_ratio_saturation_limit_owner_count":
            state.cp378_supply_humidity_ratio_saturation_limit_owner_count,
        "latest": state.latest.map(snapshot_json),
    })
}
