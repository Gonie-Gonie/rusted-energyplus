use super::super::{CompileResult, Compiler, DiagnosticSeverity, compile_raw_model};
use ep_model::{SpaceId, SpaceOrigin, SpaceTypeId, TypedModel};
use ep_raw_model::{RawModel, parse_epjson_str};

fn compile_fixture(
    zones: &str,
    spaces: Option<&str>,
    surfaces: &str,
) -> Result<CompileResult, Box<dyn std::error::Error>> {
    let raw = raw_fixture(zones, spaces, surfaces)?;
    Ok(compile_raw_model(&raw))
}

fn raw_fixture(
    zones: &str,
    spaces: Option<&str>,
    surfaces: &str,
) -> Result<RawModel, Box<dyn std::error::Error>> {
    let spaces = spaces.map_or_else(String::new, |instances| {
        format!(r#", "Space": {{{instances}}}"#)
    });
    let epjson = format!(
        r#"{{
            "Material:NoMass": {{
                "R13": {{"roughness":"Rough","thermal_resistance":1.0}}
            }},
            "Construction": {{
                "Wall Construction": {{"outside_layer":"R13"}}
            }},
            "Zone": {{{zones}}}
            {spaces},
            "BuildingSurface:Detailed": {{{surfaces}}}
        }}"#
    );
    Ok(parse_epjson_str(&epjson)?)
}

fn surface(name: &str, zone: &str, space_field: &str, x_offset: f64) -> String {
    format!(
        r#""{name}": {{
            "surface_type":"Wall",
            "construction_name":"Wall Construction",
            "zone_name":"{zone}"
            {space_field},
            "outside_boundary_condition":"Outdoors",
            "vertices":[
                {{"vertex_x_coordinate":{x_offset},"vertex_y_coordinate":0,"vertex_z_coordinate":0}},
                {{"vertex_x_coordinate":{x_offset},"vertex_y_coordinate":1,"vertex_z_coordinate":0}},
                {{"vertex_x_coordinate":{x_offset},"vertex_y_coordinate":1,"vertex_z_coordinate":1}},
                {{"vertex_x_coordinate":{x_offset},"vertex_y_coordinate":0,"vertex_z_coordinate":1}}
            ]
        }}"#
    )
}

fn model(result: &CompileResult) -> Result<&TypedModel, Box<dyn std::error::Error>> {
    if result.has_errors() {
        return Err(std::io::Error::other(format!(
            "fixture should compile without errors: {:?}",
            result.report.diagnostics
        ))
        .into());
    }
    result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed model").into())
}

fn surface_space(model: &TypedModel, surface_name: &str) -> SpaceId {
    let surface = model
        .surface_names
        .resolve(surface_name)
        .expect("fixture surface should resolve");
    model.surfaces[surface.0 as usize].space
}

