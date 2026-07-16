use super::super::{
    CompileResult, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{MaterialFamily, MaterialKind, MaterialSurfaceRoughness};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "WindowMaterial:SimpleGlazingSystem";

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() <= 1.0e-12,
        "expected {expected:.15}, got {actual:.15}"
    );
}

fn has_diagnostic(
    result: &CompileResult,
    severity: DiagnosticSeverity,
    code: &str,
    object_type: &str,
    object_name: &str,
    field: &str,
) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == severity
            && diagnostic.code == code
            && diagnostic.object_type == object_type
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == Some(field)
    })
}

fn simple_glazing_material<'a>(
    model: &'a ep_model::TypedModel,
    normalized_name: &str,
) -> Result<&'a ep_model::WindowSimpleGlazingMaterial, std::io::Error> {
    model
        .materials
        .iter()
        .find(|material| material.name.0 == normalized_name)
        .and_then(ep_model::Material::as_window_simple_glazing)
        .ok_or_else(|| std::io::Error::other(format!("missing {normalized_name}")))
}

#[test]
fn simple_glazing_system_materializes_source_exact_block_model_and_optional_visible_input()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:Glazing": {
                "TC Child": {
                    "optical_data_type":"SpectralAverage",
                    "thickness":0.004
                }
            },
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "Earlier TC Parent": {
                    "temperature_data":[{
                        "optical_data_temperature":20.0,
                        "window_material_glazing_name":"TC Child"
                    }]
                }
            },
            "WindowMaterial:SimpleGlazingSystem": {
                "Z High U": {
                    "u_factor":100.0,
                    "solar_heat_gain_coefficient":0.4,
                    "visible_transmittance":0.6
                },
                "C Default Visible": {
                    "u_factor":3.0,
                    "solar_heat_gain_coefficient":0.5
                },
                "B Explicit Visible": {
                    "u_factor":5.0,
                    "solar_heat_gain_coefficient":0.7,
                    "visible_transmittance":0.4
                },
                "A Interpolated Boundary": {
                    "u_factor":4.5,
                    "solar_heat_gain_coefficient":0.5
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
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Warning,
        "SimpleGlazingFilmResistanceClamped",
        OBJECT_TYPE,
        "Z High U",
        "u_factor"
    ));
    assert_eq!(
        result
            .report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == DiagnosticSeverity::Warning)
            .count(),
        1,
        "only the source high-U film-resistance warning is expected"
    );

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected typed simple glazing materials"))?;
    assert_eq!(
        model
            .materials
            .iter()
            .map(|material| material.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "TC CHILD",
            "EARLIER TC PARENT",
            "A INTERPOLATED BOUNDARY",
            "B EXPLICIT VISIBLE",
            "C DEFAULT VISIBLE",
            "Z HIGH U",
        ],
        "simple glazing definitions must follow the thermochromic family and retain effective object order"
    );

    for material in &model.materials[2..] {
        assert_eq!(material.kind(), MaterialKind::WindowSimpleGlazing);
        assert_eq!(material.family(), MaterialFamily::SimpleGlazing);
        assert!(material.as_opaque().is_none());
    }

    let interpolated = model.materials[2]
        .as_window_simple_glazing()
        .ok_or_else(|| std::io::Error::other("missing interpolated simple glazing payload"))?;
    assert_eq!(
        interpolated.input_visible_transmittance_at_normal_incidence,
        None
    );
    assert_eq!(interpolated.roughness, MaterialSurfaceRoughness::VerySmooth);
    assert!(!interpolated.film_resistance_clamped);
    assert_close(
        interpolated.thermal_resistance_m2_k_per_w,
        0.054_555_205_776_439_4,
    );
    assert_close(interpolated.thickness_m, 0.002);
    assert_close(interpolated.conductivity_w_per_m_k, 0.036_660_112_844_148_3);
    assert_close(
        interpolated.solar_transmittance_at_normal_incidence,
        0.336_659_5,
    );
    assert_close(
        interpolated.front_side_solar_reflectance_at_normal_incidence,
        0.167_576_322_956_248,
    );
    assert_eq!(
        interpolated.front_side_solar_reflectance_at_normal_incidence,
        interpolated.back_side_solar_reflectance_at_normal_incidence
    );
    assert_eq!(
        interpolated.visible_transmittance_at_normal_incidence,
        interpolated.solar_transmittance_at_normal_incidence
    );
    assert_eq!(
        interpolated.front_side_visible_reflectance_at_normal_incidence,
        interpolated.front_side_solar_reflectance_at_normal_incidence
    );

    let explicit = model.materials[3]
        .as_window_simple_glazing()
        .ok_or_else(|| std::io::Error::other("missing explicit-VT simple glazing payload"))?;
    assert_eq!(
        explicit.input_visible_transmittance_at_normal_incidence,
        Some(0.4)
    );
    assert_close(
        explicit.thermal_resistance_m2_k_per_w,
        0.033_018_740_083_688_2,
    );
    assert_close(explicit.thickness_m, 0.002);
    assert_close(explicit.conductivity_w_per_m_k, 0.060_571_663_089_835_3);
    assert_close(
        explicit.solar_transmittance_at_normal_incidence,
        0.602_923_02,
    );
    assert_close(
        explicit.front_side_solar_reflectance_at_normal_incidence,
        0.092_199_993_574_043_4,
    );
    assert_close(explicit.visible_transmittance_at_normal_incidence, 0.4);
    assert_close(
        explicit.front_side_visible_reflectance_at_normal_incidence,
        0.137_591_2,
    );
    assert_close(
        explicit.back_side_visible_reflectance_at_normal_incidence,
        0.179_618_4,
    );

    let default_visible = model.materials[4]
        .as_window_simple_glazing()
        .ok_or_else(|| std::io::Error::other("missing default-VT simple glazing payload"))?;
    assert_eq!(
        default_visible.input_visible_transmittance_at_normal_incidence,
        None
    );
    assert_close(
        default_visible.thermal_resistance_m2_k_per_w,
        0.162_975_248_660_525,
    );
    assert_close(default_visible.thickness_m, 0.015_329_666_475_843_1);
    assert_close(
        default_visible.conductivity_w_per_m_k,
        0.094_061_316_683_581_7,
    );
    assert_close(
        default_visible.solar_transmittance_at_normal_incidence,
        0.418_462_75,
    );
    assert_close(
        default_visible.front_side_solar_reflectance_at_normal_incidence,
        0.368_380_443_637_332,
    );

    let high_u = model.materials[5]
        .as_window_simple_glazing()
        .ok_or_else(|| std::io::Error::other("missing high-U simple glazing payload"))?;
    assert_eq!(
        high_u.u_factor_with_film_coefficients_w_per_m2_k, 100.0,
        "the schema has no U-factor maximum"
    );
    assert!(high_u.film_resistance_clamped);
    assert_close(high_u.thermal_resistance_m2_k_per_w, 0.001);
    assert_close(high_u.thickness_m, 0.002);
    assert_close(high_u.conductivity_w_per_m_k, 2.0);
    assert_close(
        high_u.solar_transmittance_at_normal_incidence,
        0.231_727_680_000_000_05,
    );
    assert_close(
        high_u.front_side_solar_reflectance_at_normal_incidence,
        0.193_433_983_866_041_71,
    );
    assert_close(high_u.visible_transmittance_at_normal_incidence, 0.6);
    assert_close(
        high_u.front_side_visible_reflectance_at_normal_incidence,
        0.130_296_800_000_000_02,
    );
    assert_close(
        high_u.back_side_visible_reflectance_at_normal_incidence,
        0.151_641_600_000_000_04,
    );
    Ok(())
}

