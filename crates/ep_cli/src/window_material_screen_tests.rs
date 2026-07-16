use std::path::PathBuf;

use ep_compare::{EioMaterialDetails, EioWindowMaterialScreen, WINDOW_MATERIAL_SCREEN_HEADER};
use ep_model::TypedModel;
use ep_raw_model::{ObjectName, ObjectType, RawModel, parse_epjson_str};

use super::optics::screen_static_inputs;
use super::{
    NumericToleranceMode, WINDOW_SHADING_CONTROL_OBJECT_TYPE,
    activated_window_screen_material_names, calculate_screen_source_optics,
    compare_window_material_screen, construction_report_requests,
    energyplus_round_sig_digits_nonnegative, generic_row_matches, parse_tolerance_mode,
    run_compare_window_material_screen, specialized_row_matches,
    window_material_screen_header_count, window_screen_definitions, window_screen_occurrences,
};

const SCREEN_EPJSON: &str = r#"{
    "WindowMaterial:Glazing": {
        "DISTINCTIVE SCREEN TEST GLASS": {
            "optical_data_type": "SpectralAverage",
            "thickness": 0.006
        }
    },
    "WindowMaterial:Screen": {
        "A DEFAULTED USED SCREEN": {
            "diffuse_solar_reflectance": 0.1,
            "diffuse_visible_reflectance": 0.2,
            "screen_material_spacing": 0.01,
            "screen_material_diameter": 0.002
        },
        "M UNUSED SCREEN": {
            "reflected_beam_transmittance_accounting_method": "DoNotModel",
            "diffuse_solar_reflectance": 0.15,
            "diffuse_visible_reflectance": 0.25,
            "thermal_hemispherical_emissivity": 0.75,
            "conductivity": 11.5,
            "screen_material_spacing": 0.01,
            "screen_material_diameter": 0.0025,
            "screen_to_glass_distance": 0.03,
            "top_opening_multiplier": 0,
            "bottom_opening_multiplier": 0,
            "left_side_opening_multiplier": 0,
            "right_side_opening_multiplier": 0,
            "angle_of_resolution_for_screen_transmittance_output_map": 0
        },
        "Z HIGH PRECISION REUSED SCREEN": {
            "reflected_beam_transmittance_accounting_method": "ModelAsDirectBeam",
            "diffuse_solar_reflectance": 0.123456,
            "diffuse_visible_reflectance": 0.234567,
            "thermal_hemispherical_emissivity": 0.812345,
            "conductivity": 17.2345,
            "screen_material_spacing": 0.0123456,
            "screen_material_diameter": 0.0034567,
            "screen_to_glass_distance": 0.0345678,
            "top_opening_multiplier": 0.1111,
            "bottom_opening_multiplier": 0.2222,
            "left_side_opening_multiplier": 0.3333,
            "right_side_opening_multiplier": 0.4444,
            "angle_of_resolution_for_screen_transmittance_output_map": 0
        }
    },
    "Construction": {
        "A BASE SCREEN TEST WINDOW CONSTRUCTION": {
            "outside_layer": "DISTINCTIVE SCREEN TEST GLASS"
        },
        "B DEFAULTED SCREEN WINDOW CONSTRUCTION": {
            "outside_layer": "A DEFAULTED USED SCREEN",
            "layer_2": "DISTINCTIVE SCREEN TEST GLASS"
        },
        "C HIGH PRECISION FIRST SCREEN WINDOW CONSTRUCTION": {
            "outside_layer": "Z HIGH PRECISION REUSED SCREEN",
            "layer_2": "DISTINCTIVE SCREEN TEST GLASS"
        },
        "D HIGH PRECISION SECOND SCREEN WINDOW CONSTRUCTION": {
            "outside_layer": "Z HIGH PRECISION REUSED SCREEN",
            "layer_2": "DISTINCTIVE SCREEN TEST GLASS"
        }
    },
    "FenestrationSurface:Detailed": {
        "DISTINCTIVE DEFAULTED SCREEN TEST WINDOW": {
            "construction_name": "A BASE SCREEN TEST WINDOW CONSTRUCTION"
        },
        "DISTINCTIVE FIRST HIGH PRECISION SCREEN TEST WINDOW": {
            "construction_name": "A BASE SCREEN TEST WINDOW CONSTRUCTION"
        },
        "DISTINCTIVE SECOND HIGH PRECISION SCREEN TEST WINDOW": {
            "construction_name": "A BASE SCREEN TEST WINDOW CONSTRUCTION"
        }
    },
    "WindowShadingControl": {
        "DEFAULTED SCREEN CONTROL": {
            "shading_type": "ExteriorScreen",
            "construction_with_shading_name": "B DEFAULTED SCREEN WINDOW CONSTRUCTION",
            "fenestration_surfaces": [
                {"fenestration_surface_name": "DISTINCTIVE DEFAULTED SCREEN TEST WINDOW"}
            ]
        },
        "HIGH PRECISION SCREEN CONTROL": {
            "shading_type": "ExteriorScreen",
            "construction_with_shading_name": "C HIGH PRECISION FIRST SCREEN WINDOW CONSTRUCTION",
            "fenestration_surfaces": [
                {"fenestration_surface_name": "DISTINCTIVE FIRST HIGH PRECISION SCREEN TEST WINDOW"},
                {"fenestration_surface_name": "DISTINCTIVE SECOND HIGH PRECISION SCREEN TEST WINDOW"}
            ]
        }
    },
    "Output:Constructions": {
        "Output Constructions 1": {
            "details_type_1": "Constructions",
            "details_type_2": "Materials"
        }
    }
}"#;

