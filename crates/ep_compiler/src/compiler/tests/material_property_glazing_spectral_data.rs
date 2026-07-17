use super::super::{
    CompileResult, Compiler, DiagnosticSeverity, ObjectCoverageStatus, compile_raw_model,
    typed_coverage_status,
};
use ep_model::{GlazingSpectralDataId, MaterialId, TypedModel};
use ep_raw_model::parse_epjson_str;

const OBJECT_TYPE: &str = "MaterialProperty:GlazingSpectralData";

fn has_error(result: &CompileResult, code: &str, object_name: &str, field: Option<&str>) -> bool {
    result.report.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.code == code
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some(object_name)
            && diagnostic.field.as_deref() == field
    })
}

fn point_fields(
    suffix: usize,
    wavelength: f64,
    transmittance: f64,
    front_reflectance: f64,
    back_reflectance: f64,
) -> String {
    format!(
        r#""wavelength_{suffix}":{wavelength},"transmittance_{suffix}":{transmittance},"front_reflectance_{suffix}":{front_reflectance},"back_reflectance_{suffix}":{back_reflectance}"#
    )
}

fn compile_single_dataset(
    name: &str,
    fields: &str,
) -> Result<CompileResult, Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(&format!(r#"{{"{OBJECT_TYPE}":{{"{name}":{{{fields}}}}}}}"#))?;
    Ok(compile_raw_model(&raw))
}

fn fields_for_point_count(point_count: usize) -> String {
    let mut fields = Vec::new();
    for point_index in 0..point_count.min(4) {
        fields.push(point_fields(
            point_index + 1,
            0.1 + point_index as f64 * 0.004,
            0.2,
            0.3,
            0.4,
        ));
    }
    if point_count > 4 {
        let extensions = (4..point_count)
            .map(|point_index| {
                format!(
                    r#"{{"wavelength":{},"transmittance":0.2,"front_reflectance":0.3,"back_reflectance":0.4}}"#,
                    0.1 + point_index as f64 * 0.004
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        fields.push(format!(r#""extensions":[{extensions}]"#));
    }
    fields.join(",")
}

#[test]
fn glazing_spectral_data_materializes_fixed_and_extensible_source_semantics()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "MaterialProperty:GlazingSpectralData": {
                "A Zero Points": {},
                "B Fixed Then Extensions": {
                    "wavelength_1":0.1,
                    "transmittance_1":-0.5,
                    "front_reflectance_1":0.2,
                    "back_reflectance_1":0.3,
                    "wavelength_2":0.2,
                    "transmittance_2":0.4,
                    "front_reflectance_2":0.1,
                    "back_reflectance_2":0.2,
                    "wavelength_3":0.3,
                    "transmittance_3":0.5,
                    "front_reflectance_3":0.2,
                    "back_reflectance_3":0.1,
                    "wavelength_4":0.4,
                    "transmittance_4":0.6,
                    "front_reflectance_4":0.2,
                    "back_reflectance_4":0.2,
                    "extensions":[
                        {
                            "wavelength":0.5,
                            "transmittance":0.7,
                            "front_reflectance":0.1,
                            "back_reflectance":0.2
                        },
                        {"wavelength":0.6}
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
    let coverage = result
        .report
        .coverage
        .iter()
        .find(|coverage| coverage.object_type == OBJECT_TYPE)
        .ok_or_else(|| std::io::Error::other("missing spectral-data coverage"))?;
    assert_eq!(coverage.status, ObjectCoverageStatus::Typed);
    assert_eq!(coverage.object_count, 2);
    assert_eq!(result.report.typed_object_count, 3);

    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected glazing spectral datasets"))?;
    assert_eq!(model.object_count(), 3);
    assert_eq!(model.glazing_spectral_data.len(), 2);
    assert_eq!(model.glazing_spectral_data_names.len(), 2);
    assert_eq!(model.glazing_spectral_data[0].id, GlazingSpectralDataId(0));
    assert_eq!(model.glazing_spectral_data[0].name.0, "A ZERO POINTS");
    assert!(model.glazing_spectral_data[0].points.is_empty());
    assert_eq!(model.glazing_spectral_data[1].id, GlazingSpectralDataId(1));
    assert_eq!(
        model
            .glazing_spectral_data_names
            .resolve("b fixed then extensions"),
        Some(GlazingSpectralDataId(1))
    );
    let points = &model.glazing_spectral_data[1].points;
    assert_eq!(points.len(), 6);
    assert_eq!(
        points
            .iter()
            .map(|point| point.wavelength_microns)
            .collect::<Vec<_>>(),
        vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6]
    );
    assert_eq!(points[0].transmittance, 0.001);
    assert_eq!(points[0].front_reflectance, 0.2);
    assert_eq!(points[0].back_reflectance, 0.3);
    assert_eq!(points[4].transmittance, 0.7);
    assert_eq!(points[5].transmittance, 0.001);
    assert_eq!(points[5].front_reflectance, 0.0);
    assert_eq!(points[5].back_reflectance, 0.0);
    Ok(())
}

#[test]
fn glazing_spectral_data_accepts_zero_one_and_eight_hundred_points_but_not_801()
-> Result<(), Box<dyn std::error::Error>> {
    for point_count in [0, 1, 800] {
        let name = format!("Valid {point_count}");
        let result = compile_single_dataset(&name, &fields_for_point_count(point_count))?;
        assert!(
            !result.has_errors(),
            "{point_count} points rejected: {:?}",
            result.report.diagnostics
        );
        assert_eq!(
            result
                .model
                .as_ref()
                .and_then(|model| model.glazing_spectral_data.first())
                .map(|dataset| dataset.points.len()),
            Some(point_count)
        );
    }

    let too_many = compile_single_dataset("Too Many", &fields_for_point_count(801))?;
    assert!(has_error(
        &too_many,
        "TooManyGlazingSpectralDataPoints",
        "Too Many",
        Some("extensions")
    ));
    Ok(())
}

#[test]
fn glazing_spectral_data_rejects_all_incomplete_fixed_quartet_remainders()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "MaterialProperty:GlazingSpectralData": {
                "A Remainder One": {"wavelength_1":0.2},
                "B Remainder Two": {"wavelength_1":0.2,"transmittance_1":0.3},
                "C Remainder Three": {
                    "wavelength_1":0.2,
                    "transmittance_1":0.3,
                    "front_reflectance_1":0.2
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    for name in ["A Remainder One", "B Remainder Two", "C Remainder Three"] {
        assert!(has_error(
            &result,
            "InvalidGlazingSpectralDataFieldCount",
            name,
            None
        ));
    }
    assert_eq!(
        result
            .report
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "InvalidGlazingSpectralDataFieldCount")
            .count(),
        3
    );
    Ok(())
}

#[test]
fn glazing_spectral_data_zero_fills_internal_and_extension_omissions_before_tau_clamp()
-> Result<(), Box<dyn std::error::Error>> {
    let internal = compile_single_dataset(
        "Internal Omissions",
        r#""wavelength_1":0.2,"back_reflectance_1":0.4"#,
    )?;
    assert!(!internal.has_errors(), "{:?}", internal.report.diagnostics);
    let point = &internal
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing internal omission model"))?
        .glazing_spectral_data[0]
        .points[0];
    assert_eq!(point.transmittance, 0.001);
    assert_eq!(point.front_reflectance, 0.0);
    assert_eq!(point.back_reflectance, 0.4);

    let mut fields = fields_for_point_count(4);
    fields.push_str(r#", "extensions":[{"wavelength":0.5}]"#);
    let extension = compile_single_dataset("Extension Omissions", &fields)?;
    assert!(
        !extension.has_errors(),
        "{:?}",
        extension.report.diagnostics
    );
    let point = &extension
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("missing extension omission model"))?
        .glazing_spectral_data[0]
        .points[4];
    assert_eq!(point.wavelength_microns, 0.5);
    assert_eq!(point.transmittance, 0.001);
    assert_eq!(point.front_reflectance, 0.0);
    assert_eq!(point.back_reflectance, 0.0);
    Ok(())
}

#[test]
fn nonempty_extensions_force_all_sixteen_fixed_positions_before_extension_points()
-> Result<(), Box<dyn std::error::Error>> {
    let result = compile_single_dataset(
        "Extension Only",
        r#""extensions":[{
            "wavelength":0.5,
            "transmittance":0.5,
            "front_reflectance":0.2,
            "back_reflectance":0.2
        }]"#,
    )?;

    for suffix in 1..=4 {
        assert!(has_error(
            &result,
            "InvalidGlazingSpectralWavelength",
            "Extension Only",
            Some(&format!("wavelength_{suffix}"))
        ));
    }
    Ok(())
}

#[test]
fn glazing_spectral_data_enforces_source_wavelength_range_and_strict_order()
-> Result<(), Box<dyn std::error::Error>> {
    for (name, wavelength) in [("Below Wavelength", 0.0999), ("Above Wavelength", 4.0001)] {
        let result = compile_single_dataset(name, &point_fields(1, wavelength, 0.5, 0.2, 0.2))?;
        assert!(has_error(
            &result,
            "InvalidGlazingSpectralWavelength",
            name,
            Some("wavelength_1")
        ));
    }

    for (name, first, second) in [
        ("Equal Wavelength", 0.5, 0.5),
        ("Descending Wavelength", 0.5, 0.4),
    ] {
        let fields = format!(
            "{},{}",
            point_fields(1, first, 0.5, 0.2, 0.2),
            point_fields(2, second, 0.5, 0.2, 0.2)
        );
        let result = compile_single_dataset(name, &fields)?;
        assert!(has_error(
            &result,
            "NonIncreasingGlazingSpectralWavelength",
            name,
            Some("wavelength_2")
        ));
    }

    let endpoints = compile_single_dataset(
        "Wavelength Endpoints",
        &format!(
            "{},{}",
            point_fields(1, 0.1, 0.5, 0.2, 0.2),
            point_fields(2, 4.0, 0.5, 0.2, 0.2)
        ),
    )?;
    assert!(
        !endpoints.has_errors(),
        "{:?}",
        endpoints.report.diagnostics
    );
    Ok(())
}

#[test]
fn glazing_spectral_data_enforces_source_tau_rho_and_sum_tolerances()
-> Result<(), Box<dyn std::error::Error>> {
    let valid = compile_single_dataset(
        "Tolerance Endpoints",
        &[
            point_fields(1, 0.1, 1.01, 0.0, 0.0),
            point_fields(2, 0.2, -5.0, 1.02, 1.02),
            point_fields(3, 0.3, 0.5, 0.53, 0.53),
            point_fields(4, 4.0, 0.001, 0.0, 0.0),
        ]
        .join(","),
    )?;
    assert!(!valid.has_errors(), "{:?}", valid.report.diagnostics);
    assert_eq!(
        valid
            .model
            .as_ref()
            .and_then(|model| model.glazing_spectral_data.first())
            .and_then(|dataset| dataset.points.get(1))
            .map(|point| point.transmittance),
        Some(0.001)
    );

    for (name, fields, code, field) in [
        (
            "Tau Above",
            point_fields(1, 0.5, 1.010_001, 0.0, 0.0),
            "InvalidGlazingSpectralTransmittance",
            "transmittance_1",
        ),
        (
            "Front Rho Above",
            point_fields(1, 0.5, 0.001, 1.020_001, 0.0),
            "InvalidGlazingSpectralReflectance",
            "front_reflectance_1",
        ),
        (
            "Back Rho Negative",
            point_fields(1, 0.5, 0.001, 0.0, -0.000_001),
            "InvalidGlazingSpectralReflectance",
            "back_reflectance_1",
        ),
        (
            "Front Sum Above",
            point_fields(1, 0.5, 0.5, 0.530_001, 0.0),
            "InvalidGlazingSpectralOpticalSum",
            "front_reflectance_1",
        ),
        (
            "Back Sum Above",
            point_fields(1, 0.5, 0.5, 0.0, 0.530_001),
            "InvalidGlazingSpectralOpticalSum",
            "back_reflectance_1",
        ),
    ] {
        let result = compile_single_dataset(name, &fields)?;
        assert!(
            has_error(&result, code, name, Some(field)),
            "missing {code} for {name}: {:?}",
            result.report.diagnostics
        );
    }
    Ok(())
}

#[test]
fn glazing_spectral_data_rejects_malformed_extensions_entries_and_scalars()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "MaterialProperty:GlazingSpectralData": {
                "A Wrong Extensions": {"extensions":{}},
                "B Wrong Entry": {"extensions":[3]},
                "C Wrong Fixed Scalars": {
                    "wavelength_1":"blue",
                    "transmittance_1":true,
                    "front_reflectance_1":[],
                    "back_reflectance_1":{}
                },
                "D Wrong Extension Scalar": {
                    "wavelength_1":0.1,"transmittance_1":0.2,
                    "front_reflectance_1":0.3,"back_reflectance_1":0.4,
                    "wavelength_2":0.2,"transmittance_2":0.2,
                    "front_reflectance_2":0.3,"back_reflectance_2":0.4,
                    "wavelength_3":0.3,"transmittance_3":0.2,
                    "front_reflectance_3":0.3,"back_reflectance_3":0.4,
                    "wavelength_4":0.4,"transmittance_4":0.2,
                    "front_reflectance_4":0.3,"back_reflectance_4":0.4,
                    "extensions":[{
                        "wavelength":"red",
                        "transmittance":0.2,
                        "front_reflectance":0.3,
                        "back_reflectance":0.4
                    }]
                }
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert!(has_error(
        &result,
        "InvalidFieldType",
        "A Wrong Extensions",
        Some("extensions")
    ));
    assert!(has_error(
        &result,
        "InvalidFieldType",
        "B Wrong Entry",
        Some("extensions")
    ));
    for field in [
        "wavelength_1",
        "transmittance_1",
        "front_reflectance_1",
        "back_reflectance_1",
    ] {
        assert!(has_error(
            &result,
            "InvalidFieldType",
            "C Wrong Fixed Scalars",
            Some(field)
        ));
    }
    assert!(has_error(
        &result,
        "InvalidFieldType",
        "D Wrong Extension Scalar",
        Some("extensions[0].wavelength")
    ));
    assert!(
        result.model.is_none(),
        "unused invalid datasets must fail compilation"
    );
    Ok(())
}

#[test]
fn glazing_spectral_data_duplicate_names_fail_closed_after_full_object_validation()
-> Result<(), Box<dyn std::error::Error>> {
    let valid_fields = point_fields(1, 0.5, 0.5, 0.2, 0.2);
    let duplicate_raw = parse_epjson_str(&format!(
        r#"{{"{OBJECT_TYPE}":{{"A Shared":{{{valid_fields}}},"a shared":{{{valid_fields}}}}}}}"#
    ))?;
    let mut compiler = Compiler::new(&duplicate_raw, None);
    let mut model = TypedModel::default();
    compiler.parse_glazing_spectral_data(&mut model);
    assert_eq!(model.glazing_spectral_data.len(), 1);
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName"
            && diagnostic.object_type == OBJECT_TYPE
            && diagnostic.object_name.as_deref() == Some("a shared")
    }));

    let invalid_first_raw = parse_epjson_str(&format!(
        r#"{{"{OBJECT_TYPE}":{{
            "A Invalid Shared":{{{}}},
            "a invalid shared":{{{valid_fields}}}
        }}}}"#,
        point_fields(1, 0.0, 0.5, 0.2, 0.2)
    ))?;
    let mut compiler = Compiler::new(&invalid_first_raw, None);
    let mut model = TypedModel::default();
    compiler.parse_glazing_spectral_data(&mut model);
    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "InvalidGlazingSpectralWavelength"
            && diagnostic.object_name.as_deref() == Some("A Invalid Shared")
    }));
    assert!(!compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "DuplicateName" && diagnostic.object_type == OBJECT_TYPE
    }));
    assert_eq!(model.glazing_spectral_data.len(), 1);
    assert_eq!(model.glazing_spectral_data[0].name.0, "A INVALID SHARED");
    assert_eq!(model.glazing_spectral_data[0].id, GlazingSpectralDataId(0));
    Ok(())
}

