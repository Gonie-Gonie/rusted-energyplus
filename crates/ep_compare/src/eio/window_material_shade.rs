use super::{EioError, EioWindowMaterialShade, required_field};

/// Exact EnergyPlus 26.1 EIO header for specialized window-shade rows.
pub const WINDOW_MATERIAL_SHADE_HEADER: &str = "! <WindowMaterial:Shade>,Material Name,Thickness {m},Conductivity {W/m-K},Thermal Absorptance,Transmittance,Visible Transmittance,Shade Reflectance";

/// Parses specialized `WindowMaterial:Shade` rows from EnergyPlus EIO contents.
///
/// Rows remain in emission order and repeated material names are preserved
/// because EnergyPlus emits one row per successfully reported ordinary-window
/// shade-layer occurrence.
pub fn parse_eio_window_material_shade(
    contents: &str,
) -> Result<Vec<EioWindowMaterialShade>, EioError> {
    const FIELD_COUNT: usize = 8;
    const ROW_LABEL: &str = "WindowMaterial:Shade,";

    let mut shade_rows = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with(ROW_LABEL) {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() != FIELD_COUNT {
            return Err(EioError::InvalidWindowMaterialShade {
                line: line_number,
                text: line.to_string(),
                reason: format!(
                    "expected exactly 7 data fields after the row label ({FIELD_COUNT} comma-separated fields total), found {} data fields",
                    fields.len().saturating_sub(1)
                ),
            });
        }

        let material_name = required_shade_field(&fields, 1, line_number, line, "Material Name")?
            .to_ascii_uppercase();
        shade_rows.push(EioWindowMaterialShade {
            material_name,
            thickness_m: parse_shade_f64_field(&fields, 2, line_number, line, "Thickness {m}")?,
            conductivity_w_per_m_k: parse_shade_f64_field(
                &fields,
                3,
                line_number,
                line,
                "Conductivity {W/m-K}",
            )?,
            thermal_absorptance: parse_shade_f64_field(
                &fields,
                4,
                line_number,
                line,
                "Thermal Absorptance",
            )?,
            solar_transmittance: parse_shade_f64_field(
                &fields,
                5,
                line_number,
                line,
                "Transmittance",
            )?,
            visible_transmittance: parse_shade_f64_field(
                &fields,
                6,
                line_number,
                line,
                "Visible Transmittance",
            )?,
            solar_reflectance: parse_shade_f64_field(
                &fields,
                7,
                line_number,
                line,
                "Shade Reflectance",
            )?,
        });
    }

    if shade_rows.is_empty() {
        return Err(EioError::MissingWindowMaterialShade);
    }

    Ok(shade_rows)
}

fn required_shade_field<'a>(
    fields: &'a [&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<&'a str, EioError> {
    let value = required_field(fields, index);
    if value.is_empty() {
        Err(EioError::InvalidWindowMaterialShade {
            line,
            text: text.to_string(),
            reason: format!("missing {field}"),
        })
    } else {
        Ok(value)
    }
}

fn parse_shade_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    let value = required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidWindowMaterialShade {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })?;
    if !value.is_finite() {
        return Err(EioError::InvalidWindowMaterialShade {
            line,
            text: text.to_string(),
            reason: format!("{field} must be finite"),
        });
    }
    Ok(value)
}
