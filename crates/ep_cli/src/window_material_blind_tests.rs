use std::path::PathBuf;

use ep_compare::{EioMaterialDetails, EioWindowMaterialBlind, WINDOW_MATERIAL_BLIND_HEADER};
use ep_model::TypedModel;
use ep_raw_model::{RawModel, parse_epjson_str};

use super::{
    MATERIAL_DETAILS_HEADER, NumericToleranceMode, compare_window_material_blind,
    construction_report_requests, energyplus_round_sig_digits_nonnegative, generic_row_matches,
    material_details_table_shape, parse_tolerance_mode, run_compare_window_material_blind,
    specialized_row_matches, window_blind_definitions, window_blind_occurrences,
    window_material_blind_header_count,
};

const BLIND_EPJSON: &str = r#"{
    "WindowMaterial:Glazing": {
        "DISTINCTIVE BLIND TEST GLASS": {
            "optical_data_type": "SpectralAverage",
            "thickness": 0.006
        }
    },
    "WindowMaterial:Blind": {
        "A DEFAULTED USED BLIND": {
            "slat_width": 0.02,
            "slat_separation": 0.02,
            "front_side_slat_beam_solar_reflectance": 0.2,
            "back_side_slat_beam_solar_reflectance": 0.3,
            "front_side_slat_diffuse_solar_reflectance": 0.2,
            "back_side_slat_diffuse_solar_reflectance": 0.3,
            "slat_beam_visible_transmittance": 0.0
        },
        "M UNUSED BLIND": {
            "slat_width": 0.03,
            "slat_separation": 0.02,
            "front_side_slat_beam_solar_reflectance": 0.2,
            "back_side_slat_beam_solar_reflectance": 0.3,
            "front_side_slat_diffuse_solar_reflectance": 0.2,
            "back_side_slat_diffuse_solar_reflectance": 0.3,
            "slat_beam_visible_transmittance": 0.0
        },
        "Z HIGH PRECISION REUSED BLIND": {
            "slat_orientation": "Vertical",
            "slat_width": 0.0234567,
            "slat_separation": 0.0223456,
            "slat_thickness": 0.00034567,
            "slat_angle": 67.8912,
            "slat_conductivity": 15.2345,
            "slat_beam_solar_transmittance": 0.123456,
            "front_side_slat_beam_solar_reflectance": 0.234567,
            "back_side_slat_beam_solar_reflectance": 0.345678,
            "slat_diffuse_solar_transmittance": 0.123456,
            "front_side_slat_diffuse_solar_reflectance": 0.234567,
            "back_side_slat_diffuse_solar_reflectance": 0.345678,
            "slat_beam_visible_transmittance": 0.134567,
            "front_side_slat_beam_visible_reflectance": 0.245678,
            "back_side_slat_beam_visible_reflectance": 0.356789,
            "slat_diffuse_visible_transmittance": 0.134567,
            "front_side_slat_diffuse_visible_reflectance": 0.245678,
            "back_side_slat_diffuse_visible_reflectance": 0.356789,
            "slat_infrared_hemispherical_transmittance": 0.1,
            "front_side_slat_infrared_hemispherical_emissivity": 0.7,
            "back_side_slat_infrared_hemispherical_emissivity": 0.6,
            "blind_to_glass_distance": 0.0345678,
            "blind_top_opening_multiplier": 0.1,
            "blind_bottom_opening_multiplier": 0.2,
            "blind_left_side_opening_multiplier": 0.3,
            "blind_right_side_opening_multiplier": 0.4,
            "minimum_slat_angle": 0.0,
            "maximum_slat_angle": 180.0
        }
    },
    "Construction": {
        "A BARE BLIND TEST WINDOW CONSTRUCTION": {
            "outside_layer": "DISTINCTIVE BLIND TEST GLASS"
        },
        "B DEFAULTED EXTERIOR BLIND CONSTRUCTION": {
            "outside_layer": "A DEFAULTED USED BLIND",
            "layer_2": "DISTINCTIVE BLIND TEST GLASS"
        },
        "C HIGH PRECISION EXTERIOR BLIND CONSTRUCTION": {
            "outside_layer": "Z HIGH PRECISION REUSED BLIND",
            "layer_2": "DISTINCTIVE BLIND TEST GLASS"
        },
        "D HIGH PRECISION INTERIOR BLIND CONSTRUCTION": {
            "outside_layer": "DISTINCTIVE BLIND TEST GLASS",
            "layer_2": "Z HIGH PRECISION REUSED BLIND"
        }
    },
    "Output:Constructions": {
        "Output Constructions 1": {
            "details_type_1": "Constructions",
            "details_type_2": "Materials"
        }
    }
}"#;

