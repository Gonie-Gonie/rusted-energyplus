use super::{EioError, EioWindowMaterialDrapeEquivalentLayer, required_field};

/// Exact EnergyPlus 26.1 EIO header for specialized equivalent-layer drape rows.
///
/// The empty column after `Back Side Beam-Diffuse Solar Transmittance` and the
/// apparent front/back beam-beam columns are source-faithful. EnergyPlus 26.1
/// emits only 12 comma-separated fields in each data row, not the 14 described
/// by this header.
pub const WINDOW_MATERIAL_DRAPE_EQUIVALENT_LAYER_HEADER: &str = "! <WindowMaterial:Drape:EquivalentLayer>, Material Name, Front Side Beam-Beam Solar Transmittance, Back Side Beam-Beam Solar Transmittance, Front Side Beam-Diffuse Solar Transmittance, Back Side Beam-Diffuse Solar Transmittance, , Front Side Beam-Diffuse Solar Reflectance, Back Side Beam-Diffuse Solar Reflectance, Infrared Transmittance, Front Side Infrared Emissivity, Back Side Infrared Emissivity, Width of Pleated Fabric, Length of Pleated Fabric";

/// Parses specialized `WindowMaterial:Drape:EquivalentLayer` EIO rows.
///
/// Rows remain in emission order and repeated material names are preserved
/// because EnergyPlus emits one row per equivalent-layer construction-layer
/// occurrence. The parser follows the 12-field data shape rather than trying to
/// align fields against the malformed 14-field header.
pub fn parse_eio_window_material_drape_equivalent_layer(
    contents: &str,
) -> Result<Vec<EioWindowMaterialDrapeEquivalentLayer>, EioError> {
    const FIELD_COUNT: usize = 12;
    const ROW_LABEL: &str = "WindowMaterial:Drape:EquivalentLayer,";

    let mut drape_rows = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with(ROW_LABEL) {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() != FIELD_COUNT {
            return Err(EioError::InvalidWindowMaterialDrapeEquivalentLayer {
                line: line_number,
                text: line.to_string(),
                reason: format!(
                    "expected exactly 11 data fields after the row label ({FIELD_COUNT} comma-separated fields total), found {} data fields",
                    fields.len().saturating_sub(1)
                ),
            });
        }

        let material_name =
            required_drape_equivalent_layer_field(&fields, 1, line_number, line, "Material Name")?
                .to_ascii_uppercase();
        let parse_number = |index, field| {
            parse_drape_equivalent_layer_f64_field(&fields, index, line_number, line, field)
        };

        drape_rows.push(EioWindowMaterialDrapeEquivalentLayer {
            material_name,
            beam_beam_solar_transmittance: parse_number(
                2,
                "Shared Front/Back Beam-Beam Solar Transmittance",
            )?,
            front_beam_diffuse_solar_transmittance: parse_number(
                3,
                "Front Side Beam-Diffuse Solar Transmittance",
            )?,
            back_beam_diffuse_solar_transmittance: parse_number(
                4,
                "Back Side Beam-Diffuse Solar Transmittance",
            )?,
            front_beam_diffuse_solar_reflectance: parse_number(
                5,
                "Front Side Beam-Diffuse Solar Reflectance",
            )?,
            back_beam_diffuse_solar_reflectance: parse_number(
                6,
                "Back Side Beam-Diffuse Solar Reflectance",
            )?,
            infrared_transmittance: parse_number(7, "Infrared Transmittance")?,
            front_infrared_emissivity: parse_number(8, "Front Side Infrared Emissivity")?,
            back_infrared_emissivity: parse_number(9, "Back Side Infrared Emissivity")?,
            pleated_width_m: parse_number(10, "Width of Pleated Fabric")?,
            pleated_length_m: parse_number(11, "Length of Pleated Fabric")?,
        });
    }

    if drape_rows.is_empty() {
        return Err(EioError::MissingWindowMaterialDrapeEquivalentLayer);
    }

    Ok(drape_rows)
}

fn required_drape_equivalent_layer_field<'a>(
    fields: &'a [&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<&'a str, EioError> {
    let value = required_field(fields, index);
    if value.is_empty() {
        Err(EioError::InvalidWindowMaterialDrapeEquivalentLayer {
            line,
            text: text.to_string(),
            reason: format!("missing {field}"),
        })
    } else {
        Ok(value)
    }
}

fn parse_drape_equivalent_layer_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    let value = required_field(fields, index)
        .parse::<f64>()
        .map_err(
            |_error| EioError::InvalidWindowMaterialDrapeEquivalentLayer {
                line,
                text: text.to_string(),
                reason: format!("invalid {field}"),
            },
        )?;
    if !value.is_finite() {
        return Err(EioError::InvalidWindowMaterialDrapeEquivalentLayer {
            line,
            text: text.to_string(),
            reason: format!("{field} must be finite"),
        });
    }
    Ok(value)
}
