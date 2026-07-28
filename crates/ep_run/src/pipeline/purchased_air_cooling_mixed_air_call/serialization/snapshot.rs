//! JSON serialization for one CP329 snapshot.

use ep_runtime::{IdealLoadsSensibleMode, PurchasedAirCalcCoolingMixedAirCallSnapshot};
use serde_json::{Map, Value, json};

const SERIALIZED_IEEE_BIT_FIELDS: &[&str] = &[
    "outdoor_air_mass_flow_rate_kg_per_s_ieee_bits",
    "supply_mass_flow_rate_kg_per_s_ieee_bits",
    "initial_recirculation_mass_flow_rate_kg_per_s_ieee_bits",
    "recirculation_temperature_c_ieee_bits",
    "recirculation_humidity_ratio_ieee_bits",
    "recirculation_enthalpy_projection_j_per_kg_ieee_bits",
    "outdoor_air_inlet_temperature_c_ieee_bits",
    "outdoor_air_inlet_humidity_ratio_ieee_bits",
    "outdoor_air_inlet_enthalpy_j_per_kg_ieee_bits",
    "outdoor_air_after_heat_recovery_temperature_c_ieee_bits",
    "outdoor_air_after_heat_recovery_humidity_ratio_ieee_bits",
    "outdoor_air_after_heat_recovery_enthalpy_j_per_kg_ieee_bits",
    "child_supply_mass_flow_rate_kg_per_s_ieee_bits",
    "resulting_recirculation_mass_flow_rate_kg_per_s_ieee_bits",
    "mixed_air_temperature_c_ieee_bits",
    "mixed_air_humidity_ratio_ieee_bits",
    "mixed_air_enthalpy_projection_j_per_kg_ieee_bits",
    "heat_recovery_sensible_output_w_ieee_bits",
    "heat_recovery_latent_output_w_ieee_bits",
];

