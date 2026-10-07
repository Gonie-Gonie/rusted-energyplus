//! Bounded PSY-01 tuple execution using the actual runtime kernels.
//!
//! Reads `psy01-tuples.v1` JSON from a single path or stdin and writes results
//! to stdout. It accepts inputs and call context, never expected outputs.

use ep_runtime::psychrometrics::{
    EnergyPlusCpAirCache, EnergyPlusCpAirCacheState, energyplus_psy_h_fn_tdb_w,
    energyplus_psy_h_fn_tdb_w_fast, energyplus_psy_rho_air_fn_pb_tdb_w,
    energyplus_psy_rho_air_fn_pb_tdb_w_fast, energyplus_psy_tdb_fn_h_w, energyplus_psy_w_fn_tdb_h,
};
use serde_json::{Map, Value, json};
use std::collections::BTreeMap;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let input = if let Some(path) = args.next() {
        std::fs::read_to_string(path)?
    } else {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;
        input
    };
    if args.next().is_some() {
        return Err("usage: psy01_tuples [INPUT.json] (results on stdout)".into());
    }
    let output = execute(&serde_json::from_str(&input)?)?;
    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

fn fields<'a>(value: &'a Value, names: &[&str]) -> Result<&'a Map<String, Value>, String> {
    let object = value.as_object().ok_or("expected JSON object")?;
    if object.len() != names.len() || names.iter().any(|name| !object.contains_key(*name)) {
        return Err(format!(
            "expected exactly these fields: {}",
            names.join(", ")
        ));
    }
    Ok(object)
}

fn scalar(value: &Value) -> Result<f64, String> {
    if let Some(number) = value.as_f64() {
        return Ok(number);
    }
    match value.as_str() {
        Some("NaN") => Ok(f64::NAN),
        Some("+Infinity") => Ok(f64::INFINITY),
        Some("-Infinity") => Ok(f64::NEG_INFINITY),
        _ => Err("expected number or NaN/+Infinity/-Infinity string".into()),
    }
}

fn encoded(value: f64) -> Value {
    if value.is_nan() {
        json!("NaN")
    } else if value == f64::INFINITY {
        json!("+Infinity")
    } else if value == f64::NEG_INFINITY {
        json!("-Infinity")
    } else {
        json!(value)
    }
}

fn cache_state(state: EnergyPlusCpAirCacheState) -> Value {
    json!({
        "dw_save": encoded(state.humidity_ratio_kg_per_kg),
        "cpa_save": encoded(state.specific_heat_j_per_kg_k),
        "dw_save_bits": format!("{:016x}", state.humidity_ratio_kg_per_kg.to_bits()),
        "cpa_save_bits": format!("{:016x}", state.specific_heat_j_per_kg_k.to_bits()),
    })
}

