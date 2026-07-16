use super::super::{ObjectCoverageStatus, compile_raw_model};
use ep_model::{
    ConstructionKind, Material, MaterialDefinition, MaterialFamily, MaterialId, MaterialKind,
    WindowGasMixture, WindowGasMixtureMaterial, WindowGasProperties, WindowStandardGasType,
};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "WindowMaterial:GasMixture";

fn assert_close(actual: f64, expected: f64) {
    let tolerance = f64::EPSILON * expected.abs().max(1.0) * 16.0;
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {expected}, got {actual}"
    );
}

fn mixture<'a>(
    materials: &'a [Material],
    name: &str,
) -> Result<&'a WindowGasMixtureMaterial, std::io::Error> {
    let material = materials
        .iter()
        .find(|material| material.name.0 == name)
        .ok_or_else(|| std::io::Error::other(format!("missing material {name}")))?;
    material
        .as_window_gas_mixture()
        .ok_or_else(|| std::io::Error::other(format!("{name} is not a gas mixture")))
}

fn assert_standard_properties(gas_type: WindowStandardGasType, properties: WindowGasProperties) {
    let expected = match gas_type {
        WindowStandardGasType::Air => (
            [2.873e-3, 7.760e-5, 0.0],
            [3.723e-6, 4.940e-8, 0.0],
            [1002.737, 1.2324e-2, 0.0],
            28.97,
            1.4,
        ),
        WindowStandardGasType::Argon => (
            [2.285e-3, 5.149e-5, 0.0],
            [3.379e-6, 6.451e-8, 0.0],
            [521.929, 0.0, 0.0],
            39.948,
            1.67,
        ),
        WindowStandardGasType::Krypton => (
            [9.443e-4, 2.826e-5, 0.0],
            [2.213e-6, 7.777e-8, 0.0],
            [248.091, 0.0, 0.0],
            83.8,
            1.68,
        ),
        WindowStandardGasType::Xenon => (
            [4.538e-4, 1.723e-5, 0.0],
            [1.069e-6, 7.414e-8, 0.0],
            [158.340, 0.0, 0.0],
            131.3,
            1.66,
        ),
    };
    assert_eq!(
        [
            properties.conductivity.coefficient_a,
            properties.conductivity.coefficient_b,
            properties.conductivity.coefficient_c,
        ],
        expected.0
    );
    assert_eq!(
        [
            properties.viscosity.coefficient_a,
            properties.viscosity.coefficient_b,
            properties.viscosity.coefficient_c,
        ],
        expected.1
    );
    assert_eq!(
        [
            properties.specific_heat.coefficient_a,
            properties.specific_heat.coefficient_b,
            properties.specific_heat.coefficient_c,
        ],
        expected.2
    );
    assert_eq!(properties.molecular_weight_g_per_mol, expected.3);
    assert_eq!(properties.specific_heat_ratio, expected.4);
}

