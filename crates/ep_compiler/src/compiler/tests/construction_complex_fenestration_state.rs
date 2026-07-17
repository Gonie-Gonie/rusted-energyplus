use super::super::{
    Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model, typed_coverage_status,
};
use ep_model::{
    ComplexFenestrationBasisSymmetry, ComplexFenestrationBasisType, ConstructionId,
    ConstructionKind, ModelGraph, TypedModel, WindowThermalCalculationModel,
    WindowThermalCalculationStandard, WindowThermalDeflectionModel,
};
use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

const OBJECT_TYPE: &str = "Construction:ComplexFenestrationState";

fn positive_epjson() -> String {
    r#"{
        "Material:NoMass": {
            "Opaque Layer": {"roughness":"Rough", "thermal_resistance":1.0}
        },
        "WindowMaterial:Glazing": {
            "Glass Outside": {"optical_data_type":"SpectralAverage", "thickness":0.003}
        },
        "WindowMaterial:Gas": {
            "Gap Gas": {"gas_type":"Air", "thickness":0.012}
        },
        "WindowMaterial:Gap": {
            "Complex Gap": {"thickness":0.012, "gas_or_gas_mixture_":"Gap Gas"}
        },
        "WindowMaterial:ComplexShade": {
            "Complex Shade": {}
        },
        "Construction": {
            "Ordinary": {"outside_layer":"Opaque Layer"}
        },
        "Construction:FfactorGroundFloor": {
            "F Ground": {"f_factor":0.5, "area":100.0, "perimeterexposed":20.0}
        },
        "Construction:CfactorUndergroundWall": {
            "C Ground": {"c_factor":0.5, "height":1.0}
        },
        "Construction:AirBoundary": {
            "Air Boundary": {"air_exchange_method":"None"}
        },
        "WindowThermalModel:Params": {
            "Thermal Defaults": {},
            "Thermal Explicit": {
                "standard":"EN673Design",
                "thermal_model":"ConvectiveScalarModel_withSDThickness",
                "sdscalar":0.25,
                "deflection_model":"TemperatureAndPressureInput",
                "vacuum_pressure_limit":12.0,
                "initial_temperature":20.0,
                "initial_pressure":90000.0
            }
        },
        "Matrix:TwoDimension": {
            "Basis": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.0},{"value":99.0}]},
            "Solar Front": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.4}]},
            "Solar Back": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":-0.2}]},
            "Visible Front": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":1.2}]},
            "Visible Back": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.3}]},
            "Abs Front": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.1}]},
            "Abs Back": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.2}]}
        },
        "Construction:ComplexFenestrationState": {
            "Zulu Single": {
                "basis_type":"LBNLWINDOW",
                "basis_symmetry_type":"None",
                "window_thermal_model":"Thermal Defaults",
                "basis_matrix_name":"Basis",
                "solar_optical_complex_front_transmittance_matrix_name":"Solar Front",
                "solar_optical_complex_back_reflectance_matrix_name":"Solar Back",
                "visible_optical_complex_front_transmittance_matrix_name":"Visible Front",
                "visible_optical_complex_back_transmittance_matrix_name":"Visible Back",
                "outside_layer_name":"Glass Outside",
                "outside_layer_directional_front_absorptance_matrix_name":"Abs Front",
                "outside_layer_directional_back_absorptance_matrix_name":"Abs Back"
            },
            "Alpha Triple": {
                "basis_type":"LBNLWINDOW",
                "basis_symmetry_type":"None",
                "window_thermal_model":"thermal explicit",
                "basis_matrix_name":"Basis",
                "solar_optical_complex_front_transmittance_matrix_name":"Solar Front",
                "solar_optical_complex_back_reflectance_matrix_name":"Solar Back",
                "visible_optical_complex_front_transmittance_matrix_name":"Visible Front",
                "visible_optical_complex_back_transmittance_matrix_name":"Visible Back",
                "outside_layer_name":"Glass Outside",
                "outside_layer_directional_front_absorptance_matrix_name":"Abs Front",
                "outside_layer_directional_back_absorptance_matrix_name":"Abs Back",
                "gap_1_name":"Complex Gap",
                "cfs_gap_1_directional_front_absorptance_matrix_name":"",
                "cfs_gap_1_directional_back_absorptance_matrix_name":"",
                "layer_2_name":"Complex Shade",
                "layer_2_directional_front_absorptance_matrix_name":"Abs Front",
                "layer_2_directional_back_absorptance_matrix_name":"Abs Back"
            }
        }
    }"#
    .to_string()
}

