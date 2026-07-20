use super::super::{Compiler, ObjectCoverageStatus, compile_raw_model, typed_coverage_status};
use ep_model::{ScheduleId, SurfaceIncidentSolarMultiplierRequestId, TypedModel};
use ep_raw_model::{
    FieldName, ObjectName, ObjectType, RawModel, RawObject, RawValue, parse_epjson_str,
};

const OBJECT_TYPE: &str = "SurfaceProperty:IncidentSolarMultiplier";

fn request_object_mut<'a>(
    raw: &'a mut RawModel,
    object_name: &str,
) -> Result<&'a mut RawObject, std::io::Error> {
    raw.objects
        .get_mut(&ObjectType(OBJECT_TYPE.to_string()))
        .and_then(|objects| objects.get_mut(&ObjectName(object_name.to_string())))
        .ok_or_else(|| std::io::Error::other(format!("missing {OBJECT_TYPE}/{object_name}")))
}

#[test]
fn requests_capture_defaults_resolved_schedules_and_unresolved_surface_names()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Schedule:Constant": {
                "Solar Gate": {"hourly_value":0.5}
            },
            "SurfaceProperty:IncidentSolarMultiplier": {
                "Alpha Declaration": {
                    "surface_name":"  Window Not Yet Typed  ",
                    "incident_solar_multiplier":0.25,
                    "incident_solar_multiplier_schedule_name":"solar gate"
                },
                "Zulu Declaration": {
                    "surface_name":"Second Deferred Window",
                    "incident_solar_multiplier_schedule_name":"   "
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert!(result.report.coverage.iter().any(|coverage| {
        coverage.object_type == OBJECT_TYPE
            && coverage.object_count == 2
            && coverage.status == ObjectCoverageStatus::Typed
    }));

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected incident-solar request state"))?;
    assert!(model.surfaces.is_empty());
    assert_eq!(model.surface_incident_solar_multiplier_requests.len(), 2);
    let explicit = &model.surface_incident_solar_multiplier_requests[0];
    assert_eq!(explicit.id, SurfaceIncidentSolarMultiplierRequestId(0));
    assert_eq!(explicit.declaration_name.0, "ALPHA DECLARATION");
    assert_eq!(explicit.surface_name.0, "WINDOW NOT YET TYPED");
    assert_eq!(explicit.multiplier, 0.25);
    assert_eq!(explicit.schedule, Some(ScheduleId(0)));

    let defaulted = &model.surface_incident_solar_multiplier_requests[1];
    assert_eq!(defaulted.id, SurfaceIncidentSolarMultiplierRequestId(1));
    assert_eq!(defaulted.declaration_name.0, "ZULU DECLARATION");
    assert_eq!(defaulted.surface_name.0, "SECOND DEFERRED WINDOW");
    assert_eq!(defaulted.multiplier, 1.0);
    assert_eq!(defaulted.schedule, None);
    assert_eq!(model.object_count(), 4);
    assert_eq!(result.report.typed_object_count, model.object_count());
    assert!(result.report.defaults_applied.iter().any(|default| {
        default.object_type == OBJECT_TYPE
            && default.object_name == "Zulu Declaration"
            && default.field == "incident_solar_multiplier"
            && default.value == "1.0"
    }));
    Ok(())
}

#[test]
fn declaration_names_are_nonsemantic_and_do_not_reserve_surface_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "SurfaceProperty:IncidentSolarMultiplier": {
                "Label": {"surface_name":"Window A"},
                "label": {"surface_name":"Window B"}
            }
        }"#,
    )?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected nonsemantic declaration snapshots"))?;
    assert_eq!(model.surface_incident_solar_multiplier_requests.len(), 2);
    assert!(
        model
            .surface_incident_solar_multiplier_requests
            .iter()
            .all(|request| request.declaration_name.0 == "LABEL")
    );
    assert!(model.surface_names.is_empty());
    Ok(())
}

#[test]
fn duplicate_valid_targets_fail_close_the_whole_pass() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "SurfaceProperty:IncidentSolarMultiplier": {
                "First Valid": {"surface_name":"Window A"},
                "Second Duplicate": {"surface_name":"  window a "}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_surface_incident_solar_multiplier_requests(&mut model);

    assert!(model.surface_incident_solar_multiplier_requests.is_empty());
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateIncidentSolarMultiplierSurface"
            && diagnostic.object_name.as_deref() == Some("Second Duplicate")
            && diagnostic.field.as_deref() == Some("surface_name")
    }));
    Ok(())
}