const HEADER_ONLY_EPJSON: &str = r#"{
    "WindowMaterial:Glazing": {
        "HEADER ONLY GLASS": {
            "optical_data_type": "SpectralAverage",
            "thickness": 0.006
        }
    },
    "WindowMaterial:Screen": {
        "UNUSED HEADER ONLY SCREEN": {
            "diffuse_solar_reflectance": 0.1,
            "diffuse_visible_reflectance": 0.2,
            "screen_material_spacing": 0.01,
            "screen_material_diameter": 0.002
        }
    },
    "Construction": {
        "HEADER ONLY BARE WINDOW CONSTRUCTION": {
            "outside_layer": "HEADER ONLY GLASS"
        }
    },
    "FenestrationSurface:Detailed": {
        "HEADER ONLY WINDOW": {
            "construction_name": "HEADER ONLY BARE WINDOW CONSTRUCTION"
        }
    },
    "WindowShadingControl": {
        "UNRELATED EXTERIOR SHADE CONTROL": {
            "shading_type": "ExteriorShade",
            "fenestration_surfaces": [
                {"fenestration_surface_name": "HEADER ONLY WINDOW"}
            ]
        },
        "UNRELATED INTERIOR SHADE CONTROL": {
            "shading_type": "InteriorShade",
            "fenestration_surfaces": [
                {"fenestration_surface_name": "HEADER ONLY WINDOW"}
            ]
        }
    },
    "Output:Constructions": {
        "Output Constructions 1": {
            "details_type_1": "Constructions"
        }
    }
}"#;

const GENERIC_HEADER: &str = "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible";
const A_GENERIC: &str = "Material Details,A DEFAULTED USED SCREEN,0.0000,MediumRough,2.0000E-003,221.000,0.000,0.000,0.3240,0.3240,0.2880";
const M_GENERIC: &str = "Material Details,M UNUSED SCREEN,0.0000,MediumRough,2.5000E-003,11.500,0.000,0.000,0.3281,0.3719,0.3281";
const Z_GENERIC: &str = "Material Details,Z HIGH PRECISION REUSED SCREEN,0.0000,MediumRough,3.4567E-003,17.235,0.000,0.000,0.3912,0.4221,0.3686";
const A_SPECIALIZED: &str = "WindowMaterial:Screen,A DEFAULTED USED SCREEN,2.00000E-003,221.000,0.324,0.640,3.561E-002,7.050E-002,4.068E-002,8.162E-002,0.200,2.500E-002";
const Z_SPECIALIZED: &str = "WindowMaterial:Screen,Z HIGH PRECISION REUSED SCREEN,3.45670E-003,17.235,0.391,0.519,5.946E-002,0.113,7.736E-002,0.147,0.280,3.457E-002";

