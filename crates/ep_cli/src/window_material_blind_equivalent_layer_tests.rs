use std::path::PathBuf;

use ep_compare::WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER;
use ep_model::TypedModel;
use ep_raw_model::{RawModel, parse_epjson_str};

use super::{
    CASE_ID, NumericToleranceMode, blind_equivalent_layer_definitions,
    blind_equivalent_layer_header_count, blind_equivalent_layer_occurrences,
    compare_window_material_blind_equivalent_layer, construction_report_requests,
    energyplus_round_sig_digits_signed, expected_blind_equivalent_layer_row,
    material_details_header_count, parse_tolerance_mode,
    run_compare_window_material_blind_equivalent_layer,
};

const BLIND_EQUIVALENT_LAYER_EPJSON: &str = r#"{
    "WindowMaterial:Glazing:EquivalentLayer": {
        "Typed EQL Glass": {
            "front_side_beam_beam_solar_transmittance": 0.61,
            "back_side_beam_beam_solar_transmittance": 0.62,
            "front_side_beam_beam_solar_reflectance": 0.21,
            "back_side_beam_beam_solar_reflectance": 0.22
        }
    },
    "WindowMaterial:Blind:EquivalentLayer": {
        "A Defaulted Used EQL Blind": {
            "slat_width": 0.02,
            "slat_separation": 0.02,
            "front_side_slat_beam_diffuse_solar_reflectance": 0.20,
            "back_side_slat_beam_diffuse_solar_reflectance": 0.30,
            "front_side_slat_beam_diffuse_visible_reflectance": 0.25,
            "back_side_slat_beam_diffuse_visible_reflectance": 0.26,
            "front_side_slat_diffuse_diffuse_solar_reflectance": 0.21,
            "back_side_slat_diffuse_diffuse_solar_reflectance": 0.31,
            "slat_diffuse_diffuse_visible_transmittance": 0.06,
            "front_side_slat_diffuse_diffuse_visible_reflectance": 0.22,
            "back_side_slat_diffuse_diffuse_visible_reflectance": 0.32
        },
        "M Unused EQL Blind": {
            "slat_width": 0.02,
            "slat_separation": 0.02,
            "front_side_slat_beam_diffuse_solar_reflectance": 0.20,
            "back_side_slat_beam_diffuse_solar_reflectance": 0.30,
            "front_side_slat_diffuse_diffuse_solar_reflectance": 0.21,
            "back_side_slat_diffuse_diffuse_solar_reflectance": 0.31
        },
        "Z High Precision Reused EQL Blind": {
            "slat_orientation": "Vertical",
            "slat_width": 0.02456789,
            "slat_separation": 0.01876543,
            "slat_crown": 0.000654321,
            "slat_angle": -63.4567,
            "front_side_slat_beam_diffuse_solar_transmittance": 0.12345678,
            "back_side_slat_beam_diffuse_solar_transmittance": 0.23456789,
            "front_side_slat_beam_diffuse_solar_reflectance": 0.34567891,
            "back_side_slat_beam_diffuse_solar_reflectance": 0.45678912,
            "front_side_slat_beam_diffuse_visible_transmittance": 0.11111111,
            "back_side_slat_beam_diffuse_visible_transmittance": 0.12222222,
            "front_side_slat_beam_diffuse_visible_reflectance": 0.23333333,
            "back_side_slat_beam_diffuse_visible_reflectance": 0.24444444,
            "slat_diffuse_diffuse_solar_transmittance": 0.13333335,
            "front_side_slat_diffuse_diffuse_solar_reflectance": 0.25555557,
            "back_side_slat_diffuse_diffuse_solar_reflectance": 0.26666668,
            "slat_diffuse_diffuse_visible_transmittance": 0.17777779,
            "front_side_slat_diffuse_diffuse_visible_reflectance": 0.28888891,
            "back_side_slat_diffuse_diffuse_visible_reflectance": 0.29999993,
            "slat_infrared_transmittance": 0.01111113,
            "front_side_slat_infrared_emissivity": 0.82222227,
            "back_side_slat_infrared_emissivity": 0.83333339,
            "slat_angle_control": "MaximizeSolar"
        }
    },
    "Construction:WindowEquivalentLayer": {
        "A Defaulted EQL Blind Window Construction": {
            "outside_layer": "Typed EQL Glass",
            "layer_2": "A Defaulted Used EQL Blind"
        },
        "B High Precision First EQL Blind Window Construction": {
            "outside_layer": "Typed EQL Glass",
            "layer_2": "Z High Precision Reused EQL Blind"
        },
        "C High Precision Second EQL Blind Window Construction": {
            "outside_layer": "Typed EQL Glass",
            "layer_2": "Z High Precision Reused EQL Blind"
        }
    }
}"#;

