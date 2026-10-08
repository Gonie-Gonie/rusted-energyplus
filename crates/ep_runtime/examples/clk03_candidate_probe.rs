//! Input-only observations of actual typed weather daily transport and lifecycle.
//! No native result, legacy result, expected answer, comparer or fixture is read.

#[path = "clk02_probe_support/digest.rs"]
mod digest;
#[path = "clk03_candidate_support/fields.rs"]
mod fields;
#[path = "clk03_candidate_support/inputs.rs"]
mod inputs;
#[path = "clk03_candidate_support/seeds.rs"]
mod seeds;

use ep_runtime::weather::day::{
    WeatherDayState, WeatherDayValues, WeatherSession, update_weather_data,
};
use ep_runtime::weather::raw::RawDayReadMode;
use inputs::{array, bound_file, flag, integer, relative, require, text};
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

fn session_snapshot(session: &WeatherSession, weather: &Path) -> Value {
    let mut value = fields::snapshot(&session.state, Some(&session.cursor), Some(weather));
    value["selected_environment_context"] =
        json!({"CurrentCycle":session.current_cycle,"SetWeekDays":session.set_week_days});
    value["actual_rust_available"] = json!(session.available);
    value["actual_rust_errors_found"] = json!(session.errors_found);
    value["actual_rust_print_environment_stamp"] = json!(session.print_environment_stamp);
    value
}

fn skipped(before: Value, op: &Value, reason: &str) -> Value {
    json!({"id":op["id"],"kind":op["kind"],"requested_operation":op,"before_caller":before,
        "before":before,"after":before,"actual_rust_invoked":false,
        "call_outcome":{"status":"not-invoked-after-prior-source-outcome","source_fatal":null,
            "skip_reason":reason,"exception_message":null,"actual_rust_error_debug":null}})
}

fn handoff_sequence(input: &Value) -> Result<Value> {
    require(
        integer(&input["time_steps_per_hour"])? == 4,
        "bounded four-step handoff required",
    )?;
    let mut state = WeatherDayState::default();
    let constructor = fields::snapshot(&state, None, None);
    state.global.time_steps_in_hour = 4;
    state.today_values = WeatherDayValues::allocated(4)?;
    state.tomorrow_values = WeatherDayValues::allocated(4)?;
    let allocated = fields::snapshot(&state, None, None);
    seeds::seed(&mut state, &input["initial"])?;
    let prepared = fields::snapshot(&state, None, None);
    // Caller-owned flag: the pure UpdateWeatherData API never receives it.
    let print_environment_stamp = false;
    let mut operations = Vec::new();
    for op in array(&input["operations"])? {
        require(
            text(&op["kind"])? == "UpdateWeatherData",
            "pure handoff operation kind required",
        )?;
        let before_caller = fields::snapshot(&state, None, None);
        seeds::seed(&mut state, &op["before"])?;
        let before = fields::snapshot(&state, None, None);
        let stamp_before = print_environment_stamp;
        update_weather_data(&mut state);
        operations.push(json!({"id":op["id"],"kind":op["kind"],"requested_operation":op,
            "before_caller":before_caller,"before":before,"after":fields::snapshot(&state,None,None),
            "actual_rust_invoked":true,"returned_bool":null,"Available":false,"ErrorsFound":false,
            "print_environment_stamp_before":stamp_before,"print_environment_stamp_after":print_environment_stamp,
            "call_outcome":{"status":"source_returned","source_fatal":false,"exception_message":null,"actual_rust_error_debug":null}}));
    }
    Ok(
        json!({"id":input["id"],"lane":"typed-whole-UpdateWeatherData-literal-input-canaries",
        "constructor":constructor,"allocated_defaults":allocated,"prepared":prepared,"operations":operations,
        "final_state":fields::snapshot(&state,None,None),"carrier_generation_science_executed":false,
        "whole_source_constructor_observed":false,"actual_selected_Rust_constructor_observed":true}),
    )
}

