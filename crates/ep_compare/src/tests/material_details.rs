use crate::{EioError, parse_eio_material_details};

#[test]
fn parses_eio_material_details_rows_and_preserves_repeats() -> Result<(), Box<dyn std::error::Error>>
{
    let rows = parse_eio_material_details(
        r#"! <Material Details>,Material Name,ThermalResistance {m2-K/w},Roughness,Thickness {m},Conductivity {w/m-K},Density {kg/m3},Specific Heat {J/kg-K},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible
 Program Version,EnergyPlus
 Material Details, a count1 xenon mix ,0.0000,MediumRough,1.2700E-002,0.000,0.000,0.000,0.0000,0.0000,0.0000
 Material Details, a count1 xenon mix ,0,MediumRough,0.0127,0,0,0,0,0,0
 Material Details, distinctive gas opaque host material,0.3256,Rough,0.1130,0.347,913.000,836.000,0.8700,0.6300,0.5800
 Material Details Extra,IGNORED,0,MediumRough,0,0,0,0,0,0,0
 material details,IGNORED,0,MediumRough,0,0,0,0,0,0,0
"#,
    )?;

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].material_name, "A COUNT1 XENON MIX");
    assert_eq!(rows[0].thermal_resistance_m2_k_per_w, 0.0);
    assert_eq!(rows[0].roughness, "MediumRough");
    assert_eq!(rows[0].thickness_m, 0.0127);
    assert_eq!(rows[0].conductivity_w_per_m_k, 0.0);
    assert_eq!(rows[0].density_kg_per_m3, 0.0);
    assert_eq!(rows[0].specific_heat_j_per_kg_k, 0.0);
    assert_eq!(rows[0].thermal_absorptance, 0.0);
    assert_eq!(rows[0].solar_absorptance, 0.0);
    assert_eq!(rows[0].visible_absorptance, 0.0);
    assert_eq!(rows[1].material_name, rows[0].material_name);
    assert_eq!(
        rows[1].thermal_resistance_m2_k_per_w,
        rows[0].thermal_resistance_m2_k_per_w
    );
    assert_eq!(
        rows[2].material_name,
        "DISTINCTIVE GAS OPAQUE HOST MATERIAL"
    );
    assert_eq!(rows[2].roughness, "Rough");
    assert_eq!(rows[2].thickness_m, 0.113);
    assert_eq!(rows[2].thermal_resistance_m2_k_per_w, 0.3256);
    assert_eq!(rows[2].conductivity_w_per_m_k, 0.347);
    assert_eq!(rows[2].density_kg_per_m3, 913.0);
    assert_eq!(rows[2].specific_heat_j_per_kg_k, 836.0);
    assert_eq!(rows[2].thermal_absorptance, 0.87);
    assert_eq!(rows[2].solar_absorptance, 0.63);
    assert_eq!(rows[2].visible_absorptance, 0.58);

    Ok(())
}

#[test]
fn eio_material_details_parser_requires_exact_field_count() {
    let too_few =
        parse_eio_material_details("Material Details,MIX,0.1,MediumRough,0.0127,0,0,0,0,0\n")
            .expect_err("a Material Details row with one missing value must fail");
    let too_many = parse_eio_material_details(
        "Material Details,MIX,0.1,MediumRough,0.0127,0,0,0,0,0,0,EXTRA\n",
    )
    .expect_err("a Material Details row with an extra value must fail");

    assert!(matches!(
        too_few,
        EioError::InvalidMaterialDetails { line: 1, .. }
    ));
    assert!(matches!(
        too_many,
        EioError::InvalidMaterialDetails { line: 1, .. }
    ));
}

#[test]
fn eio_material_details_parser_rejects_missing_text_fields() {
    for (row, expected_reason) in [
        (
            "Material Details,,0.1,MediumRough,0.0127,0,0,0,0,0,0\n",
            "missing Material Name",
        ),
        (
            "Material Details,MIX,0.1,,0.0127,0,0,0,0,0,0\n",
            "missing Roughness",
        ),
    ] {
        let error = parse_eio_material_details(row).expect_err("blank required text must fail");
        assert!(matches!(&error, EioError::InvalidMaterialDetails { .. }));
        if let EioError::InvalidMaterialDetails { line, reason, .. } = error {
            assert_eq!(line, 1);
            assert_eq!(reason, expected_reason);
        }
    }
}

#[test]
fn eio_material_details_parser_rejects_invalid_and_nonfinite_numbers() {
    const NUMERIC_FIELD_NAMES: [&str; 8] = [
        "ThermalResistance {m2-K/w}",
        "Thickness {m}",
        "Conductivity {w/m-K}",
        "Density {kg/m3}",
        "Specific Heat {J/kg-K}",
        "Absorptance:Thermal",
        "Absorptance:Solar",
        "Absorptance:Visible",
    ];
    const NUMERIC_FIELD_INDICES: [usize; 8] = [2, 4, 5, 6, 7, 8, 9, 10];

    for (index, field_name) in NUMERIC_FIELD_INDICES.into_iter().zip(NUMERIC_FIELD_NAMES) {
        for invalid in ["not-a-number", "NaN", "inf", "-inf"] {
            let mut fields = [
                "Material Details",
                "MIX",
                "0.1",
                "MediumRough",
                "0.0127",
                "0",
                "0",
                "0",
                "0",
                "0",
                "0",
            ];
            fields[index] = invalid;
            let error = parse_eio_material_details(&format!("{}\n", fields.join(",")))
                .expect_err("invalid or non-finite Material Details numbers must fail");
            assert!(matches!(&error, EioError::InvalidMaterialDetails { .. }));
            if let EioError::InvalidMaterialDetails { line, reason, .. } = error {
                assert_eq!(line, 1);
                let expected_reason = if invalid == "not-a-number" {
                    format!("invalid {field_name}")
                } else {
                    format!("{field_name} must be finite")
                };
                assert_eq!(reason, expected_reason);
            }
        }
    }
}

#[test]
fn eio_material_details_parser_allows_zero_but_rejects_negative_thickness()
-> Result<(), Box<dyn std::error::Error>> {
    let zero = parse_eio_material_details("Material Details,MIX,0,MediumRough,0,0,0,0,0,0,0\n")?;
    assert_eq!(zero[0].thickness_m, 0.0);

    let error =
        parse_eio_material_details("Material Details,MIX,0,MediumRough,-0.0001,0,0,0,0,0,0\n")
            .expect_err("negative material thickness must fail");
    assert!(matches!(&error, EioError::InvalidMaterialDetails { .. }));
    if let EioError::InvalidMaterialDetails { line, reason, .. } = error {
        assert_eq!(line, 1);
        assert_eq!(reason, "Thickness {m} must be nonnegative");
    }
    Ok(())
}

#[test]
fn eio_material_details_parser_reports_missing_rows() {
    assert!(matches!(
        parse_eio_material_details("Program Version,EnergyPlus\n"),
        Err(EioError::MissingMaterialDetails)
    ));
}
