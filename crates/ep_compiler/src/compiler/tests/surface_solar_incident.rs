use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{NormalizedName, SurfaceSolarIncidentId, TypedModel};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "SurfaceProperty:SolarIncidentInside";
const SCHEDULE_FIELD: &str = "inside_surface_incident_sun_solar_radiation_schedule_name";

fn model_with_incidents(incidents: &str, extra_objects: &str) -> String {
    format!(
        r#"{{
            "Material:NoMass": {{"Layer": {{"roughness":"Rough","thermal_resistance":1}}}},
            "Construction": {{
                "Surface Construction": {{"outside_layer":"Layer"}},
                "Alternate Construction": {{"outside_layer":"Layer"}}
            }},
            "Schedule:Constant": {{
                "Solar A": {{"hourly_value":100}},
                "Solar B": {{"hourly_value":200}}
            }},
            "Zone": {{"Zone One": {{}}}},
            "BuildingSurface:Detailed": {{
                "Wall One": {{
                    "surface_type":"Wall","construction_name":"Surface Construction","zone_name":"Zone One","outside_boundary_condition":"Outdoors",
                    "vertices":[{{"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0}},{{"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0}},{{"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":0}}]
                }},
                "Wall Two": {{
                    "surface_type":"Wall","construction_name":"Surface Construction","zone_name":"Zone One","outside_boundary_condition":"Outdoors",
                    "vertices":[{{"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":1}},{{"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":1}},{{"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":1}}]
                }}
            }},
            {extra_objects}
            "{OBJECT_TYPE}": {{{incidents}}}
        }}"#
    )
}

fn has_error(result: &CompileResult, code: &str, object_name: &str, field: &str) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == Some(field)
    })
}

#[test]
fn solar_incident_inside_resolves_typed_dependencies_and_preserves_allowed_distinctions()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(&model_with_incidents(
        r#"
            "Shared Name": {
                "surface_name":" wall one ",
                "construction_name":"alternate construction",
                "inside_surface_incident_sun_solar_radiation_schedule_name":"SOLAR A"
            },
            "shared name": {
                "surface_name":"WALL ONE",
                "construction_name":"surface construction",
                "inside_surface_incident_sun_solar_radiation_schedule_name":"solar b"
            }
        "#,
        "",
    ))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed solar incidents"))?;

    assert_eq!(model.surface_solar_incidents.len(), 2);
    assert_eq!(
        model
            .surface_solar_incidents
            .iter()
            .map(|incident| incident.id)
            .collect::<Vec<_>>(),
        vec![SurfaceSolarIncidentId(0), SurfaceSolarIncidentId(1)]
    );
    assert!(
        model
            .surface_solar_incidents
            .iter()
            .all(|incident| incident.name == NormalizedName::new("shared name"))
    );
    let wall_one = model.surface_names.resolve("Wall One").expect("surface");
    assert!(
        model
            .surface_solar_incidents
            .iter()
            .all(|incident| incident.surface == wall_one)
    );
    let constructions = model
        .surface_solar_incidents
        .iter()
        .map(|incident| incident.construction)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(constructions.len(), 2);
    assert!(
        constructions.contains(
            &model
                .construction_names
                .resolve("Surface Construction")
                .expect("construction")
        )
    );
    assert!(
        constructions.contains(
            &model
                .construction_names
                .resolve("Alternate Construction")
                .expect("construction")
        )
    );
    assert_eq!(model.object_count(), 11);
    assert_eq!(result.report.typed_object_count, model.object_count());
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.coverage.iter().any(|entry| {
        entry.object_type == OBJECT_TYPE
            && entry.object_count == 2
            && entry.status == ObjectCoverageStatus::Typed
    }));
    Ok(())
}

fn parse_solar_incident_prerequisites(compiler: &mut Compiler<'_>, model: &mut TypedModel) {
    compiler.parse_materials(model);
    compiler.parse_constructions(model);
    compiler.parse_schedule_type_limits(model);
    compiler.parse_schedules(model);
    compiler.parse_zones(model);
    compiler.parse_space_data(model);
    compiler.parse_surfaces(model);
}