fn execute(input: &Value) -> Result<Value, String> {
    let request = fields(input, &["schema", "calls"])?;
    if request["schema"] != "psy01-tuples.v1" {
        return Err("expected schema psy01-tuples.v1".into());
    }
    let calls = request["calls"]
        .as_array()
        .ok_or("calls must be an array")?;
    if calls.is_empty() {
        return Err("calls must be nonempty".into());
    }
    let mut previous = BTreeMap::<u64, (String, f64)>::new();
    let mut cache = EnergyPlusCpAirCache::default();
    let mut results = Vec::with_capacity(calls.len());
    for call in calls {
        let object = fields(
            call,
            &["sequence_id", "call_index", "function", "inputs", "context"],
        )?;
        if object["sequence_id"]
            .as_str()
            .is_none_or(|id| id.is_empty())
            || !object["context"].is_object()
        {
            return Err("sequence_id must be nonempty and context an object".into());
        }
        let index = object["call_index"]
            .as_u64()
            .ok_or("call_index must be u64")?;
        if previous.contains_key(&index) {
            return Err(format!("duplicate call_index {index}"));
        }
        let function = object["function"]
            .as_str()
            .ok_or("function must be string")?;
        let input_names: &[&str] = match function {
            "PsyCpAirFnW" | "PsyCpAirFnW_fast" => &["w_kg_per_kg"],
            "PsyRhoAirFnPbTdbW" | "PsyRhoAirFnPbTdbW_fast" => &["p_pa", "t_db_c", "w_kg_per_kg"],
            "PsyHFnTdbW" | "PsyHFnTdbW_fast" => &["t_db_c", "w_kg_per_kg"],
            "PsyTdbFnHW" => &["h_j_per_kg", "w_kg_per_kg"],
            "PsyWFnTdbH" => &["t_db_c", "h_j_per_kg"],
            _ => return Err(format!("unsupported function {function}")),
        };
        let inputs = fields(&object["inputs"], input_names)?;
        let mut resolved = BTreeMap::<&str, f64>::new();
        for &name in input_names {
            let value = if name == "h_j_per_kg" && inputs[name].is_object() {
                let reference = fields(&inputs[name], &["from_call"])?["from_call"]
                    .as_u64()
                    .ok_or("from_call must be u64")?;
                let (source_function, source_value) = previous
                    .get(&reference)
                    .ok_or("from_call must refer to an earlier call")?;
                if !matches!(source_function.as_str(), "PsyHFnTdbW" | "PsyHFnTdbW_fast") {
                    return Err("from_call must refer to an enthalpy output".into());
                }
                *source_value
            } else {
                scalar(&inputs[name])?
            };
            resolved.insert(name, value);
        }
        let w = || resolved["w_kg_per_kg"];
        // Keep source assertions enabled and reject invalid fast tuples even
        // in release builds. Normal kernels deliberately retain IEEE behavior.
        if function.ends_with("_fast") && (w().is_nan() || w() < 1.0e-5) {
            return Err(format!("{function} requires W >= 1.0e-5"));
        }
        let before = match function {
            "PsyCpAirFnW" => Some(cache.normal_state()),
            "PsyCpAirFnW_fast" => Some(cache.fast_state()),
            _ => None,
        };
        let (value, unit) = match function {
            "PsyCpAirFnW" => (cache.psy_cp_air_fn_w(w()), "J/(kg*K)"),
            "PsyCpAirFnW_fast" => (cache.psy_cp_air_fn_w_fast(w()), "J/(kg*K)"),
            "PsyRhoAirFnPbTdbW" => (
                energyplus_psy_rho_air_fn_pb_tdb_w(resolved["p_pa"], resolved["t_db_c"], w()),
                "kg/m3",
            ),
            "PsyRhoAirFnPbTdbW_fast" => (
                energyplus_psy_rho_air_fn_pb_tdb_w_fast(resolved["p_pa"], resolved["t_db_c"], w()),
                "kg/m3",
            ),
            "PsyHFnTdbW" => (energyplus_psy_h_fn_tdb_w(resolved["t_db_c"], w()), "J/kg"),
            "PsyHFnTdbW_fast" => (
                energyplus_psy_h_fn_tdb_w_fast(resolved["t_db_c"], w()),
                "J/kg",
            ),
            "PsyTdbFnHW" => (
                energyplus_psy_tdb_fn_h_w(resolved["h_j_per_kg"], w()),
                "degC",
            ),
            "PsyWFnTdbH" => (
                energyplus_psy_w_fn_tdb_h(resolved["t_db_c"], resolved["h_j_per_kg"]),
                "kg/kg",
            ),
            _ => return Err(format!("unsupported function {function}")),
        };
        let cache_hit = before.map(|saved| saved.humidity_ratio_kg_per_kg == w());
        let mut output = object.clone();
        output.insert(
            "resolved_inputs".into(),
            Value::Object(
                resolved
                    .into_iter()
                    .map(|(name, number)| (name.into(), encoded(number)))
                    .collect(),
            ),
        );
        output.insert("value".into(), encoded(value));
        output.insert(
            "value_bits".into(),
            json!(format!("{:016x}", value.to_bits())),
        );
        output.insert(
            "value_class".into(),
            json!(if value.is_nan() {
                "nan"
            } else if value == f64::INFINITY {
                "+infinity"
            } else if value == f64::NEG_INFINITY {
                "-infinity"
            } else {
                "finite"
            }),
        );
        output.insert("unit".into(), json!(unit));
        if let (Some(before), Some(cache_hit)) = (before, cache_hit) {
            let after = if function.ends_with("_fast") {
                cache.fast_state()
            } else {
                cache.normal_state()
            };
            output.insert("cache_hit".into(), json!(cache_hit));
            output.insert("cache_before".into(), cache_state(before));
            output.insert("cache_after".into(), cache_state(after));
        }
        previous.insert(index, (function.into(), value));
        results.push(Value::Object(output));
    }
    Ok(json!({"schema":"psy01-results.v1", "calls":results}))
}

