//! EnergyPlus EIO diagnostic table readers.

mod types;

pub use types::*;

use std::path::Path;

/// Loads the global `Surface Geometry` row from an EnergyPlus EIO file.
pub fn load_eio_surface_geometry_rules(
    path: impl AsRef<Path>,
) -> Result<EioSurfaceGeometryRules, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_surface_geometry_rules(&contents)
}

/// Loads zone geometry rows from an EnergyPlus EIO file.
pub fn load_eio_zone_geometry(path: impl AsRef<Path>) -> Result<Vec<EioZoneGeometry>, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_zone_geometry(&contents)
}

/// Loads heat-transfer surface rows from an EnergyPlus EIO file.
pub fn load_eio_heat_transfer_surfaces(
    path: impl AsRef<Path>,
) -> Result<Vec<EioHeatTransferSurface>, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_heat_transfer_surfaces(&contents)
}

/// Loads OtherEquipment nominal internal gain rows from an EnergyPlus EIO file.
pub fn load_eio_other_equipment_nominal(
    path: impl AsRef<Path>,
) -> Result<Vec<EioOtherEquipmentNominal>, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_other_equipment_nominal(&contents)
}

/// Loads construction CTF rows from an EnergyPlus EIO file.
pub fn load_eio_construction_ctf(
    path: impl AsRef<Path>,
) -> Result<Vec<EioConstructionCtf>, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_construction_ctf(&contents)
}

/// Loads construction CTF rows grouped with their ordered material layers.
pub fn load_eio_construction_material_summaries(
    path: impl AsRef<Path>,
) -> Result<Vec<EioConstructionMaterialSummary>, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_construction_material_summaries(&contents)
}

/// Loads construction CTF coefficient rows from an EnergyPlus EIO file.
pub fn load_eio_construction_ctf_coefficients(
    path: impl AsRef<Path>,
) -> Result<Vec<EioConstructionCtfCoefficient>, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_construction_ctf_coefficients(&contents)
}

/// Loads material CTF summary rows from an EnergyPlus EIO file.
pub fn load_eio_material_ctf_summary(
    path: impl AsRef<Path>,
) -> Result<Vec<EioMaterialCtfSummary>, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_material_ctf_summary(&contents)
}

/// Loads warmup environment rows from an EnergyPlus EIO file.
pub fn load_eio_warmup_environments(
    path: impl AsRef<Path>,
) -> Result<Vec<EioWarmupEnvironment>, EioError> {
    let contents = std::fs::read_to_string(path)?;
    parse_eio_warmup_environments(&contents)
}

/// Parses the unique `Surface Geometry` row from EnergyPlus EIO contents.
pub fn parse_eio_surface_geometry_rules(
    contents: &str,
) -> Result<EioSurfaceGeometryRules, EioError> {
    let mut rules = None;
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with("Surface Geometry,") {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() < 6 {
            return Err(EioError::InvalidSurfaceGeometry {
                line: line_number,
                text: line.to_string(),
                reason: format!("expected at least 6 fields, found {}", fields.len()),
            });
        }
        if rules.is_some() {
            return Err(EioError::InvalidSurfaceGeometry {
                line: line_number,
                text: line.to_string(),
                reason: "expected exactly one Surface Geometry row".to_string(),
            });
        }

        rules = Some(EioSurfaceGeometryRules {
            starting_corner: parse_surface_geometry_choice(
                &fields,
                1,
                line_number,
                line,
                "Starting Corner",
                &[
                    "UpperLeftCorner",
                    "LowerLeftCorner",
                    "LowerRightCorner",
                    "UpperRightCorner",
                ],
            )?,
            vertex_input_direction: parse_surface_geometry_choice(
                &fields,
                2,
                line_number,
                line,
                "Vertex Input Direction",
                &["Counterclockwise", "Clockwise"],
            )?,
            coordinate_system: parse_surface_geometry_choice(
                &fields,
                3,
                line_number,
                line,
                "Coordinate System",
                &["WorldCoordinateSystem", "RelativeCoordinateSystem"],
            )?,
            daylight_reference_point_coordinate_system: parse_surface_geometry_choice(
                &fields,
                4,
                line_number,
                line,
                "Daylight Reference Point Coordinate System",
                &["WorldCoordinateSystem", "RelativeCoordinateSystem"],
            )?,
            rectangular_surface_coordinate_system: parse_surface_geometry_choice(
                &fields,
                5,
                line_number,
                line,
                "Rectangular Surface Coordinate System",
                &["WorldCoordinateSystem", "RelativeToZoneOrigin"],
            )?,
        });
    }

    rules.ok_or(EioError::MissingSurfaceGeometry)
}

