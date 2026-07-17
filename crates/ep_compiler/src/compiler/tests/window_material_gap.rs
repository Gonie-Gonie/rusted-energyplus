use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{
    ConstructionKind, MaterialDefinition, MaterialFamily, MaterialId, MaterialKind, TypedModel,
    WindowComplexGapGasComposition, WindowGasType, WindowStandardGasType,
};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "WindowMaterial:Gap";

fn has_diagnostic(
    result: &CompileResult,
    code: &str,
    object_type: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == object_type
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

fn complex_gap<'a>(
    model: &'a TypedModel,
    normalized_name: &str,
) -> Result<&'a ep_model::WindowComplexGapMaterial, std::io::Error> {
    model
        .materials
        .iter()
        .find(|material| material.name.0 == normalized_name)
        .and_then(ep_model::Material::as_window_complex_gap)
        .ok_or_else(|| std::io::Error::other(format!("missing complex gap {normalized_name}")))
}

#[test]
fn window_complex_gap_copies_single_custom_and_mixture_state_in_source_order()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass": {"optical_data_type":"SpectralAverage", "thickness":0.004}
            },
            "WindowMaterial:Gas": {
                "Air Source": {"gas_type":"Air", "thickness":0.020},
                "Custom Source": {
                    "gas_type":"Custom", "thickness":0.030,
                    "conductivity_coefficient_a":0.010,
                    "conductivity_coefficient_b":0.00001,
                    "conductivity_coefficient_c":0.000000001,
                    "viscosity_coefficient_a":0.00001,
                    "viscosity_coefficient_b":0.00000002,
                    "viscosity_coefficient_c":0.00000000003,
                    "specific_heat_coefficient_a":900.0,
                    "specific_heat_coefficient_b":0.1,
                    "specific_heat_coefficient_c":0.001,
                    "molecular_weight":44.0,
                    "specific_heat_ratio":1.25
                }
            },
            "WindowMaterial:GasMixture": {
                "Air Argon Source": {
                    "thickness":0.040, "number_of_gases_in_mixture":2,
                    "gas_1_type":"Air", "gas_1_fraction":0.25,
                    "gas_2_type":"Argon", "gas_2_fraction":0.75
                }
            },
            "WindowMaterial:SimpleGlazingSystem": {
                "Earlier Simple": {"u_factor":3.0, "solar_heat_gain_coefficient":0.5}
            },
            "WindowMaterial:Gap": {
                "A Standard Gap": {
                    "thickness":0.006, "gas_or_gas_mixture_":"aIr SoUrCe"
                },
                "B Custom Gap": {
                    "thickness":0.007, "gas_or_gas_mixture_":"CUSTOM SOURCE",
                    "pressure":87654.321
                },
                "C Mixture Gap": {
                    "thickness":0.008, "gas_or_gas_mixture_":"air argon source",
                    "pressure":1.0
                }
            },
            "Construction": {
                "Ordinary Window": {
                    "outside_layer":"Glass", "layer_2":"Air Source", "layer_3":"Glass"
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
    assert_eq!(
        typed_coverage_status("WindowGap:DeflectionState"),
        ObjectCoverageStatus::RawOnly
    );
    assert_eq!(
        typed_coverage_status("WindowGap:SupportPillar"),
        ObjectCoverageStatus::RawOnly
    );
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed complex gaps"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "GLASS",
            "AIR SOURCE",
            "CUSTOM SOURCE",
            "AIR ARGON SOURCE",
            "EARLIER SIMPLE",
            "A STANDARD GAP",
            "B CUSTOM GAP",
            "C MIXTURE GAP",
        ],
        "WindowMaterial:Gap must follow SimpleGlazingSystem in material-family source order"
    );

    let air_source = model.materials[1]
        .as_window_gas()
        .ok_or_else(|| std::io::Error::other("missing air source"))?;
    let standard = complex_gap(model, "A STANDARD GAP")?;
    assert_eq!(standard.thickness_m, 0.006);
    assert_ne!(standard.thickness_m, air_source.thickness_m);
    assert_eq!(standard.pressure_pa, 101_325.0);
    assert_eq!(standard.deflected_thickness_m, 0.0);
    assert_eq!(standard.support_pillar, None);
    assert_eq!(standard.source_material_id(), MaterialId(1));
    let WindowComplexGapGasComposition::Single {
        source_material_id,
        gas_type,
        properties,
    } = standard.gas
    else {
        return Err(std::io::Error::other("expected copied single gas").into());
    };
    assert_eq!(source_material_id, MaterialId(1));
    assert_eq!(gas_type, WindowGasType::Air);
    assert_eq!(properties, air_source.properties);

    let custom_source = model.materials[2]
        .as_window_gas()
        .ok_or_else(|| std::io::Error::other("missing custom source"))?;
    let custom = complex_gap(model, "B CUSTOM GAP")?;
    assert_eq!(custom.thickness_m, 0.007);
    assert_ne!(custom.thickness_m, custom_source.thickness_m);
    assert_eq!(custom.pressure_pa, 87654.321);
    assert_eq!(
        custom.gas,
        WindowComplexGapGasComposition::Single {
            source_material_id: MaterialId(2),
            gas_type: WindowGasType::Custom,
            properties: custom_source.properties,
        }
    );

    let mixture_source = model.materials[3]
        .as_window_gas_mixture()
        .ok_or_else(|| std::io::Error::other("missing mixture source"))?;
    let mixture = complex_gap(model, "C MIXTURE GAP")?;
    assert_eq!(mixture.thickness_m, 0.008);
    assert_ne!(mixture.thickness_m, mixture_source.thickness_m);
    assert_eq!(mixture.pressure_pa, 1.0);
    assert_eq!(mixture.active_gas_count(), 2);
    assert_eq!(
        mixture.gas,
        WindowComplexGapGasComposition::Mixture {
            source_material_id: MaterialId(3),
            gases: mixture_source.gases,
        }
    );
    assert_eq!(
        mixture_source.gases.components()[0].gas_type,
        WindowStandardGasType::Air
    );
    assert_eq!(
        mixture_source.gases.components()[1].gas_type,
        WindowStandardGasType::Argon
    );

    for material in &model.materials[5..] {
        assert_eq!(material.kind(), MaterialKind::WindowComplexGap);
        assert_eq!(material.family(), MaterialFamily::ComplexFenestration);
        assert!(material.as_window_complex_gap().is_some());
        assert!(material.as_opaque().is_none());
        assert_eq!(material.thickness_m(), None);
        assert_eq!(material.thermal_resistance(), None);
    }
    assert_eq!(model.constructions.len(), 1);
    assert_eq!(model.constructions[0].kind, ConstructionKind::Fenestration);
    Ok(())
}

