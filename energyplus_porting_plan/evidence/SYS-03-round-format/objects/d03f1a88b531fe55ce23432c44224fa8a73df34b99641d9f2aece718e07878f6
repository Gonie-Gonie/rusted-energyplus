//! Direct observation of the actual unit-2 formatter source, with no expected text.
#[path = "../../ep_runtime/examples/clk02_probe_support/digest.rs"]
mod digest;
#[path = "../../ep_runtime/src/heat_balance/source_round_format.rs"]
mod formatting;
use serde_json::{Value, json};
use std::collections::BTreeSet;
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn require(ok: bool, message: &str) -> Result<()> {
    if ok {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}
fn keys(value: &Value, names: &[&str]) -> Result<()> {
    require(
        value.as_object().is_some_and(|object| {
            object.len() == names.len() && names.iter().all(|name| object.contains_key(*name))
        }),
        "exact input object required",
    )
}
fn run() -> Result<i32> {
    let args: Vec<String> = std::env::args().collect();
    require(
        args.len() == 3,
        "usage: source_round_format_unit_observer <input-only-request.json> <actual-SHA256>",
    )?;
    let bytes = std::fs::read(&args[1])?;
    require(
        digest::sha256(&bytes) == args[2],
        "full input-only request SHA differs",
    )?;
    let request: Value = serde_json::from_slice(&bytes)?;
    keys(
        &request,
        &[
            "schema",
            "card",
            "expected_values_supplied",
            "values",
            "formats",
        ],
    )?;
    require(
        request["schema"] == "source-round-format-unit-request.v1"
            && request["card"] == "SYS-03"
            && request["expected_values_supplied"] == false,
        "unadmitted request",
    )?;
    let formats = [
        ("{:.0R}", 0),
        ("{:.2R}", 2),
        ("{:.3R}", 3),
        ("{:.4R}", 4),
        ("{:.10R}", 10),
    ];
    require(
        request["formats"] == json!(formats.map(|(pattern, _)| pattern)),
        "exact Source R formats required",
    )?;
    let inputs = request["values"]
        .as_array()
        .ok_or("input-only values unavailable")?;
    require(!inputs.is_empty(), "input-only values required")?;
    let mut rows = Vec::new();
    let mut ids = BTreeSet::new();
    let mut unavailable = 0;
    for input in inputs {
        keys(input, &["case_id", "value_bits"])?;
        let id = input["case_id"].as_str().ok_or("case ID required")?;
        require(ids.insert(id), "duplicate input identity")?;
        let bits = input["value_bits"].as_str().ok_or("IEEE bits required")?;
        require(
            bits.len() == 16 && bits.bytes().all(|value| value.is_ascii_hexdigit()),
            "exact IEEE input required",
        )?;
        let value = f64::from_bits(u64::from_str_radix(bits, 16)?);
        let mut observations = Vec::new();
        for (pattern, precision) in formats {
            if value == f64::INFINITY {
                observations.push(json!({"format":pattern,"actual_value_bits_before_call":format!("{:016x}",value.to_bits()),"actual_source_invoked":false,"outcome":"source_domain_unavailable","text":null,"unavailable":"UndefinedLog10Conversion"}));
                unavailable += 1;
                continue;
            }
            let result = formatting::round(value, precision);
            let record = match result {
                Ok(text) => {
                    json!({"format":pattern,"actual_value_bits_before_call":format!("{:016x}",value.to_bits()),"actual_source_invoked":true,"outcome":"source_returned","text":text,"unavailable":null})
                }
                Err(reason) => {
                    unavailable += 1;
                    json!({"format":pattern,"actual_value_bits_before_call":format!("{:016x}",value.to_bits()),"actual_source_invoked":true,"outcome":"typed_unavailable","text":null,"unavailable":format!("{reason:?}")})
                }
            };
            observations.push(record);
        }
        rows.push(json!({"case_id":id,"value_bits":bits,"observations":observations}));
    }
    println!(
        "{}",
        json!({"schema":"source-round-format-unit-candidate-results.v1","actual_input_SHA256":args[2],"rows":rows,
        "unavailable_count":unavailable,"expected_values_supplied":false,"formatter_PASS_claimed":false,"full_report_or_backend_equivalence_claimed":false})
    );
    Ok(
        if rows.iter().any(|row| {
            row["observations"].as_array().is_some_and(|items| {
                items
                    .iter()
                    .any(|item| item["outcome"] == "typed_unavailable")
            })
        }) {
            2
        } else {
            0
        },
    )
}
fn main() {
    match run() {
        Ok(code) => std::process::exit(code),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
