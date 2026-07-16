use super::super::{ObjectCoverageStatus, compile_raw_model};
use ep_model::{
    ConstructionKind, MaterialFamily, MaterialId, MaterialKind, WindowGasProperties, WindowGasType,
};
use ep_raw_model::parse_epjson_str;

fn assert_close(actual: f64, expected: f64) {
    let tolerance = f64::EPSILON * expected.abs().max(1.0) * 16.0;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}

#[test]
fn standard_window_gases_use_v261_constants_and_ignore_custom_inputs()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Air Gap": {"gas_type":"Air", "thickness":0.012},
                "Argon Gap": {
                    "gas_type":"Argon", "thickness":0.013,
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
                "Krypton Gap": {"gas_type":"Krypton", "thickness":0.014},
                "Xenon Gap": {"gas_type":"Xenon", "thickness":0.015}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected standard window gases"))?;
    let expected = [
        (
            WindowGasType::Air,
            0.012,
            [2.873e-3, 7.760e-5, 0.0],
            [3.723e-6, 4.940e-8, 0.0],
            [1002.737, 1.2324e-2, 0.0],
            28.97,
            1.4,
        ),
        (
            WindowGasType::Argon,
            0.013,
            [2.285e-3, 5.149e-5, 0.0],
            [3.379e-6, 6.451e-8, 0.0],
            [521.929, 0.0, 0.0],
            39.948,
            1.67,
        ),
        (
            WindowGasType::Krypton,
            0.014,
            [9.443e-4, 2.826e-5, 0.0],
            [2.213e-6, 7.777e-8, 0.0],
            [248.091, 0.0, 0.0],
            83.8,
            1.68,
        ),
        (
            WindowGasType::Xenon,
            0.015,
            [4.538e-4, 1.723e-5, 0.0],
            [1.069e-6, 7.414e-8, 0.0],
            [158.340, 0.0, 0.0],
            131.3,
            1.66,
        ),
    ];

    assert_eq!(model.materials.len(), expected.len());
    for (material, (gas_type, thickness, conductivity, viscosity, specific_heat, weight, ratio)) in
        model.materials.iter().zip(expected)
    {
        assert_eq!(material.kind(), MaterialKind::WindowGas);
        assert_eq!(material.family(), MaterialFamily::Fenestration);
        assert!(material.as_opaque().is_none());
        assert_eq!(material.thickness_m(), None);
        assert_eq!(material.conductivity_w_per_m_k(), None);
        assert_eq!(material.thermal_resistance(), None);
        assert_eq!(material.heat_capacity_per_area(), None);

        let gas = material
            .as_window_gas()
            .ok_or_else(|| std::io::Error::other("expected window-gas payload"))?;
        assert_eq!(gas.gas_type, gas_type);
        assert_eq!(gas.thickness_m, thickness);
        assert_polynomial(gas.properties, conductivity, viscosity, specific_heat);
        assert_eq!(gas.properties.molecular_weight_g_per_mol, weight);
        assert_eq!(gas.properties.specific_heat_ratio, ratio);
        assert_close(
            gas.nominal_thermal_resistance_m2_k_per_w()
                .ok_or_else(|| std::io::Error::other("expected nominal gas resistance"))?,
            thickness / (conductivity[0] + 300.0 * conductivity[1]),
        );
    }
    Ok(())
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
fn custom_window_gas_preserves_coefficients_and_missing_specific_heat_ratio()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Custom Gap": {
                    "gas_type":"Custom", "thickness":0.01,
                    "conductivity_coefficient_a":-0.001,
                    "conductivity_coefficient_b":0.0001,
                    "conductivity_coefficient_c":0.00000001,
                    "viscosity_coefficient_a":0.00001,
                    "viscosity_coefficient_b":0.00000002,
                    "viscosity_coefficient_c":-0.00000000001,
                    "specific_heat_coefficient_a":1000.0,
                    "specific_heat_coefficient_b":0.1,
                    "specific_heat_coefficient_c":0.001,
                    "molecular_weight":44.0,
                    "specific_heat_ratio":1.25
                },
                "Custom Gap Blank Ratio": {
                    "gas_type":"Custom", "thickness":0.02,
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":44.0
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected custom window gas"))?;
    let gas = model
        .materials
        .iter()
        .find(|material| material.name.0 == "CUSTOM GAP")
        .ok_or_else(|| std::io::Error::other("missing full custom gas"))?
        .as_window_gas()
        .ok_or_else(|| std::io::Error::other("expected custom window-gas payload"))?;
    assert_eq!(gas.gas_type, WindowGasType::Custom);
    assert_eq!(gas.thickness_m, 0.01);
    assert_polynomial(
        gas.properties,
        [-0.001, 0.0001, 0.00000001],
        [0.00001, 0.00000002, -0.00000000001],
        [1000.0, 0.1, 0.001],
    );
    assert_eq!(gas.properties.molecular_weight_g_per_mol, 44.0);
    assert_eq!(gas.properties.specific_heat_ratio, 1.25);
    let blank_ratio_gas = model
        .materials
        .iter()
        .find(|material| material.name.0 == "CUSTOM GAP BLANK RATIO")
        .ok_or_else(|| std::io::Error::other("missing blank-ratio custom gas"))?
        .as_window_gas()
        .ok_or_else(|| std::io::Error::other("expected blank-ratio custom payload"))?;
    assert_eq!(
        blank_ratio_gas.properties.specific_heat_ratio, 0.0,
        "EnergyPlus 26.1 stores the input processor's numeric zero for a missing ratio"
    );
    assert_close(gas.conductivity_at_temperature_k(300.0), 0.0299);
    assert_close(
        gas.nominal_thermal_resistance_m2_k_per_w()
            .ok_or_else(|| std::io::Error::other("expected custom nominal resistance"))?,
        0.01 / 0.0299,
    );
    Ok(())
}