#[test]
fn simple_glazing_system_preserves_source_branch_boundaries_and_open_interval_inputs()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:SimpleGlazingSystem": {
                "A U Just Below 3.4": {"u_factor":3.3999999,"solar_heat_gain_coefficient":0.5},
                "B U At 3.4": {"u_factor":3.4,"solar_heat_gain_coefficient":0.5},
                "C U Just Above 3.4": {"u_factor":3.4000001,"solar_heat_gain_coefficient":0.5},
                "D U Just Below 4.5": {"u_factor":4.4999999,"solar_heat_gain_coefficient":0.5},
                "E U At 4.5": {"u_factor":4.5,"solar_heat_gain_coefficient":0.5},
                "F U Just Above 4.5": {"u_factor":4.5000001,"solar_heat_gain_coefficient":0.5},
                "G U Just Below 5.85": {"u_factor":5.8499999,"solar_heat_gain_coefficient":0.5},
                "H U At 5.85": {"u_factor":5.85,"solar_heat_gain_coefficient":0.5},
                "I U Just Above 5.85": {"u_factor":5.8500001,"solar_heat_gain_coefficient":0.5},
                "J SHGC Just Below .15": {"u_factor":3.0,"solar_heat_gain_coefficient":0.1499999},
                "K SHGC At .15": {"u_factor":3.0,"solar_heat_gain_coefficient":0.15},
                "L SHGC Just Above .15": {"u_factor":3.0,"solar_heat_gain_coefficient":0.1500001},
                "M SHGC Just Below .7206": {"u_factor":5.0,"solar_heat_gain_coefficient":0.7205999},
                "N SHGC At .7206": {"u_factor":5.0,"solar_heat_gain_coefficient":0.7206},
                "O SHGC Just Above .7206": {"u_factor":5.0,"solar_heat_gain_coefficient":0.7206001},
                "P Near Lower Bounds": {
                    "u_factor":1.0,
                    "solar_heat_gain_coefficient":0.000001,
                    "visible_transmittance":0.000001
                },
                "Q Near Upper Bounds": {
                    "u_factor":1.0,
                    "solar_heat_gain_coefficient":0.999999,
                    "visible_transmittance":0.999999
                },
                "R Visible Reflectance Saturation": {
                    "u_factor":3.0,
                    "solar_heat_gain_coefficient":0.5,
                    "visible_transmittance":0.99
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
        .ok_or_else(|| std::io::Error::other("expected branch-boundary materials"))?;

    for (name, expected_transmittance, expected_reflectance) in [
        ("A U JUST BELOW 3.4", 0.418_462_75, 0.359_635_819_765_011_97),
        ("B U AT 3.4", 0.418_462_75, 0.359_635_817_502_211_7),
        (
            "C U JUST ABOVE 3.4",
            0.418_462_742_563_340_9,
            0.359_635_802_802_171_56,
        ),
        (
            "D U JUST BELOW 4.5",
            0.336_659_507_436_659_1,
            0.167_576_343_564_667_94,
        ),
        ("E U AT 4.5", 0.336_659_5, 0.167_576_322_956_248_02),
        ("F U JUST ABOVE 4.5", 0.336_659_5, 0.187_293_345_603_192_05),
    ] {
        let material = simple_glazing_material(model, name)?;
        assert_close(
            material.solar_transmittance_at_normal_incidence,
            expected_transmittance,
        );
        assert_close(
            material.front_side_solar_reflectance_at_normal_incidence,
            expected_reflectance,
        );
    }

    let just_above_4_5 = simple_glazing_material(model, "F U JUST ABOVE 4.5")?;
    assert_close(
        just_above_4_5.thermal_resistance_m2_k_per_w,
        0.054_555_200_983_359_725,
    );
    assert_close(just_above_4_5.thickness_m, 0.002);
    assert_close(
        just_above_4_5.conductivity_w_per_m_k,
        0.036_660_116_065_011_56,
    );
    assert!(
        just_above_4_5.front_side_solar_reflectance_at_normal_incidence
            - simple_glazing_material(model, "E U AT 4.5")?
                .front_side_solar_reflectance_at_normal_incidence
            > 0.019,
        "the source's 4.5/just-above-4.5 summer-film discontinuity must remain visible"
    );

    for (name, expected_resistance) in [
        ("G U JUST BELOW 5.85", 0.004_971_450_242_963_679),
        ("H U AT 5.85", 0.004_783_738_829_763_712),
        ("I U JUST ABOVE 5.85", 0.004_783_739_028_069_375),
    ] {
        assert_close(
            simple_glazing_material(model, name)?.thermal_resistance_m2_k_per_w,
            expected_resistance,
        );
    }

    for (name, expected_transmittance, expected_reflectance) in [
        (
            "J SHGC JUST BELOW .15",
            0.061_559_958_96,
            0.707_793_509_903_622_6,
        ),
        ("K SHGC AT .15", 0.061_56, 0.707_793_319_633_981_1),
        (
            "L SHGC JUST ABOVE .15",
            0.061_565_136_468_650_86,
            0.707_800_930_637_633_8,
        ),
        (
            "M SHGC JUST BELOW .7206",
            0.634_619_696_066_777_6,
            0.094_043_578_129_855_37,
        ),
        (
            "N SHGC AT .7206",
            0.634_620_490_000_000_1,
            0.094_044_889_277_172_64,
        ),
        (
            "O SHGC JUST ABOVE .7206",
            0.634_620_620_415,
            0.094_044_851_127_592_87,
        ),
    ] {
        let material = simple_glazing_material(model, name)?;
        assert_close(
            material.solar_transmittance_at_normal_incidence,
            expected_transmittance,
        );
        assert_close(
            material.front_side_solar_reflectance_at_normal_incidence,
            expected_reflectance,
        );
    }

    for (name, expected) in [
        ("P NEAR LOWER BOUNDS", 0.000_001),
        ("Q NEAR UPPER BOUNDS", 0.999_999),
    ] {
        let material = simple_glazing_material(model, name)?;
        assert_close(material.solar_heat_gain_coefficient, expected);
        assert_eq!(
            material.input_visible_transmittance_at_normal_incidence,
            Some(expected)
        );
        assert_close(material.visible_transmittance_at_normal_incidence, expected);
    }

    let saturated_visible_reflectance =
        simple_glazing_material(model, "R VISIBLE REFLECTANCE SATURATION")?;
    assert_close(
        saturated_visible_reflectance.front_side_visible_reflectance_at_normal_incidence,
        0.009,
    );
    assert_close(
        saturated_visible_reflectance.back_side_visible_reflectance_at_normal_incidence,
        0.009,
    );
    Ok(())
}