/// Parses `Zone Information` rows from EnergyPlus EIO contents.
pub fn parse_eio_zone_geometry(contents: &str) -> Result<Vec<EioZoneGeometry>, EioError> {
    let mut zones = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with("Zone Information,") {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() <= 26 {
            return Err(EioError::InvalidZoneInformation {
                line: line_number,
                text: line.to_string(),
                reason: format!("expected at least 27 fields, found {}", fields.len()),
            });
        }

        zones.push(EioZoneGeometry {
            zone_name: required_field(&fields, 1).to_ascii_uppercase(),
            volume_m3: parse_f64_field(&fields, 19, line_number, line, "Volume {m3}")?,
            floor_area_m2: parse_f64_field(&fields, 22, line_number, line, "Floor Area {m2}")?,
            exterior_gross_wall_area_m2: parse_f64_field(
                &fields,
                23,
                line_number,
                line,
                "Exterior Gross Wall Area {m2}",
            )?,
            surface_count: parse_usize_field(&fields, 26, line_number, line, "Number of Surfaces")?,
        });
    }

    if zones.is_empty() {
        return Err(EioError::MissingZoneInformation);
    }

    Ok(zones)
}

/// Parses `HeatTransfer Surface` rows from EnergyPlus EIO contents.
pub fn parse_eio_heat_transfer_surfaces(
    contents: &str,
) -> Result<Vec<EioHeatTransferSurface>, EioError> {
    let mut surfaces = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with("HeatTransfer Surface,") {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        const DETAILS_FIELD_COUNT: usize = 27;
        const FIRST_VERTEX_FIELD: usize = DETAILS_FIELD_COUNT;
        if fields.len() < DETAILS_FIELD_COUNT {
            return Err(EioError::InvalidHeatTransferSurface {
                line: line_number,
                text: line.to_string(),
                reason: format!(
                    "expected at least {DETAILS_FIELD_COUNT} fields, found {}",
                    fields.len()
                ),
            });
        }

        let side_count = parse_surface_usize_field(&fields, 26, line_number, line, "#Sides")?;
        if side_count == 0 {
            return Err(EioError::InvalidHeatTransferSurface {
                line: line_number,
                text: line.to_string(),
                reason: "#Sides must be greater than zero".to_string(),
            });
        }
        let expected_details_with_vertices_field_count = side_count
            .checked_mul(3)
            .and_then(|coordinate_count| DETAILS_FIELD_COUNT.checked_add(coordinate_count))
            .ok_or_else(|| EioError::InvalidHeatTransferSurface {
                line: line_number,
                text: line.to_string(),
                reason: "DetailsWithVertices field count overflow".to_string(),
            })?;
        let world_vertices = if fields.len() == DETAILS_FIELD_COUNT {
            None
        } else if fields.len() == expected_details_with_vertices_field_count {
            let mut vertices = Vec::with_capacity(side_count);
            for vertex_index in 0..side_count {
                let field_index = FIRST_VERTEX_FIELD + 3 * vertex_index;
                let vertex_number = vertex_index + 1;
                vertices.push(EioSurfaceVertex {
                    x_m: parse_surface_f64_field(
                        &fields,
                        field_index,
                        line_number,
                        line,
                        &format!("Vertex {vertex_number} X {{m}}"),
                    )?,
                    y_m: parse_surface_f64_field(
                        &fields,
                        field_index + 1,
                        line_number,
                        line,
                        &format!("Vertex {vertex_number} Y {{m}}"),
                    )?,
                    z_m: parse_surface_f64_field(
                        &fields,
                        field_index + 2,
                        line_number,
                        line,
                        &format!("Vertex {vertex_number} Z {{m}}"),
                    )?,
                });
            }
            Some(vertices)
        } else {
            return Err(EioError::InvalidHeatTransferSurface {
                line: line_number,
                text: line.to_string(),
                reason: format!(
                    "expected {DETAILS_FIELD_COUNT} fields for Details or {expected_details_with_vertices_field_count} fields for DetailsWithVertices with {side_count} sides, found {}",
                    fields.len()
                ),
            });
        };

        surfaces.push(EioHeatTransferSurface {
            surface_name: required_field(&fields, 1).to_ascii_uppercase(),
            surface_class: required_field(&fields, 2).to_string(),
            construction_name: required_field(&fields, 5).to_ascii_uppercase(),
            area_net_m2: parse_surface_f64_field(&fields, 9, line_number, line, "Area (Net) {m2}")?,
            area_gross_m2: parse_surface_f64_field(
                &fields,
                10,
                line_number,
                line,
                "Area (Gross) {m2}",
            )?,
            azimuth_deg: parse_surface_f64_field(&fields, 12, line_number, line, "Azimuth {deg}")?,
            tilt_deg: parse_surface_f64_field(&fields, 13, line_number, line, "Tilt {deg}")?,
            side_count,
            world_vertices,
        });
    }

    if surfaces.is_empty() {
        return Err(EioError::MissingHeatTransferSurface);
    }

    Ok(surfaces)
}

