//! Input-only ordinary A24 geometry observation through the real compiler and initializer.
#![recursion_limit = "256"]
#[path = "../../ep_runtime/examples/clk02_probe_support/digest.rs"]
mod digest;
#[path = "src06_ordinary_geometry_support/fields.rs"]
mod fields;

use ep_compiler::compile_raw_model;
use ep_model::SimulationModel;
use ep_raw_model::parse_epjson_str_with_idf_order;
use ep_run::{PortingScope, inspect_porting_scope};
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const INPUT_CONTEXT_SHA: &str = "f9f645bd15fbd6cc96f584ea3d0b058ed0156da2f0bde8ffab56394835e8fa4e";
const POLICY_SHA: &str = "0bb48c5f93aea8393c8012f28ccfaa37c9e191e201f402a547156a8376092f6b";
fn need(value: bool, message: &str) -> Result<()> {
    if value {
        Ok(())
    } else {
        Err(message.to_owned().into())
    }
}
fn text(value: &Value) -> Result<&str> {
    value.as_str().ok_or_else(|| "string required".into())
}
fn rows(value: &Value) -> Result<&[Value]> {
    value
        .as_array()
        .map(Vec::as_slice)
        .ok_or_else(|| "array required".into())
}
fn keys(value: &Value, expected: &[&str]) -> Result<()> {
    need(
        value.as_object().is_some_and(|v| {
            v.len() == expected.len() && expected.iter().all(|key| v.contains_key(*key))
        }),
        "exact object keys required",
    )
}
struct Bindings {
    root: PathBuf,
    checked: BTreeMap<PathBuf, (String, usize)>,
}
impl Bindings {
    fn bytes(&mut self, row: &Value) -> Result<Vec<u8>> {
        let path = self.root.join(text(&row["path"])?).canonicalize()?;
        need(
            path.starts_with(&self.root) && path.is_file(),
            "contained regular input/Source required",
        )?;
        let bytes = std::fs::read(&path)?;
        need(
            bytes.len() <= 16 * 1024 * 1024 && digest::sha256(&bytes) == text(&row["sha256"])?,
            "actual bound SHA differs",
        )?;
        if let Some(size) = row.get("size_bytes") {
            need(
                size.as_u64() == Some(u64::try_from(bytes.len())?),
                "actual bound size differs",
            )?;
        }
        self.checked
            .insert(path, (digest::sha256(&bytes), bytes.len()));
        Ok(bytes)
    }
    fn json(&mut self, row: &Value) -> Result<Value> {
        Ok(serde_json::from_slice(&self.bytes(row)?)?)
    }
    fn unchanged(&self) -> Result<()> {
        for (path, (sha, size)) in &self.checked {
            let bytes = std::fs::read(path)?;
            need(
                bytes.len() == *size && digest::sha256(&bytes) == *sha,
                "bound input/Source changed during observation",
            )?;
        }
        Ok(())
    }
}
fn source_bindings(bindings: &mut Bindings, row: &Value) -> Result<Value> {
    let value = bindings.json(row)?;
    keys(
        &value,
        &["schema", "observer_sources", "actual_public_API_sources"],
    )?;
    need(
        value["schema"] == "src06-ordinary-geometry-runtime-source-bindings.v1",
        "actual formatted runtime Source packet required",
    )?;
    for (key, expected) in [
        ("observer_sources", fields::OBSERVER_PATHS),
        ("actual_public_API_sources", fields::API_PATHS),
    ] {
        let actual = rows(&value[key])?;
        need(
            actual.len() == expected.len(),
            "complete ordered direct Source closure required",
        )?;
        for (item, path) in actual.iter().zip(expected) {
            need(text(&item["path"])? == *path, "Source order differs")?;
            bindings.bytes(item)?;
        }
    }
    Ok(value)
}
fn converted_input(bindings: &mut Bindings, context: &Value) -> Result<(String, String, Value)> {
    need(
        context["schema"] == "src06-ordinary-geometry-fixed-input-context.v1"
            && context["case_id"] == "A-24H"
            && context["expected_values_supplied"] == false
            && context["Native_results_supplied"] == false,
        "fixed input-only A24 context required",
    )?;
    let original = bindings.bytes(&context["input"])?;
    need(
        original.len() == 9275
            && digest::sha256(&original)
                == "db66ac75d3ed26b6e7cd35c6f09b02a05110e800a16713eccee47e387d6c3755",
        "unchanged original A24 IDF required",
    )?;
    let proof = rows(&context["conversion_proof_bindings"])?;
    need(
        proof.len() == 6,
        "complete existing conversion proof required",
    )?;
    for row in proof {
        bindings.bytes(row)?;
    }
    let ledger = bindings.json(&proof[0])?;
    let receipt = bindings.json(&proof[1])?;
    let child = bindings.json(&proof[2])?;
    need(
        ledger["schema"] == "sch01-converted-inputs.v1"
            && ledger["scientific_execution_performed"] == false
            && ledger["Native_scientific_outputs_used_as_inputs"] == false
            && ledger["expected_values_supplied"] == false,
        "existing factual input-only ledger required",
    )?;
    let entries = rows(&ledger["conversions"])?;
    let selected = entries
        .iter()
        .filter(|row| row["case_id"] == "A-24H")
        .collect::<Vec<_>>();
    need(
        selected.len() == 1 && selected[0]["available"] == true,
        "one actual available A24 conversion required",
    )?;
    need(
        child["schema"] == "sch01-input-conversion-command.v1"
            && child["case_id"] == "A-24H"
            && child["exit_code"] == 0
            && child["launch_error"].is_null()
            && child["epjson_available"] == true
            && child["original_and_staged_IDF_preserved"] == true
            && child["numerical_engine_executed"] == false
            && child["Native_scientific_outputs_used_as_inputs"] == false
            && child["expected_answers_supplied"] == false,
        "actual completed A24 converter child required",
    )?;
    need(
        receipt["schema"] == "recorded-porting-command.v1"
            && receipt["exit_code"] == 0
            && receipt["launch_error"].is_null()
            && receipt["source_bytes_match_before_and_after"] == true
            && receipt["executed_launcher"]["archive"]["sha256"]
                == "8468072e78d90a17051c9eac4766bbcbe2e079c399981b9e00d23ff500ec7d3c",
        "authentic historical recorded converter required",
    )?;
    bindings.bytes(&receipt["executed_launcher"]["archive"])?;
    bindings.bytes(&receipt["source_snapshot"])?;
    need(
        bindings.json(&receipt["stdout"])? == ledger,
        "actual converter stdout/ledger differs",
    )?;
    bindings.bytes(&receipt["stderr"])?;
    need(
        bindings.bytes(&child["source_input"])? == original
            && bindings.bytes(&child["staged_input"])? == original
            && bindings.bytes(&selected[0]["input"])? == original
            && bindings.bytes(&proof[3])? == original
            && bindings.bytes(&selected[0]["actual_conversion_execution"])?
                == bindings.bytes(&proof[2])?,
        "actual source/staged/ledger IDF joins differ",
    )?;
    let epjson = bindings.bytes(&context["epjson_input"])?;
    need(
        bindings.bytes(&child["available_epjson_input"])? == epjson
            && bindings.bytes(&selected[0]["epjson_input"])? == epjson,
        "actual converted epJSON joins differ",
    )?;
    need(
        ledger["converter"] == child["converter"]
            && child["converter"]["sha256"]
                == "04f1327fef7148118ea1d79d9e990fb2447b9731e9c1b28a9a8276b840d7cbfe",
        "actual converter identity differs",
    )?;
    let source_hashes = [
        "82f27c8327d2409bb3bcba727044ebe6a36641ed8d7c404e08dd4bc92c990dbb",
        "b38466a0dd8851935d0ef28c079220d7c48f2c05506926d1ad4e7fb463f6de77",
    ];
    let runner_sources = rows(&ledger["executed_runner_sources"])?;
    need(
        runner_sources.len() == source_hashes.len(),
        "complete actual converter Source required",
    )?;
    for (row, sha) in runner_sources.iter().zip(source_hashes) {
        need(row["archive"]["sha256"] == sha, "converter Source differs")?;
        bindings.bytes(&row["archive"])?;
    }
    let times = [
        &receipt["started_utc"],
        &ledger["started_utc"],
        &child["started_utc"],
        &child["finished_utc"],
        &ledger["finished_utc"],
        &receipt["finished_utc"],
    ]
    .into_iter()
    .map(text)
    .collect::<Result<Vec<_>>>()?;
    need(
        times.iter().all(|v| v.ends_with("+00:00")) && times.windows(2).all(|v| v[0] <= v[1]),
        "actual conversion chronology differs",
    )?;
    let declared = bindings.json(&ledger["input_cases"])?;
    let declared_rows = rows(&declared["model_cases"])?;
    let declared_case = declared_rows
        .iter()
        .filter(|row| row["id"] == "A-24H")
        .collect::<Vec<_>>();
    need(
        declared_case.len() == 1 && bindings.bytes(&declared_case[0]["input"])? == original,
        "actual converter declared source differs",
    )?;
    for row in rows(&child["actual_output_inventory"])? {
        bindings.bytes(row)?;
    }
    let command = rows(&receipt["command"])?;
    need(
        command.len() == 13
            && command[0] == "python"
            && command[1] == "-X"
            && command[2] == "utf8"
            && command[3] == "-B"
            && text(&command[4])?.replace('\\', "/")
                == ".runtime/porting/SCH-01/input-prep-02/convert_inputs.py"
            && command[5] == "--approved-source-sha256"
            && command[6] == source_hashes[0]
            && command[7] == "--input-cases"
            && text(&command[8])?.replace('\\', "/")
                == format!(
                    "{}#{}",
                    text(&ledger["input_cases"]["path"])?,
                    text(&ledger["input_cases"]["sha256"])?
                )
            && command[9] == "--converter"
            && command[10]
                == format!(
                    "{}#{}",
                    text(&ledger["converter"]["path"])?,
                    text(&ledger["converter"]["sha256"])?
                )
            && command[11] == "--output-dir"
            && text(&command[12])?.replace('\\', "/")
                == ".runtime/porting/SCH-01/converted-inputs-01",
        "actual historical converter argv differs",
    )?;
    let child_argv = rows(&child["command"])?;
    need(
        child_argv.len() == 2
            && Path::new(text(&child_argv[0])?).canonicalize()?
                == bindings
                    .root
                    .join(text(&child["converter"]["path"])?)
                    .canonicalize()?
            && child_argv[1] == "input.idf"
            && bindings
                .root
                .join(text(&child["cwd"])?)
                .join("input.idf")
                .canonicalize()?
                == bindings
                    .root
                    .join(text(&child["staged_input"]["path"])?)
                    .canonicalize()?,
        "actual child argv/cwd differs",
    )?;
    let native = bindings.json(&context["Native_input_request"])?;
    need(
        native["schema"] == "ordinary-geometry-public-loader-request.v1"
            && native["case"]["id"] == "A-24H"
            && bindings.bytes(&native["case"]["input"])? == original,
        "same frozen genuine Native input required",
    )?;
    bindings.bytes(&native["case_metadata"])?;
    bindings.bytes(&native["fixed_scope"])?;
    need(
        rows(&native["all_45_guard_refs"])?.len() == 45,
        "all fixed original guards required",
    )?;
    for row in rows(&native["all_45_guard_refs"])? {
        bindings.bytes(row)?;
    }
    Ok((
        String::from_utf8(original)?,
        String::from_utf8(epjson)?,
        json!({"ledger":proof[0],"recorded_execution":proof[1],"actual_child":proof[2],"actual_child_argv":child["command"],"actual_ledger_argv":receipt["command"],"Native_input_request":context["Native_input_request"]}),
    ))
}
fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    need(
        args.len() == 3,
        "usage: src06_ordinary_geometry_observer REQUEST_PATH REQUEST_SHA256",
    )?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()?;
    let mut bindings = Bindings {
        root,
        checked: BTreeMap::new(),
    };
    let request_ref = json!({"path":args[1],"sha256":args[2]});
    let request = bindings.json(&request_ref)?;
    keys(
        &request,
        &[
            "schema",
            "input_context",
            "runtime_source_bindings",
            "pairing_policy",
        ],
    )?;
    need(
        request["schema"] == "src06-ordinary-geometry-observer-request.v1"
            && request["input_context"]["sha256"] == INPUT_CONTEXT_SHA
            && request["pairing_policy"]["sha256"] == POLICY_SHA,
        "frozen reviewed input-only request required",
    )?;
    let context = bindings.json(&request["input_context"])?;
    let source = source_bindings(&mut bindings, &request["runtime_source_bindings"])?;
    let policy = bindings.json(&request["pairing_policy"])?;
    need(
        policy["schema"] == "src06-ordinary-geometry-prospective-pairing-policy.v1"
            && policy["Native_answers_or_results_used_for_authorship"] == false,
        "frozen prospective policy required",
    )?;
    let (idf, epjson, conversion) = converted_input(&mut bindings, &context)?;
    let raw = parse_epjson_str_with_idf_order(&epjson, &idf)?;
    let raw_owners = fields::raw(&raw)?;
    let compiled = compile_raw_model(&raw);
    let compile = fields::compile(&compiled.report);
    let scope = inspect_porting_scope(
        &raw,
        &compiled.report,
        compiled.model.as_ref(),
        PortingScope::A,
    );
    let mut result = json!({"schema":"src06-ordinary-geometry-observer-results.v1","case_id":"A-24H",
        "actual_request":request_ref,"actual_input_context":request["input_context"],"actual_runtime_source_bindings":source,
        "frozen_pairing_policy":request["pairing_policy"],"actual_conversion":conversion,
        "complete_input":{"original_IDF":idf,"converted_epJSON":epjson,"RawModel":raw_owners},
        "actual_compile_report":compile,"actual_A_scope_preflight":scope,
        "actual_calls":{"parse_epjson_with_IDF_order":1,"compile_raw_model":1,"initialize_heat_balance_state_with_initialization_context":0},
        "typed_geometry_inputs":null,"caller_before":null,"caller_after":null,
        "selected_owners":fields::unavailable("initializer has not returned an actual State"),
        "initializer_result":null,"Native_intermediate_calls_reconstructed":false,"geometry_producers_reexecuted_by_observer":false,
        "Native_answers_or_results_supplied":false,"numerical_PASS_claimed":false,"whole_geometry_PASS_claimed":false,"ordinary_convection_or_cadence_PASS_claimed":false,"gates_updated":false});
    let exit_code;
    if scope.admissible {
        if let Some(typed) = compiled.model {
            result["typed_geometry_inputs"] = fields::typed(&typed);
            let mut caller = fields::Caller::new(&typed.version.to_string());
            result["caller_before"] = caller.snapshot();
            let model = SimulationModel::from_typed(typed);
            result["actual_calls"]["initialize_heat_balance_state_with_initialization_context"] =
                json!(1);
            match caller.initialize(&model) {
                Ok(state) => {
                    result["selected_owners"] = fields::selected(&state);
                    result["initializer_result"] = json!({"returned":true,"error":null});
                    exit_code = 0;
                }
                Err(error) => {
                    result["initializer_result"] = fields::error(&error);
                    result["selected_owners"] = fields::unavailable(
                        "actual initializer Err drops State; partial geometry is not reconstructed",
                    );
                    exit_code = 2;
                }
            }
            result["caller_after"] = caller.snapshot();
        } else {
            result["initializer_result"] = json!({"returned":false,"reason":"compiler returned no TypedModel","counted_as_PASS":false});
            exit_code = 2;
        }
    } else {
        result["initializer_result"] = json!({"returned":false,"reason":"actual Scope A preflight unavailable","counted_as_PASS":false});
        exit_code = 2;
    }
    bindings.unchanged()?;
    println!("{}", serde_json::to_string(&result)?);
    std::process::exit(exit_code);
}
