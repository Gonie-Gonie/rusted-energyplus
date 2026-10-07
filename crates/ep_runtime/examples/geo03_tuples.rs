//! Input-only GEO-03 baseline through the current public zone-summary owner.
//!
//! Native results are never read. Prepared area/Space fields are input provenance
//! only in this baseline; absent height, topology and mutable-state owners remain
//! explicitly unimplemented until a later canonical implementation is authorized.

use ep_model::{
    AutoOrNumber, ConstructionId, InsideSurfaceConvectionAlgorithm, NormalizedName,
    OutsideBoundaryCondition, OutsideSurfaceConvectionAlgorithm, Point3, SpaceId, SunExposure,
    Surface, SurfaceId, SurfaceType, TypedModel, WindExposure, Zone, ZoneConvectionAlgorithm,
    ZoneId,
};
use ep_runtime::geometry::zone_geometry_summaries;
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
        return Err("usage: geo03_tuples [INPUT.json] (results on stdout)".into());
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&execute(&serde_json::from_str(&input)?)?)?
    );
    Ok(())
}

fn text<'a>(value: &'a Value, field: &str) -> Result<&'a str, String> {
    value[field]
        .as_str()
        .ok_or_else(|| format!("expected string field: {field}"))
}

fn number(value: &Value) -> Result<f64, String> {
    let number = value.as_f64().ok_or("expected finite numeric input")?;
    if !number.is_finite() {
        return Err("expected finite numeric input".into());
    }
    Ok(number)
}

fn auto_or_number(value: &Value) -> Result<AutoOrNumber, String> {
    if value.as_str() == Some("AutoCalculate") {
        Ok(AutoOrNumber::AutoCalculate)
    } else {
        Ok(AutoOrNumber::Value(number(value)?))
    }
}

fn bits(value: f64) -> String {
    format!("{:016x}", value.to_bits())
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
    json!({"value":if value.is_finite() {Some(value)} else {None},
        "value_bits":bits(value), "value_class":value_class})
}

fn validate_scalar_bits(case: &Value) -> Result<(), String> {
    for group in ["zone_input", "prepared_zone", "prepared_implicit_space"] {
        let values = case[group]
            .as_object()
            .ok_or("expected prepared input object")?;
        let declared = case["input_scalar_bits"][group]
            .as_object()
            .ok_or("expected authoritative scalar bits object")?;
        let mut observed = 0;
        for (field, value) in values {
            if value.is_number() {
                observed += 1;
                if declared.get(field).and_then(Value::as_str)
                    != Some(bits(number(value)?).as_str())
                {
                    return Err(format!(
                        "{group}/{field}: numeric input differs from declared bits"
                    ));
                }
            } else if !(value.is_boolean() || value.as_str() == Some("AutoCalculate")) {
                return Err(format!("{group}/{field}: unsupported prepared input type"));
            }
        }
        if observed != declared.len() {
            return Err(format!("{group}: unbound or extra scalar bit fields"));
        }
    }
    Ok(())
}

