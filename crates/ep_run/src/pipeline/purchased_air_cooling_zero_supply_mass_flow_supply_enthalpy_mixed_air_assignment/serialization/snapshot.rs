//! Lossless JSON serialization for one CP425 zero-flow enthalpy assignment snapshot.

use ep_runtime::{
    PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot,
    cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_predecessor_cp424_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_cooling_supply_mass_flow_positive_guard_else_branch_entry::serialization::snapshot::snapshot_json as cp424_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcCoolingZeroSupplyMassFlowSupplyEnthalpyMixedAirAssignmentSnapshot,
) -> Value {
    let predecessor =
        cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_predecessor_cp424_snapshot(
            snapshot,
        );
    let mut value = cp424_snapshot_json(predecessor);
    let Value::Object(target) = &mut value else {
        return Value::Null;
    };
    target.insert("source".to_string(), json!(snapshot.source));
    target.insert(
        "first_excluded_source".to_string(),
        json!(snapshot.first_excluded_source),
    );
    target.insert("source_order".to_string(), json!(snapshot.source_order));
    for key in [
        "resulting_supply_humidity_ratio",
        "resulting_supply_humidity_ratio_ieee_bits",
        "resulting_supply_enthalpy_j_per_kg",
        "resulting_supply_enthalpy_j_per_kg_ieee_bits",
        "resulting_supply_temperature_c",
        "resulting_supply_temperature_c_ieee_bits",
        "cooling_supply_mass_flow_positive_guard_else_branch_entered",
    ] {
        target.remove(key);
    }
    extend_object(
        target,
        json!({
            "predecessor_cp424_resulting_supply_humidity_ratio": json_number(snapshot.predecessor_cp424_resulting_supply_humidity_ratio),
            "predecessor_cp424_resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.predecessor_cp424_resulting_supply_humidity_ratio),
            "predecessor_cp424_resulting_supply_enthalpy_j_per_kg": json_number(snapshot.predecessor_cp424_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp424_resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.predecessor_cp424_resulting_supply_enthalpy_j_per_kg),
            "predecessor_cp424_resulting_supply_temperature_c": json_number(snapshot.predecessor_cp424_resulting_supply_temperature_c),
            "predecessor_cp424_resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.predecessor_cp424_resulting_supply_temperature_c),
            "cooling_supply_mass_flow_positive_guard_else_branch_entered": snapshot.cooling_supply_mass_flow_positive_guard_else_branch_entered,
            "cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_executed": snapshot.cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_executed,
            "cp424_retained_supply_humidity_ratio_state_owned": snapshot.cp424_retained_supply_humidity_ratio_state_owned,
            "cp424_retained_supply_enthalpy_state_owned": snapshot.cp424_retained_supply_enthalpy_state_owned,
            "cp424_retained_supply_temperature_state_owned": snapshot.cp424_retained_supply_temperature_state_owned,
            "cp329_retained_mixed_air_enthalpy_owned_read": snapshot.cp329_retained_mixed_air_enthalpy_owned_read,
            "mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_read": snapshot.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_read,
            "mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg": json_number(snapshot.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg),
            "mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.mixed_air_enthalpy_for_zero_supply_mass_flow_supply_enthalpy_j_per_kg),
            "zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_performed": snapshot.zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment_performed,
            "assigned_supply_enthalpy_from_mixed_air_j_per_kg": json_number(snapshot.assigned_supply_enthalpy_from_mixed_air_j_per_kg),
            "assigned_supply_enthalpy_from_mixed_air_j_per_kg_ieee_bits": ieee_bits(snapshot.assigned_supply_enthalpy_from_mixed_air_j_per_kg),
            "resulting_supply_humidity_ratio": json_number(snapshot.resulting_supply_humidity_ratio),
            "resulting_supply_humidity_ratio_ieee_bits": ieee_bits(snapshot.resulting_supply_humidity_ratio),
            "resulting_supply_enthalpy_j_per_kg": json_number(snapshot.resulting_supply_enthalpy_j_per_kg),
            "resulting_supply_enthalpy_j_per_kg_ieee_bits": ieee_bits(snapshot.resulting_supply_enthalpy_j_per_kg),
            "resulting_supply_temperature_c": json_number(snapshot.resulting_supply_temperature_c),
            "resulting_supply_temperature_c_ieee_bits": ieee_bits(snapshot.resulting_supply_temperature_c),
        }),
    );
    value
}

fn extend_object(target: &mut serde_json::Map<String, Value>, extension: Value) {
    if let Value::Object(extension) = extension {
        target.extend(extension);
    }
}