#[test]
fn duplicate_surface_construction_pair_fails_close_the_whole_pass()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(&model_with_incidents(
        r#"
            "First": {
                "surface_name":"Wall One","construction_name":"Surface Construction",
                "inside_surface_incident_sun_solar_radiation_schedule_name":"Solar A"
            },
            "Second": {
                "surface_name":" wall one ","construction_name":"surface construction",
                "inside_surface_incident_sun_solar_radiation_schedule_name":"Solar B"
            }
        "#,
        "",
    ))?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    parse_solar_incident_prerequisites(&mut compiler, &mut model);
    compiler.parse_surface_solar_incidents(&mut model);

    assert!(model.surface_solar_incidents.is_empty());
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateSurfaceSolarIncidentPair"
            && diagnostic.object_name.as_deref() == Some("Second")
            && diagnostic.field.as_deref() == Some("construction_name")
    }));
    Ok(())
}

#[test]
fn invalid_record_does_not_reserve_its_surface_construction_pair()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(&model_with_incidents(
        r#"
            "A Invalid": {
                "surface_name":"Wall One","construction_name":"Surface Construction",
                "inside_surface_incident_sun_solar_radiation_schedule_name":"Missing Schedule"
            },
            "B Valid": {
                "surface_name":"wall one","construction_name":"surface construction",
                "inside_surface_incident_sun_solar_radiation_schedule_name":"Solar A"
            }
        "#,
        "",
    ))?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    parse_solar_incident_prerequisites(&mut compiler, &mut model);
    compiler.parse_surface_solar_incidents(&mut model);

    assert!(model.surface_solar_incidents.is_empty());
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference"
            && diagnostic.object_name.as_deref() == Some("A Invalid")
            && diagnostic.field.as_deref() == Some(SCHEDULE_FIELD)
    }));
    assert!(
        compiler
            .diagnostics
            .iter()
            .all(|diagnostic| { diagnostic.code != "DuplicateSurfaceSolarIncidentPair" })
    );
    Ok(())
}

#[test]
fn semantic_name_required_fields_and_references_fail_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        (
            r#""": {"surface_name":"Wall One","construction_name":"Surface Construction","inside_surface_incident_sun_solar_radiation_schedule_name":"Solar A"}"#,
            "MissingRequiredField",
            "",
            "name",
        ),
        (
            r#""Missing Surface": {"construction_name":"Surface Construction","inside_surface_incident_sun_solar_radiation_schedule_name":"Solar A"}"#,
            "MissingRequiredField",
            "Missing Surface",
            "surface_name",
        ),
        (
            r#""Blank Construction": {"surface_name":"Wall One","construction_name":"   ","inside_surface_incident_sun_solar_radiation_schedule_name":"Solar A"}"#,
            "MissingRequiredField",
            "Blank Construction",
            "construction_name",
        ),
        (
            r#""Malformed Schedule": {"surface_name":"Wall One","construction_name":"Surface Construction","inside_surface_incident_sun_solar_radiation_schedule_name":1}"#,
            "InvalidFieldType",
            "Malformed Schedule",
            SCHEDULE_FIELD,
        ),
        (
            r#""Unknown Surface": {"surface_name":"Missing Wall","construction_name":"Surface Construction","inside_surface_incident_sun_solar_radiation_schedule_name":"Solar A"}"#,
            "MissingReference",
            "Unknown Surface",
            "surface_name",
        ),
        (
            r#""Unknown Construction": {"surface_name":"Wall One","construction_name":"Missing Construction","inside_surface_incident_sun_solar_radiation_schedule_name":"Solar A"}"#,
            "MissingReference",
            "Unknown Construction",
            "construction_name",
        ),
        (
            r#""Unknown Schedule": {"surface_name":"Wall One","construction_name":"Surface Construction","inside_surface_incident_sun_solar_radiation_schedule_name":"Missing Schedule"}"#,
            "MissingReference",
            "Unknown Schedule",
            SCHEDULE_FIELD,
        ),
    ];

    for (incident, code, object_name, field) in cases {
        let result = compile_raw_model(&parse_epjson_str(&model_with_incidents(incident, ""))?);
        assert!(result.model.is_none());
        assert!(
            has_error(&result, code, object_name, field),
            "case={object_name}, diagnostics={:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn raw_fenestration_surface_name_does_not_satisfy_the_typed_surface_reference()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(&model_with_incidents(
        r#"
            "Raw Window Target": {
                "surface_name":"Raw Window","construction_name":"Surface Construction",
                "inside_surface_incident_sun_solar_radiation_schedule_name":"Solar A"
            }
        "#,
        r#""FenestrationSurface:Detailed": {"Raw Window": {}},"#,
    ))?);

    assert!(result.model.is_none());
    assert!(has_error(
        &result,
        "MissingReference",
        "Raw Window Target",
        "surface_name"
    ));
    Ok(())
}
