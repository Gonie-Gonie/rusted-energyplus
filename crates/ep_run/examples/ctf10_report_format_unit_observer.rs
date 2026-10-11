//! Direct observation of the actual unit-2 formatter source, with no expected text.
use ep_runtime::heat_balance::ctf_reports::CtfReportUnavailable;
#[path = "../../ep_runtime/examples/clk02_probe_support/digest.rs"]
mod digest;
#[path = "../../ep_runtime/src/heat_balance/ctf_reports_format.rs"]
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
        "usage: ctf10_report_format_unit_observer <input-only-request.json> <actual-SHA256>",
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
        request["schema"] == "ctf10-report-format-unit-request.v1"
            && request["card"] == "CTF-10"
            && request["expected_values_supplied"] == false,
        "unadmitted request",
    )?;
    let formats = [
        ("{:8.3F}", 8, 3, false),
        ("{:8.4F}", 8, 4, false),
        ("{:14.3F}", 14, 3, false),
        ("{:11.3F}", 11, 3, false),
        ("{:13.3F}", 13, 3, false),
        ("{:15.4G}", 15, 4, true),
        ("{:12.4G}", 12, 4, true),
        ("{:20.8G}", 20, 8, true),
    ];
    require(
        request["formats"] == json!(formats.map(|(pattern, _, _, _)| pattern)),
        "exact Source formats required",
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
        for (pattern, width, precision, general) in formats {
            let result = if general {
                formatting::general(value, width, precision)
            } else {
                Ok(formatting::fixed(value, width, precision))
            };
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
        json!({"schema":"ctf10-report-format-unit-candidate-results.v1","actual_input_SHA256":args[2],"rows":rows,
        "unavailable_count":unavailable,"expected_values_supplied":false,"formatter_PASS_claimed":false,"full_report_or_backend_equivalence_claimed":false})
    );
    Ok(if unavailable == 0 { 0 } else { 2 })
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