fn has_error(
    diagnostics: &[super::super::ModelDiagnostic],
    code: &str,
    object_type: &str,
    object_name: Option<&str>,
    field: Option<&str>,
) -> bool {
    diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == object_type
            && diagnostic.object_name.as_deref() == object_name
            && diagnostic.field.as_deref() == field
    })
}

#[test]
fn complex_fenestration_states_materialize_after_air_with_ordered_layers_and_dependency_identities()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(&positive_epjson())?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert_eq!(
        typed_coverage_status(OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert_eq!(
        typed_coverage_status("WindowThermalModel:Params"),
        ObjectCoverageStatus::RawOnly
    );
    assert_eq!(
        typed_coverage_status("Matrix:TwoDimension"),
        ObjectCoverageStatus::RawOnly
    );

    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed complex states"))?;
    assert_eq!(
        model
            .constructions
            .iter()
            .map(|construction| construction.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "ORDINARY",
            "F GROUND",
            "C GROUND",
            "AIR BOUNDARY",
            "ALPHA TRIPLE",
            "ZULU SINGLE",
        ]
    );

    let triple = &model.constructions[4];
    assert_eq!(triple.id, ConstructionId(4));
    assert_eq!(triple.kind, ConstructionKind::ComplexFenestration);
    assert!(triple.is_complex_fenestration());
    assert!(!triple.is_ordinary_opaque());
    assert!(!triple.is_air_boundary());
    assert_eq!(triple.outside_layer, Some(triple.layers[0]));
    assert_eq!(triple.layers.len(), 3);
    assert_eq!(
        triple
            .layers
            .iter()
            .map(|id| model.materials[id.0 as usize].name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["GLASS OUTSIDE", "COMPLEX GAP", "COMPLEX SHADE"]
    );
    let metadata = triple
        .complex_fenestration
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing CFS metadata"))?;
    assert_eq!(
        metadata.basis_type,
        ComplexFenestrationBasisType::LbnlWindow
    );
    assert_eq!(
        metadata.basis_symmetry,
        ComplexFenestrationBasisSymmetry::None
    );
    assert_eq!(metadata.basis_length, 1);
    assert_eq!(metadata.basis_matrix.values.as_ref(), &[0.0]);
    assert_eq!(metadata.solar_back_reflectance.values.as_ref(), &[-0.2]);
    assert_eq!(metadata.visible_front_transmittance.values.as_ref(), &[1.2]);
    assert_eq!(metadata.optical_layers.len(), 2);
    assert_eq!(
        metadata.thermal_model.standard,
        WindowThermalCalculationStandard::En673Design
    );
    assert_eq!(
        metadata.thermal_model.thermal_model,
        WindowThermalCalculationModel::ConvectiveScalarWithSdThickness
    );
    assert_eq!(metadata.thermal_model.shading_device_scalar, 0.25);
    assert_eq!(
        metadata.thermal_model.deflection_model,
        WindowThermalDeflectionModel::TemperatureAndPressureInput {
            vacuum_pressure_limit_pa: 12.0,
            initial_temperature_c: 20.0,
            initial_pressure_pa: 90_000.0,
        }
    );

    let single = &model.constructions[5];
    let single_metadata = single
        .complex_fenestration
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing default CFS metadata"))?;
    assert_eq!(single.layers.len(), 1);
    assert_eq!(single_metadata.optical_layers.len(), 1);
    assert_eq!(
        single_metadata.thermal_model.standard,
        WindowThermalCalculationStandard::Iso15099
    );
    assert_eq!(
        single_metadata.thermal_model.deflection_model,
        WindowThermalDeflectionModel::NoDeflection
    );

    let graph = ModelGraph::from_typed(&model);
    let triple_edges = graph
        .construction_materials
        .iter()
        .filter(|edge| edge.construction == triple.id)
        .collect::<Vec<_>>();
    assert_eq!(triple_edges.len(), 3);
    assert_eq!(
        triple_edges
            .iter()
            .map(|edge| edge.layer_index)
            .collect::<Vec<_>>(),
        vec![0, 1, 2]
    );
    assert_eq!(
        triple_edges
            .iter()
            .map(|edge| edge.material)
            .collect::<Vec<_>>(),
        triple.layers
    );
    Ok(())
}

#[test]
fn staged_idf_overlay_preserves_complex_state_declaration_order()
-> Result<(), Box<dyn std::error::Error>> {
    let idf = r#"
        Construction, Ordinary;
        Construction:FfactorGroundFloor, F Ground;
        Construction:CfactorUndergroundWall, C Ground;
        Construction:ComplexFenestrationState, Zulu Single;
        Construction:ComplexFenestrationState, Alpha Triple;
    "#;
    let raw = parse_epjson_str_with_idf_order(&positive_epjson(), idf)?;
    assert!(raw.has_idf_declaration_order(OBJECT_TYPE));
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected staged CFS order"))?;
    assert_eq!(model.constructions[4].name.0, "ZULU SINGLE");
    assert_eq!(model.constructions[5].name.0, "ALPHA TRIPLE");
    Ok(())
}

#[test]
fn two_column_lbnl_basis_derives_source_basis_length_and_keeps_effective_matrix_prefix()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass": {"optical_data_type":"SpectralAverage", "thickness":0.003}
            },
            "WindowThermalModel:Params": {"Thermal": {}},
            "Matrix:TwoDimension": {
                "Basis": {
                    "number_of_rows":2, "number_of_columns":2,
                    "values":[
                        {"value":0.0}, {"value":0.0},
                        {"value":45.0}, {"value":2.0}
                    ]
                },
                "Square": {
                    "number_of_rows":3, "number_of_columns":3,
                    "values":[
                        {"value":0.0}, {"value":0.1}, {"value":0.2},
                        {"value":0.3}, {"value":0.4}, {"value":0.5},
                        {"value":0.6}, {"value":0.7}, {"value":0.8},
                        {"value":99.0}
                    ]
                },
                "Abs": {
                    "number_of_rows":1, "number_of_columns":3,
                    "values":[{"value":0.1}, {"value":0.2}, {"value":0.3}, {"value":88.0}]
                }
            },
            "Construction:ComplexFenestrationState": {
                "Three Direction Basis": {
                    "window_thermal_model":"Thermal",
                    "basis_matrix_name":"Basis",
                    "solar_optical_complex_front_transmittance_matrix_name":"Square",
                    "solar_optical_complex_back_reflectance_matrix_name":"Square",
                    "visible_optical_complex_front_transmittance_matrix_name":"Square",
                    "visible_optical_complex_back_transmittance_matrix_name":"Square",
                    "outside_layer_name":"Glass",
                    "outside_layer_directional_front_absorptance_matrix_name":"Abs",
                    "outside_layer_directional_back_absorptance_matrix_name":"Abs"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected two-column basis state"))?;
    let metadata = model.constructions[0]
        .complex_fenestration
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing two-column basis metadata"))?;
    assert_eq!(metadata.basis_length, 3);
    assert_eq!(
        metadata.basis_matrix.values.as_ref(),
        &[0.0, 0.0, 45.0, 2.0]
    );
    assert_eq!(
        metadata.solar_front_transmittance.values.as_ref(),
        &[0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8]
    );
    assert_eq!(
        metadata.optical_layers[0].front_absorptance.values.as_ref(),
        &[0.1, 0.2, 0.3]
    );
    Ok(())
}

#[test]
fn unused_invalid_thermal_and_matrix_helpers_block_every_complex_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = positive_epjson()
        .replace(
            r#""Thermal Defaults": {},"#,
            r#""Thermal Defaults": {}, "Unused Bad Thermal":{"sdscalar":1.1},"#,
        )
        .replace(
            r#""Basis": {"number_of_rows":1, "number_of_columns":1,"#,
            r#""Unused Bad Matrix":{"number_of_rows":0,"number_of_columns":1,"values":[]},
               "basis":{"number_of_rows":1,"number_of_columns":1,"values":[{"value":0.0}]},
               "Basis": {"number_of_rows":1, "number_of_columns":1,"#,
        );
    let raw = parse_epjson_str(&epjson)?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    compiler.parse_air_boundary_constructions(&mut model);
    compiler.parse_complex_fenestration_states(&mut model);

    assert!(has_error(
        &compiler.diagnostics,
        "InvalidNumericRange",
        "WindowThermalModel:Params",
        Some("Unused Bad Thermal"),
        Some("sdscalar")
    ));
    assert!(has_error(
        &compiler.diagnostics,
        "InvalidNumericRange",
        "Matrix:TwoDimension",
        Some("Unused Bad Matrix"),
        Some("number_of_rows")
    ));
    assert!(has_error(
        &compiler.diagnostics,
        "DuplicateName",
        "Matrix:TwoDimension",
        Some("basis"),
        None
    ));
    assert!(
        model
            .constructions
            .iter()
            .all(|construction| !construction.is_complex_fenestration())
    );
    assert_eq!(model.construction_names.resolve("Alpha Triple"), None);
    Ok(())
}

#[test]
fn all_shared_name_collisions_skip_the_lazy_matrix_catalog_but_still_read_thermal_helpers()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = positive_epjson()
        .replace(r#""Zulu Single": {"#, r#""Ordinary": {"#)
        .replace(r#""Alpha Triple": {"#, r#""Air Boundary": {"#)
        .replace(
            r#""Thermal Defaults": {},"#,
            r#""Thermal Defaults": {}, "Unused Bad Thermal":{"sdscalar":1.1},"#,
        )
        .replace(
            r#""Basis": {"number_of_rows":1, "number_of_columns":1,"#,
            r#""Unused Bad Matrix":{"number_of_rows":0,"number_of_columns":1,"values":[]},
               "Basis": {"number_of_rows":1, "number_of_columns":1,"#,
        );
    let raw = parse_epjson_str(&epjson)?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    compiler.parse_air_boundary_constructions(&mut model);
    compiler.parse_complex_fenestration_states(&mut model);

    assert!(has_error(
        &compiler.diagnostics,
        "InvalidNumericRange",
        "WindowThermalModel:Params",
        Some("Unused Bad Thermal"),
        Some("sdscalar")
    ));
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Ordinary")
    }));
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Air Boundary")
    }));
    assert!(!compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.object_type == "Matrix:TwoDimension"
            && diagnostic.object_name.as_deref() == Some("Unused Bad Matrix")
    }));
    assert!(
        model
            .constructions
            .iter()
            .all(|construction| !construction.is_complex_fenestration())
    );
    Ok(())
}

