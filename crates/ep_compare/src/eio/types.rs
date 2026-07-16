//! EnergyPlus EIO diagnostic table value types and errors.

use std::fmt::{Display, Formatter};

/// Global surface-geometry rules read from EnergyPlus `eplusout.eio`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EioSurfaceGeometryRules {
    /// Normalized starting corner.
    pub starting_corner: String,
    /// Normalized vertex input direction.
    pub vertex_input_direction: String,
    /// Normalized detailed-surface coordinate system.
    pub coordinate_system: String,
    /// Normalized daylight reference-point coordinate system.
    pub daylight_reference_point_coordinate_system: String,
    /// Normalized rectangular-surface coordinate system.
    pub rectangular_surface_coordinate_system: String,
}

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

/// A world-coordinate surface vertex read from EnergyPlus `eplusout.eio`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EioSurfaceVertex {
    /// World X coordinate in meters.
    pub x_m: f64,
    /// World Y coordinate in meters.
    pub y_m: f64,
    /// World Z coordinate in meters.
    pub z_m: f64,
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
    /// EIO `#Sides`.
    pub side_count: usize,
    /// Canonical world-coordinate vertices when `DetailsWithVertices` was requested.
    pub world_vertices: Option<Vec<EioSurfaceVertex>>,
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

/// Construction transfer-function coefficient values read from EnergyPlus `eplusout.eio`.
#[derive(Clone, Debug, PartialEq)]
pub struct EioConstructionCtfCoefficient {
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// EIO `Time` history index.
    pub time_index: usize,
    /// EIO outside/X coefficient.
    pub outside: f64,
    /// EIO cross/Y coefficient.
    pub cross: f64,
    /// EIO inside/Z coefficient.
    pub inside: f64,
    /// EIO flux coefficient; absent for the final time-zero row.
    pub flux: Option<f64>,
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

/// Generic material values read from an EnergyPlus `Material Details` row.
///
/// Rows are returned in EnergyPlus emission order. Repeated material names are
/// retained because this type represents EIO rows rather than a name-keyed map.
#[derive(Clone, Debug, PartialEq)]
pub struct EioMaterialDetails {
    /// EnergyPlus-normalized material name.
    pub material_name: String,
    /// EIO `ThermalResistance {m2-K/w}`.
    pub thermal_resistance_m2_k_per_w: f64,
    /// EIO surface roughness label.
    pub roughness: String,
    /// Material thickness in meters.
    pub thickness_m: f64,
    /// Material conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Material density in kg/m3.
    pub density_kg_per_m3: f64,
    /// Material specific heat in J/kg-K.
    pub specific_heat_j_per_kg_k: f64,
    /// Long-wave thermal absorptance.
    pub thermal_absorptance: f64,
    /// Solar absorptance.
    pub solar_absorptance: f64,
    /// Visible absorptance.
    pub visible_absorptance: f64,
}

/// EIO row format used to describe a construction material layer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EioMaterialCtfSummaryFormat {
    /// The seven-field `Material CTF Summary` format.
    ///
    /// This format is also emitted for `Material:InfraredTransparent`, so it
    /// identifies the EIO row shape rather than the EnergyPlus object type.
    Material,
    /// The resistance-only `Material:Air CTF Summary` format.
    Air,
}

/// One material layer nested under an EnergyPlus `Construction CTF` row.
#[derive(Clone, Debug, PartialEq)]
pub struct EioConstructionMaterialLayer {
    /// EnergyPlus-normalized material name.
    pub material_name: String,
    /// EIO row format; this is not a reliable EnergyPlus material object type.
    pub summary_format: EioMaterialCtfSummaryFormat,
    /// EIO material thickness in meters when the generic material row supplies it.
    pub thickness_m: Option<f64>,
    /// EIO conductivity in W/m-K when the generic material row supplies it.
    pub conductivity_w_per_m_k: Option<f64>,
    /// EIO density in kg/m3 when the generic material row supplies it.
    pub density_kg_per_m3: Option<f64>,
    /// EIO specific heat in J/kg-K when the generic material row supplies it.
    pub specific_heat_j_per_kg_k: Option<f64>,
    /// EIO area-normalized thermal resistance in m2-K/W.
    pub thermal_resistance_m2_k_per_w: f64,
}

/// A `Construction CTF` row grouped with its ordered material-layer summaries.
#[derive(Clone, Debug, PartialEq)]
pub struct EioConstructionMaterialSummary {
    /// Construction-level CTF summary and declared layer count.
    pub construction: EioConstructionCtf,
    /// Material layers in EnergyPlus outside-to-inside emission order.
    pub layers: Vec<EioConstructionMaterialLayer>,
}

