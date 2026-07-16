use crate::{
    EioError, WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER,
    parse_eio_window_material_shade_equivalent_layer,
};

#[test]
fn parses_equivalent_layer_shade_rows_in_order_and_preserves_repeats()
-> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER,
        "! <WindowMaterial:Shade:EquivalentLayer>, Material Name, Front Side Beam-Beam Solar Transmittance, Back Side Beam-Beam Solar Transmittance, Front Side Beam-Diffuse Solar Transmittance, Back Side Beam-Diffuse Solar Transmittance, Front Side Beam-Diffuse Solar Reflectance, Back Side Beam-Diffuse Solar Reflectance, Infrared Transmittance, Front Side Infrared Emissivity, Back Side Infrared Emissivity"
    );
    let rows = parse_eio_window_material_shade_equivalent_layer(&format!(
        "{WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER}\n\
         Program Version,EnergyPlus\n\
         WindowMaterial:Shade:EquivalentLayer, reused eql shade ,1.2346E-005,1.2346E-005,0.1235,0.2346,0.3457,0.4568,3.4568E-005,0.7654,0.6543\n\
         WindowMaterial:Shade,IGNORED,0,0,0,0,0,0\n\
         WindowMaterial:Shade:EquivalentLayer, defaulted shade ,0,0,0.1111,0.1222,0.2333,0.2444,5.0000E-002,0.91,0.91\n\
         WindowMaterial:Shade:EquivalentLayer, reused eql shade ,1.2346E-005,1.2346E-005,0.1235,0.2346,0.3457,0.4568,3.4568E-005,0.7654,0.6543\n\
         WindowMaterial:Shade:EquivalentLayerExtra,IGNORED,0,0,0,0,0,0,0,0,0\n\
         windowmaterial:shade:equivalentlayer,IGNORED,0,0,0,0,0,0,0,0,0\n"
    ))?;

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].material_name, "REUSED EQL SHADE");
    assert_eq!(rows[0].front_beam_beam_solar_transmittance, 0.000012346);
    assert_eq!(rows[0].back_beam_beam_solar_transmittance, 0.000012346);
    assert_eq!(rows[0].front_beam_diffuse_solar_transmittance, 0.1235);
    assert_eq!(rows[0].back_beam_diffuse_solar_transmittance, 0.2346);
    assert_eq!(rows[0].front_beam_diffuse_solar_reflectance, 0.3457);
    assert_eq!(rows[0].back_beam_diffuse_solar_reflectance, 0.4568);
    assert_eq!(rows[0].infrared_transmittance, 0.000034568);
    assert_eq!(rows[0].front_infrared_emissivity, 0.7654);
    assert_eq!(rows[0].back_infrared_emissivity, 0.6543);
    assert_eq!(rows[1].material_name, "DEFAULTED SHADE");
    assert_eq!(rows[1].front_beam_beam_solar_transmittance, 0.0);
    assert_eq!(rows[1].infrared_transmittance, 0.05);
    assert_eq!(rows[2], rows[0]);

    Ok(())
}

#[test]
fn equivalent_layer_shade_parser_requires_exact_field_count()
-> Result<(), Box<dyn std::error::Error>> {
    for row in [
        "WindowMaterial:Shade:EquivalentLayer,SHADE,0,0,0.1,0.1,0.2,0.2,0.05,0.9\n",
        "WindowMaterial:Shade:EquivalentLayer,SHADE,0,0,0.1,0.1,0.2,0.2,0.05,0.9,0.9,EXTRA\n",
    ] {
        let error = parser_error(row)?;
        let EioError::InvalidWindowMaterialShadeEquivalentLayer { line, text, reason } = error
        else {
            return Err("expected InvalidWindowMaterialShadeEquivalentLayer".into());
        };
        assert_eq!(line, 1);
        assert_eq!(text, row.trim_end());
        assert!(reason.contains("expected exactly 10 data fields after the row label"));
    }

    Ok(())
}

#[test]
fn equivalent_layer_shade_parser_rejects_blank_name() -> Result<(), Box<dyn std::error::Error>> {
    let error = parser_error(
        "Program Version,EnergyPlus\nWindowMaterial:Shade:EquivalentLayer,  ,0,0,0.1,0.1,0.2,0.2,0.05,0.9,0.9\n",
    )?;
    let EioError::InvalidWindowMaterialShadeEquivalentLayer { line, reason, .. } = error else {
        return Err("expected InvalidWindowMaterialShadeEquivalentLayer".into());
    };
    assert_eq!(line, 2);
    assert_eq!(reason, "missing Material Name");

    Ok(())
}

#[test]
fn equivalent_layer_shade_parser_rejects_invalid_and_nonfinite_numbers()
-> Result<(), Box<dyn std::error::Error>> {
    const FIELD_NAMES: [&str; 9] = [
        "Front Side Beam-Beam Solar Transmittance",
        "Back Side Beam-Beam Solar Transmittance",
        "Front Side Beam-Diffuse Solar Transmittance",
        "Back Side Beam-Diffuse Solar Transmittance",
        "Front Side Beam-Diffuse Solar Reflectance",
        "Back Side Beam-Diffuse Solar Reflectance",
        "Infrared Transmittance",
        "Front Side Infrared Emissivity",
        "Back Side Infrared Emissivity",
    ];
    for (index, field_name) in (2..=10).zip(FIELD_NAMES) {
        for invalid in ["", "not-a-number", "NaN", "inf", "-inf"] {
            let mut fields = [
                "WindowMaterial:Shade:EquivalentLayer",
                "SHADE",
                "0",
                "0",
                "0.1",
                "0.1",
                "0.2",
                "0.2",
                "0.05",
                "0.9",
                "0.9",
            ];
            fields[index] = invalid;
            let error = parser_error(&format!("ignored line\n{}\n", fields.join(",")))?;
            let EioError::InvalidWindowMaterialShadeEquivalentLayer { line, reason, .. } = error
            else {
                return Err("expected InvalidWindowMaterialShadeEquivalentLayer".into());
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
fn equivalent_layer_shade_parser_reports_missing_rows() {
    assert!(matches!(
        parse_eio_window_material_shade_equivalent_layer(&format!(
            "{WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER}\nProgram Version,EnergyPlus\n"
        )),
        Err(EioError::MissingWindowMaterialShadeEquivalentLayer)
    ));
}

fn parser_error(contents: &str) -> Result<EioError, Box<dyn std::error::Error>> {
    match parse_eio_window_material_shade_equivalent_layer(contents) {
        Err(error) => Ok(error),
        Ok(rows) => Err(format!("expected parser error, parsed {rows:?}").into()),
    }
}
