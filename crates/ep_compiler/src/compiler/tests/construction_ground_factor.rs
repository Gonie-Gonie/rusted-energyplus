use super::super::{
    Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model, typed_coverage_status,
};
use ep_model::{
    ConstructionGroundFactor, ConstructionId, ConstructionKind, MaterialDefinition, MaterialId,
    MaterialSurfaceRoughness, ModelGraph, NormalizedName, TypedModel,
};
use ep_raw_model::{parse_epjson_str, parse_epjson_str_with_idf_order};

const FFACTOR_OBJECT_TYPE: &str = "Construction:FfactorGroundFloor";
const CFACTOR_OBJECT_TYPE: &str = "Construction:CfactorUndergroundWall";
const CONCRETE_RESISTANCE: f64 = 0.15 / 1.95;

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12,
        "expected {expected:.15}, got {actual:.15}"
    );
}

fn has_error(
    compiler: &Compiler<'_>,
    code: &str,
    object_type: &str,
    object_name: &str,
    field: Option<&str>,
) -> bool {
    compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == object_type
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

fn no_mass_resistance(model: &TypedModel, id: MaterialId) -> Result<f64, std::io::Error> {
    match model.materials[id.0 as usize].definition {
        MaterialDefinition::NoMass(material) => Ok(material.thermal_resistance_m2_k_per_w),
        _ => Err(std::io::Error::other(
            "expected generated no-mass insulation",
        )),
    }
}

#[test]
fn ground_factor_constructions_preserve_source_order_internal_materials_formulas_and_graph()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "Regular User": {
                    "roughness":"Rough", "thickness":0.2, "conductivity":0.4,
                    "density":800.0, "specific_heat":900.0
                }
            },
            "Material:NoMass": {
                "NoMass User": {"roughness":"Smooth", "thermal_resistance":1.0}
            },
            "Material:AirGap": {
                "Air Gap": {"thermal_resistance":0.15}
            },
            "Construction": {
                "Ordinary": {"outside_layer":"NoMass User"}
            },
            "Construction:FfactorGroundFloor": {
                "A F Positive": {"f_factor":0.5, "area":100.0, "perimeterexposed":20.0},
                "B F Zero": {"f_factor":0.4, "area":50.0, "perimeterexposed":0.0}
            },
            "Construction:CfactorUndergroundWall": {
                "A C Quarter": {"c_factor":0.5, "height":0.25},
                "B C Mid": {"c_factor":0.5, "height":1.0},
                "C C Two Point Five": {"c_factor":0.5, "height":2.5}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed F/C-factor model"))?;

    assert_eq!(
        typed_coverage_status(FFACTOR_OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert_eq!(
        typed_coverage_status(CFACTOR_OBJECT_TYPE),
        ObjectCoverageStatus::Typed
    );
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "REGULAR USER",
            "~FC_CONCRETE",
            "NOMASS USER",
            "~FC_INSULATION_1",
            "~FC_INSULATION_2",
            "~FC_INSULATION_3",
            "~FC_INSULATION_4",
            "~FC_INSULATION_5",
            "AIR GAP",
        ]
    );
    assert_eq!(model.material_names.len(), 3);
    assert_eq!(
        model.material_names.resolve("Regular User"),
        Some(MaterialId(0))
    );
    assert_eq!(
        model.material_names.resolve("NoMass User"),
        Some(MaterialId(2))
    );
    assert_eq!(model.material_names.resolve("Air Gap"), Some(MaterialId(8)));
    assert_eq!(model.material_names.resolve("~FC_Concrete"), None);
    assert_eq!(model.material_names.resolve("~FC_Insulation_1"), None);

    let MaterialDefinition::Regular(concrete) = model.materials[1].definition else {
        return Err(std::io::Error::other("missing generated concrete").into());
    };
    assert_eq!(concrete.roughness, MaterialSurfaceRoughness::MediumRough);
    assert_close(concrete.thickness_m, 0.15);
    assert_close(concrete.conductivity_w_per_m_k, 1.95);
    assert_close(concrete.density_kg_per_m3, 2240.0);
    assert_close(concrete.specific_heat_j_per_kg_k, 900.0);
    assert_close(concrete.surface.thermal_absorptance, 0.9);
    assert_close(concrete.surface.solar_absorptance, 0.7);
    assert_close(concrete.surface.visible_absorptance, 0.7);

    for material in &model.materials[3..8] {
        let MaterialDefinition::NoMass(insulation) = material.definition else {
            return Err(std::io::Error::other("missing generated insulation").into());
        };
        assert_eq!(insulation.roughness, MaterialSurfaceRoughness::MediumRough);
        assert_close(insulation.surface.thermal_absorptance, 0.0);
        assert_close(insulation.surface.solar_absorptance, 0.0);
        assert_close(insulation.surface.visible_absorptance, 0.0);
    }

    assert_eq!(
        model
            .constructions
            .iter()
            .map(|construction| construction.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "ORDINARY",
            "A F POSITIVE",
            "B F ZERO",
            "A C QUARTER",
            "B C MID",
            "C C TWO POINT FIVE",
        ]
    );
    assert!(model.constructions[0].is_ordinary_opaque());
    assert_eq!(model.constructions[0].ground_factor, None);

    let expected_effective = [9.835, 177.0, 2.12, 2.4086, 2.92];
    let expected_soil = [0.12, 0.4086, 0.92];
    for (index, construction) in model.constructions[1..].iter().enumerate() {
        assert_eq!(construction.kind, ConstructionKind::Opaque);
        assert!(!construction.is_ordinary_opaque());
        assert_eq!(
            construction.outside_layer,
            Some(MaterialId(3 + index as u32))
        );
        assert_eq!(
            construction.layers,
            vec![MaterialId(3 + index as u32), MaterialId(1)]
        );
        assert_eq!(construction.thermochromic_master, None);
        let (effective, insulation) = match construction
            .ground_factor
            .ok_or_else(|| std::io::Error::other("missing factor metadata"))?
        {
            ConstructionGroundFactor::FfactorGroundFloor {
                effective_thermal_resistance_m2_k_per_w,
                insulation_thermal_resistance_m2_k_per_w,
                ..
            }
            | ConstructionGroundFactor::CfactorUndergroundWall {
                effective_thermal_resistance_m2_k_per_w,
                insulation_thermal_resistance_m2_k_per_w,
                ..
            } => (
                effective_thermal_resistance_m2_k_per_w,
                insulation_thermal_resistance_m2_k_per_w,
            ),
        };
        assert_close(effective, expected_effective[index]);
        assert_close(insulation, expected_effective[index] - CONCRETE_RESISTANCE);
        assert_close(
            no_mass_resistance(
                &model,
                construction
                    .outside_layer
                    .ok_or_else(|| std::io::Error::other("missing outside layer"))?,
            )?,
            insulation,
        );
    }

    match model.constructions[1].ground_factor {
        Some(ConstructionGroundFactor::FfactorGroundFloor {
            f_factor_w_per_m_k,
            area_m2,
            perimeter_exposed_m,
            ..
        }) => {
            assert_close(f_factor_w_per_m_k, 0.5);
            assert_close(area_m2, 100.0);
            assert_close(perimeter_exposed_m, 20.0);
        }
        _ => return Err(std::io::Error::other("wrong F-factor metadata").into()),
    }
    for (offset, expected) in expected_soil.into_iter().enumerate() {
        match model.constructions[3 + offset].ground_factor {
            Some(ConstructionGroundFactor::CfactorUndergroundWall {
                equivalent_soil_thermal_resistance_m2_k_per_w,
                ..
            }) => assert_close(equivalent_soil_thermal_resistance_m2_k_per_w, expected),
            _ => return Err(std::io::Error::other("wrong C-factor metadata").into()),
        }
    }

    let graph = ModelGraph::from_typed(&model);
    for construction in &model.constructions[1..] {
        let edges = graph
            .construction_materials
            .iter()
            .filter(|edge| edge.construction == construction.id)
            .collect::<Vec<_>>();
        assert_eq!(edges.len(), 2);
        assert_eq!(edges[0].material, construction.layers[0]);
        assert_eq!(edges[0].layer_index, 0);
        assert_eq!(edges[1].material, construction.layers[1]);
        assert_eq!(edges[1].layer_index, 1);
    }
    Ok(())
}