#[test]
fn simple_glazing_system_enforces_required_types_and_exclusive_bounds()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:SimpleGlazingSystem": {
                "A Missing U": {"solar_heat_gain_coefficient":0.5},
                "B Missing SHGC": {"u_factor":3.0},
                "C Wrong U": {"u_factor":"three","solar_heat_gain_coefficient":0.5},
                "D Wrong SHGC": {"u_factor":3.0,"solar_heat_gain_coefficient":"half"},
                "E Wrong VT": {
                    "u_factor":3.0,
                    "solar_heat_gain_coefficient":0.5,
                    "visible_transmittance":"half"
                },
                "F Zero U": {"u_factor":0.0,"solar_heat_gain_coefficient":0.5},
                "G Zero SHGC": {"u_factor":3.0,"solar_heat_gain_coefficient":0.0},
                "H One SHGC": {"u_factor":3.0,"solar_heat_gain_coefficient":1.0},
                "I Zero VT": {
                    "u_factor":3.0,
                    "solar_heat_gain_coefficient":0.5,
                    "visible_transmittance":0.0
                },
                "J One VT": {
                    "u_factor":3.0,
                    "solar_heat_gain_coefficient":0.5,
                    "visible_transmittance":1.0
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    for (object_name, field) in [
        ("A Missing U", "u_factor"),
        ("B Missing SHGC", "solar_heat_gain_coefficient"),
    ] {
        assert!(has_diagnostic(
            &result,
            DiagnosticSeverity::Error,
            "MissingRequiredField",
            OBJECT_TYPE,
            object_name,
            field
        ));
    }
    for (object_name, field) in [
        ("C Wrong U", "u_factor"),
        ("D Wrong SHGC", "solar_heat_gain_coefficient"),
        ("E Wrong VT", "visible_transmittance"),
    ] {
        assert!(has_diagnostic(
            &result,
            DiagnosticSeverity::Error,
            "InvalidFieldType",
            OBJECT_TYPE,
            object_name,
            field
        ));
    }
    for (object_name, field) in [
        ("F Zero U", "u_factor"),
        ("G Zero SHGC", "solar_heat_gain_coefficient"),
        ("H One SHGC", "solar_heat_gain_coefficient"),
        ("I Zero VT", "visible_transmittance"),
        ("J One VT", "visible_transmittance"),
    ] {
        assert!(has_diagnostic(
            &result,
            DiagnosticSeverity::Error,
            "InvalidNumericRange",
            OBJECT_TYPE,
            object_name,
            field
        ));
    }
    Ok(())
}

