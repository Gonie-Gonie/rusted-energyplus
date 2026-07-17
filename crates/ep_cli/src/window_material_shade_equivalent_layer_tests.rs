use std::path::PathBuf;

use ep_compare::WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER;
use ep_model::TypedModel;
use ep_raw_model::{RawModel, parse_epjson_str};

use super::{
    NumericToleranceMode, compare_window_material_shade_equivalent_layer,
    constructions_report_requested, parse_tolerance_mode,
    run_compare_window_material_shade_equivalent_layer, shade_equivalent_layer_definitions,
    shade_equivalent_layer_header_count, shade_equivalent_layer_occurrences,
};

const SHADE_EQUIVALENT_LAYER_EPJSON: &str = r#"{
    "WindowMaterial:Shade:EquivalentLayer": {
        "A Unused EQL Shade": {
            "front_side_shade_beam_diffuse_solar_transmittance": 0.1111,
            "back_side_shade_beam_diffuse_solar_transmittance": 0.1222,
            "front_side_shade_beam_diffuse_solar_reflectance": 0.2333,
            "back_side_shade_beam_diffuse_solar_reflectance": 0.2444
        },
        "M Once EQL Shade": {
            "shade_beam_beam_solar_transmittance": 0.02,
            "front_side_shade_beam_diffuse_solar_transmittance": 0.10,
            "back_side_shade_beam_diffuse_solar_transmittance": 0.20,
            "front_side_shade_beam_diffuse_solar_reflectance": 0.30,
            "back_side_shade_beam_diffuse_solar_reflectance": 0.40,
            "shade_beam_beam_visible_transmittance_at_normal_incidence": 0.11,
            "shade_beam_diffuse_visible_transmittance_at_normal_incidence": 0.12,
            "shade_beam_diffuse_visible_reflectance_at_normal_incidence": 0.13,
            "shade_material_infrared_transmittance": 0.03,
            "front_side_shade_material_infrared_emissivity": 0.70,
            "back_side_shade_material_infrared_emissivity": 0.60
        },
        "Z Reused EQL Shade": {
            "shade_beam_beam_solar_transmittance": 0.0123456,
            "front_side_shade_beam_diffuse_solar_transmittance": 0.123456,
            "back_side_shade_beam_diffuse_solar_transmittance": 0.234567,
            "front_side_shade_beam_diffuse_solar_reflectance": 0.345678,
            "back_side_shade_beam_diffuse_solar_reflectance": 0.456789,
            "shade_beam_beam_visible_transmittance_at_normal_incidence": 0.21,
            "shade_beam_diffuse_visible_transmittance_at_normal_incidence": 0.22,
            "shade_beam_diffuse_visible_reflectance_at_normal_incidence": 0.23,
            "shade_material_infrared_transmittance": 0.0345678,
            "front_side_shade_material_infrared_emissivity": 0.765432,
            "back_side_shade_material_infrared_emissivity": 0.654321
        }
    },
    "Construction:WindowEquivalentLayer": {
        "A First EQL Construction": {
            "outside_layer": "Z Reused EQL Shade",
            "layer_2": "M Once EQL Shade"
        },
        "C Last EQL Construction": {
            "outside_layer": "Z Reused EQL Shade"
        }
    }
}"#;

const GENERIC_HEADER: &str = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible";
const A_GENERIC: &str = "Material Details,A UNUSED EQL SHADE,0.0000,MediumRough,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000";
const M_GENERIC: &str = "Material Details,M ONCE EQL SHADE,0.0000,MediumRough,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000";
const Z_GENERIC: &str = "Material Details,Z REUSED EQL SHADE,0.0000,MediumRough,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000";
const M_SPECIALIZED: &str = "WindowMaterial:Shade:EquivalentLayer,M ONCE EQL SHADE,2.0000E-002,2.0000E-002,0.1000,0.2000,0.3000,0.4000,3.0000E-002,0.7000,0.6000";
const Z_SPECIALIZED: &str = "WindowMaterial:Shade:EquivalentLayer,Z REUSED EQL SHADE,1.2346E-002,1.2346E-002,0.1235,0.2346,0.3457,0.4568,3.4568E-002,0.7654,0.6543";

fn test_models() -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    compile_models(&with_constructions_report(
        SHADE_EQUIVALENT_LAYER_EPJSON,
        "Constructions",
    ))
}

fn compile_models(epjson: &str) -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(epjson)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "equivalent-layer shade model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result
        .model
        .ok_or("equivalent-layer shade compiler returned no typed model")?;
    Ok((raw_model, model))
}