#[test]
fn staged_idf_order_controls_ordinary_f_and_c_construction_ordinals()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = r#"{
        "Material:NoMass": {
            "Layer": {"roughness":"Rough", "thermal_resistance":1.0}
        },
        "Construction": {
            "Alpha Ordinary": {"outside_layer":"Layer"},
            "Zulu Ordinary": {"outside_layer":"Layer"}
        },
        "Construction:FfactorGroundFloor": {
            "Alpha F": {"f_factor":0.5, "area":100.0, "perimeterexposed":20.0},
            "Zulu F": {"f_factor":0.4, "area":80.0, "perimeterexposed":20.0}
        },
        "Construction:CfactorUndergroundWall": {
            "Alpha C": {"c_factor":0.5, "height":0.25},
            "Zulu C": {"c_factor":0.4, "height":2.5}
        }
    }"#;
    let idf = r#"
        Material:NoMass, Layer, Rough, 1.0;
        Construction, Zulu Ordinary, Layer;
        Construction, Alpha Ordinary, Layer;
        Construction:FfactorGroundFloor, Zulu F, 0.4, 80.0, 20.0;
        Construction:FfactorGroundFloor, Alpha F, 0.5, 100.0, 20.0;
        Construction:CfactorUndergroundWall, Zulu C, 0.4, 2.5;
        Construction:CfactorUndergroundWall, Alpha C, 0.5, 0.25;
    "#;
    let raw = parse_epjson_str_with_idf_order(epjson, idf)?;
    for object_type in ["Construction", FFACTOR_OBJECT_TYPE, CFACTOR_OBJECT_TYPE] {
        assert!(raw.has_idf_declaration_order(object_type));
    }
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected IDF-ordered constructions"))?;
    assert_eq!(
        model
            .constructions
            .iter()
            .map(|construction| construction.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "ZULU ORDINARY",
            "ALPHA ORDINARY",
            "ZULU F",
            "ALPHA F",
            "ZULU C",
            "ALPHA C",
        ]
    );
    for (index, construction) in model.constructions[2..].iter().enumerate() {
        assert_eq!(
            construction.outside_layer,
            Some(MaterialId(2 + index as u32))
        );
        let outside_layer = construction
            .outside_layer
            .ok_or_else(|| std::io::Error::other("missing outside layer"))?;
        assert_eq!(
            model.materials[outside_layer.0 as usize].name,
            NormalizedName::new(&format!("~FC_Insulation_{}", index + 1))
        );
    }
    Ok(())
}

