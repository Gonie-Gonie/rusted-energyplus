//! JSON serialization for CP316 lifecycle evidence.

use ep_model::OutdoorAirEconomizerType;
use ep_runtime::{
    PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
    PurchasedAirCalcCoolingEconomizerConditionSnapshot,
};
use serde_json::{Value, json};

pub(in crate::pipeline) fn lifecycle_json(
    lifecycle: &PurchasedAirCalcCoolingEconomizerConditionLifecycleSummary,
) -> Value {
    let state = &lifecycle.state;
    json!({
        "source": lifecycle.source,
        "first_excluded_source": lifecycle.first_excluded_source,
        "system": state.system.0,
        "transition_count": state.transition_count,
        "condition_evaluation_count": state.condition_evaluation_count,
        "unit_off_skip_count": state.unit_off_skip_count,
        "non_cooling_skip_count": state.non_cooling_skip_count,
        "maximum_cooling_flow_body_sibling_skip_count":
            state.maximum_cooling_flow_body_sibling_skip_count,
        "no_economizer_outer_guard_fallthrough_skip_count":
            state.no_economizer_outer_guard_fallthrough_skip_count,
        "differential_dry_bulb_economizer_type_read_count":
            state.differential_dry_bulb_economizer_type_read_count,
        "differential_dry_bulb_selector_comparison_count":
            state.differential_dry_bulb_selector_comparison_count,
        "differential_dry_bulb_selector_match_count":
            state.differential_dry_bulb_selector_match_count,
        "outdoor_air_temperature_read_count": state.outdoor_air_temperature_read_count,
        "recirculation_air_temperature_read_count":
            state.recirculation_air_temperature_read_count,
        "dry_bulb_temperature_comparison_count":
            state.dry_bulb_temperature_comparison_count,
        "dry_bulb_temperature_comparison_satisfied_count":
            state.dry_bulb_temperature_comparison_satisfied_count,
        "differential_enthalpy_economizer_type_read_count":
            state.differential_enthalpy_economizer_type_read_count,
        "differential_enthalpy_selector_comparison_count":
            state.differential_enthalpy_selector_comparison_count,
        "differential_enthalpy_selector_match_count":
            state.differential_enthalpy_selector_match_count,
        "outdoor_air_enthalpy_read_count": state.outdoor_air_enthalpy_read_count,
        "recirculation_air_enthalpy_read_count":
            state.recirculation_air_enthalpy_read_count,
        "enthalpy_comparison_count": state.enthalpy_comparison_count,
        "enthalpy_comparison_satisfied_count":
            state.enthalpy_comparison_satisfied_count,
        "economizer_calculation_body_entry_count":
            state.economizer_calculation_body_entry_count,
        "economizer_condition_fallthrough_count":
            state.economizer_condition_fallthrough_count,
        "latest": state.latest.map(snapshot_json),
    })
}

fn snapshot_json(snapshot: PurchasedAirCalcCoolingEconomizerConditionSnapshot) -> Value {
    json!({
        "source": snapshot.source,
        "first_excluded_source": snapshot.first_excluded_source,
        "system": snapshot.system.0,
        "parent_call_ordinal": snapshot.parent_call_ordinal,
        "source_order": snapshot.source_order,
        "controlled_zone": snapshot.controlled_zone.0,
        "unit_body_entered": snapshot.unit_body_entered,
        "predecessor_cooling_body_entered": snapshot.predecessor_cooling_body_entered,
        "predecessor_maximum_cooling_flow_body_entered":
            snapshot.predecessor_maximum_cooling_flow_body_entered,
        "predecessor_active_guard_false_economizer_fallthrough":
            snapshot.predecessor_active_guard_false_economizer_fallthrough,
        "predecessor_economizer_guard_evaluated":
            snapshot.predecessor_economizer_guard_evaluated,
        "predecessor_economizer_body_entered":
            snapshot.predecessor_economizer_body_entered,
        "predecessor_no_economizer_fallthrough":
            snapshot.predecessor_no_economizer_fallthrough,
        "unit_off_skipped": snapshot.unit_off_skipped,
        "non_cooling_skipped": snapshot.non_cooling_skipped,
        "maximum_cooling_flow_body_sibling_skipped":
            snapshot.maximum_cooling_flow_body_sibling_skipped,
        "no_economizer_outer_guard_fallthrough_skipped":
            snapshot.no_economizer_outer_guard_fallthrough_skipped,
        "economizer_condition_evaluated": snapshot.economizer_condition_evaluated,
        "differential_dry_bulb_economizer_type_read":
            snapshot.differential_dry_bulb_economizer_type_read,
        "differential_dry_bulb_economizer_type":
            snapshot.differential_dry_bulb_economizer_type.map(economizer_type_name),
        "differential_dry_bulb_selector_comparison_evaluated":
            snapshot.differential_dry_bulb_selector_comparison_evaluated,
        "differential_dry_bulb_selector_matched":
            snapshot.differential_dry_bulb_selector_matched,
        "outdoor_air_temperature_read": snapshot.outdoor_air_temperature_read,
        "outdoor_air_temperature_c": snapshot.outdoor_air_temperature_c,
        "recirculation_air_temperature_read": snapshot.recirculation_air_temperature_read,
        "recirculation_air_temperature_c": snapshot.recirculation_air_temperature_c,
        "dry_bulb_temperature_comparison_evaluated":
            snapshot.dry_bulb_temperature_comparison_evaluated,
        "outdoor_air_temperature_below_recirculation_temperature":
            snapshot.outdoor_air_temperature_below_recirculation_temperature,
        "differential_enthalpy_economizer_type_read":
            snapshot.differential_enthalpy_economizer_type_read,
        "differential_enthalpy_economizer_type":
            snapshot.differential_enthalpy_economizer_type.map(economizer_type_name),
        "differential_enthalpy_selector_comparison_evaluated":
            snapshot.differential_enthalpy_selector_comparison_evaluated,
        "differential_enthalpy_selector_matched":
            snapshot.differential_enthalpy_selector_matched,
        "outdoor_air_enthalpy_read": snapshot.outdoor_air_enthalpy_read,
        "outdoor_air_enthalpy_j_per_kg": snapshot.outdoor_air_enthalpy_j_per_kg,
        "recirculation_air_enthalpy_read": snapshot.recirculation_air_enthalpy_read,
        "recirculation_air_enthalpy_j_per_kg":
            snapshot.recirculation_air_enthalpy_j_per_kg,
        "enthalpy_comparison_evaluated": snapshot.enthalpy_comparison_evaluated,
        "outdoor_air_enthalpy_below_recirculation_enthalpy":
            snapshot.outdoor_air_enthalpy_below_recirculation_enthalpy,
        "economizer_condition_satisfied": snapshot.economizer_condition_satisfied,
        "economizer_calculation_body_entered":
            snapshot.economizer_calculation_body_entered,
        "economizer_condition_fallthrough": snapshot.economizer_condition_fallthrough,
    })
}

fn economizer_type_name(value: OutdoorAirEconomizerType) -> &'static str {
    match value {
        OutdoorAirEconomizerType::NoEconomizer => "NoEconomizer",
        OutdoorAirEconomizerType::DifferentialDryBulb => "DifferentialDryBulb",
        OutdoorAirEconomizerType::DifferentialEnthalpy => "DifferentialEnthalpy",
    }
}
