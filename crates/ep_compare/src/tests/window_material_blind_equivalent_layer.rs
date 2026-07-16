use crate::{
    EioError, WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER,
    parse_eio_window_material_blind_equivalent_layer,
};

const DEFAULT_ROW: &str = "WindowMaterial:Blind:EquivalentLayer, default eql blind ,Horizontal,0.02000,0.01200,0.00150,45.00000,0.00000,0.00000,0.20000,0.25000,0.00000,0.00000,0.00000,0.00000,0.00000,0.00000";
const EXPLICIT_ROW: &str = "WindowMaterial:Blind:EquivalentLayer, reused eql blind ,Vertical,0.02346,0.01765,0.00123,-32.12346,0.12346,0.11235,0.23457,0.22346,0.34568,0.15679,0.14568,0.03457,0.76543,0.75432";

#[test]
fn parses_exact_malformed_source_shape_and_concatenated_records()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER,
        "! <WindowMaterial:Blind:EquivalentLayer>, Material Name, Slat Orientation, Slat Width, Slat Separation, Slat Crown, Slat Angle, Front Side Slate Beam-Diffuse Solar Transmittance, Back Side Slate Beam-Diffuse Solar Transmittance, Front Side Slate Beam-Diffuse Solar Reflectance, Back Side Slate Beam-Diffuse Solar Reflectance, Slat Diffuse-Diffuse Solar Transmittance, Front Side Slat Diffuse-Diffuse Solar Reflectance, Back Side Slat Diffuse-Diffuse Solar Reflectance, Infrared Transmittance, Front Side Infrared Emissivity, Back Side Infrared Emissivity, Slat Angle Control"
    );
    assert_eq!(
        WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER
            .split(',')
            .count(),
        18
    );
    assert_eq!(DEFAULT_ROW.split(',').count(), 17);

    let contents = format!(
        "{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\n\
         Program Version,EnergyPlus\n\
          {DEFAULT_ROW} WindowMaterial:Gap:EquivalentLayer,GAP,Air,0.012700,Sealed\n\
          {EXPLICIT_ROW} Construction:WindowEquivalentLayer,FIRST,1,2,1.0,0.5,0.4\n\
          {EXPLICIT_ROW} WindowConstruction,SECOND,2,2,Rough,1.0\n\
          {EXPLICIT_ROW} {DEFAULT_ROW}\n\
          {DEFAULT_ROW}! <Following Header>,Field\n"
    );
    let rows = parse_eio_window_material_blind_equivalent_layer(&contents)?;

    assert_eq!(rows.len(), 6);
    assert_eq!(rows[0].material_name, "DEFAULT EQL BLIND");
    assert_eq!(rows[0].slat_orientation, "Horizontal");
    assert_eq!(rows[0].slat_width_m, 0.02);
    assert_eq!(rows[0].slat_separation_m, 0.012);
    assert_eq!(rows[0].slat_crown_m, 0.0015);
    assert_eq!(rows[0].slat_angle_deg, 45.0);
    assert_eq!(rows[0].front_beam_diffuse_solar_transmittance, 0.0);
    assert_eq!(rows[0].back_beam_diffuse_solar_transmittance, 0.0);
    assert_eq!(rows[0].front_beam_diffuse_solar_reflectance, 0.2);
    assert_eq!(rows[0].back_beam_diffuse_solar_reflectance, 0.25);
    assert_eq!(rows[0].diffuse_diffuse_solar_transmittance, 0.0);
    assert_eq!(rows[0].front_diffuse_diffuse_solar_reflectance, 0.0);
    assert_eq!(rows[0].back_diffuse_diffuse_solar_reflectance, 0.0);
    assert_eq!(rows[0].infrared_transmittance, 0.0);
    assert_eq!(rows[0].front_infrared_emissivity, 0.0);
    assert_eq!(rows[0].back_infrared_emissivity, 0.0);

    assert_eq!(rows[1].material_name, "REUSED EQL BLIND");
    assert_eq!(rows[1].slat_orientation, "Vertical");
    assert_eq!(rows[1].slat_angle_deg, -32.12346);
    assert_eq!(rows[1].front_beam_diffuse_solar_transmittance, 0.12346);
    assert_eq!(rows[1].back_beam_diffuse_solar_transmittance, 0.11235);
    assert_eq!(rows[1].front_beam_diffuse_solar_reflectance, 0.23457);
    assert_eq!(rows[1].back_beam_diffuse_solar_reflectance, 0.22346);
    assert_eq!(rows[1].diffuse_diffuse_solar_transmittance, 0.34568);
    assert_eq!(rows[1].front_diffuse_diffuse_solar_reflectance, 0.15679);
    assert_eq!(rows[1].back_diffuse_diffuse_solar_reflectance, 0.14568);
    assert_eq!(rows[1].infrared_transmittance, 0.03457);
    assert_eq!(rows[1].front_infrared_emissivity, 0.76543);
    assert_eq!(rows[1].back_infrared_emissivity, 0.75432);
    assert_eq!(rows[2], rows[1]);
    assert_eq!(rows[3], rows[1]);
    assert_eq!(rows[4], rows[0]);
    assert_eq!(rows[5], rows[0]);
    Ok(())
}

