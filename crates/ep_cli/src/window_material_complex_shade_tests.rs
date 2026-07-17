use std::path::PathBuf;

use ep_compare::{EioMaterialDetails, parse_eio_material_details};
use ep_model::TypedModel;
use ep_raw_model::{RawModel, parse_epjson_str};

use super::{
    CASE_ID, COMPLEX_SHADE_ROUGHNESS, ConstructionReportRequests, MATERIAL_DETAILS_HEADER,
    NumericToleranceMode, SOURCE_IDF_MATERIAL_ORDER, compare_window_material_complex_shade,
    construction_report_requests, expected_numeric_fields, forbidden_window_table_shape,
    generic_row_matches, indices_by_material_name, material_details_table_shape,
    parse_tolerance_mode, record_generic_divergence, run_compare_window_material_complex_shade,
    window_material_complex_shade_definitions,
};

const COMPLEX_SHADE_EPJSON: &str = r#"{
    "WindowMaterial:ComplexShade": {
        "Z Full Defaults": {},
        "Y Bsdf Custom Base": {
            "layer_type": "BSDF",
            "thickness": 0.003,
            "conductivity": 2.0,
            "ir_transmittance": 0.25,
            "front_emissivity": 0.35,
            "back_emissivity": 0.80,
            "top_opening_multiplier": 0.1,
            "bottom_opening_multiplier": 0.2,
            "left_side_opening_multiplier": 0.3,
            "right_side_opening_multiplier": 0.4,
            "front_opening_multiplier": 0.5,
            "slat_width": 0.020,
            "slat_spacing": 0.015,
            "slat_thickness": 0.001,
            "slat_angle": -30.0,
            "slat_conductivity": 100.0,
            "slat_curve": 0.010
        },
        "X Other Nonvenetian Subhalf": {
            "layer_type": "OtherShadingType",
            "thickness": 0.004,
            "conductivity": 3.0,
            "ir_transmittance": 0.1,
            "front_emissivity": 0.4,
            "back_emissivity": 0.9,
            "slat_width": 0.040,
            "slat_curve": 0.001
        },
        "W Perforated Defaults": {"layer_type": "Perforated"},
        "V Venetian H Equal Half": {
            "layer_type": "VenetianHorizontal",
            "slat_width": 0.016,
            "slat_curve": 0.008
        },
        "U Venetian V Flat": {
            "layer_type": "VenetianVertical",
            "slat_curve": 0.0
        },
        "T Woven Defaults": {"layer_type": "Woven"}
    }
}"#;

const Z_GENERIC: &str = "Material Details,Z FULL DEFAULTS,0.0000,Rough,2.0000E-003,1.000,0.000,0.000,0.8400,0.0000,0.0000";
const Y_GENERIC: &str = "Material Details,Y BSDF CUSTOM BASE,0.0000,Rough,3.0000E-003,2.000,0.000,0.000,0.8000,0.0000,0.0000";
const X_GENERIC: &str = "Material Details,X OTHER NONVENETIAN SUBHALF,0.0000,Rough,4.0000E-003,3.000,0.000,0.000,0.9000,0.0000,0.0000";
const W_GENERIC: &str = "Material Details,W PERFORATED DEFAULTS,0.0000,Rough,2.0000E-003,1.000,0.000,0.000,0.8400,0.0000,0.0000";
const V_GENERIC: &str = "Material Details,V VENETIAN H EQUAL HALF,0.0000,Rough,2.0000E-003,1.000,0.000,0.000,0.8400,0.0000,0.0000";
const U_GENERIC: &str = "Material Details,U VENETIAN V FLAT,0.0000,Rough,2.0000E-003,1.000,0.000,0.000,0.8400,0.0000,0.0000";
const T_GENERIC: &str = "Material Details,T WOVEN DEFAULTS,0.0000,Rough,2.0000E-003,1.000,0.000,0.000,0.8400,0.0000,0.0000";
const SOURCE_ROWS: [&str; 7] = [
    Z_GENERIC, Y_GENERIC, X_GENERIC, W_GENERIC, V_GENERIC, U_GENERIC, T_GENERIC,
];
const MATERIAL_AIR_HEADER: &str = "! <Material:Air>,Material Name,ThermalResistance {m2-K/w}";
const EMPTY_CTF_HEADERS: [&str; 4] = [
    "! <CTF>,Time,Outside,Cross,Inside,Flux (except final one)",
    "! <CTF Source/Sink>,Time,Outside,Cross,Inside,Flux,Source/Sink",
    "! <Construction CTF>,Construction Name,Index,#Layers",
    "! <Construction Internal Source Location>,Construction Name,Source After Layer",
];