/// Parses `OtherEquipment Internal Gains Nominal` rows from EnergyPlus EIO contents.
pub fn parse_eio_other_equipment_nominal(
    contents: &str,
) -> Result<Vec<EioOtherEquipmentNominal>, EioError> {
    let mut equipment = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with("OtherEquipment Internal Gains Nominal,") {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() <= 12 {
            return Err(EioError::InvalidOtherEquipmentNominal {
                line: line_number,
                text: line.to_string(),
                reason: format!("expected at least 13 fields, found {}", fields.len()),
            });
        }

        equipment.push(EioOtherEquipmentNominal {
            equipment_name: required_field(&fields, 1).to_ascii_uppercase(),
            schedule_name: required_field(&fields, 2).to_ascii_uppercase(),
            zone_name: required_field(&fields, 3).to_ascii_uppercase(),
            zone_floor_area_m2: parse_other_f64_field(
                &fields,
                4,
                line_number,
                line,
                "Zone Floor Area {m2}",
            )?,
            equipment_level_w: parse_other_f64_field(
                &fields,
                6,
                line_number,
                line,
                "Equipment Level {W}",
            )?,
            equipment_per_floor_area_w_per_m2: parse_other_f64_field(
                &fields,
                7,
                line_number,
                line,
                "Equipment/Floor Area {W/m2}",
            )?,
            fraction_latent: parse_other_f64_field(
                &fields,
                9,
                line_number,
                line,
                "Fraction Latent",
            )?,
            fraction_radiant: parse_other_f64_field(
                &fields,
                10,
                line_number,
                line,
                "Fraction Radiant",
            )?,
            fraction_lost: parse_other_f64_field(&fields, 11, line_number, line, "Fraction Lost")?,
            fraction_convected: parse_other_f64_field(
                &fields,
                12,
                line_number,
                line,
                "Fraction Convected",
            )?,
        });
    }

    if equipment.is_empty() {
        return Err(EioError::MissingOtherEquipmentNominal);
    }

    Ok(equipment)
}

/// Parses `Construction CTF` rows from EnergyPlus EIO contents.
pub fn parse_eio_construction_ctf(contents: &str) -> Result<Vec<EioConstructionCtf>, EioError> {
    let mut constructions = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with("Construction CTF,") {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() <= 11 {
            return Err(EioError::InvalidConstructionCtf {
                line: line_number,
                text: line.to_string(),
                reason: format!("expected at least 12 fields, found {}", fields.len()),
            });
        }

        constructions.push(parse_construction_ctf_row(&fields, line_number, line)?);
    }

    if constructions.is_empty() {
        return Err(EioError::MissingConstructionCtf);
    }

    Ok(constructions)
}

