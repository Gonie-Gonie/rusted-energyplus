//! Compact serialization of optional observations from the actual Rust pipeline.

use crate::{RunConfig, RunError, RunExitCode};
use ep_runtime::psychrometrics::EnergyPlusCpAirCacheState;
use ep_runtime::psychrometrics::production_trace::{
    ExecutionContext, PsychrometricProductionTrace,
};
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::{BufWriter, Write};

#[derive(Serialize)]
struct TraceArtifact<'a> {
    schema: &'static str,
    capture_source: &'static str,
    thread_coverage: &'static str,
    phase_contract: &'static str,
    context_contract: &'static str,
    input_origin: &'static str,
    input_order: Value,
    event_limit: usize,
    unique_tuple_limit: usize,
    total_call_count: u64,
    recorded_call_count: usize,
    omitted_call_count: u64,
    truncation_reason: Option<&'static str>,
    complete_on_collecting_thread: bool,
    routine_counts: &'a BTreeMap<&'static str, u64>,
    dictionary: Vec<Value>,
    ordered_ids: &'a [u32],
}

pub(crate) fn write_production_trace(
    config: &RunConfig,
    trace: &PsychrometricProductionTrace,
) -> Result<(), RunError> {
    let dictionary = trace.calls.iter().map(|call| json!({
        "routine": call.routine,
        "phase": call.phase,
        "caller": { "file": call.caller.file, "line": call.caller.line, "column": call.caller.column },
        "context": call.context.map(execution_context),
        "input_bits": call.input_bits.iter().map(|bits| format!("{bits:016x}")).collect::<Vec<_>>(),
        "inputs": call.input_bits.iter().map(|bits| finite_number(f64::from_bits(*bits))).collect::<Vec<_>>(),
        "result_bits": format!("{:016x}", call.result_bits),
        "result": finite_number(f64::from_bits(call.result_bits)),
        "cp_cache": call.cp_cache.as_ref().map(|cache| json!({
            "before": cache_state(cache.before), "after": cache_state(cache.after), "hit": cache.hit,
        })),
    })).collect();
    let artifact = TraceArtifact {
        schema: "psychrometrics-calls.v2",
        capture_source: "actual-rust-pipeline-kernel-hooks",
        thread_coverage: "collecting-thread-only",
        phase_contract: "actual-Rust-pipeline-scope; not-an-EnergyPlus-stage-claim",
        context_contract: "actual-Rust-invocation-or-referenced-output; existing-runtime-axes; no-inferred-EP-system-order",
        input_origin: "actual-Rust-call-arguments; no-EnergyPlus-result-or-fixture-seeding",
        input_order: json!({
            "PsyRhoAirFnPbTdbW": ["P_Pa", "T_C", "W_kg_kg"],
            "PsyRhoAirFnPbTdbW_fast": ["P_Pa", "T_C", "W_kg_kg"],
            "PsyCpAirFnW": ["W_kg_kg"], "PsyCpAirFnW_fast": ["W_kg_kg"],
            "PsyHFnTdbW": ["T_C", "W_kg_kg"], "PsyHFnTdbW_fast": ["T_C", "W_kg_kg"],
            "PsyTdbFnHW": ["H_J_kg", "W_kg_kg"], "PsyWFnTdbH": ["T_C", "H_J_kg"],
        }),
        event_limit: trace.event_limit,
        unique_tuple_limit: trace.unique_tuple_limit,
        total_call_count: trace.total_call_count,
        recorded_call_count: trace.ordered_ids.len(),
        omitted_call_count: trace.omitted_call_count,
        truncation_reason: trace.truncation_reason,
        complete_on_collecting_thread: trace.omitted_call_count == 0,
        routine_counts: &trace.routine_counts,
        dictionary,
        ordered_ids: &trace.ordered_ids,
    };
    let path = config.output_dir.join("psychrometrics-calls.json");
    let write = || -> Result<(), String> {
        let file = std::fs::File::create(&path)
            .map_err(|error| format!("failed to create {}: {error}", path.display()))?;
        let mut writer = BufWriter::new(file);
        // Stream the typed ID slice compactly instead of allocating millions of
        // JSON Values or a second large pretty-printed representation.
        serde_json::to_writer(&mut writer, &artifact)
            .map_err(|error| format!("failed to serialize {}: {error}", path.display()))?;
        writer
            .write_all(b"\n")
            .and_then(|()| writer.flush())
            .map_err(|error| format!("failed to write {}: {error}", path.display()))
    };
    write().map_err(|message| RunError {
        exit_code: RunExitCode::OutputExport,
        message,
    })
}

pub(crate) fn execution_context(context: ExecutionContext) -> Value {
    json!({
        "scope": context.scope,
        "zone_timestep": context.zone_timestep.map(|zone| json!({
            "hour_index": zone.hour_index, "zone_timestep": zone.zone_timestep,
            "zone_steps_per_hour": zone.zone_steps_per_hour, "sample_index": zone.sample_index,
            "timestep_seconds_bits": format!("{:016x}", zone.timestep_seconds_bits),
            "calendar": zone.calendar.map(|calendar| json!({
                "year": calendar.year, "month": calendar.month, "day_of_month": calendar.day_of_month,
                "day_of_sim": calendar.day_of_sim, "hour_ending": calendar.hour_ending,
            })),
            "environment": zone.environment.map(|environment| json!({
                "environment_index": environment.environment_index, "sample_index": environment.sample_index,
                "simulation_timestep": environment.simulation_timestep, "zone_timestep": environment.zone_timestep,
                "start_minute_bits": format!("{:016x}", environment.start_minute_bits),
                "end_minute_bits": format!("{:016x}", environment.end_minute_bits),
                "current_time_hours_bits": format!("{:016x}", environment.current_time_hours_bits),
                "begin_environment": environment.begin_environment, "end_environment": environment.end_environment,
                "begin_day": environment.begin_day, "end_day": environment.end_day,
                "begin_hour": environment.begin_hour, "end_hour": environment.end_hour,
            })),
        })),
        "system_call": context.system_call.map(|system| json!({
            "schedule_sample_index": system.schedule_sample_index, "begin_environment": system.begin_environment,
            "timestep_seconds_bits": format!("{:016x}", system.timestep_seconds_bits),
        })),
    })
}

fn finite_number(value: f64) -> Value {
    if value.is_finite() {
        json!(value)
    } else {
        Value::Null
    }
}
fn cache_state(state: EnergyPlusCpAirCacheState) -> Value {
    json!({
        "dw_save": finite_number(state.humidity_ratio_kg_per_kg), "cpa_save": finite_number(state.specific_heat_j_per_kg_k),
        "dw_save_bits": format!("{:016x}", state.humidity_ratio_kg_per_kg.to_bits()),
        "cpa_save_bits": format!("{:016x}", state.specific_heat_j_per_kg_k.to_bits()),
    })
}
