use crate::{EioError, WINDOW_MATERIAL_SHADE_HEADER, parse_eio_window_material_shade};

#[test]
fn parses_window_material_shade_rows_in_order_and_preserves_repeats()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        WINDOW_MATERIAL_SHADE_HEADER,
        "! <WindowMaterial:Shade>,Material Name,Thickness {m},Conductivity {W/m-K},Thermal Absorptance,Transmittance,Visible Transmittance,Shade Reflectance"
    );
    let rows = parse_eio_window_material_shade(&format!(
        "{WINDOW_MATERIAL_SHADE_HEADER}\n\
         Program Version,EnergyPlus\n\
         WindowMaterial:Shade, reused shade ,4.567E-004,0.123,0.568,0.123,0.346,0.235\n\
         WindowMaterial:Gas,IGNORED,Argon,0.0127\n\
         WindowMaterial:Shade, second shade ,0.001,2.5E-1,0.8,0.1,0.2,0.3\n\
         WindowMaterial:Shade, reused shade ,4.567E-004,0.123,0.568,0.123,0.346,0.235\n\
         WindowMaterial:Shade Extra,IGNORED,0,0,0,0,0,0\n"
    ))?;

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].material_name, "REUSED SHADE");
    assert_eq!(rows[0].thickness_m, 0.0004567);
    assert_eq!(rows[0].conductivity_w_per_m_k, 0.123);
    assert_eq!(rows[0].thermal_absorptance, 0.568);
    assert_eq!(rows[0].solar_transmittance, 0.123);
    assert_eq!(rows[0].visible_transmittance, 0.346);
    assert_eq!(rows[0].solar_reflectance, 0.235);
    assert_eq!(rows[1].material_name, "SECOND SHADE");
    assert_eq!(rows[1].thickness_m, 0.001);
    assert_eq!(rows[2], rows[0]);

    Ok(())
}

#[test]
fn window_material_shade_parser_requires_exact_field_count()
-> Result<(), Box<dyn std::error::Error>> {
    for row in [
        "WindowMaterial:Shade,SHADE,0.001,0.2,0.8,0.1,0.2\n",
        "WindowMaterial:Shade,SHADE,0.001,0.2,0.8,0.1,0.2,0.3,EXTRA\n",
    ] {
        let error = parser_error(row)?;
        assert!(matches!(
            error,
            EioError::InvalidWindowMaterialShade { line: 1, .. }
        ));
    }

    Ok(())
}

#[test]
fn window_material_shade_parser_rejects_missing_name_and_invalid_numbers()
-> Result<(), Box<dyn std::error::Error>> {
    let missing_name = parser_error("WindowMaterial:Shade,,0.001,0.2,0.8,0.1,0.2,0.3\n")?;
    assert!(matches!(
        &missing_name,
        EioError::InvalidWindowMaterialShade { line: 1, .. }
    ));
    if let EioError::InvalidWindowMaterialShade { reason, .. } = missing_name {
        assert_eq!(reason, "missing Material Name");
    }

    const FIELD_NAMES: [&str; 6] = [
        "Thickness {m}",
        "Conductivity {W/m-K}",
        "Thermal Absorptance",
        "Transmittance",
        "Visible Transmittance",
        "Shade Reflectance",
    ];
    for (index, field_name) in (2..=7).zip(FIELD_NAMES) {
        for invalid in ["not-a-number", "NaN", "inf", "-inf"] {
            let mut fields = [
                "WindowMaterial:Shade",
                "SHADE",
                "0.001",
                "0.2",
                "0.8",
                "0.1",
                "0.2",
                "0.3",
            ];
            fields[index] = invalid;
            let error = parser_error(&format!("{}\n", fields.join(",")))?;
            let EioError::InvalidWindowMaterialShade { line, reason, .. } = error else {
                return Err("expected InvalidWindowMaterialShade".into());
            };
            assert_eq!(line, 1);
            let expected_reason = if invalid == "not-a-number" {
                format!("invalid {field_name}")
            } else {
                format!("{field_name} must be finite")
            };
            assert_eq!(reason, expected_reason);
        }
    }

    Ok(())
}

#[test]
fn window_material_shade_parser_reports_missing_rows() {
    assert!(matches!(
        parse_eio_window_material_shade("Program Version,EnergyPlus\n"),
        Err(EioError::MissingWindowMaterialShade)
    ));
}

fn parser_error(contents: &str) -> Result<EioError, Box<dyn std::error::Error>> {
    match parse_eio_window_material_shade(contents) {
        Err(error) => Ok(error),
        Ok(rows) => Err(format!("expected parser error, parsed {rows:?}").into()),
    }
}