fn with_constructions_report(epjson: &str, detail_type: &str) -> String {
    let closing_brace = epjson
        .rfind('}')
        .expect("test epJSON must have a root closing brace");
    format!(
        "{},\n    \"Output:Constructions\": {{\n        \"Output Constructions 1\": {{\n            \"details_type_1\": \"{}\"\n        }}\n    }}\n}}",
        epjson[..closing_brace].trim_end(),
        detail_type
    )
}

fn exact_eio() -> String {
    [
        GENERIC_HEADER,
        Z_GENERIC,
        A_GENERIC,
        M_GENERIC,
        WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER,
        Z_SPECIALIZED,
        M_SPECIALIZED,
        Z_SPECIALIZED,
        "",
    ]
    .join("\n")
}

#[test]
fn rows_include_unused_definitions_and_raw_construction_occurrences_in_order()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let definitions = shade_equivalent_layer_definitions(&model);
    let occurrences = shade_equivalent_layer_occurrences(&raw_model, &model)?;

    assert_eq!(
        definitions
            .iter()
            .map(|row| row.material_name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "A UNUSED EQL SHADE",
            "M ONCE EQL SHADE",
            "Z REUSED EQL SHADE"
        ]
    );
    assert_eq!(
        occurrences
            .iter()
            .map(|row| {
                (
                    row.construction_name.as_str(),
                    row.layer_number,
                    row.material_name.as_str(),
                )
            })
            .collect::<Vec<_>>(),
        vec![
            ("A FIRST EQL CONSTRUCTION", 1, "Z REUSED EQL SHADE"),
            ("A FIRST EQL CONSTRUCTION", 2, "M ONCE EQL SHADE"),
            ("C LAST EQL CONSTRUCTION", 1, "Z REUSED EQL SHADE"),
        ]
    );
    assert!(
        occurrences
            .iter()
            .all(|row| row.material_name != "A UNUSED EQL SHADE")
    );
    Ok(())
}

#[test]
fn exact_comparison_gates_generic_rows_header_order_and_duplicate_occurrences()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        &exact_eio(),
        NumericToleranceMode::Exact,
    )?;

    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.definitions.len(), 3);
    assert_eq!(comparison.occurrences.len(), 3);
    assert_eq!(comparison.oracle_material_details.len(), 3);
    assert_eq!(comparison.oracle_occurrences.len(), 3);
    assert_eq!(comparison.header_rows, 1);

    let duplicate_header = exact_eio().replacen(
        WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER,
        &format!(
            "{WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER}\n{WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER}"
        ),
        1,
    );
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        &duplicate_header,
        NumericToleranceMode::Exact,
    )?;
    assert!(!comparison.passed);
    assert_eq!(comparison.header_rows, 2);

    let swapped = exact_eio().replace(
        &format!("{Z_SPECIALIZED}\n{M_SPECIALIZED}"),
        &format!("{M_SPECIALIZED}\n{Z_SPECIALIZED}"),
    );
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        &swapped,
        NumericToleranceMode::Exact,
    )?;
    assert!(
        !comparison.passed,
        "specialized emission order must be gated"
    );
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("field material_name"))
    );
    Ok(())
}

#[test]
fn comparison_rejects_missing_duplicate_extra_unknown_and_malformed_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let exact = exact_eio();

    let missing_generic = exact.replace(&format!("{A_GENERIC}\n"), "");
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        &missing_generic,
        NumericToleranceMode::Exact,
    )?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("Material Details row observed 0"))
    );

    let duplicate_generic = exact.replace(
        &format!("{A_GENERIC}\n"),
        &format!("{A_GENERIC}\n{A_GENERIC}\n"),
    );
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        &duplicate_generic,
        NumericToleranceMode::Exact,
    )?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("Material Details row observed 2"))
    );

    for changed in [
        exact.replacen(&format!("{Z_SPECIALIZED}\n"), "", 1),
        format!("{exact}{Z_SPECIALIZED}\n"),
        format!(
            "{exact}WindowMaterial:Shade:EquivalentLayer,UNKNOWN EQL SHADE,0,0,0,0,0,0,0,0,0\n"
        ),
    ] {
        let comparison = compare_window_material_shade_equivalent_layer(
            &raw_model,
            &model,
            &changed,
            NumericToleranceMode::Exact,
        )?;
        assert!(
            !comparison.passed,
            "missing, extra, and unknown specialized rows must fail"
        );
    }

    let malformed = exact.replacen(
        Z_SPECIALIZED,
        "WindowMaterial:Shade:EquivalentLayer,Z REUSED EQL SHADE,0.1",
        1,
    );
    let error = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        &malformed,
        NumericToleranceMode::Exact,
    )
    .expect_err("a malformed specialized row must fail parsing");
    assert!(
        error.contains("invalid EIO WindowMaterial:Shade:EquivalentLayer"),
        "{error}"
    );
    Ok(())
}

