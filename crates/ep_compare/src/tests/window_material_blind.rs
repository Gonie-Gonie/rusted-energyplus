use crate::{
    EioError, WINDOW_MATERIAL_BLIND_HEADER, parse_eio_material_details,
    parse_eio_window_material_blind,
};

const SOURCE_ROW: &str = " WindowMaterial:Blind, reused blind ,2.3457E-002,1.9877E-002,2.4680E-004,47.1,0.123,0.456,2.50E-002";

#[test]
fn parses_exact_source_blind_header_and_duplicate_aware_rows()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        WINDOW_MATERIAL_BLIND_HEADER,
        "! <WindowMaterial:Blind>,Material Name,Slat Width {m},Slat Separation {m},Slat Thickness {m},Slat Angle {deg},Slat Beam Solar Transmittance,Slat Beam Solar Front Reflectance,Blind To Glass Distance {m}"
    );
    assert_eq!(WINDOW_MATERIAL_BLIND_HEADER.split(',').count(), 9);

    let contents = format!(
        "! <Material Details>,Material Name,ThermalResistance {{m2-K/w}},Roughness,Thickness {{m}},Conductivity {{w/m-K}},Density {{kg/m3}},Specific Heat {{J/kg-K}},Absorptance:Thermal,Absorptance:Solar,Absorptance:Visible\n\
         Material Details, unused blind definition ,0,Rough,0,0,0,0,0,0,0\n\
         {WINDOW_MATERIAL_BLIND_HEADER}\n\
         Program Version,EnergyPlus\n\
         {SOURCE_ROW}\n\
         WindowMaterial:Blind, default blind ,0.0200,0.0200,0.0002500,45.0,0,0.2,0.050\n\
         WindowMaterial:Blind:EquivalentLayer,IGNORED,0,0,0,0,0,0,0\n\
         WindowMaterial:BlindExtra,IGNORED,0,0,0,0,0,0,0\n\
         windowmaterial:blind,IGNORED,0,0,0,0,0,0,0\n\
         {SOURCE_ROW}\n"
    );

    // Blind definitions continue to use the generic parser; this parser owns
    // only specialized construction-layer occurrence rows.
    let generic_rows = parse_eio_material_details(&contents)?;
    assert_eq!(generic_rows.len(), 1);
    assert_eq!(generic_rows[0].material_name, "UNUSED BLIND DEFINITION");

    let rows = parse_eio_window_material_blind(&contents)?;
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].material_name, "REUSED BLIND");
    assert_eq!(rows[0].slat_width_m, 0.023457);
    assert_eq!(rows[0].slat_separation_m, 0.019877);
    assert_eq!(rows[0].slat_thickness_m, 0.0002468);
    assert_eq!(rows[0].slat_angle_deg, 47.1);
    assert_eq!(rows[0].slat_beam_solar_transmittance, 0.123);
    assert_eq!(rows[0].slat_beam_solar_front_reflectance, 0.456);
    assert_eq!(rows[0].blind_to_glass_distance_m, 0.025);
    assert_eq!(rows[1].material_name, "DEFAULT BLIND");
    assert_eq!(rows[1].slat_width_m, 0.02);
    assert_eq!(rows[2], rows[0]);
    Ok(())
}

#[test]
fn blind_parser_rejects_malformed_header_literals() {
    for malformed in [
        WINDOW_MATERIAL_BLIND_HEADER.replace("Blind To Glass", "Blind-to-Glass"),
        format!("{WINDOW_MATERIAL_BLIND_HEADER},Extra"),
        format!(" {WINDOW_MATERIAL_BLIND_HEADER}"),
    ] {
        let contents = format!("{malformed}\n{SOURCE_ROW}\n");
        let error = parse_eio_window_material_blind(&contents)
            .expect_err("a non-literal Blind header must fail");
        assert!(matches!(
            &error,
            EioError::InvalidWindowMaterialBlindHeader { line: 1, .. }
        ));
        if let EioError::InvalidWindowMaterialBlindHeader { text, reason, .. } = error {
            assert_eq!(text, malformed);
            assert_eq!(
                reason,
                "header must exactly match the EnergyPlus 26.1 source literal"
            );
        }
    }
}