const A_GENERIC: &str = "Material Details,A DEFAULTED USED BLIND,0.0000,Rough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const M_GENERIC: &str =
    "Material Details,M UNUSED BLIND,0.0000,Rough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const Z_GENERIC: &str = "Material Details,Z HIGH PRECISION REUSED BLIND,0.0000,Rough,0.0000,0.000,0.000,0.000,0.0000,0.0000,0.0000";
const A_SPECIALIZED: &str = "WindowMaterial:Blind,A DEFAULTED USED BLIND,2.0000E-002,2.0000E-002,2.5000E-004,45.0,0.0,0.200,5.000E-002";
const Z_SPECIALIZED: &str = "WindowMaterial:Blind,Z HIGH PRECISION REUSED BLIND,2.3457E-002,2.2346E-002,3.4567E-004,67.891,0.123,0.235,3.457E-002";

const HEADER_ONLY_EPJSON: &str = r#"{
    "WindowMaterial:Glazing": {
        "HEADER ONLY BLIND GLASS": {
            "optical_data_type": "SpectralAverage",
            "thickness": 0.006
        }
    },
    "WindowMaterial:Blind": {
        "UNUSED HEADER ONLY BLIND": {
            "slat_width": 0.02,
            "slat_separation": 0.02,
            "front_side_slat_beam_solar_reflectance": 0.2,
            "back_side_slat_beam_solar_reflectance": 0.3,
            "front_side_slat_diffuse_solar_reflectance": 0.2,
            "back_side_slat_diffuse_solar_reflectance": 0.3,
            "slat_beam_visible_transmittance": 0.0
        }
    },
    "Construction": {
        "HEADER ONLY BARE BLIND WINDOW": {
            "outside_layer": "HEADER ONLY BLIND GLASS"
        }
    },
    "Output:Constructions": {
        "Output Constructions 1": {
            "details_type_1": "Constructions"
        }
    }
}"#;

fn compile_models(epjson: &str) -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(epjson)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "blind model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result
        .model
        .ok_or("blind compiler returned no typed model")?;
    Ok((raw_model, model))
}

fn test_models() -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    compile_models(BLIND_EPJSON)
}

fn materials_only_epjson() -> String {
    BLIND_EPJSON.replace(
        "\"details_type_1\": \"Constructions\",\n            \"details_type_2\": \"Materials\"",
        "\"details_type_1\": \"Materials\"",
    )
}

fn constructions_only_epjson() -> String {
    BLIND_EPJSON.replace(
        "\"details_type_1\": \"Constructions\",\n            \"details_type_2\": \"Materials\"",
        "\"details_type_1\": \"Constructions\"",
    )
}

fn exact_eio() -> String {
    [
        MATERIAL_DETAILS_HEADER,
        Z_GENERIC,
        M_GENERIC,
        A_GENERIC,
        WINDOW_MATERIAL_BLIND_HEADER,
        A_SPECIALIZED,
        Z_SPECIALIZED,
        Z_SPECIALIZED,
        "",
    ]
    .join("\n")
}

fn materials_only_eio() -> String {
    [MATERIAL_DETAILS_HEADER, Z_GENERIC, M_GENERIC, A_GENERIC, ""].join("\n")
}

fn constructions_only_eio() -> String {
    [
        WINDOW_MATERIAL_BLIND_HEADER,
        A_SPECIALIZED,
        Z_SPECIALIZED,
        Z_SPECIALIZED,
        "",
    ]
    .join("\n")
}

#[test]
fn definitions_and_occurrences_preserve_a_z_z_construction_order()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = test_models()?;
    let definitions = window_blind_definitions(&model);
    let occurrences = window_blind_occurrences(&model)?;

    assert_eq!(definitions.len(), 3);
    assert_eq!(
        occurrences
            .iter()
            .map(|row| (
                row.construction_name.as_str(),
                row.layer_number,
                row.material_name.as_str()
            ))
            .collect::<Vec<_>>(),
        vec![
            (
                "B DEFAULTED EXTERIOR BLIND CONSTRUCTION",
                1,
                "A DEFAULTED USED BLIND"
            ),
            (
                "C HIGH PRECISION EXTERIOR BLIND CONSTRUCTION",
                1,
                "Z HIGH PRECISION REUSED BLIND"
            ),
            (
                "D HIGH PRECISION INTERIOR BLIND CONSTRUCTION",
                2,
                "Z HIGH PRECISION REUSED BLIND"
            ),
        ]
    );
    assert!(
        occurrences
            .iter()
            .all(|row| row.material_name != "M UNUSED BLIND")
    );
    Ok(())
}

