use std::path::PathBuf;

use ep_compare::{EioMaterialDetails, EioWindowMaterialShade, WINDOW_MATERIAL_SHADE_HEADER};
use ep_model::TypedModel;
use ep_raw_model::parse_epjson_str;

use super::{
    compare_window_material_shade, energyplus_round_sig_digits_nonnegative,
    indices_by_material_name, normalized_material_name, run_compare_window_material_shade,
    window_material_shade_header_count, window_shade_definitions, window_shade_generic_row_matches,
    window_shade_occurrence_row_matches, window_shade_occurrences,
};

const SHADE_EPJSON: &str = r#"{
    "WindowMaterial:Glazing": {
        "C Clear Glass": {
            "optical_data_type": "SpectralAverage",
            "thickness": 0.003
        }
    },
    "WindowMaterial:Shade": {
        "A Defaulted Unused Shade": {
            "solar_transmittance": 0.1,
            "solar_reflectance": 0.3,
            "visible_transmittance": 0.2,
            "visible_reflectance": 0.3,
            "infrared_hemispherical_emissivity": 0.76543,
            "infrared_transmittance": 0.1,
            "thickness": 0.00123456,
            "conductivity": 0.98765
        },
        "B Reused Shade": {
            "solar_transmittance": 0.12345,
            "solar_reflectance": 0.23456,
            "visible_transmittance": 0.34567,
            "visible_reflectance": 0.12345,
            "infrared_hemispherical_emissivity": 0.56789,
            "infrared_transmittance": 0.12345,
            "thickness": 0.00045678,
            "conductivity": 0.12345
        }
    },
    "Construction": {
        "A Exterior Shade Construction": {
            "outside_layer": "B Reused Shade",
            "layer_2": "C Clear Glass"
        },
        "B Interior Shade Construction": {
            "outside_layer": "C Clear Glass",
            "layer_2": "B Reused Shade"
        }
    }
}"#;

fn shade_test_model() -> Result<TypedModel, Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(SHADE_EPJSON)?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "shade model failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    result
        .model
        .ok_or_else(|| "shade compiler returned no typed model".into())
}

fn exact_eio() -> String {
    [
        "! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible",
        "Material Details,C CLEAR GLASS,0.1500,Smooth,0.0030,1.000,2500.000,750.000,0.8400,0.7000,0.7000",
        "Material Details,B REUSED SHADE,0.0000,MediumRough,4.5678E-004,0.123,0.000,0.000,0.5679,0.6420,0.0000",
        "Material Details,A DEFAULTED UNUSED SHADE,0.0000,MediumRough,1.2346E-003,0.988,0.000,0.000,0.7654,0.6000,0.0000",
        WINDOW_MATERIAL_SHADE_HEADER,
        "WindowMaterial:Shade,B REUSED SHADE,4.568E-004,0.123,0.568,0.123,0.346,0.235",
        "WindowMaterial:Shade,B REUSED SHADE,4.568E-004,0.123,0.568,0.123,0.346,0.235",
        "",
    ]
    .join("\n")
}

#[test]
fn shade_rows_include_all_definitions_and_duplicate_construction_occurrences()
-> Result<(), Box<dyn std::error::Error>> {
    let model = shade_test_model()?;
    let definitions = window_shade_definitions(&model);
    let occurrences = window_shade_occurrences(&model)?;

    assert_eq!(
        definitions
            .iter()
            .map(|row| row.material_name.as_str())
            .collect::<Vec<_>>(),
        vec!["A DEFAULTED UNUSED SHADE", "B REUSED SHADE"]
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
            ("A EXTERIOR SHADE CONSTRUCTION", 1, "B REUSED SHADE"),
            ("B INTERIOR SHADE CONSTRUCTION", 2, "B REUSED SHADE"),
        ]
    );
    assert!(
        occurrences
            .iter()
            .all(|row| row.material_name != "A DEFAULTED UNUSED SHADE"),
        "an unused definition must have a generic row but no specialized occurrence"
    );
    Ok(())
}

#[test]
fn shade_occurrence_scan_ignores_zero_layer_air_boundaries()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "Construction:AirBoundary": {
                "Unused Air Boundary": {"air_exchange_method":"None"}
            }
        }"#,
    )?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed air boundary"))?;
    assert!(window_shade_occurrences(&model)?.is_empty());
    Ok(())
}

