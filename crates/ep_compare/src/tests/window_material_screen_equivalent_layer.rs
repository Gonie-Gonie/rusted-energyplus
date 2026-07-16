use crate::{
    EioError, WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER,
    parse_eio_window_material_screen_equivalent_layer,
};

const AUTO_ROW: &str = " WindowMaterial:Screen:EquivalentLayer, default auto screen ,-99999.0000,0.1111,0.1111,0.2222,0.2222,2.0000E-002,0.9300,0.9300,0.00000,0.00000";
const EXPLICIT_ROW: &str = " WindowMaterial:Screen:EquivalentLayer, reused eql screen ,0.6400,0.1235,0.1235,0.2346,0.2346,3.4568E-005,0.7654,0.7654,1.23456E-002,2.34567E-003";

#[test]
fn parses_exact_source_header_auto_sentinel_and_duplicate_aware_rows()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER,
        "! <WindowMaterial:Screen:EquivalentLayer>, Material Name, Screen Beam-Beam Solar Transmittance, Screen Beam-Diffuse Solar Transmittance, Screen Beam-Diffuse Solar Reflectance, Screen Infrared Transmittance, Screen Infrared Emissivity, Screen Wire Spacing, Screen Wire Diameter"
    );
    assert_eq!(
        WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER
            .split(',')
            .count(),
        9
    );

    let rows = parse_eio_window_material_screen_equivalent_layer(&format!(
        "{WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}\n\
         Program Version,EnergyPlus\n\
         {AUTO_ROW}\n\
         WindowMaterial:Screen,IGNORED,0,0,0,0,0,0,0,0,0,0\n\
         {EXPLICIT_ROW}\n\
         WindowMaterial:Screen:EquivalentLayerExtra,IGNORED,0,0,0,0,0,0,0,0,0,0\n\
         windowmaterial:screen:equivalentlayer,IGNORED,0,0,0,0,0,0,0,0,0,0\n\
         {EXPLICIT_ROW}\n"
    ))?;

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].material_name, "DEFAULT AUTO SCREEN");
    assert_eq!(rows[0].beam_beam_solar_transmittance, -99_999.0);
    assert_eq!(rows[0].front_beam_diffuse_solar_transmittance, 0.1111);
    assert_eq!(rows[0].back_beam_diffuse_solar_transmittance, 0.1111);
    assert_eq!(rows[0].front_beam_diffuse_solar_reflectance, 0.2222);
    assert_eq!(rows[0].back_beam_diffuse_solar_reflectance, 0.2222);
    assert_eq!(rows[0].infrared_transmittance, 0.02);
    assert_eq!(rows[0].front_infrared_emissivity, 0.93);
    assert_eq!(rows[0].back_infrared_emissivity, 0.93);
    assert_eq!(rows[0].wire_spacing_m, 0.0);
    assert_eq!(rows[0].wire_diameter_m, 0.0);
    assert_eq!(rows[1].material_name, "REUSED EQL SCREEN");
    assert_eq!(rows[1].beam_beam_solar_transmittance, 0.64);
    assert_eq!(rows[1].wire_spacing_m, 0.0123456);
    assert_eq!(rows[1].wire_diameter_m, 0.00234567);
    assert_eq!(rows[2], rows[1]);
    Ok(())
}

#[test]
fn screen_equivalent_layer_parser_requires_one_exact_header() {
    let missing = parse_eio_window_material_screen_equivalent_layer(EXPLICIT_ROW)
        .expect_err("a row without its specialized header must fail");
    assert!(matches!(
        &missing,
        EioError::InvalidWindowMaterialScreenEquivalentLayer { line: 1, .. }
    ));

    let duplicate = parse_eio_window_material_screen_equivalent_layer(&format!(
        "{WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}\n\
         {EXPLICIT_ROW}\n\
         {WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}\n"
    ))
    .expect_err("a repeated exact header must fail");
    assert!(matches!(
        duplicate,
        EioError::DuplicateWindowMaterialScreenEquivalentLayerHeader { line: 3, .. }
    ));

    for malformed in [
        WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER.replace("Wire Spacing", "WireSpacing"),
        format!("{WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}, Extra"),
        format!(" {WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}"),
    ] {
        let error = parse_eio_window_material_screen_equivalent_layer(&format!(
            "{malformed}\n{EXPLICIT_ROW}\n"
        ))
        .expect_err("a non-literal Screen:EQL header must fail");
        assert!(matches!(
            error,
            EioError::InvalidWindowMaterialScreenEquivalentLayerHeader { line: 1, .. }
        ));
    }
}

