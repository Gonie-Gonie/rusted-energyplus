//! Input-only GEO-03 dispatch through actual mutable zone-geometry ownership.
//!
//! Native results are never read. Zone area reads are declared preparation;
//! Space/global/warning lifecycles remain unpaired. Height executes once and
//! actual volume selection executes twice on the same owner and ordered faces.

use ep_model::{
    AutoOrNumber, ConstructionId, InsideSurfaceConvectionAlgorithm, NormalizedName,
    OutsideBoundaryCondition, OutsideSurfaceConvectionAlgorithm, Point3, SpaceId, SunExposure,
    Surface, SurfaceId, SurfaceType, TypedModel, WindExposure, Zone, ZoneConvectionAlgorithm,
    ZoneId,
};
use ep_runtime::geometry::{
    ZoneGeometryProperties, ZoneVolumeFace, calculate_zone_volume, prepare_zone_height,
    zone_geometry_input_value,
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

fn zone_json(state: &ZoneGeometryProperties, typed: &TypedModel) -> Value {
    json!({"id":typed.zones[0].id.0,"name":typed.zones[0].name.0,
        "surface_count":state.faces.len(),
        "volume_m3":scalar(state.volume_m3),
        "ceiling_height_m":scalar(state.ceiling_height_m),
        "ceiling_height_entered":state.ceiling_height_entered,
        "floor_area_m2":scalar(state.floor_area_m2),
        "user_entered_floor_area_m2":scalar(state.user_entered_floor_area_m2),
        "geometric_floor_area_m2":scalar(state.geometric_floor_area_m2),
        "ceiling_area_m2":scalar(state.ceiling_area_m2),
        "geometric_ceiling_area_m2":scalar(state.geometric_ceiling_area_m2),
        "has_floor":state.has_floor,"has_roof":state.has_roof})
}

fn vector(value: [f64; 3]) -> Value {
    json!(value.map(scalar))
}

fn owner_snapshot(state: &ZoneGeometryProperties, typed: &TypedModel, phase: &str) -> Value {
    let surfaces: Vec<Value> = state.faces.iter().map(|face| json!({
        "id":face.surface_id.0,"name":face.name,"zone_id":typed.zones[0].id.0,
        "class":format!("{:?}",face.surface_type),"sides":face.vertices.len(),
        "world_vertex_bits":face.vertices.iter().map(|p| [bits(p.x_m),bits(p.y_m),bits(p.z_m)]).collect::<Vec<_>>(),
        "area_m2":scalar(face.area_m2),"gross_area_m2":scalar(face.gross_area_m2),
        "newell_area_vector_m2":vector(face.newell_area_vector_m2),"tilt_deg":scalar(face.tilt_deg)
    })).collect();
    let faces: Vec<Value> = state.faces.iter().map(|face| json!({
        "own_surface_id":face.surface_id.0,"name":face.name,"class":format!("{:?}",face.surface_type)
    })).collect();
    json!({"phase":phase,"zones":[zone_json(state,typed)],"surfaces":surfaces,
        "ordered_volume_base_faces":[{"own_zone_id":typed.zones[0].id.0,"faces":faces}],
        "own_origin_m":vector([state.p0_m.x_m,state.p0_m.y_m,state.p0_m.z_m]),
        "volume_calculation_count":state.volume_calculation_count,
        "volume_differs_by_more_than_five_percent":state.diagnostics.volume_differs_by_more_than_five_percent,
        "method":"copy actual Rust owner fields; no geometry or source-state reconstruction"})
}

fn prepared_read_fields(state: &mut ZoneGeometryProperties, case: &Value) -> Result<(), String> {
    let input = &case["zone_input"];
    state.ceiling_height_m = zone_geometry_input_value(auto_or_number(&input["ceiling_height_m"])?);
    state.volume_m3 = zone_geometry_input_value(auto_or_number(&input["volume_m3"])?);
    state.user_entered_floor_area_m2 =
        zone_geometry_input_value(auto_or_number(&input["user_entered_floor_area_m2"])?);
    let prepared = &case["prepared_zone"];
    state.floor_area_m2 = number(&prepared["floor_area_m2"])?;
    state.geometric_floor_area_m2 = number(&prepared["geometric_floor_area_m2"])?;
    state.ceiling_area_m2 = number(&prepared["ceiling_area_m2"])?;
    state.geometric_ceiling_area_m2 = number(&prepared["geometric_ceiling_area_m2"])?;
    state.has_floor = prepared["has_floor"]
        .as_bool()
        .ok_or("expected boolean has_floor")?;
    state.has_roof = prepared["has_roof"]
        .as_bool()
        .ok_or("expected boolean has_roof")?;
    Ok(())
}

fn execute_owner(case: &Value, typed: &TypedModel) -> Result<(Value, Value, Value), String> {
    // This factory preserves request order; no normal-parser W/F/R sorting.
    let mut state = ZoneGeometryProperties {
        faces: typed
            .surfaces
            .iter()
            .map(ZoneVolumeFace::from_surface)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| error.to_string())?,
        ..ZoneGeometryProperties::default()
    };
    let mut snapshots = json!({"geometry_prepared":owner_snapshot(&state,typed,"after-own-geometry-before-declared-zone-inputs")});
    prepared_read_fields(&mut state, case)?;
    snapshots["declared_pre_volume_read_fields"] =
        owner_snapshot(&state, typed, "declared-input-only-prepared-read-fields");
    prepare_zone_height(&mut state).map_err(|error| error.to_string())?;
    snapshots["before_volume"] =
        owner_snapshot(&state, typed, "after-own-height-before-first-volume");
    calculate_zone_volume(&mut state).map_err(|error| error.to_string())?;
    snapshots["after_volume"] = owner_snapshot(&state, typed, "after-first-own-volume");
    calculate_zone_volume(&mut state).map_err(|error| error.to_string())?;
    snapshots["after_second_volume"] = owner_snapshot(
        &state,
        typed,
        "after-second-own-volume-without-height-repeat",
    );
    let d = &state.diagnostics;
    let edges: Vec<Value> = d
        .initial_edges_not_used_twice
        .iter()
        .map(|edge| {
            json!({
                "surface_id":edge.surface_id.0,"count":edge.count,
                "start_m":vector([edge.start_m.x_m,edge.start_m.y_m,edge.start_m.z_m]),
                "end_m":vector([edge.end_m.x_m,edge.end_m.y_m,edge.end_m.z_m]),
                "other_surface_ids":edge.other_surface_ids.iter().map(|id|id.0).collect::<Vec<_>>()
            })
        })
        .collect();
    let auxiliary = json!({"initial_unique_vertex_count":d.initial_unique_vertex_count,
        "initial_edges_not_used_twice":edges,"enclosed":d.initially_closed,
        "edges_winding_consistent":d.edges_winding_consistent,
        "floor_horizontal":d.floor_horizontal,"roof_horizontal":d.roof_horizontal,
        "walls_vertical":d.walls_vertical,"same_wall_height":d.same_wall_height,
        "signed_polyhedron_volume_m3":d.signed_polyhedron_volume_m3.map(scalar),
        "topology_rejection":d.topology_rejection,
        "volume_calculation_count":state.volume_calculation_count,
        "method":"actual Rust owner numerical observations; not C++ internal local trace or warning counters"});
    Ok((zone_json(&state, typed), snapshots, auxiliary))
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
        let (zone, source_state, auxiliary) = if paired {
            execute_owner(case, &typed)?
        } else {
            (Value::Null, Value::Null, Value::Null)
        };
        results.push(json!({
            "case_id":case_id,"kind":kind,"input":case,
            "status":if paired {"source_complete"} else {"unsupported_source_only"},
            "route":if paired {"ep_runtime::geometry::prepare_zone_height + calculate_zone_volume"} else {"unsupported-source-only-original-diagnostic"},
            "zone":zone,"typed_surface_order":typed_surface_order,
            "numeric_input_identity_checked":true,"canonical_coordinate_conversion_claimed":false,
            "prepared_read_fields_consumed":paired,"original_parser_admission_claimed":false,
            "admission_checked":paired,"source_state_observed":paired,"physics_executed":false,
            "unconsumed_prepared_input_groups":["prepared_implicit_space"],
            "source_state":source_state,"auxiliary_rust_helpers":auxiliary,
            "unavailable_source_state_phases":if paired {json!([])} else {json!(["geometry_prepared","declared_pre_volume_read_fields","before_volume","after_volume","after_second_volume"])},
            "unimplemented_fields":if paired {json!([])} else {json!(["source-only open/reversed/duplicated topology fallback and diagnostics"])},
            "source_only_state_unpaired":["implicit Space","global counters","warning/error IO","source scratch allocations"]
        }));
    }
    Ok(
        json!({"schema":"geo03-helper-results.v1","implementation_stage":"canonical_owned_state",
        "cases":results,"reference_outputs_supplied_to_Rust":false,"expected_answers_supplied":false,
        "original_parser_admission_claimed":false,"physics_executed":false,"gates_updated":false}),
    )
}
