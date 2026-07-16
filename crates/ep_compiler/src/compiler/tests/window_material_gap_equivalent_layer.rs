use super::super::{ObjectCoverageStatus, compile_raw_model};
use ep_model::{
    MaterialDefinition, MaterialFamily, MaterialId, MaterialKind, WindowGapVentType,
    WindowGasProperties, WindowGasType,
};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "WindowMaterial:Gap:EquivalentLayer";

fn assert_close(actual: f64, expected: f64) {
    let tolerance = f64::EPSILON * expected.abs().max(1.0) * 16.0;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}

fn assert_polynomial(
    properties: WindowGasProperties,
    conductivity: [f64; 3],
    viscosity: [f64; 3],
    specific_heat: [f64; 3],
) {
    assert_eq!(
        [
            properties.conductivity.coefficient_a,
            properties.conductivity.coefficient_b,
            properties.conductivity.coefficient_c,
        ],
        conductivity
    );
    assert_eq!(
        [
            properties.viscosity.coefficient_a,
            properties.viscosity.coefficient_b,
            properties.viscosity.coefficient_c,
        ],
        viscosity
    );
    assert_eq!(
        [
            properties.specific_heat.coefficient_a,
            properties.specific_heat.coefficient_b,
            properties.specific_heat.coefficient_c,
        ],
        specific_heat
    );
}

#[test]
fn equivalent_layer_standard_gaps_use_v261_constants_and_preserve_vent_types()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Air Gap": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                },
                "Argon Gap": {
                    "gas_type":"ARGON",
                    "thickness":0.013,
                    "gap_vent_type":"VentedIndoor",
                    "conductivity_coefficient_a":99.0,
                    "conductivity_coefficient_b":98.0,
                    "conductivity_coefficient_c":97.0,
                    "viscosity_coefficient_a":1.0,
                    "viscosity_coefficient_b":2.0,
                    "viscosity_coefficient_c":3.0,
                    "specific_heat_coefficient_a":100.0,
                    "specific_heat_coefficient_b":101.0,
                    "specific_heat_coefficient_c":102.0,
                    "molecular_weight":100.0,
                    "specific_heat_ratio":2.0
                },
                "Krypton Gap": {
                    "gas_type":"KRYPTON",
                    "thickness":0.014,
                    "gap_vent_type":"VentedOutdoor"
                },
                "Xenon Gap": {
                    "gas_type":"XENON",
                    "thickness":0.015,
                    "gap_vent_type":"Sealed"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected standard equivalent-layer gaps"))?;
    let expected = [
        (
            WindowGasType::Air,
            WindowGapVentType::Sealed,
            0.012,
            [2.873e-3, 7.760e-5, 0.0],
            [3.723e-6, 4.940e-8, 0.0],
            [1002.737, 1.2324e-2, 0.0],
            28.97,
            1.4,
        ),
        (
            WindowGasType::Argon,
            WindowGapVentType::VentedIndoor,
            0.013,
            [2.285e-3, 5.149e-5, 0.0],
            [3.379e-6, 6.451e-8, 0.0],
            [521.929, 0.0, 0.0],
            39.948,
            1.67,
        ),
        (
            WindowGasType::Krypton,
            WindowGapVentType::VentedOutdoor,
            0.014,
            [9.443e-4, 2.826e-5, 0.0],
            [2.213e-6, 7.777e-8, 0.0],
            [248.091, 0.0, 0.0],
            83.8,
            1.68,
        ),
        (
            WindowGasType::Xenon,
            WindowGapVentType::Sealed,
            0.015,
            [4.538e-4, 1.723e-5, 0.0],
            [1.069e-6, 7.414e-8, 0.0],
            [158.340, 0.0, 0.0],
            131.3,
            1.66,
        ),
    ];

    assert_eq!(model.materials.len(), expected.len());
    for (
        material,
        (gas_type, gap_vent_type, thickness, conductivity, viscosity, specific_heat, weight, ratio),
    ) in model.materials.iter().zip(expected)
    {
        assert_eq!(material.kind(), MaterialKind::WindowGapEquivalentLayer);
        assert_eq!(material.family(), MaterialFamily::EquivalentLayer);
        assert!(matches!(
            material.definition,
            MaterialDefinition::WindowGapEquivalentLayer(_)
        ));
        assert!(material.as_opaque().is_none());
        assert_eq!(material.thickness_m(), None);
        assert_eq!(material.conductivity_w_per_m_k(), None);
        assert_eq!(material.thermal_resistance(), None);
        assert_eq!(material.heat_capacity_per_area(), None);

        let gap = material
            .as_window_gap_equivalent_layer()
            .ok_or_else(|| std::io::Error::other("expected equivalent-layer gap payload"))?;
        assert_eq!(gap.gas_type, gas_type);
        assert_eq!(gap.gap_vent_type, gap_vent_type);
        assert_eq!(gap.thickness_m, thickness);
        assert_polynomial(gap.properties, conductivity, viscosity, specific_heat);
        assert_eq!(gap.properties.molecular_weight_g_per_mol, weight);
        assert_eq!(gap.properties.specific_heat_ratio, ratio);
        assert_close(
            gap.nominal_thermal_resistance_m2_k_per_w()
                .ok_or_else(|| std::io::Error::other("expected nominal gap resistance"))?,
            thickness / (conductivity[0] + 300.0 * conductivity[1]),
        );
    }

    Ok(())
}