#[test]
fn exact_comparison_is_definition_based_and_duplicate_aware()
-> Result<(), Box<dyn std::error::Error>> {
    let model = shade_test_model()?;
    let comparison = compare_window_material_shade(&model, &exact_eio())?;

    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert_eq!(comparison.definitions.len(), 2);
    assert_eq!(comparison.occurrences.len(), 2);
    assert_eq!(comparison.oracle_material_details.len(), 3);
    assert_eq!(comparison.oracle_shade_rows.len(), 2);
    assert_eq!(comparison.shade_header_rows, 1);

    let generic_indices = indices_by_material_name(&comparison.oracle_material_details, |row| {
        row.material_name.as_str()
    });
    assert_eq!(generic_indices["A DEFAULTED UNUSED SHADE"].len(), 1);
    assert_eq!(generic_indices["B REUSED SHADE"].len(), 1);
    assert_eq!(
        generic_indices["C CLEAR GLASS"].len(),
        1,
        "an unrelated generic material row is parsed but ignored by the shade gate"
    );
    Ok(())
}

#[test]
fn generic_matching_rejects_every_exposed_field_mismatch() -> Result<(), Box<dyn std::error::Error>>
{
    let model = shade_test_model()?;
    let definition = window_shade_definitions(&model)
        .into_iter()
        .find(|row| row.material_name == "B REUSED SHADE")
        .ok_or("missing reused shade")?;
    let base = EioMaterialDetails {
        material_name: " b reused shade ".to_string(),
        thermal_resistance_m2_k_per_w: 0.0,
        roughness: "MediumRough".to_string(),
        thickness_m: 0.00045678,
        conductivity_w_per_m_k: 0.123,
        density_kg_per_m3: 0.0,
        specific_heat_j_per_kg_k: 0.0,
        thermal_absorptance: 0.5679,
        solar_absorptance: 0.642,
        visible_absorptance: 0.0,
    };
    assert!(window_shade_generic_row_matches(&definition, &base));

    for mismatch in 0..10 {
        let mut row = base.clone();
        match mismatch {
            0 => row.material_name = "WRONG SHADE".to_string(),
            1 => row.roughness = "Smooth".to_string(),
            2 => row.thermal_resistance_m2_k_per_w = 0.0001,
            3 => row.thickness_m += 0.00000001,
            4 => row.conductivity_w_per_m_k += 0.001,
            5 => row.density_kg_per_m3 = 1.0,
            6 => row.specific_heat_j_per_kg_k = 1.0,
            7 => row.thermal_absorptance += 0.0001,
            8 => row.solar_absorptance += 0.0001,
            9 => row.visible_absorptance = 0.0001,
            _ => unreachable!(),
        }
        assert!(
            !window_shade_generic_row_matches(&definition, &row),
            "mismatch case {mismatch} must fail"
        );
    }
    Ok(())
}

#[test]
fn specialized_matching_rejects_every_exposed_field_mismatch()
-> Result<(), Box<dyn std::error::Error>> {
    let model = shade_test_model()?;
    let occurrence = window_shade_occurrences(&model)?
        .into_iter()
        .next()
        .ok_or("missing shade occurrence")?;
    let base = EioWindowMaterialShade {
        material_name: "b reused shade".to_string(),
        thickness_m: 0.0004568,
        conductivity_w_per_m_k: 0.123,
        thermal_absorptance: 0.568,
        solar_transmittance: 0.123,
        visible_transmittance: 0.346,
        solar_reflectance: 0.235,
    };
    assert!(window_shade_occurrence_row_matches(&occurrence, &base));

    for mismatch in 0..7 {
        let mut row = base.clone();
        match mismatch {
            0 => row.material_name = "WRONG SHADE".to_string(),
            1 => row.thickness_m += 0.0000001,
            2 => row.conductivity_w_per_m_k += 0.001,
            3 => row.thermal_absorptance += 0.001,
            4 => row.solar_transmittance += 0.001,
            5 => row.visible_transmittance += 0.001,
            6 => row.solar_reflectance += 0.001,
            _ => unreachable!(),
        }
        assert!(
            !window_shade_occurrence_row_matches(&occurrence, &row),
            "mismatch case {mismatch} must fail"
        );
    }
    Ok(())
}