#[test]
fn primary_materials_only_and_constructions_only_lanes_are_exact()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let primary = compare_window_material_blind(
        &raw_model,
        &model,
        &exact_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(primary.passed, "{:?}", primary.first_divergence);
    assert_eq!(primary.definitions.len(), 3);
    assert_eq!(primary.activated_material_names.len(), 2);
    assert_eq!(primary.occurrences.len(), 3);
    assert_eq!(primary.oracle_occurrences.len(), 3);
    assert_eq!(primary.material_details_header_rows, 1);
    assert_eq!(primary.header_rows, 1);

    let (raw_model, model) = compile_models(&materials_only_epjson())?;
    let requests = construction_report_requests(&raw_model)?;
    assert!(requests.materials);
    assert!(!requests.constructions);
    let materials = compare_window_material_blind(
        &raw_model,
        &model,
        &materials_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(materials.passed, "{:?}", materials.first_divergence);
    assert!(materials.occurrences.is_empty());
    assert_eq!(materials.material_details_header_rows, 1);
    assert_eq!(materials.header_rows, 0);

    let (raw_model, model) = compile_models(&constructions_only_epjson())?;
    let requests = construction_report_requests(&raw_model)?;
    assert!(!requests.materials);
    assert!(requests.constructions);
    let constructions = compare_window_material_blind(
        &raw_model,
        &model,
        &constructions_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(constructions.passed, "{:?}", constructions.first_divergence);
    assert!(constructions.oracle_material_details.is_empty());
    assert_eq!(constructions.material_details_header_rows, 0);
    assert_eq!(constructions.occurrences.len(), 3);
    Ok(())
}

#[test]
fn generic_rows_require_rough_zero_fields_and_exact_cardinality()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = test_models()?;
    let definition = window_blind_definitions(&model)
        .into_iter()
        .find(|row| row.material_name == "A DEFAULTED USED BLIND")
        .ok_or("missing A blind")?;
    let base = EioMaterialDetails {
        material_name: " a defaulted used blind ".to_string(),
        thermal_resistance_m2_k_per_w: 0.0,
        roughness: "Rough".to_string(),
        thickness_m: 0.0,
        conductivity_w_per_m_k: 0.0,
        density_kg_per_m3: 0.0,
        specific_heat_j_per_kg_k: 0.0,
        thermal_absorptance: 0.0,
        solar_absorptance: 0.0,
        visible_absorptance: 0.0,
    };
    assert!(generic_row_matches(&definition, &base));
    for mismatch in 0..10 {
        let mut row = base.clone();
        match mismatch {
            0 => row.material_name = "OTHER BLIND".to_string(),
            1 => row.roughness = "MediumRough".to_string(),
            2 => row.thermal_resistance_m2_k_per_w = 0.0000001,
            3 => row.thickness_m = 0.0000001,
            4 => row.conductivity_w_per_m_k = 0.0000001,
            5 => row.density_kg_per_m3 = 0.0000001,
            6 => row.specific_heat_j_per_kg_k = 0.0000001,
            7 => row.thermal_absorptance = 0.0000001,
            8 => row.solar_absorptance = 0.0000001,
            9 => row.visible_absorptance = 0.0000001,
            _ => unreachable!(),
        }
        assert!(
            !generic_row_matches(&definition, &row),
            "mismatch {mismatch}"
        );
    }

    let (raw_model, model) = test_models()?;
    let missing = exact_eio()
        .lines()
        .filter(|line| *line != A_GENERIC)
        .collect::<Vec<_>>()
        .join("\n");
    let result =
        compare_window_material_blind(&raw_model, &model, &missing, NumericToleranceMode::Exact)?;
    assert!(!result.passed);
    assert!(
        result
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("observed 0"))
    );

    let duplicate = format!("{}{}\n", exact_eio(), A_GENERIC);
    let result =
        compare_window_material_blind(&raw_model, &model, &duplicate, NumericToleranceMode::Exact)?;
    assert!(!result.passed);
    assert!(
        result
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("observed 2"))
    );

    let (raw_model, model) = compile_models(&constructions_only_epjson())?;
    let unexpected = format!(
        "{}\n{}\n{}",
        MATERIAL_DETAILS_HEADER,
        A_GENERIC,
        constructions_only_eio()
    );
    let result = compare_window_material_blind(
        &raw_model,
        &model,
        &unexpected,
        NumericToleranceMode::Exact,
    )?;
    assert!(
        !result.passed,
        "a generic Blind row is forbidden in Constructions-only"
    );
    Ok(())
}