#[test]
fn window_complex_gap_resolves_helpers_case_insensitively_and_preserves_only_copied_fields()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Air Source": {"gas_type":"Air", "thickness":0.0127}
            },
            "WindowGap:DeflectionState": {
                "Defaults": {},
                "Wild Geometry": {
                    "deflected_thickness":0.5,
                    "initial_temperature":0.0,
                    "initial_pressure":0.0
                },
                "Unreferenced Bad": {
                    "deflected_thickness":"bad",
                    "initial_temperature":-1.0,
                    "initial_pressure":false
                }
            },
            "WindowGap:SupportPillar": {
                "Pillar Defaults": {},
                "Wild Pillar": {"spacing":0.0001, "radius":0.01},
                "Unreferenced Bad": {"spacing":0.0, "radius":"bad"}
            },
            "WindowMaterial:Gap": {
                "A Blank Helpers": {
                    "thickness":0.009, "gas_or_gas_mixture_":"Air Source",
                    "deflection_state":"", "support_pillar":""
                },
                "B Defaults": {
                    "thickness":0.010, "gas_or_gas_mixture_":"Air Source",
                    "deflection_state":"dEfAuLtS", "support_pillar":"pIlLaR dEfAuLtS"
                },
                "C Wild Independent Geometry": {
                    "thickness":0.001, "gas_or_gas_mixture_":"Air Source",
                    "deflection_state":"WILD GEOMETRY", "support_pillar":"wild pillar"
                },
                "D Reused Defaults": {
                    "thickness":0.011, "gas_or_gas_mixture_":"Air Source",
                    "deflection_state":"DEFAULTS", "support_pillar":"PILLAR DEFAULTS"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    assert!(result.report.diagnostics.is_empty());
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected helper-backed gaps"))?;

    let blank = complex_gap(model, "A BLANK HELPERS")?;
    assert_eq!(blank.deflected_thickness_m, 0.0);
    assert_eq!(blank.support_pillar, None);

    for name in ["B DEFAULTS", "D REUSED DEFAULTS"] {
        let gap = complex_gap(model, name)?;
        assert_eq!(gap.deflected_thickness_m, 0.0);
        let pillar = gap
            .support_pillar
            .ok_or_else(|| std::io::Error::other("expected defaulted pillar"))?;
        assert_eq!(pillar.spacing_m, 0.04);
        assert_eq!(pillar.radius_m, 0.0004);
    }

    let wild = complex_gap(model, "C WILD INDEPENDENT GEOMETRY")?;
    assert_eq!(wild.thickness_m, 0.001);
    assert_eq!(wild.deflected_thickness_m, 0.5);
    let pillar = wild
        .support_pillar
        .ok_or_else(|| std::io::Error::other("expected explicit pillar"))?;
    assert_eq!(pillar.spacing_m, 0.0001);
    assert_eq!(pillar.radius_m, 0.01);
    assert!(wild.deflected_thickness_m > wild.thickness_m);
    assert!(pillar.radius_m > pillar.spacing_m);
    Ok(())
}

#[test]
fn window_complex_gap_requires_exact_fields_and_strict_positive_primary_values()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Air Source": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:Gap": {
                "A Missing Thickness": {"gas_or_gas_mixture_":"Air Source"},
                "B Zero Thickness": {"thickness":0.0, "gas_or_gas_mixture_":"Air Source"},
                "C Negative Thickness": {"thickness":-0.1, "gas_or_gas_mixture_":"Air Source"},
                "D Missing Exact Gas Key": {
                    "thickness":0.01, "gas_or_gas_mixture":"Air Source"
                },
                "E Wrong Gas Type": {"thickness":0.01, "gas_or_gas_mixture_":7},
                "F Zero Pressure": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source", "pressure":0.0
                },
                "G Negative Pressure": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source", "pressure":-1.0
                },
                "H Wrong Pressure": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source", "pressure":"high"
                },
                "I Wrong Deflection Reference": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source", "deflection_state":3
                },
                "J Wrong Pillar Reference": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source", "support_pillar":true
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    assert!(has_diagnostic(
        &result,
        "MissingRequiredField",
        OBJECT_TYPE,
        "A Missing Thickness",
        Some("thickness")
    ));
    for name in ["B Zero Thickness", "C Negative Thickness"] {
        assert!(has_diagnostic(
            &result,
            "InvalidNumericRange",
            OBJECT_TYPE,
            name,
            Some("thickness")
        ));
    }
    assert!(has_diagnostic(
        &result,
        "MissingRequiredField",
        OBJECT_TYPE,
        "D Missing Exact Gas Key",
        Some("gas_or_gas_mixture_")
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidFieldType",
        OBJECT_TYPE,
        "E Wrong Gas Type",
        Some("gas_or_gas_mixture_")
    ));
    for name in ["F Zero Pressure", "G Negative Pressure"] {
        assert!(has_diagnostic(
            &result,
            "InvalidNumericRange",
            OBJECT_TYPE,
            name,
            Some("pressure")
        ));
    }
    for (name, field) in [
        ("H Wrong Pressure", "pressure"),
        ("I Wrong Deflection Reference", "deflection_state"),
        ("J Wrong Pillar Reference", "support_pillar"),
    ] {
        assert!(has_diagnostic(
            &result,
            "InvalidFieldType",
            OBJECT_TYPE,
            name,
            Some(field)
        ));
    }
    Ok(())
}

