use std::path::PathBuf;

use ep_compare::{EioMaterialDetails, parse_eio_material_details};
use ep_model::TypedModel;
use ep_raw_model::{RawModel, parse_epjson_str};

use super::{
    MATERIAL_DETAILS_HEADER, NumericToleranceMode, compare_window_material_gap,
    construction_report_requests, expected_numeric_fields, forbidden_window_table_shape,
    generic_row_matches, indices_by_material_name, material_details_table_shape,
    parse_tolerance_mode, record_generic_divergence, run_compare_window_material_gap,
    window_material_gap_definitions,
};

const GAP_EPJSON: &str = r#"{
    "WindowMaterial:Gas": {
        "Source Air": {
            "gas_type": "Air",
            "thickness": 0.04
        }
    },
    "WindowMaterial:GasMixture": {
        "Source Mixture": {
            "thickness": 0.03,
            "number_of_gases_in_mixture": 2,
            "gas_1_type": "Air",
            "gas_1_fraction": 0.25,
            "gas_2_type": "Argon",
            "gas_2_fraction": 0.75
        }
    },
    "WindowGap:DeflectionState": {
        "Changed State": {
            "deflected_thickness": 0.5,
            "initial_temperature": 20.0,
            "initial_pressure": 90000.0
        }
    },
    "WindowGap:SupportPillar": {
        "Changed Pillar": {
            "spacing": 0.0001,
            "radius": 0.01
        }
    },
    "WindowMaterial:Gap": {
        "A Different Thickness": {
            "thickness": 0.006,
            "gas_or_gas_mixture_": "Source Mixture",
            "pressure": 1.0
        },
        "M Same Thickness Different State": {
            "thickness": 0.0127,
            "gas_or_gas_mixture_": "Source Mixture",
            "pressure": 87654.321,
            "deflection_state": "Changed State",
            "support_pillar": "Changed Pillar"
        },
        "Z Default Pressure": {
            "thickness": 0.0127,
            "gas_or_gas_mixture_": "Source Air"
        }
    }
}"#;

const A_GENERIC: &str = "Material Details,A DIFFERENT THICKNESS,0.0000,Rough,6.0000E-003,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const M_GENERIC: &str = "Material Details,M SAME THICKNESS DIFFERENT STATE,0.0000,Rough,1.2700E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const Z_GENERIC: &str = "Material Details,Z DEFAULT PRESSURE,0.0000,Rough,1.2700E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const SOURCE_GAS_GENERIC: &str = "Material Details,SOURCE AIR,0.0000,MediumRough,4.0000E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const SOURCE_MIXTURE_GENERIC: &str = "Material Details,SOURCE MIXTURE,0.0000,MediumRough,3.0000E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const MATERIAL_AIR_HEADER: &str = "! <Material:Air>,Material Name,ThermalResistance {m2-K/w}";
const CTF_HEADER: &str = "! <CTF>,Time,Outside,Cross,Inside,Flux (except final one)";

fn compile_models(epjson: &str) -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(epjson)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "WindowMaterial:Gap comparison model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result
        .model
        .ok_or("WindowMaterial:Gap compiler returned no typed model")?;
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
        MATERIAL_AIR_HEADER,
        CTF_HEADER,
        Z_GENERIC,
        SOURCE_GAS_GENERIC,
        M_GENERIC,
        SOURCE_MIXTURE_GENERIC,
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
fn exact_rows_use_only_gap_identity_thickness_fixed_roughness_and_zero_fields()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = compile_models(GAP_EPJSON)?;
    let definitions = window_material_gap_definitions(&model);
    assert_eq!(
        definitions
            .iter()
            .map(|definition| definition.material_name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "A DIFFERENT THICKNESS",
            "M SAME THICKNESS DIFFERENT STATE",
            "Z DEFAULT PRESSURE"
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
        [0.0, 0.006, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]
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
        definitions[1].fields.pressure_pa,
        definitions[2].fields.pressure_pa
    );
    assert_ne!(
        definitions[1].fields.source_material_id(),
        definitions[2].fields.source_material_id()
    );
    assert_ne!(
        definitions[1].fields.deflected_thickness_m,
        definitions[2].fields.deflected_thickness_m
    );
    assert_ne!(
        definitions[1].fields.support_pillar,
        definitions[2].fields.support_pillar
    );
    Ok(())
}

