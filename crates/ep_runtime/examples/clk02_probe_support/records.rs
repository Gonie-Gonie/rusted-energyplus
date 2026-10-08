//! Execute actual raw owners, preserving per-call canaries and shared counters.

use super::{Result, digest, dto, inputs};
use ep_runtime::weather::raw::{
    RawEpwHeaderState, RawEpwParserState, RawRecordFailure, interpret_weather_data_line,
};
use serde_json::{Value, json};

pub(super) fn call(owner: &mut RawEpwHeaderState, canaries: &Value, line: &str) -> Result<Value> {
    let mut outputs = inputs::outputs(canaries)?;
    let before = dto::raw(&outputs);
    let missed_before = owner.weather_code_missed_count;
    // The record method receives the actual current fields of this same owner.
    // Both stores return to that owner before its next observed phase.
    let mut parser = RawEpwParserState {
        end_day_of_month: owner.month_ends,
        weather_code_missed_count: owner.weather_code_missed_count,
    };
    let result = interpret_weather_data_line(&mut parser, line, &mut outputs);
    owner.month_ends = parser.end_day_of_month;
    owner.weather_code_missed_count = parser.weather_code_missed_count;
    let outcome = match result {
        Ok(()) => dto::returned(),
        Err(RawRecordFailure::OutsideBoundedDomain(reason)) => {
            return Err(format!("raw probe input outside bounded source domain: {reason}").into());
        }
        Err(error) => dto::fatal(&error.to_string()),
    };
    Ok(
        json!({"before":before,"after":dto::raw(&outputs),"call_outcome":outcome,
        "weather_code_missed_count_before":missed_before,
        "weather_code_missed_count_after":owner.weather_code_missed_count,
        "public_storage_defined":true,"individual_source_write_events_observed":false,
        "fatal_output_policy":"actual-reference-arguments-retain-defined-caller-storage; source-local-write-completion-not-inferred"}),
    )
}

pub(super) fn sequence(item: &Value) -> Result<Value> {
    inputs::require(
        inputs::text(&item["route"])? == "whole-original-InterpretWeatherDataLine"
            && inputs::flag(&item["fresh_owner"])?
            && inputs::flag(&item["reset_declared_output_canaries_before_each_call"])?,
        "raw sequence route/preparation policy differs",
    )?;
    let mut owner = RawEpwHeaderState::default();
    let constructor = dto::header(&owner);
    owner.weather_code_missed_count = inputs::integer(&item["prepared_weather_code_missed_count"])?;
    owner.month_ends = inputs::int_array(&item["prepared_EndDayOfMonth"])?;
    let prepared = dto::header(&owner);
    let operation_inputs = item["operations"]
        .as_array()
        .ok_or("raw operations must be an array")?;
    let mut operations = Vec::with_capacity(operation_inputs.len());
    for operation in operation_inputs {
        inputs::require(
            operation["expected_values_supplied"] == false
                && operation["expected_exit_supplied"] == false,
            "raw diagnostic must supply no answers",
        )?;
        let line = inputs::line(operation, "line_utf8", "line_sha256")?;
        let mut row = call(&mut owner, &item["initial_outputs"], &line)?;
        row["record_id"] = operation["record_id"].clone();
        row["kind"] = operation["kind"].clone();
        row["input_line_utf8"] = json!(line);
        row["input_line_sha256"] = json!(digest::sha256(line.as_bytes()));
        operations.push(row);
    }
    Ok(
        json!({"sequence_id":inputs::text(&item["sequence_id"])?,"route":item["route"],
        "constructor":constructor,"prepared":prepared,"operations":operations,
        "final_state":dto::header(&owner),"actual_wrapper_root_invocations":operation_inputs.len(),
        "physics_executed":false}),
    )
}