#[test]
fn gas_mixture_encodes_counts_one_through_four_and_preserves_active_order()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:GasMixture": {
                "One": {
                    "thickness":0.010, "number_of_gases_in_mixture":1,
                    "gas_1_type":"Xenon", "gas_1_fraction":0.25,
                    "gas_2_type":"Argon", "gas_2_fraction":0.75
                },
                "Two Nonunit": {
                    "thickness":0.011, "number_of_gases_in_mixture":2,
                    "gas_1_type":"Air", "gas_1_fraction":0.60,
                    "gas_2_type":"Krypton", "gas_2_fraction":0.20
                },
                "Three Duplicate Blank Fraction": {
                    "thickness":0.012, "number_of_gases_in_mixture":3,
                    "gas_1_type":"Argon", "gas_1_fraction":0.40,
                    "gas_2_type":"Argon", "gas_2_fraction":0.40,
                    "gas_3_type":"Air"
                },
                "Four": {
                    "thickness":0.013, "number_of_gases_in_mixture":4,
                    "gas_1_type":"Xenon", "gas_1_fraction":0.10,
                    "gas_2_type":"Krypton", "gas_2_fraction":0.20,
                    "gas_3_type":"Argon", "gas_3_fraction":0.30,
                    "gas_4_type":"Air"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed gas mixtures"))?;

    let one = mixture(&model.materials, "ONE")?;
    assert!(matches!(one.gases, WindowGasMixture::One(_)));
    assert_eq!(one.gases.len(), 1);
    assert_eq!(
        one.gases.components()[0].gas_type,
        WindowStandardGasType::Xenon
    );
    assert_eq!(one.gases.components()[0].fraction, 0.25);
    assert!(
        one.gases
            .components()
            .iter()
            .all(|component| component.gas_type != WindowStandardGasType::Argon),
        "the schema-required gas-2 pair is validated but discarded when N=1"
    );

    let two = mixture(&model.materials, "TWO NONUNIT")?;
    assert!(matches!(two.gases, WindowGasMixture::Two(_)));
    assert_eq!(
        two.gases
            .components()
            .iter()
            .map(|component| (component.gas_type, component.fraction))
            .collect::<Vec<_>>(),
        vec![
            (WindowStandardGasType::Air, 0.60),
            (WindowStandardGasType::Krypton, 0.20),
        ],
        "non-unit fraction sums are preserved without normalization"
    );

    let three = mixture(&model.materials, "THREE DUPLICATE BLANK FRACTION")?;
    assert!(matches!(three.gases, WindowGasMixture::Three(_)));
    assert_eq!(
        three
            .gases
            .components()
            .iter()
            .map(|component| (component.gas_type, component.fraction))
            .collect::<Vec<_>>(),
        vec![
            (WindowStandardGasType::Argon, 0.40),
            (WindowStandardGasType::Argon, 0.40),
            (WindowStandardGasType::Air, 0.0),
        ],
        "duplicates are retained and a missing active optional fraction keeps the source numeric zero"
    );

    let four = mixture(&model.materials, "FOUR")?;
    assert!(matches!(four.gases, WindowGasMixture::Four(_)));
    assert_eq!(
        four.gases
            .components()
            .iter()
            .map(|component| (component.gas_type, component.fraction))
            .collect::<Vec<_>>(),
        vec![
            (WindowStandardGasType::Xenon, 0.10),
            (WindowStandardGasType::Krypton, 0.20),
            (WindowStandardGasType::Argon, 0.30),
            (WindowStandardGasType::Air, 0.0),
        ],
        "a missing active fourth fraction keeps the source numeric zero"
    );

    for material in &model.materials {
        assert_eq!(material.kind(), MaterialKind::WindowGasMixture);
        assert_eq!(material.family(), MaterialFamily::Fenestration);
        assert!(material.as_opaque().is_none());
        assert!(material.as_window_gap_equivalent_layer().is_none());
        let mixture = material
            .as_window_gas_mixture()
            .ok_or_else(|| std::io::Error::other("expected mixture payload"))?;
        for component in mixture.gases.components() {
            assert_eq!(
                WindowStandardGasType::from_energyplus_name(component.gas_type.energyplus_name()),
                Some(component.gas_type)
            );
            assert_standard_properties(component.gas_type, component.properties());
        }
    }

    Ok(())
}