#[test]
fn window_gas_enforces_schema_and_v261_source_validations() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Missing Required": {},
                "Bad Enum": {"gas_type":"Steam", "thickness":0.01},
                "Lowercase Enum": {"gas_type":"air", "thickness":0.01},
                "Blank Enum": {"gas_type":"   ", "thickness":0.01},
                "Bad Thickness": {"gas_type":"Air", "thickness":0.0},
                "Missing Custom": {
                    "gas_type":"Custom", "thickness":0.01,
                    "conductivity_coefficient_a":0.02
                },
                "Bad Custom Bounds": {
                    "gas_type":"Custom", "thickness":0.01,
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.0,
                    "specific_heat_coefficient_a":0.0,
                    "molecular_weight":19.0,
                    "specific_heat_ratio":1.0
                },
                "Bad Conductivity": {
                    "gas_type":"Custom", "thickness":0.01,
                    "conductivity_coefficient_a":-0.01,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":44.0
                },
                "Bad Standard Extra": {
                    "gas_type":"Air", "thickness":0.01,
                    "viscosity_coefficient_a":0.0
                },
                "Bad Standard Type": {
                    "gas_type":"Air", "thickness":0.01,
                    "conductivity_coefficient_b":"not a number"
                },
                "Bad Upper Weight": {
                    "gas_type":"Custom", "thickness":0.01,
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":201.0
                },
                "Zero Conductivity": {
                    "gas_type":"Custom", "thickness":0.01,
                    "conductivity_coefficient_a":0.0,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":44.0
                }
            }
        }"#,
    )
    .expect("invalid gas cases should parse as raw epJSON");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for field in ["gas_type", "thickness"] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "MissingRequiredField"
                && diagnostic.object_type == "WindowMaterial:Gas"
                && diagnostic.object_name.as_deref() == Some("Missing Required")
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEnumValue"
            && diagnostic.object_name.as_deref() == Some("Bad Enum")
            && diagnostic.field.as_deref() == Some("gas_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEnumValue"
            && diagnostic.object_name.as_deref() == Some("Lowercase Enum")
            && diagnostic.field.as_deref() == Some("gas_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_name.as_deref() == Some("Blank Enum")
            && diagnostic.field.as_deref() == Some("gas_type")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
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
                diagnostic.code == "MissingCustomWindowGasProperty"
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
                    && diagnostic.object_name.as_deref() == Some("Bad Custom Bounds")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing custom range diagnostic for {field}"
        );
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidWindowGasConductivityAt300K"
            && diagnostic.object_name.as_deref() == Some("Bad Conductivity")
            && diagnostic.field.as_deref() == Some("conductivity_coefficient_a")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_name.as_deref() == Some("Bad Standard Extra")
            && diagnostic.field.as_deref() == Some("viscosity_coefficient_a")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidFieldType"
            && diagnostic.object_name.as_deref() == Some("Bad Standard Type")
            && diagnostic.field.as_deref() == Some("conductivity_coefficient_b")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_name.as_deref() == Some("Bad Upper Weight")
            && diagnostic.field.as_deref() == Some("molecular_weight")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidWindowGasConductivityAt300K"
            && diagnostic.object_name.as_deref() == Some("Zero Conductivity")
            && diagnostic.field.as_deref() == Some("conductivity_coefficient_a")
    }));
}

