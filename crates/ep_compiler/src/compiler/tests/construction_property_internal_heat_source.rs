use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    Construction, ConstructionId, ConstructionInternalHeatSourceDimensions, ConstructionKind,
    ModelGraph, NormalizedName, TypedModel,
};
use ep_raw_model::{
    FieldName, ObjectName, ObjectType, RawModel, RawValue, parse_epjson_str,
    parse_epjson_str_with_idf_order,
};

const OBJECT_TYPE: &str = "ConstructionProperty:InternalHeatSource";
const CONSTRUCTION_FIELD: &str = "construction_name";
const SOURCE_LAYER_FIELD: &str = "thermal_source_present_after_layer_number";
const TEMPERATURE_LAYER_FIELD: &str = "temperature_calculation_requested_after_layer_number";
const DIMENSIONS_FIELD: &str = "dimensions_for_the_ctf_calculation";
const TUBE_SPACING_FIELD: &str = "tube_spacing";
const TEMPERATURE_POSITION_FIELD: &str = "two_dimensional_temperature_calculation_position";

fn two_layer_model(source_objects: &str) -> String {
    format!(
        r#"{{
            "Material:NoMass": {{
                "Outside Layer": {{"roughness":"Rough", "thermal_resistance":1.0}},
                "Inside Layer": {{"roughness":"Rough", "thermal_resistance":0.5}}
            }},
            "Construction": {{
                "Wall": {{"outside_layer":"Outside Layer", "layer_2":"Inside Layer"}}
            }},
            "{OBJECT_TYPE}": {{{source_objects}}}
        }}"#
    )
}

fn valid_source_fields(construction_name: &str) -> String {
    format!(
        r#""construction_name":"{construction_name}",
            "thermal_source_present_after_layer_number":1,
            "temperature_calculation_requested_after_layer_number":1,
            "dimensions_for_the_ctf_calculation":1,
            "tube_spacing":0.20"#
    )
}

fn source_object_mut<'a>(
    raw: &'a mut RawModel,
    name: &str,
) -> Result<&'a mut ep_raw_model::RawObject, Box<dyn std::error::Error>> {
    raw.objects
        .get_mut(&ObjectType(OBJECT_TYPE.to_string()))
        .and_then(|instances| instances.get_mut(&ObjectName(name.to_string())))
        .ok_or_else(|| std::io::Error::other("missing raw internal heat-source object").into())
}

fn has_error(result: &CompileResult, code: &str, object_name: &str, field: Option<&str>) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

