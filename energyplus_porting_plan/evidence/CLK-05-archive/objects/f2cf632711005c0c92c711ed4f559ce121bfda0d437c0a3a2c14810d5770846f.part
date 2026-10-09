//! Execute existing public calls only; no native outputs or scientific formulas.
use super::{Result, fields, seeds, inputs::{array, configuration, flag, integer, real, require, seed_caller, text}};
use ep_runtime::weather::day::{WeatherDayState, WeatherSession};
use serde_json::{Value, json};
use std::path::Path;

fn outcome(status: &str, fatal: Option<bool>, error: Option<String>) -> Value {
    json!({"status":status,"source_fatal":fatal,"actual_rust_error":error,
        "native_exception_text_parity_claimed":false})
}
fn seed_clock(state: &mut WeatherDayState, declared: &Value) -> Result<()> {
    let steps = integer(&declared["time_steps_per_hour"])?;
    require([1, 3, 4].contains(&steps), "bounded one/three/four-step clock required")?;
    let (zone, seconds, minutes) = match steps {
        1 => ("3ff0000000000000", "40ac200000000000", 60),
        3 => ("3fd5555555555555", "4092c00000000000", 20),
        _ => ("3fd0000000000000", "408c200000000000", 15),
    };
    require(declared["TimeStepZone"]["bits"] == zone
        && declared["TimeStepFraction"] == declared["TimeStepZone"]
        && declared["TimeStepZoneSec"]["bits"] == seconds
        && integer(&declared["MinutesInTimeStep"])? == minutes, "literal caller clock differs")?;
    state.global.time_steps_in_hour = i32::try_from(steps)?;
    state.global.time_step_zone = real(&declared["TimeStepZone"])?;
    state.global.time_step_zone_sec = real(&declared["TimeStepZoneSec"])?;
    state.global.minutes_in_time_step = i32::try_from(minutes)?;
    // Pure Setup does not assign weather.TimeStepFraction from its descriptor.
    Ok(())
}

pub(super) fn setup(input: &Value) -> Result<Value> {
    let mut state = WeatherDayState::default();
    seed_clock(&mut state, input)?;
    let before = fields::state_snapshot(&state, None, None, true);
    let result = state.setup_interpolation_values();
    let observed = match result {
        Ok(()) => outcome("source_returned", Some(false), None),
        Err(error) => outcome(if error.is_source_fatal() {"source_fatal"} else {"rust_admission_error"},
            Some(error.is_source_fatal()), Some(error.to_string())),
    };
    Ok(json!({"id":input["id"],"kind":"SetupInterpolationValues","requested_operation":input,
        "before":before,"after":fields::state_snapshot(&state,None,None,true),
        "actual_rust_invoked":true,"call_outcome":observed,
        "actual_public_Rust_API":"WeatherDayState::setup_interpolation_values",
        "declared_TimeStepFraction_applied_to_pure_Setup_weather_owner":false,
        "whole_native_Factory_or_preparation_observed":false,"numerical_PASS_claimed":false}))
}

