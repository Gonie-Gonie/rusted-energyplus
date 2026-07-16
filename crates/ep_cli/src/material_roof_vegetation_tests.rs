use std::path::PathBuf;

use ep_compare::{EioMaterialDetails, parse_eio_material_details};
use ep_model::{MaterialSurfaceRoughness, TypedModel};
use ep_raw_model::{RawModel, parse_epjson_str};

use super::{
    MATERIAL_DETAILS_HEADER, NumericToleranceMode, canonical_roughness,
    compare_material_roof_vegetation, construction_report_requests, expected_numeric_fields,
    generic_row_matches, indices_by_material_name, material_details_table_shape,
    parse_tolerance_mode, record_generic_divergence, roof_vegetation_definitions,
    run_compare_material_roof_vegetation,
};

const ROOF_VEGETATION_EPJSON: &str = r#"{
    "Material:RoofVegetation": {
        "A Defaulted Used Vegetation": {},
        "M Explicit Unused Vegetation": {
            "height_of_plants": 0.42,
            "leaf_area_index": 3.2,
            "leaf_reflectivity": 0.27,
            "leaf_emissivity": 0.91,
            "minimum_stomatal_resistance": 210.0,
            "soil_layer_name": "Unreported M Soil",
            "roughness": "VeryRough",
            "thickness": 0.2345678,
            "conductivity_of_dry_soil": 0.456789,
            "density_of_dry_soil": 987.6543,
            "specific_heat_of_dry_soil": 1234.5678,
            "thermal_absorptance": 0.8765432,
            "solar_absorptance": 0.654321,
            "visible_absorptance": 0.765432,
            "saturation_volumetric_moisture_content_of_the_soil_layer": 0.41,
            "residual_volumetric_moisture_content_of_the_soil_layer": 0.07,
            "initial_volumetric_moisture_content_of_the_soil_layer": 0.31,
            "moisture_diffusion_calculation_method": "Simple"
        },
        "Z High Precision Unused Vegetation": {
            "height_of_plants": 0.57,
            "leaf_area_index": 4.2,
            "leaf_reflectivity": 0.31,
            "leaf_emissivity": 0.93,
            "minimum_stomatal_resistance": 245.0,
            "soil_layer_name": "Unreported Z Soil",
            "roughness": "VerySmooth",
            "thickness": 0.1234567,
            "conductivity_of_dry_soil": 0.3456789,
            "density_of_dry_soil": 876.5432,
            "specific_heat_of_dry_soil": 1456.789,
            "thermal_absorptance": 0.9234567,
            "solar_absorptance": 0.8123456,
            "visible_absorptance": 0.8345678,
            "saturation_volumetric_moisture_content_of_the_soil_layer": 0.44,
            "residual_volumetric_moisture_content_of_the_soil_layer": 0.08,
            "initial_volumetric_moisture_content_of_the_soil_layer": 0.34,
            "moisture_diffusion_calculation_method": "Advanced"
        }
    }
}"#;

const A_GENERIC: &str = "Material Details,A DEFAULTED USED VEGETATION,0.2857,MediumRough,0.1000,0.350,1100.000,1200.000,0.9000,0.7000,0.7500";
const M_GENERIC: &str = "Material Details,M EXPLICIT UNUSED VEGETATION,0.5135,VeryRough,0.2346,0.457,987.654,1234.568,0.8765,0.6543,0.7654";
const Z_GENERIC: &str = "Material Details,Z HIGH PRECISION UNUSED VEGETATION,0.3571,VerySmooth,0.1235,0.346,876.543,1456.789,0.9235,0.8123,0.8346";
const UNRELATED_GENERIC: &str = "Material Details,UNRELATED HOST,0.2000,Rough,0.1000,0.500,800.000,900.000,0.9000,0.7000,0.7000";

fn compile_models(epjson: &str) -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(epjson)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "roof-vegetation comparison model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result
        .model
        .ok_or("roof-vegetation compiler returned no typed model")?;
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

fn constructions_only_eio() -> String {
    "Program Version,EnergyPlus, Version 26.1.0\nConstruction CTF,IGNORED\n".to_string()
}

fn parsed_row(row: &str) -> EioMaterialDetails {
    parse_eio_material_details(row)
        .expect("test Material Details row should parse")
        .remove(0)
}