#[test]
fn screen_equivalent_layer_parser_rejects_rows_before_header_and_wrong_token_counts() {
    let before = parse_eio_window_material_screen_equivalent_layer(&format!(
        "{EXPLICIT_ROW}\n{WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}\n"
    ))
    .expect_err("a row before the source header must fail");
    assert!(matches!(
        before,
        EioError::InvalidWindowMaterialScreenEquivalentLayer { line: 1, .. }
    ));

    for row in [
        "WindowMaterial:Screen:EquivalentLayer,SCREEN,0.64,0.1,0.1,0.2,0.2,0.02,0.93,0.93,0.01",
        "WindowMaterial:Screen:EquivalentLayer,SCREEN,0.64,0.1,0.1,0.2,0.2,0.02,0.93,0.93,0.01,0.002,EXTRA",
    ] {
        let error = parse_eio_window_material_screen_equivalent_layer(&format!(
            "{WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}\n{row}\n"
        ))
        .expect_err("a non-source token count must fail");
        assert!(matches!(
            error,
            EioError::InvalidWindowMaterialScreenEquivalentLayer { line: 2, .. }
        ));
    }
}

#[test]
fn screen_equivalent_layer_parser_rejects_blank_names_and_invalid_numbers()
-> Result<(), Box<dyn std::error::Error>> {
    let missing_name = parse_eio_window_material_screen_equivalent_layer(&format!(
        "{WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}\n\
         WindowMaterial:Screen:EquivalentLayer,,0.64,0.1,0.1,0.2,0.2,0.02,0.93,0.93,0.01,0.002\n"
    ))
    .expect_err("a blank material name must fail");
    assert!(matches!(
        &missing_name,
        EioError::InvalidWindowMaterialScreenEquivalentLayer { line: 2, .. }
    ));
    if let EioError::InvalidWindowMaterialScreenEquivalentLayer { reason, .. } = missing_name {
        assert_eq!(reason, "missing Material Name");
    }

    const FIELD_NAMES: [&str; 10] = [
        "Shared Front/Back Beam-Beam Solar Transmittance",
        "Front Side Beam-Diffuse Solar Transmittance",
        "Back Side Beam-Diffuse Solar Transmittance",
        "Front Side Beam-Diffuse Solar Reflectance",
        "Back Side Beam-Diffuse Solar Reflectance",
        "Infrared Transmittance",
        "Front Side Infrared Emissivity",
        "Back Side Infrared Emissivity",
        "Screen Wire Spacing",
        "Screen Wire Diameter",
    ];
    for (index, field_name) in (2..=11).zip(FIELD_NAMES) {
        for invalid in ["", "not-a-number", "NaN", "inf", "-inf"] {
            let mut fields = [
                "WindowMaterial:Screen:EquivalentLayer",
                "SCREEN",
                "0.64",
                "0.1",
                "0.1",
                "0.2",
                "0.2",
                "0.02",
                "0.93",
                "0.93",
                "0.01",
                "0.002",
            ];
            fields[index] = invalid;
            let error = parse_eio_window_material_screen_equivalent_layer(&format!(
                "{WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}\n{}\n",
                fields.join(",")
            ))
            .expect_err("invalid or non-finite values must fail");
            let EioError::InvalidWindowMaterialScreenEquivalentLayer { reason, .. } = error else {
                return Err("expected InvalidWindowMaterialScreenEquivalentLayer".into());
            };
            let expected = if invalid.is_empty() || invalid == "not-a-number" {
                format!("invalid {field_name}")
            } else {
                format!("{field_name} must be finite")
            };
            assert_eq!(reason, expected);
        }
    }
    Ok(())
}

#[test]
fn screen_equivalent_layer_parser_accepts_exact_header_without_rows()
-> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_eio_window_material_screen_equivalent_layer(&format!(
        "{WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER}\nProgram Version,EnergyPlus\n"
    ))?;
    assert!(rows.is_empty());
    Ok(())
}