#[test]
fn equivalent_layer_custom_gap_preserves_blank_values_endpoints_and_k300()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Custom Gap": {
                    "gas_type":"CUSTOM",
                    "thickness":0.01,
                    "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":-0.001,
                    "conductivity_coefficient_b":0.0001,
                    "conductivity_coefficient_c":0.00000001,
                    "viscosity_coefficient_a":0.00001,
                    "viscosity_coefficient_b":0.00000002,
                    "viscosity_coefficient_c":-0.00000000001,
                    "specific_heat_coefficient_a":1000.0,
                    "specific_heat_coefficient_b":0.1,
                    "specific_heat_coefficient_c":-0.001,
                    "molecular_weight":20.0
                },
                "Weight 200": {
                    "gas_type":"CUSTOM",
                    "thickness":0.02,
                    "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.00002,
                    "specific_heat_coefficient_a":900.0,
                    "molecular_weight":200.0,
                    "specific_heat_ratio":1.0001
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected custom equivalent-layer gaps"))?;
    let custom = model
        .materials
        .iter()
        .find(|material| material.name.0 == "CUSTOM GAP")
        .ok_or_else(|| std::io::Error::other("missing custom equivalent-layer gap"))?
        .as_window_gap_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("expected custom gap payload"))?;
    assert_eq!(custom.gas_type, WindowGasType::Custom);
    assert_eq!(custom.gap_vent_type, WindowGapVentType::Sealed);
    assert_eq!(custom.thickness_m, 0.01);
    assert_polynomial(
        custom.properties,
        [-0.001, 0.0001, 0.00000001],
        [0.00001, 0.00000002, -0.00000000001],
        [1000.0, 0.1, -0.001],
    );
    assert_eq!(custom.properties.molecular_weight_g_per_mol, 20.0);
    assert_eq!(
        custom.properties.specific_heat_ratio, 0.0,
        "EnergyPlus 26.1 retains numeric zero for a blank custom ratio"
    );
    assert_close(custom.conductivity_at_temperature_k(300.0), 0.0299);
    assert_close(
        custom
            .nominal_thermal_resistance_m2_k_per_w()
            .ok_or_else(|| std::io::Error::other("expected custom nominal resistance"))?,
        0.01 / 0.0299,
    );

    let upper_endpoint = model
        .materials
        .iter()
        .find(|material| material.name.0 == "WEIGHT 200")
        .ok_or_else(|| std::io::Error::other("missing upper-endpoint gap"))?
        .as_window_gap_equivalent_layer()
        .ok_or_else(|| std::io::Error::other("expected upper-endpoint payload"))?;
    assert_eq!(upper_endpoint.properties.molecular_weight_g_per_mol, 200.0);
    assert_eq!(upper_endpoint.properties.specific_heat_ratio, 1.0001);

    Ok(())
}

