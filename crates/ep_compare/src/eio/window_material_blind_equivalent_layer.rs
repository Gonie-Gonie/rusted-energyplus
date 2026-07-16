use super::{EioError, EioWindowMaterialBlindEquivalentLayer, required_field};

/// Exact EnergyPlus 26.1 EIO header for specialized equivalent-layer blind
/// rows.
///
/// The source header contains 18 comma-separated tokens, including four
/// `Slate` typos and a final `Slat Angle Control` label. The corresponding
/// source data record contains only 17 tokens because it omits the angle
/// control value.
pub const WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER: &str = "! <WindowMaterial:Blind:EquivalentLayer>, Material Name, Slat Orientation, Slat Width, Slat Separation, Slat Crown, Slat Angle, Front Side Slate Beam-Diffuse Solar Transmittance, Back Side Slate Beam-Diffuse Solar Transmittance, Front Side Slate Beam-Diffuse Solar Reflectance, Back Side Slate Beam-Diffuse Solar Reflectance, Slat Diffuse-Diffuse Solar Transmittance, Front Side Slat Diffuse-Diffuse Solar Reflectance, Back Side Slat Diffuse-Diffuse Solar Reflectance, Infrared Transmittance, Front Side Infrared Emissivity, Back Side Infrared Emissivity, Slat Angle Control";

const FIELD_COUNT: usize = 17;
const HEADER_MARKER: &str = "! <WindowMaterial:Blind:EquivalentLayer>";
const ROW_LABEL: &str = "WindowMaterial:Blind:EquivalentLayer,";

/// Parses specialized `WindowMaterial:Blind:EquivalentLayer` EIO records.
///
/// EnergyPlus 26.1's source format omits the newline after each equivalent-
/// layer blind record. A following material or construction record can
/// therefore occupy the same physical line. This parser scans exact record
/// markers, preserves emission order and duplicates, and delimits the final
/// numeric field from recognized same-line EIO records instead of assuming
/// one record per line.
pub fn parse_eio_window_material_blind_equivalent_layer(
    contents: &str,
) -> Result<Vec<EioWindowMaterialBlindEquivalentLayer>, EioError> {
    let mut header_byte = None;
    let mut line_start = 0;
    for (line_index, raw_line) in contents.split_inclusive('\n').enumerate() {
        let line_number = line_index + 1;
        let line = raw_line.strip_suffix('\n').unwrap_or(raw_line);
        let line = line.strip_suffix('\r').unwrap_or(line);
        if line.trim_start().starts_with(HEADER_MARKER) {
            if line != WINDOW_MATERIAL_BLIND_EQUIVALENT_LAYER_HEADER {
                return Err(EioError::InvalidWindowMaterialBlindEquivalentLayerHeader {
                    line: line_number,
                    text: line.to_string(),
                    reason: "header must exactly match the EnergyPlus 26.1 source literal"
                        .to_string(),
                });
            }
            if header_byte.replace(line_start).is_some() {
                return Err(
                    EioError::DuplicateWindowMaterialBlindEquivalentLayerHeader {
                        line: line_number,
                        text: line.to_string(),
                    },
                );
            }
        }
        line_start += raw_line.len();
    }

    let row_starts = blind_equivalent_layer_row_starts(contents);
    let Some(header_byte) = header_byte else {
        if let Some(&start) = row_starts.first() {
            let (line, text) = source_line(contents, start);
            return Err(EioError::InvalidWindowMaterialBlindEquivalentLayer {
                line,
                text: text.to_string(),
                reason: "row appears without the exact WindowMaterial:Blind:EquivalentLayer header"
                    .to_string(),
            });
        }
        return Err(EioError::MissingWindowMaterialBlindEquivalentLayerHeader);
    };

    let mut rows = Vec::with_capacity(row_starts.len());
    for start in row_starts {
        let (line_number, source_text) = source_line(contents, start);
        if start < header_byte {
            return Err(EioError::InvalidWindowMaterialBlindEquivalentLayer {
                line: line_number,
                text: source_text.to_string(),
                reason: "row appears before the exact WindowMaterial:Blind:EquivalentLayer header"
                    .to_string(),
            });
        }

        let record =
            blind_equivalent_layer_record(source_text, start - source_line_start(contents, start));
        let fields = record.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() != FIELD_COUNT {
            return Err(EioError::InvalidWindowMaterialBlindEquivalentLayer {
                line: line_number,
                text: source_text.to_string(),
                reason: format!(
                    "expected exactly 16 data fields after the row label ({FIELD_COUNT} comma-separated fields total), found {} data fields",
                    fields.len().saturating_sub(1)
                ),
            });
        }

        let material_name = required_blind_equivalent_layer_field(
            &fields,
            1,
            line_number,
            source_text,
            "Material Name",
        )?
        .to_ascii_uppercase();
        let slat_orientation = required_blind_equivalent_layer_field(
            &fields,
            2,
            line_number,
            source_text,
            "Slat Orientation",
        )?
        .to_string();
        let parse_number = |index, field| {
            parse_blind_equivalent_layer_f64_field(&fields, index, line_number, source_text, field)
        };

        rows.push(EioWindowMaterialBlindEquivalentLayer {
            material_name,
            slat_orientation,
            slat_width_m: parse_number(3, "Slat Width")?,
            slat_separation_m: parse_number(4, "Slat Separation")?,
            slat_crown_m: parse_number(5, "Slat Crown")?,
            slat_angle_deg: parse_number(6, "Slat Angle")?,
            front_beam_diffuse_solar_transmittance: parse_number(
                7,
                "Front Side Slat Beam-Diffuse Solar Transmittance",
            )?,
            back_beam_diffuse_solar_transmittance: parse_number(
                8,
                "Back Side Slat Beam-Diffuse Solar Transmittance",
            )?,
            front_beam_diffuse_solar_reflectance: parse_number(
                9,
                "Front Side Slat Beam-Diffuse Solar Reflectance",
            )?,
            back_beam_diffuse_solar_reflectance: parse_number(
                10,
                "Back Side Slat Beam-Diffuse Solar Reflectance",
            )?,
            diffuse_diffuse_solar_transmittance: parse_number(
                11,
                "Slat Diffuse-Diffuse Solar Transmittance",
            )?,
            front_diffuse_diffuse_solar_reflectance: parse_number(
                12,
                "Front Side Slat Diffuse-Diffuse Solar Reflectance",
            )?,
            back_diffuse_diffuse_solar_reflectance: parse_number(
                13,
                "Back Side Slat Diffuse-Diffuse Solar Reflectance",
            )?,
            infrared_transmittance: parse_number(14, "Infrared Transmittance")?,
            front_infrared_emissivity: parse_number(15, "Front Side Infrared Emissivity")?,
            back_infrared_emissivity: parse_number(16, "Back Side Infrared Emissivity")?,
        });
    }

    Ok(rows)
}

