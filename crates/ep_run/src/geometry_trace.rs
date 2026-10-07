//! Output-only projections of compiled coordinates and actual runtime consumers.

use crate::{RunConfig, RunError, RunExitCode, TraceLevel};
use ep_model::{Point3, TypedModel};
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
use ep_runtime::heat_balance::HeatBalanceState;
use serde_json::{Value, json};
use std::io::{BufWriter, Write};

const SELECTED_OBJECTS: &[&str] = &[
    "Building",
    "Zone",
    "GlobalGeometryRules",
    "BuildingSurface:Detailed",
    "GeometryTransform",
    "Compliance:Building",
];

fn point(point: &Point3) -> Value {
    json!([point.x_m, point.y_m, point.z_m])
}

fn point_bits(point: &Point3) -> Value {
    json!([bits(point.x_m), bits(point.y_m), bits(point.z_m)])
}

fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
}

// RawModel stores numerical lexemes until the compiler parses them as f64.
// These observations use that same parse, not independently reconstructed IDF values.
fn raw_value(value: &RawValue, path: &str, numeric_bits: &mut Vec<Value>) -> Result<Value, String> {
    Ok(match value {
        RawValue::Null => Value::Null,
        RawValue::Bool(value) => json!(value),
        RawValue::String(value) => json!(value),
        RawValue::Number(text) => {
            let value = text
                .parse::<f64>()
                .map_err(|error| format!("failed to observe raw number {path}: {error}"))?;
            numeric_bits.push(json!({"path":path, "bits":bits(value)}));
            json!(value)
        }
        RawValue::Array(values) => Value::Array(
            values
                .iter()
                .enumerate()
                .map(|(index, value)| raw_value(value, &format!("{path}/{index}"), numeric_bits))
                .collect::<Result<_, _>>()?,
        ),
        RawValue::Object(values) => Value::Object(
            values
                .iter()
                .map(|(name, value)| {
                    Ok((
                        name.0.clone(),
                        raw_value(value, &format!("{path}/{}", name.0), numeric_bits)?,
                    ))
                })
                .collect::<Result<_, String>>()?,
        ),
    })
}

fn parsed_input(raw: &RawModel) -> Result<Value, String> {
    let mut objects = serde_json::Map::new();
    let mut numeric_bits = Vec::new();
    for kind in SELECTED_OBJECTS {
        if let Some(rows) = raw.objects.get(&ObjectType((*kind).to_string())) {
            let rows = rows
                .iter()
                .map(|(name, object)| {
                    let fields = RawValue::Object(object.fields.clone());
                    Ok((
                        name.0.clone(),
                        raw_value(&fields, &format!("/{kind}/{}", name.0), &mut numeric_bits)?,
                    ))
                })
                .collect::<Result<serde_json::Map<_, _>, String>>()?;
            objects.insert((*kind).to_string(), Value::Object(rows));
        }
    }
    Ok(json!({
        "objects":objects, "numeric_bits":numeric_bits,
        "method":"copy selected actual RawModel objects after ordinary lexical conversion; f64 bits use the compiler numeric parse",
        "numeric_path_semantics":"slash-delimited object/field names and array indices; not escaped JSON pointers",
    }))
}

fn raw_number(raw: &RawModel, object_type: &str, field: &str) -> Option<f64> {
    raw.objects
        .get(&ObjectType(object_type.to_string()))?
        .values()
        .next()?
        .fields
        .get(&FieldName(field.to_string()))
        .and_then(|value| match value {
            RawValue::Number(text) => text.parse().ok(),
            _ => None,
        })
}

fn compiled_projection(
    raw: &RawModel,
    model: &TypedModel,
    input_hashes: Value,
    idf: bool,
) -> Result<Value, String> {
    let rules = model.global_geometry_rules.unwrap_or_default();
    Ok(json!({
        "schema":"compiled-geometry.v1", "phase":"typed_compile",
        "physics_executed":false, "preparation_only":true,
        "input_origin":"normal staged input / normal RawModel / existing compiler",
        "observer_supplies_inputs":false,
        "inputs":{
            "hash_algorithm":input_hashes["algorithm"],
            "original_input":input_hashes["source"],
            "staged_original_input":input_hashes["staged_original"],
            "converted_epjson":input_hashes["converted_epjson"],
            "input_hashes_receipt":"input/input-hashes.json",
            "source_assisted_lexical_conversion":idf,
            "conversion_is_physical_observation":false,
        },
        "parsed_input":parsed_input(raw)?,
        "settings":{
            "building_raw_north_axis_deg":raw_number(raw,"Building","north_axis"),
            "building_effective_north_axis_deg":model.building.as_ref().map(|building| building.north_axis_deg),
            "building_effective_north_axis_bits":model.building.as_ref().map(|building| bits(building.north_axis_deg)),
            "appendix_g_rotation_deg":0.0,
            "appendix_g_rotation_origin":"fixed GEO-01 admitted domain; no corresponding Rust rotation input/state",
            "global_geometry_rules":{
                "starting_vertex_position":format!("{:?}",rules.starting_vertex_position),
                "vertex_entry_direction":format!("{:?}",rules.vertex_entry_direction),
                "coordinate_system":format!("{:?}",rules.coordinate_system),
            },
            "geometry_rules_origin":if model.global_geometry_rules.is_some(){"actual typed GlobalGeometryRules"}else{"existing Rust GlobalGeometryRules::default used by compiler"},
        },
        "order_semantics":"Rust compiler/model collection iteration; not original IDF declaration order or native EnergyPlus IDs/order",
        "zones":model.zones.iter().enumerate().map(|(index,zone)|json!({
            "id":zone.id.0, "name":zone.name.0,
            "compiler_iteration_order":index+1,
            "relative_north_deg":zone.direction_of_relative_north_deg,
            "relative_north_bits":bits(zone.direction_of_relative_north_deg),
            "origin_m":point(&zone.origin), "origin_bits":point_bits(&zone.origin),
        })).collect::<Vec<_>>(),
        "surfaces":model.surfaces.iter().enumerate().map(|(index,surface)|json!({
            "id":surface.id.0, "name":surface.name.0, "zone_id":surface.zone.0,
            "zone_name":model.zones.iter().find(|zone|zone.id==surface.zone).map(|zone|zone.name.0.as_str()),
            "compiler_iteration_order":index+1, "class":format!("{:?}",surface.surface_type),
            "sides":surface.vertices.len(),
            "world_vertices_m":surface.vertices.iter().map(point).collect::<Vec<_>>(),
            "world_vertex_bits":surface.vertices.iter().map(point_bits).collect::<Vec<_>>(),
        })).collect::<Vec<_>>(),
        "native_only_unpaired_state":["MaxVerticesPerSurface", "original trig caches / scratch-array lifecycle"],
        "unclaimed_algorithms":["GEO-02", "GEO-03", "whole-building physics"],
    }))
}

