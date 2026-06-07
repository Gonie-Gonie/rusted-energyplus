//! EnergyPlus EIO diagnostic table readers.

use std::fmt::{Display, Formatter};
use std::path::Path;

/// Zone geometry values read from EnergyPlus `eplusout.eio`.
#[derive(Clone, Debug, PartialEq)]
pub struct EioZoneGeometry {
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// EIO `Number of Surfaces`.
    pub surface_count: usize,
    /// EIO `Floor Area {m2}`.
    pub floor_area_m2: f64,
    /// EIO `Volume {m3}`.
    pub volume_m3: f64,
    /// EIO `Exterior Gross Wall Area {m2}`.
    pub exterior_gross_wall_area_m2: f64,
}

/// Surface geometry values read from EnergyPlus `eplusout.eio`.
#[derive(Clone, Debug, PartialEq)]
pub struct EioHeatTransferSurface {
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// EIO surface class.
    pub surface_class: String,
    /// EIO construction name.
    pub construction_name: String,
    /// EIO `Area (Net) {m2}`.
    pub area_net_m2: f64,
    /// EIO `Area (Gross) {m2}`.
    pub area_gross_m2: f64,
    /// EIO `Azimuth {deg}`.
    pub azimuth_deg: f64,
    /// EIO `Tilt {deg}`.
    pub tilt_deg: f64,
}

/// OtherEquipment nominal internal gain values read from EnergyPlus `eplusout.eio`.
#[derive(Clone, Debug, PartialEq)]
pub struct EioOtherEquipmentNominal {
    /// Equipment name.
    pub equipment_name: String,
    /// Referenced schedule name.
    pub schedule_name: String,
    /// Target zone name.
    pub zone_name: String,
    /// EIO `Zone Floor Area {m2}`.
    pub zone_floor_area_m2: f64,
    /// EIO `Equipment Level {W}`.
    pub equipment_level_w: f64,
    /// EIO `Equipment/Floor Area {W/m2}`.
    pub equipment_per_floor_area_w_per_m2: f64,
    /// EIO `Fraction Latent`.
    pub fraction_latent: f64,
    /// EIO `Fraction Radiant`.
    pub fraction_radiant: f64,
    /// EIO `Fraction Lost`.
    pub fraction_lost: f64,
    /// EIO `Fraction Convected`.
    pub fraction_convected: f64,
}

/// Construction transfer-function summary values read from EnergyPlus `eplusout.eio`.
#[derive(Clone, Debug, PartialEq)]
pub struct EioConstructionCtf {
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// EIO construction index.
    pub index: usize,
    /// EIO number of construction layers.
    pub layer_count: usize,
    /// EIO number of CTF terms.
    pub ctf_count: usize,
    /// CTF timestep in hours.
    pub timestep_hours: f64,
    /// EIO `ThermalConductance {w/m2-K}`.
    pub thermal_conductance_w_per_m2_k: f64,
    /// Outer thermal absorptance.
    pub outer_thermal_absorptance: f64,
    /// Inner thermal absorptance.
    pub inner_thermal_absorptance: f64,
    /// Outer solar absorptance.
    pub outer_solar_absorptance: f64,
    /// Inner solar absorptance.
    pub inner_solar_absorptance: f64,
    /// EIO roughness label.
    pub roughness: String,
}

/// Material CTF summary values read from EnergyPlus `eplusout.eio`.
#[derive(Clone, Debug, PartialEq)]
pub struct EioMaterialCtfSummary {
    /// EnergyPlus-normalized material name.
    pub material_name: String,
    /// EIO material thickness in meters.
    pub thickness_m: f64,
    /// EIO conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// EIO density in kg/m3.
    pub density_kg_per_m3: f64,
    /// EIO specific heat in J/kg-K.
    pub specific_heat_j_per_kg_k: f64,
    /// EIO `ThermalResistance {m2-K/w}`.
    pub thermal_resistance_m2_k_per_w: f64,
}