pub(super) fn snapshot_json(snapshot: PurchasedAirCalcCoolingMixedAirCallSnapshot) -> Value {
    let mut object = match json!({
        "source": snapshot.source,
        "child_source": snapshot.child_source,
        "first_excluded_source": snapshot.first_excluded_source,
        "source_order": snapshot.source_order,
        "no_oa_child_source_order": snapshot.no_oa_child_source_order,
    }) {
        Value::Object(object) => object,
        _ => Map::new(),
    };
    macro_rules! field {
        ($name:literal, $value:expr) => {
            object.insert($name.to_string(), json!($value));
        };
    }
    field!("system", snapshot.system.0);
    field!("parent_call_ordinal", snapshot.parent_call_ordinal);
    field!("controlled_zone", snapshot.controlled_zone.0);
    field!("unit_body_entered", snapshot.unit_body_entered);
    field!(
        "predecessor_cooling_body_entered",
        snapshot.predecessor_cooling_body_entered
    );
    field!(
        "predecessor_zero_flow_reset_body_entered",
        snapshot.predecessor_zero_flow_reset_body_entered
    );
    field!(
        "predecessor_active_guard_false_fallthrough",
        snapshot.predecessor_active_guard_false_fallthrough
    );
    field!("unit_off_skipped", snapshot.unit_off_skipped);
    field!("non_cooling_skipped", snapshot.non_cooling_skipped);
    field!("cooling_call_executed", snapshot.cooling_call_executed);
    field!("state_reference_bound", snapshot.state_reference_bound);
    field!(
        "purchased_air_number_read",
        snapshot.purchased_air_number_read
    );
    field!(
        "outdoor_air_mass_flow_rate_read",
        snapshot.outdoor_air_mass_flow_rate_read
    );
    optional_f64_fields(
        &mut object,
        "outdoor_air_mass_flow_rate_kg_per_s",
        snapshot.outdoor_air_mass_flow_rate_kg_per_s,
    );
    field!(
        "supply_mass_flow_rate_read",
        snapshot.supply_mass_flow_rate_read
    );
    optional_f64_fields(
        &mut object,
        "supply_mass_flow_rate_kg_per_s",
        snapshot.supply_mass_flow_rate_kg_per_s,
    );
    field!(
        "mixed_air_temperature_output_reference_bound",
        snapshot.mixed_air_temperature_output_reference_bound
    );
    field!(
        "mixed_air_humidity_ratio_output_reference_bound",
        snapshot.mixed_air_humidity_ratio_output_reference_bound
    );
    field!(
        "mixed_air_enthalpy_output_reference_bound",
        snapshot.mixed_air_enthalpy_output_reference_bound
    );
    field!("operating_mode_read", snapshot.operating_mode_read);
    field!(
        "operating_mode",
        snapshot.operating_mode.map(operating_mode_name)
    );
    field!(
        "calc_purch_air_mixed_air_called",
        snapshot.calc_purch_air_mixed_air_called
    );
    field!(
        "purchased_air_alias_bound",
        snapshot.purchased_air_alias_bound
    );
    field!(
        "outdoor_air_node_number_copied",
        snapshot.outdoor_air_node_number_copied
    );
    field!(
        "outdoor_air_node",
        snapshot.outdoor_air_node.map(|node| node.0)
    );
    field!(
        "recirculation_node_number_copied",
        snapshot.recirculation_node_number_copied
    );
    field!(
        "recirculation_node",
        snapshot.recirculation_node.map(|node| node.0)
    );
    field!(
        "recirculation_mass_flow_rate_initialized",
        snapshot.recirculation_mass_flow_rate_initialized
    );
    optional_f64_fields(
        &mut object,
        "initial_recirculation_mass_flow_rate_kg_per_s",
        snapshot.initial_recirculation_mass_flow_rate_kg_per_s,
    );
    field!(
        "recirculation_temperature_read",
        snapshot.recirculation_temperature_read
    );
    optional_f64_fields(
        &mut object,
        "recirculation_temperature_c",
        snapshot.recirculation_temperature_c,
    );
    field!(
        "recirculation_humidity_ratio_read",
        snapshot.recirculation_humidity_ratio_read
    );
    optional_f64_fields(
        &mut object,
        "recirculation_humidity_ratio",
        snapshot.recirculation_humidity_ratio,
    );
    field!(
        "recirculation_enthalpy_projection_read",
        snapshot.recirculation_enthalpy_projection_read
    );
    optional_f64_fields(
        &mut object,
        "recirculation_enthalpy_projection_j_per_kg",
        snapshot.recirculation_enthalpy_projection_j_per_kg,
    );
    field!(
        "outdoor_air_initialization_guard_evaluated",
        snapshot.outdoor_air_initialization_guard_evaluated
    );
    field!("outdoor_air_enabled", snapshot.outdoor_air_enabled);
    optional_f64_fields(
        &mut object,
        "outdoor_air_inlet_temperature_c",
        snapshot.outdoor_air_inlet_temperature_c,
    );
    optional_f64_fields(
        &mut object,
        "outdoor_air_inlet_humidity_ratio",
        snapshot.outdoor_air_inlet_humidity_ratio,
    );
    optional_f64_fields(
        &mut object,
        "outdoor_air_inlet_enthalpy_j_per_kg",
        snapshot.outdoor_air_inlet_enthalpy_j_per_kg,
    );
    optional_f64_fields(
        &mut object,
        "outdoor_air_after_heat_recovery_temperature_c",
        snapshot.outdoor_air_after_heat_recovery_temperature_c,
    );
    optional_f64_fields(
        &mut object,
        "outdoor_air_after_heat_recovery_humidity_ratio",
        snapshot.outdoor_air_after_heat_recovery_humidity_ratio,
    );
    optional_f64_fields(
        &mut object,
        "outdoor_air_after_heat_recovery_enthalpy_j_per_kg",
        snapshot.outdoor_air_after_heat_recovery_enthalpy_j_per_kg,
    );
    field!(
        "heat_recovery_on_false_assigned",
        snapshot.heat_recovery_on_false_assigned
    );
    field!("heat_recovery_on", snapshot.heat_recovery_on);
    field!(
        "outdoor_air_active_guard_first_operand_evaluated",
        snapshot.outdoor_air_active_guard_first_operand_evaluated
    );
    field!(
        "outdoor_air_mass_flow_positive_comparison_evaluated",
        snapshot.outdoor_air_mass_flow_positive_comparison_evaluated
    );
    field!(
        "no_outdoor_air_fallback_entered",
        snapshot.no_outdoor_air_fallback_entered
    );
    field!(
        "child_supply_mass_flow_rate_read",
        snapshot.child_supply_mass_flow_rate_read
    );
    optional_f64_fields(
        &mut object,
        "child_supply_mass_flow_rate_kg_per_s",
        snapshot.child_supply_mass_flow_rate_kg_per_s,
    );
    field!(
        "recirculation_mass_flow_rate_assigned_from_supply",
        snapshot.recirculation_mass_flow_rate_assigned_from_supply
    );
    optional_f64_fields(
        &mut object,
        "resulting_recirculation_mass_flow_rate_kg_per_s",
        snapshot.resulting_recirculation_mass_flow_rate_kg_per_s,
    );
    field!(
        "mixed_air_temperature_assigned",
        snapshot.mixed_air_temperature_assigned
    );
    optional_f64_fields(
        &mut object,
        "mixed_air_temperature_c",
        snapshot.mixed_air_temperature_c,
    );
    field!(
        "mixed_air_humidity_ratio_assigned",
        snapshot.mixed_air_humidity_ratio_assigned
    );
    optional_f64_fields(
        &mut object,
        "mixed_air_humidity_ratio",
        snapshot.mixed_air_humidity_ratio,
    );
    field!(
        "mixed_air_enthalpy_projection_assigned",
        snapshot.mixed_air_enthalpy_projection_assigned
    );
    optional_f64_fields(
        &mut object,
        "mixed_air_enthalpy_projection_j_per_kg",
        snapshot.mixed_air_enthalpy_projection_j_per_kg,
    );
    field!(
        "heat_recovery_sensible_output_positive_zero_assigned",
        snapshot.heat_recovery_sensible_output_positive_zero_assigned
    );
    optional_f64_fields(
        &mut object,
        "heat_recovery_sensible_output_w",
        snapshot.heat_recovery_sensible_output_w,
    );
    field!(
        "heat_recovery_latent_output_positive_zero_assigned",
        snapshot.heat_recovery_latent_output_positive_zero_assigned
    );
    optional_f64_fields(
        &mut object,
        "heat_recovery_latent_output_w",
        snapshot.heat_recovery_latent_output_w,
    );
    debug_assert!(
        SERIALIZED_IEEE_BIT_FIELDS
            .iter()
            .all(|field| object.contains_key(*field))
    );
    Value::Object(object)
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}

fn optional_f64_fields(object: &mut Map<String, Value>, name: &str, value: Option<f64>) {
    object.insert(name.to_string(), json!(value));
    object.insert(format!("{name}_ieee_bits"), json!(ieee_bits(value)));
}

fn operating_mode_name(value: IdealLoadsSensibleMode) -> &'static str {
    match value {
        IdealLoadsSensibleMode::Off => "Off",
        IdealLoadsSensibleMode::Deadband => "Deadband",
        IdealLoadsSensibleMode::Cooling => "Cooling",
        IdealLoadsSensibleMode::Heating => "Heating",
    }
}