fn compile_models(epjson: &str) -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(epjson)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "screen model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result
        .model
        .ok_or("screen compiler returned no typed model")?;
    Ok((raw_model, model))
}

fn test_models() -> Result<(RawModel, TypedModel), Box<dyn std::error::Error>> {
    compile_models(SCREEN_EPJSON)
}

fn materials_only_epjson() -> String {
    SCREEN_EPJSON.replace(
        "\"details_type_1\": \"Constructions\",\n            \"details_type_2\": \"Materials\"",
        "\"details_type_1\": \"Materials\"",
    )
}

fn constructions_only_epjson() -> String {
    SCREEN_EPJSON.replace(
        "\"details_type_1\": \"Constructions\",\n            \"details_type_2\": \"Materials\"",
        "\"details_type_1\": \"Constructions\"",
    )
}

fn duplicate_screen_control_epjson() -> String {
    SCREEN_EPJSON.replace(
        "\"WindowShadingControl\": {",
        r#""WindowShadingControl": {
        "DUPLICATE DEFAULTED SCREEN CONTROL": {
            "shading_type": "ExteriorScreen",
            "construction_with_shading_name": "B DEFAULTED SCREEN WINDOW CONSTRUCTION",
            "fenestration_surfaces": [
                {"fenestration_surface_name": "DISTINCTIVE DEFAULTED SCREEN TEST WINDOW"}
            ]
        },"#,
    )
}

fn unmatched_d_bare_epjson() -> String {
    SCREEN_EPJSON
        .replace(
            "\"DISTINCTIVE SCREEN TEST GLASS\": {\n            \"optical_data_type\": \"SpectralAverage\",\n            \"thickness\": 0.006\n        }",
            "\"DISTINCTIVE SCREEN TEST GLASS\": {\n            \"optical_data_type\": \"SpectralAverage\",\n            \"thickness\": 0.006\n        },\n        \"UNMATCHED D SCREEN TEST GLASS\": {\n            \"optical_data_type\": \"SpectralAverage\",\n            \"thickness\": 0.007\n        }",
        )
        .replace(
            "\"D HIGH PRECISION SECOND SCREEN WINDOW CONSTRUCTION\": {\n            \"outside_layer\": \"Z HIGH PRECISION REUSED SCREEN\",\n            \"layer_2\": \"DISTINCTIVE SCREEN TEST GLASS\"",
            "\"D HIGH PRECISION SECOND SCREEN WINDOW CONSTRUCTION\": {\n            \"outside_layer\": \"Z HIGH PRECISION REUSED SCREEN\",\n            \"layer_2\": \"UNMATCHED D SCREEN TEST GLASS\"",
        )
}

fn exact_eio() -> String {
    [
        GENERIC_HEADER,
        Z_GENERIC,
        M_GENERIC,
        A_GENERIC,
        WINDOW_MATERIAL_SCREEN_HEADER,
        A_SPECIALIZED,
        Z_SPECIALIZED,
        Z_SPECIALIZED,
        "",
    ]
    .join("\n")
}

fn materials_only_eio() -> String {
    [GENERIC_HEADER, Z_GENERIC, M_GENERIC, A_GENERIC, ""].join("\n")
}

fn constructions_only_eio() -> String {
    [
        WINDOW_MATERIAL_SCREEN_HEADER,
        A_SPECIALIZED,
        Z_SPECIALIZED,
        Z_SPECIALIZED,
        "",
    ]
    .join("\n")
}

