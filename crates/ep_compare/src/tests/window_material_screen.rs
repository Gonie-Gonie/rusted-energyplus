use crate::{
    EioError, WINDOW_MATERIAL_SCREEN_HEADER, parse_eio_material_details,
    parse_eio_window_material_screen,
};

const SOURCE_ROW: &str = " WindowMaterial:Screen, reused screen ,1.2346E-004,2.21E+002,3.94E-001,5.62E-001,8.75E-002,1.31E-001,2.00E-001,3.00E-001,2.50E-001,2.50E-002";

#[test]
fn parses_exact_source_screen_header_and_duplicate_aware_rows()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        WINDOW_MATERIAL_SCREEN_HEADER,
        "! <WindowMaterial:Screen>,Material Name,Thickness {m},Conductivity {W/m-K},Thermal Absorptance,Transmittance,Reflectance,Visible Reflectance,Diffuse Reflectance,Diffuse Visible Reflectance,Screen Material Diameter To Spacing Ratio,Screen To GlassDistance {m}"
    );
    assert_eq!(WINDOW_MATERIAL_SCREEN_HEADER.split(',').count(), 12);

    let contents = format!(
        "! <Material Details>,Material Name,ThermalResistance {{m2-K/w}},Roughness,Thickness {{m}},Conductivity {{w/m-K}},Density {{kg/m3}},Specific Heat {{J/kg-K}},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible\n\
         Material Details, unused screen definition ,0,MediumRough,0.0005,221,0,0,0.39375,0.35,0.30625\n\
         {WINDOW_MATERIAL_SCREEN_HEADER}\n\
         Program Version,EnergyPlus\n\
         {SOURCE_ROW}\n\
         WindowMaterial:Screen, default screen ,0.00050,221,0.394,0.562,0.088,0.131,0.2,0.3,0.25,0.025\n\
         WindowMaterial:Screen:EquivalentLayer,IGNORED,0,0,0,0,0,0,0,0,0,0\n\
         WindowMaterial:ScreenExtra,IGNORED,0,0,0,0,0,0,0,0,0,0\n\
         windowmaterial:screen,IGNORED,0,0,0,0,0,0,0,0,0,0\n\
         {SOURCE_ROW}\n"
    );

    // Screen definitions continue to use the existing generic parser; the
    // specialized parser owns only construction-layer occurrence rows.
    let generic_rows = parse_eio_material_details(&contents)?;
    assert_eq!(generic_rows.len(), 1);
    assert_eq!(generic_rows[0].material_name, "UNUSED SCREEN DEFINITION");

    let rows = parse_eio_window_material_screen(&contents)?;
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].material_name, "REUSED SCREEN");
    assert_eq!(rows[0].thickness_m, 0.00012346);
    assert_eq!(rows[0].conductivity_w_per_m_k, 221.0);
    assert_eq!(rows[0].thermal_absorptance, 0.394);
    assert_eq!(rows[0].solar_transmittance, 0.562);
    assert_eq!(rows[0].solar_reflectance, 0.0875);
    assert_eq!(rows[0].visible_reflectance, 0.131);
    assert_eq!(rows[0].diffuse_solar_reflectance, 0.2);
    assert_eq!(rows[0].diffuse_visible_reflectance, 0.3);
    assert_eq!(rows[0].diameter_to_spacing_ratio, 0.25);
    assert_eq!(rows[0].screen_to_glass_distance_m, 0.025);
    assert_eq!(rows[1].material_name, "DEFAULT SCREEN");
    assert_eq!(rows[1].thickness_m, 0.0005);
    assert_eq!(rows[2], rows[0]);
    Ok(())
}

#[test]
fn screen_parser_rejects_malformed_header_literals() {
    for malformed in [
        WINDOW_MATERIAL_SCREEN_HEADER.replace("GlassDistance", "Glass Distance"),
        format!("{WINDOW_MATERIAL_SCREEN_HEADER},Extra"),
        format!(" {WINDOW_MATERIAL_SCREEN_HEADER}"),
    ] {
        let contents = format!("{malformed}\n{SOURCE_ROW}\n");
        let error = parse_eio_window_material_screen(&contents)
            .expect_err("a non-literal Screen header must fail");
        assert!(matches!(
            &error,
            EioError::InvalidWindowMaterialScreenHeader { line: 1, .. }
        ));
        if let EioError::InvalidWindowMaterialScreenHeader { text, reason, .. } = error {
            assert_eq!(text, malformed);
            assert_eq!(
                reason,
                "header must exactly match the EnergyPlus 26.1 source literal"
            );
        }
    }
}