#[test]
fn unsupported_basis_branches_fail_closed_and_matrix_references_follow_input_case_normalization()
-> Result<(), Box<dyn std::error::Error>> {
    for (field, value) in [
        ("basis_type", "UserDefined"),
        ("basis_symmetry_type", "Axisymmetric"),
    ] {
        let old = format!(
            r#""{field}":"{}""#,
            if field == "basis_type" {
                "LBNLWINDOW"
            } else {
                "None"
            }
        );
        let new = format!(r#""{field}":"{value}""#);
        let raw = parse_epjson_str(&positive_epjson().replace(&old, &new))?;
        let result = compile_raw_model(&raw);
        assert!(result.has_errors());
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "UnsupportedComplexFenestrationBasis"
                && diagnostic.object_type == OBJECT_TYPE
                && diagnostic.field.as_deref() == Some(field)
        }));
    }

    let raw = parse_epjson_str(&positive_epjson().replace(
        r#""basis_matrix_name":"Basis""#,
        r#""basis_matrix_name":"basis""#,
    ))?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected case-normalized matrix reference"))?;
    assert_eq!(
        model.constructions[4]
            .complex_fenestration
            .as_ref()
            .ok_or_else(|| std::io::Error::other("missing case-normalized metadata"))?
            .basis_matrix
            .source_name,
        "Basis"
    );
    Ok(())
}