#[test]
fn invalid_ground_factor_inputs_do_not_reserve_identity_and_internal_names_stay_private()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "~fc_concrete": {
                    "roughness":"Rough", "thickness":0.2, "conductivity":0.4,
                    "density":800.0, "specific_heat":900.0
                }
            },
            "Material:NoMass": {
                "~FC_Insulation_1": {"roughness":"Smooth", "thermal_resistance":1.0}
            },
            "Material:AirGap": {
                "~FC_Insulation_01": {"thermal_resistance":0.1}
            },
            "Construction": {
                "Internal Layer Escape": {"outside_layer":"~FC_Concrete"}
            },
            "Construction:FfactorGroundFloor": {
                "Reserve Me": {"f_factor":0.5, "area":10.0, "perimeter_exposed":2.0},
                "Z Derived": {"f_factor":100.0, "area":1.0, "perimeterexposed":1.0},
                "reserve me": {"f_factor":0.5, "area":10.0, "perimeterexposed":2.0}
            },
            "Construction:CfactorUndergroundWall": {
                "Z Zero": {"c_factor":0.0, "height":1.0},
                "reserve me": {"c_factor":0.5, "height":1.0}
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_materials(&mut model);
    compiler.parse_constructions(&mut model);

    assert!(has_error(
        &compiler,
        "ReservedInternalGroundFactorMaterialName",
        "Material",
        "~fc_concrete",
        Some("name")
    ));
    assert!(has_error(
        &compiler,
        "ReservedInternalGroundFactorMaterialName",
        "Material:NoMass",
        "~FC_Insulation_1",
        Some("name")
    ));
    assert!(has_error(
        &compiler,
        "MissingReference",
        "Construction",
        "Internal Layer Escape",
        Some("outside_layer")
    ));
    assert!(has_error(
        &compiler,
        "MissingRequiredField",
        FFACTOR_OBJECT_TYPE,
        "Reserve Me",
        Some("perimeterexposed")
    ));
    assert!(has_error(
        &compiler,
        "InvalidGroundFactorThermalResistance",
        FFACTOR_OBJECT_TYPE,
        "Z Derived",
        None
    ));
    assert!(has_error(
        &compiler,
        "InvalidNumericRange",
        CFACTOR_OBJECT_TYPE,
        "Z Zero",
        Some("c_factor")
    ));
    assert!(has_error(
        &compiler,
        "DuplicateName",
        CFACTOR_OBJECT_TYPE,
        "reserve me",
        None
    ));

    assert_eq!(model.material_names.len(), 1);
    assert_eq!(
        model.material_names.resolve("~fc_insulation_01"),
        Some(MaterialId(6))
    );
    assert_eq!(model.materials.len(), 7);
    assert_eq!(model.materials[0].name, NormalizedName::new("~FC_Concrete"));
    for index in 0..5 {
        assert_eq!(
            model.materials[1 + index].name,
            NormalizedName::new(&format!("~FC_Insulation_{}", index + 1))
        );
    }
    assert_close(no_mass_resistance(&model, MaterialId(1))?, 0.0);
    assert_close(no_mass_resistance(&model, MaterialId(2))?, 0.0);
    assert!(no_mass_resistance(&model, MaterialId(3))? > 0.0);
    assert_close(no_mass_resistance(&model, MaterialId(4))?, 0.0);
    assert_close(no_mass_resistance(&model, MaterialId(5))?, 0.0);
    assert_eq!(
        model.materials[6].name,
        NormalizedName::new("~FC_Insulation_01")
    );

    assert_eq!(model.constructions.len(), 1);
    assert_eq!(model.constructions[0].id, ConstructionId(0));
    assert_eq!(
        model.constructions[0].name,
        NormalizedName::new("reserve me")
    );
    assert_eq!(
        model.constructions[0].layers,
        vec![MaterialId(3), MaterialId(0)]
    );
    assert_eq!(
        model.construction_names.resolve("RESERVE ME"),
        Some(ConstructionId(0))
    );
    Ok(())
}