#[test]
fn blind_parser_requires_one_exact_header() {
    let only_equivalent_layer = parse_eio_window_material_blind(
        "! <WindowMaterial:Blind:EquivalentLayer>,Material Name\n\
         WindowMaterial:Blind:EquivalentLayer,IGNORED,0,0,0,0,0,0,0\n",
    )
    .expect_err("an equivalent-layer header must not satisfy the ordinary Blind contract");
    assert!(matches!(
        only_equivalent_layer,
        EioError::MissingWindowMaterialBlindHeader
    ));

    let row_without_header = parse_eio_window_material_blind(&format!(
        "! <WindowMaterial:Blind:EquivalentLayer>,Material Name\n{SOURCE_ROW}\n"
    ))
    .expect_err("a Blind row without its specialized header must fail");
    assert!(matches!(
        &row_without_header,
        EioError::InvalidWindowMaterialBlind { line: 2, .. }
    ));
    if let EioError::InvalidWindowMaterialBlind { reason, .. } = row_without_header {
        assert_eq!(
            reason,
            "row appears without the exact WindowMaterial:Blind header"
        );
    }

    let duplicate = parse_eio_window_material_blind(&format!(
        "{WINDOW_MATERIAL_BLIND_HEADER}\n{SOURCE_ROW}\n{WINDOW_MATERIAL_BLIND_HEADER}\n"
    ))
    .expect_err("a repeated exact header must fail");
    assert!(matches!(
        &duplicate,
        EioError::DuplicateWindowMaterialBlindHeader { line: 3, .. }
    ));
    if let EioError::DuplicateWindowMaterialBlindHeader { text, .. } = duplicate {
        assert_eq!(text, WINDOW_MATERIAL_BLIND_HEADER);
    }
}

#[test]
fn blind_parser_rejects_rows_before_header_and_wrong_token_counts() {
    let row_before_header =
        parse_eio_window_material_blind(&format!("{SOURCE_ROW}\n{WINDOW_MATERIAL_BLIND_HEADER}\n"))
            .expect_err("a row before the source header must fail");
    assert!(matches!(
        &row_before_header,
        EioError::InvalidWindowMaterialBlind { line: 1, .. }
    ));
    if let EioError::InvalidWindowMaterialBlind { reason, .. } = row_before_header {
        assert_eq!(
            reason,
            "row appears before the exact WindowMaterial:Blind header"
        );
    }

    for row in [
        "WindowMaterial:Blind,BLIND,0.02,0.02,0.00025,45,0,0.2",
        "WindowMaterial:Blind,BLIND,0.02,0.02,0.00025,45,0,0.2,0.05,EXTRA",
    ] {
        let error =
            parse_eio_window_material_blind(&format!("{WINDOW_MATERIAL_BLIND_HEADER}\n{row}\n"))
                .expect_err("a Blind row with a non-source token count must fail");
        assert!(matches!(
            error,
            EioError::InvalidWindowMaterialBlind { line: 2, .. }
        ));
    }
}

#[test]
fn blind_parser_rejects_blank_names_and_every_invalid_numeric_field() {
    let missing_name = parse_eio_window_material_blind(&format!(
        "{WINDOW_MATERIAL_BLIND_HEADER}\n\
         WindowMaterial:Blind,,0.02,0.02,0.00025,45,0,0.2,0.05\n"
    ))
    .expect_err("a blank Blind material name must fail");
    assert!(matches!(
        &missing_name,
        EioError::InvalidWindowMaterialBlind { line: 2, .. }
    ));
    if let EioError::InvalidWindowMaterialBlind { reason, .. } = missing_name {
        assert_eq!(reason, "missing Material Name");
    }

    const FIELD_NAMES: [&str; 7] = [
        "Slat Width {m}",
        "Slat Separation {m}",
        "Slat Thickness {m}",
        "Slat Angle {deg}",
        "Slat Beam Solar Transmittance",
        "Slat Beam Solar Front Reflectance",
        "Blind To Glass Distance {m}",
    ];
    for (index, field_name) in (2..=8).zip(FIELD_NAMES) {
        for invalid in ["not-a-number", "NaN", "inf", "-inf"] {
            let mut fields = [
                "WindowMaterial:Blind",
                "BLIND",
                "0.02",
                "0.02",
                "0.00025",
                "45",
                "0",
                "0.2",
                "0.05",
            ];
            fields[index] = invalid;
            let error = parse_eio_window_material_blind(&format!(
                "{WINDOW_MATERIAL_BLIND_HEADER}\n{}\n",
                fields.join(",")
            ))
            .expect_err("invalid or non-finite Blind values must fail");
            assert!(matches!(
                &error,
                EioError::InvalidWindowMaterialBlind { line: 2, .. }
            ));
            if let EioError::InvalidWindowMaterialBlind { reason, .. } = error {
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
fn blind_parser_accepts_exact_header_without_rows() -> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_eio_window_material_blind(&format!(
        "{WINDOW_MATERIAL_BLIND_HEADER}\nProgram Version,EnergyPlus\n"
    ))?;
    assert!(rows.is_empty());
    Ok(())
}
