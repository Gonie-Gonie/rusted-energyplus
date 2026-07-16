use super::{EioError, EioWindowMaterialScreenEquivalentLayer, required_field};

/// Exact EnergyPlus 26.1 EIO header for specialized equivalent-layer screen
/// rows.
///
/// The nine-token header describes shared screen inputs, while each source
/// data row expands the symmetric solar and infrared values into front/back
/// slots and therefore contains twelve tokens.
pub const WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER: &str = "! <WindowMaterial:Screen:EquivalentLayer>, Material Name, Screen Beam-Beam Solar Transmittance, Screen Beam-Diffuse Solar Transmittance, Screen Beam-Diffuse Solar Reflectance, Screen Infrared Transmittance, Screen Infrared Emissivity, Screen Wire Spacing, Screen Wire Diameter";

/// Parses specialized `WindowMaterial:Screen:EquivalentLayer` EIO rows.
///
/// The exact nine-token source header must occur once. Data rows contain the
/// row label, material name, eight `{:.4R}` solar/infrared values, and two
/// `{:.5R}` wire-geometry values. Rows remain in emission order and duplicate
/// material names are preserved. An exact header with no rows is valid because
/// EnergyPlus gates the header on definitions and window constructions rather
/// than on a matching construction-layer occurrence.
pub fn parse_eio_window_material_screen_equivalent_layer(
    contents: &str,
) -> Result<Vec<EioWindowMaterialScreenEquivalentLayer>, EioError> {
    const FIELD_COUNT: usize = 12;
    const HEADER_MARKER: &str = "! <WindowMaterial:Screen:EquivalentLayer>";
    const ROW_LABEL: &str = "WindowMaterial:Screen:EquivalentLayer,";

    let mut header_line = None;
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        if !line.trim_start().starts_with(HEADER_MARKER) {
            continue;
        }
        if line != WINDOW_MATERIAL_SCREEN_EQUIVALENT_LAYER_HEADER {
            return Err(EioError::InvalidWindowMaterialScreenEquivalentLayerHeader {
                line: line_number,
                text: line.to_string(),
                reason: "header must exactly match the EnergyPlus 26.1 source literal".to_string(),
            });
        }
        if header_line.replace(line_number).is_some() {
            return Err(
                EioError::DuplicateWindowMaterialScreenEquivalentLayerHeader {
                    line: line_number,
                    text: line.to_string(),
                },
            );
        }
    }
    let Some(header_line) = header_line else {
        if let Some((line_index, line)) = contents
            .lines()
            .enumerate()
            .find(|(_line_index, line)| line.trim().starts_with(ROW_LABEL))
        {
            return Err(EioError::InvalidWindowMaterialScreenEquivalentLayer {
                line: line_index + 1,
                text: line.to_string(),
                reason:
                    "row appears without the exact WindowMaterial:Screen:EquivalentLayer header"
                        .to_string(),
            });
        }
        return Err(EioError::MissingWindowMaterialScreenEquivalentLayerHeader);
    };

    let mut rows = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with(ROW_LABEL) {
            continue;
        }
        if line_number <= header_line {
            return Err(EioError::InvalidWindowMaterialScreenEquivalentLayer {
                line: line_number,
                text: line.to_string(),
                reason: "row appears before the exact WindowMaterial:Screen:EquivalentLayer header"
                    .to_string(),
            });
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() != FIELD_COUNT {
            return Err(EioError::InvalidWindowMaterialScreenEquivalentLayer {
                line: line_number,
                text: line.to_string(),
                reason: format!(
                    "expected exactly 11 data fields after the row label ({FIELD_COUNT} comma-separated fields total), found {} data fields",
                    fields.len().saturating_sub(1)
                ),
            });
        }

        let material_name =
            required_screen_equivalent_layer_field(&fields, 1, line_number, line, "Material Name")?
                .to_ascii_uppercase();
        let parse_number = |index, field| {
            parse_screen_equivalent_layer_f64_field(&fields, index, line_number, line, field)
        };
        rows.push(EioWindowMaterialScreenEquivalentLayer {
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
            wire_spacing_m: parse_number(10, "Screen Wire Spacing")?,
            wire_diameter_m: parse_number(11, "Screen Wire Diameter")?,
        });
    }

    Ok(rows)
}

fn required_screen_equivalent_layer_field<'a>(
    fields: &'a [&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<&'a str, EioError> {
    let value = required_field(fields, index);
    if value.is_empty() {
        Err(EioError::InvalidWindowMaterialScreenEquivalentLayer {
            line,
            text: text.to_string(),
            reason: format!("missing {field}"),
        })
    } else {
        Ok(value)
    }
}

fn parse_screen_equivalent_layer_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    let value = required_field(fields, index)
        .parse::<f64>()
        .map_err(
            |_error| EioError::InvalidWindowMaterialScreenEquivalentLayer {
                line,
                text: text.to_string(),
                reason: format!("invalid {field}"),
            },
        )?;
    if !value.is_finite() {
        return Err(EioError::InvalidWindowMaterialScreenEquivalentLayer {
            line,
            text: text.to_string(),
            reason: format!("{field} must be finite"),
        });
    }
    Ok(value)
}
