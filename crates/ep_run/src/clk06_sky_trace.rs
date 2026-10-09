//! Output-only streaming serialization of actual selected sky transport receipts.

use crate::clk02_trace::scalar;
use crate::{RunConfig, RunError, RunExitCode};
use ep_runtime::weather::day::WeatherDayPhase;
use ep_runtime::weather::day::sky_transport_trace::{
    EVENT_KIND_COUNT, EVENT_LIMIT_PER_KIND, SkyTransportEvent, SkyTransportKind,
    SkyTransportSeries, SkyTransportStamp, SkyTransportTrace, SkyTransportValues,
};
use serde_json::{Value, json};
use std::io::{BufWriter, Write};

const KINDS: [SkyTransportKind; EVENT_KIND_COUNT] = [
    SkyTransportKind::OwnedContextReceipt,
    SkyTransportKind::SamplerReturn,
    SkyTransportKind::ReportAccumulator,
    SkyTransportKind::HourlyOutput,
    SkyTransportKind::SeriesHandoff,
    SkyTransportKind::PhysicalIngress,
    SkyTransportKind::PhysicalResolved,
    SkyTransportKind::ReportResolved,
];

fn stamp(value: SkyTransportStamp) -> Value {
    let phase = match value.phase {
        WeatherDayPhase::Warmup { day } => json!({"kind":"warmup","day":day}),
        WeatherDayPhase::Run { day } => json!({"kind":"run","day":day}),
    };
    json!({"phase":phase,"record_index":value.record_index,"hour":value.hour,
        "timestep":value.timestep,"completed_operation_count":value.completed_operation_count})
}

fn pair(values: [f64; 2]) -> Value {
    json!({"SkyTemp":scalar(values[0]),"HorizIRSky":scalar(values[1])})
}

fn payload(value: SkyTransportValues) -> Value {
    match value {
        SkyTransportValues::OwnedContext { today, sample_ir } => json!({
            "received_Today_slot":{"SkyTemp":scalar(today[0]),"HorizIRSky":scalar(today[1]),
                "TotalSkyCover":scalar(today[2]),"OpaqueSkyCover":scalar(today[3])},
            "received_sample_horizontal_infrared_radiation_w_per_m2":scalar(sample_ir)}),
        SkyTransportValues::Sampler { values, owned } => json!({
            "actual_returned_locals":pair(values),"actual_owned_context_available":owned,
            "fallback_values_numerically_paired":false}),
        SkyTransportValues::Accumulator {
            hour_index,
            substep,
            received,
            before,
            after,
        } => json!({
            "hour_index":hour_index,"substep":substep,"received_locals":pair(received),
            "actual_sum_before":pair(before),"actual_sum_after":pair(after),
            "expected_sum_recomputed":false}),
        SkyTransportValues::Hourly {
            hour_index,
            divisor,
            pushed,
        } => json!({
            "hour_index":hour_index,"actual_divisor":scalar(divisor),"actual_pushed":pair(pushed),
            "hourly_mean_claimed_equal_to_individual_Today_slot":false}),
        SkyTransportValues::Series {
            hour_index,
            handle,
            series,
            value,
        } => {
            let (name, units) = match series {
                SkyTransportSeries::SkyTemperature => ("Site Sky Temperature", "C"),
                SkyTransportSeries::HorizontalInfrared => {
                    ("Site Horizontal Infrared Radiation Rate per Area", "W/m2")
                }
            };
            json!({"hour_index":hour_index,"actual_output_handle":handle,"key":"Environment",
                "variable_name":name,"units":units,"actual_series_value":scalar(value)})
        }
        SkyTransportValues::Ingress { origin, sky, ir } => json!({
            "actual_caller_origin":origin.id(),"caller_origin_inferred_from_TLS":false,
            "actual_sky_argument_available":sky.is_some(),"actual_sky_argument":sky.map(scalar),
            "actual_horizontal_infrared_argument_w_per_m2":scalar(ir),
            "recorded_before_physical_early_return":true}),
        SkyTransportValues::Resolved {
            origin,
            sky,
            ir,
            owned,
        } => json!({
            "actual_caller_origin":origin.id(),"caller_origin_inferred_from_TLS":false,
            "actual_resolved_sky_argument":scalar(sky),
            "actual_horizontal_infrared_local_w_per_m2":scalar(ir),
            "actual_owned_sky_path":owned,"fallback_values_numerically_paired":false,
            "SRC08_coefficients_or_flux_certified":false}),
    }
}

