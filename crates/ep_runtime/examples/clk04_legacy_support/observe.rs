//! Full declared callback history through the actual existing public lifecycle.
use super::{Result,fields,inputs::{array,configuration,flag,integer,real,require,seed_caller,text}};
use ep_runtime::weather::day::WeatherSession;
use serde_json::{Value,json};
use std::path::Path;

fn outcome(status: &str, fatal: Option<bool>, error: Option<String>) -> Value {
    json!({"status":status,"source_fatal":fatal,"actual_rust_error":error,
        "native_exception_text_parity_claimed":false})
}
pub(super) fn sequence(input: &Value, idf: &[u8], weather: &Path, bytes: Vec<u8>) -> Result<Value> {
    let config = configuration(input,idf,weather)?;
    let mut session = WeatherSession::new(bytes,config)?;
    let constructor = fields::snapshot(&session,weather,true);
    let declared = &input["native_preparation"];
    let steps = integer(&input["time_steps_per_hour"])?;
    require(integer(&declared["time_steps_per_hour"])? == steps, "preparation/input step count differs")?;
    let fraction = real(&declared["TimeStepFraction"])?;
    let zone = real(&declared["TimeStepZone"])?;
    let expected_bits = if steps == 1 {0x3ff0000000000000} else {0x3fd0000000000000};
    require(zone.to_bits() == expected_bits && fraction.to_bits() == expected_bits,
        "literal one/four-step input timing differs")?;
    // Only existing fields receive literal caller-preparation assignments.
    // Seconds/minutes remain unavailable; no missing owner is synthesized.
    session.state.global.time_step_zone = zone;
    session.state.weather.time_step_fraction = fraction;
    session.state.global.begin_sim_flag = flag(&declared["BeginSimFlag"])?;
    session.state.global.do_weath_sim = flag(&declared["DoWeathSim"])?;
    session.state.global.do_des_day_sim = flag(&declared["DoDesDaySim"])?;
    session.state.weather.get_environment_first_call = flag(&declared["GetEnvironmentFirstCall"])?;
    session.state.weather.get_branch_input_one_time_flag = flag(&declared["GetBranchInputOneTimeFlag"])?;
    session.state.weather.water_mains_parameter_report = flag(&declared["WaterMainsParameterReport"])?;
    let prepared = fields::snapshot(&session,weather,true);
    let mut operations = Vec::new();
    let mut stop: Option<String> = None;
    for op in array(&input["operations"])? {
        let kind = text(&op["kind"])?;
        // Full-grid retention follows effective partial caller inputs, with
        // omitted flags retained from the actual incoming owner. No output mask
        // controls which methods execute or which source flags are assigned.
        let day = op["caller"].get("BeginDayFlag").map(flag).transpose()?
            .unwrap_or(session.state.global.begin_day_flag);
        let environment = op["caller"].get("BeginEnvrnFlag").map(flag).transpose()?
            .unwrap_or(session.state.global.begin_envrn_flag);
        let full_grids = kind == "GetNextEnvironment" || (kind == "InitializeWeather" && (day || environment));
        let before_caller = fields::snapshot(&session,weather,full_grids);
        if let Some(reason) = &stop {
            operations.push(json!({"id":op["id"],"kind":op["kind"],"requested_operation":op,
                "before_caller":before_caller,"before":before_caller,"after":before_caller,
                "actual_rust_invoked":false,"call_outcome":{
                    "status":"not-invoked-after-prior-existing-Rust-outcome","source_fatal":null,"skip_reason":reason}}));
            continue;
        }
        seed_caller(&mut session.state.global,&op["caller"])?;
        let before = fields::snapshot(&session,weather,full_grids);
        let stamp = session.print_environment_stamp;
        let mut returned = None;
        if kind == "SetCurrentWeather" {
            // Read actual Today after the preceding genuine existing Initialize.
            // No replacement function, copied RHS, computed current state or
            // native callback runs; omitted owners remain unavailable.
            operations.push(json!({"id":op["id"],"kind":kind,"requested_operation":op,
                "before_caller":before_caller,"before":before,"after":fields::snapshot(&session,weather,full_grids),
                "actual_rust_invoked":false,"actual_Today_getter_invoked":true,
                "legacy_selected_today_slot":fields::selected(&session),
                "selected_observation":op["selected_observation"],
                "call_outcome":outcome("unavailable-existing-current-weather-owner",None,None),
                "current_environment_observation_available":false,"counted_as_PASS":false,
                "returned_bool":null,"Available":session.available,"ErrorsFound":session.errors_found,
                "print_environment_stamp_before":stamp,"print_environment_stamp_after":session.print_environment_stamp}));
            continue;
        }
        let result = match kind {
            "GetNextEnvironment" => session.get_next_environment().map(|value|{returned=Some(value);}),
            "InitializeWeather" => session.initialize_weather(),
            _ => return Err("unknown closed lifecycle operation".into()),
        };
        let observed = match result {
            Ok(()) => {
                if returned == Some(false) {stop=Some("actual-existing-selected-environment-unavailable".into());}
                outcome("source_returned",Some(false),None)
            }
            Err(error) => {
                let fatal = error.is_source_fatal();
                stop=Some(if fatal {"actual-existing-source-fatal"} else {"actual-existing-admission-error"}.into());
                outcome(if fatal {"source_fatal"} else {"rust_admission_error"},Some(fatal),Some(error.to_string()))
            }
        };
        operations.push(json!({"id":op["id"],"kind":kind,"requested_operation":op,
            "before_caller":before_caller,"before":before,"after":fields::snapshot(&session,weather,full_grids),
            "actual_rust_invoked":true,"call_outcome":observed,"returned_bool":returned,
            "Available":session.available,"ErrorsFound":session.errors_found,
            "print_environment_stamp_before":stamp,"print_environment_stamp_after":session.print_environment_stamp}));
    }
    Ok(json!({"id":input["id"],"lane":input["lane"],"input":input["input"],"weather_input":input["weather"],
        "declared_native_preparation":declared,"declared_run_period_input":input["run_period"],
        "existing_Rust_after_public_constructor":constructor,"prepared":prepared,"operations":operations,
        "final_state":fields::snapshot(&session,weather,true),"native_constructor":null,"native_allocated_defaults":null,
        "whole_native_preparation_calls_available":false,"native_SetCurrentWeather_counterpart_available":false,
        "configuration_metadata_preview_separate_from_live_cursor":true,
        "metadata_preview_weather_values_supplied_to_live_owner":false,
        "full_17_field_generation_parity_claimed":false,"unavailable_counted_as_PASS":false}))
}
