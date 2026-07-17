use std::path::PathBuf;

use ep_compare::{EioMaterialDetails, parse_eio_material_details};
use ep_model::TypedModel;
use ep_raw_model::{RawModel, parse_epjson_str};

use super::{
    MATERIAL_DETAILS_HEADER, NumericToleranceMode, compare_window_material_simple_glazing_system,
    construction_report_requests, expected_numeric_fields, forbidden_window_table_shape,
    generic_row_matches, indices_by_material_name, material_details_table_shape,
    parse_tolerance_mode, record_generic_divergence,
    run_compare_window_material_simple_glazing_system, simple_glazing_definitions,
};

const SIMPLE_GLAZING_EPJSON: &str = r#"{
    "WindowMaterial:SimpleGlazingSystem": {
        "A Distinct U Simple Glazing": {
            "u_factor": 5.0,
            "solar_heat_gain_coefficient": 0.7,
            "visible_transmittance": 0.4
        },
        "M Explicit Visible Same U Simple Glazing": {
            "u_factor": 2.7,
            "solar_heat_gain_coefficient": 0.8,
            "visible_transmittance": 0.6
        },
        "Z Default Visible Simple Glazing": {
            "u_factor": 2.7,
            "solar_heat_gain_coefficient": 0.4
        }
    }
}"#;

const A_GENERIC: &str = "Material Details,A DISTINCT U SIMPLE GLAZING,3.3019E-002,VerySmooth,2.0000E-003,6.057E-002,0.000,0.000,0.8400,0.0000,0.0000";
const M_GENERIC: &str = "Material Details,M EXPLICIT VISIBLE SAME U SIMPLE GLAZING,0.1993,VerySmooth,2.3314E-002,0.117,0.000,0.000,0.8400,0.0000,0.0000";
const Z_GENERIC: &str = "Material Details,Z DEFAULT VISIBLE SIMPLE GLAZING,0.1993,VerySmooth,2.3314E-002,0.117,0.000,0.000,0.8400,0.0000,0.0000";
const UNRELATED_GENERIC: &str = "Material Details,UNRELATED OPAQUE HOST,0.2000,Rough,0.1000,0.500,800.000,900.000,0.9000,0.7000,0.7000";

fn compile_models(epjson: &str) -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(epjson)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "simple-glazing comparison model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result
        .model
        .ok_or("simple-glazing compiler returned no typed model")?;
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

fn materials_eio() -> String {
    [
        MATERIAL_DETAILS_HEADER,
        Z_GENERIC,
        UNRELATED_GENERIC,
        M_GENERIC,
        A_GENERIC,
        "",
    ]
    .join("\n")
}

fn no_material_details_eio() -> String {
    "Program Version,EnergyPlus, Version 26.1.0\nConstruction CTF,IGNORED\n".to_string()
}

fn parsed_row(row: &str) -> EioMaterialDetails {
    parse_eio_material_details(row)
        .expect("test Material Details row should parse")
        .remove(0)
}

#[test]
fn exact_rows_apply_source_rounding_and_exclude_optical_inputs()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = compile_models(SIMPLE_GLAZING_EPJSON)?;
    let definitions = simple_glazing_definitions(&model);
    assert_eq!(
        definitions
            .iter()
            .map(|definition| definition.material_name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "A DISTINCT U SIMPLE GLAZING",
            "M EXPLICIT VISIBLE SAME U SIMPLE GLAZING",
            "Z DEFAULT VISIBLE SIMPLE GLAZING"
        ]
    );

    for (definition, row) in definitions
        .iter()
        .zip([A_GENERIC, M_GENERIC, Z_GENERIC].map(parsed_row))
    {
        assert!(generic_row_matches(
            definition,
            &row,
            NumericToleranceMode::Exact
        ));
    }

    let a_expected = expected_numeric_fields(&definitions[0], &parsed_row(A_GENERIC))
        .ok_or("expected source-formatted A fields")?;
    assert_eq!(
        a_expected.map(|(_field, expected, _observed)| expected),
        [0.033019, 0.002, 0.06057, 0.0, 0.0, 0.84, 0.0, 0.0]
    );
    let m_expected = expected_numeric_fields(&definitions[1], &parsed_row(M_GENERIC))
        .ok_or("expected source-formatted M fields")?;
    let z_expected = expected_numeric_fields(&definitions[2], &parsed_row(Z_GENERIC))
        .ok_or("expected source-formatted Z fields")?;
    assert_eq!(
        m_expected.map(|(_field, expected, _observed)| expected),
        z_expected.map(|(_field, expected, _observed)| expected)
    );
    assert_ne!(
        definitions[1].fields.solar_heat_gain_coefficient,
        definitions[2].fields.solar_heat_gain_coefficient
    );
    assert_ne!(
        definitions[1]
            .fields
            .input_visible_transmittance_at_normal_incidence,
        definitions[2]
            .fields
            .input_visible_transmittance_at_normal_incidence
    );
    Ok(())
}