#[test]
fn gas_mixture_nominal_resistance_uses_only_first_gas_at_300_k()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:GasMixture": {
                "Air First": {
                    "thickness":0.012, "number_of_gases_in_mixture":2,
                    "gas_1_type":"Air", "gas_1_fraction":0.01,
                    "gas_2_type":"Xenon", "gas_2_fraction":0.99
                },
                "Xenon First": {
                    "thickness":0.012, "number_of_gases_in_mixture":2,
                    "gas_1_type":"Xenon", "gas_1_fraction":0.99,
                    "gas_2_type":"Air", "gas_2_fraction":0.01
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected gas mixtures"))?;
    let air_first = mixture(&model.materials, "AIR FIRST")?;
    let xenon_first = mixture(&model.materials, "XENON FIRST")?;
    let air_k300 = 2.873e-3 + 7.760e-5 * 300.0;
    let xenon_k300 = 4.538e-4 + 1.723e-5 * 300.0;
    assert_close(
        air_first
            .nominal_thermal_resistance_m2_k_per_w()
            .ok_or_else(|| std::io::Error::other("expected air-first nominal R"))?,
        0.012 / air_k300,
    );
    assert_close(
        xenon_first
            .nominal_thermal_resistance_m2_k_per_w()
            .ok_or_else(|| std::io::Error::other("expected xenon-first nominal R"))?,
        0.012 / xenon_k300,
    );
    assert_ne!(
        air_first.nominal_thermal_resistance_m2_k_per_w(),
        xenon_first.nominal_thermal_resistance_m2_k_per_w(),
        "later gases and fractions must not influence the source-order nominal R shortcut"
    );
    Ok(())
}