#[test]
fn invalid_request_does_not_reserve_its_target_name() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "SurfaceProperty:IncidentSolarMultiplier": {
                "A Invalid": {
                    "surface_name":"Window A",
                    "incident_solar_multiplier":1.1
                },
                "B Valid": {"surface_name":" window a "}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_surface_incident_solar_multiplier_requests(&mut model);

    assert!(model.surface_incident_solar_multiplier_requests.is_empty());
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_name.as_deref() == Some("A Invalid")
            && diagnostic.field.as_deref() == Some("incident_solar_multiplier")
    }));
    assert!(
        compiler
            .diagnostics
            .iter()
            .all(|diagnostic| { diagnostic.code != "DuplicateIncidentSolarMultiplierSurface" })
    );
    Ok(())
}

#[test]
fn multiplier_accepts_both_inclusive_endpoints() -> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "SurfaceProperty:IncidentSolarMultiplier": {
                "Zero": {"surface_name":"Window Zero", "incident_solar_multiplier":0.0},
                "One": {"surface_name":"Window One", "incident_solar_multiplier":1.0}
            }
        }"#,
    )?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected inclusive endpoint requests"))?;
    let mut multipliers = model
        .surface_incident_solar_multiplier_requests
        .iter()
        .map(|request| request.multiplier)
        .collect::<Vec<_>>();
    multipliers.sort_by(f64::total_cmp);
    assert_eq!(multipliers, vec![0.0, 1.0]);
    Ok(())
}

#[test]
fn malformed_fields_and_nonfinite_values_are_transactional()
-> Result<(), Box<dyn std::error::Error>> {
    let base = parse_epjson_str(
        r#"{
            "SurfaceProperty:IncidentSolarMultiplier": {
                "Invalid": {"surface_name":"Deferred Window"}
            }
        }"#,
    )?;
    let cases = [
        ("surface_name", None, "MissingRequiredField"),
        (
            "surface_name",
            Some(RawValue::String("   ".to_string())),
            "MissingRequiredField",
        ),
        (
            "surface_name",
            Some(RawValue::Number("1".to_string())),
            "InvalidFieldType",
        ),
        (
            "incident_solar_multiplier",
            Some(RawValue::String("0.5".to_string())),
            "InvalidFieldType",
        ),
        (
            "incident_solar_multiplier",
            Some(RawValue::Number("NaN".to_string())),
            "InvalidNumber",
        ),
        (
            "incident_solar_multiplier",
            Some(RawValue::Number("-0.1".to_string())),
            "InvalidNumericRange",
        ),
        (
            "incident_solar_multiplier",
            Some(RawValue::Number("1.1".to_string())),
            "InvalidNumericRange",
        ),
        (
            "incident_solar_multiplier_schedule_name",
            Some(RawValue::Number("1".to_string())),
            "InvalidFieldType",
        ),
        (
            "incident_solar_multiplier_schedule_name",
            Some(RawValue::String("Missing Schedule".to_string())),
            "MissingReference",
        ),
    ];

    for (field, value, code) in cases {
        let mut raw = base.clone();
        let fields = &mut request_object_mut(&mut raw, "Invalid")?.fields;
        if let Some(value) = value {
            fields.insert(FieldName(field.to_string()), value);
        } else {
            fields.remove(&FieldName(field.to_string()));
        }

        let mut compiler = Compiler::new(&raw, None);
        let mut model = TypedModel::default();
        compiler.parse_surface_incident_solar_multiplier_requests(&mut model);
        assert!(model.surface_incident_solar_multiplier_requests.is_empty());
        assert!(
            compiler.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == code
                    && diagnostic.object_name.as_deref() == Some("Invalid")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "field={field}, code={code}, diagnostics={:?}",
            compiler.diagnostics
        );
    }

    Ok(())
}

#[test]
fn blank_nonsemantic_declaration_name_is_retained() -> Result<(), Box<dyn std::error::Error>> {
    let result = compile_raw_model(&parse_epjson_str(
        r#"{
            "SurfaceProperty:IncidentSolarMultiplier": {
                "": {"surface_name":"Deferred Window"}
            }
        }"#,
    )?);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let request = &result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected blank declaration snapshot"))?
        .surface_incident_solar_multiplier_requests[0];
    assert!(request.declaration_name.0.is_empty());
    assert_eq!(request.surface_name.0, "DEFERRED WINDOW");
    Ok(())
}