#[test]
fn materials_both_constructions_and_default_lanes_follow_report_activation()
-> Result<(), Box<dyn std::error::Error>> {
    let (both_raw, both_model) = compile_models(&with_both_reports(SIMPLE_GLAZING_EPJSON))?;
    let both = compare_window_material_simple_glazing_system(
        &both_raw,
        &both_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(both.passed, "{:?}", both.first_divergence);
    assert_eq!(both.oracle_material_details.len(), 4);
    assert_eq!(both.material_details_shape.exact_header_rows, 1);
    assert_eq!(both.forbidden_window_shape.total_rows(), 0);
    assert_eq!(
        both.report_requests,
        super::ConstructionReportRequests {
            constructions: true,
            materials: true,
        }
    );

    let (materials_raw, materials_model) =
        compile_models(&with_single_report(SIMPLE_GLAZING_EPJSON, "Materials"))?;
    let materials = compare_window_material_simple_glazing_system(
        &materials_raw,
        &materials_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(materials.passed, "{:?}", materials.first_divergence);
    assert!(materials.report_requests.materials);
    assert!(!materials.report_requests.constructions);

    let (constructions_raw, constructions_model) =
        compile_models(&with_single_report(SIMPLE_GLAZING_EPJSON, "Constructions"))?;
    let constructions = compare_window_material_simple_glazing_system(
        &constructions_raw,
        &constructions_model,
        &no_material_details_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(constructions.passed, "{:?}", constructions.first_divergence);
    assert!(constructions.oracle_material_details.is_empty());
    assert!(!constructions.report_requests.materials);
    assert!(constructions.report_requests.constructions);

    let (default_raw, default_model) = compile_models(SIMPLE_GLAZING_EPJSON)?;
    let default = compare_window_material_simple_glazing_system(
        &default_raw,
        &default_model,
        &no_material_details_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(default.passed, "{:?}", default.first_divergence);
    assert_eq!(
        default.report_requests,
        super::ConstructionReportRequests::default()
    );

    let normalized_selectors = with_report_fields(
        SIMPLE_GLAZING_EPJSON,
        "\n            \"details_type_1\": \" materials \",\n            \"details_type_2\": \"cOnStRuCtIoNs\"\n        ",
    );
    let normalized_raw = parse_epjson_str(&normalized_selectors)?;
    let requests = construction_report_requests(&normalized_raw)?;
    assert!(requests.materials);
    assert!(requests.constructions);
    Ok(())
}

#[test]
fn matching_requires_every_definition_once_and_allows_unrelated_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) =
        compile_models(&with_single_report(SIMPLE_GLAZING_EPJSON, "Materials"))?;
    let baseline = compare_window_material_simple_glazing_system(
        &raw_model,
        &model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(baseline.passed, "{:?}", baseline.first_divergence);
    let indices = indices_by_material_name(&baseline.oracle_material_details);
    assert_eq!(indices.get("UNRELATED OPAQUE HOST").map(Vec::len), Some(1));

    let missing = materials_eio().replace(&format!("{M_GENERIC}\n"), "");
    let missing = compare_window_material_simple_glazing_system(
        &raw_model,
        &model,
        &missing,
        NumericToleranceMode::Exact,
    )?;
    assert!(!missing.passed);
    assert!(
        missing
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("M EXPLICIT VISIBLE"))
    );

    let duplicate = format!("{}{Z_GENERIC}\n", materials_eio());
    let duplicate = compare_window_material_simple_glazing_system(
        &raw_model,
        &model,
        &duplicate,
        NumericToleranceMode::Exact,
    )?;
    assert!(!duplicate.passed);
    assert!(
        duplicate
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("observed 2"))
    );
    Ok(())
}

#[test]
fn every_generic_field_diverges_and_near_uses_source_rounded_expected()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = compile_models(SIMPLE_GLAZING_EPJSON)?;
    let definition = simple_glazing_definitions(&model)
        .into_iter()
        .find(|definition| definition.material_name.starts_with('Z'))
        .ok_or("missing default-visible definition")?;
    let base = parsed_row(Z_GENERIC);

    let mut mutations = Vec::<(&str, EioMaterialDetails)>::new();
    let mut row = base.clone();
    row.material_name = "ANOTHER SIMPLE GLAZING".to_string();
    mutations.push(("material_name", row));
    let mut row = base.clone();
    row.roughness = "Smooth".to_string();
    mutations.push(("roughness", row));
    let mut row = base.clone();
    row.thermal_resistance_m2_k_per_w += 0.001;
    mutations.push(("thermal_resistance_m2_k_per_w", row));
    let mut row = base.clone();
    row.thickness_m += 0.001;
    mutations.push(("thickness_m", row));
    let mut row = base.clone();
    row.conductivity_w_per_m_k += 0.001;
    mutations.push(("conductivity_w_per_m_k", row));
    let mut row = base.clone();
    row.density_kg_per_m3 += 0.001;
    mutations.push(("density_kg_per_m3", row));
    let mut row = base.clone();
    row.specific_heat_j_per_kg_k += 0.001;
    mutations.push(("specific_heat_j_per_kg_k", row));
    let mut row = base.clone();
    row.thermal_absorptance += 0.001;
    mutations.push(("thermal_absorptance", row));
    let mut row = base.clone();
    row.solar_absorptance += 0.001;
    mutations.push(("solar_absorptance", row));
    let mut row = base.clone();
    row.visible_absorptance += 0.001;
    mutations.push(("visible_absorptance", row));

    for (field, row) in mutations {
        assert!(
            !generic_row_matches(&definition, &row, NumericToleranceMode::Exact),
            "mutation of {field} must fail"
        );
        let mut divergence = None;
        record_generic_divergence(
            &mut divergence,
            &definition,
            &row,
            NumericToleranceMode::Exact,
        );
        assert!(
            divergence
                .as_deref()
                .is_some_and(|value| value.contains(field)),
            "unexpected divergence for {field}: {divergence:?}"
        );
    }

    let mut normalized_name = base.clone();
    normalized_name.material_name = "  z default visible simple glazing  ".to_string();
    assert!(generic_row_matches(
        &definition,
        &normalized_name,
        NumericToleranceMode::Exact
    ));

    let mut small_delta = base.clone();
    small_delta.thermal_absorptance += 0.000005;
    assert!(!generic_row_matches(
        &definition,
        &small_delta,
        NumericToleranceMode::Exact
    ));
    assert!(generic_row_matches(
        &definition,
        &small_delta,
        NumericToleranceMode::Near
    ));
    let mut large_delta = base;
    large_delta.thermal_absorptance += 0.001;
    assert!(!generic_row_matches(
        &definition,
        &large_delta,
        NumericToleranceMode::Near
    ));
    Ok(())
}