fn event(row: &SkyTransportEvent) -> Value {
    json!({"sequence":row.sequence,"kind_sequence":row.kind_sequence,"kind":row.kind.id(),
        "actual_source_stamp":row.stamp.map(stamp),"source_stamp_available":row.stamp.is_some(),
        "surface_id":row.surface_id.map(|id|id.0),
        "caller":{"file":row.caller.file(),"line":row.caller.line(),"column":row.caller.column()},
        "observed":payload(row.values),"missing_source_identity_counted_as_PASS":false})
}

pub(crate) fn write_trace(config: &RunConfig, trace: &SkyTransportTrace) -> Result<(), RunError> {
    let counts = KINDS.iter().map(|kind| {
        let index = kind.index();
        let total = trace.total_by_kind[index];
        let retained = trace.retained_by_kind[index].len() as u64;
        json!({"kind":kind.id(),"total":total,"retained":retained,"omitted":total-retained,
            "truncated":total!=retained,
            "truncation_reason":if total==retained { None } else { Some("per_kind_retained_prefix_limit") }})
    }).collect::<Vec<_>>();
    let retained: usize = trace.retained_by_kind.iter().map(Vec::len).sum();
    let complete = trace.total_event_count == retained as u64;
    let metadata = json!({"schema":"clk06-sky-transport-trace.v1",
        "trace_level":config.trace_level.id(),"collecting_thread_only":true,
        "capture_enabled_in_Full_and_Summary":true,"retained_limit_per_kind":EVENT_LIMIT_PER_KIND,
        "total_completed_weather_operations":trace.completed_operation_count,
        "total_event_count":trace.total_event_count,"retained_event_count":retained,
        "omitted_event_count":trace.total_event_count-retained as u64,"event_kind_counts":counts,
        "complete_on_collecting_thread":complete,"truncated":!complete,
        "required_truncated_transport_counted_as_PASS":false,"repeated_identities_deduplicated":false,
        "surface_names":trace.surface_names.iter().map(|(id,name)|json!({"id":id.0,"name":name})).collect::<Vec<_>>(),
        "shared_kernel_origin_policy":"explicit surface_solve/report_outside_face_recomputation/explicit_direct_call; report_terms stays a separate event kind",
        "kernel_event_count_is_timestep_solve_count":false,
        "source_identity_policy":"actual completion stamp transported with context and per-hour receipt; no latest-TLS reconstruction",
        "bounded_scope_policy":"1M per kind is predeclared; six-run config/surface/warmup bound requires external frozen preflight",
        "general_unbounded_runtime_settings_coverage_certified":false,
        "source_fields_recomputed_or_supplied_as_inputs":false,"numerical_PASS_claimed":false,
        "unpaired_scope":["native Current sky3/cloud2","native prepared sky model owners","fallback sky calculations","SRC08 coefficients and fluxes"],
        "input_build_source_and_six_run_configuration_authentication":"external recorded workflow required"});
    let path = config.output_dir.join("clk06_sky_transport_trace.json");
    let write = || -> Result<(), String> {
        let file = std::fs::File::create(&path).map_err(|error| error.to_string())?;
        let mut writer = BufWriter::new(file);
        writer
            .write_all(b"{\"metadata\":")
            .map_err(|error| error.to_string())?;
        serde_json::to_writer(&mut writer, &metadata).map_err(|error| error.to_string())?;
        writer
            .write_all(b",\"events\":[")
            .map_err(|error| error.to_string())?;
        // Merge only eight retained prefixes. No second full-size event array/Value.
        let mut next = [0; EVENT_KIND_COUNT];
        let mut first = true;
        loop {
            let chosen = (0..EVENT_KIND_COUNT)
                .filter_map(|index| {
                    trace.retained_by_kind[index]
                        .get(next[index])
                        .map(|row| (index, row.sequence))
                })
                .min_by_key(|(_, sequence)| *sequence);
            let Some((index, _)) = chosen else {
                break;
            };
            if !first {
                writer.write_all(b",").map_err(|error| error.to_string())?;
            }
            first = false;
            serde_json::to_writer(
                &mut writer,
                &event(&trace.retained_by_kind[index][next[index]]),
            )
            .map_err(|error| error.to_string())?;
            next[index] += 1;
        }
        writer
            .write_all(b"]}\n")
            .and_then(|()| writer.flush())
            .map_err(|error| error.to_string())
    };
    write().map_err(|message| RunError {
        exit_code: RunExitCode::OutputExport,
        message,
    })
}