#[test]
fn nonfinite_derived_ground_factor_resistances_fail_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Construction:FfactorGroundFloor": {
                "F Overflow": {
                    "f_factor":1.0e-308,
                    "area":1.0e308,
                    "perimeterexposed":1.0
                }
            },
            "Construction:CfactorUndergroundWall": {
                "C Overflow": {"c_factor":1.0e-320, "height":1.0}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(result.has_errors());
    for (object_type, object_name) in [
        (FFACTOR_OBJECT_TYPE, "F Overflow"),
        (CFACTOR_OBJECT_TYPE, "C Overflow"),
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidGroundFactorThermalResistance"
                && diagnostic.object_type == object_type
                && diagnostic.object_name.as_deref() == Some(object_name)
                && diagnostic.field.is_none()
        }));
    }
    assert!(result.model.is_none());
    Ok(())
}

#[test]
fn building_surface_cannot_consume_ground_factor_construction_before_surface_binding_port()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Construction:FfactorGroundFloor": {
                "Slab": {"f_factor":0.5, "area":100.0, "perimeterexposed":20.0}
            },
            "BuildingSurface:Detailed": {
                "Floor": {"surface_type":"Floor", "construction_name":"Slab"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidBuildingSurfaceConstructionKind"
            && diagnostic.object_type == "BuildingSurface:Detailed"
            && diagnostic.object_name.as_deref() == Some("Floor")
            && diagnostic.field.as_deref() == Some("construction_name")
    }));
    assert!(result.model.is_none());
    Ok(())
}