#[test]
fn materials_both_constructions_and_default_lanes_follow_report_activation()
-> Result<(), Box<dyn std::error::Error>> {
    let (both_raw, both_model) = compile_models(&with_both_reports(GAP_EPJSON))?;
    let both = compare_window_material_gap(
        &both_raw,
        &both_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(both.passed, "{:?}", both.first_divergence);
    assert_eq!(both.oracle_material_details.len(), 5);
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
        compile_models(&with_single_report(GAP_EPJSON, "Materials"))?;
    let materials = compare_window_material_gap(
        &materials_raw,
        &materials_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(materials.passed, "{:?}", materials.first_divergence);
    assert!(materials.report_requests.materials);
    assert!(!materials.report_requests.constructions);

    let (constructions_raw, constructions_model) =
        compile_models(&with_single_report(GAP_EPJSON, "Constructions"))?;
    let constructions = compare_window_material_gap(
        &constructions_raw,
        &constructions_model,
        &no_material_details_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(constructions.passed, "{:?}", constructions.first_divergence);
    assert!(constructions.oracle_material_details.is_empty());
    assert!(!constructions.report_requests.materials);
    assert!(constructions.report_requests.constructions);

    let (default_raw, default_model) = compile_models(GAP_EPJSON)?;
    let default = compare_window_material_gap(
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
    Ok(())
}

#[test]
fn matching_requires_every_gap_once_and_allows_source_material_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_single_report(GAP_EPJSON, "Materials"))?;
    let baseline = compare_window_material_gap(
        &raw_model,
        &model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(baseline.passed, "{:?}", baseline.first_divergence);
    let indices = indices_by_material_name(&baseline.oracle_material_details);
    assert_eq!(indices.get("SOURCE AIR").map(Vec::len), Some(1));
    assert_eq!(indices.get("SOURCE MIXTURE").map(Vec::len), Some(1));

    let missing_eio = materials_eio().replace(&format!("{M_GENERIC}\n"), "");
    let missing = compare_window_material_gap(
        &raw_model,
        &model,
        &missing_eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(!missing.passed);
    assert!(
        missing
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("M SAME THICKNESS"))
    );

    let duplicate_eio = format!("{}{Z_GENERIC}\n", materials_eio());
    let duplicate = compare_window_material_gap(
        &raw_model,
        &model,
        &duplicate_eio,
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
    let (_raw_model, model) = compile_models(GAP_EPJSON)?;
    let definition = window_material_gap_definitions(&model)
        .into_iter()
        .find(|definition| definition.material_name.starts_with('Z'))
        .ok_or("missing default-pressure gap")?;
    let base = parsed_row(Z_GENERIC);

    let mut mutations = Vec::<(&str, EioMaterialDetails)>::new();
    let mut row = base.clone();
    row.material_name = "ANOTHER GAP".to_string();
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
    normalized_name.material_name = "  z default pressure  ".to_string();
    assert!(generic_row_matches(
        &definition,
        &normalized_name,
        NumericToleranceMode::Exact
    ));

    let mut small_delta = base.clone();
    small_delta.thermal_resistance_m2_k_per_w += 0.000001;
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
    large_delta.thermal_resistance_m2_k_per_w += 0.001;
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
    let (raw_model, model) = compile_models(&with_single_report(GAP_EPJSON, "Materials"))?;

    let malformed_header = MATERIAL_DETAILS_HEADER.replacen(",Material Name", ", Material Name", 1);
    let malformed_eio = [
        malformed_header.as_str(),
        Z_GENERIC,
        M_GENERIC,
        A_GENERIC,
        "",
    ]
    .join("\n");
    let malformed = compare_window_material_gap(
        &raw_model,
        &model,
        &malformed_eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(!malformed.passed);
    assert_eq!(malformed.material_details_shape.candidate_header_rows, 1);
    assert_eq!(malformed.material_details_shape.exact_header_rows, 0);

    let duplicate_header_eio = [
        MATERIAL_DETAILS_HEADER,
        MATERIAL_DETAILS_HEADER,
        Z_GENERIC,
        M_GENERIC,
        A_GENERIC,
        "",
    ]
    .join("\n");
    let duplicate_header = compare_window_material_gap(
        &raw_model,
        &model,
        &duplicate_header_eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(!duplicate_header.passed);
    assert_eq!(duplicate_header.material_details_shape.exact_header_rows, 2);

    let row_first_eio = [Z_GENERIC, MATERIAL_DETAILS_HEADER, M_GENERIC, A_GENERIC, ""].join("\n");
    let row_first = compare_window_material_gap(
        &raw_model,
        &model,
        &row_first_eio,
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
        "{MATERIAL_DETAILS_HEADER}\nMaterial Details,A DIFFERENT THICKNESS,0.0,Rough,0.006,0,0,0,0,0\n"
    );
    let error = compare_window_material_gap(
        &raw_model,
        &model,
        &malformed_row,
        NumericToleranceMode::Exact,
    )
    .expect_err("a malformed Material Details payload must fail parsing");
    assert!(error.contains("invalid EIO Material Details"));

    let (default_raw, default_model) = compile_models(GAP_EPJSON)?;
    let unexpected = compare_window_material_gap(
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
fn specialized_gap_glazing_and_window_construction_tables_are_forbidden()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&with_single_report(GAP_EPJSON, "Materials"))?;
    let forbidden_lines = [
        "! <WindowMaterial:Gap>, Material Name, Gap State",
        "WindowMaterial:Gap,A DIFFERENT THICKNESS,FORBIDDEN",
        "! <WindowMaterial:Glazing>, Material Name, Optical Data Type",
        "WindowMaterial:Glazing,A DIFFERENT THICKNESS,SpectralAverage",
        "! <WindowConstruction>,Construction Name,Index,#Layers",
        "WindowConstruction,FORBIDDEN COMPLEX WINDOW,1,1",
    ];

    for forbidden_line in forbidden_lines {
        let eio = format!("{}{forbidden_line}\n", materials_eio());
        let comparison =
            compare_window_material_gap(&raw_model, &model, &eio, NumericToleranceMode::Exact)?;
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
fn selectors_are_normalized_but_blank_unknown_and_wrong_types_are_bounded()
-> Result<(), Box<dyn std::error::Error>> {
    let normalized_selectors = with_report_fields(
        GAP_EPJSON,
        "\n            \"details_type_1\": \" materials \",\n            \"details_type_2\": \"cOnStRuCtIoNs\"\n        ",
    );
    let normalized_raw = parse_epjson_str(&normalized_selectors)?;
    let requests = construction_report_requests(&normalized_raw)?;
    assert!(requests.materials);
    assert!(requests.constructions);

    let blank_selector = with_report_fields(
        GAP_EPJSON,
        "\n            \"details_type_1\": \"   \",\n            \"details_type_2\": \"Materials\"\n        ",
    );
    let blank_raw = parse_epjson_str(&blank_selector)?;
    let requests = construction_report_requests(&blank_raw)?;
    assert!(requests.materials);
    assert!(!requests.constructions);

    let invalid_selector =
        with_report_fields(GAP_EPJSON, "\n            \"details_type_1\": 42\n        ");
    let invalid_raw = parse_epjson_str(&invalid_selector)?;
    let error = construction_report_requests(&invalid_raw)
        .expect_err("non-string Output:Constructions selector must fail");
    assert!(error.contains("must be a string"));

    let unsupported_selector = with_report_fields(
        GAP_EPJSON,
        "\n            \"details_type_1\": \"Bogus\"\n        ",
    );
    let unsupported_raw = parse_epjson_str(&unsupported_selector)?;
    let error = construction_report_requests(&unsupported_raw)
        .expect_err("unsupported Output:Constructions selector must fail");
    assert!(error.contains("must be blank, Constructions, or Materials"));
    assert!(error.contains("Bogus"));
    Ok(())
}

#[test]
fn options_cli_dispatch_files_and_no_gap_object_are_bounded()
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

    assert_eq!(run_compare_window_material_gap(&[]), 2);
    assert_eq!(
        run_compare_window_material_gap(&["only-input.epJSON".to_string()]),
        2
    );
    assert_eq!(
        run_compare_window_material_gap(&[
            "unused.epJSON".to_string(),
            "unused.eio".to_string(),
            "--tolerance".to_string(),
            "loose".to_string(),
        ]),
        2
    );

    let temp = unique_temp_directory();
    std::fs::create_dir_all(&temp)?;
    let epjson_path = temp.join("window-material-gap.epJSON");
    let no_gap_path = temp.join("no-gap.epJSON");
    let eio_path = temp.join("eplusout.eio");
    std::fs::write(&epjson_path, with_both_reports(GAP_EPJSON))?;
    std::fs::write(
        &no_gap_path,
        r#"{
            "WindowMaterial:Gas": {
                "Source Air": {
                    "gas_type": "Air",
                    "thickness": 0.0127
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
    assert_eq!(run_compare_window_material_gap(&args), 0);
    let mut dispatch_args = vec!["window-material-gap".to_string()];
    dispatch_args.extend(args.clone());
    assert_eq!(crate::run_compare_command(&dispatch_args), 0);
    let no_gap_args = vec![
        no_gap_path.display().to_string(),
        eio_path.display().to_string(),
    ];
    assert_eq!(run_compare_window_material_gap(&no_gap_args), 1);
    std::fs::remove_dir_all(&temp)?;
    Ok(())
}

fn unique_temp_directory() -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "rusted-energyplus-window-material-gap-{}-{nonce}",
        std::process::id()
    ))
}