#[test]
fn material_details_header_and_selector_shape_fail_closed() -> Result<(), Box<dyn std::error::Error>>
{
    let (raw_model, model) = test_models()?;

    let missing_header = exact_eio().replacen(&format!("{MATERIAL_DETAILS_HEADER}\n"), "", 1);
    let result = compare_window_material_blind(
        &raw_model,
        &model,
        &missing_header,
        NumericToleranceMode::Exact,
    )?;
    assert!(!result.passed);
    assert!(
        result
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("Material Details header expected 1 observed 0"))
    );

    let malformed_header = exact_eio().replacen(
        MATERIAL_DETAILS_HEADER,
        &format!(" {MATERIAL_DETAILS_HEADER}"),
        1,
    );
    let result = compare_window_material_blind(
        &raw_model,
        &model,
        &malformed_header,
        NumericToleranceMode::Exact,
    )?;
    assert!(!result.passed);
    let shape = material_details_table_shape(&malformed_header);
    assert_eq!(shape.candidate_header_rows, 1);
    assert_eq!(shape.exact_header_rows, 0);

    let duplicate_header = exact_eio().replacen(
        MATERIAL_DETAILS_HEADER,
        &format!("{MATERIAL_DETAILS_HEADER}\n{MATERIAL_DETAILS_HEADER}"),
        1,
    );
    let result = compare_window_material_blind(
        &raw_model,
        &model,
        &duplicate_header,
        NumericToleranceMode::Exact,
    )?;
    assert!(!result.passed);
    assert!(
        result
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("expected 1 observed 2"))
    );

    let row_before_header = exact_eio().replacen(
        &format!("{MATERIAL_DETAILS_HEADER}\n{Z_GENERIC}"),
        &format!("{Z_GENERIC}\n{MATERIAL_DETAILS_HEADER}"),
        1,
    );
    let result = compare_window_material_blind(
        &raw_model,
        &model,
        &row_before_header,
        NumericToleranceMode::Exact,
    )?;
    assert!(!result.passed);
    assert!(
        result
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("without a preceding exact"))
    );

    let (raw_model, model) = compile_models(&constructions_only_epjson())?;
    let header_only = format!("{MATERIAL_DETAILS_HEADER}\n{}", constructions_only_eio());
    let result = compare_window_material_blind(
        &raw_model,
        &model,
        &header_only,
        NumericToleranceMode::Exact,
    )?;
    assert!(
        !result.passed,
        "a generic header is forbidden without Materials"
    );

    let stray_other_material = format!(
        "Material Details,UNRELATED MATERIAL,0,Rough,0,0,0,0,0,0,0\n{}",
        constructions_only_eio()
    );
    let result = compare_window_material_blind(
        &raw_model,
        &model,
        &stray_other_material,
        NumericToleranceMode::Exact,
    )?;
    assert!(
        !result.passed,
        "all generic rows are forbidden without Materials reporting"
    );
    Ok(())
}