#[test]
fn activation_and_occurrence_order_match_fixture_a_z_z() -> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let activated = activated_window_screen_material_names(&raw_model, &model)?;
    let occurrences = window_screen_occurrences(&model, &activated)?;

    assert_eq!(
        activated,
        ["A DEFAULTED USED SCREEN", "Z HIGH PRECISION REUSED SCREEN"]
            .into_iter()
            .map(str::to_string)
            .collect()
    );
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
                "B DEFAULTED SCREEN WINDOW CONSTRUCTION",
                1,
                "A DEFAULTED USED SCREEN"
            ),
            (
                "C HIGH PRECISION FIRST SCREEN WINDOW CONSTRUCTION",
                1,
                "Z HIGH PRECISION REUSED SCREEN"
            ),
            (
                "D HIGH PRECISION SECOND SCREEN WINDOW CONSTRUCTION",
                1,
                "Z HIGH PRECISION REUSED SCREEN"
            ),
        ]
    );
    assert!(
        occurrences
            .iter()
            .all(|row| row.material_name != "M UNUSED SCREEN")
    );
    Ok(())
}

#[test]
fn source_replay_matches_energyplus_26_1_normal_and_reverse_18_by_18_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = test_models()?;
    let definitions = window_screen_definitions(&model);
    let a = definitions
        .iter()
        .find(|row| row.material_name == "A DEFAULTED USED SCREEN")
        .ok_or("missing A")?;
    let z = definitions
        .iter()
        .find(|row| row.material_name == "Z HIGH PRECISION REUSED SCREEN")
        .ok_or("missing Z")?;
    let a_optics = calculate_screen_source_optics(a.fields)?;
    let z_optics = calculate_screen_source_optics(z.fields)?;

    assert_eq!(
        energyplus_round_sig_digits_nonnegative(a_optics.normal_solar_transmittance, 3),
        Some(0.640)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(a_optics.normal_solar_reflectance, 3),
        Some(0.03561)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(a_optics.normal_visible_reflectance, 3),
        Some(0.07050)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(a_optics.diffuse_solar_reflectance, 3),
        Some(0.04068)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(a_optics.diffuse_visible_reflectance, 3),
        Some(0.08162)
    );

    assert_eq!(
        energyplus_round_sig_digits_nonnegative(z_optics.normal_solar_transmittance, 3),
        Some(0.519)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(z_optics.normal_solar_reflectance, 3),
        Some(0.05946)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(z_optics.normal_visible_reflectance, 3),
        Some(0.113)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(z_optics.diffuse_solar_reflectance, 3),
        Some(0.07736)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(z_optics.diffuse_visible_reflectance, 3),
        Some(0.147)
    );
    Ok(())
}

#[test]
fn source_reconstruction_preserves_rounding_boundary_order()
-> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = test_models()?;
    let mut fields = window_screen_definitions(&model)
        .into_iter()
        .find(|row| row.material_name == "A DEFAULTED USED SCREEN")
        .ok_or("missing A")?
        .fields;

    fields.screen_material_diameter_m = 1.0367449437653353e-7;
    fields.screen_material_spacing_m = 3.657274186094305e-6;
    let direct_ratio = fields.screen_material_diameter_m / fields.screen_material_spacing_m;
    fields.direct_normal_transmittance = (1.0 - direct_ratio) * (1.0 - direct_ratio);
    let reconstructed = screen_static_inputs(fields)?;
    assert_eq!(direct_ratio.to_bits(), 0x3f9d_071e_f386_e04e);
    assert_eq!(
        reconstructed.diameter_to_spacing_ratio.to_bits(),
        0x3f9d_071e_f386_e040
    );
    assert_ne!(
        direct_ratio.to_bits(),
        reconstructed.diameter_to_spacing_ratio.to_bits()
    );

    fields.screen_material_diameter_m = 0.0003312804153522599;
    fields.screen_material_spacing_m = 0.0005835431686616492;
    let ratio = fields.screen_material_diameter_m / fields.screen_material_spacing_m;
    fields.direct_normal_transmittance = (1.0 - ratio) * (1.0 - ratio);
    let solid_fraction = 1.0 - fields.direct_normal_transmittance;
    let raw_cylinder_reflectance = f64::from_bits(0x3fed_7358_d23a_f6ff);
    fields.solar_reflectance = raw_cylinder_reflectance * solid_fraction;
    fields.visible_reflectance = raw_cylinder_reflectance * solid_fraction;
    let reconstructed = screen_static_inputs(fields)?;
    assert_eq!(raw_cylinder_reflectance, 0.9203304391919288);
    assert_eq!(
        reconstructed.cylinder_solar_reflectance.to_bits(),
        0x3fed_7358_d23a_f700
    );
    assert_eq!(
        reconstructed.cylinder_visible_reflectance.to_bits(),
        0x3fed_7358_d23a_f700
    );
    assert_ne!(
        raw_cylinder_reflectance.to_bits(),
        reconstructed.cylinder_solar_reflectance.to_bits()
    );
    Ok(())
}

