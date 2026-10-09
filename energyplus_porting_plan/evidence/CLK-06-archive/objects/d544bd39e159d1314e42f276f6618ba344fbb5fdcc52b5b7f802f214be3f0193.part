//! Passive observations of candidate public APIs and actual storage. No native answers as inputs.
#[path = "clk02_probe_support/digest.rs"]
mod digest;
#[allow(dead_code, clippy::duplicate_mod)]
#[path = "clk04_candidate_support/fields.rs"]
mod existing_current_fields;
#[allow(dead_code)]
#[path = "clk03_candidate_support/inputs.rs"]
mod existing_inputs;
#[path = "clk06_candidate_support/fields.rs"]
mod fields;
#[path = "clk06_candidate_support/inputs.rs"]
mod inputs;
#[allow(clippy::duplicate_mod)]
#[path = "clk03_candidate_support/fields.rs"]
mod legacy_fields;
#[path = "clk06_candidate_support/observe.rs"]
mod observe;
#[allow(dead_code, clippy::duplicate_mod)]
#[path = "clk02_probe_support/dto.rs"]
mod raw_dto;
#[path = "clk06_candidate_support/seeds.rs"]
mod seeds;
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
        .ok_or("usage: clk06_candidate_probe <input-only-observer-request.json>")?;
    require(
        args.next().is_none(),
        "exactly one input-only observer request required",
    )?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("repository root unavailable")?
        .canonicalize()?;
    let request_path = PathBuf::from(supplied).canonicalize()?;
    require(
        request_path.starts_with(&root),
        "contained observer request required",
    )?;
    let request_bytes = std::fs::read(&request_path)?;
    let observer: Value = serde_json::from_slice(&request_bytes)?;
    require(
        text(&observer["schema"])? == "clk06-candidate-observer-request.v1"
            && observer["frozen_before_Rust_candidate_numerical_execution"] == true
            && observer["expected_values_supplied"] == false
            && observer["native_results_supplied"] == false
            && observer["Rust_outputs_supplied_as_inputs"] == false,
        "final input-only candidate observer request required",
    )?;
    let (_, native_bytes) = bound_file(&root, &observer["native_request"])?;
    let native: Value = serde_json::from_slice(&native_bytes)?;
    inputs::admit(&native)?;
    let (_, admission_bytes) = bound_file(&root, &observer["native_admission_contract"])?;
    let admission: Value = serde_json::from_slice(&admission_bytes)?;
    require(
        admission["schema"] == "clk06-native-admission-contract.v1"
            && admission["helper_request"] == observer["native_request"]
            && admission["contracts"] == observer["contracts"]
            && admission["fixed_scope"] == native["fixed_scope"]
            && admission["all_45_guard_refs"] == native["all_45_guard_refs"]
            && admission["requested_counts"] == native["requested_counts"]
            && admission["root_preoutput_decision"] == native["root_preoutput_decision"]
            && observer["root_preoutput_decision"] == native["root_preoutput_decision"]
            && admission["frozen_before_numerical_execution"] == true,
        "one-way native admission differs",
    )?;
    for name in ["source", "tolerances"] {
        require(
            native["contracts"][name] == admission["contracts"][name],
            "native contract differs",
        )?;
    }
    let (_, projection_bytes) = bound_file(&root, &admission["projection"])?;
    let projection: Value = serde_json::from_slice(&projection_bytes)?;
    require(
        projection["schema"] == "clk06-observation-projection.v1"
            && projection["helper_request"] == observer["native_request"]
            && projection["contracts"] == observer["contracts"]
            && projection["native_observation_policy"] == native["observation_policy"]
            && observer["native_observation_policy"] == native["observation_policy"]
            && observer["native_observation_projection"] == admission["projection"]
            && projection["root_preoutput_decision"] == native["root_preoutput_decision"]
            && projection["production_transport_source_inventory"]
                == observer["production_transport_source_inventory"],
        "native projection differs",
    )?;
    let mut bindings = inputs::bindings(&native)?;
    for binding in [
        &observer["native_request"],
        &observer["native_admission_contract"],
        &admission["contracts"]["cases"],
        &admission["projection"],
        &observer["independent_source_review"],
        &observer["root_preoutput_decision"],
        &observer["production_transport_source_inventory"],
    ] {
        bindings.push(binding.clone());
    }
    for name in [
        "observer_sources",
        "actual_public_API_sources",
        "actual_promoted_observer_sources",
    ] {
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
    inputs::check_all(&root, &bindings)?;
    require(
        observer["requested_counts"] == native["requested_counts"],
        "observer/native counts differ",
    )?;
    let mut setup = Vec::new();
    for input in array(&native["setup_cases"])? {
        setup.push(observe::setup(input)?);
    }
    let mut emissivity = Vec::new();
    for input in array(&native["emissivity_cases"])? {
        emissivity.push(observe::emissivity(input)?);
    }
    let mut weather = Vec::new();
    for input in array(&native["weather_sequences"])? {
        let (_, idf) = bound_file(&root, &input["input"])?;
        let (path, bytes) = bound_file(&root, &input["weather"])?;
        weather.push(ep_runtime::psychrometrics::with_fresh_psychrometric_state(
            || observe::sequence(input, &idf, &path, bytes),
        )?);
    }
    inputs::check_all(&root, &bindings)?;
    require(
        std::fs::read(&request_path)? == request_bytes,
        "observer request changed during actual public calls",
    )?;
    let mut calls = std::collections::BTreeMap::<String, usize>::new();
    let mut outcomes = std::collections::BTreeMap::<String, usize>::new();
    let mut unavailable = 0usize;
    let mut not_invoked = 0usize;
    for item in setup.iter().chain(emissivity.iter()).chain(
        weather
            .iter()
            .flat_map(|sequence| sequence["operations"].as_array().into_iter().flatten()),
    ) {
        let kind = text(&item["kind"])?;
        let status = text(&item["call_outcome"]["status"])?;
        *outcomes.entry(status.to_owned()).or_default() += 1;
        if inputs::flag(&item["actual_rust_invoked"])? {
            *calls.entry(kind.to_owned()).or_default() += 1;
        } else if status == "unavailable-public-Rust-API" {
            unavailable += 1;
        } else {
            not_invoked += 1;
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({"schema":"clk06-candidate-probe-results.v1",
        "actual_request":{"path":inputs::relative(&root,&request_path)?,"sha256":digest::sha256(&request_bytes)},
        "actual_native_request":observer["native_request"],"actual_native_admission_contract":observer["native_admission_contract"],
        "native_observation_projection":admission["projection"],"contracts":observer["contracts"],
        "fixed_scope":native["fixed_scope"],"requested_counts":native["requested_counts"],
        "setup_cases":setup,"emissivity_cases":emissivity,"weather_sequences":weather,
        "actual_rust_function_counts":calls,"actual_rust_status_counts":outcomes,
        "actual_rust_unavailable_direct_API_operations":unavailable,"actual_rust_not_invoked_after_prior_outcome":not_invoked,
        "all_bound_inputs_unchanged":true,"solar_control_owned_storage_available":true,
        "requested_solar_controls_applied":true,"whole_native_sky_environment_owner_available":false,
        "native_current_sky_globals_paired":false,"root_preoutput_decision":observer["root_preoutput_decision"],
        "fresh_public_psychrometric_state_per_sequence":true,"native_callbacks_invoked":false,
        "expected_answers_read":false,"native_packet_rewritten_or_native_answers_supplied":false,
        "scientific_comparison_performed":false,"numerical_comparison_count":null,
        "numerical_mismatch_count":null,"numerical_PASS_claimed":false,"gates_updated":false}))?
    );
    Ok(())
}
