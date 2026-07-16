use std::path::Path;

use super::{EioError, EioMaterialDetails, required_field};

/// Loads generic material-detail rows from an EnergyPlus EIO file.
pub fn load_eio_material_details(
    path: impl AsRef<Path>,
) -> Result<Vec<EioMaterialDetails>, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_material_details(&contents)
}

/// Parses generic `Material Details` rows from EnergyPlus EIO contents.
///
/// Rows are returned in emission order and repeated material names are
/// preserved. Zero-valued numeric fields are valid because EnergyPlus emits
/// fixed zero columns for some material groups, including window gas mixtures.
pub fn parse_eio_material_details(contents: &str) -> Result<Vec<EioMaterialDetails>, EioError> {
    const FIELD_COUNT: usize = 11;
    const ROW_LABEL: &str = "Material Details,";

    let mut detail_rows = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with(ROW_LABEL) {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() != FIELD_COUNT {
            return Err(EioError::InvalidMaterialDetails {
                line: line_number,
                text: line.to_string(),
                reason: format!(
                    "expected exactly 10 data fields after the row label ({FIELD_COUNT} comma-separated fields total), found {} data fields",
                    fields.len().saturating_sub(1)
                ),
            });
        }

        let material_name =
            required_material_details_field(&fields, 1, line_number, line, "Material Name")?
                .to_ascii_uppercase();
        let roughness =
            required_material_details_field(&fields, 3, line_number, line, "Roughness")?
                .to_string();
        let thickness_m =
            parse_material_details_f64_field(&fields, 4, line_number, line, "Thickness {m}")?;
        if thickness_m < 0.0 {
            return Err(EioError::InvalidMaterialDetails {
                line: line_number,
                text: line.to_string(),
                reason: "Thickness {m} must be nonnegative".to_string(),
            });
        }

        detail_rows.push(EioMaterialDetails {
            material_name,
            thermal_resistance_m2_k_per_w: parse_material_details_f64_field(
                &fields,
                2,
                line_number,
                line,
                "ThermalResistance {m2-K/w}",
            )?,
            roughness,
            thickness_m,
            conductivity_w_per_m_k: parse_material_details_f64_field(
                &fields,
                5,
                line_number,
                line,
                "Conductivity {w/m-K}",
            )?,
            density_kg_per_m3: parse_material_details_f64_field(
                &fields,
                6,
                line_number,
                line,
                "Density {kg/m3}",
            )?,
            specific_heat_j_per_kg_k: parse_material_details_f64_field(
                &fields,
                7,
                line_number,
                line,
                "Specific Heat {J/kg-K}",
            )?,
            thermal_absorptance: parse_material_details_f64_field(
                &fields,
                8,
                line_number,
                line,
                "Absorptance:Thermal",
            )?,
            solar_absorptance: parse_material_details_f64_field(
                &fields,
                9,
                line_number,
                line,
                "Absorptance:Solar",
            )?,
            visible_absorptance: parse_material_details_f64_field(
                &fields,
                10,
                line_number,
                line,
                "Absorptance:Visible",
            )?,
        });
    }

    if detail_rows.is_empty() {
        return Err(EioError::MissingMaterialDetails);
    }

    Ok(detail_rows)
}

fn required_material_details_field<'a>(
    fields: &'a [&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<&'a str, EioError> {
    let value = required_field(fields, index);
    if value.is_empty() {
        Err(EioError::InvalidMaterialDetails {
            line,
            text: text.to_string(),
            reason: format!("missing {field}"),
        })
    } else {
        Ok(value)
    }
}

fn parse_material_details_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    let value = required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidMaterialDetails {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })?;
    if !value.is_finite() {
        return Err(EioError::InvalidMaterialDetails {
            line,
            text: text.to_string(),
            reason: format!("{field} must be finite"),
        });
    }
    Ok(value)
}