fn compiler_has_error(
    compiler: &Compiler<'_>,
    code: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

fn parse_through_internal_heat_sources(compiler: &mut Compiler<'_>, model: &mut TypedModel) {
    compiler.parse_materials(model);
    compiler.parse_constructions(model);
    compiler.parse_air_boundary_constructions(model);
    compiler.parse_complex_fenestration_states(model);
    compiler.parse_construction_internal_heat_sources(model);
}

#[test]
fn internal_heat_sources_materialize_source_effective_state_after_complex_fenestration()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Outside Layer": {"roughness":"Rough", "thermal_resistance":1.0},
                "Inside Layer": {"roughness":"Rough", "thermal_resistance":0.5}
            },
            "Construction": {
                "One Dimensional Wall": {
                    "outside_layer":"Outside Layer", "layer_2":"Inside Layer"
                },
                "Two Dimensional Wall": {
                    "outside_layer":"Inside Layer", "layer_2":"Outside Layer"
                }
            },
            "ConstructionProperty:InternalHeatSource": {
                "One Dimensional Source": {
                    "construction_name":"one dimensional wall",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":1,
                    "tube_spacing":0.20
                },
                "Two Dimensional Source": {
                    "construction_name":"TWO DIMENSIONAL WALL",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":2,
                    "tube_spacing":0.30,
                    "two_dimensional_temperature_calculation_position":0.75
                }
            },
            "Zone": {"Zone One": {}},
            "BuildingSurface:Detailed": {
                "One Dimensional Surface": {
                    "surface_type":"Wall",
                    "construction_name":"One Dimensional Wall",
                    "zone_name":"Zone One",
                    "outside_boundary_condition":"Outdoors",
                    "vertices":[
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":0,"vertex_z_coordinate":1},
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":0,"vertex_z_coordinate":1}
                    ]
                },
                "Two Dimensional Surface": {
                    "surface_type":"Wall",
                    "construction_name":"Two Dimensional Wall",
                    "zone_name":"Zone One",
                    "outside_boundary_condition":"Outdoors",
                    "vertices":[
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":1,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":0},
                        {"vertex_x_coordinate":1,"vertex_y_coordinate":1,"vertex_z_coordinate":1},
                        {"vertex_x_coordinate":0,"vertex_y_coordinate":1,"vertex_z_coordinate":1}
                    ]
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
        .ok_or_else(|| std::io::Error::other("expected typed internal heat sources"))?;
    let one_id = model
        .construction_names
        .resolve("One Dimensional Wall")
        .ok_or_else(|| std::io::Error::other("missing 1-D construction"))?;
    let two_id = model
        .construction_names
        .resolve("Two Dimensional Wall")
        .ok_or_else(|| std::io::Error::other("missing 2-D construction"))?;
    let one = model.constructions[one_id.0 as usize]
        .internal_heat_source
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing 1-D source metadata"))?;
    assert_eq!(one.name, NormalizedName::new("One Dimensional Source"));
    assert_eq!(one.source_after_layer, 1);
    assert_eq!(one.temperature_after_layer, 1);
    assert_eq!(
        one.ctf_dimensions,
        ConstructionInternalHeatSourceDimensions::OneDimensional
    );
    assert_eq!(one.tube_spacing_m, 0.20);
    assert_eq!(one.half_tube_spacing_m, 0.10);
    assert_eq!(one.temperature_location_perpendicular, 0.0);

    let two = model.constructions[two_id.0 as usize]
        .internal_heat_source
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing 2-D source metadata"))?;
    assert_eq!(two.name, NormalizedName::new("Two Dimensional Source"));
    assert_eq!(
        two.ctf_dimensions,
        ConstructionInternalHeatSourceDimensions::TwoDimensional
    );
    assert_eq!(two.tube_spacing_m, 0.30);
    assert_eq!(two.half_tube_spacing_m, 0.15);
    assert_eq!(two.temperature_location_perpendicular, 0.75);

    assert_eq!(model.surfaces.len(), 2);
    assert_eq!(model.surfaces[0].construction, one_id);
    assert_eq!(model.surfaces[1].construction, two_id);
    let graph = ModelGraph::from_typed(model);
    assert_eq!(graph.zone_surfaces.len(), 2);
    assert_eq!(graph.construction_materials.len(), 4);
    for construction_id in [one_id, two_id] {
        assert_eq!(
            graph
                .construction_materials
                .iter()
                .filter(|edge| edge.construction == construction_id)
                .count(),
            2
        );
    }
    assert_eq!(model.object_count(), 10);
    assert_eq!(result.report.typed_object_count, model.object_count());
    Ok(())
}