#[test]
fn exact_comparison_gates_all_generic_definitions_and_specialized_a_z_z()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let comparison = compare_window_material_screen(
        &raw_model,
        &model,
        &exact_eio(),
        NumericToleranceMode::Exact,
    )?;

    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.definitions.len(), 3);
    assert_eq!(comparison.activated_material_names.len(), 2);
    assert_eq!(comparison.occurrences.len(), 3);
    assert_eq!(comparison.oracle_occurrences.len(), 3);
    assert_eq!(comparison.header_rows, 1);
    Ok(())
}

#[test]
fn generic_match_rejects_each_exposed_field() -> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = test_models()?;
    let definition = window_screen_definitions(&model)
        .into_iter()
        .find(|row| row.material_name == "A DEFAULTED USED SCREEN")
        .ok_or("missing A")?;
    let base = EioMaterialDetails {
        material_name: definition.material_name.clone(),
        thermal_resistance_m2_k_per_w: 0.0,
        roughness: "MediumRough".to_string(),
        thickness_m: 0.002,
        conductivity_w_per_m_k: 221.0,
        density_kg_per_m3: 0.0,
        specific_heat_j_per_kg_k: 0.0,
        thermal_absorptance: 0.324,
        solar_absorptance: 0.324,
        visible_absorptance: 0.288,
    };
    assert!(generic_row_matches(&definition, &base));

    for mismatch in 0..10 {
        let mut row = base.clone();
        match mismatch {
            0 => row.material_name = "OTHER".to_string(),
            1 => row.roughness = "Smooth".to_string(),
            2 => row.thermal_resistance_m2_k_per_w = 0.1,
            3 => row.thickness_m += 0.1,
            4 => row.conductivity_w_per_m_k += 0.1,
            5 => row.density_kg_per_m3 = 1.0,
            6 => row.specific_heat_j_per_kg_k = 1.0,
            7 => row.thermal_absorptance += 0.1,
            8 => row.solar_absorptance += 0.1,
            9 => row.visible_absorptance += 0.1,
            _ => unreachable!(),
        }
        assert!(
            !generic_row_matches(&definition, &row),
            "mismatch {mismatch}"
        );
    }
    Ok(())
}

