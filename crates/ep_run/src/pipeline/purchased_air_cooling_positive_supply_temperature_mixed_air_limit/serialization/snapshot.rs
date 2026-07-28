//! JSON serialization for one CP334 snapshot.

use ep_runtime::PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot;
use serde_json::{Value, json};

pub(super) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
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
        "supply_temperature_mixed_air_limit_executed":
            snapshot.supply_temperature_mixed_air_limit_executed,
        "supply_temperature_for_minimum_read":
            snapshot.supply_temperature_for_minimum_read,
        "supply_temperature_before_mixed_air_limit_c":
            snapshot.supply_temperature_before_mixed_air_limit_c,
        "supply_temperature_before_mixed_air_limit_c_ieee_bits":
            ieee_bits(snapshot.supply_temperature_before_mixed_air_limit_c),
        "mixed_air_temperature_for_minimum_read":
            snapshot.mixed_air_temperature_for_minimum_read,
        "mixed_air_temperature_c": snapshot.mixed_air_temperature_c,
        "mixed_air_temperature_c_ieee_bits":
            ieee_bits(snapshot.mixed_air_temperature_c),
        "source_shaped_two_argument_minimum_evaluated":
            snapshot.source_shaped_two_argument_minimum_evaluated,
        "minimum_supply_temperature_c": snapshot.minimum_supply_temperature_c,
        "minimum_supply_temperature_c_ieee_bits":
            ieee_bits(snapshot.minimum_supply_temperature_c),
        "supply_temperature_assignment_performed":
            snapshot.supply_temperature_assignment_performed,
        "assigned_supply_temperature_c": snapshot.assigned_supply_temperature_c,
        "assigned_supply_temperature_c_ieee_bits":
            ieee_bits(snapshot.assigned_supply_temperature_c),
    })
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}