#[test]
fn gas_mixture_validates_schema_bounds_active_types_and_inactive_inputs()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:GasMixture": {
                "Missing Required": {},
                "Count Zero": {
                    "thickness":0.01, "number_of_gases_in_mixture":0,
                    "gas_1_type":"Air", "gas_1_fraction":0.5,
                    "gas_2_type":"Argon", "gas_2_fraction":0.5
                },
                "Count Five": {
                    "thickness":0.01, "number_of_gases_in_mixture":5,
                    "gas_1_type":"Air", "gas_1_fraction":0.5,
                    "gas_2_type":"Argon", "gas_2_fraction":0.5
                },
                "Count Fractional": {
                    "thickness":0.01, "number_of_gases_in_mixture":2.5,
                    "gas_1_type":"Air", "gas_1_fraction":0.5,
                    "gas_2_type":"Argon", "gas_2_fraction":0.5
                },
                "Zero Thickness": {
                    "thickness":0.0, "number_of_gases_in_mixture":2,
                    "gas_1_type":"Air", "gas_1_fraction":0.5,
                    "gas_2_type":"Argon", "gas_2_fraction":0.5
                },
                "Custom Rejected": {
                    "thickness":0.01, "number_of_gases_in_mixture":2,
                    "gas_1_type":"Custom", "gas_1_fraction":0.5,
                    "gas_2_type":"Argon", "gas_2_fraction":0.5
                },
                "Lowercase Rejected": {
                    "thickness":0.01, "number_of_gases_in_mixture":2,
                    "gas_1_type":"air", "gas_1_fraction":0.5,
                    "gas_2_type":"Argon", "gas_2_fraction":0.5
                },
                "Missing Dummy Gas Two": {
                    "thickness":0.01, "number_of_gases_in_mixture":1,
                    "gas_1_type":"Air", "gas_1_fraction":1.0
                },
                "Missing Active Three Type": {
                    "thickness":0.01, "number_of_gases_in_mixture":3,
                    "gas_1_type":"Air", "gas_1_fraction":0.3,
                    "gas_2_type":"Argon", "gas_2_fraction":0.3,
                    "gas_3_fraction":0.4
                },
                "Missing Active Four Type": {
                    "thickness":0.01, "number_of_gases_in_mixture":4,
                    "gas_1_type":"Air", "gas_1_fraction":0.25,
                    "gas_2_type":"Argon", "gas_2_fraction":0.25,
                    "gas_3_type":"Krypton", "gas_3_fraction":0.25,
                    "gas_4_fraction":0.25
                },
                "Explicit Zero Active Three Fraction": {
                    "thickness":0.01, "number_of_gases_in_mixture":3,
                    "gas_1_type":"Air", "gas_1_fraction":0.5,
                    "gas_2_type":"Argon", "gas_2_fraction":0.5,
                    "gas_3_type":"Krypton", "gas_3_fraction":0.0
                },
                "Fraction Above One": {
                    "thickness":0.01, "number_of_gases_in_mixture":2,
                    "gas_1_type":"Air", "gas_1_fraction":1.01,
                    "gas_2_type":"Argon", "gas_2_fraction":0.5
                },
                "Invalid Inactive Type": {
                    "thickness":0.01, "number_of_gases_in_mixture":1,
                    "gas_1_type":"Air", "gas_1_fraction":1.0,
                    "gas_2_type":"Argon", "gas_2_fraction":1.0,
                    "gas_4_type":"Steam"
                },
                "Invalid Inactive Fraction": {
                    "thickness":0.01, "number_of_gases_in_mixture":1,
                    "gas_1_type":"Air", "gas_1_fraction":1.0,
                    "gas_2_type":"Argon", "gas_2_fraction":1.0,
                    "gas_3_fraction":0.0
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(result.has_errors());
    for field in [
        "thickness",
        "number_of_gases_in_mixture",
        "gas_1_type",
        "gas_1_fraction",
        "gas_2_type",
        "gas_2_fraction",
    ] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "MissingRequiredField"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some("Missing Required")
                    && diagnostic.field.as_deref() == Some(field)
            }),
            "missing schema-required diagnostic for {field}"
        );
    }
    for object_name in ["Count Zero", "Count Five"] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidNumericRange"
                && diagnostic.object_name.as_deref() == Some(object_name)
                && diagnostic.field.as_deref() == Some("number_of_gases_in_mixture")
        }));
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidInteger"
            && diagnostic.object_name.as_deref() == Some("Count Fractional")
            && diagnostic.field.as_deref() == Some("number_of_gases_in_mixture")
    }));
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidNumericRange"
            && diagnostic.object_name.as_deref() == Some("Zero Thickness")
            && diagnostic.field.as_deref() == Some("thickness")
    }));
    for object_name in ["Custom Rejected", "Lowercase Rejected"] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidEnumValue"
                && diagnostic.object_name.as_deref() == Some(object_name)
                && diagnostic.field.as_deref() == Some("gas_1_type")
        }));
    }
    for field in ["gas_2_type", "gas_2_fraction"] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "MissingRequiredField"
                && diagnostic.object_name.as_deref() == Some("Missing Dummy Gas Two")
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
    for (object_name, field) in [
        ("Missing Active Three Type", "gas_3_type"),
        ("Missing Active Four Type", "gas_4_type"),
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "MissingActiveWindowGasMixtureType"
                && diagnostic.object_name.as_deref() == Some(object_name)
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
    for (object_name, field) in [
        ("Explicit Zero Active Three Fraction", "gas_3_fraction"),
        ("Fraction Above One", "gas_1_fraction"),
        ("Invalid Inactive Fraction", "gas_3_fraction"),
    ] {
        assert!(result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidNumericRange"
                && diagnostic.object_name.as_deref() == Some(object_name)
                && diagnostic.field.as_deref() == Some(field)
        }));
    }
    assert!(result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidEnumValue"
            && diagnostic.object_name.as_deref() == Some("Invalid Inactive Type")
            && diagnostic.field.as_deref() == Some("gas_4_type")
    }));
    Ok(())
}

#[test]
fn gas_mixture_discards_valid_inactive_fields_but_validates_them_first()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:GasMixture": {
                "Inactive Valid": {
                    "thickness":0.01, "number_of_gases_in_mixture":1,
                    "gas_1_type":"Air", "gas_1_fraction":0.7,
                    "gas_2_type":"Argon", "gas_2_fraction":0.3,
                    "gas_3_type":"Krypton", "gas_3_fraction":0.8,
                    "gas_4_type":"Xenon", "gas_4_fraction":0.9
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected valid inactive inputs"))?;
    let gases = mixture(&model.materials, "INACTIVE VALID")?
        .gases
        .components();
    assert_eq!(gases.len(), 1);
    assert_eq!(gases[0].gas_type, WindowStandardGasType::Air);
    assert_eq!(gases[0].fraction, 0.7);
    Ok(())
}

