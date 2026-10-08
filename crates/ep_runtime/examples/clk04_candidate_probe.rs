//! Output-only observations of genuine mutable weather owners.
//! Original answers, comparison reports and production traces are never inputs.
#[path = "clk02_probe_support/digest.rs"]
mod digest;
#[allow(dead_code)]
#[path = "clk03_candidate_support/inputs.rs"]
mod existing_inputs;
#[path = "clk04_candidate_support/fields.rs"]
mod fields;
#[path = "clk04_candidate_support/inputs.rs"]
mod inputs;
#[path = "clk03_candidate_support/fields.rs"]
mod legacy_fields;
#[path = "clk04_candidate_support/observe.rs"]
mod observe;
#[allow(dead_code)]
#[path = "clk02_probe_support/dto.rs"]
mod raw_dto;
#[path = "clk04_candidate_support/wind.rs"]
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
    let supplied = args
        .next()
        .ok_or("usage: clk04_candidate_probe <candidate-input-only-request.json>")?;
    require(
        args.next().is_none(),
        "exactly one input-only candidate request required",
    )?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("repository root unavailable")?
        .canonicalize()?;
    let request_path = PathBuf::from(supplied).canonicalize()?;
    require(
        request_path.starts_with(&root),
        "contained candidate request required",
    )?;
    let request_bytes = std::fs::read(&request_path)?;
    let observer: Value = serde_json::from_slice(&request_bytes)?;
    require(
        text(&observer["schema"])? == "clk04-candidate-observer-request.v1"
            && observer["frozen_before_Rust_candidate_numerical_execution"] == true
            && observer["expected_values_supplied"] == false
            && observer["native_results_supplied"] == false
            && observer["Rust_outputs_supplied_as_inputs"] == false,
        "final input-only candidate request required",
    )?;
    let (_, native_bytes) = bound_file(&root, &observer["native_request"])?;
    let native: Value = serde_json::from_slice(&native_bytes)?;
    inputs::admit(&native)?;
    let (_, admission_bytes) = bound_file(&root, &observer["native_admission_contract"])?;
    let admission: Value = serde_json::from_slice(&admission_bytes)?;
    require(
        admission["schema"] == "clk04-native-admission-contract.v1"
            && admission["helper_request"] == observer["native_request"]
            && admission["contracts"] == observer["contracts"]
            && admission["fixed_scope"] == native["fixed_scope"]
            && admission["frozen_before_numerical_execution"] == true,
        "one-way native admission differs",
    )?;
    require(
        observer["native_observation_policy"] == native["observation_policy"],
        "exact native observation policy required",
    )?;
    for name in ["source", "tolerances"] {
        require(
            native["contracts"][name] == admission["contracts"][name],
            "frozen native contract differs",
        )?;
    }
    let mut bindings = inputs::bindings(&native)?;
    bindings.push(observer["native_request"].clone());
    bindings.push(observer["native_admission_contract"].clone());
    bindings.push(admission["contracts"]["cases"].clone());
    bindings.push(admission["projection"].clone());
    for name in ["observer_sources", "actual_public_API_sources"] {
        for binding in array(&observer[name])? {
            bindings.push(binding.clone());
        }
    }
    for binding in observer["source_authorities"]
        .as_object()
        .ok_or("source authorities required")?
        .values()
    {
        bindings.push(binding.clone());
    }
    bindings.push(observer["independent_source_review"].clone());
    inputs::check_all(&root, &bindings)?;
    require(
        observer["requested_counts"] == native["requested_counts"],
        "candidate/native counts differ",
    )?;
    let mut setup = Vec::new();
    for input in array(&native["setup_cases"])? {
        setup.push(observe::setup(input)?);
    }
    let mut wind_cases = Vec::new();
    for input in array(&native["wind_cases"])? {
        wind_cases.push(ep_runtime::psychrometrics::with_fresh_psychrometric_state(
            || wind::observe(input, &observer["public_wind_adapter"]),
        )?);
    }
    let mut weather = Vec::new();
    for input in array(&native["weather_sequences"])? {
        let (_, idf_bytes) = bound_file(&root, &input["input"])?;
        let (weather_path, weather_bytes) = bound_file(&root, &input["weather"])?;
        weather.push(ep_runtime::psychrometrics::with_fresh_psychrometric_state(
            || observe::sequence(input, &idf_bytes, &weather_path, weather_bytes),
        )?);
    }
    inputs::check_all(&root, &bindings)?;
    require(
        std::fs::read(&request_path)? == request_bytes,
        "candidate request changed during public calls",
    )?;
    let result = json!({"schema":"clk04-candidate-probe-results.v1",
        "actual_request":{"path":inputs::relative(&root,&request_path)?,"sha256":digest::sha256(&request_bytes)},
        "actual_native_request":observer["native_request"],"actual_native_admission_contract":observer["native_admission_contract"],
        "contracts":observer["contracts"],"fixed_scope":native["fixed_scope"],"requested_counts":native["requested_counts"],
        "setup_cases":setup,"wind_cases":wind_cases,"weather_sequences":weather,
        "public_wind_adapter_inputs":observer["public_wind_adapter"],"all_bound_inputs_unchanged":true,
        "fresh_public_psychrometric_state_per_sequence":true,
        "native_packet_rewritten_or_native_answers_supplied":false,
        "expected_answers_read":false,"whole_native_constructor_observed":false,"native_callbacks_invoked":false,
        "scientific_comparison_performed":false,"numerical_comparison_count":null,"numerical_mismatch_count":null,
        "numerical_PASS_claimed":false,"gates_updated":false});
    println!("{}", serde_json::to_string_pretty(&result)?);
    Ok(())
}