fn weather_sequence(
    input: &Value,
    idf_bytes: &[u8],
    weather: &Path,
    declared: &Value,
) -> Result<Value> {
    require(
        integer(&input["time_steps_per_hour"])? == 4 && flag(&input["prepared_environment_lane"])?,
        "bounded prepared four-step weather required",
    )?;
    let constructor = fields::snapshot(&WeatherDayState::default(), None, None);
    let configuration = inputs::configuration(input, idf_bytes, weather)?;
    let mut session = WeatherSession::new(std::fs::read(weather)?, configuration)?;
    let allocated = session_snapshot(&session, weather);
    // These are declared caller preparation controls, not native observed answers.
    session.state.weather.get_environment_first_call = flag(&declared["GetEnvironmentFirstCall"])?;
    session.state.weather.get_branch_input_one_time_flag =
        flag(&declared["GetBranchInputOneTimeFlag"])?;
    session.state.weather.water_mains_parameter_report =
        flag(&declared["WaterMainsParameterReport"])?;
    session.state.global.begin_sim_flag = flag(&declared["BeginSimFlag"])?;
    session.state.global.do_weath_sim = flag(&declared["DoWeathSim"])?;
    session.state.global.do_des_day_sim = flag(&declared["DoDesDaySim"])?;
    session.state.weather.envrn = session.configuration.design_day_count;
    let prepared = session_snapshot(&session, weather);
    let mut operations = Vec::new();
    let mut stop: Option<String> = None;
    for op in array(&input["operations"])? {
        let before_caller = session_snapshot(&session, weather);
        if let Some(reason) = &stop {
            operations.push(skipped(before_caller, op, reason));
            continue;
        }
        seeds::seed(&mut session.state, &json!({"global":op["caller"]}))?;
        let before = session_snapshot(&session, weather);
        let stamp_before = session.print_environment_stamp;
        let kind = text(&op["kind"])?;
        let mut returned = None;
        let result = match kind {
            "UpdateWeatherData" => {
                update_weather_data(&mut session.state);
                Ok(())
            }
            "GetNextEnvironment" => session.get_next_environment().map(|value| {
                returned = Some(value);
            }),
            "InitializeWeather" => session.initialize_weather(),
            "ReadWeatherForDay" => {
                let day = integer(&op["day_to_read"])?;
                require(day > 0, "positive day-to-read required")?;
                session.read_weather_day(
                    if day == 1 {
                        RawDayReadMode::FirstDay
                    } else {
                        RawDayReadMode::NextDay
                    },
                    day as usize,
                    flag(&op["backspace_after_read"])?,
                )
            }
            _ => return Err(format!("unknown declared operation {kind}").into()),
        };
        let outcome = match result {
            Ok(()) => {
                if returned == Some(false) {
                    stop = Some("actual-Rust-selected-environment-unavailable".into());
                }
                json!({"status":"source_returned","source_fatal":false,"exception_message":null,"actual_rust_error_debug":null})
            }
            Err(error) => {
                let source_fatal = error.is_source_fatal();
                stop = Some(
                    if source_fatal {
                        "actual-Rust-source-fatal"
                    } else {
                        "actual-Rust-admission-error"
                    }
                    .into(),
                );
                json!({"status":if source_fatal {"source_fatal"}else{"rust_admission_error"},"source_fatal":source_fatal,
                    "exception_message":error.to_string(),"actual_rust_error_debug":format!("{error:?}")})
            }
        };
        operations.push(json!({"id":op["id"],"kind":op["kind"],"requested_operation":op,
            "before_caller":before_caller,"before":before,"after":session_snapshot(&session,weather),
            "actual_rust_invoked":true,"call_outcome":outcome,"returned_bool":returned,
            "Available":session.available,"ErrorsFound":session.errors_found,
            "print_environment_stamp_before":stamp_before,"print_environment_stamp_after":session.print_environment_stamp}));
    }
    Ok(
        json!({"id":input["id"],"input":input["input"],"weather_input":input["weather"],
        "declared_run_period_input":input["run_period"],"constructor":constructor,"allocated_defaults":allocated,
        "prepared":prepared,"operations":operations,"final_state":session_snapshot(&session,weather),
        "declared_native_preparation":declared,"registered_whole_simulation_initialization_claimed":false,
        "processed_weather_numerical_fields_paired":false,"source_internal_record_index_observed":false,
        "actual_preparation_scope":"existing input/calendar adapters then actual WeatherSession; no whole native preparation calls claimed"}),
    )
}

