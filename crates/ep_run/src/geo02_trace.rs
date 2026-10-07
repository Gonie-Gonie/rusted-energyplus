//! Optional copies of the one stored geometry bundle and actual consumer operands.
//! No geometry is evaluated here and no observation becomes a simulation input.

use crate::{RunConfig, RunError, RunExitCode};
use ep_runtime::geometry::production_trace::{
    GeometryOperand, GeometryOperandBits, GeometryProductionTrace,
};
use ep_runtime::geometry::{
    SOURCE_CENTROID_PRODUCT_PRECISION_BITS, SOURCE_CENTROID_THIRD_BITS, SurfaceGeometryProperties,
};
use ep_runtime::heat_balance::HeatBalanceState;
use serde::Serialize;
use serde_json::{Value, json};
use std::{
    collections::BTreeMap,
    io::{BufWriter, Write},
};

pub(crate) fn runtime_projection(state: &HeatBalanceState) -> Value {
    json!({
        "schema":"geo02-runtime-geometry.v1","phase":"runtime_final_state_geometry",
        "physics_executed":true,"observer_supplies_inputs":false,
        "observed_timestep_index":state.timestep_index,
        "geometry_owner":"ep_runtime::geometry::surface_geometry_properties; assigned once at heat-balance initialization",
        "cen_precision_profile":{
            "sum_precision_bits":53,"product_precision_bits":SOURCE_CENTROID_PRODUCT_PRECISION_BITS,
            "third_bits":format!("{SOURCE_CENTROID_THIRD_BITS:016x}"),
            "rounding":"nearest_ties_even","control_writes_added":false,
            "selection_basis":"actual_original_x87_precision_observation"
        },
        "surfaces":state.surfaces.iter().enumerate().map(|(index,surface)|json!({
            "id":surface.surface_id.0,"name":surface.surface_name,"zone_id":surface.zone_id.0,
            "runtime_iteration_order":index+1,"geometry":geometry(surface.geometry),
            "legacy_geometry":{"area_m2":scalar(surface.area_m2),
                "azimuth_deg":scalar(surface.azimuth_deg),"tilt_deg":scalar(surface.tilt_deg)}
        })).collect::<Vec<_>>(),
        "native_only_unpaired_state":["global counters","scratch arrays","shape","warning recurrence","VerticesProcessed lifecycle"],
        "connection_boundary":"actual stored geometry and actual Rust consumer arguments; no SRC04 solar-clock/incidence-output or SRC05 atmospheric-equation equivalence; outward normal is stored but no normal-dot consumer is claimed"
    })
}

fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value == f64::INFINITY {
        "positive_infinity"
    } else if value == f64::NEG_INFINITY {
        "negative_infinity"
    } else if value == 0.0 && value.is_sign_negative() {
        "negative_zero"
    } else if value == 0.0 {
        "positive_zero"
    } else {
        "finite"
    };
    let encoded = if value.is_nan() {
        json!("NaN")
    } else if value == f64::INFINITY {
        json!("+Infinity")
    } else if value == f64::NEG_INFINITY {
        json!("-Infinity")
    } else {
        json!(value)
    };
    json!({"value":encoded,"value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}
fn geometry(value: SurfaceGeometryProperties) -> Value {
    let vector = |values: [f64; 3]| values.map(scalar);
    json!({"area_m2":scalar(value.area_m2),"gross_area_m2":scalar(value.gross_area_m2),
        "net_area_shadow_m2":scalar(value.net_area_shadow_m2),"azimuth_deg":scalar(value.azimuth_deg),
        "tilt_deg":scalar(value.tilt_deg),"newell_area_vector_m2":vector(value.newell_area_vector_m2),
        "newell_normal":vector(value.newell_normal),"out_norm":vector(value.out_norm),
        "centroid_m":vector([value.centroid_m.x_m,value.centroid_m.y_m,value.centroid_m.z_m]),
        "lcsx":vector(value.lcsx),"lcsy":vector(value.lcsy),"lcsz":vector(value.lcsz),
        "sin_azimuth":scalar(value.sin_azimuth),"cos_azimuth":scalar(value.cos_azimuth),
        "sin_tilt":scalar(value.sin_tilt),"cos_tilt":scalar(value.cos_tilt)})
}
fn operand(value: GeometryOperand) -> Value {
    match value {
        GeometryOperand::CentroidHeight {
            centroid_m,
            height_m,
        } => json!({
            "kind":"centroid_height","centroid_m":centroid_m,"height_m":height_m}),
        GeometryOperand::Orientation {
            azimuth_deg,
            tilt_deg,
            azimuth_rad,
            tilt_rad,
        } => json!({
            "kind":"orientation","azimuth_deg":azimuth_deg,"tilt_deg":tilt_deg,
            "azimuth_rad":azimuth_rad,"tilt_rad":tilt_rad}),
        GeometryOperand::ConvectionOrientation {
            azimuth_deg,
            tilt_deg,
            cos_tilt,
        } => json!({
            "kind":"convection_orientation","azimuth_deg":azimuth_deg,"tilt_deg":tilt_deg,"cos_tilt":cos_tilt}),
        GeometryOperand::Area { area_m2 } => json!({"kind":"area","area_m2":area_m2}),
    }
}
fn operand_bits(value: GeometryOperandBits) -> Vec<String> {
    match value {
        GeometryOperandBits::CentroidHeight(values) | GeometryOperandBits::Orientation(values) => {
            values.map(|value| format!("{value:016x}")).to_vec()
        }
        GeometryOperandBits::ConvectionOrientation(values) => {
            values.map(|value| format!("{value:016x}")).to_vec()
        }
        GeometryOperandBits::Area(value) => vec![format!("{value:016x}")],
    }
}
#[derive(Serialize)]
struct OperandArtifact<'a> {
    schema: &'static str,
    capture_source: &'static str,
    thread_coverage: &'static str,
    observer_supplies_inputs: bool,
    phase_contract: &'static str,
    context_contract: &'static str,
    event_limit: usize,
    unique_tuple_limit: usize,
    total_call_count: u64,
    retained_call_count: u64,
    omitted_call_count: u64,
    truncation_reason: Option<&'static str>,
    complete_on_collecting_thread: bool,
    consumer_counts: BTreeMap<&'static str, u64>,
    dictionary: Vec<Value>,
    ordered_ids: &'a [u32],
}
pub(crate) fn write_trace(
    config: &RunConfig,
    trace: &GeometryProductionTrace,
) -> Result<(), RunError> {
    let dictionary=trace.dictionary.iter().map(|call|json!({
        "first_sequence":call.sequence,"consumer":call.consumer.label(),
        "surface_id":call.surface_id.0,"zone_id":call.zone_id.0,"phase":call.phase,
        "caller":{"file":call.caller.file,"line":call.caller.line,"column":call.caller.column},
        "context":call.context.map(crate::psychrometrics_trace::execution_context),
        "operand":operand(call.operand),"operand_bits":operand_bits(call.operand_bits)
    })).collect();
    let artifact = OperandArtifact {
        schema: "geo02-geometry-operands.v1",
        capture_source: "actual-rust-pipeline-geometry-consumer-hooks",
        thread_coverage: "collecting-thread-only",
        observer_supplies_inputs: false,
        phase_contract: "actual-Rust-pipeline-scope; not-an-EnergyPlus-stage-claim",
        context_contract: "existing actual Rust runtime scopes; no inferred native clock or synthetic invocation",
        event_limit: trace.event_limit,
        unique_tuple_limit: trace.unique_tuple_limit,
        total_call_count: trace.total_call_count,
        retained_call_count: trace.retained_call_count,
        omitted_call_count: trace.omitted_call_count,
        truncation_reason: trace.truncation_reason,
        complete_on_collecting_thread: trace.omitted_call_count == 0,
        consumer_counts: trace
            .consumer_counts
            .iter()
            .map(|(consumer, count)| (consumer.label(), *count))
            .collect(),
        dictionary,
        ordered_ids: &trace.ordered_ids,
    };
    write(config, "geo02-geometry-operands.json", &artifact)
}
pub(crate) fn write_geometry(config: &RunConfig, artifact: &Value) -> Result<(), RunError> {
    write(config, "geo02-geometry.json", artifact)
}
fn write(config: &RunConfig, name: &str, artifact: &impl Serialize) -> Result<(), RunError> {
    let path = config.output_dir.join(name);
    let result = || -> Result<(), String> {
        let file = std::fs::File::create(&path)
            .map_err(|e| format!("failed to create {}: {e}", path.display()))?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, artifact)
            .map_err(|e| format!("failed to serialize {}: {e}", path.display()))?;
        writer
            .write_all(b"\n")
            .and_then(|()| writer.flush())
            .map_err(|e| format!("failed to write {}: {e}", path.display()))
    };
    result().map_err(|message| RunError {
        exit_code: RunExitCode::OutputExport,
        message,
    })
}
