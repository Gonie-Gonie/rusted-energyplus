//! Observes corrected public parsing, compilation and schedule-consumer owners.
#[path = "sch01_existing_support/consumer.rs"]
mod consumer;
#[path = "../../ep_runtime/examples/clk02_probe_support/digest.rs"]
mod digest;
#[path = "sch01_existing_support/fields.rs"]
mod fields;
#[path = "sch01_existing_support/inputs.rs"]
mod inputs;
#[path = "sch01_existing_support/observe.rs"]
mod observe;

use inputs::{array, require, text};
use serde_json::{Value, json};
use std::collections::BTreeMap;
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
        .ok_or("usage: sch01_existing_observer <input-only-observer-request.json>")?;
    require(
        args.next().is_none(),
        "exactly one observer request required",
    )?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("repository root unavailable")?
        .canonicalize()?;
    let request_path = PathBuf::from(supplied).canonicalize()?;
    require(
        request_path.starts_with(&root),
        "contained request required",
    )?;
    let request_bytes = std::fs::read(&request_path)?;
    let wrapper: Value = serde_json::from_slice(&request_bytes)?;
    let (native, bindings) = inputs::admit(&root, &wrapper)?;
    let companions = array(&wrapper["model_cases"])?;
    let mut models = Vec::new();
    for (case, companion) in array(&native["model_cases"])?.iter().zip(companions) {
        models.push(observe::model_case(&root, case, companion)?);
    }
    inputs::check_all(&root, &bindings)?;
    require(
        std::fs::read(&request_path)? == request_bytes,
        "observer request changed during public calls",
    )?;
    let mut calls = BTreeMap::<String, usize>::new();
    let mut outcomes = BTreeMap::<String, usize>::new();
    let mut not_invoked = 0usize;
    for model in &models {
        for operation in array(&model["operations"])? {
            *outcomes
                .entry(text(&operation["call_outcome"]["status"])?.to_owned())
                .or_default() += 1;
            if inputs::flag(&operation["actual_rust_invoked"])? {
                *calls
                    .entry(text(&operation["actual_public_API"])?.to_owned())
                    .or_default() += 1;
            } else {
                not_invoked += 1;
            }
        }
        if let Some(invocations) =
            model["selected_consumer_handoff"]["actual_public_API_invocations"].as_object()
        {
            for (name, count) in invocations {
                *calls.entry(name.clone()).or_default() += usize::try_from(
                    count
                        .as_u64()
                        .ok_or("typed consumer invocation count required")?,
                )?;
            }
        }
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schema":"sch01-candidate-probe-results.v1","observation_stage":"corrected-candidate",
            "actual_request":{"path":inputs::relative(&root,&request_path)?,
                "sha256":digest::sha256(&request_bytes)},
            "actual_native_request":wrapper["native_request"],
            "actual_native_admission_contract":wrapper["native_admission_contract"],
            "native_observation_projection":wrapper["native_observation_projection"],
            "contracts":wrapper["contracts"],
            "actual_scope_amendment":native["actual_scope_amendment"],
            "actual_scope_amendment_execution":native["actual_scope_amendment_execution"],
            "requested_counts":native["requested_counts"],"model_cases":models,
            "actual_rust_function_counts":calls,"actual_rust_status_counts":outcomes,
            "actual_rust_requested_operations_not_invoked":not_invoked,
            "all_bound_inputs_unchanged":true,"existing_default_IDF_overlay_used":true,
            "compiler_or_runtime_core_modified":true,"Native_partial_graph_reconstructed":false,
            "native_answers_read":false,"expected_answers_read":false,
            "scientific_comparison_performed":false,"numerical_comparison_count":null,
            "numerical_mismatch_count":null,"numerical_PASS_claimed":false,
            "SCH02_arithmetic_certified":false,"SCH03_lookup_certified":false,
            "gates_updated":false
        }))?
    );
    Ok(())
}