#[test]
fn malformed_duplicate_and_out_of_order_material_details_fail_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) =
        compile_models(&with_single_report(SIMPLE_GLAZING_EPJSON, "Materials"))?;

    let malformed_header = MATERIAL_DETAILS_HEADER.replacen(",Material Name", ", Material Name", 1);
    let malformed = [
        malformed_header.as_str(),
        Z_GENERIC,
        M_GENERIC,
        A_GENERIC,
        "",
    ]
    .join("\n");
    let malformed = compare_window_material_simple_glazing_system(
        &raw_model,
        &model,
        &malformed,
        NumericToleranceMode::Exact,
    )?;
    assert!(!malformed.passed);
    assert_eq!(malformed.material_details_shape.candidate_header_rows, 1);
    assert_eq!(malformed.material_details_shape.exact_header_rows, 0);

    let duplicate_header = [
        MATERIAL_DETAILS_HEADER,
        MATERIAL_DETAILS_HEADER,
        Z_GENERIC,
        M_GENERIC,
        A_GENERIC,
        "",
    ]
    .join("\n");
    let duplicate_header = compare_window_material_simple_glazing_system(
        &raw_model,
        &model,
        &duplicate_header,
        NumericToleranceMode::Exact,
    )?;
    assert!(!duplicate_header.passed);
    assert_eq!(duplicate_header.material_details_shape.exact_header_rows, 2);

    let row_first = [Z_GENERIC, MATERIAL_DETAILS_HEADER, M_GENERIC, A_GENERIC, ""].join("\n");
    let row_first = compare_window_material_simple_glazing_system(
        &raw_model,
        &model,
        &row_first,
        NumericToleranceMode::Exact,
    )?;
    assert!(!row_first.passed);
    assert_eq!(
        row_first
            .material_details_shape
            .first_row_without_preceding_exact_header,
        Some(1)
    );

    let malformed_row = format!(
        "{MATERIAL_DETAILS_HEADER}\nMaterial Details,A DISTINCT U SIMPLE GLAZING,0.0330,VerySmooth,0.002,0.061,0,0,0.84,0\n"
    );
    let error = compare_window_material_simple_glazing_system(
        &raw_model,
        &model,
        &malformed_row,
        NumericToleranceMode::Exact,
    )
    .expect_err("a malformed Material Details payload must fail parsing");
    assert!(error.contains("invalid EIO Material Details"));

    let (default_raw, default_model) = compile_models(SIMPLE_GLAZING_EPJSON)?;
    let unexpected = compare_window_material_simple_glazing_system(
        &default_raw,
        &default_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(!unexpected.passed);
    assert_eq!(
        material_details_table_shape(&materials_eio()).exact_header_rows,
        1
    );
    Ok(())
}