#[test]
fn default_space_accepts_missing_blank_and_all_explicit_surface_assignments()
-> Result<(), Box<dyn std::error::Error>> {
    let unassigned = [
        surface("Missing Field", "Zone One", "", 0.0),
        surface("Blank Field", "Zone One", r#", "space_name":"""#, 1.0),
    ]
    .join(",");
    let unassigned_result = compile_fixture(r#""Zone One":{}"#, None, &unassigned)?;
    let unassigned_model = model(&unassigned_result)?;

    assert_eq!(unassigned_model.spaces.len(), 1);
    assert_eq!(
        unassigned_model.spaces[0].origin,
        SpaceOrigin::AutoZoneDefault
    );
    assert_eq!(surface_space(unassigned_model, "Missing Field"), SpaceId(0));
    assert_eq!(surface_space(unassigned_model, "Blank Field"), SpaceId(0));

    let explicit = [
        surface(
            "Explicit One",
            "Zone One",
            r#", "space_name":"Zone One""#,
            0.0,
        ),
        surface(
            "Explicit Two",
            "Zone One",
            r#", "space_name":"zone one""#,
            1.0,
        ),
    ]
    .join(",");
    let explicit_result = compile_fixture(r#""Zone One":{}"#, None, &explicit)?;
    let explicit_model = model(&explicit_result)?;

    assert_eq!(explicit_model.spaces.len(), 1);
    assert_eq!(
        explicit_model.spaces[0].origin,
        SpaceOrigin::AutoZoneDefault
    );
    assert_eq!(surface_space(explicit_model, "Explicit One"), SpaceId(0));
    assert_eq!(surface_space(explicit_model, "Explicit Two"), SpaceId(0));
    Ok(())
}

#[test]
fn wholly_unassigned_zone_uses_its_last_authored_space_without_a_remainder()
-> Result<(), Box<dyn std::error::Error>> {
    let surfaces = [
        surface("Wall One", "Zone One", "", 0.0),
        surface("Wall Two", "Zone One", "", 1.0),
    ]
    .join(",");
    let result = compile_fixture(
        r#""Zone One":{}"#,
        Some(
            r#"
                "Alpha Space":{"zone_name":"Zone One"},
                "Zulu Space":{"zone_name":"Zone One"}
            "#,
        ),
        &surfaces,
    )?;
    let model = model(&result)?;

    assert_eq!(model.spaces.len(), 2);
    assert_eq!(model.zones[0].spaces, vec![SpaceId(0), SpaceId(1)]);
    assert_eq!(surface_space(model, "Wall One"), SpaceId(1));
    assert_eq!(surface_space(model, "Wall Two"), SpaceId(1));
    assert!(
        model
            .spaces
            .iter()
            .all(|space| space.origin == SpaceOrigin::Authored)
    );
    Ok(())
}

#[test]
fn mixed_assignments_create_one_general_remainder_and_route_every_blank_surface()
-> Result<(), Box<dyn std::error::Error>> {
    let surfaces = [
        surface(
            "Assigned Wall",
            "Zone One",
            r#", "space_name":"Office Space""#,
            0.0,
        ),
        surface("Blank Wall", "Zone One", r#", "space_name":"""#, 1.0),
        surface("Missing Wall", "Zone One", "", 2.0),
    ]
    .join(",");
    let result = compile_fixture(
        r#""Zone One":{}"#,
        Some(r#""Office Space":{"zone_name":"Zone One","space_type":"Office"}"#),
        &surfaces,
    )?;
    let model = model(&result)?;

    assert_eq!(model.spaces.len(), 2);
    assert_eq!(model.zones[0].spaces, vec![SpaceId(0), SpaceId(1)]);
    assert_eq!(surface_space(model, "Assigned Wall"), SpaceId(0));
    assert_eq!(surface_space(model, "Blank Wall"), SpaceId(1));
    assert_eq!(surface_space(model, "Missing Wall"), SpaceId(1));

    let remainder = &model.spaces[1];
    assert_eq!(remainder.name.0, "ZONE ONE-REMAINDER");
    assert_eq!(remainder.origin, SpaceOrigin::AutoZoneRemainder);
    assert_eq!(remainder.space_type.0, "GENERAL");
    assert_eq!(remainder.space_type_id, SpaceTypeId(1));
    assert_eq!(model.space_type_names.names()[0].0, "OFFICE");
    assert_eq!(model.space_type_names.names()[1].0, "GENERAL");
    assert_eq!(
        model.authored_space_names.resolve("Zone One-Remainder"),
        None
    );
    Ok(())
}

#[test]
fn all_explicit_authored_assignments_do_not_create_a_remainder()
-> Result<(), Box<dyn std::error::Error>> {
    let surfaces = [
        surface(
            "Alpha Wall",
            "Zone One",
            r#", "space_name":"Alpha Space""#,
            0.0,
        ),
        surface(
            "Zulu Wall",
            "Zone One",
            r#", "space_name":"Zulu Space""#,
            1.0,
        ),
    ]
    .join(",");
    let result = compile_fixture(
        r#""Zone One":{}"#,
        Some(
            r#"
                "Alpha Space":{"zone_name":"Zone One"},
                "Zulu Space":{"zone_name":"Zone One"}
            "#,
        ),
        &surfaces,
    )?;
    let model = model(&result)?;

    assert_eq!(model.spaces.len(), 2);
    assert_eq!(surface_space(model, "Alpha Wall"), SpaceId(0));
    assert_eq!(surface_space(model, "Zulu Wall"), SpaceId(1));
    assert!(
        model
            .spaces
            .iter()
            .all(|space| space.origin == SpaceOrigin::Authored)
    );
    Ok(())
}

#[test]
fn surface_space_assignment_rejects_unknown_cross_zone_and_non_string_values()
-> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        (
            r#""Zone One":{}"#,
            None,
            surface(
                "Unknown Space",
                "Zone One",
                r#", "space_name":"Missing Space""#,
                0.0,
            ),
        ),
        (
            r#""Zone A":{},"Zone B":{}"#,
            None,
            surface("Cross Zone", "Zone A", r#", "space_name":"Zone B""#, 0.0),
        ),
        (
            r#""Zone One":{}"#,
            None,
            surface("Non String", "Zone One", r#", "space_name":42"#, 0.0),
        ),
    ];

    for (zones, spaces, surface) in cases {
        let result = compile_fixture(zones, spaces, &surface)?;
        assert!(result.has_errors(), "expected rejection: {surface}");
        assert!(result.model.is_none());
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.object_type == "BuildingSurface:Detailed"
                && diagnostic.field.as_deref() == Some("space_name")
        }));
    }
    Ok(())
}

#[test]
fn surface_input_errors_skip_remainder_publication_at_the_source_fatal_barrier()
-> Result<(), Box<dyn std::error::Error>> {
    let surfaces = [
        surface(
            "Assigned Wall",
            "Zone One",
            r#", "space_name":"Office Space""#,
            0.0,
        ),
        surface("Blank Wall", "Zone One", "", 1.0),
        surface(
            "Invalid Wall",
            "Zone One",
            r#", "space_name":"Missing Space""#,
            2.0,
        ),
    ]
    .join(",");
    let raw = raw_fixture(
        r#""Zone One":{}"#,
        Some(r#""Office Space":{"zone_name":"Zone One","space_type":"Office"}"#),
        &surfaces,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    compiler.parse_zones(&mut model);
    compiler.parse_space_data(&mut model);
    compiler.parse_surfaces(&mut model);

    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference"
            && diagnostic.object_name.as_deref() == Some("Invalid Wall")
            && diagnostic.field.as_deref() == Some("space_name")
    }));
    assert_eq!(model.surfaces.len(), 2);
    assert_eq!(model.spaces.len(), 1);
    assert_eq!(model.spaces[0].origin, SpaceOrigin::Authored);
    assert_eq!(model.zones[0].spaces, vec![SpaceId(0)]);
    assert_eq!(model.space_type_names.names().len(), 1);
    assert_eq!(model.space_type_names.names()[0].0, "OFFICE");
    assert_eq!(surface_space(&model, "Assigned Wall"), SpaceId(0));
    assert_eq!(surface_space(&model, "Blank Wall"), SpaceId(0));
    Ok(())
}

#[test]
fn multi_zone_remainders_follow_zone_order_reuse_general_and_do_not_count_as_objects()
-> Result<(), Box<dyn std::error::Error>> {
    let surfaces = [
        surface("A Assigned", "Zone A", r#", "space_name":"A Office""#, 0.0),
        surface("A Unassigned", "Zone A", "", 1.0),
        surface("B Assigned", "Zone B", r#", "space_name":"B Office""#, 2.0),
        surface("B Unassigned", "Zone B", "", 3.0),
    ]
    .join(",");
    let result = compile_fixture(
        r#""Zone A":{},"Zone B":{}"#,
        Some(
            r#"
                "A Office":{"zone_name":"Zone A","space_type":"Office"},
                "B Office":{"zone_name":"Zone B","space_type":"Office"}
            "#,
        ),
        &surfaces,
    )?;
    let model = model(&result)?;

    assert_eq!(model.spaces.len(), 4);
    assert_eq!(model.spaces[2].name.0, "ZONE A-REMAINDER");
    assert_eq!(model.spaces[3].name.0, "ZONE B-REMAINDER");
    assert_eq!(model.spaces[2].origin, SpaceOrigin::AutoZoneRemainder);
    assert_eq!(model.spaces[3].origin, SpaceOrigin::AutoZoneRemainder);
    assert_eq!(model.spaces[2].space_type_id, SpaceTypeId(1));
    assert_eq!(model.spaces[3].space_type_id, SpaceTypeId(1));
    assert_eq!(model.space_type_names.len(), 2);
    assert_eq!(model.space_type_names.names()[1].0, "GENERAL");
    assert_eq!(model.zones[0].spaces, vec![SpaceId(0), SpaceId(2)]);
    assert_eq!(model.zones[1].spaces, vec![SpaceId(1), SpaceId(3)]);
    assert_eq!(surface_space(model, "A Unassigned"), SpaceId(2));
    assert_eq!(surface_space(model, "B Unassigned"), SpaceId(3));

    // Version + material + construction + 2 Zones + 2 authored Spaces + 4 surfaces.
    assert_eq!(model.object_count(), 11);
    Ok(())
}
