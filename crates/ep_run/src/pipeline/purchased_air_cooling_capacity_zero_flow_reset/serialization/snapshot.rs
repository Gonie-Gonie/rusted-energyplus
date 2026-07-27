//! JSON serialization for one CP321 source-site snapshot.

use ep_model::IdealLoadsLimit;
use ep_runtime::PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingCapacityZeroFlowResetSnapshot,
) -> Value {
    let mut value = json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "cooling_body_entered": snapshot.cooling_body_entered,
        "first_cooling_limit_read": snapshot.first_cooling_limit_read,
        "first_cooling_limit": snapshot.first_cooling_limit.map(limit_name),
        "cooling_limit_capacity": snapshot.cooling_limit_capacity,
        "second_cooling_limit_read": snapshot.second_cooling_limit_read,
        "second_cooling_limit": snapshot.second_cooling_limit.map(limit_name),
        "cooling_limit_flow_rate_and_capacity":
            snapshot.cooling_limit_flow_rate_and_capacity,
        "cooling_limit_condition_satisfied":
            snapshot.cooling_limit_condition_satisfied,
        "maximum_total_cooling_capacity_read":
            snapshot.maximum_total_cooling_capacity_read,
        "maximum_total_cooling_capacity_w":
            snapshot.maximum_total_cooling_capacity_w,
    });
    extend_object(
        &mut value,
        json!({
            "maximum_total_cooling_capacity_comparison_evaluated":
                snapshot.maximum_total_cooling_capacity_comparison_evaluated,
            "maximum_total_cooling_capacity_equal_to_zero":
                snapshot.maximum_total_cooling_capacity_equal_to_zero,
            "zero_cooling_capacity_body_entered":
                snapshot.zero_cooling_capacity_body_entered,
            "predecessor_supply_mass_flow_rate_for_cool_kg_per_s":
                snapshot.predecessor_supply_mass_flow_rate_for_cool_kg_per_s,
            "predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s":
                snapshot.predecessor_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            "predecessor_supply_mass_flow_rate_for_humidification_kg_per_s":
                snapshot.predecessor_supply_mass_flow_rate_for_humidification_kg_per_s,
            "supply_mass_flow_rate_for_cool_zero_assigned":
                snapshot.supply_mass_flow_rate_for_cool_zero_assigned,
            "assigned_supply_mass_flow_rate_for_cool_kg_per_s":
                snapshot.assigned_supply_mass_flow_rate_for_cool_kg_per_s,
            "supply_mass_flow_rate_for_dehumidification_zero_assigned":
                snapshot.supply_mass_flow_rate_for_dehumidification_zero_assigned,
            "assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s":
                snapshot.assigned_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            "supply_mass_flow_rate_for_humidification_zero_assigned":
                snapshot.supply_mass_flow_rate_for_humidification_zero_assigned,
            "assigned_supply_mass_flow_rate_for_humidification_kg_per_s":
                snapshot.assigned_supply_mass_flow_rate_for_humidification_kg_per_s,
            "resulting_supply_mass_flow_rate_for_cool_kg_per_s":
                snapshot.resulting_supply_mass_flow_rate_for_cool_kg_per_s,
            "resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s":
                snapshot.resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s,
            "resulting_supply_mass_flow_rate_for_humidification_kg_per_s":
                snapshot.resulting_supply_mass_flow_rate_for_humidification_kg_per_s,
        }),
    );
    value
}

fn limit_name(limit: IdealLoadsLimit) -> &'static str {
    match limit {
        IdealLoadsLimit::NoLimit => "NoLimit",
        IdealLoadsLimit::LimitFlowRate => "LimitFlowRate",
        IdealLoadsLimit::LimitCapacity => "LimitCapacity",
        IdealLoadsLimit::LimitFlowRateAndCapacity => "LimitFlowRateAndCapacity",
    }
}

fn extend_object(target: &mut Value, extension: Value) {
    let Value::Object(extension) = extension else {
        return;
    };
    if let Value::Object(target) = target {
        target.extend(extension);
    }
}
