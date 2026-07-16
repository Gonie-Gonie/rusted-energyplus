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
        if fields.len() <= 13 {
            return Err(EioError::InvalidHeatTransferSurface {
                line: line_number,
                text: line.to_string(),
                reason: format!("expected at least 14 fields, found {}", fields.len()),
            });
        }

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

        constructions.push(EioConstructionCtf {
            construction_name: required_field(&fields, 1).to_ascii_uppercase(),
            index: parse_construction_usize_field(&fields, 2, line_number, line, "Index")?,
            layer_count: parse_construction_usize_field(&fields, 3, line_number, line, "#Layers")?,
            ctf_count: parse_construction_usize_field(&fields, 4, line_number, line, "#CTFs")?,
            timestep_hours: parse_construction_f64_field(
                &fields,
                5,
                line_number,
                line,
                "Time Step {hours}",
            )?,
            thermal_conductance_w_per_m2_k: parse_construction_f64_field(
                &fields,
                6,
                line_number,
                line,
                "ThermalConductance {w/m2-K}",
            )?,
            outer_thermal_absorptance: parse_construction_f64_field(
                &fields,
                7,
                line_number,
                line,
                "OuterThermalAbsorptance",
            )?,
            inner_thermal_absorptance: parse_construction_f64_field(
                &fields,
                8,
                line_number,
                line,
                "InnerThermalAbsorptance",
            )?,
            outer_solar_absorptance: parse_construction_f64_field(
                &fields,
                9,
                line_number,
                line,
                "OuterSolarAbsorptance",
            )?,
            inner_solar_absorptance: parse_construction_f64_field(
                &fields,
                10,
                line_number,
                line,
                "InnerSolarAbsorptance",
            )?,
            roughness: required_field(&fields, 11).to_string(),
        });
    }

    if constructions.is_empty() {
        return Err(EioError::MissingConstructionCtf);
    }

    Ok(constructions)
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
