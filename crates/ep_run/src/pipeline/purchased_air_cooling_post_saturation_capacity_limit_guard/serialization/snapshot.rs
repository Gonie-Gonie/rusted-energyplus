//! JSON serialization for one CP380 control-only snapshot.

use ep_model::IdealLoadsLimit;
use ep_runtime::PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPostSaturationCapacityLimitGuardSnapshot,
) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "heating_availability_guard_false_fallthrough":
            snapshot.heating_availability_guard_false_fallthrough,
        "humidification_control_guard_false_fallthrough":
            snapshot.humidification_control_guard_false_fallthrough,
        "dehumidification_control_humidistat_maximum_assignment_executed":
            snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        "dehumidification_control_none_maximum_assignment_executed":
            snapshot.dehumidification_control_none_maximum_assignment_executed,
        "dehumidification_control_guard_false_fallthrough":
            snapshot.dehumidification_control_guard_false_fallthrough,
        "predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed":
            snapshot.predecessor_local_supply_enthalpy_after_saturation_limit_assignment_performed,
        "capacity_limit_guard_evaluated": snapshot.capacity_limit_guard_evaluated,
        "configured_cooling_limit_owned_read": snapshot.configured_cooling_limit_owned_read,
        "cp337_same_call_selector_lineage_corroborated":
            snapshot.cp337_same_call_selector_lineage_corroborated,
        "first_cooling_limit_read": snapshot.first_cooling_limit_read,
        "first_cooling_limit": snapshot.first_cooling_limit.map(limit_name),
        "cooling_limit_capacity_comparison_evaluated":
            snapshot.cooling_limit_capacity_comparison_evaluated,
        "cooling_limit_capacity": snapshot.cooling_limit_capacity,
        "second_cooling_limit_read": snapshot.second_cooling_limit_read,
        "second_cooling_limit": snapshot.second_cooling_limit.map(limit_name),
        "cooling_limit_flow_rate_and_capacity_comparison_evaluated":
            snapshot.cooling_limit_flow_rate_and_capacity_comparison_evaluated,
        "cooling_limit_flow_rate_and_capacity":
            snapshot.cooling_limit_flow_rate_and_capacity,
        "cooling_limit_condition_satisfied": snapshot.cooling_limit_condition_satisfied,
        "cooling_limit_rejected": snapshot.cooling_limit_rejected,
        "capacity_limit_body_entered": snapshot.capacity_limit_body_entered,
        "active_guard_false_fallthrough": snapshot.active_guard_false_fallthrough,
    })
}

fn limit_name(limit: IdealLoadsLimit) -> &'static str {
    match limit {
        IdealLoadsLimit::NoLimit => "NoLimit",
        IdealLoadsLimit::LimitFlowRate => "LimitFlowRate",
        IdealLoadsLimit::LimitCapacity => "LimitCapacity",
        IdealLoadsLimit::LimitFlowRateAndCapacity => "LimitFlowRateAndCapacity",
    }
}
