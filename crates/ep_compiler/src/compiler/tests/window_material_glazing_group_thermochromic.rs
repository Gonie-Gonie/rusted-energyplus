use super::super::{
    CompileResult, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{MaterialDefinition, MaterialFamily, MaterialId, MaterialKind};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "WindowMaterial:GlazingGroup:Thermochromic";

fn has_diagnostic(
    result: &CompileResult,
    severity: DiagnosticSeverity,
    code: &str,
    object_name: &str,
    field: &str,
) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == severity
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == Some(field)
    })
}

#[test]
fn thermochromic_group_materializes_source_ordered_states_and_supported_references()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "Earlier Opaque": {
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":1000.0,
                    "specific_heat":1000.0
                }
            },
            "WindowMaterial:Glazing": {
                "Spectral Child": {
                    "optical_data_type":"SpectralAverage",
                    "thickness":0.004
                }
            },
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {
                "Refraction Child": {
                    "thickness":0.003,
                    "solar_index_of_refraction":1.5,
                    "solar_extinction_coefficient":20.0,
                    "visible_index_of_refraction":1.6,
                    "visible_extinction_coefficient":30.0
                }
            },
            "Material:RoofVegetation": {
                "Earlier Roof Vegetation": {}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "Z Second Group": {
                    "temperature_data": [
                        {
                            "optical_data_temperature":5.0,
                            "window_material_glazing_name":"SpEcTrAl ChIlD"
                        }
                    ]
                },
                "A First Group": {
                    "temperature_data": [
                        {
                            "optical_data_temperature":40.0,
                            "window_material_glazing_name":"refraction child"
                        },
                        {
                            "optical_data_temperature":-999.5,
                            "window_material_glazing_name":"SPECTRAL CHILD"
                        },
                        {
                            "optical_data_temperature":40.0,
                            "window_material_glazing_name":"REFRACTION CHILD"
                        }
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
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed thermochromic groups"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "EARLIER OPAQUE",
            "SPECTRAL CHILD",
            "REFRACTION CHILD",
            "EARLIER ROOF VEGETATION",
            "A FIRST GROUP",
            "Z SECOND GROUP",
        ],
        "material IDs must retain EnergyPlus family order and effective object order"
    );

    let first_material = &model.materials[4];
    assert_eq!(first_material.id, MaterialId(4));
    assert_eq!(
        first_material.kind(),
        MaterialKind::WindowGlazingThermochromicGroup
    );
    assert_eq!(first_material.family(), MaterialFamily::ThermochromicGroup);
    assert!(first_material.as_opaque().is_none());
    assert_eq!(first_material.thickness_m(), None);
    assert_eq!(first_material.thermal_resistance(), None);
    let MaterialDefinition::WindowGlazingThermochromicGroup(first_group) =
        first_material.definition
    else {
        return Err(std::io::Error::other("expected first thermochromic group payload").into());
    };
    assert_eq!(first_group.first_state, 0);
    assert_eq!(first_group.state_count, 3);
    let first_states = model
        .window_glazing_thermochromic_states(first_group)
        .ok_or_else(|| std::io::Error::other("expected first group state slice"))?;
    assert_eq!(
        first_states
            .iter()
            .map(|state| state.optical_data_temperature_c)
            .collect::<Vec<_>>(),
        vec![40.0, -999.5, 40.0],
        "temperatures must not be sorted, bounded, or deduplicated"
    );
    assert_eq!(
        first_states
            .iter()
            .map(|state| state.glazing_material)
            .collect::<Vec<_>>(),
        vec![MaterialId(2), MaterialId(1), MaterialId(2)],
        "duplicate child references and case-insensitive resolution are source-effective"
    );

    let MaterialDefinition::WindowGlazingThermochromicGroup(second_group) =
        model.materials[5].definition
    else {
        return Err(std::io::Error::other("expected second thermochromic group payload").into());
    };
    assert_eq!(second_group.first_state, 3);
    assert_eq!(second_group.state_count, 1);
    assert_eq!(model.window_glazing_thermochromic_state_arena.len(), 4);
    assert_eq!(
        model
            .window_glazing_thermochromic_states(second_group)
            .and_then(|states| states.first())
            .map(|state| state.glazing_material),
        Some(MaterialId(1))
    );
    Ok(())
}

#[test]
fn thermochromic_group_rejects_empty_malformed_and_incomplete_state_data()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass": {"optical_data_type":"SpectralAverage","thickness":0.004}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "A Missing Array": {},
                "B Wrong Array": {"temperature_data":"not-an-array"},
                "C Empty Array": {"temperature_data":[]},
                "D Wrong Entry": {"temperature_data":[42]},
                "E Missing Temperature": {
                    "temperature_data":[{"window_material_glazing_name":"Glass"}]
                },
                "F Missing Glazing": {
                    "temperature_data":[{"optical_data_temperature":10.0}]
                },
                "G Wrong Temperature": {
                    "temperature_data":[{
                        "optical_data_temperature":"hot",
                        "window_material_glazing_name":"Glass"
                    }]
                },
                "H Wrong Glazing": {
                    "temperature_data":[{
                        "optical_data_temperature":20.0,
                        "window_material_glazing_name":3
                    }]
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "MissingRequiredField",
        "A Missing Array",
        "temperature_data"
    ));
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "InvalidFieldType",
        "B Wrong Array",
        "temperature_data"
    ));
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "MissingThermochromicGlazingState",
        "C Empty Array",
        "temperature_data"
    ));
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "InvalidFieldType",
        "D Wrong Entry",
        "temperature_data"
    ));
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "MissingRequiredField",
        "E Missing Temperature[0]",
        "optical_data_temperature"
    ));
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "MissingRequiredField",
        "F Missing Glazing[0]",
        "window_material_glazing_name"
    ));
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "InvalidFieldType",
        "G Wrong Temperature[0]",
        "optical_data_temperature"
    ));
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "InvalidFieldType",
        "H Wrong Glazing[0]",
        "window_material_glazing_name"
    ));
    Ok(())
}