#[test]
fn internal_heat_source_accepts_independent_layer_positions_and_inclusive_numeric_bounds()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Layer A": {"roughness":"Rough", "thermal_resistance":1.0},
                "Layer B": {"roughness":"Rough", "thermal_resistance":0.5}
            },
            "Construction": {
                "Low Bound": {
                    "outside_layer":"Layer A",
                    "layer_2":"Layer B",
                    "layer_3":"Layer A",
                    "layer_4":"Layer B"
                },
                "High Bound": {
                    "outside_layer":"Layer B",
                    "layer_2":"Layer A",
                    "layer_3":"Layer B",
                    "layer_4":"Layer A"
                }
            },
            "ConstructionProperty:InternalHeatSource": {
                "Low Bound Source": {
                    "construction_name":"Low Bound",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":3,
                    "dimensions_for_the_ctf_calculation":2,
                    "tube_spacing":0.01,
                    "two_dimensional_temperature_calculation_position":0.0
                },
                "High Bound Source": {
                    "construction_name":"High Bound",
                    "thermal_source_present_after_layer_number":3,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":2,
                    "tube_spacing":1.0,
                    "two_dimensional_temperature_calculation_position":1.0
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected bounded internal heat sources"))?;

    for (name, source_layer, temperature_layer, spacing, half_spacing, position) in [
        ("Low Bound", 1, 3, 0.01, 0.005, 0.0),
        ("High Bound", 3, 1, 1.0, 0.5, 1.0),
    ] {
        let id = model
            .construction_names
            .resolve(name)
            .ok_or_else(|| std::io::Error::other(format!("missing {name}")))?;
        let source = model.constructions[id.0 as usize]
            .internal_heat_source
            .as_ref()
            .ok_or_else(|| std::io::Error::other(format!("missing source for {name}")))?;
        assert_eq!(source.source_after_layer, source_layer);
        assert_eq!(source.temperature_after_layer, temperature_layer);
        assert_eq!(source.tube_spacing_m, spacing);
        assert_eq!(source.half_tube_spacing_m, half_spacing);
        assert_eq!(source.temperature_location_perpendicular, position);
    }
    Ok(())
}

#[test]
fn absent_internal_heat_source_idf_overlay_falls_back_to_lexical_first_target_wins()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = two_layer_model(&format!(
        r#""Zulu Second": {{
                "construction_name":"Wall",
                "thermal_source_present_after_layer_number":1,
                "temperature_calculation_requested_after_layer_number":1,
                "dimensions_for_the_ctf_calculation":2,
                "tube_spacing":0.40
            }},
            "Alpha First": {{{}}}"#,
        valid_source_fields("Wall")
    ));
    let idf = r#"
        Material:NoMass, Outside Layer, Rough, 1.0;
        Material:NoMass, Inside Layer, Rough, 0.5;
        Construction, Wall, Outside Layer, Inside Layer;
    "#;
    let raw = parse_epjson_str_with_idf_order(&epjson, idf)?;
    assert!(!raw.has_idf_declaration_order(OBJECT_TYPE));

    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    parse_through_internal_heat_sources(&mut compiler, &mut model);
    assert!(compiler_has_error(
        &compiler,
        "DuplicateInternalHeatSourceConstruction",
        "Zulu Second",
        Some(CONSTRUCTION_FIELD)
    ));
    let metadata = model.constructions[0]
        .internal_heat_source
        .as_ref()
        .ok_or_else(|| std::io::Error::other("lexical-first source did not materialize"))?;
    assert_eq!(metadata.name, NormalizedName::new("Alpha First"));
    assert_eq!(metadata.tube_spacing_m, 0.20);
    assert_eq!(
        metadata.ctf_dimensions,
        ConstructionInternalHeatSourceDimensions::OneDimensional
    );
    Ok(())
}