const DEFINITIONS_ONLY_EPJSON: &str = r#"{
    "WindowMaterial:Blind:EquivalentLayer": {
        "Only EQL Blind": {
            "slat_width": 0.02,
            "slat_separation": 0.02,
            "front_side_slat_beam_diffuse_solar_reflectance": 0.20,
            "back_side_slat_beam_diffuse_solar_reflectance": 0.30,
            "front_side_slat_diffuse_diffuse_solar_reflectance": 0.21,
            "back_side_slat_diffuse_diffuse_solar_reflectance": 0.31
        }
    }
}"#;

const ORDINARY_WINDOW_WITHOUT_BLIND_EQUIVALENT_LAYER_OCCURRENCE_EPJSON: &str = r#"{
    "WindowMaterial:Blind:EquivalentLayer": {
        "Only EQL Blind": {
            "slat_width": 0.02,
            "slat_separation": 0.02,
            "front_side_slat_beam_diffuse_solar_reflectance": 0.20,
            "back_side_slat_beam_diffuse_solar_reflectance": 0.30,
            "front_side_slat_diffuse_diffuse_solar_reflectance": 0.21,
            "back_side_slat_diffuse_diffuse_solar_reflectance": 0.31
        }
    },
    "WindowMaterial:Glazing": {
        "Only Glass": {
            "optical_data_type": "SpectralAverage",
            "thickness": 0.003
        }
    },
    "Construction": {
        "Ordinary Window": {
            "outside_layer": "Only Glass"
        }
    }
}"#;

const GENERIC_HEADER: &str = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible";
const A_GENERIC: &str = "Material Details,A DEFAULTED USED EQL BLIND,0.0000,Rough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const M_GENERIC: &str = "Material Details,M UNUSED EQL BLIND,0.0000,Rough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const Z_GENERIC: &str = "Material Details,Z HIGH PRECISION REUSED EQL BLIND,0.0000,Rough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const A_SPECIALIZED: &str = "WindowMaterial:Blind:EquivalentLayer,A DEFAULTED USED EQL BLIND,Horizontal,2.00000E-002,2.00000E-002,1.50000E-003,45.00000,0.00000,0.00000,0.20000,0.30000,0.00000,0.00000,0.00000,0.00000,0.00000,0.00000";
const Z_SPECIALIZED: &str = "WindowMaterial:Blind:EquivalentLayer,Z HIGH PRECISION REUSED EQL BLIND,Vertical,2.45679E-002,1.87654E-002,6.54321E-004,-63.45670,0.12346,0.23457,0.34568,0.45679,0.13333,0.25556,0.26667,1.11111E-002,0.82222,0.83333";

fn compile_models(epjson: &str) -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(epjson)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "equivalent-layer blind model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result
        .model
        .ok_or("equivalent-layer blind compiler returned no typed model")?;
    Ok((raw_model, model))
}

fn with_report_fields(epjson: &str, report_fields: &str) -> String {
    let closing_brace = epjson
        .rfind('}')
        .expect("test epJSON must have a root closing brace");
    format!(
        "{},\n    \"Output:Constructions\": {{\n        \"Output Constructions 1\": {{{report_fields}}}\n    }}\n}}",
        epjson[..closing_brace].trim_end()
    )
}

fn with_both_reports(epjson: &str) -> String {
    with_report_fields(
        epjson,
        "\n            \"details_type_1\": \"Constructions\",\n            \"details_type_2\": \"Materials\"\n        ",
    )
}

fn with_single_report(epjson: &str, report: &str) -> String {
    with_report_fields(
        epjson,
        &format!("\n            \"details_type_1\": \"{report}\"\n        "),
    )
}

fn both_eio() -> String {
    [
        GENERIC_HEADER.to_string(),
        Z_GENERIC.to_string(),
        M_GENERIC.to_string(),
        A_GENERIC.to_string(),
        WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER.to_string(),
        format!(
            "{A_SPECIALIZED} Construction:WindowEquivalentLayer,B HIGH PRECISION FIRST EQL BLIND WINDOW CONSTRUCTION,3,3,1.999,0.586,0.208"
        ),
        format!(
            "{Z_SPECIALIZED} Construction:WindowEquivalentLayer,C HIGH PRECISION SECOND EQL BLIND WINDOW CONSTRUCTION,4,3,1.999,0.586,0.208"
        ),
        format!(
            "{Z_SPECIALIZED}! <Surface Convection Parameters>, Surface Name, Outside Model Assignment"
        ),
        String::new(),
    ]
    .join("\n")
}

