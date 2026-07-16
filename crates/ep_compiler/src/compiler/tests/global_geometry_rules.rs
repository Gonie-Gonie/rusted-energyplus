use super::super::{
    CompileResult, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{GeometryCoordinateSystem, StartingVertexPosition, VertexEntryDirection};
use ep_raw_model::parse_epjson_str;

fn compile_rules(fields: &str) -> CompileResult {
    let epjson = format!(
        r#"{{
            "GlobalGeometryRules": {{
                "Rules": {{
                    {fields}
                }}
            }}
        }}"#
    );
    let raw_model = parse_epjson_str(&epjson).expect("GlobalGeometryRules epJSON should parse");
    compile_raw_model(&raw_model)
}

fn required_fields(corner: &str, direction: &str, coordinate_system: &str) -> String {
    format!(
        r#""starting_vertex_position": "{corner}",
           "vertex_entry_direction": "{direction}",
           "coordinate_system": "{coordinate_system}""#
    )
}

#[test]
fn parses_all_normalized_values_and_aliases() {
    for (input, expected) in [
        ("UpperLeftCorner", StartingVertexPosition::UpperLeftCorner),
        ("LowerLeftCorner", StartingVertexPosition::LowerLeftCorner),
        ("UpperRightCorner", StartingVertexPosition::UpperRightCorner),
        ("LowerRightCorner", StartingVertexPosition::LowerRightCorner),
    ] {
        let result = compile_rules(&required_fields(input, "Counterclockwise", "World"));
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        assert_eq!(
            result
                .model
                .expect("rules should compile")
                .global_geometry_rules
                .expect("rules should be typed")
                .starting_vertex_position,
            expected
        );
    }

    for (input, expected) in [
        ("CCW", VertexEntryDirection::CounterClockwise),
        ("Counterclockwise", VertexEntryDirection::CounterClockwise),
        ("CW", VertexEntryDirection::Clockwise),
        ("Clockwise", VertexEntryDirection::Clockwise),
    ] {
        let result = compile_rules(&required_fields("UpperLeftCorner", input, "World"));
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        assert_eq!(
            result
                .model
                .expect("rules should compile")
                .global_geometry_rules
                .expect("rules should be typed")
                .vertex_entry_direction,
            expected
        );
    }

    for (input, expected) in [
        ("Relative", GeometryCoordinateSystem::Relative),
        ("World", GeometryCoordinateSystem::World),
        ("Absolute", GeometryCoordinateSystem::World),
    ] {
        let result = compile_rules(&required_fields(
            "UpperLeftCorner",
            "Counterclockwise",
            input,
        ));
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        assert_eq!(
            result
                .model
                .expect("rules should compile")
                .global_geometry_rules
                .expect("rules should be typed")
                .coordinate_system,
            expected
        );
    }

    let result = compile_rules(
        r#""starting_vertex_position": "UpperLeftCorner",
           "vertex_entry_direction": "Counterclockwise",
           "coordinate_system": "Relative",
           "daylighting_reference_point_coordinate_system": "Absolute",
           "rectangular_surface_coordinate_system": "World""#,
    );
    let rules = result
        .model
        .expect("rules should compile")
        .global_geometry_rules
        .expect("rules should be typed");
    assert_eq!(
        rules.daylighting_reference_point_coordinate_system,
        GeometryCoordinateSystem::World
    );
    assert_eq!(
        rules.rectangular_surface_coordinate_system,
        GeometryCoordinateSystem::World
    );
}

#[test]
fn defaults_optional_coordinate_systems_and_reports_typed_coverage() {
    let result = compile_rules(&required_fields(
        "UpperLeftCorner",
        "Counterclockwise",
        "World",
    ));

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status("GlobalGeometryRules"),
        ObjectCoverageStatus::Typed
    );
    assert_eq!(result.report.typed_object_count, 2);
    assert!(result.report.coverage.iter().any(|entry| {
        entry.object_type == "GlobalGeometryRules"
            && entry.status == ObjectCoverageStatus::Typed
            && entry.object_count == 1
    }));
    let rules = result
        .model
        .expect("rules should compile")
        .global_geometry_rules
        .expect("rules should be typed");
    assert_eq!(
        rules.daylighting_reference_point_coordinate_system,
        GeometryCoordinateSystem::Relative
    );
    assert_eq!(
        rules.rectangular_surface_coordinate_system,
        GeometryCoordinateSystem::Relative
    );
    for field in [
        "daylighting_reference_point_coordinate_system",
        "rectangular_surface_coordinate_system",
    ] {
        assert!(result.report.defaults_applied.iter().any(|default| {
            default.object_type == "GlobalGeometryRules"
                && default.object_name == "Rules"
                && default.field == field
                && default.value == "Relative"
        }));
    }
}