#[test]
fn specialized_match_rejects_each_exposed_field() -> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = test_models()?;
    let activated = activated_window_screen_material_names(&raw_model, &model)?;
    let occurrence = window_screen_occurrences(&model, &activated)?
        .into_iter()
        .next()
        .ok_or("missing A occurrence")?;
    let base = EioWindowMaterialScreen {
        material_name: occurrence.material_name.clone(),
        thickness_m: 0.002,
        conductivity_w_per_m_k: 221.0,
        thermal_absorptance: 0.324,
        solar_transmittance: 0.640,
        solar_reflectance: 0.03561,
        visible_reflectance: 0.07050,
        diffuse_solar_reflectance: 0.04068,
        diffuse_visible_reflectance: 0.08162,
        diameter_to_spacing_ratio: 0.2,
        screen_to_glass_distance_m: 0.025,
    };
    assert!(specialized_row_matches(
        &occurrence,
        &base,
        NumericToleranceMode::Exact
    ));

    for mismatch in 0..11 {
        let mut row = base.clone();
        match mismatch {
            0 => row.material_name = "OTHER".to_string(),
            1 => row.thickness_m += 0.1,
            2 => row.conductivity_w_per_m_k += 0.1,
            3 => row.thermal_absorptance += 0.1,
            4 => row.solar_transmittance += 0.1,
            5 => row.solar_reflectance += 0.1,
            6 => row.visible_reflectance += 0.1,
            7 => row.diffuse_solar_reflectance += 0.1,
            8 => row.diffuse_visible_reflectance += 0.1,
            9 => row.diameter_to_spacing_ratio += 0.1,
            10 => row.screen_to_glass_distance_m += 0.1,
            _ => unreachable!(),
        }
        assert!(
            !specialized_row_matches(&occurrence, &row, NumericToleranceMode::Exact),
            "mismatch {mismatch}"
        );
    }
    Ok(())
}

#[test]
fn materials_only_accepts_missing_specialized_header() -> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&materials_only_epjson())?;
    let requests = construction_report_requests(&raw_model)?;
    assert!(requests.materials);
    assert!(!requests.constructions);

    let comparison = compare_window_material_screen(
        &raw_model,
        &model,
        &materials_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert!(comparison.occurrences.is_empty());
    assert!(comparison.oracle_occurrences.is_empty());
    assert_eq!(comparison.header_rows, 0);

    let malformed_eio = format!("{}\n{A_SPECIALIZED}\n", materials_only_eio());
    let error = compare_window_material_screen(
        &raw_model,
        &model,
        &malformed_eio,
        NumericToleranceMode::Exact,
    )
    .expect_err("a specialized Screen row without its exact header must fail closed");
    assert!(
        error.contains("row appears without the exact WindowMaterial:Screen header"),
        "{error}"
    );
    Ok(())
}

#[test]
fn constructions_only_accepts_specialized_a_z_z_without_generic_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&constructions_only_epjson())?;
    let requests = construction_report_requests(&raw_model)?;
    assert!(!requests.materials);
    assert!(requests.constructions);

    let comparison = compare_window_material_screen(
        &raw_model,
        &model,
        &constructions_only_eio(),
        NumericToleranceMode::Exact,
    )?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.definitions.len(), 3);
    assert!(comparison.oracle_material_details.is_empty());
    assert_eq!(comparison.activated_material_names.len(), 2);
    assert_eq!(comparison.occurrences.len(), 3);
    assert_eq!(comparison.oracle_occurrences.len(), 3);
    assert_eq!(comparison.header_rows, 1);
    Ok(())
}

#[test]
fn constructions_header_without_screen_occurrences_is_legal()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(HEADER_ONLY_EPJSON)?;
    let requests = construction_report_requests(&raw_model)?;
    assert!(requests.constructions);
    assert!(!requests.materials);
    let eio = format!("{WINDOW_MATERIAL_SCREEN_HEADER}\n");
    let comparison =
        compare_window_material_screen(&raw_model, &model, &eio, NumericToleranceMode::Exact)?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.definitions.len(), 1);
    assert!(comparison.activated_material_names.is_empty());
    assert!(comparison.occurrences.is_empty());
    assert!(comparison.oracle_occurrences.is_empty());
    assert_eq!(comparison.header_rows, 1);
    Ok(())
}