#[test]
fn specialized_glazing_and_window_construction_tables_are_forbidden()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) =
        compile_models(&with_single_report(SIMPLE_GLAZING_EPJSON, "Materials"))?;
    let forbidden_lines = [
        "! <WindowMaterial:Glazing>, Material Name, Optical Data Type",
        "WindowMaterial:Glazing,Z DEFAULT VISIBLE SIMPLE GLAZING,SpectralAverage",
        "! <WindowConstruction>,Construction Name,Index,#Layers",
        "WindowConstruction,FORBIDDEN SIMPLE WINDOW,1,1",
    ];

    for forbidden_line in forbidden_lines {
        let eio = format!("{}{forbidden_line}\n", materials_eio());
        let comparison = compare_window_material_simple_glazing_system(
            &raw_model,
            &model,
            &eio,
            NumericToleranceMode::Exact,
        )?;
        assert!(
            !comparison.passed,
            "line must be forbidden: {forbidden_line}"
        );
        assert_eq!(comparison.forbidden_window_shape.total_rows(), 1);
        assert!(
            comparison
                .first_divergence
                .as_deref()
                .is_some_and(|value| value.contains("forbidden specialized window"))
        );
        assert_eq!(forbidden_window_table_shape(&eio).total_rows(), 1);
    }
    Ok(())
}

#[test]
fn options_selector_types_cli_files_and_no_simple_object_are_bounded()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(parse_tolerance_mode(&[])?, NumericToleranceMode::Exact);
    assert_eq!(
        parse_tolerance_mode(&["--tolerance".to_string(), "exact".to_string()])?,
        NumericToleranceMode::Exact
    );
    assert_eq!(
        parse_tolerance_mode(&["--tolerance".to_string(), "near".to_string()])?,
        NumericToleranceMode::Near
    );
    assert!(parse_tolerance_mode(&["--tolerance".to_string(), "loose".to_string()]).is_err());
    assert!(parse_tolerance_mode(&["--unknown".to_string()]).is_err());

    let invalid_selector = with_report_fields(
        SIMPLE_GLAZING_EPJSON,
        "\n            \"details_type_1\": 42\n        ",
    );
    let invalid_raw = parse_epjson_str(&invalid_selector)?;
    let error = construction_report_requests(&invalid_raw)
        .expect_err("non-string Output:Constructions selector must fail");
    assert!(error.contains("must be a string"));

    let unsupported_selector = with_report_fields(
        SIMPLE_GLAZING_EPJSON,
        "\n            \"details_type_1\": \"Bogus\"\n        ",
    );
    let unsupported_raw = parse_epjson_str(&unsupported_selector)?;
    let error = construction_report_requests(&unsupported_raw)
        .expect_err("unsupported Output:Constructions selector must fail");
    assert!(error.contains("must be blank, Constructions, or Materials"));
    assert!(error.contains("Bogus"));

    assert_eq!(run_compare_window_material_simple_glazing_system(&[]), 2);
    assert_eq!(
        run_compare_window_material_simple_glazing_system(&["only-input.epJSON".to_string()]),
        2
    );
    assert_eq!(
        run_compare_window_material_simple_glazing_system(&[
            "unused.epJSON".to_string(),
            "unused.eio".to_string(),
            "--tolerance".to_string(),
            "loose".to_string(),
        ]),
        2
    );

    let temp = unique_temp_directory();
    std::fs::create_dir_all(&temp)?;
    let epjson_path = temp.join("window-material-simple-glazing-system.epJSON");
    let no_simple_path = temp.join("no-simple.epJSON");
    let eio_path = temp.join("eplusout.eio");
    std::fs::write(&epjson_path, with_both_reports(SIMPLE_GLAZING_EPJSON))?;
    std::fs::write(
        &no_simple_path,
        r#"{
            "Material": {
                "Opaque Only": {
                    "roughness": "Rough",
                    "thickness": 0.1,
                    "conductivity": 0.5,
                    "density": 800.0,
                    "specific_heat": 900.0
                }
            }
        }"#,
    )?;
    std::fs::write(&eio_path, materials_eio())?;
    let args = vec![
        epjson_path.display().to_string(),
        eio_path.display().to_string(),
        "--tolerance".to_string(),
        "exact".to_string(),
    ];
    assert_eq!(run_compare_window_material_simple_glazing_system(&args), 0);
    let no_simple_args = vec![
        no_simple_path.display().to_string(),
        eio_path.display().to_string(),
    ];
    assert_eq!(
        run_compare_window_material_simple_glazing_system(&no_simple_args),
        1
    );
    std::fs::remove_dir_all(&temp)?;
    Ok(())
}

fn unique_temp_directory() -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "rusted-energyplus-window-simple-glazing-{}-{nonce}",
        std::process::id()
    ))
}
