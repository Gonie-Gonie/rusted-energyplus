//! Runtime error types.

use std::fmt::{Display, Formatter};

use ep_model::{ConstructionKind, MaterialFamily};

/// Runtime error for the first simulation subset.
#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    /// No zones were available to simulate.
    NoZones,
    /// No air-side nodes were available for a node-state projection.
    NoNodeStateProjectionNodes,
    /// No plant loops were available for a plant-state projection.
    NoPlantStateProjectionLoops,
    /// No weather data was supplied.
    NoWeatherData,
    /// The live weather-day owner could not complete an operation.
    WeatherDay {
        /// Actual owner failure; source diagnostic-string parity is not claimed.
        reason: String,
        /// Whether the selected original branch represents a source fatal.
        source_fatal: bool,
    },
    /// Requested more hourly samples than the weather series contains.
    SampleCountExceedsWeather {
        /// Requested sample count.
        requested: usize,
        /// Available weather samples.
        available: usize,
    },
    /// An internal-gain object references a schedule that an hour-only consumer cannot evaluate.
    InvalidInternalGainSchedule {
        /// EnergyPlus-normalized OtherEquipment name.
        equipment_name: String,
        /// Typed schedule identifier referenced by the object.
        schedule_id: u32,
        /// Missing-schedule or calendar-variation detail.
        reason: String,
    },
    /// Zone volume could not be derived from inputs.
    MissingZoneVolume {
        /// Zone name.
        zone_name: String,
    },
    /// The automatic-volume geometry lies outside the admitted closed-face path.
    UnsupportedZoneVolumeGeometry {
        /// Actual typed zone name.
        zone_name: String,
        /// Geometry-owner failure; no source diagnostic-string parity is claimed.
        reason: String,
    },
    /// A surface references a construction that is not available.
    MissingConstruction {
        /// Surface name.
        surface_name: String,
    },
    /// Actual surface geometry cannot initialize a numerical heat-transfer state.
    InvalidSurfaceGeometry {
        /// Actual EnergyPlus-normalized surface name.
        surface_name: String,
        /// Invalid input or derived geometry, without source warning-string parity.
        reason: String,
    },
    /// An opaque building surface references a non-opaque construction.
    UnsupportedConstructionForOpaqueHeatBalance {
        /// Surface name.
        surface_name: String,
        /// Referenced construction name.
        construction_name: String,
        /// Referenced construction family.
        construction_kind: ConstructionKind,
    },
    /// A construction references a material that is not available.
    MissingMaterial {
        /// Construction name.
        construction_name: String,
    },
    /// An opaque construction contains a material from another consumer family.
    UnsupportedMaterialForOpaqueHeatBalance {
        /// Construction name.
        construction_name: String,
        /// Material name.
        material_name: String,
        /// Material family.
        material_family: MaterialFamily,
    },
    /// A material has no usable thermal resistance.
    MissingThermalResistance {
        /// Material name.
        material_name: String,
    },
    /// A surface boundary references a target surface that is not available.
    MissingSurfaceBoundaryTarget {
        /// Surface name.
        surface_name: String,
        /// Referenced target name.
        target_name: String,
    },
    /// A surface boundary references a target zone or space that is not available.
    MissingZoneBoundaryTarget {
        /// Surface name.
        surface_name: String,
        /// Referenced target name.
        target_name: String,
    },
}

impl Display for RuntimeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoZones => write!(
                formatter,
                "first-zone simulation requires at least one Zone"
            ),
            Self::NoNodeStateProjectionNodes => write!(
                formatter,
                "node-state projection requires at least one resolved air-side node"
            ),
            Self::NoPlantStateProjectionLoops => write!(
                formatter,
                "plant-state projection requires at least one resolved plant loop"
            ),
            Self::NoWeatherData => write!(formatter, "first-zone simulation requires weather data"),
            Self::WeatherDay {
                reason,
                source_fatal,
            } => {
                write!(
                    formatter,
                    "weather-day operation failed (source fatal: {source_fatal}): {reason}"
                )
            }
            Self::SampleCountExceedsWeather {
                requested,
                available,
            } => write!(
                formatter,
                "requested {requested} weather samples but only {available} are available"
            ),
            Self::InvalidInternalGainSchedule {
                equipment_name,
                schedule_id,
                reason,
            } => write!(
                formatter,
                "OtherEquipment {equipment_name} schedule {schedule_id} is invalid for hour-only internal-gain consumption: {reason}"
            ),
            Self::MissingZoneVolume { zone_name } => write!(
                formatter,
                "could not derive a positive volume for zone {zone_name}"
            ),
            Self::UnsupportedZoneVolumeGeometry { zone_name, reason } => write!(
                formatter,
                "zone {zone_name} automatic-volume geometry is unsupported: {reason}"
            ),
            Self::MissingConstruction { surface_name } => write!(
                formatter,
                "surface {surface_name} references a missing construction"
            ),
            Self::InvalidSurfaceGeometry {
                surface_name,
                reason,
            } => write!(
                formatter,
                "surface {surface_name} geometry is invalid: {reason}"
            ),
            Self::UnsupportedConstructionForOpaqueHeatBalance {
                surface_name,
                construction_name,
                construction_kind,
            } => write!(
                formatter,
                "surface {surface_name} references {kind} construction {construction_name}, which the opaque heat-balance runtime cannot consume",
                kind = construction_kind.id()
            ),
            Self::MissingMaterial { construction_name } => write!(
                formatter,
                "construction {construction_name} references a missing material"
            ),
            Self::UnsupportedMaterialForOpaqueHeatBalance {
                construction_name,
                material_name,
                material_family,
            } => write!(
                formatter,
                "opaque construction {construction_name} contains {family} material {material_name}, which the opaque heat-balance runtime cannot consume",
                family = material_family.id()
            ),
            Self::MissingThermalResistance { material_name } => write!(
                formatter,
                "material {material_name} has no positive thermal resistance"
            ),
            Self::MissingSurfaceBoundaryTarget {
                surface_name,
                target_name,
            } => write!(
                formatter,
                "surface {surface_name} references missing outside boundary surface {target_name}"
            ),
            Self::MissingZoneBoundaryTarget {
                surface_name,
                target_name,
            } => write!(
                formatter,
                "surface {surface_name} references missing outside boundary zone {target_name}"
            ),
        }
    }
}

impl std::error::Error for RuntimeError {}

impl From<crate::weather::day::WeatherDayError> for RuntimeError {
    fn from(error: crate::weather::day::WeatherDayError) -> Self {
        Self::WeatherDay {
            source_fatal: error.is_source_fatal(),
            reason: error.to_string(),
        }
    }
}
