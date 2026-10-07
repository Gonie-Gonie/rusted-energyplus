//! Output-only copies of actual zone-volume initialization results.

use crate::{RunConfig, RunError, RunExitCode};
use ep_runtime::geometry::zone_volume_trace::ZoneVolumeTrace;
use ep_runtime::geometry::{ZoneGeometryProperties, ZoneVolumeFace};
use serde_json::{Value, json};
use std::io::{BufWriter, Write};

fn scalar(value: f64) -> Value {
    let class = if value.is_nan() {
        "nan"
    } else if value == f64::INFINITY {
        "positive_infinity"
    } else if value == f64::NEG_INFINITY {
        "negative_infinity"
    } else if value == 0.0 {
        if value.is_sign_negative() {
            "negative_zero"
        } else {
            "positive_zero"
        }
    } else {
        "finite"
    };
    json!({"value":if value.is_finite(){Some(value)}else{None},
        "value_bits":format!("{:016x}",value.to_bits()),"value_class":class})
}

fn point(value: &ep_model::Point3) -> Value {
    json!([scalar(value.x_m), scalar(value.y_m), scalar(value.z_m)])
}

fn face(face: &ZoneVolumeFace) -> Value {
    json!({
        "id":face.surface_id.0, "name":face.name,
        "class":format!("{:?}",face.surface_type),
        "world_vertices_m":face.vertices.iter().map(point).collect::<Vec<_>>(),
        "area_m2":scalar(face.area_m2), "gross_area_m2":scalar(face.gross_area_m2),
        "newell_area_vector_m2":face.newell_area_vector_m2.map(scalar),
        "tilt_deg":scalar(face.tilt_deg),
    })
}

fn properties(value: &ZoneGeometryProperties) -> Value {
    json!({
        "zone":{
            "volume_m3":scalar(value.volume_m3),
            "ceiling_height_m":scalar(value.ceiling_height_m),
            "ceiling_height_entered":value.ceiling_height_entered,
            "floor_area_m2":scalar(value.floor_area_m2),
            "user_entered_floor_area_m2":scalar(value.user_entered_floor_area_m2),
            "geometric_floor_area_m2":scalar(value.geometric_floor_area_m2),
            "ceiling_area_m2":scalar(value.ceiling_area_m2),
            "geometric_ceiling_area_m2":scalar(value.geometric_ceiling_area_m2),
            "has_floor":value.has_floor, "has_roof":value.has_roof,
        },
        "ordered_volume_faces":value.faces.iter().enumerate().map(|(index,value)|
            json!({"volume_face_order":index+1,"face":face(value)})).collect::<Vec<_>>(),
        "p0_m":point(&value.p0_m),
        "volume_calculation_count":value.volume_calculation_count,
        "diagnostics":{
            "initial_unique_vertex_count":value.diagnostics.initial_unique_vertex_count,
            "initial_edges_not_used_twice":value.diagnostics.initial_edges_not_used_twice.iter().map(|edge|json!({
                "surface_id":edge.surface_id.0,"start_m":point(&edge.start_m),"end_m":point(&edge.end_m),
                "count":edge.count,"other_surface_ids":edge.other_surface_ids.iter().map(|id|id.0).collect::<Vec<_>>(),
            })).collect::<Vec<_>>(),
            "initially_closed":value.diagnostics.initially_closed,
            "floor_horizontal":value.diagnostics.floor_horizontal,
            "roof_horizontal":value.diagnostics.roof_horizontal,
            "walls_vertical":value.diagnostics.walls_vertical,
            "same_wall_height":value.diagnostics.same_wall_height,
            "edges_winding_consistent":value.diagnostics.edges_winding_consistent,
            "signed_polyhedron_volume_m3":value.diagnostics.signed_polyhedron_volume_m3.map(scalar),
            "topology_rejection":value.diagnostics.topology_rejection,
            "volume_differs_by_more_than_five_percent":value.diagnostics.volume_differs_by_more_than_five_percent,
        },
    })
}

pub(crate) fn write_trace(config: &RunConfig, trace: &ZoneVolumeTrace) -> Result<(), RunError> {
    let artifact = json!({
        "schema":"geo03-zone-volume-owner.v1",
        "capture_source":"actual-Rust-heat-balance-initializer-result",
        "thread_coverage":"collecting-thread-only",
        "observer_supplies_inputs":false, "observer_recalculates_geometry":false,
        "total_call_count":trace.total_call_count,
        "retained_call_count":trace.observations.len(),
        "omitted_call_count":trace.total_call_count-trace.observations.len() as u64,
        "observations":trace.observations.iter().enumerate().map(|(index,row)|json!({
            "sequence":index+1,"zone_id":row.zone_id.0,"zone_name":row.zone_name,
            "caller":{"file":row.caller.file(),"line":row.caller.line(),"column":row.caller.column()},
            "phase":row.phase,"context":row.context.map(crate::psychrometrics_trace::execution_context),
            "outcome":if row.properties.is_some(){"resolved"}else{"rejected"},
            "properties":row.properties.as_ref().map(properties),"rejection":row.rejection,
        })).collect::<Vec<_>>(),
        "unpaired_source_state":["implicit Space","ErrCount5 and global counters","warning/error IO","scratch allocations"],
        "claim_boundary":"actual initializer return and stored-volume connection only; no AirPowerCap, ZON-02, SYS or full physics certification",
    });
    let path = config.output_dir.join("geo03-zone-volume.json");
    let write = || -> Result<(), String> {
        let file = std::fs::File::create(&path).map_err(|error| error.to_string())?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer(&mut writer, &artifact).map_err(|error| error.to_string())?;
        writer
            .write_all(b"\n")
            .and_then(|()| writer.flush())
            .map_err(|error| error.to_string())
    };
    write().map_err(|message| RunError {
        exit_code: RunExitCode::OutputExport,
        message,
    })
}