/// Copies existing final fields; it never recalculates a geometry consumer.
pub(crate) fn runtime_projection(model: &TypedModel, state: &HeatBalanceState) -> Value {
    json!({
        "schema":"geo01-runtime-consumers.v1", "phase":"runtime_final_state_geometry",
        "geo02_geometry_snapshot":crate::geo02_trace::runtime_projection(state),
        "physics_executed":true, "observer_supplies_inputs":false,
        "compiled_geometry_artifact":"compiled-geometry.json",
        "observed_timestep_index":state.timestep_index,
        "compiled_surface_count":model.surfaces.len(), "compiled_zone_count":model.zones.len(),
        "field_origin":"actual successful simulation final state; these static geometry fields are assigned by existing heat-balance initialization",
        "surfaces":state.surfaces.iter().enumerate().map(|(index,surface)|json!({
            "id":surface.surface_id.0, "name":surface.surface_name,
            "zone_id":surface.zone_id.0, "runtime_iteration_order":index+1,
            "rust_consumer":{
                "area_m2":surface.area_m2, "area_bits":bits(surface.area_m2),
                "azimuth_deg":surface.azimuth_deg, "azimuth_bits":bits(surface.azimuth_deg),
                "tilt_deg":surface.tilt_deg, "tilt_bits":bits(surface.tilt_deg),
            },
        })).collect::<Vec<_>>(),
        "zones":state.zones.iter().enumerate().map(|(index,zone)|json!({
            "id":zone.zone_id.0, "name":zone.zone_name, "runtime_iteration_order":index+1,
            "rust_consumer":{"volume_m3":zone.volume_m3,"volume_bits":bits(zone.volume_m3)},
        })).collect::<Vec<_>>(),
        "claim_boundary":"stored-field connection projection only; no GEO-02/GEO-03 algorithm or whole-physics certification",
    })
}

pub(crate) fn write_compiled_geometry(
    config: &RunConfig,
    raw: &RawModel,
    model: Option<&TypedModel>,
) -> Result<(), RunError> {
    if config.trace_level != TraceLevel::Full {
        return Ok(());
    }
    let Some(model) = model else {
        return Ok(());
    };
    let result = || -> Result<(), String> {
        let receipt = config.output_dir.join("input/input-hashes.json");
        let input_hashes: Value = serde_json::from_slice(
            &std::fs::read(&receipt)
                .map_err(|error| format!("failed to read {}: {error}", receipt.display()))?,
        )
        .map_err(|error| format!("failed to parse {}: {error}", receipt.display()))?;
        let idf = config
            .input_path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| extension.eq_ignore_ascii_case("idf"));
        let artifact = compiled_projection(raw, model, input_hashes, idf)?;
        write(config, "compiled-geometry.json", &artifact)
    };
    result().map_err(|message| RunError {
        exit_code: RunExitCode::OutputExport,
        message,
    })
}

pub(crate) fn write_runtime_geometry(config: &RunConfig, artifact: &Value) -> Result<(), RunError> {
    write(config, "geometry-consumers.json", artifact).map_err(|message| RunError {
        exit_code: RunExitCode::OutputExport,
        message,
    })?;
    if let Some(geometry) = artifact.get("geo02_geometry_snapshot") {
        crate::geo02_trace::write_geometry(config, geometry)?;
    }
    Ok(())
}

fn write(config: &RunConfig, filename: &str, artifact: &Value) -> Result<(), String> {
    let path = config.output_dir.join(filename);
    let file = std::fs::File::create(&path)
        .map_err(|error| format!("failed to create {}: {error}", path.display()))?;
    let mut writer = BufWriter::new(file);
    serde_json::to_writer(&mut writer, artifact)
        .map_err(|error| format!("failed to serialize {}: {error}", path.display()))?;
    writer
        .write_all(b"\n")
        .and_then(|()| writer.flush())
        .map_err(|error| format!("failed to write {}: {error}", path.display()))
}

#[cfg(test)]
mod tests;