#[cfg(test)]
mod tests {
    use super::execute;
    use serde_json::{Value, json};

    fn call(index: u64, function: &str, inputs: Value) -> Value {
        json!({"sequence_id":"unit", "call_index":index, "function":function,
            "inputs":inputs, "context":{"caller":"unit test"}})
    }

    #[test]
    fn rejects_reference_outputs_unknown_functions_and_invalid_fast_inputs() {
        for function in [
            "PsyCpAirFnW_fast",
            "PsyHFnTdbW_fast",
            "PsyRhoAirFnPbTdbW_fast",
        ] {
            let mut inputs = json!({"w_kg_per_kg":"NaN"});
            if function != "PsyCpAirFnW_fast" {
                inputs["t_db_c"] = json!(20.0);
            }
            if function == "PsyRhoAirFnPbTdbW_fast" {
                inputs["p_pa"] = json!(101_325.0);
            }
            assert!(
                execute(&json!({"schema":"psy01-tuples.v1", "calls":[call(0,function,inputs)]}))
                    .is_err()
            );
        }
        let mut injected = call(0, "PsyCpAirFnW", json!({"w_kg_per_kg":0.008}));
        injected["expected"] = json!(1019.0);
        assert!(execute(&json!({"schema":"psy01-tuples.v1","calls":[injected]})).is_err());
        assert!(
            execute(&json!({"schema":"psy01-tuples.v1","calls":[call(0,"Invented",json!({}))]}))
                .is_err()
        );
    }

    #[test]
    fn round_trip_uses_its_own_preceding_enthalpy_and_rejects_false_dependencies()
    -> Result<(), String> {
        let forward = call(0, "PsyHFnTdbW", json!({"t_db_c":24.0,"w_kg_per_kg":0.0}));
        let inverse = call(
            1,
            "PsyTdbFnHW",
            json!({"h_j_per_kg":{"from_call":0},"w_kg_per_kg":0.0}),
        );
        let humidity_inverse = call(
            2,
            "PsyWFnTdbH",
            json!({"t_db_c":24.0,"h_j_per_kg":{"from_call":0}}),
        );
        let result = execute(
            &json!({"schema":"psy01-tuples.v1","calls":[forward.clone(),inverse.clone(),humidity_inverse]}),
        )?;
        assert_eq!(
            result["calls"][1]["resolved_inputs"]["h_j_per_kg"],
            result["calls"][0]["value"]
        );
        assert!(
            result["calls"][1]["value"]
                .as_f64()
                .is_some_and(|value| (value - 24.0).abs() < 1.0e-10)
        );
        assert_eq!(
            result["calls"][2]["resolved_inputs"]["h_j_per_kg"],
            result["calls"][0]["value"]
        );
        assert_eq!(result["calls"][2]["unit"], "kg/kg");
        assert!(
            result["calls"][2]["value"]
                .as_f64()
                .is_some_and(|value| value.is_finite() && value > 0.0)
        );
        assert!(
            execute(&json!({"schema":"psy01-tuples.v1","calls":[inverse.clone(),forward]}))
                .is_err()
        );
        assert!(execute(&json!({"schema":"psy01-tuples.v1","calls":[call(0,"PsyCpAirFnW",json!({"w_kg_per_kg":0.008})),inverse]})).is_err());
        Ok(())
    }

    #[test]
    fn sequence_ids_do_not_reset_the_observable_cache() -> Result<(), String> {
        let mut calls = vec![
            call(0, "PsyCpAirFnW", json!({"w_kg_per_kg":-100.0})),
            call(1, "PsyCpAirFnW", json!({"w_kg_per_kg":0.008})),
            call(2, "PsyCpAirFnW", json!({"w_kg_per_kg":-100.0})),
        ];
        calls[2]["sequence_id"] = json!("second-sequence");
        let result = execute(&json!({"schema":"psy01-tuples.v1","calls":calls}))?;
        assert_eq!(result["calls"][0]["value"], -100.0);
        assert_eq!(result["calls"][0]["cache_hit"], true);
        assert!(
            result["calls"][2]["value"]
                .as_f64()
                .is_some_and(|value| value > 1000.0)
        );
        assert_eq!(result["calls"][2]["cache_hit"], false);
        Ok(())
    }
}
