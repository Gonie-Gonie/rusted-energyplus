use super::{EioError, EioWindowMaterialScreen, required_field};

/// Exact EnergyPlus 26.1 EIO header for specialized ordinary screen rows.
///
/// The missing space in `Screen To GlassDistance` is source-authentic.
pub const WINDOW_MATERIAL_SCREEN_HEADER: &str = "! <WindowMaterial:Screen>,Material Name,Thickness {m},Conductivity {W/m-K},Thermal Absorptance,Transmittance,Reflectance,Visible Reflectance,Diffuse Reflectance,Diffuse Visible Reflectance,Screen Material Diameter To Spacing Ratio,Screen To GlassDistance {m}";

/// Parses specialized `WindowMaterial:Screen` EIO rows.
///
/// The exact 12-token source header must occur once. Data rows likewise contain
/// exactly 12 comma-separated tokens: the row label, material name, and ten
/// numeric values. EnergyPlus emits thickness with `{:.5R}` and the other nine
/// values with `{:.3R}`; parsing retains those already-rounded values. Rows stay
/// in emission order and repeated material names remain separate occurrences.
/// An exact header with no rows is valid because EnergyPlus can report the
/// specialized table header even when no window construction uses a Screen.
pub fn parse_eio_window_material_screen(
    contents: &str,
) -> Result<Vec<EioWindowMaterialScreen>, EioError> {
    const FIELD_COUNT: usize = 12;
    const HEADER_MARKER: &str = "! <WindowMaterial:Screen>";
    const ROW_LABEL: &str = "WindowMaterial:Screen,";

    let mut header_line = None;
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        if !line.trim_start().starts_with(HEADER_MARKER) {
            continue;
        }
        if line != WINDOW_MATERIAL_SCREEN_HEADER {
            return Err(EioError::InvalidWindowMaterialScreenHeader {
                line: line_number,
                text: line.to_string(),
                reason: "header must exactly match the EnergyPlus 26.1 source literal".to_string(),
            });
        }
        if header_line.replace(line_number).is_some() {
            return Err(EioError::DuplicateWindowMaterialScreenHeader {
                line: line_number,
                text: line.to_string(),
            });
        }
    }
    let Some(header_line) = header_line else {
        if let Some((line_index, line)) = contents
            .lines()
            .enumerate()
            .find(|(_line_index, line)| line.trim().starts_with(ROW_LABEL))
        {
            return Err(EioError::InvalidWindowMaterialScreen {
                line: line_index + 1,
                text: line.to_string(),
                reason: "row appears without the exact WindowMaterial:Screen header".to_string(),
            });
        }
        return Err(EioError::MissingWindowMaterialScreenHeader);
    };

    let mut screen_rows = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with(ROW_LABEL) {
            continue;
        }
        if line_number <= header_line {
            return Err(EioError::InvalidWindowMaterialScreen {
                line: line_number,
                text: line.to_string(),
                reason: "row appears before the exact WindowMaterial:Screen header".to_string(),
            });
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() != FIELD_COUNT {
            return Err(EioError::InvalidWindowMaterialScreen {
                line: line_number,
                text: line.to_string(),
                reason: format!(
                    "expected exactly 11 data fields after the row label ({FIELD_COUNT} comma-separated fields total), found {} data fields",
                    fields.len().saturating_sub(1)
                ),
            });
        }

        let material_name = required_screen_field(&fields, 1, line_number, line, "Material Name")?
            .to_ascii_uppercase();
        let parse_number =
            |index, field| parse_screen_f64_field(&fields, index, line_number, line, field);
        screen_rows.push(EioWindowMaterialScreen {
            material_name,
            thickness_m: parse_number(2, "Thickness {m}")?,
            conductivity_w_per_m_k: parse_number(3, "Conductivity {W/m-K}")?,
            thermal_absorptance: parse_number(4, "Thermal Absorptance")?,
            solar_transmittance: parse_number(5, "Transmittance")?,
            solar_reflectance: parse_number(6, "Reflectance")?,
            visible_reflectance: parse_number(7, "Visible Reflectance")?,
            diffuse_solar_reflectance: parse_number(8, "Diffuse Reflectance")?,
            diffuse_visible_reflectance: parse_number(9, "Diffuse Visible Reflectance")?,
            diameter_to_spacing_ratio: parse_number(
                10,
                "Screen Material Diameter To Spacing Ratio",
            )?,
            screen_to_glass_distance_m: parse_number(11, "Screen To GlassDistance {m}")?,
        });
    }

    Ok(screen_rows)
}

fn required_screen_field<'a>(
    fields: &'a [&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<&'a str, EioError> {
    let value = required_field(fields, index);
    if value.is_empty() {
        Err(EioError::InvalidWindowMaterialScreen {
            line,
            text: text.to_string(),
            reason: format!("missing {field}"),
        })
    } else {
        Ok(value)
    }
}

fn parse_screen_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    let value = required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidWindowMaterialScreen {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })?;
    if !value.is_finite() {
        return Err(EioError::InvalidWindowMaterialScreen {
            line,
            text: text.to_string(),
            reason: format!("{field} must be finite"),
        });
    }
    Ok(value)
}
