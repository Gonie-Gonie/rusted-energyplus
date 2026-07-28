//! JSON serialization for CP331 lifecycle evidence.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary;
use serde_json::{Value, json};

mod snapshot;

use snapshot::snapshot_json;

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingPositiveSupplyCpAirAssignmentLifecycleSummary,
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
        "cp_air_assignment_count": state.cp_air_assignment_count,
        "source_site_execution_count": state.source_site_execution_count,
        "zone_humidity_ratio_read_count": state.zone_humidity_ratio_read_count,
        "psychrometric_cp_air_evaluation_count":
            state.psychrometric_cp_air_evaluation_count,
        "cp_air_assignment_write_count": state.cp_air_assignment_write_count,
        "latest": state.latest.map(snapshot_json),
    })
}