#[test]
fn specialized_rows_gate_every_field_order_name_count_and_rounding()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let occurrence = window_blind_occurrences(&model)?
        .into_iter()
        .next()
        .ok_or("missing A occurrence")?;
    let base = EioWindowMaterialBlind {
        material_name: "a defaulted used blind".to_string(),
        slat_width_m: 0.02,
        slat_separation_m: 0.02,
        slat_thickness_m: 0.00025,
        slat_angle_deg: 45.0,
        slat_beam_solar_transmittance: 0.0,
        slat_beam_solar_front_reflectance: 0.2,
        blind_to_glass_distance_m: 0.05,
    };
    assert!(specialized_row_matches(
        &occurrence,
        &base,
        NumericToleranceMode::Exact
    ));
    for mismatch in 0..8 {
        let mut row = base.clone();
        match mismatch {
            0 => row.material_name = "OTHER BLIND".to_string(),
            1 => row.slat_width_m += 0.0001,
            2 => row.slat_separation_m += 0.0001,
            3 => row.slat_thickness_m += 0.0001,
            4 => row.slat_angle_deg += 0.1,
            5 => row.slat_beam_solar_transmittance += 0.001,
            6 => row.slat_beam_solar_front_reflectance += 0.001,
            7 => row.blind_to_glass_distance_m += 0.001,
            _ => unreachable!(),
        }
        assert!(!specialized_row_matches(
            &occurrence,
            &row,
            NumericToleranceMode::Exact
        ));
    }

    let bad_order = [
        MATERIAL_DETAILS_HEADER,
        Z_GENERIC,
        M_GENERIC,
        A_GENERIC,
        WINDOW_MATERIAL_BLIND_HEADER,
        Z_SPECIALIZED,
        A_SPECIALIZED,
        Z_SPECIALIZED,
        "",
    ]
    .join("\n");
    let result =
        compare_window_material_blind(&raw_model, &model, &bad_order, NumericToleranceMode::Exact)?;
    assert!(!result.passed);
    assert!(
        result
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("field material_name"))
    );

    let missing = exact_eio().replacen(&format!("{Z_SPECIALIZED}\n"), "", 1);
    let result =
        compare_window_material_blind(&raw_model, &model, &missing, NumericToleranceMode::Exact)?;
    assert!(!result.passed);
    assert_eq!(result.oracle_occurrences.len(), 2);

    let unknown = exact_eio().replacen(
        "WindowMaterial:Blind,A DEFAULTED USED BLIND,",
        "WindowMaterial:Blind,UNKNOWN BLIND,",
        1,
    );
    let result =
        compare_window_material_blind(&raw_model, &model, &unknown, NumericToleranceMode::Exact)?;
    assert!(!result.passed);
    assert!(
        result
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("material_name"))
    );

    let exact_mutation =
        exact_eio().replacen("2.0000E-002,2.0000E-002", "2.0005E-002,2.0000E-002", 1);
    let exact_result = compare_window_material_blind(
        &raw_model,
        &model,
        &exact_mutation,
        NumericToleranceMode::Exact,
    )?;
    assert!(!exact_result.passed);
    let near_result = compare_window_material_blind(
        &raw_model,
        &model,
        &exact_mutation,
        NumericToleranceMode::Near,
    )?;
    assert!(near_result.passed, "{:?}", near_result.first_divergence);
    let outside_near =
        exact_eio().replacen("2.0000E-002,2.0000E-002", "2.0020E-002,2.0000E-002", 1);
    let result = compare_window_material_blind(
        &raw_model,
        &model,
        &outside_near,
        NumericToleranceMode::Near,
    )?;
    assert!(!result.passed);
    Ok(())
}

#[test]
fn exact_source_rounding_and_header_only_contract_are_bounded()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(0.0234567, 4),
        Some(0.023457)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(0.0223456, 4),
        Some(0.022346)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(0.00034567, 4),
        Some(0.00034567)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(67.8912, 3),
        Some(67.891)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(0.123456, 3),
        Some(0.123)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(0.234567, 3),
        Some(0.235)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(0.0345678, 3),
        Some(0.03457)
    );
    assert_eq!(
        window_material_blind_header_count(WINDOW_MATERIAL_BLIND_HEADER),
        1
    );

    let (raw_model, model) = compile_models(HEADER_ONLY_EPJSON)?;
    let comparison = compare_window_material_blind(
        &raw_model,
        &model,
        WINDOW_MATERIAL_BLIND_HEADER,
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert!(comparison.occurrences.is_empty());
    assert_eq!(comparison.header_rows, 1);
    Ok(())
}