/// Parses `Construction CTF` rows together with their outside-to-inside material summaries.
///
/// Both `Material CTF Summary` and resistance-only `Material:Air CTF Summary` rows are
/// accepted. The generic material row is also used for infrared-transparent material, so
/// callers must not infer an EnergyPlus object type from the row format alone.
pub fn parse_eio_construction_material_summaries(
    contents: &str,
) -> Result<Vec<EioConstructionMaterialSummary>, EioError> {
    type PendingSummary = (usize, String, EioConstructionMaterialSummary);

    let mut summaries = Vec::new();
    let mut pending: Option<PendingSummary> = None;
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if trimmed.starts_with("Construction CTF,") {
            if let Some(previous) = pending.take() {
                finish_construction_material_summary(previous, &mut summaries)?;
            }
            let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
            if fields.len() <= 11 {
                return Err(EioError::InvalidConstructionCtf {
                    line: line_number,
                    text: line.to_string(),
                    reason: format!("expected at least 12 fields, found {}", fields.len()),
                });
            }
            pending = Some((
                line_number,
                line.to_string(),
                EioConstructionMaterialSummary {
                    construction: parse_construction_ctf_row(&fields, line_number, line)?,
                    layers: Vec::new(),
                },
            ));
            continue;
        }

        let summary_format = if trimmed.starts_with("Material CTF Summary,") {
            Some(EioMaterialCtfSummaryFormat::Material)
        } else if trimmed.starts_with("Material:Air CTF Summary,") {
            Some(EioMaterialCtfSummaryFormat::Air)
        } else {
            None
        };
        let Some(summary_format) = summary_format else {
            continue;
        };
        let Some((_construction_line, _construction_text, current)) = pending.as_mut() else {
            return Err(EioError::InvalidConstructionMaterialSummary {
                line: line_number,
                text: line.to_string(),
                reason: "material summary appeared before any Construction CTF row".to_string(),
            });
        };
        current.layers.push(parse_construction_material_layer(
            trimmed,
            summary_format,
            line_number,
            line,
        )?);
    }

    if let Some(previous) = pending.take() {
        finish_construction_material_summary(previous, &mut summaries)?;
    }
    if summaries.is_empty() {
        return Err(EioError::MissingConstructionCtf);
    }

    Ok(summaries)
}

fn finish_construction_material_summary(
    pending: (usize, String, EioConstructionMaterialSummary),
    summaries: &mut Vec<EioConstructionMaterialSummary>,
) -> Result<(), EioError> {
    let (line, text, summary) = pending;
    if summary.construction.layer_count != summary.layers.len() {
        return Err(EioError::InvalidConstructionMaterialSummary {
            line,
            text,
            reason: format!(
                "construction {} declares {} layers but has {} material summary rows",
                summary.construction.construction_name,
                summary.construction.layer_count,
                summary.layers.len()
            ),
        });
    }
    summaries.push(summary);
    Ok(())
}

fn parse_construction_material_layer(
    trimmed: &str,
    summary_format: EioMaterialCtfSummaryFormat,
    line_number: usize,
    line: &str,
) -> Result<EioConstructionMaterialLayer, EioError> {
    let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
    let required_fields = match summary_format {
        EioMaterialCtfSummaryFormat::Material => 7,
        EioMaterialCtfSummaryFormat::Air => 3,
    };
    if fields.len() < required_fields {
        return Err(EioError::InvalidConstructionMaterialSummary {
            line: line_number,
            text: line.to_string(),
            reason: format!(
                "expected at least {required_fields} fields, found {}",
                fields.len()
            ),
        });
    }

    let parse_field = |index, field| {
        parse_construction_material_f64_field(&fields, index, line_number, line, field)
    };
    let (
        thickness_m,
        conductivity_w_per_m_k,
        density_kg_per_m3,
        specific_heat_j_per_kg_k,
        resistance_index,
    ) = match summary_format {
        EioMaterialCtfSummaryFormat::Material => (
            Some(parse_field(2, "Thickness {m}")?),
            Some(parse_field(3, "Conductivity {w/m-K}")?),
            Some(parse_field(4, "Density {kg/m3}")?),
            Some(parse_field(5, "Specific Heat {J/kg-K}")?),
            6,
        ),
        EioMaterialCtfSummaryFormat::Air => (None, None, None, None, 2),
    };
    Ok(EioConstructionMaterialLayer {
        material_name: required_field(&fields, 1).to_ascii_uppercase(),
        summary_format,
        thickness_m,
        conductivity_w_per_m_k,
        density_kg_per_m3,
        specific_heat_j_per_kg_k,
        thermal_resistance_m2_k_per_w: parse_field(resistance_index, "ThermalResistance {m2-K/w}")?,
    })
}