fn json_number(value: Option<f64>) -> Value {
    value
        .filter(|value| value.is_finite())
        .map_or(Value::Null, |value| json!(value))
}

fn ieee_bits(value: Option<f64>) -> Option<String> {
    value.map(|value| format!("0x{:016x}", value.to_bits()))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use ep_compiler::compile_raw_model;
    use ep_model::{SimulationModel, ZoneId};
    use ep_raw_model::parse_epjson_str;
    use ep_runtime::schedules::precompute_schedule_cache;
    use ep_runtime::{
        DirectZonePurchasedAirScheduledCouplingInput, PurchasedAirRuntimeState,
        ZoneAirTemperatureCoefficients, ZoneHeatBalanceState, bind_direct_zone_purchased_air_model,
        couple_model_bound_direct_zone_purchased_air,
    };

    use super::*;

    #[test]
    fn serializer_keeps_exact_cp424_first_350_then_appends_24_unique_keys() {
        let cp424 = canonical_cp424_keys();
        let tail = literal_keys(include_str!("snapshot.rs"));
        assert_eq!(cp424.len(), 357);
        assert_eq!(tail.len(), 24);
        let mut keys = cp424[..350].to_vec();
        keys.extend(tail);
        assert_eq!(keys.len(), 374);
        assert_eq!(keys.iter().copied().collect::<BTreeSet<_>>().len(), 374);
        assert_eq!(
            keys.iter()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count(),
            99
        );
        assert_eq!(
            keys[350],
            "predecessor_cp424_resulting_supply_humidity_ratio"
        );
        assert_eq!(
            keys[356],
            "cooling_supply_mass_flow_positive_guard_else_branch_entered"
        );
        assert_eq!(keys[373], "resulting_supply_temperature_c_ieee_bits");
    }

    #[test]
    fn real_scheduled_snapshot_serializes_the_exact_374_key_set_and_99_sidecars() {
        let output = real_output();
        let snapshot =
            output.calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment;
        let value = snapshot_json(snapshot);
        let object = value.as_object().expect("real CP425 JSON object");
        let keys = object.keys().map(String::as_str).collect::<Vec<_>>();
        assert_eq!(keys.len(), 374);
        assert_eq!(keys.iter().copied().collect::<BTreeSet<_>>().len(), 374);
        assert_eq!(
            keys.iter()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count(),
            99
        );
        let mut expected = canonical_cp424_keys()[..350].to_vec();
        expected.extend(literal_keys(include_str!("snapshot.rs")));
        assert_eq!(
            keys.iter().copied().collect::<BTreeSet<_>>(),
            expected.into_iter().collect::<BTreeSet<_>>()
        );
    }

    #[test]
    fn nonfinite_local_and_result_enthalpies_are_null_with_exact_ieee_sidecars() {
        let mut snapshot = real_output()
            .calculation_cooling_zero_supply_mass_flow_supply_enthalpy_mixed_air_assignment;
        let assigned = f64::from_bits(0x7ff8_0000_0000_0425);
        let resulting = f64::NEG_INFINITY;
        snapshot.assigned_supply_enthalpy_from_mixed_air_j_per_kg = Some(assigned);
        snapshot.resulting_supply_enthalpy_j_per_kg = Some(resulting);
        let value = snapshot_json(snapshot);
        assert!(value["assigned_supply_enthalpy_from_mixed_air_j_per_kg"].is_null());
        assert_eq!(
            value["assigned_supply_enthalpy_from_mixed_air_j_per_kg_ieee_bits"],
            format!("0x{:016x}", assigned.to_bits())
        );
        assert!(value["resulting_supply_enthalpy_j_per_kg"].is_null());
        assert_eq!(
            value["resulting_supply_enthalpy_j_per_kg_ieee_bits"],
            format!("0x{:016x}", resulting.to_bits())
        );
    }

    fn canonical_cp424_keys() -> Vec<&'static str> {
        let cp420 = literal_keys(include_str!(
            "../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_assignment/serialization/snapshot.rs"
        ));
        let cp421_tail = literal_keys(include_str!(
            "../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_guard/serialization/snapshot.rs"
        ));
        let cp422_tail = literal_keys(include_str!(
            "../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_maximum_capacity_assignment/serialization/snapshot.rs"
        ));
        let cp423_tail = literal_keys(include_str!(
            "../../purchased_air_cooling_post_saturation_capacity_limit_dehumidification_guard_else_branch_sensible_output_supply_temperature_assignment/serialization/snapshot.rs"
        ));
        assert_eq!(
            (
                cp420.len(),
                cp421_tail.len(),
                cp422_tail.len(),
                cp423_tail.len()
            ),
            (273, 29, 27, 45)
        );
        let mut cp421 = cp420[..267].to_vec();
        cp421.extend(cp421_tail);
        let mut cp422 = cp421[..290].to_vec();
        cp422.extend(cp422_tail);
        let mut keys = cp422[..311].to_vec();
        keys.extend(cp423_tail);
        keys.push("cooling_supply_mass_flow_positive_guard_else_branch_entered");
        keys
    }

    fn literal_keys(source: &'static str) -> Vec<&'static str> {
        source
            .lines()
            .filter_map(|line| {
                line.trim_start()
                    .strip_prefix('"')
                    .and_then(|line| line.split_once("\":").map(|(key, _)| key))
            })
            .collect()
    }

    fn real_output() -> ep_runtime::DirectZonePurchasedAirScheduledCouplingOutput {
        let fixtures = include_str!("../../../../tests/arbitrary_run/fixtures.rs");
        let epjson = fixtures
            .split_once("pub(crate) const IDEAL_LOADS_EPJSON: &str = r#\"")
            .and_then(|(_, tail)| tail.split_once("\"#;"))
            .map(|(fixture, _)| fixture)
            .expect("embedded IdealLoads epJSON fixture")
            .replace(
                r#""Heating Setpoint": {"hourly_value": 21}"#,
                r#""Heating Setpoint": {"hourly_value": 0}"#,
            )
            .replace(
                r#""Cooling Setpoint": {"hourly_value": 24}"#,
                r#""Cooling Setpoint": {"hourly_value": 15}"#,
            );
        let raw = parse_epjson_str(&epjson).expect("real CP425 raw fixture");
        let typed = compile_raw_model(&raw)
            .model
            .expect("real CP425 typed fixture");
        let model = SimulationModel::from_typed(typed);
        let cache = precompute_schedule_cache(&model.typed, 1).expect("real CP425 schedule cache");
        let binding = bind_direct_zone_purchased_air_model(&model).expect("real CP425 binding");
        let mut zone_state = cooling_zone_state(binding.nominal_system_timestep_seconds);
        let mut runtime = PurchasedAirRuntimeState::default();
        couple_model_bound_direct_zone_purchased_air(DirectZonePurchasedAirScheduledCouplingInput {
            binding: &binding,
            schedule_cache: &cache,
            schedule_sample_index: 0,
            zone_state: &mut zone_state,
            purchased_air_runtime_state: &mut runtime,
            begin_environment: true,
            barometric_pressure_pa: binding.limit_context.barometric_pressure_pa,
            system_timestep_seconds: binding.nominal_system_timestep_seconds,
        })
        .expect("real CP425 scheduled coupling")
    }

    fn cooling_zone_state(system_timestep_seconds: f64) -> ZoneHeatBalanceState {
        ZoneHeatBalanceState {
            zone_id: ZoneId(0),
            zone_name: "ZONE ONE".to_string(),
            mean_air_temperature_c: 22.0,
            zone_timestep_average_air_temperature_c: 22.0,
            previous_mean_air_temperatures_c: [0.0; 3],
            previous_system_mean_air_temperatures_c: [0.0; 3],
            previous_system_timestep_count: 1,
            air_humidity_ratio: 0.020,
            zone_timestep_average_air_humidity_ratio: 0.020,
            previous_air_humidity_ratios: [0.020; 3],
            previous_system_air_humidity_ratios: [0.020; 3],
            use_zone_timestep_history: false,
            shorten_timestep_sys: false,
            prior_timestep_seconds: system_timestep_seconds,
            volume_m3: 100.0,
            air_heat_capacity_j_per_k: 0.0,
            convective_internal_gain_w: 0.0,
            opaque_surface_conductance_w_per_k: 100.0,
            opaque_surface_heat_gain_w: 0.0,
            opaque_surface_outside_conduction_w: 0.0,
            sum_ha_w_per_k: 100.0,
            sum_hat_surf_w: 3_000.0,
            sum_hat_ref_w: 0.0,
            sum_mcp_w_per_k: 0.0,
            sum_mcp_t_w: 0.0,
            sum_sys_mcp_w_per_k: 7.0,
            sum_sys_mcp_t_w: 11.0,
            system_dependent_zone_loads_lagged_w: 0.0,
            zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients {
                temp_dependent_coefficient_w_per_k: 0.0,
                temp_independent_coefficient_w: 0.0,
                air_power_cap_w_per_k: 0.0,
                third_order_history_term_w: 0.0,
                third_order_temp_dependent_load_w_per_k: 0.0,
                third_order_temp_independent_load_w: 0.0,
            },
            system_timestep_average_surface_convection_report_w: None,
            system_timestep_average_air_storage_report_w: None,
        }
    }
}