#[test]
fn thermochromic_group_enforces_source_ordered_reference_resolution_and_child_types()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "Opaque": {
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":1000.0,
                    "specific_heat":1000.0
                }
            },
            "WindowMaterial:Glazing": {
                "Glass": {"optical_data_type":"SpectralAverage","thickness":0.004}
            },
            "WindowMaterial:Glazing:EquivalentLayer": {
                "Equivalent Glass": {
                    "front_side_beam_beam_solar_transmittance":0.61,
                    "back_side_beam_beam_solar_transmittance":0.62,
                    "front_side_beam_beam_solar_reflectance":0.21,
                    "back_side_beam_beam_solar_reflectance":0.22
                }
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "A Earlier Group": {
                    "temperature_data":[{
                        "optical_data_temperature":1.0,
                        "window_material_glazing_name":"Glass"
                    }]
                },
                "B Missing Child": {
                    "temperature_data":[{
                        "optical_data_temperature":2.0,
                        "window_material_glazing_name":"Never Defined"
                    }]
                },
                "C Non Glazing": {
                    "temperature_data":[{
                        "optical_data_temperature":3.0,
                        "window_material_glazing_name":"Opaque"
                    }]
                },
                "D Equivalent Layer": {
                    "temperature_data":[{
                        "optical_data_temperature":4.0,
                        "window_material_glazing_name":"Equivalent Glass"
                    }]
                },
                "E Self Reference": {
                    "temperature_data":[{
                        "optical_data_temperature":5.0,
                        "window_material_glazing_name":"E Self Reference"
                    }]
                },
                "F Earlier Group Reference": {
                    "temperature_data":[{
                        "optical_data_temperature":6.0,
                        "window_material_glazing_name":"A Earlier Group"
                    }]
                },
                "G Later Group Reference": {
                    "temperature_data":[{
                        "optical_data_temperature":7.0,
                        "window_material_glazing_name":"H Later Group"
                    }]
                },
                "H Later Group": {
                    "temperature_data":[{
                        "optical_data_temperature":8.0,
                        "window_material_glazing_name":"Glass"
                    }]
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    for object_name in ["B Missing Child[0]", "G Later Group Reference[0]"] {
        assert!(
            has_diagnostic(
                &result,
                DiagnosticSeverity::Error,
                "MissingReference",
                object_name,
                "window_material_glazing_name"
            ),
            "missing source-order missing-reference diagnostic for {object_name}"
        );
    }
    for object_name in [
        "C Non Glazing[0]",
        "D Equivalent Layer[0]",
        "E Self Reference[0]",
        "F Earlier Group Reference[0]",
    ] {
        assert!(
            has_diagnostic(
                &result,
                DiagnosticSeverity::Error,
                "InvalidThermochromicGlazingReferenceType",
                object_name,
                "window_material_glazing_name"
            ),
            "missing wrong-child-type diagnostic for {object_name}"
        );
    }
    Ok(())
}

#[test]
fn thermochromic_group_shares_the_global_material_name_namespace()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Shared": {"optical_data_type":"SpectralAverage","thickness":0.004}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "shared": {
                    "temperature_data":[{
                        "optical_data_temperature":10.0,
                        "window_material_glazing_name":"Shared"
                    }]
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("shared")
    }));
    Ok(())
}

#[test]
fn thermochromic_group_construction_consumption_maps_the_first_state_and_master_metadata()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass": {"optical_data_type":"SpectralAverage","thickness":0.004}
            },
            "WindowMaterial:Gas": {
                "Air Gap": {"gas_type":"Air","thickness":0.01}
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "TC Group": {
                    "temperature_data":[{
                        "optical_data_temperature":10.0,
                        "window_material_glazing_name":"Glass"
                    }]
                }
            },
            "Construction": {
                "A Group Outside": {"outside_layer":"TC Group"},
                "B Group Later": {
                    "outside_layer":"Glass",
                    "layer_2":"Air Gap",
                    "layer_3":"TC Group"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed TC master constructions"))?;
    let glass = model
        .material_names
        .resolve("Glass")
        .ok_or_else(|| std::io::Error::other("missing effective glazing"))?;
    let air_gap = model
        .material_names
        .resolve("Air Gap")
        .ok_or_else(|| std::io::Error::other("missing window gas"))?;
    let parent = model
        .material_names
        .resolve("TC Group")
        .ok_or_else(|| std::io::Error::other("missing TC parent"))?;
    let outside = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "A GROUP OUTSIDE")
        .ok_or_else(|| std::io::Error::other("missing outside TC construction"))?;
    assert_eq!(outside.layers, vec![glass]);
    let outside_master = outside
        .thermochromic_master
        .ok_or_else(|| std::io::Error::other("missing outside TC metadata"))?;
    assert_eq!(outside_master.parent_material, parent);
    assert_eq!(outside_master.layer_index, 0);
    assert_eq!(outside_master.glazing_layer_index, 0);

    let later = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "B GROUP LATER")
        .ok_or_else(|| std::io::Error::other("missing later TC construction"))?;
    assert_eq!(later.layers, vec![glass, air_gap, glass]);
    let later_master = later
        .thermochromic_master
        .ok_or_else(|| std::io::Error::other("missing later TC metadata"))?;
    assert_eq!(later_master.parent_material, parent);
    assert_eq!(later_master.layer_index, 2);
    assert_eq!(later_master.glazing_layer_index, 1);
    assert_eq!(model.constructions.len(), 2, "TC children remain deferred");
    Ok(())
}