#[test]
fn layer_topology_material_families_reserved_fields_and_matrix_shapes_are_validated()
-> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        (
            positive_epjson().replacen(
                r#""outside_layer_directional_back_absorptance_matrix_name":"Abs Back""#,
                r#""outside_layer_directional_back_absorptance_matrix_name":"Abs Back","gap_1_name":"""#,
                1,
            ),
            "InvalidComplexFenestrationLayerTopology",
            None,
        ),
        (
            positive_epjson().replace(r#""gap_1_name":"Complex Gap""#, r#""gap_1_name":"""#),
            "MissingRequiredField",
            Some("gap_1_name"),
        ),
        (
            positive_epjson().replace(
                r#""outside_layer_name":"Glass Outside""#,
                r#""outside_layer_name":"Complex Gap""#,
            ),
            "InvalidComplexFenestrationLayerMaterial",
            Some("outside_layer_name"),
        ),
        (
            positive_epjson().replace(
                r#""cfs_gap_1_directional_front_absorptance_matrix_name":"""#,
                r#""cfs_gap_1_directional_front_absorptance_matrix_name":"Abs Front""#,
            ),
            "UnsupportedComplexFenestrationReservedField",
            Some("cfs_gap_1_directional_front_absorptance_matrix_name"),
        ),
        (
            positive_epjson()
                .replace(
                    r#""Abs Front": {"number_of_rows":1, "number_of_columns":1,"#,
                    r#""Abs Front": {"number_of_rows":2, "number_of_columns":1,"#,
                )
                .replace(
                    r#""values":[{"value":0.1}]"#,
                    r#""values":[{"value":0.1},{"value":0.2}]"#,
                ),
            "InvalidComplexFenestrationMatrixDimensions",
            Some("outside_layer_directional_front_absorptance_matrix_name"),
        ),
    ];

    for (epjson, code, field) in cases {
        let raw = parse_epjson_str(&epjson)?;
        let result = compile_raw_model(&raw);
        assert!(result.has_errors(), "expected {code} for {field:?}");
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == code
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.field.as_deref() == field
            }),
            "missing {code} for {field:?}: {:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn building_surface_rejects_complex_state_until_bsdf_window_execution_is_ported()