#[test]
fn window_gas_accepts_molecular_weight_endpoints_and_rejects_blank_names()
-> Result<(), Box<dyn std::error::Error>> {
    let valid = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Weight 20": {
                    "gas_type":"Custom", "thickness":0.01,
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":20.0
                },
                "Weight 200": {
                    "gas_type":"Custom", "thickness":0.01,
                    "conductivity_coefficient_a":0.02,
                    "viscosity_coefficient_a":0.00001,
                    "specific_heat_coefficient_a":1000.0,
                    "molecular_weight":200.0,
                    "specific_heat_ratio":1.0001
                }
            }
        }"#,
    )?;
    let valid_result = compile_raw_model(&valid);
    assert!(
        !valid_result.has_errors(),
        "{:?}",
        valid_result.report.diagnostics
    );
    let model = valid_result
        .model
        .ok_or_else(|| std::io::Error::other("expected endpoint gases"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .filter_map(|material| material.as_window_gas())
            .map(|gas| gas.properties.molecular_weight_g_per_mol)
            .collect::<Vec<_>>(),
        vec![20.0, 200.0]
    );

    let blank_name = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "": {"gas_type":"Air", "thickness":0.01}
            }
        }"#,
    )?;
    let blank_result = compile_raw_model(&blank_name);
    assert!(blank_result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "MissingRequiredField"
            && diagnostic.object_type == "WindowMaterial:Gas"
            && diagnostic.object_name.as_deref() == Some("")
            && diagnostic.field.as_deref() == Some("name")
    }));
    Ok(())
}

#[test]
fn window_gas_follows_material_source_order_reports_typed_coverage_and_shares_names()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Gas": {
                "Gas Gap": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:Glazing:EquivalentLayer": {
                "Equivalent Glass": {
                    "front_side_beam_beam_solar_transmittance":0.61,
                    "back_side_beam_beam_solar_transmittance":0.62,
                    "front_side_beam_beam_solar_reflectance":0.21,
                    "back_side_beam_beam_solar_reflectance":0.22
                }
            },
            "WindowMaterial:Glazing:RefractionExtinctionMethod": {
                "Alternative Glass": {
                    "thickness":0.003,
                    "solar_index_of_refraction":1.5,
                    "solar_extinction_coefficient":20.0,
                    "visible_index_of_refraction":1.6,
                    "visible_extinction_coefficient":30.0
                }
            },
            "WindowMaterial:Glazing": {
                "Direct Glass": {
                    "optical_data_type":"SpectralAverage", "thickness":0.004
                }
            },
            "Material:InfraredTransparent": {"IRT": {}},
            "Material:AirGap": {"Opaque Gap": {"thermal_resistance":0.15}},
            "Material:NoMass": {
                "No Mass": {"roughness":"Rough", "thermal_resistance":0.2}
            },
            "Material": {
                "Opaque": {
                    "roughness":"Rough", "thickness":0.1, "conductivity":1.0,
                    "density":1000.0, "specific_heat":1000.0
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
            "NO MASS",
            "OPAQUE GAP",
            "IRT",
            "DIRECT GLASS",
            "ALTERNATIVE GLASS",
            "EQUIVALENT GLASS",
            "GAS GAP",
        ],
        "WindowMaterial:Gas must be read after equivalent-layer glazing"
    );
    let gas = &model.materials[7];
    assert_eq!(gas.id, MaterialId(7));
    assert_eq!(gas.kind(), MaterialKind::WindowGas);
    assert_eq!(gas.family(), MaterialFamily::Fenestration);
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == "WindowMaterial:Gas")
        .ok_or_else(|| std::io::Error::other("missing gas coverage row"))?;
    assert_eq!(coverage.object_count, 1);
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);

    let duplicate = parse_epjson_str(
        r#"{
            "Material": {
                "Shared": {
                    "roughness":"Rough", "thickness":0.1, "conductivity":1.0,
                    "density":1000.0, "specific_heat":1000.0
                }
            },
            "WindowMaterial:Gas": {
                "shared": {"gas_type":"Air", "thickness":0.012}
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
                    && diagnostic.object_type == "WindowMaterial:Gas"
                    && diagnostic.object_name.as_deref() == Some("shared")
            })
    );
    Ok(())
}