#[test]
fn window_complex_gap_references_fail_closed_for_missing_wrong_family_ambiguous_and_bad_helpers()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "Opaque Source": {
                    "roughness":"Rough", "thickness":0.1, "conductivity":1.0,
                    "density":1000.0, "specific_heat":1000.0
                }
            },
            "WindowMaterial:Gas": {
                "Air Source": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowGap:DeflectionState": {
                "Twin": {}, "tWiN": {},
                "Bad Deflection": {
                    "deflected_thickness":-1.0,
                    "initial_temperature":-2.0,
                    "initial_pressure":-3.0
                }
            },
            "WindowGap:SupportPillar": {
                "Twin": {}, "tWiN": {},
                "Bad Pillar": {"spacing":0.0, "radius":0.0}
            },
            "WindowMaterial:Gap": {
                "A Missing Gas": {"thickness":0.01, "gas_or_gas_mixture_":"Missing"},
                "B Wrong Family": {"thickness":0.01, "gas_or_gas_mixture_":"Opaque Source"},
                "C Missing Deflection": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source",
                    "deflection_state":"Missing"
                },
                "D Missing Pillar": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source",
                    "support_pillar":"Missing"
                },
                "E Ambiguous Deflection": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source",
                    "deflection_state":"TWIN"
                },
                "F Ambiguous Pillar": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source",
                    "support_pillar":"twin"
                },
                "G Bad Deflection": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source",
                    "deflection_state":"Bad Deflection"
                },
                "H Bad Pillar": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source",
                    "support_pillar":"Bad Pillar"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    assert!(has_diagnostic(
        &result,
        "MissingReference",
        OBJECT_TYPE,
        "A Missing Gas",
        Some("gas_or_gas_mixture_")
    ));
    assert!(has_diagnostic(
        &result,
        "InvalidWindowComplexGapGasReference",
        OBJECT_TYPE,
        "B Wrong Family",
        Some("gas_or_gas_mixture_")
    ));
    for (name, field) in [
        ("C Missing Deflection", "deflection_state"),
        ("D Missing Pillar", "support_pillar"),
    ] {
        assert!(has_diagnostic(
            &result,
            "MissingReference",
            OBJECT_TYPE,
            name,
            Some(field)
        ));
    }
    for (name, field) in [
        ("E Ambiguous Deflection", "deflection_state"),
        ("F Ambiguous Pillar", "support_pillar"),
    ] {
        assert!(has_diagnostic(
            &result,
            "AmbiguousReference",
            OBJECT_TYPE,
            name,
            Some(field)
        ));
    }
    for field in [
        "deflected_thickness",
        "initial_temperature",
        "initial_pressure",
    ] {
        assert!(has_diagnostic(
            &result,
            "InvalidNumericRange",
            "WindowGap:DeflectionState",
            "Bad Deflection",
            Some(field)
        ));
    }
    for field in ["spacing", "radius"] {
        assert!(has_diagnostic(
            &result,
            "InvalidNumericRange",
            "WindowGap:SupportPillar",
            "Bad Pillar",
            Some(field)
        ));
    }
    Ok(())
}

