//! Probe input routing and read-only observations, without reference answers.

mod digest;
mod dto;
mod fixed;
mod headers;
mod inputs;
mod records;

use serde_json::{Map, Value, json};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub(super) fn run(request_path: &Path) -> Result<Value> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("missing repository root")?
        .canonicalize()?;
    let request_path = request_path.canonicalize()?;
    inputs::require(
        request_path.starts_with(&root),
        "request must be inside repository",
    )?;
    let request_bytes = std::fs::read(&request_path)?;
    let request: Value = serde_json::from_slice(&request_bytes)?;
    inputs::require(
        inputs::text(&request["schema"])? == "clk02-helper-cases.v1"
            && request["expected_values_supplied"] == false
            && request["expected_exits_supplied"] == false,
        "input-only request schema/policy differs",
    )?;
    // Identity metadata is captured before either selected owner executes.
    // Contract content never supplies parser operands or expected answers.
    let mut contracts = Map::new();
    for name in ["source", "cases", "tolerances"] {
        let relative = format!("energyplus_porting_plan/contracts/CLK-02-{name}.json");
        let path = root.join(&relative).canonicalize()?;
        inputs::require(
            path.starts_with(&root),
            "contract must be inside repository",
        )?;
        let bytes = std::fs::read(&path)?;
        contracts.insert(
            name.to_owned(),
            json!({"path":relative,"sha256":digest::sha256(&bytes)}),
        );
    }
    let sequence_inputs = inputs::array(&request["record_sequences"], 40)?;
    let header_inputs = inputs::array(&request["header_cases"], 22)?;
    let sequences = sequence_inputs
        .iter()
        .map(records::sequence)
        .collect::<Result<Vec<_>>>()?;
    let header_cases = header_inputs
        .iter()
        .map(|item| headers::case(item, &root))
        .collect::<Result<Vec<_>>>()?;
    let fixed_epw = fixed::run(&request["fixed_epw"], &root)?;
    let diagnostic_calls: usize = sequences
        .iter()
        .map(|row| row["operations"].as_array().map_or(0, Vec::len))
        .sum();
    inputs::require(
        diagnostic_calls == 46,
        "all declared diagnostic raw calls must execute",
    )?;
    Ok(json!({
        "schema":"clk02-rust-probe-results.v1",
        "contracts":contracts,
        "actual_request":{"path":request_path.strip_prefix(&root)?,"sha256":digest::sha256(&request_bytes)},
        "record_sequences":sequences,"header_cases":header_cases,"fixed_epw":fixed_epw,
        "actual_wrapper_counts":{"diagnostic_raw_calls":diagnostic_calls,
            "diagnostic_header_roots":header_inputs.len(),"fixed_epw_raw_calls":fixed_epw["record_count"],
            "fixed_epw_open_roots":1},
        "complete":true,"physics_executed":false,"expected_answers_supplied":false,"gates_updated":false,
        "source_locals_observed":false,"original_warning_error_IO_peer_parity_claimed":false,
        "direct_enum_and_whole_open_routes_distinct":true,"unpaired_downstream_weather_science":true,
    }))
}