#[test]
fn window_construction_accepts_glass_gas_alternation_through_four_panes()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass A": {"optical_data_type":"SpectralAverage", "thickness":0.003},
                "Glass B": {"optical_data_type":"SpectralAverage", "thickness":0.003},
                "Glass C": {"optical_data_type":"SpectralAverage", "thickness":0.003},
                "Glass D": {"optical_data_type":"SpectralAverage", "thickness":0.003}
            },
            "WindowMaterial:Gas": {
                "Gas A": {"gas_type":"Air", "thickness":0.012},
                "Gas B": {"gas_type":"Argon", "thickness":0.012},
                "Gas C": {"gas_type":"Krypton", "thickness":0.012}
            },
            "Construction": {
                "Quad Pane": {
                    "outside_layer":"Glass A", "layer_2":"Gas A",
                    "layer_3":"Glass B", "layer_4":"Gas B",
                    "layer_5":"Glass C", "layer_6":"Gas C",
                    "layer_7":"Glass D"
                },
                "Triple Pane": {
                    "outside_layer":"Glass A", "layer_2":"Gas A",
                    "layer_3":"Glass B", "layer_4":"Gas B",
                    "layer_5":"Glass C"
                },
                "Double Pane": {
                    "outside_layer":"Glass A", "layer_2":"Gas A",
                    "layer_3":"Glass B"
                },
                "Single Pane": {"outside_layer":"Glass A"}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed window constructions"))?;
    let quad_pane = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "QUAD PANE")
        .ok_or_else(|| std::io::Error::other("missing quad-pane construction"))?;
    assert_eq!(quad_pane.kind, ConstructionKind::Fenestration);
    assert_eq!(
        quad_pane.layers,
        vec![
            MaterialId(0),
            MaterialId(4),
            MaterialId(1),
            MaterialId(5),
            MaterialId(2),
            MaterialId(6),
            MaterialId(3),
        ]
    );
    let single_pane = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "SINGLE PANE")
        .ok_or_else(|| std::io::Error::other("missing single-pane construction"))?;
    assert_eq!(single_pane.kind, ConstructionKind::Fenestration);
    assert_eq!(single_pane.layers, vec![MaterialId(0)]);
    let double_pane = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "DOUBLE PANE")
        .ok_or_else(|| std::io::Error::other("missing double-pane construction"))?;
    assert_eq!(double_pane.kind, ConstructionKind::Fenestration);
    assert_eq!(
        double_pane.layers,
        vec![MaterialId(0), MaterialId(4), MaterialId(1)]
    );
    let triple_pane = model
        .constructions
        .iter()
        .find(|construction| construction.name.0 == "TRIPLE PANE")
        .ok_or_else(|| std::io::Error::other("missing triple-pane construction"))?;
    assert_eq!(triple_pane.kind, ConstructionKind::Fenestration);
    assert_eq!(
        triple_pane.layers,
        vec![
            MaterialId(0),
            MaterialId(4),
            MaterialId(1),
            MaterialId(5),
            MaterialId(2),
        ]
    );
    Ok(())
}

#[test]
fn window_construction_rejects_non_alternating_or_overlong_glass_gas_stacks() {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass A": {"optical_data_type":"SpectralAverage", "thickness":0.003},
                "Glass B": {"optical_data_type":"SpectralAverage", "thickness":0.003},
                "Glass C": {"optical_data_type":"SpectralAverage", "thickness":0.003},
                "Glass D": {"optical_data_type":"SpectralAverage", "thickness":0.003}
            },
            "WindowMaterial:Gas": {
                "Gas A": {"gas_type":"Air", "thickness":0.012},
                "Gas B": {"gas_type":"Argon", "thickness":0.012},
                "Gas C": {"gas_type":"Krypton", "thickness":0.012},
                "Gas D": {"gas_type":"Xenon", "thickness":0.012}
            },
            "Construction": {
                "Starts Gas": {"outside_layer":"Gas A"},
                "Starts Gas Then Glass": {
                    "outside_layer":"Gas A", "layer_2":"Glass A"
                },
                "Ends Gas": {"outside_layer":"Glass A", "layer_2":"Gas A"},
                "Adjacent Glass": {"outside_layer":"Glass A", "layer_2":"Glass B"},
                "Adjacent Gas": {
                    "outside_layer":"Glass A", "layer_2":"Gas A", "layer_3":"Gas B"
                },
                "Too Many": {
                    "outside_layer":"Glass A", "layer_2":"Gas A",
                    "layer_3":"Glass B", "layer_4":"Gas B",
                    "layer_5":"Glass C", "layer_6":"Gas C",
                    "layer_7":"Glass D", "layer_8":"Gas D"
                }
            }
        }"#,
    )
    .expect("invalid window constructions should parse as raw epJSON");
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for (object_name, field) in [
        ("Starts Gas", "outside_layer"),
        ("Starts Gas Then Glass", "outside_layer"),
        ("Ends Gas", "layer_2"),
        ("Adjacent Glass", "layer_2"),
        ("Adjacent Gas", "layer_3"),
        ("Too Many", "layer_8"),
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "InvalidWindowConstructionLayering"
                    && diagnostic.object_type == "Construction"
                    && diagnostic.object_name.as_deref() == Some(object_name)
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing invalid window-layer diagnostic for {object_name}/{field}"
        );
    }
    let adjacent_glass = result
        .report
        .diagnostics
        .iter()
        .find(|diagnostic| {
            diagnostic.code == "InvalidWindowConstructionLayering"
                && diagnostic.object_name.as_deref() == Some("Adjacent Glass")
        })
        .expect("adjacent-glass diagnostic must exist");
    assert!(
        adjacent_glass
            .message
            .contains("must be a WindowMaterial:Gas gap")
    );
}