fn materials_only_eio() -> String {
    [GENERIC_HEADER, Z_GENERIC, M_GENERIC, A_GENERIC, ""].join("\n")
}

fn constructions_only_eio() -> String {
    [
        WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER.to_string(),
        format!(
            "{A_SPECIALIZED} Construction:WindowEquivalentLayer,B HIGH PRECISION FIRST EQL BLIND WINDOW CONSTRUCTION,3,3,1.999,0.586,0.208"
        ),
        format!(
            "{Z_SPECIALIZED} Construction:WindowEquivalentLayer,C HIGH PRECISION SECOND EQL BLIND WINDOW CONSTRUCTION,4,3,1.999,0.586,0.208"
        ),
        format!(
            "{Z_SPECIALIZED}! <Surface Convection Parameters>, Surface Name, Outside Model Assignment"
        ),
        String::new(),
    ]
    .join("\n")
}

#[test]
fn expected_rows_preserve_defaulted_state_signed_5r_and_a_z_z_reuse()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_both_reports(BLIND_EQUIVALENT_LAYER_EPJSON))?;
    let definitions = blind_equivalent_layer_definitions(&model);
    let occurrences = blind_equivalent_layer_occurrences(&raw_model, &model)?;

    assert_eq!(CASE_ID, "window_material_blind_equivalent_layer_001");
    assert_eq!(
        definitions
            .iter()
            .map(|row| row.material_name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "A DEFAULTED USED EQL BLIND",
            "M UNUSED EQL BLIND",
            "Z HIGH PRECISION REUSED EQL BLIND"
        ]
    );
    assert_eq!(
        occurrences
            .iter()
            .map(|row| (
                row.construction_name.as_str(),
                row.layer_number,
                row.material_name.as_str(),
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                "A DEFAULTED EQL BLIND WINDOW CONSTRUCTION",
                2,
                "A DEFAULTED USED EQL BLIND"
            ),
            (
                "B HIGH PRECISION FIRST EQL BLIND WINDOW CONSTRUCTION",
                2,
                "Z HIGH PRECISION REUSED EQL BLIND"
            ),
            (
                "C HIGH PRECISION SECOND EQL BLIND WINDOW CONSTRUCTION",
                2,
                "Z HIGH PRECISION REUSED EQL BLIND"
            ),
        ]
    );
    assert!(
        occurrences
            .iter()
            .all(|row| row.material_name != "M UNUSED EQL BLIND")
    );

    let defaulted = expected_blind_equivalent_layer_row(&occurrences[0]);
    assert_eq!(defaulted.slat_orientation, "Horizontal");
    assert_eq!(defaulted.slat_crown_m, 0.0015);
    assert_eq!(defaulted.slat_angle_deg, 45.0);
    assert_eq!(defaulted.front_beam_diffuse_solar_transmittance, 0.0);
    assert_eq!(defaulted.diffuse_diffuse_solar_transmittance, 0.0);
    assert_eq!(defaulted.front_diffuse_diffuse_solar_reflectance, 0.0);
    assert_eq!(defaulted.back_diffuse_diffuse_solar_reflectance, 0.0);
    assert_eq!(defaulted.infrared_transmittance, 0.0);
    assert_eq!(defaulted.front_infrared_emissivity, 0.0);
    assert_eq!(defaulted.back_infrared_emissivity, 0.0);

    let high_precision = expected_blind_equivalent_layer_row(&occurrences[1]);
    assert_eq!(high_precision.slat_angle_deg, -63.45670);
    assert_eq!(high_precision.slat_width_m, 0.0245679);
    assert_eq!(
        energyplus_round_sig_digits_signed(-0.000654321, 5),
        Some(-0.000654321)
    );
    assert_eq!(energyplus_round_sig_digits_signed(0.0, 5), Some(0.0));
    for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(energyplus_round_sig_digits_signed(invalid, 5), None);
    }
    Ok(())
}