/// Window glazing material values read from an EnergyPlus `eplusout.eio` row.
///
/// EnergyPlus emits one row for every construction-layer occurrence, so callers
/// must not assume material names are unique within the parsed row sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct EioWindowMaterialGlazing {
    /// EnergyPlus-normalized material name.
    pub material_name: String,
    /// EIO optical data model name, such as `SpectralAverage`.
    pub optical_data_type: String,
    /// EnergyPlus-normalized spectral data set name, when one is emitted.
    pub spectral_data_set_name: Option<String>,
    /// Glazing thickness in meters.
    pub thickness_m: f64,
    /// Normal-incidence solar transmittance.
    pub solar_transmittance: f64,
    /// Front-side normal-incidence solar reflectance.
    pub front_solar_reflectance: f64,
    /// Back-side normal-incidence solar reflectance.
    pub back_solar_reflectance: f64,
    /// Normal-incidence visible transmittance.
    pub visible_transmittance: f64,
    /// Front-side normal-incidence visible reflectance.
    pub front_visible_reflectance: f64,
    /// Back-side normal-incidence visible reflectance.
    pub back_visible_reflectance: f64,
    /// Infrared transmittance.
    pub infrared_transmittance: f64,
    /// Front-side thermal emissivity.
    pub front_thermal_emissivity: f64,
    /// Back-side thermal emissivity.
    pub back_thermal_emissivity: f64,
    /// Glazing conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Dirt correction factor for solar and visible transmittance.
    pub dirt_factor: f64,
    /// Whether the glazing is solar diffusing.
    pub solar_diffusing: bool,
}

/// Window gas material values read from an EnergyPlus `eplusout.eio` row.
///
/// EnergyPlus emits one row for every construction-layer occurrence, so
/// repeated material names remain distinct rows.
#[derive(Clone, Debug, PartialEq)]
pub struct EioWindowMaterialGas {
    /// EnergyPlus-normalized material name.
    pub material_name: String,
    /// Canonical EnergyPlus gas type text, such as `Air` or `Argon`.
    pub gas_type: String,
    /// Gas-layer thickness in meters.
    pub thickness_m: f64,
}

/// Window shade material values read from an EnergyPlus `eplusout.eio` row.
///
/// EnergyPlus emits one row for every successfully reported ordinary-window
/// shade-layer occurrence, so repeated material names remain distinct rows.
#[derive(Clone, Debug, PartialEq)]
pub struct EioWindowMaterialShade {
    /// EnergyPlus-normalized material name.
    pub material_name: String,
    /// Shade thickness in meters.
    pub thickness_m: f64,
    /// Shade conductivity in W/m-K.
    pub conductivity_w_per_m_k: f64,
    /// Long-wave thermal absorptance.
    pub thermal_absorptance: f64,
    /// Solar transmittance.
    pub solar_transmittance: f64,
    /// Visible transmittance.
    pub visible_transmittance: f64,
    /// Solar reflectance emitted as `Shade Reflectance`.
    pub solar_reflectance: f64,
}

/// Equivalent-layer window gap values read from an EnergyPlus `eplusout.eio` row.
///
/// EnergyPlus emits one row for every equivalent-layer construction-layer
/// occurrence, so repeated material names remain distinct rows.
#[derive(Clone, Debug, PartialEq)]
pub struct EioWindowMaterialGapEquivalentLayer {
    /// EnergyPlus-normalized material name.
    pub material_name: String,
    /// Canonical EnergyPlus gas type text, such as `Air` or `Argon`.
    pub gas_type: String,
    /// Gap-layer thickness in meters.
    pub gap_thickness_m: f64,
    /// Canonical EnergyPlus gap vent type text, such as `Sealed`.
    pub gap_vent_type: String,
}