#[test]
fn expected_rows_apply_roof_source_rounding_and_canonical_roughness()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = compile_models(ROOF_VEGETATION_EPJSON)?;
    let definitions = roof_vegetation_definitions(&model);
    assert_eq!(
        definitions
            .iter()
            .map(|definition| definition.material_name.as_str())
            .collect::<Vec<_>>(),
        vec![
            "A DEFAULTED USED VEGETATION",
            "M EXPLICIT UNUSED VEGETATION",
            "Z HIGH PRECISION UNUSED VEGETATION"
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
        assert_eq!(
            row.roughness,
            canonical_roughness(definition.fields.roughness)
        );
    }

    let m_expected = expected_numeric_fields(&definitions[1], &parsed_row(M_GENERIC))
        .ok_or("expected source-formatted M fields")?;
    assert_eq!(
        m_expected.map(|(_field, expected, _observed)| expected),
        [
            0.5135, 0.2346, 0.457, 987.654, 1234.568, 0.8765, 0.6543, 0.7654
        ]
    );
    assert_eq!(
        canonical_roughness(MaterialSurfaceRoughness::MediumSmooth),
        "MediumSmooth"
    );
    Ok(())
}

#[test]
fn both_materials_and_constructions_only_lanes_match_selector_activation()
-> Result<(), Box<dyn std::error::Error>> {
    let (both_raw, both_model) = compile_models(&with_both_reports(ROOF_VEGETATION_EPJSON))?;
    let both = compare_material_roof_vegetation(
        &both_raw,
        &both_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(both.passed, "{:?}", both.first_divergence);
    assert_eq!(both.oracle_material_details.len(), 4);
    assert_eq!(both.material_details_shape.exact_header_rows, 1);
    assert_eq!(
        both.report_requests,
        super::ConstructionReportRequests {
            constructions: true,
            materials: true,
        }
    );

    let (materials_raw, materials_model) =
        compile_models(&with_single_report(ROOF_VEGETATION_EPJSON, "Materials"))?;
    let materials = compare_material_roof_vegetation(
        &materials_raw,
        &materials_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(materials.passed, "{:?}", materials.first_divergence);
    assert!(materials.report_requests.materials);
    assert!(!materials.report_requests.constructions);

    let (constructions_raw, constructions_model) =
        compile_models(&with_single_report(ROOF_VEGETATION_EPJSON, "Constructions"))?;
    let constructions = compare_material_roof_vegetation(
        &constructions_raw,
        &constructions_model,
        &constructions_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(constructions.passed, "{:?}", constructions.first_divergence);
    assert!(constructions.oracle_material_details.is_empty());
    assert_eq!(constructions.material_details_shape.exact_header_rows, 0);
    assert!(!constructions.report_requests.materials);
    assert!(constructions.report_requests.constructions);

    let normalized_selectors = with_report_fields(
        ROOF_VEGETATION_EPJSON,
        "\n            \"details_type_1\": \" materials \",\n            \"details_type_2\": \"cOnStRuCtIoNs\"\n        ",
    );
    let normalized_raw = parse_epjson_str(&normalized_selectors)?;
    let requests = construction_report_requests(&normalized_raw)?;
    assert!(requests.materials);
    assert!(requests.constructions);
    Ok(())
}

#[test]
fn matching_requires_each_definition_once_but_allows_unrelated_material_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) =
        compile_models(&with_single_report(ROOF_VEGETATION_EPJSON, "Materials"))?;
    let baseline = compare_material_roof_vegetation(
        &raw_model,
        &model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(baseline.passed, "{:?}", baseline.first_divergence);
    let indices = indices_by_material_name(&baseline.oracle_material_details);
    assert_eq!(indices.get("UNRELATED HOST").map(Vec::len), Some(1));

    let missing = materials_eio().replace(&format!("{M_GENERIC}\n"), "");
    let missing = compare_material_roof_vegetation(
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
            .is_some_and(|value| value.contains("M EXPLICIT UNUSED VEGETATION"))
    );

    let duplicate = format!("{}{Z_GENERIC}\n", materials_eio());
    let duplicate = compare_material_roof_vegetation(
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
fn every_reported_field_is_a_divergence_and_near_uses_source_rounded_expected()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = compile_models(ROOF_VEGETATION_EPJSON)?;
    let definition = roof_vegetation_definitions(&model)
        .into_iter()
        .find(|definition| definition.material_name.starts_with('Z'))
        .ok_or("missing high-precision definition")?;
    let base = parsed_row(Z_GENERIC);

    let mut mutations = Vec::<(&str, EioMaterialDetails)>::new();
    let mut row = base.clone();
    row.material_name = "ANOTHER VEGETATION".to_string();
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
    row.density_kg_per_m3 += 0.01;
    mutations.push(("density_kg_per_m3", row));
    let mut row = base.clone();
    row.specific_heat_j_per_kg_k += 0.01;
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
    normalized_name.material_name = "  z high precision unused vegetation  ".to_string();
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
fn exact_header_shape_rejects_malformed_duplicate_and_row_before_header()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) =
        compile_models(&with_single_report(ROOF_VEGETATION_EPJSON, "Materials"))?;

    let malformed_header = MATERIAL_DETAILS_HEADER.replacen(",Material Name", ", Material Name", 1);
    let malformed = [
        malformed_header.as_str(),
        Z_GENERIC,
        M_GENERIC,
        A_GENERIC,
        "",
    ]
    .join("\n");
    let malformed = compare_material_roof_vegetation(
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
    let duplicate_header = compare_material_roof_vegetation(
        &raw_model,
        &model,
        &duplicate_header,
        NumericToleranceMode::Exact,
    )?;
    assert!(!duplicate_header.passed);
    assert_eq!(duplicate_header.material_details_shape.exact_header_rows, 2);

    let row_first = [Z_GENERIC, MATERIAL_DETAILS_HEADER, M_GENERIC, A_GENERIC, ""].join("\n");
    let row_first = compare_material_roof_vegetation(
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

    let shape = material_details_table_shape(&materials_eio());
    assert_eq!(shape.candidate_header_rows, 1);
    assert_eq!(shape.exact_header_rows, 1);

    let (constructions_raw, constructions_model) =
        compile_models(&with_single_report(ROOF_VEGETATION_EPJSON, "Constructions"))?;
    let unexpected_rows = compare_material_roof_vegetation(
        &constructions_raw,
        &constructions_model,
        &materials_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(!unexpected_rows.passed);
    Ok(())
}

#[test]
fn unreported_plant_and_moisture_fields_do_not_change_material_details_expectation()
-> Result<(), Box<dyn std::error::Error>> {
    let base_epjson = r#"{
        "Material:RoofVegetation": {"Same Roof": {}}
    }"#;
    let changed_epjson = r#"{
        "Material:RoofVegetation": {
            "Same Roof": {
                "height_of_plants": 0.61,
                "leaf_area_index": 4.1,
                "leaf_reflectivity": 0.33,
                "leaf_emissivity": 0.88,
                "minimum_stomatal_resistance": 250.0,
                "soil_layer_name": "Different Unreported Soil",
                "saturation_volumetric_moisture_content_of_the_soil_layer": 0.42,
                "residual_volumetric_moisture_content_of_the_soil_layer": 0.08,
                "initial_volumetric_moisture_content_of_the_soil_layer": 0.32,
                "moisture_diffusion_calculation_method": "Simple"
            }
        }
    }"#;
    let (_base_raw, base_model) = compile_models(base_epjson)?;
    let (_changed_raw, changed_model) = compile_models(changed_epjson)?;
    let base = roof_vegetation_definitions(&base_model).remove(0);
    let changed = roof_vegetation_definitions(&changed_model).remove(0);
    assert_ne!(
        base.fields.height_of_plants_m,
        changed.fields.height_of_plants_m
    );
    assert_ne!(
        base.fields.initial_volumetric_moisture_content,
        changed.fields.initial_volumetric_moisture_content
    );

    let row = parsed_row(
        "Material Details,SAME ROOF,0.2857,MediumRough,0.1000,0.350,1100.000,1200.000,0.9000,0.7000,0.7500",
    );
    assert!(generic_row_matches(
        &base,
        &row,
        NumericToleranceMode::Exact
    ));
    assert!(generic_row_matches(
        &changed,
        &row,
        NumericToleranceMode::Exact
    ));
    Ok(())
}

#[test]
fn options_selector_types_and_cli_file_contracts_are_bounded()
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
        ROOF_VEGETATION_EPJSON,
        "\n            \"details_type_1\": 42\n        ",
    );
    let invalid_raw = parse_epjson_str(&invalid_selector)?;
    let error = construction_report_requests(&invalid_raw)
        .expect_err("non-string Output:Constructions selector must fail");
    assert!(error.contains("must be a string"));

    assert_eq!(run_compare_material_roof_vegetation(&[]), 2);
    assert_eq!(
        run_compare_material_roof_vegetation(&["only-input.epJSON".to_string()]),
        2
    );
    assert_eq!(
        run_compare_material_roof_vegetation(&[
            "unused.epJSON".to_string(),
            "unused.eio".to_string(),
            "--tolerance".to_string(),
            "loose".to_string(),
        ]),
        2
    );

    let temp = unique_temp_directory();
    std::fs::create_dir_all(&temp)?;
    let epjson_path = temp.join("material-roof-vegetation.epJSON");
    let eio_path = temp.join("eplusout.eio");
    std::fs::write(&epjson_path, with_both_reports(ROOF_VEGETATION_EPJSON))?;
    std::fs::write(&eio_path, materials_eio())?;
    let args = vec![
        epjson_path.display().to_string(),
        eio_path.display().to_string(),
        "--tolerance".to_string(),
        "exact".to_string(),
    ];
    let exit = run_compare_material_roof_vegetation(&args);
    std::fs::remove_dir_all(&temp)?;
    assert_eq!(exit, 0);
    Ok(())
}

fn unique_temp_directory() -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "rusted-energyplus-material-roof-vegetation-{}-{nonce}",
        std::process::id()
    ))
}