#[test]
fn comparison_rejects_missing_duplicate_unexpected_and_malformed_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let model = shade_test_model()?;
    let exact = exact_eio();

    let missing_generic = exact
        .lines()
        .filter(|line| !line.contains("A DEFAULTED UNUSED SHADE"))
        .collect::<Vec<_>>()
        .join("\n");
    let comparison = compare_window_material_shade(&model, &missing_generic)?;
    assert!(!comparison.passed);
    assert!(comparison.first_divergence.as_deref().is_some_and(|value| {
        value.contains("expected exactly one Material Details row observed 0")
    }));

    let duplicate_generic = exact.replacen(
        "Material Details,B REUSED SHADE,",
        "Material Details,B REUSED SHADE,0.0000,MediumRough,4.5678E-004,0.123,0.000,0.000,0.5679,0.6420,0.0000\nMaterial Details,B REUSED SHADE,",
        1,
    );

    let no_generic_rows = exact
        .lines()
        .filter(|line| !line.starts_with("Material Details,"))
        .collect::<Vec<_>>()
        .join("\n");
    let comparison = compare_window_material_shade(&model, &no_generic_rows)?;
    assert!(!comparison.passed);
    assert!(comparison.first_divergence.as_deref().is_some_and(|value| {
        value.contains("expected exactly one Material Details row observed 0")
    }));
    let comparison = compare_window_material_shade(&model, &duplicate_generic)?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("observed 2"))
    );

    let one_occurrence = exact.replacen(
        "WindowMaterial:Shade,B REUSED SHADE,4.568E-004,0.123,0.568,0.123,0.346,0.235\n",
        "",
        1,
    );
    let comparison = compare_window_material_shade(&model, &one_occurrence)?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| { value.contains("shade occurrences expected 2 observed 1") })
    );

    let no_occurrences = exact
        .lines()
        .filter(|line| !line.starts_with("WindowMaterial:Shade,"))
        .collect::<Vec<_>>()
        .join("\n");
    let comparison = compare_window_material_shade(&model, &no_occurrences)?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| { value.contains("shade occurrences expected 2 observed 0") })
    );

    let extra_occurrence = format!(
        "{exact}WindowMaterial:Shade,B REUSED SHADE,4.568E-004,0.123,0.568,0.123,0.346,0.235\n"
    );
    let comparison = compare_window_material_shade(&model, &extra_occurrence)?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| { value.contains("shade occurrences expected 2 observed 3") })
    );

    let unexpected_unused = format!(
        "{exact}WindowMaterial:Shade,A DEFAULTED UNUSED SHADE,1.235E-003,0.988,0.765,0.100,0.200,0.300\n"
    );
    let comparison = compare_window_material_shade(&model, &unexpected_unused)?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| { value.contains("shade occurrences expected 0 observed 1") })
    );

    let unexpected_material = format!(
        "{exact}WindowMaterial:Shade,UNKNOWN SHADE,1.000E-003,0.100,0.800,0.100,0.100,0.100\n"
    );
    let comparison = compare_window_material_shade(&model, &unexpected_material)?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| { value.contains("unexpected WindowMaterial:Shade row") })
    );

    let malformed = exact.replacen(
        "WindowMaterial:Shade,B REUSED SHADE,4.568E-004,0.123,0.568,0.123,0.346,0.235",
        "WindowMaterial:Shade,B REUSED SHADE,4.568E-004",
        1,
    );
    let error = compare_window_material_shade(&model, &malformed)
        .expect_err("a malformed specialized row must fail parsing");
    assert!(
        error.contains("invalid EIO WindowMaterial:Shade"),
        "{error}"
    );
    Ok(())
}

#[test]
fn all_unused_shade_definitions_without_a_window_accept_no_header_or_occurrences()
-> Result<(), Box<dyn std::error::Error>> {
    let raw_model = parse_epjson_str(
        r#"{
            "WindowMaterial:Shade": {
                "Only Unused Shade": {
                    "solar_transmittance": 0.1,
                    "solar_reflectance": 0.3,
                    "visible_transmittance": 0.2,
                    "visible_reflectance": 0.3,
                    "infrared_hemispherical_emissivity": 0.8,
                    "infrared_transmittance": 0.1,
                    "thickness": 0.001,
                    "conductivity": 0.2
                }
            }
        }"#,
    )?;
    let result = ep_compiler::compile_raw_model(&raw_model);
    if result.has_errors() {
        return Err(format!(
            "unused shade failed to compile: {:?}",
            result.report.diagnostics
        )
        .into());
    }
    let model = result.model.ok_or("missing all-unused shade model")?;
    let eio = "Material Details,ONLY UNUSED SHADE,0.0000,MediumRough,1.0000E-003,0.200,0.000,0.000,0.8000,0.6000,0.0000\n";

    let comparison = compare_window_material_shade(&model, eio)?;
    assert!(comparison.passed, "{:?}", comparison.first_divergence);
    assert!(comparison.occurrences.is_empty());
    assert!(comparison.oracle_shade_rows.is_empty());
    assert_eq!(comparison.shade_header_rows, 0);

    let comparison =
        compare_window_material_shade(&model, &format!("{eio}{WINDOW_MATERIAL_SHADE_HEADER}\n"))?;
    assert!(!comparison.passed);
    assert_eq!(
        comparison.first_divergence.as_deref(),
        Some("WindowMaterial:Shade header expected 0 observed 1")
    );
    Ok(())
}