#[test]
fn equivalent_layer_gap_enforces_schema_and_v261_source_validations() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Missing Required": {},
                "Bad Enum": {
                    "gas_type":"STEAM", "thickness":0.01, "gap_vent_type":"Sealed"
                },
                "Titlecase Enum": {
                    "gas_type":"Air", "thickness":0.01, "gap_vent_type":"Sealed"
                },
                "Lowercase Enum": {
                    "gas_type":"air", "thickness":0.01, "gap_vent_type":"Sealed"
                },
                "Blank Enum": {
                    "gas_type":"   ", "thickness":0.01, "gap_vent_type":"Sealed"
                },
                "Bad Vent": {
                    "gas_type":"AIR", "thickness":0.01, "gap_vent_type":"sealed"
                },
                "Blank Vent": {
                    "gas_type":"AIR", "thickness":0.01, "gap_vent_type":" "
                },
                "Bad Thickness": {
                    "gas_type":"AIR", "thickness":0.0, "gap_vent_type":"Sealed"
                },
                "Missing Custom": {
                    "gas_type":"CUSTOM", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":0.02
                },
                "Bad Custom Bounds": {
                    "gas_type":"CUSTOM", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.0,
                    "specific_heat_coefficient_a":0.0,
                    "molecular_weight":19.0,
                    "specific_heat_ratio":1.0
                },
                "Bad Conductivity": {
                    "gas_type":"CUSTOM", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":-0.01,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":44.0
                },
                "Bad Standard Extra": {
                    "gas_type":"AIR", "thickness":0.01, "gap_vent_type":"Sealed",
                    "viscosity_coefficient_a":0.0
                },
                "Bad Standard Type": {
                    "gas_type":"AIR", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_b":"not a number"
                },
                "Bad Upper Weight": {
                    "gas_type":"CUSTOM", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":201.0
                },
                "Zero Conductivity": {
                    "gas_type":"CUSTOM", "thickness":0.01, "gap_vent_type":"Sealed",
                    "conductivity_coefficient_a":0.0,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":44.0
                }
            }
        }"#,
    )
    .expect("invalid equivalent-layer gap cases should parse as raw epJSON");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for field in ["gas_type", "thickness", "gap_vent_type"] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "MissingRequiredField"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some("Missing Required")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing required-field diagnostic for {field}"
        );
    }
    for object_name in ["Bad Enum", "Titlecase Enum", "Lowercase Enum"] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidEnumValue"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some(object_name)
                    && diagnostic.field.as_deref() == Some("gas_type")
            }),
            "missing strict uppercase gas-type diagnostic for {object_name}"
        );
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Blank Enum")
            && diagnostic.field.as_deref() == Some("gas_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEnumValue"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Bad Vent")
            && diagnostic.field.as_deref() == Some("gap_vent_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Blank Vent")
            && diagnostic.field.as_deref() == Some("gap_vent_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Bad Thickness")
            && diagnostic.field.as_deref() == Some("thickness")
    }));
    for field in [
        "viscosity_coefficient_a",
        "specific_heat_coefficient_a",
        "molecular_weight",
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "MissingCustomWindowGapEquivalentLayerProperty"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some("Missing Custom")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing effective-required diagnostic for {field}"
        );
    }
    for field in [
        "viscosity_coefficient_a",
        "specific_heat_coefficient_a",
        "molecular_weight",
        "specific_heat_ratio",
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidNumericRange"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some("Bad Custom Bounds")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing custom-range diagnostic for {field}"
        );
    }
    for object_name in ["Bad Conductivity", "Zero Conductivity"] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidWindowGapEquivalentLayerConductivityAt300K"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some(object_name)
                    && diagnostic.field.as_deref() == Some("conductivity_coefficient_a")
            }),
            "missing non-positive k300 diagnostic for {object_name}"
        );
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Bad Standard Extra")
            && diagnostic.field.as_deref() == Some("viscosity_coefficient_a")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidFieldType"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Bad Standard Type")
            && diagnostic.field.as_deref() == Some("conductivity_coefficient_b")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("Bad Upper Weight")
            && diagnostic.field.as_deref() == Some("molecular_weight")
    }));

    let sub_runtime_minimum = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Thin Gap": {
                    "gas_type":"AIR",
                    "thickness":0.00001,
                    "gap_vent_type":"Sealed"
                }
            }
        }"#,
    )
    .expect("positive sub-runtime-minimum thickness should parse");
    let thin_result = compile_raw_model(&sub_runtime_minimum);
    assert!(
        !thin_result.has_errors(),
        "GetMaterialData accepts every positive thickness; BuildGap's later 0.0001 m replacement is outside this typed boundary: {:?}",
        thin_result.report.diagnostics
    );
}

