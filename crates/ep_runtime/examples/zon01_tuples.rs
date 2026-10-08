//! Input-only observation of the existing public heat-balance shell initializer.
//! This baseline has no original constructor/bulk/begin/guard implementation.
//! Existing three-slot histories are explicit partial source-key projections.

use ep_model::{
    AutoOrNumber, InsideSurfaceConvectionAlgorithm, NormalizedName,
    OutsideSurfaceConvectionAlgorithm, Point3, SimulationModel, TypedModel, Zone,
    ZoneConvectionAlgorithm, ZoneId,
};
use ep_runtime::heat_balance::{ZoneHeatBalanceState, initialize_heat_balance_state};
use serde_json::{Value, json};
use std::{collections::BTreeSet, error::Error, io::Read};

const SCALARS: [&str; 18] = [
    "MAT",
    "ZT",
    "ZTAV",
    "XMPT",
    "TMX",
    "TM2",
    "airHumRat",
    "airHumRatAvg",
    "WTimeMinusP",
    "W1",
    "WMX",
    "WM2",
    "airHumRatTemp",
    "tempIndLoad",
    "tempDepLoad",
    "airRelHum",
    "AirPowerCap",
    "T1",
];
const ARRAYS: [&str; 6] = [
    "XMAT",
    "DSXMAT",
    "ZTM",
    "WPrevZoneTS",
    "DSWPrevZoneTS",
    "WPrevZoneTSTemp",
];
const UNIMPLEMENTED: [&str; 16] = [
    "ZT",
    "XMPT",
    "TMX",
    "TM2",
    "ZTM",
    "WPrevZoneTSTemp",
    "WTimeMinusP",
    "W1",
    "WMX",
    "WM2",
    "airHumRatTemp",
    "tempIndLoad",
    "tempDepLoad",
    "airRelHum",
    "AirPowerCap",
    "T1",
];

fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}
fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value == f64::INFINITY {
        "positive_infinity"
    } else if value == f64::NEG_INFINITY {
        "negative_infinity"
    } else if value == 0.0 {
        if value.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value": if value.is_finite() { Some(value) } else { None },
        "value_bits":bits(value), "value_class":class})
}
fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value[key]
        .as_str()
        .ok_or_else(|| format!("expected string {key}"))
}
fn own_number(value: &Value, token: &Value) -> Result<f64, String> {
    let number = value.as_f64().ok_or("expected numerical input")?;
    if !number.is_finite() || token.as_str() != Some(bits(number).as_str()) {
        return Err(
            "independently parsed finite binary64 input differs from authoritative bits".into(),
        );
    }
    Ok(number)
}
fn validate_operation(operation: &Value) -> Result<(), String> {
    text(operation, "operation_id")?;
    let name = text(operation, "operation")?;
    if !matches!(
        name,
        "snapshot"
            | "seed_zone_state"
            | "bulk_reconstruct_and_current_w_seed"
            | "bare_begin_environment"
            | "guarded_init_zone_air_setpoints"
    ) {
        return Err("unknown frozen operation".into());
    }
    if let Some(number) = operation.get("out_hum_rat") {
        own_number(number, &operation["out_hum_rat_bits"])?;
    } else if operation.get("out_hum_rat_bits").is_some() {
        return Err("unbound external-W bits".into());
    }
    if operation
        .get("begin_environment")
        .is_some_and(|v| !v.is_boolean())
    {
        return Err("expected boolean environment input".into());
    }
    if name == "seed_zone_state" {
        let fields = operation["fields"]
            .as_object()
            .ok_or("expected seed fields")?;
        let tokens = operation["input_field_bits"]
            .as_object()
            .ok_or("expected seed bit fields")?;
        if fields.len() != tokens.len() || fields.keys().any(|k| !tokens.contains_key(k)) {
            return Err("seed field/bit keys differ".into());
        }
        for (key, value) in fields {
            if SCALARS.contains(&key.as_str()) {
                own_number(value, &tokens[key])?;
            } else if ARRAYS.contains(&key.as_str()) {
                let values = value.as_array().ok_or("expected source four-slot input")?;
                let bits = tokens[key].as_array().ok_or("expected four bit tokens")?;
                if values.len() != 4 || bits.len() != 4 {
                    return Err("source input slot count differs".into());
                }
                for (number, token) in values.iter().zip(bits) {
                    own_number(number, token)?;
                }
            } else {
                return Err(format!("unknown source input field {key}"));
            }
        }
    }
    Ok(())
}
fn model() -> SimulationModel {
    let mut typed = TypedModel::default();
    typed.zones.push(Zone {
        id: ZoneId(0),
        name: NormalizedName::new("PREPARED-ONE-ZONE"),
        direction_of_relative_north_deg: 0.0,
        origin: Point3 {
            x_m: 0.0,
            y_m: 0.0,
            z_m: 0.0,
        },
        zone_type: 1,
        multiplier: 1,
        list_multiplier: 1,
        list_group: None,
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::Value(1.0),
        floor_area: AutoOrNumber::AutoCalculate,
        inside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            InsideSurfaceConvectionAlgorithm::Tarp,
        ),
        outside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            OutsideSurfaceConvectionAlgorithm::Doe2,
        ),
        is_part_of_total_floor_area: true,
        is_nominal_controlled: false,
        linked_outdoor_air_node: None,
        spaces: Vec::new(),
    });
    SimulationModel::from_typed(typed)
}
fn observe(zone: &ZoneHeatBalanceState) -> Value {
    json!({"phase":"legacy_shell_return","zone":{"id":zone.zone_id.0,"name":zone.zone_name},
        "legacy_fields":{
            "mean_air_temperature_c":scalar(zone.mean_air_temperature_c),
            "zone_timestep_average_air_temperature_c":scalar(zone.zone_timestep_average_air_temperature_c),
            "air_humidity_ratio":scalar(zone.air_humidity_ratio),
            "zone_timestep_average_air_humidity_ratio":scalar(zone.zone_timestep_average_air_humidity_ratio),
            "previous_mean_air_temperatures_c":zone.previous_mean_air_temperatures_c.map(scalar),
            "previous_system_mean_air_temperatures_c":zone.previous_system_mean_air_temperatures_c.map(scalar),
            "previous_air_humidity_ratios":zone.previous_air_humidity_ratios.map(scalar),
            "previous_system_air_humidity_ratios":zone.previous_system_air_humidity_ratios.map(scalar)},
        "source_correspondence":{
            "MAT":"mean_air_temperature_c","ZTAV":"zone_timestep_average_air_temperature_c",
            "airHumRat":"air_humidity_ratio","airHumRatAvg":"zone_timestep_average_air_humidity_ratio",
            "XMAT_first_three":"previous_mean_air_temperatures_c",
            "DSXMAT_first_three":"previous_system_mean_air_temperatures_c",
            "WPrevZoneTS_first_three":"previous_air_humidity_ratios",
            "DSWPrevZoneTS_first_three":"previous_system_air_humidity_ratios"},
        "legacy_diagnostic_snapshots":{
            "third_order_temp_independent_load_w":scalar(zone.zone_air_temperature_coefficients.third_order_temp_independent_load_w),
            "third_order_temp_dependent_load_w_per_k":scalar(zone.zone_air_temperature_coefficients.third_order_temp_dependent_load_w_per_k),
            "air_power_cap_w_per_k":scalar(zone.zone_air_temperature_coefficients.air_power_cap_w_per_k)},
        "existing_diagnostic_snapshots_not_validated_as_source_initializer_owner":{
            "tempIndLoad":"third_order_temp_independent_load_w",
            "tempDepLoad":"third_order_temp_dependent_load_w_per_k",
            "AirPowerCap":"air_power_cap_w_per_k"},
        "unimplemented_source_fields":UNIMPLEMENTED,
        "unimplemented_means_source_initializer_ownership":true,
        "unimplemented_source_slots":["XMAT[3]","DSXMAT[3]","WPrevZoneTS[3]","DSWPrevZoneTS[3]"],
        "source_flags":null,"source_state":null,
        "original_constructor_bulk_or_begin_phase_claimed":false})
}
fn seed(zone: &mut ZoneHeatBalanceState, operation: &Value) -> Result<Value, String> {
    let fields = operation["fields"].as_object().ok_or("expected fields")?;
    let mut applied = Vec::new();
    let mut unsupported = Vec::new();
    let mut fourth_slots = Vec::new();
    for (key, value) in fields {
        let scalar_target = match key.as_str() {
            "MAT" => Some(&mut zone.mean_air_temperature_c),
            "ZTAV" => Some(&mut zone.zone_timestep_average_air_temperature_c),
            "airHumRat" => Some(&mut zone.air_humidity_ratio),
            "airHumRatAvg" => Some(&mut zone.zone_timestep_average_air_humidity_ratio),
            _ => None,
        };
        if let Some(target) = scalar_target {
            *target = own_number(value, &operation["input_field_bits"][key])?;
            applied.push(key.clone());
            continue;
        }
        let array_target = match key.as_str() {
            "XMAT" => Some(&mut zone.previous_mean_air_temperatures_c),
            "DSXMAT" => Some(&mut zone.previous_system_mean_air_temperatures_c),
            "WPrevZoneTS" => Some(&mut zone.previous_air_humidity_ratios),
            "DSWPrevZoneTS" => Some(&mut zone.previous_system_air_humidity_ratios),
            _ => None,
        };
        if let Some(target) = array_target {
            for (index, element) in target.iter_mut().enumerate() {
                *element = own_number(&value[index], &operation["input_field_bits"][key][index])?;
            }
            applied.push(format!("{key}[0..3)"));
            fourth_slots.push(format!("{key}[3]"));
        } else {
            unsupported.push(key.clone());
        }
    }
    Ok(
        json!({"applied_existing_field_inputs":applied,"unconsumed_source_fields":unsupported,
        "unconsumed_source_slots":fourth_slots}),
    )
}
fn execute(request: &Value) -> Result<Value, String> {
    if request["schema"].as_str() != Some("zon01-helper-cases.v1")
        || request["expected_values_supplied"].as_bool() != Some(false)
    {
        return Err("expected frozen input-only ZON-01 request".into());
    }
    let sequences = request["sequences"]
        .as_array()
        .ok_or("expected sequences")?;
    let mut ids = BTreeSet::new();
    let mut results = Vec::new();
    for sequence in sequences {
        let id = text(sequence, "sequence_id")?;
        if !ids.insert(id) {
            return Err("duplicate sequence_id".into());
        }
        let mut state = initialize_heat_balance_state(&model(), 23.0).map_err(|e| e.to_string())?;
        let zone = state
            .zones
            .get_mut(0)
            .ok_or("existing initializer returned no zone")?;
        let initial = observe(zone);
        let mut operations = Vec::new();
        let mut operation_ids = BTreeSet::new();
        for operation in sequence["operations"]
            .as_array()
            .ok_or("expected operations")?
        {
            validate_operation(operation)?;
            let operation_id = text(operation, "operation_id")?;
            if !operation_ids.insert(operation_id) {
                return Err("duplicate operation_id".into());
            }
            let name = text(operation, "operation")?;
            let before = observe(zone);
            let (status, seed_projection) = match name {
                "snapshot" => ("legacy_shell_snapshot", Value::Null),
                "seed_zone_state" => ("baseline_partial_seed", seed(zone, operation)?),
                _ => ("unsupported_source_operation", Value::Null),
            };
            operations.push(
                json!({"operation_id":operation_id,"operation":name,"input":operation,
                "status":status,"legacy_before":before,"legacy_after":observe(zone),
                "seed_projection":seed_projection,"numeric_input_identity_checked":true,
                "out_hum_rat_consumed":false,"begin_environment_consumed":false,
                "source_state":null,"source_flags":null,"source_operation_executed":false}),
            );
        }
        results.push(json!({"sequence_id":id,"kind":text(sequence,"kind")?,"input":sequence,
            "status":"baseline_partial","route":"ep_runtime::heat_balance::initialize_heat_balance_state",
            "legacy_shell_return":initial,"operations":operations,
            "source_state":null,"source_flags":null,"unimplemented_source_fields":UNIMPLEMENTED,
            "unavailable_source_phases":["constructor","allocated_zone_constructor","prepared_manager_state","before_inputs","before","after"],
            "environment_guard_implemented":false,"four_slot_source_arrays_implemented":false,
            "physics_executed":false,"original_parser_admission_claimed":false}));
    }
    Ok(
        json!({"schema":"zon01-helper-results.v1","implementation_stage":"baseline_partial",
        "sequences":results,"factory":{"zone_count":1,"zone_volume_m3":scalar(1.0),
            "initial_zone_air_temperature_c":scalar(23.0),"surfaces":0,
            "weather_seed_called":false,"prepared_input_not_CON_model":true},
        "reference_outputs_supplied_to_Rust":false,"expected_values_supplied":false,
        "physics_executed":false,"gates_updated":false}),
    )
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let input = if let Some(path) = args.next() {
        std::fs::read_to_string(path)?
    } else {
        let mut data = String::new();
        std::io::stdin().read_to_string(&mut data)?;
        data
    };
    if args.next().is_some() {
        return Err("usage: zon01_tuples [INPUT.json]".into());
    }
    println!(
        "{}",
        serde_json::to_string(&execute(&serde_json::from_str(&input)?)?)?
    );
    Ok(())
}
