//! Input-only GEO-02 dispatch through the actual initialized geometry owner.
//!
//! Inputs contain coordinates and operand bits only. Native results are never read.
//! Unsafe native warning/centroid-retention helpers remain explicitly unpaired.

use ep_model::{
    AutoOrNumber, ConstructionId, InsideSurfaceConvectionAlgorithm, NormalizedName,
    OutsideBoundaryCondition, OutsideSurfaceConvectionAlgorithm, Point3, SpaceId, SunExposure,
    Surface, SurfaceId, SurfaceType, TypedModel, WindExposure, Zone, ZoneConvectionAlgorithm,
    ZoneId,
};
use ep_runtime::geometry::{
    SOURCE_CENTROID_PRODUCT_PRECISION_BITS, SOURCE_CENTROID_THIRD_BITS, SurfaceGeometryProperties,
    source_triangle_centroid, surface_geometry_properties,
};
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
        let geometry = if kind == "valid_quad" {
            Some(
                surface_geometry_properties(&typed.surfaces[0].vertices)
                    .map_err(|error| format!("{case_id}: {error}"))?,
            )
        } else {
            None
        };
        if case["input_vertex_bits"] != json!(input_vertex_bits) {
            return Err(format!(
                "{case_id}: actual numeric input differs from declared input bits"
            ));
        }
        results.push(json!({
            "case_id":case_id,"kind":kind,"input":case,
            "status":if geometry.is_some() {"source_complete"} else {"unsupported_source_only"},
            "route":"ep_runtime::geometry::surface_geometry_properties",
            "surface_id":typed.surfaces[0].id.0,"surface_name":typed.surfaces[0].name.0,
            "zone_name":typed.zones[0].name.0,
            "observed_input_vertex_bits":input_vertex_bits,
            "geometry":geometry.map(geometry_json),
            "initial_centroid_consumed":false,
            "admission_checked":false,
            "source_only_state_unpaired":["global counters","scratch arrays","shape","warning/centroid retention"]
        }));
    }
    let cen_calls = input["cen_calls"]
        .as_array()
        .ok_or("expected cen_calls array")?;
    let mut cen_results = Vec::with_capacity(cen_calls.len());
    for call in cen_calls {
        let case_id = call["case_id"].as_str().ok_or("expected cen case_id")?;
        let tokens = call["x_operand_bits"]
            .as_array()
            .ok_or("expected three input bit operands")?;
        if tokens.len() != 3 {
            return Err("cen requires exactly three input operands".into());
        }
        let mut operands = [0.0; 3];
        for (operand, token) in operands.iter_mut().zip(tokens) {
            let token = token.as_str().ok_or("expected input hexadecimal bits")?;
            if token.len() != 16 {
                return Err("input bits require 16 hexadecimal digits".into());
            }
            *operand = f64::from_bits(u64::from_str_radix(token, 16).map_err(|e| e.to_string())?);
        }
        let point = |x_m| Point3 {
            x_m,
            y_m: 0.0,
            z_m: 0.0,
        };
        let result =
            source_triangle_centroid(point(operands[0]), point(operands[1]), point(operands[2]));
        cen_results.push(json!({"case_id":case_id,"input":call,
            "route":"ep_runtime::geometry::source_triangle_centroid", "value":scalar(result.x_m)}));
    }
    Ok(json!({"schema":"geo02-helper-results.v1",
    "implementation_stage":"source_geometry_owner", "cases":results,
    "cen_calls":cen_results,"physics_executed":false,
    "reference_outputs_supplied_to_Rust":false,
    "cen_precision_profile":{
        "sum_precision_bits":53,"product_precision_bits":SOURCE_CENTROID_PRODUCT_PRECISION_BITS,
        "third_bits":format!("{SOURCE_CENTROID_THIRD_BITS:016x}"),
        "rounding":"nearest_ties_even","control_writes_added":false,
        "selection_basis":"actual_original_x87_precision_observation"
    }}))
}

fn geometry_json(value: SurfaceGeometryProperties) -> Value {
    let vector = |components: [f64; 3]| components.map(scalar);
    json!({
        "area_m2":scalar(value.area_m2),"gross_area_m2":scalar(value.gross_area_m2),
        "net_area_shadow_m2":scalar(value.net_area_shadow_m2),
        "azimuth_deg":scalar(value.azimuth_deg),"tilt_deg":scalar(value.tilt_deg),
        "newell_area_vector_m2":vector(value.newell_area_vector_m2),
        "newell_normal":vector(value.newell_normal),"out_norm":vector(value.out_norm),
        "centroid_m":vector([value.centroid_m.x_m,value.centroid_m.y_m,value.centroid_m.z_m]),
        "lcsx":vector(value.lcsx),"lcsy":vector(value.lcsy),"lcsz":vector(value.lcsz),
        "sin_azimuth":scalar(value.sin_azimuth),"cos_azimuth":scalar(value.cos_azimuth),
        "sin_tilt":scalar(value.sin_tilt),"cos_tilt":scalar(value.cos_tilt)
    })
}