#[test]
fn glazing_spectral_data_uses_a_namespace_separate_from_materials()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "MaterialProperty:GlazingSpectralData": {
                "Shared Name": {
                    "wavelength_1":0.5,
                    "transmittance_1":0.5,
                    "front_reflectance_1":0.2,
                    "back_reflectance_1":0.2
                }
            },
            "Material:NoMass": {
                "shared name": {"roughness":"Rough","thermal_resistance":1.0}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .as_ref()
        .ok_or_else(|| std::io::Error::other("expected separate namespaces"))?;
    assert_eq!(
        model.glazing_spectral_data_names.resolve("SHARED NAME"),
        Some(GlazingSpectralDataId(0))
    );
    assert_eq!(
        model.material_names.resolve("SHARED NAME"),
        Some(MaterialId(0))
    );
    assert_eq!(model.glazing_spectral_data.len(), 1);
    assert_eq!(model.materials.len(), 1);
    Ok(())
}

#[test]
fn spectral_dataset_does_not_unblock_deferred_window_glazing_optical_modes()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "MaterialProperty:GlazingSpectralData": {
                "Valid Data": {
                    "wavelength_1":0.5,
                    "transmittance_1":0.5,
                    "front_reflectance_1":0.2,
                    "back_reflectance_1":0.2
                }
            },
            "WindowMaterial:Glazing": {
                "A Spectral": {
                    "optical_data_type":"Spectral",
                    "window_glass_spectral_data_set_name":"Valid Data",
                    "thickness":0.006
                },
                "B Spectral And Angle": {
                    "optical_data_type":"SpectralAndAngle",
                    "thickness":0.006
                },
                "C Bsdf": {"optical_data_type":"BSDF","thickness":0.006}
            }
        }"#,
    )?;
    let result = compile_raw_model(&raw);

    assert_eq!(
        result
            .report
            .diagnostics
            .iter()
            .filter(|diagnostic| {
                diagnostic.code == "UnsupportedWindowGlazingOpticalDataType"
                    && diagnostic.object_type == "WindowMaterial:Glazing"
            })
            .count(),
        3
    );
    assert_eq!(
        result
            .report
            .coverage
            .iter()
            .find(|coverage| coverage.object_type == OBJECT_TYPE)
            .map(|coverage| coverage.status),
        Some(ObjectCoverageStatus::Typed)
    );
    Ok(())
}