#[test]
fn every_generic_field_is_fixed_zero_even_in_near_mode() -> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    for field_index in 0..8 {
        let mut fields = A_GENERIC.split(',').collect::<Vec<_>>();
        let numeric_index = match field_index {
            0 => 2,
            1 => 4,
            2 => 5,
            3 => 6,
            4 => 7,
            5 => 8,
            6 => 9,
            7 => 10,
            _ => unreachable!(),
        };
        fields[numeric_index] = "1.0E-12";
        let changed = exact_eio().replace(A_GENERIC, &fields.join(","));
        let comparison = compare_window_material_shade_equivalent_layer(
            &raw_model,
            &model,
            &changed,
            NumericToleranceMode::Near,
        )?;
        assert!(
            !comparison.passed,
            "generic numeric field {field_index} must remain exact zero"
        );
    }
    Ok(())
}

#[test]
fn specialized_exact_and_near_modes_gate_all_nine_reported_fields()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let base_fields = Z_SPECIALIZED
        .split(',')
        .map(str::to_string)
        .collect::<Vec<_>>();

    for field_index in 2..=10 {
        let mut fields = base_fields.clone();
        let value = fields[field_index].parse::<f64>()?;
        fields[field_index] = format!("{:.9}", value + 0.000005);
        let changed = exact_eio().replacen(Z_SPECIALIZED, &fields.join(","), 1);

        let exact = compare_window_material_shade_equivalent_layer(
            &raw_model,
            &model,
            &changed,
            NumericToleranceMode::Exact,
        )?;
        assert!(!exact.passed, "exact field index {field_index} must fail");

        let near = compare_window_material_shade_equivalent_layer(
            &raw_model,
            &model,
            &changed,
            NumericToleranceMode::Near,
        )?;
        assert!(near.passed, "near field index {field_index} must pass");
    }
    Ok(())
}

#[test]
fn header_requires_constructions_report_and_a_window_construction()
-> Result<(), Box<dyn std::error::Error>> {
    let no_window_epjson = r#"{
        "WindowMaterial:Shade:EquivalentLayer": {
            "Only Unused EQL Shade": {
                "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                "back_side_shade_beam_diffuse_solar_transmittance": 0.2,
                "front_side_shade_beam_diffuse_solar_reflectance": 0.3,
                "back_side_shade_beam_diffuse_solar_reflectance": 0.4
            }
        }
    }"#;
    let (raw_model, model) = compile_models(&with_constructions_report(
        no_window_epjson,
        "Constructions",
    ))?;
    assert!(constructions_report_requested(&raw_model)?);
    let generic = "Material Details,ONLY UNUSED EQL SHADE,0,MediumRough,0,0,0,0,0,0,0\n";
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        generic,
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.header_rows, 0);
    let fabricated = format!("{generic}{WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER}\n");
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        &fabricated,
        NumericToleranceMode::Exact,
    )?;
    assert!(!comparison.passed);
    assert_eq!(
        comparison.first_divergence.as_deref(),
        Some("WindowMaterial:Shade:EquivalentLayer header expected 0 observed 1")
    );

    let ordinary_window_epjson = r#"{
        "WindowMaterial:Glazing": {
            "Clear Glass": {
                "optical_data_type": "SpectralAverage",
                "thickness": 0.003
            }
        },
        "WindowMaterial:Shade:EquivalentLayer": {
            "Only Unused EQL Shade": {
                "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                "back_side_shade_beam_diffuse_solar_transmittance": 0.2,
                "front_side_shade_beam_diffuse_solar_reflectance": 0.3,
                "back_side_shade_beam_diffuse_solar_reflectance": 0.4
            }
        },
        "Construction": {
            "Ordinary Window Construction": {
                "outside_layer": "Clear Glass"
            }
        }
    }"#;
    let (raw_model, model) = compile_models(ordinary_window_epjson)?;
    assert!(!constructions_report_requested(&raw_model)?);
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        generic,
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.header_rows, 0);

    let materials_only = with_constructions_report(ordinary_window_epjson, "Materials");
    let (raw_model, model) = compile_models(&materials_only)?;
    assert!(!constructions_report_requested(&raw_model)?);
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        generic,
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);

    let second_slot_report = with_constructions_report(ordinary_window_epjson, "constructions")
        .replace("\"details_type_1\"", "\"details_type_2\"");
    let (raw_model, _model) = compile_models(&second_slot_report)?;
    assert!(constructions_report_requested(&raw_model)?);

    let constructions_report = with_constructions_report(ordinary_window_epjson, "Constructions");
    let (raw_model, model) = compile_models(&constructions_report)?;
    assert!(constructions_report_requested(&raw_model)?);
    let eio = format!("{generic}{WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER}\n");
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        &eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert!(comparison.occurrences.is_empty());

    let missing = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        generic,
        NumericToleranceMode::Exact,
    )?;
    assert!(!missing.passed);
    assert_eq!(
        missing.first_divergence.as_deref(),
        Some("WindowMaterial:Shade:EquivalentLayer header expected 1 observed 0")
    );

    let complex_window_epjson = r#"{
        "WindowMaterial:Shade:EquivalentLayer": {
            "Only Unused EQL Shade": {
                "front_side_shade_beam_diffuse_solar_transmittance": 0.1,
                "back_side_shade_beam_diffuse_solar_transmittance": 0.2,
                "front_side_shade_beam_diffuse_solar_reflectance": 0.3,
                "back_side_shade_beam_diffuse_solar_reflectance": 0.4
            }
        },
        "WindowMaterial:Glazing": {
            "CFS Glass": {"optical_data_type":"SpectralAverage", "thickness":0.003}
        },
        "WindowThermalModel:Params": {"CFS Thermal": {}},
        "Matrix:TwoDimension": {
            "CFS Basis": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.0}]},
            "CFS Solar Front": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.1}]},
            "CFS Solar Back": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.2}]},
            "CFS Visible Front": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.3}]},
            "CFS Visible Back": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.4}]},
            "CFS Abs Front": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.5}]},
            "CFS Abs Back": {"number_of_rows":1, "number_of_columns":1,
                "values":[{"value":0.6}]}
        },
        "Construction:ComplexFenestrationState": {
            "Typed BSDF Construction": {
                "window_thermal_model":"CFS Thermal",
                "basis_matrix_name":"CFS Basis",
                "solar_optical_complex_front_transmittance_matrix_name":"CFS Solar Front",
                "solar_optical_complex_back_reflectance_matrix_name":"CFS Solar Back",
                "visible_optical_complex_front_transmittance_matrix_name":"CFS Visible Front",
                "visible_optical_complex_back_transmittance_matrix_name":"CFS Visible Back",
                "outside_layer_name":"CFS Glass",
                "outside_layer_directional_front_absorptance_matrix_name":"CFS Abs Front",
                "outside_layer_directional_back_absorptance_matrix_name":"CFS Abs Back"
            }
        }
    }"#;
    let (raw_model, model) = compile_models(&with_constructions_report(
        complex_window_epjson,
        "Constructions",
    ))?;
    let comparison = compare_window_material_shade_equivalent_layer(
        &raw_model,
        &model,
        &eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(
        comparison.passed,
        "a typed complex-fenestration construction triggers the shared window header: {:?}",
        comparison.first_divergence
    );
    assert!(comparison.occurrences.is_empty());
    Ok(())
}