/// Parses `CTF` coefficient rows from EnergyPlus EIO contents.
pub fn parse_eio_construction_ctf_coefficients(
    contents: &str,
) -> Result<Vec<EioConstructionCtfCoefficient>, EioError> {
    let mut coefficients = Vec::new();
    let mut current_construction_name: Option<String> = None;
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if trimmed.starts_with("Construction CTF,") {
            let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
            current_construction_name = Some(required_field(&fields, 1).to_ascii_uppercase());
            continue;
        }
        if !trimmed.starts_with("CTF,") {
            continue;
        }

        let Some(construction_name) = current_construction_name.clone() else {
            return Err(EioError::InvalidConstructionCtfCoefficient {
                line: line_number,
                text: line.to_string(),
                reason: "CTF row appeared before any Construction CTF row".to_string(),
            });
        };
        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() <= 4 {
            return Err(EioError::InvalidConstructionCtfCoefficient {
                line: line_number,
                text: line.to_string(),
                reason: format!("expected at least 5 fields, found {}", fields.len()),
            });
        }

        let flux = match fields.get(5).copied().filter(|field| !field.is_empty()) {
            Some(_field) => Some(parse_ctf_f64_field(&fields, 5, line_number, line, "Flux")?),
            None => None,
        };
        coefficients.push(EioConstructionCtfCoefficient {
            construction_name,
            time_index: parse_ctf_usize_field(&fields, 1, line_number, line, "Time")?,
            outside: parse_ctf_f64_field(&fields, 2, line_number, line, "Outside")?,
            cross: parse_ctf_f64_field(&fields, 3, line_number, line, "Cross")?,
            inside: parse_ctf_f64_field(&fields, 4, line_number, line, "Inside")?,
            flux,
        });
    }

    if coefficients.is_empty() {
        return Err(EioError::MissingConstructionCtfCoefficient);
    }

    Ok(coefficients)
}

/// Parses `Material CTF Summary` rows from EnergyPlus EIO contents.
pub fn parse_eio_material_ctf_summary(
    contents: &str,
) -> Result<Vec<EioMaterialCtfSummary>, EioError> {
    let mut materials = Vec::new();
    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if !trimmed.starts_with("Material CTF Summary,") {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        if fields.len() <= 6 {
            return Err(EioError::InvalidMaterialCtfSummary {
                line: line_number,
                text: line.to_string(),
                reason: format!("expected at least 7 fields, found {}", fields.len()),
            });
        }

        materials.push(EioMaterialCtfSummary {
            material_name: required_field(&fields, 1).to_ascii_uppercase(),
            thickness_m: parse_material_f64_field(&fields, 2, line_number, line, "Thickness {m}")?,
            conductivity_w_per_m_k: parse_material_f64_field(
                &fields,
                3,
                line_number,
                line,
                "Conductivity {w/m-K}",
            )?,
            density_kg_per_m3: parse_material_f64_field(
                &fields,
                4,
                line_number,
                line,
                "Density {kg/m3}",
            )?,
            specific_heat_j_per_kg_k: parse_material_f64_field(
                &fields,
                5,
                line_number,
                line,
                "Specific Heat {J/kg-K}",
            )?,
            thermal_resistance_m2_k_per_w: parse_material_f64_field(
                &fields,
                6,
                line_number,
                line,
                "ThermalResistance {m2-K/w}",
            )?,
        });
    }

    if materials.is_empty() {
        return Err(EioError::MissingMaterialCtfSummary);
    }

    Ok(materials)
}