#[test]
fn accepts_missing_global_geometry_rules_for_compatibility() {
    let raw_model = parse_epjson_str("{}").expect("empty epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(result.report.typed_object_count, 1);
    assert!(
        result
            .model
            .expect("empty compatibility model should compile")
            .global_geometry_rules
            .is_none()
    );
}

#[test]
fn rejects_invalid_required_corner_and_direction() {
    let result = compile_rules(&required_fields("Center", "Diagonal", "World"));

    assert!(result.has_errors());
    assert!(result.model.is_none());
    let invalid_fields = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "InvalidEnumValue"
        })
        .filter_map(|diagnostic| diagnostic.field.as_deref())
        .collect::<Vec<_>>();
    assert_eq!(
        invalid_fields,
        vec!["starting_vertex_position", "vertex_entry_direction"]
    );
}

#[test]
fn rejects_blank_or_missing_required_corner_and_direction() {
    let result = compile_rules(
        r#""starting_vertex_position": "",
           "coordinate_system": "World""#,
    );

    assert!(result.has_errors());
    assert!(result.model.is_none());
    let missing_fields = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error
                && diagnostic.code == "MissingRequiredField"
        })
        .filter_map(|diagnostic| diagnostic.field.as_deref())
        .collect::<Vec<_>>();
    assert_eq!(
        missing_fields,
        vec!["starting_vertex_position", "vertex_entry_direction"]
    );
}

#[test]
fn warns_and_uses_world_for_missing_or_blank_required_coordinate_system() {
    for coordinate_field in ["", r#","coordinate_system": """#] {
        let fields = format!(
            r#""starting_vertex_position": "UpperLeftCorner",
               "vertex_entry_direction": "Counterclockwise"
               {coordinate_field}"#
        );
        let result = compile_rules(&fields);

        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Warning
                && diagnostic.code == "MissingRequiredFieldDefaulted"
                && diagnostic.field.as_deref() == Some("coordinate_system")
        }));
        assert!(result.report.defaults_applied.iter().any(|default| {
            default.object_type == "GlobalGeometryRules"
                && default.object_name == "Rules"
                && default.field == "coordinate_system"
                && default.value == "World"
        }));
        assert_eq!(
            result
                .model
                .expect("coordinate fallback should retain the model")
                .global_geometry_rules
                .expect("rules should be typed")
                .coordinate_system,
            GeometryCoordinateSystem::World
        );
    }
}

#[test]
fn warns_and_falls_back_for_invalid_coordinate_systems() {
    let result = compile_rules(
        r#""starting_vertex_position": "UpperLeftCorner",
           "vertex_entry_direction": "Counterclockwise",
           "coordinate_system": "Planet",
           "daylighting_reference_point_coordinate_system": "Building",
           "rectangular_surface_coordinate_system": "Zone""#,
    );

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let warnings = result
        .report
        .diagnostics
        .iter()
        .filter(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Warning
                && diagnostic.code == "InvalidEnumValueDefaulted"
        })
        .collect::<Vec<_>>();
    assert_eq!(warnings.len(), 3);
    let rules = result
        .model
        .expect("warning fallbacks should retain the model")
        .global_geometry_rules
        .expect("rules should be typed");
    assert_eq!(rules.coordinate_system, GeometryCoordinateSystem::World);
    assert_eq!(
        rules.daylighting_reference_point_coordinate_system,
        GeometryCoordinateSystem::Relative
    );
    assert_eq!(
        rules.rectangular_surface_coordinate_system,
        GeometryCoordinateSystem::Relative
    );
}

#[test]
fn rejects_duplicate_global_geometry_rules_singleton() {
    let raw_model = parse_epjson_str(
        r#"{
            "GlobalGeometryRules": {
                "First": {
                    "starting_vertex_position": "UpperLeftCorner",
                    "vertex_entry_direction": "Counterclockwise",
                    "coordinate_system": "World"
                },
                "Second": {
                    "starting_vertex_position": "LowerLeftCorner",
                    "vertex_entry_direction": "Clockwise",
                    "coordinate_system": "Relative"
                }
            }
        }"#,
    )
    .expect("duplicate singleton epJSON should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.model.is_none());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == "TooManyObjects"
            && diagnostic.object_type == "GlobalGeometryRules"
            && diagnostic.object_name.is_none()
            && diagnostic.field.is_none()
    }));
}