#[test]
fn gas_mixture_follows_source_order_shares_names_and_validates_construction_layers()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Construction": {
                "Good Window": {
                    "outside_layer":"Glass", "layer_2":"Mixture", "layer_3":"Glass"
                }
            },
            "WindowMaterial:GasMixture": {
                "Mixture": {
                    "thickness":0.012, "number_of_gases_in_mixture":2,
                    "gas_1_type":"Air", "gas_1_fraction":0.5,
                    "gas_2_type":"Argon", "gas_2_fraction":0.5
                }
            },
            "WindowMaterial:Gap:EquivalentLayer": {
                "Equivalent Gap": {
                    "gas_type":"AIR", "thickness":0.012, "gap_vent_type":"Sealed"
                }
            },
            "WindowMaterial:Gas": {
                "Single Gap": {"gas_type":"Air", "thickness":0.012}
            },
            "WindowMaterial:Glazing": {
                "Glass": {"optical_data_type":"SpectralAverage", "thickness":0.006}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw_model);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected source-ordered material model"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec!["GLASS", "SINGLE GAP", "EQUIVALENT GAP", "MIXTURE"],
        "GasMixture must compile immediately after Gap:EquivalentLayer"
    );
    let gas_mixture = model
        .materials
        .iter()
        .find(|material| material.name.0 == "MIXTURE")
        .ok_or_else(|| std::io::Error::other("missing mixture"))?;
    assert_eq!(gas_mixture.id, MaterialId(3));
    assert_eq!(gas_mixture.kind(), MaterialKind::WindowGasMixture);
    assert_eq!(gas_mixture.family(), MaterialFamily::Fenestration);
    assert!(matches!(
        gas_mixture.definition,
        MaterialDefinition::WindowGasMixture(_)
    ));
    assert_eq!(model.constructions.len(), 1);
    assert_eq!(model.constructions[0].kind, ConstructionKind::Fenestration);
    assert_eq!(model.constructions[0].layers.len(), 3);

    let coverage = result
        .report
        .coverage
        .iter()
        .find(|entry| entry.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing gas-mixture coverage"))?;
    assert_eq!(coverage.object_count, 1);
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);

    let duplicate = parse_epjson_str(
        r#"{
            "WindowMaterial:Gap:EquivalentLayer": {
                "Shared": {
                    "gas_type":"AIR", "thickness":0.012, "gap_vent_type":"Sealed"
                }
            },
            "WindowMaterial:GasMixture": {
                "shared": {
                    "thickness":0.012, "number_of_gases_in_mixture":1,
                    "gas_1_type":"Air", "gas_1_fraction":1.0,
                    "gas_2_type":"Argon", "gas_2_fraction":1.0
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

    let invalid_layers = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "Glass": {"optical_data_type":"SpectralAverage", "thickness":0.006}
            },
            "WindowMaterial:GasMixture": {
                "Mixture": {
                    "thickness":0.012, "number_of_gases_in_mixture":2,
                    "gas_1_type":"Air", "gas_1_fraction":0.5,
                    "gas_2_type":"Argon", "gas_2_fraction":0.5
                }
            },
            "Construction": {
                "Gas Outside": {"outside_layer":"Mixture"},
                "Trailing Gas": {"outside_layer":"Glass", "layer_2":"Mixture"}
            }
        }"#,
    )?;
    let invalid_result = compile_raw_model(&invalid_layers);
    assert!(invalid_result.has_errors());
    for (construction, field) in [
        ("Gas Outside", "outside_layer"),
        ("Trailing Gas", "layer_2"),
    ] {
        assert!(invalid_result.report.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "InvalidWindowConstructionLayering"
                && diagnostic.object_name.as_deref() == Some(construction)
                && diagnostic.field.as_deref() == Some(field)
        }));
    }

    Ok(())
}
