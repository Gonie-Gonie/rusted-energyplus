use std::path::PathBuf;

use ep_compare::WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER;
use ep_model::TypedModel;
use ep_raw_model::{RawModel, parse_epjson_str};

use super::{
    CASE_ID, NumericToleranceMode, compare_window_material_screen_equivalent_layer,
    construction_report_requests, expected_screen_equivalent_layer_row, parse_tolerance_mode,
    run_compare_window_material_screen_equivalent_layer, screen_equivalent_layer_definitions,
    screen_equivalent_layer_header_count, screen_equivalent_layer_occurrences,
};

const SCREEN_EQUIVALENT_LAYER_EPJSON: &str = r#"{
    "WindowMaterial:Screen:EquivalentLayer": {
        "A Auto Used EQL Screen": {
            "screen_beam_diffuse_solar_transmittance": 0.1111,
            "screen_beam_diffuse_solar_reflectance": 0.2222,
            "screen_beam_beam_visible_transmittance": 0.1,
            "screen_beam_diffuse_visible_transmittance": 0.2,
            "screen_beam_diffuse_visible_reflectance": 0.3
        },
        "M Unused EQL Screen": {
            "screen_beam_beam_solar_transmittance": 0.5,
            "screen_beam_diffuse_solar_transmittance": 0.1,
            "screen_beam_diffuse_solar_reflectance": 0.2,
            "screen_beam_beam_visible_transmittance": 0.1,
            "screen_beam_diffuse_visible_transmittance": 0.2,
            "screen_beam_diffuse_visible_reflectance": 0.3
        },
        "Z Reused EQL Screen": {
            "screen_beam_beam_solar_transmittance": 0.656099212675197,
            "screen_beam_diffuse_solar_transmittance": 0.123456,
            "screen_beam_diffuse_solar_reflectance": 0.234567,
            "screen_beam_beam_visible_transmittance": 0.21,
            "screen_beam_diffuse_visible_transmittance": 0.22,
            "screen_beam_diffuse_visible_reflectance": 0.23,
            "screen_infrared_transmittance": 0.0000345678,
            "screen_infrared_emissivity": 0.765432,
            "screen_wire_spacing": 0.0123456,
            "screen_wire_diameter": 0.00234567
        }
    },
    "Construction:WindowEquivalentLayer": {
        "A First EQL Construction": {
            "outside_layer": "Z Reused EQL Screen",
            "layer_2": "A Auto Used EQL Screen"
        },
        "C Last EQL Construction": {
            "outside_layer": "Z Reused EQL Screen"
        }
    }
}"#;

const DEFINITIONS_ONLY_EPJSON: &str = r#"{
    "WindowMaterial:Screen:EquivalentLayer": {
        "Only EQL Screen": {
            "screen_beam_diffuse_solar_transmittance": 0.1,
            "screen_beam_diffuse_solar_reflectance": 0.2,
            "screen_beam_beam_visible_transmittance": 0.1,
            "screen_beam_diffuse_visible_transmittance": 0.2,
            "screen_beam_diffuse_visible_reflectance": 0.3
        }
    }
}"#;

const ORDINARY_WINDOW_WITHOUT_SCREEN_EQUIVALENT_LAYER_OCCURRENCE_EPJSON: &str = r#"{
    "WindowMaterial:Screen:EquivalentLayer": {
        "Only EQL Screen": {
            "screen_beam_diffuse_solar_transmittance": 0.1,
            "screen_beam_diffuse_solar_reflectance": 0.2,
            "screen_beam_beam_visible_transmittance": 0.1,
            "screen_beam_diffuse_visible_transmittance": 0.2,
            "screen_beam_diffuse_visible_reflectance": 0.3
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
const A_GENERIC: &str = "Material Details,A AUTO USED EQL SCREEN,0.0000,MediumRough,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000";
const M_GENERIC: &str = "Material Details,M UNUSED EQL SCREEN,0.0000,MediumRough,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000";
const Z_GENERIC: &str = "Material Details,Z REUSED EQL SCREEN,0.0000,MediumRough,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000";
const A_SPECIALIZED: &str = "WindowMaterial:Screen:EquivalentLayer,A AUTO USED EQL SCREEN,-99999.0000,0.1111,0.1111,0.2222,0.2222,2.0000E-002,0.9300,0.9300,0.00000,0.00000";
const Z_SPECIALIZED: &str = "WindowMaterial:Screen:EquivalentLayer,Z REUSED EQL SCREEN,0.6561,0.1235,0.1235,0.2346,0.2346,3.4568E-005,0.7654,0.7654,1.23456E-002,2.34567E-003";

fn compile_models(epjson: &str) -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(epjson)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "equivalent-layer screen model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result
        .model
        .ok_or("equivalent-layer screen compiler returned no typed model")?;
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
        GENERIC_HEADER,
        Z_GENERIC,
        A_GENERIC,
        M_GENERIC,
        WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER,
        Z_SPECIALIZED,
        A_SPECIALIZED,
        Z_SPECIALIZED,
        "",
    ]
    .join("\n")
}