#[test]
fn exact_comparison_matches_rough_generic_and_malformed_source_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_both_reports(BLIND_EQUIVALENT_LAYER_EPJSON))?;
    let eio = both_eio();
    let comparison = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &eio,
        NumericToleranceMode::Exact,
    )?;

    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.definitions.len(), 3);
    assert_eq!(comparison.occurrences.len(), 3);
    assert_eq!(comparison.oracle_material_details.len(), 3);
    assert_eq!(comparison.oracle_occurrences.len(), 3);
    assert_eq!(comparison.material_details_header_rows, 1);
    assert_eq!(comparison.header_rows, 1);
    assert_eq!(
        WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER
            .split(',')
            .count(),
        18
    );
    assert_eq!(A_SPECIALIZED.split(',').count(), 17);
    assert!(
        !eio.contains(&format!("{A_SPECIALIZED}\n")),
        "the source-shaped row must concatenate directly with the next record"
    );
    assert_eq!(comparison.oracle_occurrences[1].slat_angle_deg, -63.45670);

    let wrong_roughness = eio.replacen(",Rough,", ",MediumRough,", 1);
    let comparison = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &wrong_roughness,
        NumericToleranceMode::Exact,
    )?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("field roughness"))
    );

    let missing_generic_header = eio.replacen(&format!("{GENERIC_HEADER}\n"), "", 1);
    let error = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &missing_generic_header,
        NumericToleranceMode::Exact,
    )
    .expect_err("generic rows without their exact header must fail closed");
    assert!(error.contains("Material Details row at line 1"));

    let duplicate_generic_header = eio.replacen(
        GENERIC_HEADER,
        &format!("{GENERIC_HEADER}\n{GENERIC_HEADER}"),
        1,
    );
    let comparison = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &duplicate_generic_header,
        NumericToleranceMode::Exact,
    )?;
    assert!(!comparison.passed);
    assert_eq!(
        comparison.first_divergence.as_deref(),
        Some("Material Details header expected 1 observed 2")
    );

    let malformed_generic_header = eio.replacen("ThermalResistance", "Thermal Resistance", 1);
    let error = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &malformed_generic_header,
        NumericToleranceMode::Exact,
    )
    .expect_err("a malformed generic header must fail closed");
    assert!(error.contains("invalid EIO Material Details header at line 1"));

    let generic_header_after_rows =
        eio.replacen(&format!("{GENERIC_HEADER}\n"), "", 1) + GENERIC_HEADER + "\n";
    let error = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &generic_header_after_rows,
        NumericToleranceMode::Exact,
    )
    .expect_err("generic rows before their exact header must fail closed");
    assert!(error.contains("Material Details row at line 1"));
    Ok(())
}

#[test]
fn materials_only_is_generic_only_and_accepts_missing_specialized_header()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_single_report(
        BLIND_EQUIVALENT_LAYER_EPJSON,
        "Materials",
    ))?;
    let requests = construction_report_requests(&raw_model)?;
    assert!(requests.materials);
    assert!(!requests.constructions);

    let comparison = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &materials_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert!(comparison.occurrences.is_empty());
    assert!(comparison.oracle_occurrences.is_empty());
    assert_eq!(comparison.material_details_header_rows, 1);
    assert_eq!(comparison.header_rows, 0);

    let fabricated = format!(
        "{}{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\n{Z_SPECIALIZED}! <Surface Convection Parameters>, Surface Name, Outside Model Assignment\n",
        materials_only_eio()
    );
    let comparison = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &fabricated,
        NumericToleranceMode::Exact,
    )?;
    assert!(!comparison.passed);
    assert_eq!(
        comparison.first_divergence.as_deref(),
        Some("WindowMaterial:Blind:EquivalentLayer header expected 0 observed 1")
    );
    Ok(())
}

#[test]
fn constructions_only_is_specialized_only_and_excludes_unused_definition()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_single_report(
        BLIND_EQUIVALENT_LAYER_EPJSON,
        "Constructions",
    ))?;
    let requests = construction_report_requests(&raw_model)?;
    assert!(requests.constructions);
    assert!(!requests.materials);

    let comparison = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &constructions_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert!(comparison.oracle_material_details.is_empty());
    assert_eq!(comparison.material_details_header_rows, 0);
    assert_eq!(comparison.oracle_occurrences.len(), 3);
    assert_eq!(comparison.header_rows, 1);
    assert!(
        comparison
            .oracle_occurrences
            .iter()
            .all(|row| row.material_name != "M UNUSED EQL BLIND")
    );

    let fabricated_generic_header = format!("{GENERIC_HEADER}\n{}", constructions_only_eio());
    let comparison = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &fabricated_generic_header,
        NumericToleranceMode::Exact,
    )?;
    assert!(!comparison.passed);
    assert_eq!(
        comparison.first_divergence.as_deref(),
        Some("Material Details header expected 0 observed 1")
    );

    let unrelated_generic_row = format!(
        "Material Details,UNRELATED OPAQUE,0.0000,Rough,0.1000,0.500,900.000,800.000,0.9000,0.7000,0.7000\n{}",
        constructions_only_eio()
    );
    let error = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &unrelated_generic_row,
        NumericToleranceMode::Exact,
    )
    .expect_err("a generic row without its exact header must fail closed");
    assert!(error.contains("Material Details row at line 1"));
    Ok(())
}

