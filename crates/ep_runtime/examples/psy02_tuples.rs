//! Input-only ordered PSY-02 dispatch through the actual instance-owned kernels.
//! One invocation owns one fresh property state. No reference outputs are read.

use ep_runtime::psychrometrics::{
    EnergyPlusPsychrometricCacheSlot as Slot, EnergyPlusPsychrometricFinalCaches,
    EnergyPlusPsychrometricFunction as Function, EnergyPlusPsychrometricStateSnapshot,
    EnergyPlusPsychrometricsState,
};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    io::Read,
};

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
        return Err("usage: psy02_tuples [INPUT.json] (results on stdout)".into());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&execute(&serde_json::from_str(&input)?)?)?
    );
    Ok(())
}
fn scalar(input: &Value) -> Result<f64, String> {
    if let Some(value) = input.as_f64() {
        return Ok(value);
    }
    match input.as_str() {
        Some("NaN") => Ok(f64::NAN),
        Some("+Infinity") => Ok(f64::INFINITY),
        Some("-Infinity") => Ok(f64::NEG_INFINITY),
        _ => Err("expected number or exact IEEE string".into()),
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
fn class(value: f64) -> &'static str {
    if value.is_nan() {
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
    }
}
fn state(s: EnergyPlusPsychrometricStateSnapshot) -> Value {
    json!({"iconv_tol":encoded(s.iconv_tol),"iconv_tol_bits":format!("{:016x}",s.iconv_tol.to_bits()),
        "last_patm":encoded(s.last_patm),"last_patm_bits":format!("{:016x}",s.last_patm.to_bits()),
        "last_t_boil":encoded(s.last_t_boil),"last_t_boil_bits":format!("{:016x}",s.last_t_boil.to_bits()),
        "press_save":encoded(s.press_save),"press_save_bits":format!("{:016x}",s.press_save.to_bits()),
        "t_sat_save":encoded(s.t_sat_save),"t_sat_save_bits":format!("{:016x}",s.t_sat_save.to_bits()),
        "warmup":s.warmup,"use_interpolation":s.use_interpolation})
}
fn slot(s: Slot) -> Value {
    let (mut row, value) = match s {
        Slot::Twb {
            index,
            i_tdb,
            i_w,
            i_pb,
            value,
        } => (
            json!({"kind":"Twb","index":index,"i_tdb":i_tdb,"i_w":i_w,"i_pb":i_pb}),
            value,
        ),
        Slot::Psat {
            index,
            i_tdb,
            value,
        } => (json!({"kind":"Psat","index":index,"i_tdb":i_tdb}), value),
        Slot::TsatPb {
            index,
            i_h,
            i_pb,
            value,
        } => (
            json!({"kind":"TsatPb","index":index,"i_h":i_h,"i_pb":i_pb}),
            value,
        ),
        Slot::TsatHPb {
            index,
            i_h,
            i_pb,
            value,
        } => (
            json!({"kind":"TsatHPb","index":index,"i_h":i_h,"i_pb":i_pb}),
            value,
        ),
    };
    row["value"] = encoded(value);
    row["value_bits"] = json!(format!("{:016x}", value.to_bits()));
    row
}
fn requested(function: Function, x: &[f64]) -> Value {
    let signed = |value: f64, shift: u32| (value.to_bits() as i64) >> shift;
    match function {
        Function::Twb => {
            json!({"i_tdb":x[0].to_bits()>>32,"i_w":x[1].to_bits()>>32,"i_pb":x[2].to_bits()>>32})
        }
        Function::Psat => json!({"i_tdb":signed(x[0],28)}),
        Function::TsatPb => json!({"i_pb":signed(x[0],28)}),
        Function::TsatHPb => json!({"i_h":signed(x[0],24),"i_pb":signed(x[1],24)}),
        _ => Value::Null,
    }
}
fn observed_slot(s: Option<Slot>, tags: &Value) -> Value {
    s.map_or(Value::Null, |s| {
        let mut s = slot(s);
        s["requested_tags"] = tags.clone();
        s
    })
}
fn caches(c: EnergyPlusPsychrometricFinalCaches) -> Value {
    json!({"Twb":c.twb.into_iter().map(slot).collect::<Vec<_>>(),"Psat":c.psat.into_iter().map(slot).collect::<Vec<_>>(),
        "TsatPb":c.tsat_pb.into_iter().map(slot).collect::<Vec<_>>(),"TsatHPb":c.tsat_h_pb.into_iter().map(slot).collect::<Vec<_>>()})
}
fn route(name: &str) -> Result<(Function, &'static [&'static str], &'static str), String> {
    use Function as F;
    Ok(match name {
        "PsyTwbFnTdbWPb" => (F::Twb, &["t_db_c", "w_kg_per_kg", "p_pa"], "degC"),
        "PsyTwbFnTdbWPb_raw" => (F::TwbRaw, &["t_db_c", "w_kg_per_kg", "p_pa"], "degC"),
        "PsyPsatFnTemp" => (F::Psat, &["t_db_c"], "Pa"),
        "PsyPsatFnTemp_raw" => (F::PsatRaw, &["t_db_c"], "Pa"),
        "PsyTsatFnPb" => (F::TsatPb, &["p_pa"], "degC"),
        "PsyTsatFnPb_raw" => (F::TsatPbRaw, &["p_pa"], "degC"),
        "PsyTsatFnHPb" => (F::TsatHPb, &["h_j_per_kg", "p_pa"], "degC"),
        "PsyTsatFnHPb_raw" => (F::TsatHPbRaw, &["h_j_per_kg", "p_pa"], "degC"),
        "PsyWFnTdbRhPb" => (F::WFromRh, &["t_db_c", "rh_fraction", "p_pa"], "kg/kg"),
        "PsyWFnTdbH" => (F::WFromH, &["t_db_c", "h_j_per_kg"], "kg/kg"),
        "PsyWFnTdbTwbPb" => (F::WFromTwb, &["t_db_c", "t_wb_c", "p_pa"], "kg/kg"),
        "PsyHFnTdbW" => (F::H, &["t_db_c", "w_kg_per_kg"], "J/kg"),
        _ => return Err(format!("unsupported function {name}")),
    })
}
fn resolve(value: &Value, previous: &BTreeMap<u64, f64>) -> Result<Value, String> {
    if let Some(object) = value.as_object() {
        if object.contains_key("from_call") {
            if object.len() != 1 {
                return Err("from_call object has extra fields".into());
            }
            let index = object["from_call"]
                .as_u64()
                .ok_or("from_call must be u64")?;
            return previous
                .get(&index)
                .copied()
                .map(encoded)
                .ok_or("unresolved or forward from_call".into());
        }
        let mut out = serde_json::Map::new();
        for (key, value) in object {
            out.insert(key.clone(), resolve(value, previous)?);
        }
        return Ok(Value::Object(out));
    }
    Ok(value.clone())
}
fn execute(request: &Value) -> Result<Value, String> {
    let object = request.as_object().ok_or("request must be object")?;
    if object.len() != 2 || request["schema"] != "psy02-tuples.v1" {
        return Err("expected only schema psy02-tuples.v1 and calls".into());
    }
    let calls = request["calls"].as_array().ok_or("calls must be array")?;
    let mut owner = EnergyPlusPsychrometricsState::default();
    let initial_state = state(owner.snapshot());
    let mut previous = BTreeMap::new();
    let mut indices = BTreeSet::new();
    let mut results = Vec::new();
    for call in calls {
        let fields = call.as_object().ok_or("call must be object")?;
        for key in fields.keys() {
            if ![
                "call_index",
                "sequence_id",
                "function",
                "inputs",
                "context",
                "state_overrides",
                "suppress_warnings",
            ]
            .contains(&key.as_str())
            {
                return Err(format!("unexpected call field {key}"));
            }
        }
        let index = call["call_index"]
            .as_u64()
            .ok_or("call_index must be u64")?;
        if !indices.insert(index) {
            return Err("duplicate call_index".into());
        }
        if call["sequence_id"].as_str().is_none_or(str::is_empty) || !call["context"].is_object() {
            return Err("sequence_id/context invalid".into());
        }
        let name = call["function"].as_str().ok_or("function must be string")?;
        let inputs = resolve(&call["inputs"], &previous)?;
        if let Some(overrides) = call.get("state_overrides") {
            let controls = overrides
                .as_object()
                .ok_or("state_overrides must be object")?;
            if controls.keys().any(|k| k != "warmup" && k != "iconv_tol") {
                return Err("unsupported state override".into());
            }
            let warmup = controls
                .get("warmup")
                .map(|v| v.as_bool().ok_or("warmup must be bool"))
                .transpose()?;
            let tol = controls.get("iconv_tol").map(scalar).transpose()?;
            if tol.is_some_and(|t| !t.is_finite() || t < 0.0) {
                return Err("iconv_tol must be nonnegative finite".into());
            }
            owner.set_controls(warmup, tol);
        }
        if call
            .get("suppress_warnings")
            .is_some_and(|v| !v.is_boolean())
        {
            return Err("suppress_warnings must be bool".into());
        }
        let mut output = call.clone();
        output["resolved_inputs"] = inputs.clone();
        if name == "General::Iterate" {
            output["status"] = json!("unsupported_source_only");
            output["reason"] = json!(
                "The frozen diagnostic route is original-source-only; selected property solvers use the actual shared General::Iterate port internally."
            );
            results.push(output);
            continue;
        }
        let (function, names, unit) = route(name)?;
        let fields = inputs.as_object().ok_or("inputs must be object")?;
        if fields.len() != names.len() || names.iter().any(|n| !fields.contains_key(*n)) {
            return Err(format!("incorrect inputs for {name}"));
        }
        let x = names
            .iter()
            .map(|n| scalar(&fields[*n]))
            .collect::<Result<Vec<_>, _>>()?;
        let op = owner.evaluate(function, &x)?;
        let tags = requested(function, &x);
        let before = observed_slot(op.cache_before, &tags);
        let hit = tags.as_object().map_or(Value::Null, |tags| {
            json!(tags.iter().all(|(key, value)| before[key] == *value))
        });
        output["value"] = encoded(op.result);
        output["value_bits"] = json!(format!("{:016x}", op.result.to_bits()));
        output["value_class"] = json!(class(op.result));
        output["unit"] = json!(unit);
        output["state_before"] = state(op.state_before);
        output["state_after"] = state(op.state_after);
        output["cache_before"] = before;
        output["cache_after"] = observed_slot(op.cache_after, &tags);
        output["cache_hit"] = hit;
        previous.insert(index, op.result);
        results.push(output);
    }
    Ok(
        json!({"schema":"psy02-results.v1","calls":results,"initial_state":initial_state,"final_state":state(owner.snapshot()),"final_caches":caches(owner.final_caches()),
        "source_warning_lifecycle_is_paired":false,"internal_local_iteration_count_observed":false}),
    )
}
