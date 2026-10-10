//! Existing public fixed-CON loader/compiler/initialization observations only.
#[path = "../../ep_runtime/examples/clk02_probe_support/digest.rs"]
mod digest;
#[path = "ctf01_existing_support/fields.rs"]
mod fields;

use ep_model::SimulationModel;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}
fn text(value: &Value) -> Result<&str> {
    value.as_str().ok_or_else(|| "typed string required".into())
}
fn rows(value: &Value) -> Result<&[Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "typed array required".into())
}
fn keys(value: &Value, expected: &[&str]) -> Result<()> {
    let object = value.as_object().ok_or("typed object required")?;
    require(
        object.len() == expected.len() && expected.iter().all(|key| object.contains_key(*key)),
        "exact input-only fields required",
    )
}
fn bound(root: &Path, binding: &Value) -> Result<(PathBuf, Vec<u8>)> {
    let relative = Path::new(text(&binding["path"])?);
    require(
        relative.is_relative(),
        "relative input/source path required",
    )?;
    let path = root.join(relative).canonicalize()?;
    require(
        path.starts_with(root) && path != root && path.is_file(),
        "contained regular input/source required",
    )?;
    require(
        std::fs::metadata(&path)?.len() <= 64 * 1024 * 1024,
        "bounded input/source file required",
    )?;
    let bytes = std::fs::read(&path)?;
    require(
        digest::sha256(&bytes) == text(&binding["sha256"])?,
        "bound input/source SHA differs",
    )?;
    for key in ["bytes", "size_bytes"] {
        if let Some(size) = binding.get(key) {
            require(
                size.as_u64() == Some(u64::try_from(bytes.len())?),
                "bound byte size differs",
            )?;
        }
    }
    Ok((path, bytes))
}
fn source_pins(root: &Path, supplied: &Value, expected: &[(&str, &str)]) -> Result<()> {
    let supplied = rows(supplied)?;
    require(
        supplied.len() == expected.len(),
        "source inventory length differs",
    )?;
    for (binding, (path, sha)) in supplied.iter().zip(expected) {
        require(
            text(&binding["path"])? == *path && text(&binding["sha256"])? == *sha,
            "source identity/order differs from reviewed baseline",
        )?;
        bound(root, binding)?;
    }
    Ok(())
}
fn observer_pins(root: &Path, supplied: &Value) -> Result<()> {
    let supplied = rows(supplied)?;
    require(
        supplied.len() == fields::OBSERVER_PATHS.len(),
        "observer source count differs",
    )?;
    for (binding, expected_path) in supplied.iter().zip(fields::OBSERVER_PATHS) {
        require(
            text(&binding["path"])? == *expected_path,
            "observer source path/order differs",
        )?;
        bound(root, binding)?;
    }
    Ok(())
}
fn model_case(root: &Path, row: &Value, initial: f64) -> Result<Value> {
    let mut out = json!({"id":row["id"],"input":row["input"],"epjson_input":row["epjson_input"],
        "conversion_available":row["conversion_available"],"conversion_proof_bindings":row["conversion_proof_bindings"],
        "conversion_unavailable_reason":row["conversion_unavailable_reason"],
        "conversion_proof_semantics_independently_validated":false,
        "loader":{"invoked":false,"status":"not-invoked-without-converted-input"},
        "compiler":{"invoked":false,"status":"not-invoked-without-raw-model"},
        "initialization":{"invoked":false,"status":"not-invoked-without-typed-model"},
        "typed_owners":null,"initialized_state_context":null,
        "selected_CTF01_owners":fields::unavailable(),"Native_or_expected_values_read":false});
    if row["conversion_available"] == false {
        return Ok(out);
    }
    let (idf, _) = bound(root, &row["input"])?;
    let (epjson, _) = bound(root, &row["epjson_input"])?;
    let raw = match ep_raw_model::load_epjson_file_with_idf_order(&epjson, &idf) {
        Ok(raw) => {
            out["loader"] = json!({"invoked":true,"status":"source-returned"});
            raw
        }
        Err(error) => {
            out["loader"] = json!({"invoked":true,"status":"source-error-returned","message":error.to_string()});
            return Ok(out);
        }
    };
    let compiled = ep_compiler::compile_raw_model(&raw);
    out["compiler"] = json!({"invoked":true,"status":"source-returned","model_available":compiled.model.is_some(),
        "has_errors":compiled.has_errors(),"raw_object_count":compiled.report.raw_object_count,
        "typed_object_count":compiled.report.typed_object_count,
        "completed_stages":format!("{:?}",compiled.report.completed_stages),
        "ordered_diagnostics":format!("{:?}",compiled.report.diagnostics),
        "defaults_applied":format!("{:?}",compiled.report.defaults_applied)});
    let Some(typed) = compiled.model else {
        return Ok(out);
    };
    out["typed_owners"] = fields::typed(&typed);
    let simulation = SimulationModel::from_typed(typed);
    match ep_runtime::heat_balance::initialize_heat_balance_state(&simulation, initial) {
        Ok(state) => {
            out["initialization"] = json!({"invoked":true,"status":"source-returned",
                "initial_zone_air_temperature_c":fields::scalar(initial),"external_CTF_rows_supplied":false});
            out["initialized_state_context"] = fields::state(&state);
        }
        Err(error) => {
            out["initialization"] = json!({"invoked":true,"status":"source-error-returned",
            "message":error.to_string(),"initial_zone_air_temperature_c":fields::scalar(initial),
            "external_CTF_rows_supplied":false})
        }
    }
    Ok(out)
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
fn run() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let request_path = PathBuf::from(
        args.next()
            .ok_or("one input-only observer request required")?,
    )
    .canonicalize()?;
    require(args.next().is_none(), "exactly one request required")?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("repository root unavailable")?
        .canonicalize()?;
    require(
        request_path.starts_with(&root),
        "contained request required",
    )?;
    let original = std::fs::read(&request_path)?;
    let request: Value = serde_json::from_slice(&original)?;
    keys(
        &request,
        &[
            "schema",
            "fixed_scope",
            "model_cases",
            "initial_zone_air_temperature_c",
            "observer_sources",
            "actual_public_API_sources",
            "frozen_before_execution",
            "expected_values_supplied",
            "Native_results_supplied",
        ],
    )?;
    require(
        request["schema"] == "ctf01-existing-observer-request.v1"
            && request["frozen_before_execution"] == true
            && request["expected_values_supplied"] == false
            && request["Native_results_supplied"] == false,
        "frozen input-only observer request required",
    )?;
    require(
        request["fixed_scope"]["path"] == "energyplus_porting_plan/contracts/scope.json"
            && request["fixed_scope"]["sha256"]
                == "b88a28d207400744a108a2aae704f8067fd6b6fd30a5812df53cd8094e94a879",
        "unchanged fixed CON scope required",
    )?;
    let (_, scope_bytes) = bound(&root, &request["fixed_scope"])?;
    let scope: Value = serde_json::from_slice(&scope_bytes)?;
    let cases = rows(&request["model_cases"])?;
    let admitted = rows(&scope["cases"])?;
    require(
        admitted.len() == 15 && cases.len() == admitted.len(),
        "ordered fixed fifteen cases required",
    )?;
    keys(&request["initial_zone_air_temperature_c"], &["bits"])?;
    let initial_bits = text(&request["initial_zone_air_temperature_c"]["bits"])?;
    require(
        initial_bits.len() == 16
            && initial_bits
                .bytes()
                .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c)),
        "exact lowerhex binary64 caller required",
    )?;
    let initial = f64::from_bits(u64::from_str_radix(initial_bits, 16)?);
    require(
        initial.is_finite(),
        "finite declared initialization caller required",
    )?;
    observer_pins(&root, &request["observer_sources"])?;
    source_pins(
        &root,
        &request["actual_public_API_sources"],
        fields::API_SOURCES,
    )?;
    let mut bindings = vec![request["fixed_scope"].clone()];
    for (case, scope_row) in cases.iter().zip(admitted) {
        keys(
            case,
            &[
                "id",
                "input",
                "conversion_available",
                "epjson_input",
                "conversion_unavailable_reason",
                "conversion_proof_bindings",
            ],
        )?;
        require(
            case["id"] == scope_row["id"] && case["input"] == scope_row["input"],
            "ordered case/IDF differs",
        )?;
        let available = case["conversion_available"]
            .as_bool()
            .ok_or("typed conversion availability required")?;
        require(
            !rows(&case["conversion_proof_bindings"])?.is_empty(),
            "actual input-only conversion proof bindings required",
        )?;
        bindings.extend(rows(&case["conversion_proof_bindings"])?.iter().cloned());
        if available {
            require(
                case["conversion_unavailable_reason"].is_null(),
                "available conversion cannot have unavailable reason",
            )?;
            bindings.push(case["epjson_input"].clone());
        } else {
            require(
                case["epjson_input"].is_null()
                    && !text(&case["conversion_unavailable_reason"])?.is_empty(),
                "unavailable conversion requires no fabricated input and actual reason",
            )?;
        }
        for name in ["input", "weather", "metadata"] {
            bindings.push(scope_row[name].clone());
        }
    }
    for binding in &bindings {
        bound(&root, binding)?;
    }
    let models = cases
        .iter()
        .map(|case| model_case(&root, case, initial))
        .collect::<Result<Vec<_>>>()?;
    for binding in &bindings {
        bound(&root, binding)?;
    }
    observer_pins(&root, &request["observer_sources"])?;
    source_pins(
        &root,
        &request["actual_public_API_sources"],
        fields::API_SOURCES,
    )?;
    require(
        std::fs::read(&request_path)? == original,
        "observer request changed during public calls",
    )?;
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({"schema":"ctf01-existing-observer-results.v1",
        "actual_request":{"path":request_path,"sha256":digest::sha256(&original)},
        "fixed_scope":request["fixed_scope"],"observer_sources":request["observer_sources"],
        "actual_public_API_sources":request["actual_public_API_sources"],"model_cases":models,
        "all_bound_inputs_unchanged":true,"same_original_IDF_order_overlay_used":true,
        "scientific_comparison_performed":false,"numerical_PASS_claimed":false,
        "gates_updated":false,"CTF02_through_CTF10_certified":false}))?
    );
    Ok(())
}