#[test]
fn equivalent_layer_gap_follows_source_order_reports_coverage_and_shares_names()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Equivalent Gap": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                }
            },
            "WindowMaterial:Gas": {
                "Ordinary Gas": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:Glazing:EquivalentLayer": {
                "Equivalent Glass": {
                    "front_side_beam_beam_solar_transmittance":0.61,
                    "back_side_beam_beam_solar_transmittance":0.62,
                    "front_side_beam_beam_solar_reflectance":0.21,
                    "back_side_beam_beam_solar_reflectance":0.22
                }
            },
            "Material": {
                "Opaque": {
                    "roughness":"Rough",
                    "thickness":0.1,
                    "conductivity":1.0,
                    "density":1000.0,
                    "specific_heat":1000.0
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected source-ordered materials"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "OPAQUE",
            "EQUIVALENT GLASS",
            "ORDINARY GAS",
            "EQUIVALENT GAP",
        ],
        "WindowMaterial:Gap:EquivalentLayer must be read after WindowMaterial:Gas"
    );
    let gap = &model.materials[3];
    assert_eq!(gap.id, MaterialId(3));
    assert_eq!(gap.kind(), MaterialKind::WindowGapEquivalentLayer);
    assert_eq!(gap.family(), MaterialFamily::EquivalentLayer);
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing equivalent-layer gap coverage row"))?;
    assert_eq!(coverage.object_count, 1);
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);

    let duplicate = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Shared": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:Gap:EquivalentLayer": {
                "shared": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                }
            }
        }"#,
    )?;
    let duplicate_result = compile_raw_model(&duplicate);
    assert!(
        duplicate_result
            .report
            .diagnostics
            .iter()
            .any(|diagnostic| {
                diagnostic.code == "DuplicateName"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some("shared")
            })
    );

    let blank_name = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                }
            }
        }"#,
    )?;
    let blank_result = compile_raw_model(&blank_name);
    assert!(blank_result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("")
            && diagnostic.field.as_deref() == Some("name")
    }));

    Ok(())
}

#[test]
fn equivalent_layer_gap_fails_closed_in_ordinary_construction() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Equivalent Gap": {
                    "gas_type":"AIR",
                    "thickness":0.012,
                    "gap_vent_type":"Sealed"
                }
            },
            "Construction": {
                "Wrong Window": {"outside_layer":"Equivalent Gap"}
            }
        }"#,
    )
    .expect("equivalent-layer gap construction boundary should parse");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEquivalentLayerConstruction"
            && diagnostic.object_type == "Construction"
            && diagnostic.object_name.as_deref() == Some("Wrong Window")
            && diagnostic.field.as_deref() == Some("outside_layer")
    }));
}
