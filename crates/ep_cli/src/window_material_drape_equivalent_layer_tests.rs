use std::path::PathBuf;

use ep_compare::WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER;
use ep_model::TypedModel;
use ep_raw_model::{RawModel, parse_epjson_str};

use super::{
    NumericToleranceMode, compare_window_material_drape_equivalent_layer,
    constructions_report_requested, drape_equivalent_layer_definitions,
    drape_equivalent_layer_header_count, drape_equivalent_layer_occurrences, parse_tolerance_mode,
    run_compare_window_material_drape_equivalent_layer,
};

const DRAPE_EQUIVALENT_LAYER_EPJSON: &str = r#"{
    "WindowMaterial:Drape:EquivalentLayer": {
        "A Unused EQL Drape": {
            "front_side_drape_beam_diffuse_solar_transmittance": 0.1111,
            "back_side_drape_beam_diffuse_solar_transmittance": 0.1222,
            "front_side_drape_beam_diffuse_solar_reflectance": 0.2333,
            "back_side_drape_beam_diffuse_solar_reflectance": 0.2444
        },
        "M Once EQL Drape": {
            "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.02,
            "front_side_drape_beam_diffuse_solar_transmittance": 0.10,
            "back_side_drape_beam_diffuse_solar_transmittance": 0.20,
            "front_side_drape_beam_diffuse_solar_reflectance": 0.30,
            "back_side_drape_beam_diffuse_solar_reflectance": 0.40,
            "drape_beam_beam_visible_transmittance": 0.11,
            "drape_beam_diffuse_visible_transmittance": 0.12,
            "drape_beam_diffuse_visible_reflectance": 0.13,
            "drape_material_infrared_transmittance": 0.03,
            "front_side_drape_material_infrared_emissivity": 0.70,
            "back_side_drape_material_infrared_emissivity": 0.60,
            "width_of_pleated_fabric": 0.0123456,
            "length_of_pleated_fabric": 0.0234567
        },
        "Z Reused EQL Drape": {
            "drape_beam_beam_solar_transmittance_at_normal_incidence": 0.0000123456,
            "front_side_drape_beam_diffuse_solar_transmittance": 0.123456,
            "back_side_drape_beam_diffuse_solar_transmittance": 0.234567,
            "front_side_drape_beam_diffuse_solar_reflectance": 0.345678,
            "back_side_drape_beam_diffuse_solar_reflectance": 0.456789,
            "drape_beam_beam_visible_transmittance": 0.21,
            "drape_beam_diffuse_visible_transmittance": 0.22,
            "drape_beam_diffuse_visible_reflectance": 0.23,
            "drape_material_infrared_transmittance": 0.0000345678,
            "front_side_drape_material_infrared_emissivity": 0.765432,
            "back_side_drape_material_infrared_emissivity": 0.654321,
            "width_of_pleated_fabric": 0.0123456,
            "length_of_pleated_fabric": 0.0234567
        }
    },
    "Construction:WindowEquivalentLayer": {
        "A First EQL Construction": {
            "outside_layer": "Z Reused EQL Drape",
            "layer_2": "M Once EQL Drape"
        },
        "C Last EQL Construction": {
            "outside_layer": "Z Reused EQL Drape"
        }
    }
}"#;

const GENERIC_HEADER: &str = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible";
const A_GENERIC: &str = "Material Details,A UNUSED EQL DRAPE,0.0000,MediumRough,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000";
const M_GENERIC: &str = "Material Details,M ONCE EQL DRAPE,0.0000,MediumRough,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000";
const Z_GENERIC: &str = "Material Details,Z REUSED EQL DRAPE,0.0000,MediumRough,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000,0.0000";
const M_SPECIALIZED: &str = "WindowMaterial:Drape:EquivalentLayer,M ONCE EQL DRAPE,2.0000E-002,0.1000,0.2000,0.3000,0.4000,3.0000E-002,0.7000,0.6000,1.23456E-002,2.34567E-002";
const Z_SPECIALIZED: &str = "WindowMaterial:Drape:EquivalentLayer,Z REUSED EQL DRAPE,1.2346E-005,0.1235,0.2346,0.3457,0.4568,3.4568E-005,0.7654,0.6543,1.23456E-002,2.34567E-002";

fn test_models() -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    compile_models(&with_constructions_report(
        DRAPE_EQUIVALENT_LAYER_EPJSON,
        "Constructions",
    ))
}

fn compile_models(epjson: &str) -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(epjson)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "equivalent-layer drape model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result
        .model
        .ok_or("equivalent-layer drape compiler returned no typed model")?;
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
        WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER,
        Z_SPECIALIZED,
        M_SPECIALIZED,
        Z_SPECIALIZED,
        "",
    ]
    .join("\n")
}

fn materials_only_eio() -> String {
    [GENERIC_HEADER, Z_GENERIC, A_GENERIC, M_GENERIC, ""].join("\n")
}

#[test]
fn rows_include_unused_definitions_and_construction_occurrences_in_order()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let definitions = drape_equivalent_layer_definitions(&model);
    let occurrences = drape_equivalent_layer_occurrences(&raw_model, &model)?;

    assert_eq!(
        definitions
            .iter()
            .map(|row| row.material_name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "A UNUSED EQL DRAPE",
            "M ONCE EQL DRAPE",
            "Z REUSED EQL DRAPE"
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
            ("A FIRST EQL CONSTRUCTION", 1, "Z REUSED EQL DRAPE"),
            ("A FIRST EQL CONSTRUCTION", 2, "M ONCE EQL DRAPE"),
            ("C LAST EQL CONSTRUCTION", 1, "Z REUSED EQL DRAPE"),
        ]
    );
    assert!(
        occurrences
            .iter()
            .all(|row| row.material_name != "A UNUSED EQL DRAPE")
    );
    Ok(())
}