#[test]
fn duplicate_surface_control_reference_fails_closed() -> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&duplicate_screen_control_epjson())?;
    let error = activated_window_screen_material_names(&raw_model, &model)
        .expect_err("duplicate surface control must fail");
    assert!(error.contains("multiple WindowShadingControl"), "{error}");
    assert!(
        error.contains("active-control selection ambiguity"),
        "{error}"
    );
    assert!(
        error.contains("DUPLICATE DEFAULTED SCREEN CONTROL"),
        "{error}"
    );
    assert!(error.contains("DEFAULTED SCREEN CONTROL"), "{error}");
    Ok(())
}

#[test]
fn screen_occurrence_without_exact_bare_layer_tail_fails_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let (raw_model, model) = compile_models(&unmatched_d_bare_epjson())?;
    let activated = activated_window_screen_material_names(&raw_model, &model)?;
    let error = window_screen_occurrences(&model, &activated)
        .expect_err("unmatched D construction must be omitted by EnergyPlus");
    assert!(error.contains("D HIGH PRECISION SECOND"), "{error}");
    assert!(
        error.contains("exact bare fenestration construction"),
        "{error}"
    );
    Ok(())
}

#[test]
fn unactivated_used_material_fails_closed() -> Result<(), Box<dyn std::error::Error>> {
    let (mut raw_model, model) = test_models()?;
    raw_model
        .objects
        .get_mut(&ObjectType(WINDOW_SHADING_CONTROL_OBJECT_TYPE.to_string()))
        .ok_or("missing WindowShadingControl map")?
        .remove(&ObjectName("HIGH PRECISION SCREEN CONTROL".to_string()));
    let activated = activated_window_screen_material_names(&raw_model, &model)?;
    let error = window_screen_occurrences(&model, &activated).expect_err("Z must be unactivated");
    assert!(
        error.contains("without an explicit ExteriorScreen"),
        "{error}"
    );
    Ok(())
}

#[test]
fn zero_reflectance_source_nan_branch_fails_closed() -> Result<(), Box<dyn std::error::Error>> {
    let (_raw_model, model) = test_models()?;
    let mut fields = window_screen_definitions(&model)
        .into_iter()
        .find(|row| row.material_name == "A DEFAULTED USED SCREEN")
        .ok_or("missing A")?
        .fields;
    fields.solar_reflectance = 0.0;
    let error = calculate_screen_source_optics(fields).expect_err("zero reflectance must fail");
    assert!(error.contains("zero-reflectance NaN branch"), "{error}");
    Ok(())
}

#[test]
fn tolerance_and_header_contracts_are_exact() {
    assert_eq!(parse_tolerance_mode(&[]), Ok(NumericToleranceMode::Exact));
    assert_eq!(
        parse_tolerance_mode(&["--tolerance".to_string(), "near".to_string()]),
        Ok(NumericToleranceMode::Near)
    );
    assert!(parse_tolerance_mode(&["--tolerance".to_string(), "wide".to_string()]).is_err());
    assert_eq!(
        window_material_screen_header_count(WINDOW_MATERIAL_SCREEN_HEADER),
        1
    );
    assert_eq!(
        window_material_screen_header_count("! <WindowMaterial:Screen>, Material Name"),
        0
    );
}

#[test]
fn cli_dispatch_accepts_exact_fixture_and_rejects_bad_arguments()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(run_compare_window_material_screen(&[]), 2);
    assert_eq!(
        run_compare_window_material_screen(&["only-input.epJSON".to_string()]),
        2
    );
    assert_eq!(
        run_compare_window_material_screen(&[
            "input.epJSON".to_string(),
            "eplusout.eio".to_string(),
            "--oracle-eio".to_string(),
            "unexpected.eio".to_string(),
        ]),
        2
    );

    let directory = unique_test_directory();
    std::fs::create_dir_all(&directory)?;
    let input_path = directory.join("screen.epJSON");
    let eio_path = directory.join("eplusout.eio");
    std::fs::write(&input_path, SCREEN_EPJSON)?;
    std::fs::write(&eio_path, exact_eio())?;
    let args = vec![
        "window-material-screen".to_string(),
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
        "rusted-energyplus-window-screen-cli-{}-{nonce}",
        std::process::id()
    ))
}