fn vertices(face: &Value, paired: bool) -> Result<Vec<Point3>, String> {
    let input = face["vertices_m"]
        .as_array()
        .ok_or("expected vertices array")?;
    let declared = face["input_vertex_bits"]
        .as_array()
        .ok_or("expected authoritative vertex bits")?;
    if input.len() != 4 || declared.len() != input.len() {
        return Err("frozen helper requires four vertices with bound input bits".into());
    }
    input
        .iter()
        .zip(declared)
        .map(|(point, declared)| {
            let coordinates = point.as_array().ok_or("expected three coordinates")?;
            let tokens = declared
                .as_array()
                .ok_or("expected three coordinate bit tokens")?;
            if coordinates.len() != 3 || tokens.len() != 3 {
                return Err("expected three coordinates and bit tokens".into());
            }
            let mut values = [0.0; 3];
            for ((target, coordinate), token) in values.iter_mut().zip(coordinates).zip(tokens) {
                *target = number(coordinate)?;
                if token.as_str() != Some(bits(*target).as_str()) {
                    return Err("parsed vertex differs from declared binary64 bits".into());
                }
                if paired && *target == 0.0 && target.is_sign_negative() {
                    return Err("paired raw World geometry requires positive zeros".into());
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
        "Floor" => Ok(SurfaceType::Floor),
        "Roof" => Ok(SurfaceType::Roof),
        _ => Err(format!("unsupported prepared surface class: {value}")),
    }
}

fn model(case: &Value, paired: bool) -> Result<TypedModel, String> {
    let mut model = TypedModel::default();
    let input = &case["zone_input"];
    model.zones.push(Zone {
        id: ZoneId(0),
        name: NormalizedName::new("PREPARED-ONE-ZONE"),
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
        ceiling_height: auto_or_number(&input["ceiling_height_m"])?,
        volume: auto_or_number(&input["volume_m3"])?,
        floor_area: auto_or_number(&input["user_entered_floor_area_m2"])?,
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
    let faces = case["surfaces"]
        .as_array()
        .ok_or("expected surfaces array")?;
    if (paired && faces.len() != 6) || !(5..=7).contains(&faces.len()) {
        return Err("frozen helper face count differs".into());
    }
    let mut names = BTreeSet::new();
    for (index, face) in faces.iter().enumerate() {
        let name = NormalizedName::new(text(face, "name")?);
        if !names.insert(name.0.clone()) {
            return Err("duplicate prepared surface name".into());
        }
        let id = u32::try_from(index).map_err(|_| "too many prepared surfaces")?;
        model.surfaces.push(Surface {
            id: SurfaceId(id),
            name,
            surface_type: surface_type(text(face, "class")?)?,
            construction: ConstructionId(0),
            zone: ZoneId(0),
            space: SpaceId(0),
            outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
            outside_boundary_condition_object: None,
            sun_exposure: SunExposure::SunExposed,
            wind_exposure: WindExposure::WindExposed,
            view_factor_to_ground: AutoOrNumber::AutoCalculate,
            vertices: vertices(face, paired)?,
            computed_geometry: None,
        });
    }
    Ok(model)
}

fn execute(input: &Value) -> Result<Value, String> {
    if input["schema"] != "geo03-helper-cases.v1" {
        return Err("expected schema geo03-helper-cases.v1".into());
    }
    let cases = input["cases"].as_array().ok_or("expected cases array")?;
    let mut ids = BTreeSet::new();
    let mut results = Vec::with_capacity(cases.len());
    for case in cases {
        let case_id = text(case, "case_id")?;
        if !ids.insert(case_id) {
            return Err(format!("duplicate case_id: {case_id}"));
        }
        let kind = text(case, "kind")?;
        if !matches!(
            kind,
            "closed_box" | "source_only_nonclosed" | "source_only_reversed_winding"
        ) {
            return Err(format!("unsupported case kind: {kind}"));
        }
        let paired = kind == "closed_box";
        let route = text(case, "route")?;
        let source_route = if kind == "source_only_reversed_winding" {
            "prepared-reversed-winding-direct-helpers-bypass-GetVertices"
        } else {
            "original-GetVertices-declared-preparation-original-height-fragment-CalculateZoneVolume"
        };
        if route != source_route {
            return Err("frozen helper kind/route differs".into());
        }
        validate_scalar_bits(case)?;
        let typed = model(case, paired)?;
        let typed_surface_order: Vec<Value> = typed.surfaces.iter().enumerate().map(|(index,surface)| json!({
            "request_ordinal":index+1,"id":surface.id.0,"zone_id":surface.zone.0,
            "name":surface.name.0,"class":format!("{:?}",surface.surface_type),
            "input_vertex_bits":surface.vertices.iter().map(|p| [bits(p.x_m),bits(p.y_m),bits(p.z_m)]).collect::<Vec<_>>()
        })).collect();
        let summary = if paired {
            zone_geometry_summaries(&typed).into_iter().next()
        } else {
            None
        };
        let zone = summary.map(|summary| json!({
            "id":summary.zone_id.0,"name":summary.zone_name,"surface_count":summary.surface_count,
            "volume_m3":summary.volume_m3.map(scalar),"floor_area_m2":scalar(summary.floor_area_m2)
        }));
        results.push(json!({
            "case_id":case_id,"kind":kind,"input":case,
            "status":if paired {"baseline_partial"} else {"unsupported_source_only"},
            "route":"ep_runtime::geometry::zone_geometry_summaries",
            "zone":zone,"typed_surface_order":typed_surface_order,
            "numeric_input_identity_checked":true,"canonical_coordinate_conversion_claimed":false,
            "prepared_read_fields_consumed":false,"original_parser_admission_claimed":false,
            "admission_checked":false,"source_state_observed":false,"physics_executed":false,
            "unconsumed_prepared_input_groups":["prepared_zone","prepared_implicit_space"],
            "source_state":null,
            "unavailable_source_state_phases":["geometry_prepared","before_volume","after_volume","after_second_volume"],
            "unimplemented_fields":["ceiling_height_m","ceiling_height_entered","user_entered_floor_area_m2",
                "geometric_floor_area_m2","ceiling_area_m2","geometric_ceiling_area_m2","has_floor","has_roof",
                "closed_topology_admission","mutable_before_after_second_volume_state"],
            "source_only_state_unpaired":["implicit Space","global counters","warning/error IO","source scratch allocations"]
        }));
    }
    Ok(
        json!({"schema":"geo03-helper-results.v1","implementation_stage":"baseline_public_summary",
        "cases":results,"reference_outputs_supplied_to_Rust":false,"expected_answers_supplied":false,
        "original_parser_admission_claimed":false,"physics_executed":false,"gates_updated":false}),
    )
}
