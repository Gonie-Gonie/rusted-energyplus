//! Input-only GEO-02 baseline dispatch through existing public geometry summaries.
//!
//! This adapter exposes the current area/angle implementation before the canonical
//! geometry owner is ported. Missing normal and centroid outputs stay explicit.

use ep_model::{
    AutoOrNumber, ConstructionId, InsideSurfaceConvectionAlgorithm, NormalizedName,
    OutsideBoundaryCondition, OutsideSurfaceConvectionAlgorithm, Point3, SpaceId, SunExposure,
    Surface, SurfaceId, SurfaceType, TypedModel, WindExposure, Zone, ZoneConvectionAlgorithm,
    ZoneId,
};
use ep_runtime::geometry::surface_geometry_summaries;
use serde_json::{Value, json};
use std::{collections::BTreeSet, error::Error, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let input = if let Some(path) = args.next() {
        std::fs::read_to_string(path)?
    } else {
        let mut input = String::new();
        std::io::stdin().read_to_string(&mut input)?;
        input
    };
    if args.next().is_some() {
        return Err("usage: geo02_tuples [INPUT.json] (results on stdout)".into());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&execute(&serde_json::from_str(&input)?)?)?
    );
    Ok(())
}

fn points(value: &Value) -> Result<Vec<Point3>, String> {
    value
        .as_array()
        .ok_or("expected an array of three-component points")?
        .iter()
        .map(|value| {
            let coordinates = value.as_array().ok_or("expected a three-component point")?;
            if coordinates.len() != 3 {
                return Err("expected exactly three point components".into());
            }
            let mut values = [0.0; 3];
            for (target, component) in values.iter_mut().zip(coordinates) {
                *target = component
                    .as_f64()
                    .ok_or("expected a finite numeric coordinate")?;
                if !target.is_finite() {
                    return Err("expected a finite numeric coordinate".into());
                }
            }
            Ok(Point3 {
                x_m: values[0],
                y_m: values[1],
                z_m: values[2],
            })
        })
        .collect()
}

fn surface_type(value: &str) -> Result<SurfaceType, String> {
    match value {
        "Wall" => Ok(SurfaceType::Wall),
        "Roof" => Ok(SurfaceType::Roof),
        "Ceiling" => Ok(SurfaceType::Ceiling),
        "Floor" => Ok(SurfaceType::Floor),
        _ => Err(format!("unsupported surface_class: {value}")),
    }
}

fn scalar(value: f64) -> Value {
    let value_class = if value.is_nan() {
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
    json!({"value":encoded, "value_bits":format!("{:016x}", value.to_bits()),
        "value_class":value_class})
}

fn model(case_id: &str, surface_type: SurfaceType, vertices: Vec<Point3>) -> TypedModel {
    let mut model = TypedModel::default();
    model.zones.push(Zone {
        id: ZoneId(0),
        name: NormalizedName::new("GEO02 UNIT ZONE"),
        direction_of_relative_north_deg: 0.0,
        origin: Point3 {
            x_m: 0.0,
            y_m: 0.0,
            z_m: 0.0,
        },
        zone_type: 1,
        multiplier: 1,
        list_multiplier: 1,
        list_group: None,
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::AutoCalculate,
        floor_area: AutoOrNumber::AutoCalculate,
        inside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            InsideSurfaceConvectionAlgorithm::Tarp,
        ),
        outside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            OutsideSurfaceConvectionAlgorithm::Doe2,
        ),
        is_part_of_total_floor_area: true,
        is_nominal_controlled: false,
        linked_outdoor_air_node: None,
        spaces: Vec::new(),
    });
    model.surfaces.push(Surface {
        id: SurfaceId(0),
        name: NormalizedName::new(case_id),
        surface_type,
        construction: ConstructionId(0),
        zone: ZoneId(0),
        space: SpaceId(0),
        outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
        outside_boundary_condition_object: None,
        sun_exposure: SunExposure::SunExposed,
        wind_exposure: WindExposure::WindExposed,
        view_factor_to_ground: AutoOrNumber::AutoCalculate,
        vertices,
        computed_geometry: None,
    });
    model
}

fn execute(input: &Value) -> Result<Value, String> {
    if input["schema"] != "geo02-helper-cases.v1" {
        return Err("expected schema geo02-helper-cases.v1".into());
    }
    let cases = input["cases"].as_array().ok_or("expected cases array")?;
    let mut ids = BTreeSet::new();
    let mut results = Vec::with_capacity(cases.len());
    for case in cases {
        let case_id = case["case_id"].as_str().ok_or("expected case_id")?;
        if !ids.insert(case_id) {
            return Err(format!("duplicate case_id: {case_id}"));
        }
        let kind = case["kind"].as_str().ok_or("expected case kind")?;
        if !matches!(kind, "valid_quad" | "source_only_degenerate") {
            return Err(format!("unsupported case kind: {kind}"));
        }
        let vertices = points(&case["vertices_m"])?;
        let class = case["surface_class"]
            .as_str()
            .ok_or("expected surface_class")?;
        let typed = model(case_id, surface_type(class)?, vertices);
        let input_vertex_bits: Vec<[String; 3]> = typed.surfaces[0]
            .vertices
            .iter()
            .map(|point| {
                [point.x_m, point.y_m, point.z_m]
                    .map(|coordinate| format!("{:016x}", coordinate.to_bits()))
            })
            .collect();
        let summaries = surface_geometry_summaries(&typed);
        let summary = summaries.first().ok_or("geometry summary missing")?;
        results.push(json!({
            "case_id":case_id,"kind":kind,"input":case,
            "status":if kind == "valid_quad" {"baseline_partial"} else {"unsupported_source_only"},
            "route":"ep_runtime::geometry::surface_geometry_summaries",
            "surface_id":summary.surface_id.0,"surface_name":summary.surface_name,
            "zone_name":summary.zone_name,
            "observed_input_vertex_bits":input_vertex_bits,
            "geometry":{
                "area_m2":scalar(summary.area_m2),
                "azimuth_deg":scalar(summary.azimuth_deg),
                "tilt_deg":scalar(summary.tilt_deg)
            },
            "unimplemented_fields":["gross_area_m2","net_area_shadow_m2",
                "newell_area_vector_m2","newell_unit_normal","outward_unit_normal",
                "centroid_m","lcs","trig"],
            "initial_centroid_consumed":false,
            "admission_checked":false
        }));
    }
    let cen_calls = input["cen_calls"]
        .as_array()
        .ok_or("expected cen_calls array")?;
    let mut cen_results = Vec::with_capacity(cen_calls.len());
    for call in cen_calls {
        call["case_id"].as_str().ok_or("expected cen case_id")?;
        cen_results.push(json!({"input":call,"status":"unsupported_baseline",
            "reason":"no existing public source cen helper or centroid geometry owner"}));
    }
    Ok(json!({"schema":"geo02-helper-results.v1",
        "implementation_stage":"existing_helpers_baseline", "cases":results,
        "cen_calls":cen_results,"physics_executed":false,
        "reference_outputs_supplied_to_Rust":false}))
}