fn materials_only_eio() -> String {
    [GENERIC_HEADER, Z_GENERIC, A_GENERIC, M_GENERIC, ""].join("\n")
}

fn constructions_only_eio() -> String {
    [
        WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER,
        Z_SPECIALIZED,
        A_SPECIALIZED,
        Z_SPECIALIZED,
        "",
    ]
    .join("\n")
}

#[test]
fn expected_rows_use_fixture_occurrences_repeats_and_raw_auto_sentinel()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_both_reports(SCREEN_EQUIVALENT_LAYER_EPJSON))?;
    let definitions = screen_equivalent_layer_definitions(&model);
    let occurrences = screen_equivalent_layer_occurrences(&raw_model, &model)?;

    assert_eq!(CASE_ID, "window_material_screen_equivalent_layer_001");
    assert_eq!(
        definitions
            .iter()
            .map(|row| row.material_name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "A AUTO USED EQL SCREEN",
            "M UNUSED EQL SCREEN",
            "Z REUSED EQL SCREEN"
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
            ("A FIRST EQL CONSTRUCTION", 1, "Z REUSED EQL SCREEN"),
            ("A FIRST EQL CONSTRUCTION", 2, "A AUTO USED EQL SCREEN"),
            ("C LAST EQL CONSTRUCTION", 1, "Z REUSED EQL SCREEN"),
        ]
    );
    assert!(
        occurrences
            .iter()
            .all(|row| row.material_name != "M UNUSED EQL SCREEN")
    );

    let auto = expected_screen_equivalent_layer_row(&occurrences[1]);
    assert_eq!(auto.beam_beam_solar_transmittance, -99_999.0);
    assert_eq!(auto.front_beam_diffuse_solar_transmittance, 0.1111);
    assert_eq!(
        auto.front_beam_diffuse_solar_transmittance,
        auto.back_beam_diffuse_solar_transmittance
    );
    assert_eq!(
        auto.front_beam_diffuse_solar_reflectance,
        auto.back_beam_diffuse_solar_reflectance
    );
    assert_eq!(
        auto.front_infrared_emissivity,
        auto.back_infrared_emissivity
    );
    assert_eq!((auto.wire_spacing_m, auto.wire_diameter_m), (0.0, 0.0));
    Ok(())
}

#[test]
fn exact_comparison_matches_generic_and_specialized_source_shapes()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_both_reports(SCREEN_EQUIVALENT_LAYER_EPJSON))?;
    let comparison = compare_window_material_screen_equivalent_layer(
        &raw_model,
        &model,
        &both_eio(),
        NumericToleranceMode::Exact,
    )?;

    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.definitions.len(), 3);
    assert_eq!(comparison.occurrences.len(), 3);
    assert_eq!(comparison.oracle_material_details.len(), 3);
    assert_eq!(comparison.oracle_occurrences.len(), 3);
    assert_eq!(comparison.header_rows, 1);
    assert_eq!(
        WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER
            .split(',')
            .count(),
        9
    );
    assert_eq!(
        comparison.oracle_occurrences[1].beam_beam_solar_transmittance,
        -99_999.0
    );
    Ok(())
}

#[test]
fn materials_only_is_generic_only_and_accepts_missing_specialized_header()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_single_report(
        SCREEN_EQUIVALENT_LAYER_EPJSON,
        "Materials",
    ))?;
    let requests = construction_report_requests(&raw_model)?;
    assert!(requests.materials);
    assert!(!requests.constructions);

    let comparison = compare_window_material_screen_equivalent_layer(
        &raw_model,
        &model,
        &materials_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert!(comparison.occurrences.is_empty());
    assert!(comparison.oracle_occurrences.is_empty());
    assert_eq!(comparison.header_rows, 0);

    let fabricated = format!(
        "{}{WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}\n{Z_SPECIALIZED}\n",
        materials_only_eio()
    );
    let comparison = compare_window_material_screen_equivalent_layer(
        &raw_model,
        &model,
        &fabricated,
        NumericToleranceMode::Exact,
    )?;
    assert!(!comparison.passed);
    assert_eq!(
        comparison.first_divergence.as_deref(),
        Some("WindowMaterial:Screen:EquivalentLayer header expected 0 observed 1")
    );
    Ok(())
}

#[test]
fn constructions_only_is_specialized_only_and_keeps_unused_generic_rows_absent()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_single_report(
        SCREEN_EQUIVALENT_LAYER_EPJSON,
        "Constructions",
    ))?;
    let requests = construction_report_requests(&raw_model)?;
    assert!(requests.constructions);
    assert!(!requests.materials);

    let comparison = compare_window_material_screen_equivalent_layer(
        &raw_model,
        &model,
        &constructions_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert!(comparison.oracle_material_details.is_empty());
    assert_eq!(comparison.oracle_occurrences.len(), 3);
    assert_eq!(comparison.header_rows, 1);
    Ok(())
}

