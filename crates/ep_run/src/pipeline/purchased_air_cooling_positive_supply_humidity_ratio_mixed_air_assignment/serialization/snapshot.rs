//! JSON serialization for one CP335 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_no_outdoor_air_fallback_entered":
            snapshot.predecessor_no_outdoor_air_fallback_entered,
        "predecessor_positive_supply_mass_flow_body_entered":
            snapshot.predecessor_positive_supply_mass_flow_body_entered,
        "predecessor_active_guard_false_fallthrough":
            snapshot.predecessor_active_guard_false_fallthrough,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "positive_guard_false_fallthrough_skipped":
            snapshot.positive_guard_false_fallthrough_skipped,
        "supply_humidity_ratio_mixed_air_assignment_executed":
            snapshot.supply_humidity_ratio_mixed_air_assignment_executed,
        "mixed_air_humidity_ratio_read": snapshot.mixed_air_humidity_ratio_read,
        "mixed_air_humidity_ratio": snapshot.mixed_air_humidity_ratio,
        "mixed_air_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.mixed_air_humidity_ratio),
        "supply_humidity_ratio_assignment_performed":
            snapshot.supply_humidity_ratio_assignment_performed,
        "assigned_supply_humidity_ratio": snapshot.assigned_supply_humidity_ratio,
        "assigned_supply_humidity_ratio_ieee_bits":
            ieee_bits(snapshot.assigned_supply_humidity_ratio),
    })
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}