fn compile_models(epjson: &str) -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(epjson)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "WindowMaterial:ComplexShade comparison model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result
        .model
        .ok_or("WindowMaterial:ComplexShade compiler returned no typed model")?;
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
    std::iter::once(MATERIAL_DETAILS_HEADER)
        .chain(std::iter::once(MATERIAL_AIR_HEADER))
        .chain(EMPTY_CTF_HEADERS)
        .chain(SOURCE_ROWS)
        .chain(std::iter::once(""))
        .collect::<Vec<_>>()
        .join("\n")
}

fn constructions_only_eio() -> String {
    std::iter::once("Program Version,EnergyPlus, Version 26.1.0")
        .chain(EMPTY_CTF_HEADERS)
        .chain(std::iter::once(""))
        .collect::<Vec<_>>()
        .join("\n")
}

fn blank_eio() -> String {
    "Program Version,EnergyPlus, Version 26.1.0\n".to_string()
}

fn parsed_row(row: &str) -> EioMaterialDetails {
    parse_eio_material_details(row)
        .expect("test Material Details row should parse")
        .remove(0)
}

#[test]
fn exact_rows_lock_source_order_tokens_and_back_emissivity_projection()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = compile_models(COMPLEX_SHADE_EPJSON)?;
    let definitions = window_material_complex_shade_definitions(&model);
    assert_eq!(CASE_ID, "window_material_complex_shade_001");
    assert_eq!(COMPLEX_SHADE_ROUGHNESS, "Rough");
    assert_eq!(
        definitions
            .iter()
            .map(|definition| definition.material_name.as_str())
            .collect::<Vec<_>>(),
        SOURCE_IDF_MATERIAL_ORDER
    );

    for (definition, row) in definitions.iter().zip(SOURCE_ROWS.map(parsed_row)) {
        assert!(generic_row_matches(
            definition,
            &row,
            NumericToleranceMode::Exact
        ));
    }

    for (definition_index, expected) in [
        [0.0, 0.002, 1.0, 0.0, 0.0, 0.84, 0.0, 0.0],
        [0.0, 0.003, 2.0, 0.0, 0.0, 0.80, 0.0, 0.0],
        [0.0, 0.004, 3.0, 0.0, 0.0, 0.90, 0.0, 0.0],
    ]
    .into_iter()
    .enumerate()
    {
        let actual = expected_numeric_fields(
            &definitions[definition_index],
            &parsed_row(SOURCE_ROWS[definition_index]),
        )
        .ok_or("expected source-formatted fields")?
        .map(|(_field, expected, _observed)| expected);
        assert_eq!(actual, expected);
    }

    let bsdf = &definitions[1].fields;
    assert_ne!(
        bsdf.front_infrared_emissivity,
        bsdf.back_infrared_emissivity
    );
    assert_eq!(bsdf.thermal_absorptance, bsdf.back_infrared_emissivity);
    let nonvenetian = &definitions[2].fields;
    assert!(nonvenetian.slat_curvature_radius_m < nonvenetian.slat_width_m / 2.0);
    assert_eq!(
        nonvenetian.thermal_absorptance,
        nonvenetian.back_infrared_emissivity
    );
    Ok(())
}