#[test]
fn internal_heat_source_strict_field_and_layer_validation_matrix()
-> Result<(), Box<dyn std::error::Error>> {
    let base = parse_epjson_str(&two_layer_model(&format!(
        r#""Source": {{{}}}"#,
        valid_source_fields("Wall")
    )))?;
    enum Mutation {
        Remove(&'static str),
        Set(&'static str, RawValue),
    }
    let cases = [
        (
            Mutation::Remove(CONSTRUCTION_FIELD),
            "MissingRequiredField",
            CONSTRUCTION_FIELD,
        ),
        (
            Mutation::Remove(SOURCE_LAYER_FIELD),
            "MissingRequiredField",
            SOURCE_LAYER_FIELD,
        ),
        (
            Mutation::Remove(TEMPERATURE_LAYER_FIELD),
            "MissingRequiredField",
            TEMPERATURE_LAYER_FIELD,
        ),
        (
            Mutation::Remove(DIMENSIONS_FIELD),
            "MissingRequiredField",
            DIMENSIONS_FIELD,
        ),
        (
            Mutation::Remove(TUBE_SPACING_FIELD),
            "MissingRequiredField",
            TUBE_SPACING_FIELD,
        ),
        (
            Mutation::Set(CONSTRUCTION_FIELD, RawValue::String("Unknown".to_string())),
            "MissingReference",
            CONSTRUCTION_FIELD,
        ),
        (
            Mutation::Set(SOURCE_LAYER_FIELD, RawValue::Number("0".to_string())),
            "InvalidNumericRange",
            SOURCE_LAYER_FIELD,
        ),
        (
            Mutation::Set(SOURCE_LAYER_FIELD, RawValue::Number("2".to_string())),
            "InvalidInternalHeatSourceLayerPosition",
            SOURCE_LAYER_FIELD,
        ),
        (
            Mutation::Set(TEMPERATURE_LAYER_FIELD, RawValue::Number("0".to_string())),
            "InvalidNumericRange",
            TEMPERATURE_LAYER_FIELD,
        ),
        (
            Mutation::Set(TEMPERATURE_LAYER_FIELD, RawValue::Number("2".to_string())),
            "InvalidInternalHeatSourceLayerPosition",
            TEMPERATURE_LAYER_FIELD,
        ),
        (
            Mutation::Set(DIMENSIONS_FIELD, RawValue::Number("0".to_string())),
            "InvalidNumericRange",
            DIMENSIONS_FIELD,
        ),
        (
            Mutation::Set(DIMENSIONS_FIELD, RawValue::Number("3".to_string())),
            "InvalidInternalHeatSourceDimensions",
            DIMENSIONS_FIELD,
        ),
        (
            Mutation::Set(DIMENSIONS_FIELD, RawValue::Number("1.5".to_string())),
            "InvalidInteger",
            DIMENSIONS_FIELD,
        ),
        (
            Mutation::Set(TUBE_SPACING_FIELD, RawValue::Number("0.009".to_string())),
            "InvalidNumericRange",
            TUBE_SPACING_FIELD,
        ),
        (
            Mutation::Set(TUBE_SPACING_FIELD, RawValue::Number("1.001".to_string())),
            "InvalidNumericRange",
            TUBE_SPACING_FIELD,
        ),
        (
            Mutation::Set(TUBE_SPACING_FIELD, RawValue::Number("NaN".to_string())),
            "InvalidNumber",
            TUBE_SPACING_FIELD,
        ),
        (
            Mutation::Set(
                TEMPERATURE_POSITION_FIELD,
                RawValue::Number("-0.01".to_string()),
            ),
            "InvalidNumericRange",
            TEMPERATURE_POSITION_FIELD,
        ),
        (
            Mutation::Set(
                TEMPERATURE_POSITION_FIELD,
                RawValue::Number("1.01".to_string()),
            ),
            "InvalidNumericRange",
            TEMPERATURE_POSITION_FIELD,
        ),
    ];

    for (mutation, code, field) in cases {
        let mut raw = base.clone();
        let fields = &mut source_object_mut(&mut raw, "Source")?.fields;
        match mutation {
            Mutation::Remove(field) => {
                fields.remove(&FieldName(field.to_string()));
            }
            Mutation::Set(field, value) => {
                fields.insert(FieldName(field.to_string()), value);
            }
        }
        let result = compile_raw_model(&raw);
        assert!(
            has_error(&result, code, "Source", Some(field)),
            "code={code}, field={field}, diagnostics={:?}",
            result.report.diagnostics
        );
        assert!(result.model.is_none());
    }

    let blank_name = parse_epjson_str(&two_layer_model(&format!(
        "\"\": {{{}}}",
        valid_source_fields("Wall")
    )))?;
    let result = compile_raw_model(&blank_name);
    assert!(has_error(&result, "MissingRequiredField", "", Some("name")));
    assert!(result.model.is_none());

    let mut blank_position = base;
    source_object_mut(&mut blank_position, "Source")?
        .fields
        .insert(
            FieldName(TEMPERATURE_POSITION_FIELD.to_string()),
            RawValue::String("   ".to_string()),
        );
    let result = compile_raw_model(&blank_position);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        result
            .model
            .as_ref()
            .ok_or_else(|| std::io::Error::other("expected blank-position model"))?
            .constructions[0]
            .internal_heat_source
            .as_ref()
            .ok_or_else(|| std::io::Error::other("missing blank-position metadata"))?
            .temperature_location_perpendicular,
        0.0
    );
    Ok(())
}

#[test]
fn internal_heat_source_targets_are_bounded_to_multilayer_ordinary_opaque_constructions()
-> Result<(), Box<dyn std::error::Error>> {
    let source =
        |name: &str, target: &str| format!(r#""{name}": {{{}}}"#, valid_source_fields(target));
    let raw = parse_epjson_str(&format!(
        r#"{{
            "Material:NoMass": {{
                "Opaque": {{"roughness":"Rough", "thermal_resistance":1.0}}
            }},
            "WindowMaterial:SimpleGlazingSystem": {{
                "Simple Glass": {{"u_factor":2.0, "solar_heat_gain_coefficient":0.5}}
            }},
            "Construction": {{
                "One Layer Opaque": {{"outside_layer":"Opaque"}},
                "Fenestration": {{"outside_layer":"Simple Glass"}}
            }},
            "Construction:FfactorGroundFloor": {{
                "F Ground": {{"f_factor":0.5, "area":100.0, "perimeterexposed":20.0}}
            }},
            "Construction:CfactorUndergroundWall": {{
                "C Ground": {{"c_factor":0.5, "height":1.0}}
            }},
            "Construction:AirBoundary": {{"Air Boundary": {{}}}},
            "{OBJECT_TYPE}": {{
                {}, {}, {}, {}, {}
            }}
        }}"#,
        source("One Layer Source", "One Layer Opaque"),
        source("Fenestration Source", "Fenestration"),
        source("F Source", "F Ground"),
        source("C Source", "C Ground"),
        source("Air Source", "Air Boundary"),
    ))?;
    let result = compile_raw_model(&raw);
    for name in [
        "One Layer Source",
        "Fenestration Source",
        "F Source",
        "C Source",
        "Air Source",
    ] {
        assert!(
            has_error(
                &result,
                "InvalidInternalHeatSourceConstruction",
                name,
                Some(CONSTRUCTION_FIELD)
            ),
            "name={name}, diagnostics={:?}",
            result.report.diagnostics
        );
    }
    assert!(result.model.is_none());

    let cfs_raw = parse_epjson_str(&format!(
        r#"{{"{OBJECT_TYPE}": {{"CFS Source": {{{}}}}}}}"#,
        valid_source_fields("CFS")
    ))?;
    let mut compiler = Compiler::new(&cfs_raw, None);
    let mut model = TypedModel::default();
    model.construction_names.insert("CFS", ConstructionId(0));
    model.constructions.push(Construction {
        id: ConstructionId(0),
        name: NormalizedName::new("CFS"),
        kind: ConstructionKind::ComplexFenestration,
        outside_layer: None,
        layers: Vec::new(),
        thermochromic_master: None,
        ground_factor: None,
        air_boundary: None,
        complex_fenestration: None,
        window_equivalent_layer: None,
        internal_heat_source: None,
    });
    compiler.parse_construction_internal_heat_sources(&mut model);
    assert!(compiler_has_error(
        &compiler,
        "InvalidInternalHeatSourceConstruction",
        "CFS Source",
        Some(CONSTRUCTION_FIELD)
    ));
    assert!(!model.constructions[0].has_internal_heat_source());
    Ok(())
}