#[test]
fn constructions_header_requires_any_window_but_can_have_zero_blind_occurrences()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_single_report(
        DEFINITIONS_ONLY_EPJSON,
        "Constructions",
    ))?;
    let comparison = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        "",
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.definitions.len(), 1);
    assert!(comparison.occurrences.is_empty());
    assert_eq!(comparison.header_rows, 0);

    let (raw_model, model) = compile_models(&with_single_report(
        ORDINARY_WINDOW_WITHOUT_BLIND_EQUIVALENT_LAYER_OCCURRENCE_EPJSON,
        "Constructions",
    ))?;
    let eio = format!("{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\n");
    let comparison = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.header_rows, 1);
    assert!(comparison.occurrences.is_empty());
    assert!(comparison.oracle_occurrences.is_empty());
    Ok(())
}

#[test]
fn exact_rejects_and_near_accepts_small_signed_5r_deltas() -> Result<(), Box<dyn std::error::Error>>
{
    let (raw_model, model) = compile_models(&with_both_reports(BLIND_EQUIVALENT_LAYER_EPJSON))?;
    for replacement in [
        Z_SPECIALIZED.replace("-63.45670", "-63.456701"),
        Z_SPECIALIZED.replace("2.45679E-002", "0.02456791"),
    ] {
        let changed = both_eio().replacen(Z_SPECIALIZED, &replacement, 1);
        let exact = compare_window_material_blind_equivalent_layer(
            &raw_model,
            &model,
            &changed,
            NumericToleranceMode::Exact,
        )?;
        assert!(!exact.passed);
        let near = compare_window_material_blind_equivalent_layer(
            &raw_model,
            &model,
            &changed,
            NumericToleranceMode::Near,
        )?;
        assert!(near.passed, "{:?}", near.first_divergence);
    }
    Ok(())
}

#[test]
fn malformed_rows_and_cli_argument_contracts_are_exact() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(material_details_header_count(GENERIC_HEADER)?, 1);
    let (raw_model, model) = compile_models(&with_both_reports(BLIND_EQUIVALENT_LAYER_EPJSON))?;
    let malformed = both_eio().replace(
        Z_SPECIALIZED,
        "WindowMaterial:Blind:EquivalentLayer,Z HIGH PRECISION REUSED EQL BLIND,Vertical,0.1",
    );
    let error = compare_window_material_blind_equivalent_layer(
        &raw_model,
        &model,
        &malformed,
        NumericToleranceMode::Exact,
    )
    .expect_err("malformed source-shaped row must fail parsing");
    assert!(error.contains("invalid EIO WindowMaterial:Blind:EquivalentLayer"));
    assert!(error.contains("expected exactly 16 data fields"));

    assert_eq!(
        blind_equivalent_layer_header_count(WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER),
        1
    );
    assert_eq!(parse_tolerance_mode(&[]), Ok(NumericToleranceMode::Exact));
    assert_eq!(
        parse_tolerance_mode(&["--tolerance".to_string(), "near".to_string()]),
        Ok(NumericToleranceMode::Near)
    );
    assert!(parse_tolerance_mode(&["--tolerance".to_string()]).is_err());
    assert_eq!(run_compare_window_material_blind_equivalent_layer(&[]), 2);
    assert_eq!(
        run_compare_window_material_blind_equivalent_layer(&["only-input.epJSON".to_string()]),
        2
    );
    assert_eq!(
        crate::run_compare_command(&["window-material-blind-equivalent-layer".to_string()]),
        2,
        "the root compare dispatcher must route the new command"
    );
    Ok(())
}

#[test]
fn cli_command_accepts_exact_oracle_files() -> Result<(), Box<dyn std::error::Error>> {
    let directory = unique_temp_directory();
    std::fs::create_dir_all(&directory)?;
    let input_path = directory.join("blind-equivalent-layer.epJSON");
    let eio_path = directory.join("eplusout.eio");
    std::fs::write(
        &input_path,
        with_both_reports(BLIND_EQUIVALENT_LAYER_EPJSON),
    )?;
    std::fs::write(&eio_path, both_eio())?;

    let exit_code = run_compare_window_material_blind_equivalent_layer(&[
        input_path.display().to_string(),
        eio_path.display().to_string(),
        "--tolerance".to_string(),
        "exact".to_string(),
    ]);
    std::fs::remove_dir_all(&directory)?;
    assert_eq!(exit_code, 0);
    Ok(())
}

fn unique_temp_directory() -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    std::env::temp_dir().join(format!(
        "rusted-energyplus-window-blind-equivalent-layer-cli-{}-{nonce}",
        std::process::id()
    ))
}