#[test]
fn requires_one_exact_header_and_reports_rows_without_it() {
    let missing = parse_eio_window_material_blind_equivalent_layer(DEFAULT_ROW)
        .expect_err("a data record without its exact header must fail");
    assert!(matches!(
        missing,
        EioError::InvalidWindowMaterialBlindEquivalentLayer { line: 1, .. }
    ));

    assert!(matches!(
        parse_eio_window_material_blind_equivalent_layer("Program Version,EnergyPlus\n"),
        Err(EioError::MissingWindowMaterialBlindEquivalentLayerHeader)
    ));

    let duplicate = parse_eio_window_material_blind_equivalent_layer(&format!(
        "{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\n\
         {DEFAULT_ROW}\n\
         {WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\n"
    ))
    .expect_err("a repeated exact header must fail");
    assert!(matches!(
        duplicate,
        EioError::DuplicateWindowMaterialBlindEquivalentLayerHeader { line: 3, .. }
    ));

    for malformed in [
        WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER.replace("Slate", "Slat"),
        format!("{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}, Extra"),
        format!(" {WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}"),
    ] {
        let error = parse_eio_window_material_blind_equivalent_layer(&format!(
            "{malformed}\n{DEFAULT_ROW}\n"
        ))
        .expect_err("a non-literal equivalent-layer Blind header must fail");
        assert!(matches!(
            error,
            EioError::InvalidWindowMaterialBlindEquivalentLayerHeader { line: 1, .. }
        ));
    }
}

#[test]
fn rejects_records_before_header_wrong_token_counts_and_unknown_suffixes() {
    let before = parse_eio_window_material_blind_equivalent_layer(&format!(
        "{DEFAULT_ROW}\n{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\n"
    ))
    .expect_err("a record before the exact header must fail");
    assert!(matches!(
        before,
        EioError::InvalidWindowMaterialBlindEquivalentLayer { line: 1, .. }
    ));

    let too_few = DEFAULT_ROW.rsplit_once(',').expect("test row has fields").0;
    let too_many = format!("{DEFAULT_ROW},EXTRA");
    for row in [too_few, too_many.as_str()] {
        let error = parse_eio_window_material_blind_equivalent_layer(&format!(
            "{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\n{row}\n"
        ))
        .expect_err("a non-source token count must fail");
        assert!(matches!(
            error,
            EioError::InvalidWindowMaterialBlindEquivalentLayer { line: 2, .. }
        ));
    }

    for suffix in [
        " UNRECOGNIZED",
        " UnrecognizedRecord,EXTRA",
        " WindowMaterial:UnknownWithoutComma",
        "! <NotACompleteHeader",
    ] {
        let error = parse_eio_window_material_blind_equivalent_layer(&format!(
            "{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\n{DEFAULT_ROW}{suffix}\n"
        ))
        .expect_err("arbitrary same-line suffix text must fail closed");
        assert!(matches!(
            error,
            EioError::InvalidWindowMaterialBlindEquivalentLayer { line: 2, .. }
        ));
    }
}

#[test]
fn rejects_blank_strings_and_every_invalid_numeric_field() -> Result<(), Box<dyn std::error::Error>>
{
    let base = DEFAULT_ROW.split(',').collect::<Vec<_>>();
    for (index, expected_reason) in [
        (1, "missing Material Name"),
        (2, "missing Slat Orientation"),
    ] {
        let mut fields = base.clone();
        fields[index] = "";
        let error = parse_eio_window_material_blind_equivalent_layer(&format!(
            "{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\n{}\n",
            fields.join(",")
        ))
        .expect_err("blank required text must fail");
        let EioError::InvalidWindowMaterialBlindEquivalentLayer { reason, .. } = error else {
            return Err("expected InvalidWindowMaterialBlindEquivalentLayer".into());
        };
        assert_eq!(reason, expected_reason);
    }

    const NUMERIC_FIELD_NAMES: [&str; 14] = [
        "Slat Width",
        "Slat Separation",
        "Slat Crown",
        "Slat Angle",
        "Front Side Slat Beam-Diffuse Solar Transmittance",
        "Back Side Slat Beam-Diffuse Solar Transmittance",
        "Front Side Slat Beam-Diffuse Solar Reflectance",
        "Back Side Slat Beam-Diffuse Solar Reflectance",
        "Slat Diffuse-Diffuse Solar Transmittance",
        "Front Side Slat Diffuse-Diffuse Solar Reflectance",
        "Back Side Slat Diffuse-Diffuse Solar Reflectance",
        "Infrared Transmittance",
        "Front Side Infrared Emissivity",
        "Back Side Infrared Emissivity",
    ];
    for (index, field_name) in (3..=16).zip(NUMERIC_FIELD_NAMES) {
        for invalid in ["", "not-a-number", "NaN", "inf", "-inf"] {
            let mut fields = base.clone();
            fields[index] = invalid;
            let error = parse_eio_window_material_blind_equivalent_layer(&format!(
                "{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\n{}\n",
                fields.join(",")
            ))
            .expect_err("invalid or non-finite numeric values must fail");
            let EioError::InvalidWindowMaterialBlindEquivalentLayer { reason, .. } = error else {
                return Err("expected InvalidWindowMaterialBlindEquivalentLayer".into());
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
fn accepts_exact_header_without_data_records() -> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_eio_window_material_blind_equivalent_layer(&format!(
        "{WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER}\r\nProgram Version,EnergyPlus\r\n"
    ))?;
    assert!(rows.is_empty());
    Ok(())
}