#[test]
fn window_complex_gap_reserves_shared_identity_only_after_all_validation()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Air Source": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:Gap": {
                "A Shared": {
                    "thickness":0.01, "gas_or_gas_mixture_":"Air Source",
                    "support_pillar":"Missing"
                },
                "a shared": {"thickness":0.02, "gas_or_gas_mixture_":"Air Source"}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);

    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingReference"
            && diagnostic.object_name.as_deref() == Some("A Shared")
            && diagnostic.field.as_deref() == Some("support_pillar")
    }));
    assert!(!compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName" && diagnostic.object_type == OBJECT_TYPE
    }));
    assert_eq!(
        model
            .materials
            .iter()
            .filter(|material| matches!(
                material.definition,
                MaterialDefinition::WindowComplexGap(_)
            ))
            .count(),
        1
    );
    assert_eq!(complex_gap(&model, "A SHARED")?.thickness_m, 0.02);

    let collision_raw = parse_epjson_str(
        r#"{
            "Material": {
                "Shared": {
                    "roughness":"Rough", "thickness":0.1, "conductivity":1.0,
                    "density":1000.0, "specific_heat":1000.0
                }
            },
            "WindowMaterial:Gas": {
                "Air Source": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:Gap": {
                "shared": {"thickness":0.01, "gas_or_gas_mixture_":"Air Source"}
            }
        }"#,
    )?;
    let collision = compile_raw_model(&collision_raw);
    assert!(has_diagnostic(
        &collision,
        "DuplicateName",
        OBJECT_TYPE,
        "shared",
        None
    ));
    Ok(())
}

#[test]
fn ordinary_construction_rejects_complex_gap_in_every_plausible_position()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass": {"optical_data_type":"SpectralAverage", "thickness":0.004}
            },
            "WindowMaterial:Gas": {
                "Air Source": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:Gap": {
                "Complex Gap": {"thickness":0.01, "gas_or_gas_mixture_":"Air Source"}
            },
            "Construction": {
                "A Sole": {"outside_layer":"Complex Gap"},
                "B Exterior": {"outside_layer":"Complex Gap", "layer_2":"Glass"},
                "C Middle": {
                    "outside_layer":"Glass", "layer_2":"Complex Gap", "layer_3":"Glass"
                },
                "D Interior": {
                    "outside_layer":"Glass", "layer_2":"Air Source", "layer_3":"Complex Gap"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    for (name, field) in [
        ("A Sole", "outside_layer"),
        ("B Exterior", "outside_layer"),
        ("C Middle", "layer_2"),
        ("D Interior", "layer_3"),
    ] {
        assert!(has_diagnostic(
            &result,
            "UnsupportedComplexFenestrationConstruction",
            "Construction",
            name,
            Some(field)
        ));
    }
    assert_eq!(
        result
            .report
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code == "UnsupportedComplexFenestrationConstruction"
                    && diagnostic
                        .message
                        .contains("Construction:ComplexFenestrationState")
            })
            .count(),
        4
    );
    Ok(())
}