#[test]
fn exact_comparison_gates_generic_header_order_repeats_and_mixed_precision()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let comparison = compare_window_material_drape_equivalent_layer(
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
    assert_eq!(
        comparison.oracle_occurrences[0].material_name,
        "Z REUSED EQL DRAPE"
    );
    assert_eq!(comparison.oracle_occurrences[0].pleated_width_m, 0.0123456);

    let duplicate_header = exact_eio().replacen(
        WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER,
        &format!(
            "{WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER}\n{WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER}"
        ),
        1,
    );
    let duplicate = compare_window_material_drape_equivalent_layer(
        &raw_model,
        &model,
        &duplicate_header,
        NumericToleranceMode::Exact,
    )?;
    assert!(!duplicate.passed);
    assert_eq!(
        duplicate.first_divergence.as_deref(),
        Some("WindowMaterial:Drape:EquivalentLayer header expected 1 observed 2")
    );
    Ok(())
}

#[test]
fn materials_only_suppresses_specialized_header_rows_and_expected_occurrences()
-> Result<(), Box<dyn std::error::Error>> {
    let epjson = with_constructions_report(DRAPE_EQUIVALENT_LAYER_EPJSON, "Materials");
    let (raw_model, model) = compile_models(&epjson)?;
    assert!(!constructions_report_requested(&raw_model)?);

    let comparison = compare_window_material_drape_equivalent_layer(
        &raw_model,
        &model,
        &materials_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.definitions.len(), 3);
    assert!(comparison.occurrences.is_empty());
    assert!(comparison.oracle_occurrences.is_empty());
    assert_eq!(comparison.header_rows, 0);

    let fabricated = format!(
        "{}{WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER}\n{Z_SPECIALIZED}\n",
        materials_only_eio()
    );
    let comparison = compare_window_material_drape_equivalent_layer(
        &raw_model,
        &model,
        &fabricated,
        NumericToleranceMode::Exact,
    )?;
    assert!(!comparison.passed);
    assert_eq!(
        comparison.first_divergence.as_deref(),
        Some("WindowMaterial:Drape:EquivalentLayer header expected 0 observed 1")
    );
    Ok(())
}

#[test]
fn exact_rejects_but_near_accepts_small_4r_and_5r_deltas() -> Result<(), Box<dyn std::error::Error>>
{
    let (raw_model, model) = test_models()?;
    for replacement in [
        Z_SPECIALIZED.replace("0.1235", "0.123501"),
        Z_SPECIALIZED.replace("1.23456E-002", "0.0123465"),
    ] {
        let changed = exact_eio().replacen(Z_SPECIALIZED, &replacement, 1);
        let exact = compare_window_material_drape_equivalent_layer(
            &raw_model,
            &model,
            &changed,
            NumericToleranceMode::Exact,
        )?;
        assert!(!exact.passed);

        let near = compare_window_material_drape_equivalent_layer(
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
fn malformed_specialized_rows_surface_parser_errors() -> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let malformed = exact_eio().replace(
        Z_SPECIALIZED,
        "WindowMaterial:Drape:EquivalentLayer,Z REUSED EQL DRAPE,0.1",
    );
    let error = compare_window_material_drape_equivalent_layer(
        &raw_model,
        &model,
        &malformed,
        NumericToleranceMode::Exact,
    )
    .expect_err("malformed source-shaped row must fail parsing");
    assert!(error.contains("invalid EIO WindowMaterial:Drape:EquivalentLayer"));
    assert!(error.contains("expected exactly 11 data fields"));
    Ok(())
}

#[test]
fn header_tolerance_and_cli_argument_contracts_are_exact() {
    assert_eq!(
        drape_equivalent_layer_header_count(WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER),
        1
    );
    assert_eq!(
        WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER
            .split(',')
            .count(),
        14
    );
    assert_eq!(parse_tolerance_mode(&[]), Ok(NumericToleranceMode::Exact));
    assert_eq!(
        parse_tolerance_mode(&["--tolerance".to_string(), "near".to_string()]),
        Ok(NumericToleranceMode::Near)
    );
    assert!(parse_tolerance_mode(&["--tolerance".to_string()]).is_err());
    assert_eq!(run_compare_window_material_drape_equivalent_layer(&[]), 2);
    assert_eq!(
        run_compare_window_material_drape_equivalent_layer(&["only-input.epJSON".to_string()]),
        2
    );
}

#[test]
fn cli_command_accepts_exact_oracle_files() -> Result<(), Box<dyn std::error::Error>> {
    let directory = unique_temp_directory();
    std::fs::create_dir_all(&directory)?;
    let input_path = directory.join("drape-equivalent-layer.epJSON");
    let eio_path = directory.join("eplusout.eio");
    std::fs::write(
        &input_path,
        with_constructions_report(DRAPE_EQUIVALENT_LAYER_EPJSON, "Constructions"),
    )?;
    std::fs::write(&eio_path, exact_eio())?;

    let exit_code = run_compare_window_material_drape_equivalent_layer(&[
        input_path.display().to_string(),
        eio_path.display().to_string(),
        "--tolerance".to_string(),
        "exact".to_string(),
    ]);
    let cleanup = std::fs::remove_dir_all(&directory);
    cleanup?;
    assert_eq!(exit_code, 0);
    Ok(())
}

fn unique_temp_directory() -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    std::env::temp_dir().join(format!(
        "rusted-energyplus-window-drape-equivalent-layer-cli-{}-{nonce}",
        std::process::id()
    ))
}
