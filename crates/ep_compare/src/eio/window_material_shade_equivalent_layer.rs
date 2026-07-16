use super::{EioError, EioWindowMaterialShadeEquivalentLayer, required_field};

/// Exact EnergyPlus 26.1 EIO header for specialized equivalent-layer shade rows.
pub const WINDOW_MATERIAL_SHADE_EQUIVALENT_LAYER_HEADER: &str = "! <WindowMaterial:Shade:EquivalentLayer>, Material Name, Front Side Beam-Beam Solar Transmittance, Back Side Beam-Beam Solar Transmittance, Front Side Beam-Diffuse Solar Transmittance, Back Side Beam-Diffuse Solar Transmittance, Front Side Beam-Diffuse Solar Reflectance, Back Side Beam-Diffuse Solar Reflectance, Infrared Transmittance, Front Side Infrared Emissivity, Back Side Infrared Emissivity";

/// Parses specialized `WindowMaterial:Shade:EquivalentLayer` EIO rows.
///
/// Rows remain in emission order and repeated material names are preserved
/// because EnergyPlus emits one row per equivalent-layer construction-layer
/// occurrence.
pub fn parse_eio_window_material_shade_equivalent_layer(
    contents: &str,
) -> Result<Vec<EioWindowMaterialShadeEquivalentLayer>, EioError> {
    const FIELD_COUNT: usize = 11;
    const ROW_LABEL: &str = "WindowMaterial:Shade:EquivalentLayer,";

    let mut shade_rows = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with(ROW_LABEL) {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() != FIELD_COUNT {
            return Err(EioError::InvalidWindowMaterialShadeEquivalentLayer {
                line: line_number,
                text: line.to_string(),
                reason: format!(
                    "expected exactly 10 data fields after the row label ({FIELD_COUNT} comma-separated fields total), found {} data fields",
                    fields.len().saturating_sub(1)
                ),
            });
        }

        let material_name =
            required_shade_equivalent_layer_field(&fields, 1, line_number, line, "Material Name")?
                .to_ascii_uppercase();
        let parse_number = |index, field| {
            parse_shade_equivalent_layer_f64_field(&fields, index, line_number, line, field)
        };

        shade_rows.push(EioWindowMaterialShadeEquivalentLayer {
            material_name,
            front_beam_beam_solar_transmittance: parse_number(
                2,
                "Front Side Beam-Beam Solar Transmittance",
            )?,
            back_beam_beam_solar_transmittance: parse_number(
                3,
                "Back Side Beam-Beam Solar Transmittance",
            )?,
            front_beam_diffuse_solar_transmittance: parse_number(
                4,
                "Front Side Beam-Diffuse Solar Transmittance",
            )?,
            back_beam_diffuse_solar_transmittance: parse_number(
                5,
                "Back Side Beam-Diffuse Solar Transmittance",
            )?,
            front_beam_diffuse_solar_reflectance: parse_number(
                6,
                "Front Side Beam-Diffuse Solar Reflectance",
            )?,
            back_beam_diffuse_solar_reflectance: parse_number(
                7,
                "Back Side Beam-Diffuse Solar Reflectance",
            )?,
            infrared_transmittance: parse_number(8, "Infrared Transmittance")?,
            front_infrared_emissivity: parse_number(9, "Front Side Infrared Emissivity")?,
            back_infrared_emissivity: parse_number(10, "Back Side Infrared Emissivity")?,
        });
    }

    if shade_rows.is_empty() {
        return Err(EioError::MissingWindowMaterialShadeEquivalentLayer);
    }

    Ok(shade_rows)
}

fn required_shade_equivalent_layer_field<'a>(
    fields: &'a [&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<&'a str, EioError> {
    let value = required_field(fields, index);
    if value.is_empty() {
        Err(EioError::InvalidWindowMaterialShadeEquivalentLayer {
            line,
            text: text.to_string(),
            reason: format!("missing {field}"),
        })
    } else {
        Ok(value)
    }
}

fn parse_shade_equivalent_layer_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    let value = required_field(fields, index)
        .parse::<f64>()
        .map_err(
            |_error| EioError::InvalidWindowMaterialShadeEquivalentLayer {
                line,
                text: text.to_string(),
                reason: format!("invalid {field}"),
            },
        )?;
    if !value.is_finite() {
        return Err(EioError::InvalidWindowMaterialShadeEquivalentLayer {
            line,
            text: text.to_string(),
            reason: format!("{field} must be finite"),
        });
    }
    Ok(value)
}
