//! Lossless JSON serialization for one CP430 structural case-entry snapshot.

use ep_runtime::{
    PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
    heating_or_no_load_case_entry_predecessor_cp429_snapshot,
};
use serde_json::{Value, json};

use crate::pipeline::purchased_air_cooling_zero_supply_mass_flow_total_output_positive_zero_assignment::serialization::snapshot::snapshot_json as cp429_snapshot_json;

pub(in crate::pipeline) fn snapshot_json(
    snapshot: PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot,
) -> Value {
    let predecessor = heating_or_no_load_case_entry_predecessor_cp429_snapshot(snapshot);
    let mut value = cp429_snapshot_json(predecessor);
    let Value::Object(target) = &mut value else {
        return Value::Null;
    };
    target.insert("source".to_string(), json!(snapshot.source));
    target.insert(
        "first_excluded_source".to_string(),
        json!(snapshot.first_excluded_source),
    );
    target.insert("source_order".to_string(), json!(snapshot.source_order));
    target.insert(
        "heating_or_no_load_case_entered".to_string(),
        json!(snapshot.heating_or_no_load_case_entered),
    );
    value
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
    fn serializer_source_composes_cp429_and_inserts_only_one_new_marker() {
        let source = include_str!("snapshot.rs")
            .split_once("#[cfg(test)]")
            .map_or(include_str!("snapshot.rs"), |(production, _)| production);
        assert!(source.contains("cp429_snapshot_json(predecessor)"));
        assert_eq!(source.matches("heating_or_no_load_case_entered").count(), 2);
        assert!(!source.contains("target.remove"));
        for forbidden in [
            "DirectZonePurchasedAirCouplingInput",
            "numerical_dto",
            "prediction",
            "feedback",
            "nodes",
            "loads",
            "reports",
        ] {
            assert!(!source.contains(forbidden), "{forbidden}");
        }
    }

    #[test]
    fn real_snapshot_json_is_exact_cp429_key_set_plus_marker_with_117_sidecars() {
        let output = real_output();
        let snapshot = output.calculation_heating_or_no_load_case_entry;
        let predecessor = heating_or_no_load_case_entry_predecessor_cp429_snapshot(snapshot);
        let predecessor_value = cp429_snapshot_json(predecessor);
        let value = snapshot_json(snapshot);
        let predecessor_object = predecessor_value
            .as_object()
            .expect("real CP429 JSON object");
        let object = value.as_object().expect("real CP430 JSON object");
        assert_eq!(predecessor_object.len(), 434);
        assert_eq!(object.len(), 435);
        let predecessor_keys = predecessor_object
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        let mut successor_keys = object.keys().map(String::as_str).collect::<BTreeSet<_>>();
        assert!(successor_keys.remove("heating_or_no_load_case_entered"));
        assert_eq!(successor_keys, predecessor_keys);
        assert_eq!(
            object
                .keys()
                .filter(|key| key.ends_with("_ieee_bits"))
                .count(),
            117,
        );
        assert_eq!(
            object["heating_or_no_load_case_entered"],
            predecessor_object["non_cooling_skipped"]
        );

        let core = include_str!(
            "../../../../../ep_runtime/src/ideal_loads/calc/heating_or_no_load_case_entry.rs"
        );
        let schema = core
            .split_once("pub struct PurchasedAirCalcHeatingOrNoLoadCaseEntrySnapshot")
            .and_then(|(_, tail)| tail.split_once("/// Final selected-unit CP430"))
            .map(|(schema, _)| schema)
            .expect("CP430 snapshot declaration");
        assert_eq!(
            schema
                .lines()
                .filter(|line| line.trim_start().starts_with("pub "))
                .count(),
            318
        );
        assert_eq!(schema.matches("Option<f64>").count(), 117);
        assert_eq!(schema.matches("Option<bool>").count(), 2);
        assert_eq!(schema.matches("Option<").count() - 119, 1);
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
        let raw = parse_epjson_str(&epjson).expect("real CP430 raw fixture");
        let typed = compile_raw_model(&raw)
            .model
            .expect("real CP430 typed fixture");
        let model = SimulationModel::from_typed(typed);
        let cache = precompute_schedule_cache(&model.typed, 1).expect("real CP430 schedule cache");
        let binding = bind_direct_zone_purchased_air_model(&model).expect("real CP430 binding");
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
        .expect("real CP430 scheduled coupling")
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