-> Result<(), Box<dyn std::error::Error>> {
    let mut epjson = positive_epjson();
    epjson.pop();
    epjson.push_str(
        r#", "BuildingSurface:Detailed": {
            "Complex Wall": {
                "surface_type":"Wall",
                "construction_name":"Alpha Triple"
            }
        }}"#,
    );
    let raw = parse_epjson_str(&epjson)?;
    let result = compile_raw_model(&raw);
    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidBuildingSurfaceConstructionKind"
            && diagnostic.object_type == "BuildingSurface:Detailed"
            && diagnostic.object_name.as_deref() == Some("Complex Wall")
            && diagnostic.field.as_deref() == Some("construction_name")
    }));
    Ok(())
}

#[test]
fn invalid_first_complex_state_does_not_reserve_shared_construction_identity()
-> Result<(), Box<dyn std::error::Error>> {
    let mut epjson = positive_epjson()
        .replace(r#""Zulu Single": {"#, r#""Reserve Me": {"#)
        .replace(r#""Alpha Triple": {"#, r#""reserve me": {"#);
    let first_start = epjson
        .find(r#""Reserve Me": {"#)
        .ok_or_else(|| std::io::Error::other("missing first case-variant state"))?;
    let relative_basis = epjson[first_start..]
        .find(r#""basis_matrix_name":"Basis""#)
        .ok_or_else(|| std::io::Error::other("missing first basis reference"))?;
    let basis_start = first_start + relative_basis;
    let basis_end = basis_start + r#""basis_matrix_name":"Basis""#.len();
    epjson.replace_range(
        basis_start..basis_end,
        r#""basis_matrix_name":"Missing Basis""#,
    );

    let raw = parse_epjson_str(&epjson)?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);
    compiler.parse_air_boundary_constructions(&mut model);

    compiler.parse_complex_fenestration_states(&mut model);
    assert!(has_error(
        &compiler.diagnostics,
        "MissingReference",
        OBJECT_TYPE,
        Some("Reserve Me"),
        Some("basis_matrix_name")
    ));
    assert!(!compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("reserve me")
    }));
    assert_eq!(
        model.construction_names.resolve("Reserve Me"),
        Some(ConstructionId(4))
    );
    assert_eq!(model.constructions[4].name.0, "RESERVE ME");
    assert_eq!(
        model.constructions[4].outside_layer,
        model.material_names.resolve("Glass Outside")
    );
    Ok(())
}