#[test]
fn invalid_first_does_not_reserve_target_and_case_colliding_outer_names_can_target_distinct_constructions()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material:NoMass": {
                "Outside": {"roughness":"Rough", "thermal_resistance":1.0},
                "Inside": {"roughness":"Rough", "thermal_resistance":0.5}
            },
            "Construction": {
                "Recover": {"outside_layer":"Outside", "layer_2":"Inside"},
                "Case Target A": {"outside_layer":"Outside", "layer_2":"Inside"},
                "Case Target B": {"outside_layer":"Inside", "layer_2":"Outside"}
            },
            "ConstructionProperty:InternalHeatSource": {
                "A Invalid": {
                    "construction_name":"Recover",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":1,
                    "tube_spacing":0.001
                },
                "B Valid": {
                    "construction_name":"RECOVER",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":2,
                    "tube_spacing":0.20
                },
                "Source": {
                    "construction_name":"Case Target A",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":1,
                    "tube_spacing":0.30
                },
                "source": {
                    "construction_name":"Case Target B",
                    "thermal_source_present_after_layer_number":1,
                    "temperature_calculation_requested_after_layer_number":1,
                    "dimensions_for_the_ctf_calculation":1,
                    "tube_spacing":0.40
                }
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    parse_through_internal_heat_sources(&mut compiler, &mut model);
    assert!(compiler_has_error(
        &compiler,
        "InvalidNumericRange",
        "A Invalid",
        Some(TUBE_SPACING_FIELD)
    ));
    assert!(
        compiler
            .diagnostics
            .iter()
            .all(|diagnostic| diagnostic.code != "DuplicateInternalHeatSourceConstruction")
    );

    let recover = model.construction_names.resolve("Recover").unwrap();
    let case_a = model.construction_names.resolve("Case Target A").unwrap();
    let case_b = model.construction_names.resolve("Case Target B").unwrap();
    assert_eq!(
        model.constructions[recover.0 as usize]
            .internal_heat_source
            .as_ref()
            .unwrap()
            .name,
        NormalizedName::new("B Valid")
    );
    let source_a = model.constructions[case_a.0 as usize]
        .internal_heat_source
        .as_ref()
        .unwrap();
    let source_b = model.constructions[case_b.0 as usize]
        .internal_heat_source
        .as_ref()
        .unwrap();
    assert_eq!(source_a.name, NormalizedName::new("Source"));
    assert_eq!(source_b.name, NormalizedName::new("source"));
    assert_eq!(source_a.name, source_b.name);
    assert_eq!(source_a.tube_spacing_m, 0.30);
    assert_eq!(source_b.tube_spacing_m, 0.40);
    Ok(())
}