/// Equivalent-layer glazing values read from an EnergyPlus `eplusout.eio` row.
///
/// EnergyPlus emits one row for every equivalent-layer construction-layer
/// occurrence, so repeated material names remain distinct rows.
#[derive(Clone, Debug, PartialEq)]
pub struct EioWindowMaterialGlazingEquivalentLayer {
    /// EnergyPlus-normalized material name.
    pub material_name: String,
    /// EIO optical data model name; EnergyPlus 26.1 emits `SpectralAverage`.
    pub optical_data_type: String,
    /// EnergyPlus-normalized spectral data set name, when one is emitted.
    pub spectral_data_set_name: Option<String>,
    /// Front-side beam-beam solar transmittance.
    pub front_beam_beam_solar_transmittance: f64,
    /// Back-side beam-beam solar transmittance.
    pub back_beam_beam_solar_transmittance: f64,
    /// Front-side beam-beam solar reflectance.
    pub front_beam_beam_solar_reflectance: f64,
    /// Back-side beam-beam solar reflectance.
    pub back_beam_beam_solar_reflectance: f64,
    /// Front-side beam-diffuse solar transmittance.
    pub front_beam_diffuse_solar_transmittance: f64,
    /// Back-side beam-diffuse solar transmittance.
    pub back_beam_diffuse_solar_transmittance: f64,
    /// Front-side beam-diffuse solar reflectance.
    pub front_beam_diffuse_solar_reflectance: f64,
    /// Back-side beam-diffuse solar reflectance.
    pub back_beam_diffuse_solar_reflectance: f64,
    /// Shared front/back diffuse-diffuse solar transmittance.
    pub diffuse_diffuse_solar_transmittance: f64,
    /// Front-side diffuse-diffuse solar reflectance.
    pub front_diffuse_diffuse_solar_reflectance: f64,
    /// Back-side diffuse-diffuse solar reflectance.
    pub back_diffuse_diffuse_solar_reflectance: f64,
    /// Shared front/back infrared transmittance.
    pub infrared_transmittance: f64,
    /// Front-side infrared emissivity.
    pub front_infrared_emissivity: f64,
    /// Back-side infrared emissivity.
    pub back_infrared_emissivity: f64,
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
    /// No `Surface Geometry` row was present.
    MissingSurfaceGeometry,
    /// No `Zone Information` rows were present.
    MissingZoneInformation,
    /// No `HeatTransfer Surface` rows were present.
    MissingHeatTransferSurface,
    /// No `OtherEquipment Internal Gains Nominal` rows were present.
    MissingOtherEquipmentNominal,
    /// No `Construction CTF` rows were present.
    MissingConstructionCtf,
    /// No `CTF` coefficient rows were present.
    MissingConstructionCtfCoefficient,
    /// No `Material CTF Summary` rows were present.
    MissingMaterialCtfSummary,
    /// No `Material Details` rows were present.
    MissingMaterialDetails,
    /// No `WindowMaterial:Glazing` rows were present.
    MissingWindowMaterialGlazing,
    /// No `WindowMaterial:Gas` rows were present.
    MissingWindowMaterialGas,
    /// No `WindowMaterial:Shade` rows were present.
    MissingWindowMaterialShade,
    /// No `WindowMaterial:Gap:EquivalentLayer` rows were present.
    MissingWindowMaterialGapEquivalentLayer,
    /// No `WindowMaterial:Glazing:EquivalentLayer` rows were present.
    MissingWindowMaterialGlazingEquivalentLayer,
    /// A grouped construction/material summary could not be parsed.
    InvalidConstructionMaterialSummary {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// A `Surface Geometry` row could not be parsed.
    InvalidSurfaceGeometry {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
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
    /// A `CTF` coefficient row could not be parsed.
    InvalidConstructionCtfCoefficient {
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
    /// A `Material Details` row could not be parsed.
    InvalidMaterialDetails {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// A `WindowMaterial:Glazing` row could not be parsed.
    InvalidWindowMaterialGlazing {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// A `WindowMaterial:Gas` row could not be parsed.
    InvalidWindowMaterialGas {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// A `WindowMaterial:Shade` row could not be parsed.
    InvalidWindowMaterialShade {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// A `WindowMaterial:Gap:EquivalentLayer` row could not be parsed.
    InvalidWindowMaterialGapEquivalentLayer {
        /// One-based line number.
        line: usize,
        /// Raw line text.
        text: String,
        /// Parse failure reason.
        reason: String,
    },
    /// A `WindowMaterial:Glazing:EquivalentLayer` row could not be parsed.
    InvalidWindowMaterialGlazingEquivalentLayer {
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
            Self::MissingSurfaceGeometry => write!(formatter, "EIO Surface Geometry not found"),
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
            Self::MissingConstructionCtfCoefficient => {
                write!(formatter, "EIO CTF coefficient rows not found")
            }
            Self::MissingMaterialCtfSummary => {
                write!(formatter, "EIO Material CTF Summary not found")
            }
            Self::MissingMaterialDetails => write!(formatter, "EIO Material Details not found"),
            Self::MissingWindowMaterialGlazing => {
                write!(formatter, "EIO WindowMaterial:Glazing not found")
            }
            Self::MissingWindowMaterialGas => {
                write!(formatter, "EIO WindowMaterial:Gas not found")
            }
            Self::MissingWindowMaterialShade => {
                write!(formatter, "EIO WindowMaterial:Shade not found")
            }
            Self::MissingWindowMaterialGapEquivalentLayer => {
                write!(
                    formatter,
                    "EIO WindowMaterial:Gap:EquivalentLayer not found"
                )
            }
            Self::MissingWindowMaterialGlazingEquivalentLayer => {
                write!(
                    formatter,
                    "EIO WindowMaterial:Glazing:EquivalentLayer not found"
                )
            }
            Self::InvalidSurfaceGeometry { line, text, reason } => write!(
                formatter,
                "invalid EIO Surface Geometry at line {line}: {reason}: {text}"
            ),
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
            Self::InvalidConstructionCtfCoefficient { line, text, reason } => write!(
                formatter,
                "invalid EIO CTF coefficient at line {line}: {reason}: {text}"
            ),
            Self::InvalidMaterialCtfSummary { line, text, reason } => write!(
                formatter,
                "invalid EIO Material CTF Summary at line {line}: {reason}: {text}"
            ),
            Self::InvalidMaterialDetails { line, text, reason } => write!(
                formatter,
                "invalid EIO Material Details at line {line}: {reason}: {text}"
            ),
            Self::InvalidConstructionMaterialSummary { line, text, reason } => write!(
                formatter,
                "invalid EIO construction/material summary at line {line}: {reason}: {text}"
            ),
            Self::InvalidWarmupEnvironment { line, text, reason } => write!(
                formatter,
                "invalid EIO Environment:WarmupDays at line {line}: {reason}: {text}"
            ),
            Self::InvalidWindowMaterialGlazing { line, text, reason } => write!(
                formatter,
                "invalid EIO WindowMaterial:Glazing at line {line}: {reason}: {text}"
            ),
            Self::InvalidWindowMaterialGas { line, text, reason } => write!(
                formatter,
                "invalid EIO WindowMaterial:Gas at line {line}: {reason}: {text}"
            ),
            Self::InvalidWindowMaterialShade { line, text, reason } => write!(
                formatter,
                "invalid EIO WindowMaterial:Shade at line {line}: {reason}: {text}"
            ),
            Self::InvalidWindowMaterialGapEquivalentLayer { line, text, reason } => write!(
                formatter,
                "invalid EIO WindowMaterial:Gap:EquivalentLayer at line {line}: {reason}: {text}"
            ),
            Self::InvalidWindowMaterialGlazingEquivalentLayer { line, text, reason } => write!(
                formatter,
                "invalid EIO WindowMaterial:Glazing:EquivalentLayer at line {line}: {reason}: {text}"
            ),
        }
    }
}

impl std::error::Error for EioError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::MissingSurfaceGeometry
            | Self::MissingZoneInformation
            | Self::MissingHeatTransferSurface
            | Self::MissingOtherEquipmentNominal
            | Self::InvalidZoneInformation { .. }
            | Self::InvalidHeatTransferSurface { .. }
            | Self::MissingConstructionCtf
            | Self::MissingConstructionCtfCoefficient
            | Self::MissingMaterialCtfSummary
            | Self::MissingMaterialDetails
            | Self::MissingWindowMaterialGlazing
            | Self::MissingWindowMaterialGas
            | Self::MissingWindowMaterialShade
            | Self::MissingWindowMaterialGapEquivalentLayer
            | Self::MissingWindowMaterialGlazingEquivalentLayer
            | Self::InvalidSurfaceGeometry { .. }
            | Self::InvalidOtherEquipmentNominal { .. }
            | Self::InvalidConstructionCtf { .. }
            | Self::InvalidConstructionCtfCoefficient { .. }
            | Self::InvalidMaterialCtfSummary { .. }
            | Self::InvalidMaterialDetails { .. }
            | Self::InvalidConstructionMaterialSummary { .. }
            | Self::InvalidWarmupEnvironment { .. }
            | Self::InvalidWindowMaterialGlazing { .. }
            | Self::InvalidWindowMaterialGas { .. }
            | Self::InvalidWindowMaterialShade { .. }
            | Self::InvalidWindowMaterialGapEquivalentLayer { .. }
            | Self::InvalidWindowMaterialGlazingEquivalentLayer { .. } => None,
        }
    }
}

impl From<std::io::Error> for EioError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}