#[test]
fn simple_glazing_system_rejects_nonpositive_source_derived_conductivity()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:SimpleGlazingSystem": {
                "Subnormal U": {
                    "u_factor":1e-320,
                    "solar_heat_gain_coefficient":0.5
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    assert!(result.model.is_none());
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "InvalidSimpleGlazingDerivedConductivity",
        OBJECT_TYPE,
        "Subnormal U",
        "u_factor"
    ));
    assert_eq!(
        result
            .report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
            .count(),
        1
    );
    Ok(())
}

#[test]
fn simple_glazing_system_shares_the_source_ordered_material_namespace()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Material": {
                "Shared Material": {
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
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "Earlier TC": {
                    "temperature_data":[{
                        "optical_data_temperature":10.0,
                        "window_material_glazing_name":"Glass"
                    }]
                }
            },
            "WindowMaterial:SimpleGlazingSystem": {
                "shared material": {"u_factor":3.0,"solar_heat_gain_coefficient":0.5},
                "earlier tc": {"u_factor":3.0,"solar_heat_gain_coefficient":0.5},
                "Unique Simple": {"u_factor":3.0,"solar_heat_gain_coefficient":0.5}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    for object_name in ["shared material", "earlier tc"] {
        assert!(
            result.report.diagnostics.iter().any(|diagnostic| {
                diagnostic.severity == DiagnosticSeverity::Error
                    && diagnostic.code == "DuplicateName"
                    && diagnostic.object_type == OBJECT_TYPE
                    && diagnostic.object_name.as_deref() == Some(object_name)
            }),
            "missing shared material-namespace collision for {object_name}"
        );
    }
    Ok(())
}