fn blind_equivalent_layer_row_starts(contents: &str) -> Vec<usize> {
    contents
        .match_indices(ROW_LABEL)
        .filter_map(|(start, _label)| {
            let is_record_boundary = start == 0
                || contents[..start]
                    .chars()
                    .next_back()
                    .is_some_and(char::is_whitespace);
            is_record_boundary.then_some(start)
        })
        .collect()
}

fn source_line_start(contents: &str, offset: usize) -> usize {
    contents[..offset].rfind('\n').map_or(0, |index| index + 1)
}

fn source_line(contents: &str, offset: usize) -> (usize, &str) {
    let start = source_line_start(contents, offset);
    let end = contents[offset..]
        .find(['\r', '\n'])
        .map_or(contents.len(), |relative| offset + relative);
    let line = contents[..start]
        .bytes()
        .filter(|byte| *byte == b'\n')
        .count()
        + 1;
    (line, &contents[start..end])
}

fn blind_equivalent_layer_record(source_line: &str, row_offset: usize) -> &str {
    let candidate = &source_line[row_offset..];
    let last_field_start = candidate
        .match_indices(',')
        .nth(FIELD_COUNT - 2)
        .map_or(candidate.len(), |(index, _comma)| index + 1);
    let suffix = &candidate[last_field_start..];
    let suffix_offset = concatenated_record_offset(suffix)
        .map_or(candidate.len(), |index| last_field_start + index);
    candidate[..suffix_offset].trim_end()
}

fn concatenated_record_offset(suffix: &str) -> Option<usize> {
    let family_record_offsets = [" WindowMaterial:", " WindowConstruction:", " Construction:"]
        .into_iter()
        .filter_map(|marker| {
            suffix.match_indices(marker).find_map(|(index, _marker)| {
                suffix[index..]
                    .find(',')
                    .is_some_and(|comma| {
                        !suffix[index + 1..index + comma].contains(char::is_whitespace)
                    })
                    .then_some(index)
            })
        });
    let window_construction_offset = suffix.find(" WindowConstruction,");
    let header_offset = suffix
        .match_indices("! <")
        .find_map(|(index, _marker)| suffix[index..].contains(">,").then_some(index));
    family_record_offsets
        .chain(window_construction_offset)
        .chain(header_offset)
        .min()
}

fn required_blind_equivalent_layer_field<'a>(
    fields: &'a [&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<&'a str, EioError> {
    let value = required_field(fields, index);
    if value.is_empty() {
        Err(EioError::InvalidWindowMaterialBlindEquivalentLayer {
            line,
            text: text.to_string(),
            reason: format!("missing {field}"),
        })
    } else {
        Ok(value)
    }
}

fn parse_blind_equivalent_layer_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    let value = required_field(fields, index)
        .parse::<f64>()
        .map_err(
            |_error| EioError::InvalidWindowMaterialBlindEquivalentLayer {
                line,
                text: text.to_string(),
                reason: format!("invalid {field}"),
            },
        )?;
    if !value.is_finite() {
        return Err(EioError::InvalidWindowMaterialBlindEquivalentLayer {
            line,
            text: text.to_string(),
            reason: format!("{field} must be finite"),
        });
    }
    Ok(value)
}