#[test]
fn comparison_rejects_header_and_numeric_mismatches_with_first_divergence()
-> Result<(), Box<dyn std::error::Error>> {
    let model = shade_test_model()?;
    let exact = exact_eio();

    let missing_header = exact.replace(WINDOW_MATERIAL_SHADE_HEADER, "");
    let comparison = compare_window_material_shade(&model, &missing_header)?;
    assert!(!comparison.passed);
    assert_eq!(
        comparison.first_divergence.as_deref(),
        Some("WindowMaterial:Shade header expected 1 observed 0")
    );

    let duplicate_header = exact.replacen(
        WINDOW_MATERIAL_SHADE_HEADER,
        &format!("{WINDOW_MATERIAL_SHADE_HEADER}\n{WINDOW_MATERIAL_SHADE_HEADER}"),
        1,
    );
    let comparison = compare_window_material_shade(&model, &duplicate_header)?;
    assert!(!comparison.passed);
    assert_eq!(comparison.shade_header_rows, 2);

    let generic_mismatch = exact.replacen("0.5679,0.6420,0.0000", "0.5679,0.6419,0.0000", 1);
    let comparison = compare_window_material_shade(&model, &generic_mismatch)?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("field solar_absorptance"))
    );

    let specialized_mismatch = exact.replacen(
        "4.568E-004,0.123,0.568,0.123,0.346,0.235",
        "4.568E-004,0.123,0.568,0.124,0.346,0.235",
        1,
    );
    let comparison = compare_window_material_shade(&model, &specialized_mismatch)?;
    assert!(!comparison.passed);
    assert!(
        comparison
            .first_divergence
            .as_deref()
            .is_some_and(|value| value.contains("field solar_transmittance"))
    );
    Ok(())
}

#[test]
fn shade_rounding_and_exact_header_shape_match_energyplus_26_1() {
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(0.00045678, 4),
        Some(0.00045678)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(0.00045678, 3),
        Some(0.0004568)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(0.12345, 3),
        Some(0.123)
    );
    assert_eq!(
        energyplus_round_sig_digits_nonnegative(0.56789, 4),
        Some(0.5679)
    );
    assert_eq!(energyplus_round_sig_digits_nonnegative(0.0, 3), Some(0.0));
    for invalid in [-0.1, f64::NAN, f64::INFINITY] {
        assert_eq!(energyplus_round_sig_digits_nonnegative(invalid, 3), None);
    }

    assert_eq!(
        window_material_shade_header_count(WINDOW_MATERIAL_SHADE_HEADER),
        1
    );
    assert_eq!(
        window_material_shade_header_count("! <WindowMaterial:Shade>, Material Name,Thickness {m}"),
        0
    );
    assert_eq!(
        normalized_material_name(" b reused shade "),
        "B REUSED SHADE"
    );
}

#[test]
fn cli_command_accepts_exact_fixture_and_rejects_missing_arguments()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(run_compare_window_material_shade(&[]), 2);
    assert_eq!(
        run_compare_window_material_shade(&["only-input.epJSON".to_string()]),
        2
    );

    let directory = unique_test_directory();
    std::fs::create_dir_all(&directory)?;
    let input_path = directory.join("shade.epJSON");
    let eio_path = directory.join("eplusout.eio");
    std::fs::write(&input_path, SHADE_EPJSON)?;
    std::fs::write(&eio_path, exact_eio())?;
    let exit_code = run_compare_window_material_shade(&[
        input_path.to_string_lossy().into_owned(),
        eio_path.to_string_lossy().into_owned(),
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
        "rusted-energyplus-window-shade-cli-{}-{nonce}",
        std::process::id()
    ))
}
