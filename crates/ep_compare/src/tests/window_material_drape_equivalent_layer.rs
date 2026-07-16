use crate::{
    EioError, WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER,
    parse_eio_window_material_drape_equivalent_layer,
};

#[test]
fn parses_source_malformed_drape_rows_in_order_and_preserves_repeats()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER,
        "! <WindowMaterial:Drape:EquivalentLayer>, Material Name, Front Side Beam-Beam Solar Transmittance, Back Side Beam-Beam Solar Transmittance, Front Side Beam-Diffuse Solar Transmittance, Back Side Beam-Diffuse Solar Transmittance, , Front Side Beam-Diffuse Solar Reflectance, Back Side Beam-Diffuse Solar Reflectance, Infrared Transmittance, Front Side Infrared Emissivity, Back Side Infrared Emissivity, Width of Pleated Fabric, Length of Pleated Fabric"
    );
    assert_eq!(
        WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER
            .split(',')
            .count(),
        14
    );

    let rows = parse_eio_window_material_drape_equivalent_layer(&format!(
        "{WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER}\n\
         Program Version,EnergyPlus\n\
         WindowMaterial:Drape:EquivalentLayer, reused eql drape ,1.2346E-005,0.1235,0.2346,0.3457,0.4568,3.4568E-005,0.7654,0.6543,1.23456E-002,2.34567E-002\n\
         WindowMaterial:Shade:EquivalentLayer,IGNORED,0,0,0,0,0,0,0,0,0\n\
         WindowMaterial:Drape:EquivalentLayer, defaulted drape ,0,0.1111,0.1222,0.2333,0.2444,5.0000E-002,0.87,0.87,0,0\n\
         WindowMaterial:Drape:EquivalentLayer, reused eql drape ,1.2346E-005,0.1235,0.2346,0.3457,0.4568,3.4568E-005,0.7654,0.6543,1.23456E-002,2.34567E-002\n\
         WindowMaterial:Drape:EquivalentLayerExtra,IGNORED,0,0,0,0,0,0,0,0,0,0\n\
         windowmaterial:drape:equivalentlayer,IGNORED,0,0,0,0,0,0,0,0,0,0\n"
    ))?;

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].material_name, "REUSED EQL DRAPE");
    assert_eq!(rows[0].beam_beam_solar_transmittance, 0.000012346);
    assert_eq!(rows[0].front_beam_diffuse_solar_transmittance, 0.1235);
    assert_eq!(rows[0].back_beam_diffuse_solar_transmittance, 0.2346);
    assert_eq!(rows[0].front_beam_diffuse_solar_reflectance, 0.3457);
    assert_eq!(rows[0].back_beam_diffuse_solar_reflectance, 0.4568);
    assert_eq!(rows[0].infrared_transmittance, 0.000034568);
    assert_eq!(rows[0].front_infrared_emissivity, 0.7654);
    assert_eq!(rows[0].back_infrared_emissivity, 0.6543);
    assert_eq!(rows[0].pleated_width_m, 0.0123456);
    assert_eq!(rows[0].pleated_length_m, 0.0234567);
    assert_eq!(rows[1].material_name, "DEFAULTED DRAPE");
    assert_eq!(rows[1].beam_beam_solar_transmittance, 0.0);
    assert_eq!(rows[1].infrared_transmittance, 0.05);
    assert_eq!(rows[1].pleated_width_m, 0.0);
    assert_eq!(rows[2], rows[0]);

    Ok(())
}

#[test]
fn drape_parser_requires_exact_twelve_field_data_shape() -> Result<(), Box<dyn std::error::Error>> {
    for row in [
        "WindowMaterial:Drape:EquivalentLayer,DRAPE,0,0.1,0.1,0.2,0.2,0.05,0.9,0.9,0\n",
        "WindowMaterial:Drape:EquivalentLayer,DRAPE,0,0.1,0.1,0.2,0.2,0.05,0.9,0.9,0,0,EXTRA\n",
    ] {
        let error = parser_error(row)?;
        let EioError::InvalidWindowMaterialDrapeEquivalentLayer { line, text, reason } = error
        else {
            return Err("expected InvalidWindowMaterialDrapeEquivalentLayer".into());
        };
        assert_eq!(line, 1);
        assert_eq!(text, row.trim_end());
        assert!(reason.contains("expected exactly 11 data fields after the row label"));
    }

    Ok(())
}

#[test]
fn drape_parser_rejects_blank_name() -> Result<(), Box<dyn std::error::Error>> {
    let error = parser_error(
        "Program Version,EnergyPlus\nWindowMaterial:Drape:EquivalentLayer,  ,0,0.1,0.1,0.2,0.2,0.05,0.9,0.9,0,0\n",
    )?;
    let EioError::InvalidWindowMaterialDrapeEquivalentLayer { line, reason, .. } = error else {
        return Err("expected InvalidWindowMaterialDrapeEquivalentLayer".into());
    };
    assert_eq!(line, 2);
    assert_eq!(reason, "missing Material Name");
    Ok(())
}

#[test]
fn drape_parser_rejects_invalid_and_nonfinite_numbers() -> Result<(), Box<dyn std::error::Error>> {
    const FIELD_NAMES: [&str; 10] = [
        "Shared Front/Back Beam-Beam Solar Transmittance",
        "Front Side Beam-Diffuse Solar Transmittance",
        "Back Side Beam-Diffuse Solar Transmittance",
        "Front Side Beam-Diffuse Solar Reflectance",
        "Back Side Beam-Diffuse Solar Reflectance",
        "Infrared Transmittance",
        "Front Side Infrared Emissivity",
        "Back Side Infrared Emissivity",
        "Width of Pleated Fabric",
        "Length of Pleated Fabric",
    ];
    for (index, field_name) in (2..=11).zip(FIELD_NAMES) {
        for invalid in ["", "not-a-number", "NaN", "inf", "-inf"] {
            let mut fields = [
                "WindowMaterial:Drape:EquivalentLayer",
                "DRAPE",
                "0",
                "0.1",
                "0.1",
                "0.2",
                "0.2",
                "0.05",
                "0.9",
                "0.9",
                "0",
                "0",
            ];
            fields[index] = invalid;
            let error = parser_error(&format!("ignored line\n{}\n", fields.join(",")))?;
            let EioError::InvalidWindowMaterialDrapeEquivalentLayer { line, reason, .. } = error
            else {
                return Err("expected InvalidWindowMaterialDrapeEquivalentLayer".into());
            };
            assert_eq!(line, 2);
            let expected_reason = if invalid.is_empty() || invalid == "not-a-number" {
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
fn drape_parser_reports_missing_rows() {
    assert!(matches!(
        parse_eio_window_material_drape_equivalent_layer(&format!(
            "{WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER}\nProgram Version,EnergyPlus\n"
        )),
        Err(EioError::MissingWindowMaterialDrapeEquivalentLayer)
    ));
}

fn parser_error(contents: &str) -> Result<EioError, Box<dyn std::error::Error>> {
    match parse_eio_window_material_drape_equivalent_layer(contents) {
        Err(error) => Ok(error),
        Ok(rows) => Err(format!("expected parser error, parsed {rows:?}").into()),
    }
}