#[test]
fn both_materials_constructions_and_blank_lanes_follow_exact_report_activation()
-> Result<(), Box<dyn std::error::Error>> {
    let (both_raw, both_model) = compile_models(&with_both_reports(COMPLEX_SHADE_EPJSON))?;
    let both = compare_window_material_complex_shade(
        &both_raw,
        &both_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(both.passed, "{:?}", both.first_divergence);
    assert_eq!(both.definitions.len(), 7);
    assert_eq!(both.oracle_material_details.len(), 7);
    assert_eq!(both.material_details_shape.exact_header_rows, 1);
    assert_eq!(both.material_details_shape.data_rows, 7);
    assert_eq!(both.forbidden_window_shape.total_rows(), 0);
    assert_eq!(
        both.report_requests,
        ConstructionReportRequests {
            constructions: true,
            materials: true,
        }
    );

    let (materials_raw, materials_model) =
        compile_models(&with_single_report(COMPLEX_SHADE_EPJSON, "Materials"))?;
    let materials = compare_window_material_complex_shade(
        &materials_raw,
        &materials_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(materials.passed, "{:?}", materials.first_divergence);
    assert!(materials.report_requests.materials);
    assert!(!materials.report_requests.constructions);

    let (constructions_raw, constructions_model) =
        compile_models(&with_single_report(COMPLEX_SHADE_EPJSON, "Constructions"))?;
    let constructions = compare_window_material_complex_shade(
        &constructions_raw,
        &constructions_model,
        &constructions_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(constructions.passed, "{:?}", constructions.first_divergence);
    assert!(constructions.oracle_material_details.is_empty());
    assert!(!constructions.report_requests.materials);
    assert!(constructions.report_requests.constructions);

    let (default_raw, default_model) = compile_models(COMPLEX_SHADE_EPJSON)?;
    let default = compare_window_material_complex_shade(
        &default_raw,
        &default_model,
        &blank_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(default.passed, "{:?}", default.first_divergence);
    assert_eq!(
        default.report_requests,
        ConstructionReportRequests::default()
    );
    assert_eq!(default.material_details_shape.exact_header_rows, 0);
    assert_eq!(default.material_details_shape.data_rows, 0);
    Ok(())
}

#[test]
fn target_rows_require_exact_count_source_order_and_unique_names()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) =
        compile_models(&with_single_report(COMPLEX_SHADE_EPJSON, "Materials"))?;
    let baseline = compare_window_material_complex_shade(
        &raw_model,
        &model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(baseline.passed, "{:?}", baseline.first_divergence);
    let indices = indices_by_material_name(&baseline.oracle_material_details);
    assert!(
        SOURCE_IDF_MATERIAL_ORDER
            .iter()
            .all(|name| indices.get(*name).map(Vec::len) == Some(1))
    );

    let missing_eio = materials_eio().replace(&format!("{W_GENERIC}\n"), "");
    let missing = compare_window_material_complex_shade(
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
            .is_some_and(|value| value.contains("rows expected 7"))
    );

    let duplicate_eio = format!("{}{Z_GENERIC}\n", materials_eio());
    let duplicate = compare_window_material_complex_shade(
        &raw_model,
        &model,
        &duplicate_eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(!duplicate.passed);
    assert_eq!(duplicate.material_details_shape.data_rows, 8);

    let out_of_order_eio = materials_eio().replace(
        &format!("{Z_GENERIC}\n{Y_GENERIC}"),
        &format!("{Y_GENERIC}\n{Z_GENERIC}"),
    );
    let out_of_order = compare_window_material_complex_shade(
        &raw_model,
        &model,
        &out_of_order_eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(!out_of_order.passed);
    assert!(
        out_of_order
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("source fixture order"))
    );

    let extra_eio = format!(
        "{}Material Details,UNEXPECTED,0.0000,Rough,0.001,1.0,0,0,0.84,0,0\n",
        materials_eio()
    );
    let extra = compare_window_material_complex_shade(
        &raw_model,
        &model,
        &extra_eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(!extra.passed);
    assert_eq!(extra.oracle_material_details.len(), 8);

    let renamed_epjson = COMPLEX_SHADE_EPJSON.replace("T Woven Defaults", "T Renamed");
    let (renamed_raw, renamed_model) =
        compile_models(&with_single_report(&renamed_epjson, "Materials"))?;
    let renamed = compare_window_material_complex_shade(
        &renamed_raw,
        &renamed_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(!renamed.passed);
    assert!(
        renamed
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("typed complex-shade definitions"))
    );
    Ok(())
}

#[test]
fn every_generic_field_diverges_and_near_uses_source_rounded_expected()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = compile_models(COMPLEX_SHADE_EPJSON)?;
    let definition = window_material_complex_shade_definitions(&model)
        .into_iter()
        .find(|definition| definition.material_name.starts_with('Y'))
        .ok_or("missing custom BSDF shade")?;
    let base = parsed_row(Y_GENERIC);

    let mut mutations = Vec::<(&str, EioMaterialDetails)>::new();
    let mut row = base.clone();
    row.material_name = "ANOTHER SHADE".to_string();
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
    normalized_name.material_name = "  y bsdf custom base  ".to_string();
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
fn malformed_duplicate_and_row_before_header_evidence_fails_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) =
        compile_models(&with_single_report(COMPLEX_SHADE_EPJSON, "Materials"))?;

    let malformed_header = MATERIAL_DETAILS_HEADER.replacen(",Material Name", ", Material Name", 1);
    let malformed_eio = std::iter::once(malformed_header.as_str())
        .chain(SOURCE_ROWS)
        .chain(std::iter::once(""))
        .collect::<Vec<_>>()
        .join("\n");
    let malformed = compare_window_material_complex_shade(
        &raw_model,
        &model,
        &malformed_eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(!malformed.passed);
    assert_eq!(malformed.material_details_shape.candidate_header_rows, 1);
    assert_eq!(malformed.material_details_shape.exact_header_rows, 0);

    let duplicate_header_eio = std::iter::once(MATERIAL_DETAILS_HEADER)
        .chain(std::iter::once(MATERIAL_DETAILS_HEADER))
        .chain(SOURCE_ROWS)
        .chain(std::iter::once(""))
        .collect::<Vec<_>>()
        .join("\n");
    let duplicate_header = compare_window_material_complex_shade(
        &raw_model,
        &model,
        &duplicate_header_eio,
        NumericToleranceMode::Exact,
    )?;
    assert!(!duplicate_header.passed);
    assert_eq!(duplicate_header.material_details_shape.exact_header_rows, 2);

    let row_first_eio = std::iter::once(Z_GENERIC)
        .chain(std::iter::once(MATERIAL_DETAILS_HEADER))
        .chain(SOURCE_ROWS.into_iter().skip(1))
        .chain(std::iter::once(""))
        .collect::<Vec<_>>()
        .join("\n");
    let row_first = compare_window_material_complex_shade(
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
        "{MATERIAL_DETAILS_HEADER}\nMaterial Details,Z FULL DEFAULTS,0.0,Rough,0.002,1,0,0,0.84,0\n"
    );
    let error = compare_window_material_complex_shade(
        &raw_model,
        &model,
        &malformed_row,
        NumericToleranceMode::Exact,
    )
    .expect_err("a malformed Material Details payload must fail parsing");
    assert!(error.contains("invalid EIO Material Details"));

    let (default_raw, default_model) = compile_models(COMPLEX_SHADE_EPJSON)?;
    let unexpected = compare_window_material_complex_shade(
        &default_raw,
        &default_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(!unexpected.passed);
    assert_eq!(material_details_table_shape(&materials_eio()).data_rows, 7);
    Ok(())
}

#[test]
fn dedicated_complex_shade_glazing_and_window_construction_tables_are_forbidden()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) =
        compile_models(&with_single_report(COMPLEX_SHADE_EPJSON, "Materials"))?;
    let forbidden_lines = [
        "! <WindowMaterial:ComplexShade>, Material Name, Layer Type",
        "WindowMaterial:ComplexShade,Z FULL DEFAULTS,OtherShadingType",
        "! <WindowMaterial:Glazing>, Material Name, Optical Data Type",
        "WindowMaterial:Glazing,Z FULL DEFAULTS,SpectralAverage",
        "! <WindowConstruction>,Construction Name,Index,#Layers",
        "WindowConstruction,FORBIDDEN COMPLEX WINDOW,1,1",
    ];

    for forbidden_line in forbidden_lines {
        let eio = format!("{}{forbidden_line}\n", materials_eio());
        let comparison = compare_window_material_complex_shade(
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
fn selectors_are_normalized_but_blank_unknown_and_wrong_types_are_bounded()
-> Result<(), Box<dyn std::error::Error>> {
    let normalized_selectors = with_report_fields(
        COMPLEX_SHADE_EPJSON,
        "\n            \"details_type_1\": \" materials \",\n            \"details_type_2\": \"cOnStRuCtIoNs\"\n        ",
    );
    let normalized_raw = parse_epjson_str(&normalized_selectors)?;
    let requests = construction_report_requests(&normalized_raw)?;
    assert!(requests.materials);
    assert!(requests.constructions);

    let blank_selector = with_report_fields(
        COMPLEX_SHADE_EPJSON,
        "\n            \"details_type_1\": \"   \",\n            \"details_type_2\": \"Materials\"\n        ",
    );
    let blank_raw = parse_epjson_str(&blank_selector)?;
    let requests = construction_report_requests(&blank_raw)?;
    assert!(requests.materials);
    assert!(!requests.constructions);

    let invalid_selector = with_report_fields(
        COMPLEX_SHADE_EPJSON,
        "\n            \"details_type_1\": 42\n        ",
    );
    let invalid_raw = parse_epjson_str(&invalid_selector)?;
    let error = construction_report_requests(&invalid_raw)
        .expect_err("non-string Output:Constructions selector must fail");
    assert!(error.contains("must be a string"));

    let unsupported_selector = with_report_fields(
        COMPLEX_SHADE_EPJSON,
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
fn options_cli_dispatch_files_and_missing_complex_shade_are_bounded()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(parse_tolerance_mode(&[])?, NumericToleranceMode::Exact);
    assert_eq!(
        NumericToleranceMode::Exact.policy_label(),
        "energyplus-26.1-window-material-complex-shade-material-details-source-format-normalized-exact"
    );
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

    assert_eq!(run_compare_window_material_complex_shade(&[]), 2);
    assert_eq!(
        run_compare_window_material_complex_shade(&["only-input.epJSON".to_string()]),
        2
    );
    assert_eq!(
        run_compare_window_material_complex_shade(&[
            "unused.epJSON".to_string(),
            "unused.eio".to_string(),
            "--tolerance".to_string(),
            "loose".to_string(),
        ]),
        2
    );

    let temp = unique_temp_directory();
    std::fs::create_dir_all(&temp)?;
    let epjson_path = temp.join("window-material-complex-shade.epJSON");
    let no_shade_path = temp.join("no-complex-shade.epJSON");
    let eio_path = temp.join("eplusout.eio");
    std::fs::write(&epjson_path, with_both_reports(COMPLEX_SHADE_EPJSON))?;
    std::fs::write(
        &no_shade_path,
        r#"{
            "Material:NoMass": {
                "Opaque": {"roughness":"Rough", "thermal_resistance":1.0}
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
    assert_eq!(run_compare_window_material_complex_shade(&args), 0);
    let mut dispatch_args = vec!["window-material-complex-shade".to_string()];
    dispatch_args.extend(args.clone());
    assert_eq!(crate::run_compare_command(&dispatch_args), 0);
    let no_shade_args = vec![
        no_shade_path.display().to_string(),
        eio_path.display().to_string(),
    ];
    assert_eq!(run_compare_window_material_complex_shade(&no_shade_args), 1);
    std::fs::remove_dir_all(&temp)?;
    Ok(())
}

fn unique_temp_directory() -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "rusted-energyplus-window-material-complex-shade-{}-{nonce}",
        std::process::id()
    ))
}