/// Parses `Environment` and following `Environment:WarmupDays` rows.
pub fn parse_eio_warmup_environments(
    contents: &str,
) -> Result<Vec<EioWarmupEnvironment>, EioError> {
    let mut rows = Vec::new();
    let mut current_environment: Option<(String, String)> = None;

    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed = line.trim();
        if trimmed.starts_with("Environment,") {
            let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
            if fields.len() > 2 {
                current_environment = Some((
                    required_field(&fields, 1).to_ascii_uppercase(),
                    required_field(&fields, 2).to_string(),
                ));
            }
            continue;
        }
        if !trimmed.starts_with("Environment:WarmupDays,") {
            continue;
        }

        let fields = trimmed.split(',').map(str::trim).collect::<Vec<_>>();
        let Some((environment_name, environment_type)) = current_environment.clone() else {
            return Err(EioError::InvalidWarmupEnvironment {
                line: line_number,
                text: line.to_string(),
                reason: "warmup row appeared before any Environment row".to_string(),
            });
        };
        let warmup_days = required_field(&fields, 1)
            .parse::<u32>()
            .map_err(|_error| EioError::InvalidWarmupEnvironment {
                line: line_number,
                text: line.to_string(),
                reason: "invalid warmup day count".to_string(),
            })?;
        rows.push(EioWarmupEnvironment {
            environment_name,
            environment_type,
            warmup_days,
        });
    }

    Ok(rows)
}

fn required_field<'a>(fields: &'a [&str], index: usize) -> &'a str {
    fields.get(index).copied().unwrap_or("")
}

fn parse_construction_ctf_row(
    fields: &[&str],
    line: usize,
    text: &str,
) -> Result<EioConstructionCtf, EioError> {
    Ok(EioConstructionCtf {
        construction_name: required_field(fields, 1).to_ascii_uppercase(),
        index: parse_construction_usize_field(fields, 2, line, text, "Index")?,
        layer_count: parse_construction_usize_field(fields, 3, line, text, "#Layers")?,
        ctf_count: parse_construction_usize_field(fields, 4, line, text, "#CTFs")?,
        timestep_hours: parse_construction_f64_field(fields, 5, line, text, "Time Step {hours}")?,
        thermal_conductance_w_per_m2_k: parse_construction_f64_field(
            fields,
            6,
            line,
            text,
            "ThermalConductance {w/m2-K}",
        )?,
        outer_thermal_absorptance: parse_construction_f64_field(
            fields,
            7,
            line,
            text,
            "OuterThermalAbsorptance",
        )?,
        inner_thermal_absorptance: parse_construction_f64_field(
            fields,
            8,
            line,
            text,
            "InnerThermalAbsorptance",
        )?,
        outer_solar_absorptance: parse_construction_f64_field(
            fields,
            9,
            line,
            text,
            "OuterSolarAbsorptance",
        )?,
        inner_solar_absorptance: parse_construction_f64_field(
            fields,
            10,
            line,
            text,
            "InnerSolarAbsorptance",
        )?,
        roughness: required_field(fields, 11).to_string(),
    })
}

fn parse_surface_geometry_choice(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
    choices: &[&str],
) -> Result<String, EioError> {
    let value = required_field(fields, index);
    if choices.contains(&value) {
        Ok(value.to_string())
    } else {
        Err(EioError::InvalidSurfaceGeometry {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
    }
}

fn parse_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidZoneInformation {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_other_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidOtherEquipmentNominal {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_surface_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidHeatTransferSurface {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_surface_usize_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<usize, EioError> {
    required_field(fields, index)
        .parse::<usize>()
        .map_err(|_error| EioError::InvalidHeatTransferSurface {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_construction_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidConstructionCtf {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_ctf_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidConstructionCtfCoefficient {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_material_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidMaterialCtfSummary {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_construction_material_f64_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<f64, EioError> {
    required_field(fields, index)
        .parse::<f64>()
        .map_err(|_error| EioError::InvalidConstructionMaterialSummary {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_construction_usize_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<usize, EioError> {
    required_field(fields, index)
        .parse::<usize>()
        .map_err(|_error| EioError::InvalidConstructionCtf {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_ctf_usize_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<usize, EioError> {
    required_field(fields, index)
        .parse::<usize>()
        .map_err(|_error| EioError::InvalidConstructionCtfCoefficient {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}

fn parse_usize_field(
    fields: &[&str],
    index: usize,
    line: usize,
    text: &str,
    field: &str,
) -> Result<usize, EioError> {
    required_field(fields, index)
        .parse::<usize>()
        .map_err(|_error| EioError::InvalidZoneInformation {
            line,
            text: text.to_string(),
            reason: format!("invalid {field}"),
        })
}