pub(super) fn sequence(input: &Value, idf: &[u8], weather: &Path, bytes: Vec<u8>) -> Result<Value> {
    let config = configuration(input, idf, weather)?;
    let mut session = WeatherSession::new(bytes, config)?;
    let constructor = fields::session_snapshot(&session, weather, true);
    let declared = &input["native_preparation"];
    require(integer(&declared["time_steps_per_hour"])? == integer(&input["time_steps_per_hour"])?,
        "declared preparation/input steps differ")?;
    seed_clock(&mut session.state, declared)?;
    session.setup_interpolation_values()?;
    session.state.weather.time_step_fraction = real(&declared["TimeStepFraction"])?;
    session.state.global.begin_sim_flag = flag(&declared["BeginSimFlag"])?;
    session.state.global.do_weath_sim = flag(&declared["DoWeathSim"])?;
    session.state.global.do_des_day_sim = flag(&declared["DoDesDaySim"])?;
    session.state.weather.get_environment_first_call = flag(&declared["GetEnvironmentFirstCall"])?;
    session.state.weather.get_branch_input_one_time_flag = flag(&declared["GetBranchInputOneTimeFlag"])?;
    session.state.weather.water_mains_parameter_report = flag(&declared["WaterMainsParameterReport"])?;
    let prepared = fields::session_snapshot(&session, weather, true);
    let before_context = prepared.clone();
    seeds::context(&mut session.state, input)?;
    let after_context = fields::session_snapshot(&session, weather, true);
    let mut operations = Vec::new();
    let mut stop: Option<String> = None;
    for op in array(&input["operations"])? {
        let kind = text(&op["kind"])?;
        let day = op["caller"].get("BeginDayFlag").map(flag).transpose()?.unwrap_or(session.state.global.begin_day_flag);
        let env = op["caller"].get("BeginEnvrnFlag").map(flag).transpose()?.unwrap_or(session.state.global.begin_envrn_flag);
        let full = kind == "GetNextEnvironment" || (kind == "InitializeWeather" && (day || env));
        let before_caller = fields::session_snapshot(&session, weather, full);
        if let Some(reason) = &stop {
            operations.push(json!({"id":op["id"],"kind":op["kind"],"requested_operation":op,
                "full_solar_grid_observation_declared":full,"before_caller":before_caller,
                "before":before_caller,"after":before_caller,"actual_rust_invoked":false,
                "call_outcome":{"status":"not-invoked-after-prior-Rust-outcome","source_fatal":null,"skip_reason":reason}}));
            continue;
        }
        seed_caller(&mut session.state.global, &op["caller"])?;
        let before = fields::session_snapshot(&session, weather, full);
        let stamp = session.print_environment_stamp;
        let mut returned = None;
        let result = match kind {
            "GetNextEnvironment" => session.get_next_environment().map(|value| {returned = Some(value);}),
            "InitializeWeather" => session.initialize_weather(),
            _ => return Err("unknown closed solar boundary operation".into()),
        };
        let observed = match result {
            Ok(()) => {
                if returned == Some(false) {stop = Some("actual-selected-environment-unavailable".into());}
                outcome("source_returned", Some(false), None)
            },
            Err(error) => {
                let fatal = error.is_source_fatal();
                stop = Some(if fatal {"actual-source-fatal"} else {"actual-Rust-admission-error"}.into());
                outcome(if fatal {"source_fatal"} else {"rust_admission_error"}, Some(fatal), Some(error.to_string()))
            },
        };
        operations.push(json!({"id":op["id"],"kind":kind,"requested_operation":op,
            "full_solar_grid_observation_declared":full,"before_caller":before_caller,"before":before,
            "after":fields::session_snapshot(&session,weather,full),"actual_rust_invoked":true,
            "call_outcome":observed,"returned_bool":returned,"Available":session.available,
            "ErrorsFound":session.errors_found,"print_environment_stamp_before":stamp,
            "print_environment_stamp_after":session.print_environment_stamp}));
    }
    Ok(json!({"id":input["id"],"lane":input["lane"],"input":input["input"],"weather_input":input["weather"],
        "declared_native_preparation":declared,"declared_run_period_input":input["run_period"],
        "allocated_defaults":constructor.clone(),"actual_Rust_after_public_constructor":constructor,
        "allocated_defaults_boundary":"actual existing WeatherSession::new return, after its genuine Setup; whole native Factory/preparation unavailable",
        "prepared":prepared,"before_declared_caller_context":before_context,"after_declared_caller_context":after_context,
        "declared_solar_controls":input["solar_controls"],"solar_controls_applied_to_existing_owner":false,
        "requested_control_equivalence_to_existing_configuration_claimed":false,
        "operations":operations,"final_state":fields::session_snapshot(&session,weather,true),
        "constructor":fields::unavailable("whole native Factory has no existing Rust counterpart"),
        "after_constant_initialization":fields::unavailable("native constant initialization owner unavailable"),
        "preparation_calls":fields::unavailable("whole native eight-call preparation has no existing Rust counterpart"),
        "configuration_metadata_preview_separate_from_live_cursor":true,
        "metadata_preview_weather_values_supplied_to_live_owner":false,"unavailable_counted_as_PASS":false}))
}