#[test]
fn screen_parser_requires_one_exact_header() {
    let missing = parse_eio_window_material_screen(&format!(
        "! <WindowMaterial:Screen:EquivalentLayer>,Material Name\n{SOURCE_ROW}\n"
    ))
    .expect_err("a Screen row without its specialized header must fail");
    assert!(matches!(
        &missing,
        EioError::InvalidWindowMaterialScreen { line: 2, .. }
    ));
    if let EioError::InvalidWindowMaterialScreen { reason, .. } = missing {
        assert_eq!(
            reason,
            "row appears without the exact WindowMaterial:Screen header"
        );
    }

    let duplicate = parse_eio_window_material_screen(&format!(
        "{WINDOW_MATERIAL_SCREEN_HEADER}\n{SOURCE_ROW}\n{WINDOW_MATERIAL_SCREEN_HEADER}\n"
    ))
    .expect_err("a repeated exact header must fail");
    assert!(matches!(
        &duplicate,
        EioError::DuplicateWindowMaterialScreenHeader { line: 3, .. }
    ));
    if let EioError::DuplicateWindowMaterialScreenHeader { text, .. } = duplicate {
        assert_eq!(text, WINDOW_MATERIAL_SCREEN_HEADER);
    }
}

#[test]
fn screen_parser_rejects_rows_before_header_and_wrong_token_counts() {
    let row_before_header = parse_eio_window_material_screen(&format!(
        "{SOURCE_ROW}\n{WINDOW_MATERIAL_SCREEN_HEADER}\n"
    ))
    .expect_err("a row before the source header must fail");
    assert!(matches!(
        &row_before_header,
        EioError::InvalidWindowMaterialScreen { line: 1, .. }
    ));
    if let EioError::InvalidWindowMaterialScreen { reason, .. } = row_before_header {
        assert_eq!(
            reason,
            "row appears before the exact WindowMaterial:Screen header"
        );
    }

    for row in [
        "WindowMaterial:Screen,SCREEN,0.0005,221,0.394,0.562,0.088,0.131,0.2,0.3,0.25",
        "WindowMaterial:Screen,SCREEN,0.0005,221,0.394,0.562,0.088,0.131,0.2,0.3,0.25,0.025,EXTRA",
    ] {
        let error =
            parse_eio_window_material_screen(&format!("{WINDOW_MATERIAL_SCREEN_HEADER}\n{row}\n"))
                .expect_err("a Screen row with a non-source token count must fail");
        assert!(matches!(
            error,
            EioError::InvalidWindowMaterialScreen { line: 2, .. }
        ));
    }
}

#[test]
fn screen_parser_rejects_blank_names_and_every_invalid_numeric_field() {
    let missing_name = parse_eio_window_material_screen(&format!(
        "{WINDOW_MATERIAL_SCREEN_HEADER}\n\
         WindowMaterial:Screen,,0.0005,221,0.394,0.562,0.088,0.131,0.2,0.3,0.25,0.025\n"
    ))
    .expect_err("a blank Screen material name must fail");
    assert!(matches!(
        &missing_name,
        EioError::InvalidWindowMaterialScreen { line: 2, .. }
    ));
    if let EioError::InvalidWindowMaterialScreen { reason, .. } = missing_name {
        assert_eq!(reason, "missing Material Name");
    }

    const FIELD_NAMES: [&str; 10] = [
        "Thickness {m}",
        "Conductivity {W/m-K}",
        "Thermal Absorptance",
        "Transmittance",
        "Reflectance",
        "Visible Reflectance",
        "Diffuse Reflectance",
        "Diffuse Visible Reflectance",
        "Screen Material Diameter To Spacing Ratio",
        "Screen To GlassDistance {m}",
    ];
    for (index, field_name) in (2..=11).zip(FIELD_NAMES) {
        for invalid in ["not-a-number", "NaN", "inf", "-inf"] {
            let mut fields = [
                "WindowMaterial:Screen",
                "SCREEN",
                "0.0005",
                "221",
                "0.394",
                "0.562",
                "0.088",
                "0.131",
                "0.2",
                "0.3",
                "0.25",
                "0.025",
            ];
            fields[index] = invalid;
            let error = parse_eio_window_material_screen(&format!(
                "{WINDOW_MATERIAL_SCREEN_HEADER}\n{}\n",
                fields.join(",")
            ))
            .expect_err("invalid or non-finite Screen values must fail");
            assert!(matches!(
                &error,
                EioError::InvalidWindowMaterialScreen { line: 2, .. }
            ));
            if let EioError::InvalidWindowMaterialScreen { reason, .. } = error {
                let expected = if invalid == "not-a-number" {
                    format!("invalid {field_name}")
                } else {
                    format!("{field_name} must be finite")
                };
                assert_eq!(reason, expected);
            }
        }
    }
}

#[test]
fn screen_parser_accepts_exact_header_without_rows() -> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_eio_window_material_screen(&format!(
        "{WINDOW_MATERIAL_SCREEN_HEADER}\nProgram Version,EnergyPlus\n"
    ))?;
    assert!(rows.is_empty());
    Ok(())
}