#[test]
fn unsupported_between_glass_and_missing_bare_companion_fail_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let between_glass = BLIND_EPJSON
        .replace(
            "    \"WindowMaterial:Blind\": {",
            "    \"WindowMaterial:Gas\": {\n        \"BLIND TEST AIR\": {\"gas_type\": \"Air\", \"thickness\": 0.01}\n    },\n    \"WindowMaterial:Blind\": {",
        )
        .replace(
            "        \"B DEFAULTED EXTERIOR BLIND CONSTRUCTION\": {\n            \"outside_layer\": \"A DEFAULTED USED BLIND\",\n            \"layer_2\": \"DISTINCTIVE BLIND TEST GLASS\"\n        },",
            "        \"B DEFAULTED BETWEEN GLASS BLIND CONSTRUCTION\": {\n            \"outside_layer\": \"DISTINCTIVE BLIND TEST GLASS\",\n            \"layer_2\": \"BLIND TEST AIR\",\n            \"layer_3\": \"A DEFAULTED USED BLIND\",\n            \"layer_4\": \"BLIND TEST AIR\",\n            \"layer_5\": \"DISTINCTIVE BLIND TEST GLASS\"\n        },",
        );
    let (raw_model, model) = compile_models(&between_glass)?;
    let error = compare_window_material_blind(
        &raw_model,
        &model,
        WINDOW_MATERIAL_BLIND_HEADER,
        NumericToleranceMode::Exact,
    )
    .expect_err("between-glass Blind must fail closed");
    assert!(error.contains("between-glass ordinary Blind"), "{error}");

    let missing_bare = BLIND_EPJSON.replace(
        "        \"A BARE BLIND TEST WINDOW CONSTRUCTION\": {\n            \"outside_layer\": \"DISTINCTIVE BLIND TEST GLASS\"\n        },\n",
        "",
    );
    let (raw_model, model) = compile_models(&missing_bare)?;
    let error = compare_window_material_blind(
        &raw_model,
        &model,
        WINDOW_MATERIAL_BLIND_HEADER,
        NumericToleranceMode::Exact,
    )
    .expect_err("missing exact bare companion must fail closed");
    assert!(error.contains("missing an exact bare companion"), "{error}");
    Ok(())
}

#[test]
fn malformed_blind_rows_propagate_and_unrelated_material_rows_are_ignored()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let malformed = exact_eio().replacen(
        A_SPECIALIZED,
        "WindowMaterial:Blind,A DEFAULTED USED BLIND,0.02",
        1,
    );
    let error =
        compare_window_material_blind(&raw_model, &model, &malformed, NumericToleranceMode::Exact)
            .expect_err("malformed specialized Blind row must propagate");
    assert!(
        error.contains("invalid EIO WindowMaterial:Blind"),
        "{error}"
    );

    let unrelated = format!(
        "{}WindowMaterial:Shade,UNRELATED SHADE,0.001,0.2,0.8,0.1,0.2,0.3\nWindowMaterial:Screen,UNRELATED SCREEN,0.001\nWindowMaterial:Screen:EquivalentLayer,UNRELATED EQL SCREEN,0.1\n",
        exact_eio()
    );
    let comparison =
        compare_window_material_blind(&raw_model, &model, &unrelated, NumericToleranceMode::Exact)?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    Ok(())
}

#[test]
fn tolerance_options_and_cli_dispatch_are_stable() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(parse_tolerance_mode(&[]), Ok(NumericToleranceMode::Exact));
    assert_eq!(
        parse_tolerance_mode(&["--tolerance".to_string(), "near".to_string()]),
        Ok(NumericToleranceMode::Near)
    );
    assert!(parse_tolerance_mode(&["--tolerance".to_string(), "wide".to_string()]).is_err());
    assert_eq!(run_compare_window_material_blind(&[]), 2);
    assert_eq!(
        run_compare_window_material_blind(&["only-input.epJSON".to_string()]),
        2
    );
    assert_eq!(
        run_compare_window_material_blind(&[
            "input.epJSON".to_string(),
            "eplusout.eio".to_string(),
            "--unexpected".to_string(),
        ]),
        2
    );

    let directory = unique_test_directory();
    std::fs::create_dir_all(&directory)?;
    let input_path = directory.join("blind.epJSON");
    let eio_path = directory.join("eplusout.eio");
    std::fs::write(&input_path, BLIND_EPJSON)?;
    std::fs::write(&eio_path, exact_eio())?;
    let args = vec![
        "window-material-blind".to_string(),
        input_path.to_string_lossy().into_owned(),
        eio_path.to_string_lossy().into_owned(),
        "--tolerance".to_string(),
        "exact".to_string(),
    ];
    let exit_code = crate::run_compare_command(&args);
    std::fs::remove_dir_all(&directory)?;
    assert_eq!(exit_code, 0);
    Ok(())
}

fn unique_test_directory() -> PathBuf {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    std::env::temp_dir().join(format!(
        "rusted-energyplus-window-blind-cli-{}-{nonce}",
        std::process::id()
    ))
}
