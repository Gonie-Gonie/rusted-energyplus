//! Serialize selected outermost property calls without supplying runtime inputs.

use crate::{RunConfig, RunError, RunExitCode};
use ep_runtime::psychrometrics::psy02_trace::Psy02ProductionTrace;
use serde::Serialize;
use serde_json::{Value, json};
use std::collections::BTreeMap;
use std::io::{BufWriter, Write};

#[derive(Serialize)]
struct Artifact<'a> {
    schema: &'static str,
    capture_source: &'static str,
    thread_coverage: &'static str,
    root_policy: &'static str,
    phase_contract: &'static str,
    input_origin: &'static str,
    input_order: Value,
    logical_cache_capacity_each: usize,
    cache_defaults: Value,
    event_limit: usize,
    unique_tuple_limit: usize,
    total_root_count: u64,
    recorded_root_count: usize,
    omitted_root_count: u64,
    truncation_reason: Option<&'static str>,
    complete_on_collecting_thread: bool,
    routine_counts: &'a BTreeMap<&'static str, u64>,
    initial_state: Value,
    final_state: Value,
    final_caches: Value,
    dictionary: Vec<Value>,
    ordered_ids: &'a [u32],
    unpaired_source_observations: [&'static str; 3],
}

fn requested_tags(routine: &str, inputs: &[u64]) -> Option<Value> {
    let signed = |index: usize, shift| (inputs[index] as i64) >> shift;
    match routine {
        "PsyTwbFnTdbWPb" => Some(json!({
            "i_tdb": inputs[0] >> 32, "i_w": inputs[1] >> 32, "i_pb": inputs[2] >> 32,
        })),
        "PsyPsatFnTemp" => Some(json!({"i_tdb": signed(0, 28)})),
        "PsyTsatFnPb" => Some(json!({"i_pb": signed(0, 28)})),
        "PsyTsatFnHPb" => Some(json!({"i_h": signed(0, 24), "i_pb": signed(1, 24)})),
        _ => None,
    }
}

pub(crate) fn write_trace(
    config: &RunConfig,
    trace: &Psy02ProductionTrace,
) -> Result<(), RunError> {
    use crate::psy02_state_json::{final_caches, scalar, slot, snapshot};
    let dictionary = trace.dictionary.iter().map(|call| {
        let operation = &call.operation;
        let tags = requested_tags(call.routine, &call.input_bits);
        let mut before = operation.cache_before.map(slot);
        let mut after = operation.cache_after.map(slot);
        let hit = before.as_ref().zip(tags.as_ref()).map(|(cache, requested)| {
            requested.as_object().is_some_and(|fields|
                fields.iter().all(|(key, value)| cache.get(key) == Some(value)))
        });
        if let Some(tags) = tags {
            if let Some(cache) = before.as_mut() { cache["requested_tags"] = tags.clone(); }
            if let Some(cache) = after.as_mut() { cache["requested_tags"] = tags; }
        }
        json!({
            "sequence": call.sequence, "routine": call.routine, "phase": call.phase,
            "caller": {"file": call.caller.file, "line": call.caller.line, "column": call.caller.column},
            "context": call.context.map(crate::psychrometrics_trace::execution_context),
            "input_bits": call.input_bits.iter().map(|bits| format!("{bits:016x}")).collect::<Vec<_>>(),
            "inputs": call.input_bits.iter().map(|bits| scalar(f64::from_bits(*bits))).collect::<Vec<_>>(),
            "result": scalar(operation.result), "result_bits": format!("{:016x}", operation.result.to_bits()),
            "state_before": snapshot(operation.state_before), "state_after": snapshot(operation.state_after),
            "cache_before": before, "cache_after": after, "cache_hit": hit,
        })
    }).collect();
    let artifact = Artifact {
        schema: "psy02-calls.v1",
        capture_source: "actual-rust-selected-property-entrypoints",
        thread_coverage: "collecting-thread-only",
        root_policy: "outermost-selected-property-operation; nested original helpers execute naturally once in replay",
        phase_contract: "actual-Rust-pipeline-phase/context; not-an-EnergyPlus-manager-order-claim",
        input_origin: "actual-Rust-arguments; no-reference-result-or-fixture-seeding",
        input_order: json!({
            "PsyTwbFnTdbWPb": ["t_db_c", "w_kg_per_kg", "p_pa"],
            "PsyTwbFnTdbWPb_raw": ["t_db_c", "w_kg_per_kg", "p_pa"],
            "PsyPsatFnTemp": ["t_db_c"], "PsyPsatFnTemp_raw": ["t_db_c"],
            "PsyTsatFnPb": ["p_pa"], "PsyTsatFnPb_raw": ["p_pa"],
            "PsyTsatFnHPb": ["h_j_per_kg", "p_pa"],
            "PsyTsatFnHPb_raw": ["h_j_per_kg", "p_pa"],
            "PsyWFnTdbRhPb": ["t_db_c", "rh_fraction", "p_pa"],
            "PsyWFnTdbH": ["t_db_c", "h_j_per_kg"],
            "PsyWFnTdbTwbPb": ["t_db_c", "t_wb_c", "p_pa"],
        }),
        logical_cache_capacity_each: ep_runtime::psychrometrics::ENERGYPLUS_PSY_CACHE_SIZE,
        cache_defaults: json!({
            "Twb": {"i_tdb": 0, "i_w": 0, "i_pb": 0, "value_bits": "0000000000000000"},
            "Psat": {"i_tdb": -1000, "value_bits": "0000000000000000"},
            "TsatPb": {"i_h": 0, "i_pb": 0, "value_bits": "0000000000000000"},
            "TsatHPb": {"i_h": 0, "i_pb": 0, "value_bits": "0000000000000000"},
        }),
        event_limit: trace.event_limit,
        unique_tuple_limit: trace.unique_tuple_limit,
        total_root_count: trace.total_root_count,
        recorded_root_count: trace.ordered_ids.len(),
        omitted_root_count: trace.omitted_root_count,
        truncation_reason: trace.truncation_reason,
        complete_on_collecting_thread: trace.omitted_root_count == 0,
        routine_counts: &trace.routine_counts,
        initial_state: snapshot(trace.initial_state),
        final_state: snapshot(trace.final_state),
        final_caches: final_caches(&trace.final_caches),
        dictionary,
        ordered_ids: &trace.ordered_ids,
        unpaired_source_observations: [
            "ErrorManager-warning-recurrence-and-message-output",
            "stats-OFF-original-local-iteration-count",
            "original-internal-call-tree-and-manager-order",
        ],
    };
    let path = config.output_dir.join("psy02-calls.json");
    let write = || -> Result<(), String> {
        let file = std::fs::File::create(&path)
            .map_err(|error| format!("failed to create {}: {error}", path.display()))?;
        let mut writer = BufWriter::new(file);
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
