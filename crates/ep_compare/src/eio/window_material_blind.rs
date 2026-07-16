use super::{EioError, EioWindowMaterialBlind, required_field};

/// Exact EnergyPlus 26.1 EIO header for specialized ordinary blind rows.
pub const WINDOW_MATERIAL_BLIND_HEADER: &str = "! <WindowMaterial:Blind>,Material Name,Slat Width {m},Slat Separation {m},Slat Thickness {m},Slat Angle {deg},Slat Beam Solar Transmittance,Slat Beam Solar Front Reflectance,Blind To Glass Distance {m}";

/// Parses specialized `WindowMaterial:Blind` EIO rows.
///
/// The exact nine-token source header must occur once. Data rows likewise
/// contain exactly nine comma-separated tokens: the row label, material name,
/// and seven numeric values. Rows stay in emission order and repeated material
/// names remain separate construction-layer occurrences. An exact header with
/// no rows is valid because EnergyPlus can emit the specialized table header
/// without a successfully reported ordinary-window Blind layer.
pub fn parse_eio_window_material_blind(
    contents: &str,
) -> Result<Vec<EioWindowMaterialBlind>, EioError> {
    const FIELD_COUNT: usize = 9;
    const HEADER_MARKER: &str = "! <WindowMaterial:Blind>";
    const ROW_LABEL: &str = "WindowMaterial:Blind,";

    let mut header_line = None;
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        if !line.trim_start().starts_with(HEADER_MARKER) {
            continue;
        }
        if line != WINDOW_MATERIAL_BLIND_HEADER {
            return Err(EioError::InvalidWindowMaterialBlindHeader {
                line: line_number,
                text: line.to_string(),
                reason: "header must exactly match the EnergyPlus 26.1 source literal".to_string(),
            });
        }
        if header_line.replace(line_number).is_some() {
            return Err(EioError::DuplicateWindowMaterialBlindHeader {
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
            return Err(EioError::InvalidWindowMaterialBlind {
                line: line_index + 1,
                text: line.to_string(),
                reason: "row appears without the exact WindowMaterial:Blind header".to_string(),
            });
        }
        return Err(EioError::MissingWindowMaterialBlindHeader);
    };

    let mut blind_rows = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with(ROW_LABEL) {
            continue;
        }
        if line_number <= header_line {
            return Err(EioError::InvalidWindowMaterialBlind {
                line: line_number,
                text: line.to_string(),
                reason: "row appears before the exact WindowMaterial:Blind header".to_string(),
            });
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() != FIELD_COUNT {
            return Err(EioError::InvalidWindowMaterialBlind {
                line: line_number,
                text: line.to_string(),
                reason: format!(
                    "expected exactly 8 data fields after the row label ({FIELD_COUNT} comma-separated fields total), found {} data fields",
                    fields.len().saturating_sub(1)
                ),
            });
        }

        let material_name = required_blind_field(&fields, 1, line_number, line, "Material Name")?
            .to_ascii_uppercase();
        let parse_number =
            |index, field| parse_blind_f64_field(&fields, index, line_number, line, field);
        blind_rows.push(EioWindowMaterialBlind {
            material_name,
            slat_width_m: parse_number(2, "Slat Width {m}")?,
            slat_separation_m: parse_number(3, "Slat Separation {m}")?,
            slat_thickness_m: parse_number(4, "Slat Thickness {m}")?,
            slat_angle_deg: parse_number(5, "Slat Angle {deg}")?,
            slat_beam_solar_transmittance: parse_number(6, "Slat Beam Solar Transmittance")?,
            slat_beam_solar_front_reflectance: parse_number(
                7,
                "Slat Beam Solar Front Reflectance",
            )?,
            blind_to_glass_distance_m: parse_number(8, "Blind To Glass Distance {m}")?,
        });
    }

    Ok(blind_rows)
}

fn required_blind_field<'a>(
    fields: &'a [&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<&'a str, EioError> {
    let value = required_field(fields, index);
    if value.is_empty() {
        Err(EioError::InvalidWindowMaterialBlind {
            line,
            text: text.to_string(),
            reason: format!("missing {field}"),
        })
    } else {
        Ok(value)
    }
}

fn parse_blind_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    let value = required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidWindowMaterialBlind {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })?;
    if !value.is_finite() {
        return Err(EioError::InvalidWindowMaterialBlind {
            line,
            text: text.to_string(),
            reason: format!("{field} must be finite"),
        });
    }
    Ok(value)
}