#[test]
fn header_tolerance_parser_and_cli_argument_contract_are_exact() {
    assert_eq!(
        shade_equivalent_layer_header_count(WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER),
        1
    );
    assert_eq!(
        shade_equivalent_layer_header_count(
            "! <WindowMaterial:Shade:EquivalentLayer>, Material Name"
        ),
        0
    );
    assert_eq!(parse_tolerance_mode(&[]), Ok(NumericToleranceMode::Exact));
    assert_eq!(
        parse_tolerance_mode(&["--tolerance".to_string(), "near".to_string()]),
        Ok(NumericToleranceMode::Near)
    );
    assert!(parse_tolerance_mode(&["--tolerance".to_string()]).is_err());
    assert!(parse_tolerance_mode(&["--tolerance".to_string(), "loose".to_string()]).is_err());
    assert_eq!(run_compare_window_material_shade_equivalent_layer(&[]), 2);
    assert_eq!(
        run_compare_window_material_shade_equivalent_layer(&["only-input.epJSON".to_string()]),
        2
    );
}

#[test]
fn cli_accepts_exact_fixture() -> Result<(), Box<dyn std::error::Error>> {
    let directory = unique_test_directory();
    std::fs::create_dir_all(&directory)?;
    let input_path = directory.join("shade-equivalent-layer.epJSON");
    let eio_path = directory.join("eplusout.eio");
    std::fs::write(
        &input_path,
        with_constructions_report(SHADE_EQUIVALENT_LAYER_EPJSON, "Constructions"),
    )?;
    std::fs::write(&eio_path, exact_eio())?;

    let exit_code = run_compare_window_material_shade_equivalent_layer(&[
        input_path.to_string_lossy().into_owned(),
        eio_path.to_string_lossy().into_owned(),
        "--tolerance".to_string(),
        "exact".to_string(),
    ]);
    std::fs::remove_dir_all(&directory)?;

    assert_eq!(exit_code, 0);
    Ok(())
}

fn unique_test_directory() -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    std::env::temp_dir().join(format!(
        "rusted-energyplus-window-shade-equivalent-layer-cli-{}-{nonce}",
        std::process::id()
    ))
}