fn run() -> Result<()> {
    let mut args = std::env::args_os().skip(1);
    let supplied = args
        .next()
        .ok_or("usage: clk03_candidate_probe <input-only-request.json>")?;
    require(
        args.next().is_none(),
        "exactly one request argument required",
    )?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .ok_or("repository root unavailable")?
        .canonicalize()?;
    let request_path = PathBuf::from(supplied).canonicalize()?;
    require(
        request_path.starts_with(&root),
        "contained request path required",
    )?;
    let bytes = std::fs::read(&request_path)?;
    let request: Value = serde_json::from_slice(&bytes)?;
    require(
        text(&request["schema"])? == "clk03-helper-cases.v1"
            && request["expected_values_supplied"] == false
            && request["expected_exits_supplied"] == false
            && request["draft_not_frozen"] != true,
        "final input-only request required",
    )?;
    let (_, scope) = bound_file(&root, &request["fixed_scope"])?;
    let mut contracts = serde_json::Map::new();
    for name in ["source", "cases", "tolerances"] {
        let relative = format!("energyplus_porting_plan/contracts/CLK-03-{name}.json");
        let file = root.join(&relative).canonicalize()?;
        require(file.starts_with(&root), "contained contract required")?;
        contracts.insert(
            name.into(),
            json!({"path":relative,"sha256":digest::sha256(&std::fs::read(file)?)}),
        );
    }
    let handoff_inputs = array(&request["handoff_sequences"])?;
    let weather_inputs = array(&request["weather_sequences"])?;
    require(
        handoff_inputs.len() == 4 && weather_inputs.len() == 10,
        "frozen sequence cardinality differs",
    )?;
    let prepared = weather_inputs
        .iter()
        .map(|item| {
            let (_, idf) = bound_file(&root, &item["input"])?;
            let (weather, _) = bound_file(&root, &item["weather"])?;
            Ok((idf, weather))
        })
        .collect::<Result<Vec<_>>>()?;
    let handoffs = handoff_inputs
        .iter()
        .map(handoff_sequence)
        .collect::<Result<Vec<_>>>()?;
    let weather = weather_inputs
        .iter()
        .zip(&prepared)
        .map(|(item, (idf, path))| {
            weather_sequence(item, idf, path, &request["native_preparation"])
        })
        .collect::<Result<Vec<_>>>()?;
    let handoff_calls = handoff_inputs
        .iter()
        .map(|row| array(&row["operations"]).map(<[Value]>::len))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<usize>();
    let weather_calls = weather_inputs
        .iter()
        .map(|row| array(&row["operations"]).map(<[Value]>::len))
        .collect::<Result<Vec<_>>>()?
        .iter()
        .sum::<usize>();
    require(
        handoff_calls + weather_calls == 51,
        "all frozen requested operations required",
    )?;
    let mut calls = BTreeMap::<String, usize>::new();
    let mut skipped = 0;
    for row in handoffs.iter().chain(&weather) {
        for op in array(&row["operations"])? {
            if op["actual_rust_invoked"] == true {
                *calls.entry(text(&op["kind"])?.into()).or_default() += 1;
            } else {
                skipped += 1;
            }
        }
    }
    for input in weather_inputs {
        bound_file(&root, &input["input"])?;
        bound_file(&root, &input["weather"])?;
    }
    require(
        std::fs::read(&request_path)? == bytes,
        "request changed during actual Rust calls",
    )?;
    let output = json!({"schema":"clk03-rust-probe-results.v1","contracts":contracts,
        "actual_request":{"path":relative(&root,&request_path)?,"sha256":digest::sha256(&bytes)},
        "fixed_scope":{"path":request["fixed_scope"]["path"],"sha256":digest::sha256(&scope)},
        "handoff_sequences":handoffs,"weather_sequences":weather,
        "requested_counts":{"handoff_sequences":handoff_inputs.len(),"weather_sequences":weather_inputs.len(),
            "handoff_operations":handoff_calls,"weather_operations":weather_calls,"total_sequences":14,"total_operations":51},
        "actual_operation_counts":calls,"actual_operation_invocations":51-skipped,"actual_operations_skipped":skipped,
        "complete":true,"expected_answers_supplied":false,"native_reference_results_read":false,"legacy_reference_results_read":false,
        "gates_updated":false,"probe_changes_production_sources":false,"processed_weather_physics_compared":false,
        "whole_native_constructor_or_private_state_claimed":false,"original_error_text_peer_parity_claimed":false});
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}
