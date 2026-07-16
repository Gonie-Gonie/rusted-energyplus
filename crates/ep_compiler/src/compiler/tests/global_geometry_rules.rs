use super::super::{
    CompileResult, DiagnosticSeverity, ObjectCoverageStatus, canonical_world_surface_vertices,
    compile_raw_model, typed_coverage_status,
};
use ep_model::{
    GeometryCoordinateSystem, GlobalGeometryRules, Point3, StartingVertexPosition,
    VertexEntryDirection,
};
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

fn point(x_m: f64, y_m: f64, z_m: f64) -> Point3 {
    Point3 { x_m, y_m, z_m }
}

fn assert_point_close(actual: Point3, expected: Point3, tolerance: f64) {
    assert!(
        (actual.x_m - expected.x_m).abs() <= tolerance,
        "x mismatch: actual {}, expected {}",
        actual.x_m,
        expected.x_m
    );
    assert!(
        (actual.y_m - expected.y_m).abs() <= tolerance,
        "y mismatch: actual {}, expected {}",
        actual.y_m,
        expected.y_m
    );
    assert!(
        (actual.z_m - expected.z_m).abs() <= tolerance,
        "z mismatch: actual {}, expected {}",
        actual.z_m,
        expected.z_m
    );
}

fn compile_surface_fixture(global_geometry_rules_object: &str) -> CompileResult {
    let epjson = format!(
        r#"{{
            {global_geometry_rules_object}
            "Building": {{
                "Building": {{"north_axis": 180}}
            }},
            "Material:NoMass": {{
                "R13": {{"roughness": "Rough", "thermal_resistance": 1.0}}
            }},
            "Construction": {{
                "Wall Construction": {{"outside_layer": "R13"}}
            }},
            "Zone": {{
                "Zone One": {{
                    "direction_of_relative_north": 90,
                    "x_origin": 10,
                    "y_origin": 20,
                    "z_origin": 30
                }}
            }},
            "BuildingSurface:Detailed": {{
                "Wall One": {{
                    "surface_type": "Wall",
                    "construction_name": "Wall Construction",
                    "zone_name": "Zone One",
                    "outside_boundary_condition": "Outdoors",
                    "vertices": [
                        {{"vertex_x_coordinate": 2, "vertex_y_coordinate": 3, "vertex_z_coordinate": 4}},
                        {{"vertex_x_coordinate": 2, "vertex_y_coordinate": 2, "vertex_z_coordinate": 4}},
                        {{"vertex_x_coordinate": 3, "vertex_y_coordinate": 2, "vertex_z_coordinate": 4}}
                    ]
                }}
            }}
        }}"#
    );
    let raw_model = parse_epjson_str(&epjson).expect("surface fixture epJSON should parse");
    compile_raw_model(&raw_model)
}

#[test]
fn canonicalizes_all_starting_corners_for_ccw_and_cw_input() {
    let upper_left = point(0.0, 1.0, 0.0);
    let lower_left = point(0.0, 0.0, 0.0);
    let lower_right = point(1.0, 0.0, 0.0);
    let upper_right = point(1.0, 1.0, 0.0);
    let canonical = vec![upper_left, lower_left, lower_right, upper_right];
    let cases = [
        (
            StartingVertexPosition::UpperLeftCorner,
            vec![upper_left, lower_left, lower_right, upper_right],
            vec![upper_left, upper_right, lower_right, lower_left],
        ),
        (
            StartingVertexPosition::LowerLeftCorner,
            vec![lower_left, lower_right, upper_right, upper_left],
            vec![lower_left, upper_left, upper_right, lower_right],
        ),
        (
            StartingVertexPosition::LowerRightCorner,
            vec![lower_right, upper_right, upper_left, lower_left],
            vec![lower_right, lower_left, upper_left, upper_right],
        ),
        (
            StartingVertexPosition::UpperRightCorner,
            vec![upper_right, upper_left, lower_left, lower_right],
            vec![upper_right, lower_right, lower_left, upper_left],
        ),
    ];

    for (starting_vertex_position, ccw_vertices, cw_vertices) in cases {
        for (vertex_entry_direction, vertices) in [
            (VertexEntryDirection::CounterClockwise, ccw_vertices),
            (VertexEntryDirection::Clockwise, cw_vertices),
        ] {
            let actual = canonical_world_surface_vertices(
                vertices,
                GlobalGeometryRules {
                    starting_vertex_position,
                    vertex_entry_direction,
                    ..GlobalGeometryRules::default()
                },
                0.0,
                point(0.0, 0.0, 0.0),
                0.0,
            );
            assert_eq!(actual, canonical);
        }
    }
}

