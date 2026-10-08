//! SOURCE-ONLY DRAFT: existing public Rust observations from declared inputs.
//! Intended future location: crates/ep_runtime/examples/clk04_legacy_probe.rs.
//! Native outputs, expected answers, comparers and production traces are never read.
#[path = "clk02_probe_support/digest.rs"]
mod digest;
#[path = "clk03_candidate_support/fields.rs"]
mod legacy_fields;
#[allow(dead_code)]
#[path = "clk03_candidate_support/inputs.rs"]
mod existing_inputs;
#[allow(dead_code)]
#[path = "clk02_probe_support/dto.rs"]
mod raw_dto;
#[path = "clk04_legacy_support/fields.rs"]
mod fields;
#[path = "clk04_legacy_support/inputs.rs"]
mod inputs;
#[path = "clk04_legacy_support/observe.rs"]
mod observe;
#[path = "clk04_legacy_support/wind.rs"]
mod wind;

use inputs::{array, bound_file, require, text};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let supplied = args.next().ok_or("usage: clk04_legacy_probe <observer-request.json>")?;
    require(args.next().is_none(), "exactly one input-only request required")?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().and_then(Path::parent)
        .ok_or("repository root unavailable")?.canonicalize()?;
    let request_path = PathBuf::from(supplied).canonicalize()?;
    require(request_path.starts_with(&root), "contained observer request required")?;
    let request_bytes = std::fs::read(&request_path)?;
    let observer: Value = serde_json::from_slice(&request_bytes)?;
    require(text(&observer["schema"])? == "clk04-existing-observer-request.v1"
        && observer["frozen_before_numerical_execution"] == true
        && observer["expected_values_supplied"] == false
        && observer["native_results_supplied"] == false
        && observer["Rust_outputs_supplied_as_inputs"] == false,
        "final input-only observer request required")?;
    let (_, native_bytes) = bound_file(&root, &observer["native_request"])?;
    let native: Value = serde_json::from_slice(&native_bytes)?;
    inputs::admit(&native)?;
    let (_, admission_bytes) = bound_file(&root,&observer["native_admission_contract"])?;
    let admission: Value = serde_json::from_slice(&admission_bytes)?;
    require(admission["schema"] == "clk04-native-admission-contract.v1"
        && admission["helper_request"] == observer["native_request"]
        && admission["contracts"] == observer["contracts"]
        && admission["fixed_scope"] == native["fixed_scope"]
        && admission["frozen_before_numerical_execution"] == true, "one-way native admission identity differs")?;
    require(observer["native_observation_policy"] == native["observation_policy"],
        "exact pre-output native observation policy identity differs")?;
    for name in ["source","tolerances"] {
        require(native["contracts"][name] == admission["contracts"][name], "native input contract identity differs")?;
    }
    let mut bindings = inputs::bindings(&native)?;
    bindings.push(observer["native_request"].clone());
    bindings.push(observer["native_admission_contract"].clone());
    bindings.push(admission["contracts"]["cases"].clone());
    bindings.push(admission["projection"].clone());
    for binding in array(&observer["observer_sources"])? {
        bindings.push(binding.clone());
    }
    for binding in array(&observer["existing_public_API_sources"])? {
        bindings.push(binding.clone());
    }
    for binding in observer["source_authorities"].as_object().ok_or("source authority bindings required")?.values() {
        bindings.push(binding.clone());
    }
    bindings.push(observer["independent_source_review"].clone());
    inputs::check_all(&root, &bindings)?;
    require(observer["requested_counts"] == native["requested_counts"], "observer/native counts differ")?;
    let setup = array(&native["setup_cases"])?.iter().map(|input| {
        json!({"id":input["id"],"requested_operation":input,"actual_rust_invoked":false,
            "before":null,"after":null,"actual_observation_available":false,
            "call_outcome":{"status":"unavailable-existing-whole-SetupInterpolationValues-owner",
                "source_fatal":null,"counted_as_PASS":false},
            "Interpolation":null,"SolarInterpolation":null,"stored_weights":null})
    }).collect::<Vec<_>>();
    let mut wind_cases = Vec::new();
    for input in array(&native["wind_cases"])? {
        wind_cases.push(wind::observe(input, &observer["public_wind_adapter"])?);
    }
    let mut weather = Vec::new();
    for input in array(&native["weather_sequences"])? {
        let (_, idf_bytes) = bound_file(&root, &input["input"])?;
        let (weather_path, weather_bytes) = bound_file(&root, &input["weather"])?;
        weather.push(observe::sequence(input, &idf_bytes, &weather_path, weather_bytes)?);
    }
    inputs::check_all(&root, &bindings)?;
    require(std::fs::read(&request_path)? == request_bytes, "observer request changed during observations")?;
    let result = json!({"schema":"clk04-legacy-probe-results.v1",
        "actual_request":{"path":inputs::relative(&root,&request_path)?,"sha256":digest::sha256(&request_bytes)},
        "actual_native_request":observer["native_request"],"contracts":observer["contracts"],
        "actual_native_admission_contract":observer["native_admission_contract"],
        "fixed_scope":native["fixed_scope"],"requested_counts":native["requested_counts"],
        "setup_cases":setup,"wind_cases":wind_cases,"weather_sequences":weather,
        "public_wind_adapter_inputs":observer["public_wind_adapter"],
        "all_bound_inputs_unchanged":true,"expected_answers_read":false,
        "whole_native_constructor_observed":false,"native_callbacks_invoked":false,
        "scientific_comparison_performed":false,"numerical_comparison_count":null,
        "numerical_mismatch_count":null,"numerical_PASS_claimed":false,"gates_updated":false});
    println!("{}",serde_json::to_string_pretty(&result)?);
    Ok(())
}
