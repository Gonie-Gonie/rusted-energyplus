//! JSON serialization for CP386 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlSwitchLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_lexical_source": lifecycle.first_excluded_lexical_source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "inactive_transition_count": state.inactive_transition_count,
        "dehumidification_control_switch_count": state.dehumidification_control_switch_count,
        "predecessor_route_counts": state.predecessor_route_counts,
        "source_site_execution_count": state.source_site_execution_count,
        "dehumidification_control_type_read_count": state.dehumidification_control_type_read_count,
        "dehumidification_control_switch_dispatch_count": state.dehumidification_control_switch_dispatch_count,
        "dehumidification_control_constant_sensible_heat_ratio_case_selection_count": state.dehumidification_control_constant_sensible_heat_ratio_case_selection_count,
        "dehumidification_control_humidistat_case_selection_count": state.dehumidification_control_humidistat_case_selection_count,
        "dehumidification_control_none_case_selection_count": state.dehumidification_control_none_case_selection_count,
        "dehumidification_control_constant_supply_humidity_ratio_case_selection_count": state.dehumidification_control_constant_supply_humidity_ratio_case_selection_count,
        "latest": state.latest.map(snapshot_json),
    })
}