#[test]
fn constructions_header_requires_any_window_construction() -> Result<(), Box<dyn std::error::Error>>
{
    let (raw_model, model) = compile_models(&with_single_report(
        DEFINITIONS_ONLY_EPJSON,
        "Constructions",
    ))?;
    let comparison = compare_window_material_screen_equivalent_layer(
        &raw_model,
        &model,
        "",
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.definitions.len(), 1);
    assert!(comparison.occurrences.is_empty());
    assert_eq!(comparison.header_rows, 0);
    Ok(())
}

#[test]
fn constructions_header_is_legal_without_a_screen_equivalent_layer_occurrence()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_single_report(
        ORDINARY_WINDOW_WITHOUT_SCREEN_EQUIVALENT_LAYER_OCCURRENCE_EPJSON,
        "Constructions",
    ))?;
    let eio = format!("{WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}\n");
    let comparison = compare_window_material_screen_equivalent_layer(
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
fn exact_rejects_near_accepts_rounding_deltas_but_auto_sentinel_stays_exact()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_both_reports(SCREEN_EQUIVALENT_LAYER_EPJSON))?;
    for replacement in [
        Z_SPECIALIZED.replace("0.1235", "0.123501"),
        Z_SPECIALIZED.replace("1.23456E-002", "0.0123465"),
    ] {
        let changed = both_eio().replacen(Z_SPECIALIZED, &replacement, 1);
        let exact = compare_window_material_screen_equivalent_layer(
            &raw_model,
            &model,
            &changed,
            NumericToleranceMode::Exact,
        )?;
        assert!(!exact.passed);
        let near = compare_window_material_screen_equivalent_layer(
            &raw_model,
            &model,
            &changed,
            NumericToleranceMode::Near,
        )?;
        assert!(near.passed, "{:?}", near.first_divergence);
    }

    let changed_auto = both_eio().replace("-99999.0000", "-99999.000001");
    for mode in [NumericToleranceMode::Exact, NumericToleranceMode::Near] {
        let comparison = compare_window_material_screen_equivalent_layer(
            &raw_model,
            &model,
            &changed_auto,
            mode,
        )?;
        assert!(!comparison.passed, "Auto sentinel must remain exact");
    }
    Ok(())
}

#[test]
fn malformed_rows_and_cli_argument_contracts_are_exact() -> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_both_reports(SCREEN_EQUIVALENT_LAYER_EPJSON))?;
    let malformed = both_eio().replace(
        Z_SPECIALIZED,
        "WindowMaterial:Screen:EquivalentLayer,Z REUSED EQL SCREEN,0.1",
    );
    let error = compare_window_material_screen_equivalent_layer(
        &raw_model,
        &model,
        &malformed,
        NumericToleranceMode::Exact,
    )
    .expect_err("malformed source-shaped row must fail parsing");
    assert!(error.contains("invalid EIO WindowMaterial:Screen:EquivalentLayer"));
    assert!(error.contains("expected exactly 11 data fields"));

    assert_eq!(
        screen_equivalent_layer_header_count(WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER),
        1
    );
    assert_eq!(parse_tolerance_mode(&[]), Ok(NumericToleranceMode::Exact));
    assert_eq!(
        parse_tolerance_mode(&["--tolerance".to_string(), "near".to_string()]),
        Ok(NumericToleranceMode::Near)
    );
    assert!(parse_tolerance_mode(&["--tolerance".to_string()]).is_err());
    assert_eq!(run_compare_window_material_screen_equivalent_layer(&[]), 2);
    assert_eq!(
        run_compare_window_material_screen_equivalent_layer(&["only-input.epJSON".to_string()]),
        2
    );
    assert_eq!(
        crate::run_compare_command(&["window-material-screen-equivalent-layer".to_string()]),
        2,
        "the root compare dispatcher must route the new command"
    );
    Ok(())
}

#[test]
fn cli_command_accepts_exact_oracle_files() -> Result<(), Box<dyn std::error::Error>> {
    let directory = unique_temp_directory();
    std::fs::create_dir_all(&directory)?;
    let input_path = directory.join("screen-equivalent-layer.epJSON");
    let eio_path = directory.join("eplusout.eio");
    std::fs::write(
        &input_path,
        with_both_reports(SCREEN_EQUIVALENT_LAYER_EPJSON),
    )?;
    std::fs::write(&eio_path, both_eio())?;

    let exit_code = run_compare_window_material_screen_equivalent_layer(&[
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
        "rusted-energyplus-window-screen-equivalent-layer-cli-{}-{nonce}",
        std::process::id()
    ))
}