#[test]
fn relative_zone_rotation_matches_official_45_and_90_degree_sign_vectors() {
    let rules = GlobalGeometryRules {
        coordinate_system: GeometryCoordinateSystem::Relative,
        ..GlobalGeometryRules::default()
    };
    let input = point(2.048, 3.048, 0.9);

    let rotated_45 =
        canonical_world_surface_vertices(vec![input], rules, 45.0, point(0.0, 0.0, 0.0), 0.0);
    assert_point_close(rotated_45[0], point(3.603, 0.707, 0.9), 5.0e-4);

    let rotated_90 =
        canonical_world_surface_vertices(vec![input], rules, 90.0, point(0.0, 0.0, 0.0), 0.0);
    assert_point_close(rotated_90[0], point(3.048, -2.048, 0.9), 1.0e-12);
}

#[test]
fn relative_transform_applies_zone_rotation_origin_then_building_rotation() {
    let rules = GlobalGeometryRules {
        coordinate_system: GeometryCoordinateSystem::Relative,
        ..GlobalGeometryRules::default()
    };
    let actual = canonical_world_surface_vertices(
        vec![point(2.0, 3.0, 4.0)],
        rules,
        90.0,
        point(10.0, 20.0, 30.0),
        180.0,
    );

    assert_point_close(actual[0], point(-13.0, -18.0, 34.0), 1.0e-12);
}

#[test]
fn building_rotation_uses_direct_negative_angle_for_negative_and_over_360_values() {
    let rules = GlobalGeometryRules {
        coordinate_system: GeometryCoordinateSystem::Relative,
        ..GlobalGeometryRules::default()
    };
    let input = point(2.0, 3.0, 4.0);

    for building_north_axis_deg in [-405.0_f64, 765.0_f64] {
        let angle_rad = (-building_north_axis_deg).to_radians();
        let expected = point(
            input.x_m * angle_rad.cos() - input.y_m * angle_rad.sin(),
            input.x_m * angle_rad.sin() + input.y_m * angle_rad.cos(),
            input.z_m,
        );
        let actual = canonical_world_surface_vertices(
            vec![input],
            rules,
            0.0,
            point(0.0, 0.0, 0.0),
            building_north_axis_deg,
        );

        assert_eq!(actual[0], expected);
    }
}

#[test]
fn world_coordinates_ignore_nonzero_building_and_zone_transforms() {
    let input = vec![
        point(2.0, 3.0, 4.0),
        point(-5.0, 7.0, 11.0),
        point(13.0, -17.0, 19.0),
    ];
    let actual = canonical_world_surface_vertices(
        input.clone(),
        GlobalGeometryRules::default(),
        91.0,
        point(10.0, 20.0, 30.0),
        -37.0,
    );

    assert_eq!(actual, input);
}

#[test]
fn compiler_stores_relative_vertices_in_canonical_world_coordinates() {
    let result = compile_surface_fixture(
        r#""GlobalGeometryRules": {
            "Rules": {
                "starting_vertex_position": "UpperLeftCorner",
                "vertex_entry_direction": "Counterclockwise",
                "coordinate_system": "Relative"
            }
        },"#,
    );

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let first_vertex = result
        .model
        .expect("relative surface fixture should compile")
        .surfaces[0]
        .vertices[0];
    assert_point_close(first_vertex, point(-13.0, -18.0, 34.0), 1.0e-12);
}

#[test]
fn compiler_missing_rules_preserves_legacy_world_vertices() {
    let result = compile_surface_fixture("");

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .expect("surface fixture without geometry rules should compile");
    assert!(model.global_geometry_rules.is_none());
    assert_eq!(model.surfaces[0].vertices[0], point(2.0, 3.0, 4.0));
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