/// Warmup day counts read from EnergyPlus `eplusout.eio` environment sections.
#[derive(Clone, Debug, PartialEq)]
pub struct EioWarmupEnvironment {
    /// EnergyPlus environment name.
    pub environment_name: String,
    /// EnergyPlus environment type.
    pub environment_type: String,
    /// EIO `Environment:WarmupDays` count.
    pub warmup_days: u32,
}

/// Error returned while reading EnergyPlus EIO tabular diagnostics.
#[derive(Debug)]
pub enum EioError {
    /// File read failed.
    Io(std::io::Error),
    /// No `Zone Information` rows were present.
    MissingZoneInformation,
    /// No `HeatTransfer Surface` rows were present.
    MissingHeatTransferSurface,
    /// No `OtherEquipment Internal Gains Nominal` rows were present.
    MissingOtherEquipmentNominal,
    /// No `Construction CTF` rows were present.
    MissingConstructionCtf,
    /// No `Material CTF Summary` rows were present.
    MissingMaterialCtfSummary,
    /// An `Environment:WarmupDays` row could not be parsed.
    InvalidWarmupEnvironment {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// A `Zone Information` row could not be parsed.
    InvalidZoneInformation {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// A `HeatTransfer Surface` row could not be parsed.
    InvalidHeatTransferSurface {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// An `OtherEquipment Internal Gains Nominal` row could not be parsed.
    InvalidOtherEquipmentNominal {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// A `Construction CTF` row could not be parsed.
    InvalidConstructionCtf {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// A `Material CTF Summary` row could not be parsed.
    InvalidMaterialCtfSummary {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
}

impl Display for EioError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "failed to read EIO: {error}"),
            Self::MissingZoneInformation => write!(formatter, "EIO Zone Information not found"),
            Self::MissingHeatTransferSurface => {
                write!(formatter, "EIO HeatTransfer Surface not found")
            }
            Self::MissingOtherEquipmentNominal => {
                write!(
                    formatter,
                    "EIO OtherEquipment Internal Gains Nominal not found"
                )
            }
            Self::MissingConstructionCtf => write!(formatter, "EIO Construction CTF not found"),
            Self::MissingMaterialCtfSummary => {
                write!(formatter, "EIO Material CTF Summary not found")
            }
            Self::InvalidZoneInformation { line, text, reason } => write!(
                formatter,
                "invalid EIO Zone Information at line {line}: {reason}: {text}"
            ),
            Self::InvalidHeatTransferSurface { line, text, reason } => write!(
                formatter,
                "invalid EIO HeatTransfer Surface at line {line}: {reason}: {text}"
            ),
            Self::InvalidOtherEquipmentNominal { line, text, reason } => write!(
                formatter,
                "invalid EIO OtherEquipment Internal Gains Nominal at line {line}: {reason}: {text}"
            ),
            Self::InvalidConstructionCtf { line, text, reason } => write!(
                formatter,
                "invalid EIO Construction CTF at line {line}: {reason}: {text}"
            ),
            Self::InvalidMaterialCtfSummary { line, text, reason } => write!(
                formatter,
                "invalid EIO Material CTF Summary at line {line}: {reason}: {text}"
            ),
            Self::InvalidWarmupEnvironment { line, text, reason } => write!(
                formatter,
                "invalid EIO Environment:WarmupDays at line {line}: {reason}: {text}"
            ),
        }
    }
}

impl std::error::Error for EioError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::MissingZoneInformation
            | Self::MissingHeatTransferSurface
            | Self::MissingOtherEquipmentNominal
            | Self::InvalidZoneInformation { .. }
            | Self::InvalidHeatTransferSurface { .. }
            | Self::MissingConstructionCtf
            | Self::MissingMaterialCtfSummary
            | Self::InvalidOtherEquipmentNominal { .. }
            | Self::InvalidConstructionCtf { .. }
            | Self::InvalidMaterialCtfSummary { .. }
            | Self::InvalidWarmupEnvironment { .. } => None,
        }
    }
}

impl From<std::io::Error> for EioError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
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