#[test]
fn thermochromic_group_cannot_resolve_a_later_simple_glazing_child()
-> Result<(), Box<dyn std::error::Error>> {
    const TC_OBJECT_TYPE: &str = "WindowMaterial:GlazingGroup:Thermochromic";
    let raw = parse_epjson_str(
        r#"{
            "WindowMaterial:GlazingGroup:Thermochromic": {
                "A Wants Later Simple": {
                    "temperature_data":[{
                        "optical_data_temperature":10.0,
                        "window_material_glazing_name":"Later Simple"
                    }]
                }
            },
            "WindowMaterial:SimpleGlazingSystem": {
                "Later Simple": {"u_factor":3.0,"solar_heat_gain_coefficient":0.5}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    assert!(has_diagnostic(
        &result,
        DiagnosticSeverity::Error,
        "MissingReference",
        TC_OBJECT_TYPE,
        "A Wants Later Simple[0]",
        "window_material_glazing_name"
    ));
    assert!(!result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidThermochromicGlazingReferenceType"
            && diagnostic.object_name.as_deref() == Some("A Wants Later Simple[0]")
    }));
    Ok(())
}

#[test]
fn every_simple_glazing_construction_reference_fails_closed()
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
            "WindowMaterial:Gas": {
                "Air Gap": {"gas_type":"Air","thickness":0.01}
            },
            "WindowMaterial:SimpleGlazingSystem": {
                "Simple": {"u_factor":3.0,"solar_heat_gain_coefficient":0.5}
            },
            "Construction": {
                "A Simple Outside": {"outside_layer":"Simple"},
                "B Simple Layer Two": {
                    "outside_layer":"Glass",
                    "layer_2":"Simple"
                },
                "C Simple Layer Three": {
                    "outside_layer":"Glass",
                    "layer_2":"Air Gap",
                    "layer_3":"Simple"
                },
                "D Opaque Then Simple": {
                    "outside_layer":"Opaque",
                    "layer_2":"Simple"
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(result.has_errors());
    for (object_name, field) in [
        ("A Simple Outside", "outside_layer"),
        ("B Simple Layer Two", "layer_2"),
        ("C Simple Layer Three", "layer_3"),
        ("D Opaque Then Simple", "layer_2"),
    ] {
        assert!(has_diagnostic(
            &result,
            DiagnosticSeverity::Error,
            "UnsupportedSimpleGlazingSystemConstruction",
            "Construction",
            object_name,
            field
        ));
    }
    Ok(())
}
