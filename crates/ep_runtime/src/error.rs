//! Runtime error types.

use std::fmt::{Display, Formatter};

use ep_model::{ConstructionKind, MaterialFamily};

/// Runtime error for the first simulation subset.
#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    /// Actual ordinary initialization owner/backend/fatal stop, with its original error retained.
    CtfInitialization(Box<crate::heat_balance::ctf_initialization_context::CtfInitializationError>),
    /// Actual selected public/history lifecycle stopped; no fallback or Native fatal is inferred.
    CtfSurfaceLifecycle(crate::heat_balance::ctf_surface_lifecycle::CtfSurfaceLifecycleUnavailable),
    /// Reached ordinary warmup output or backend stop with the original cause.
    WarmupOutput(Box<WarmupOutputError>),
    /// Actual surface outdoor-temperature guard or its retained backend failure.
    SurfaceOutdoorEnvironment(
        Box<crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentError>,
    ),
    /// No zones were available to simulate.
    NoZones,
    /// Actual warmup demand/topology/daily owner is outside this bounded Source route.
    WarmupUnavailable {
        /// Missing or unrepresentable real owner; never a fabricated Native fatal.
        reason: String,
    },
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
            Self::CtfInitialization(error) => Display::fmt(error, formatter),
            Self::WarmupOutput(error) => Display::fmt(error, formatter),
            Self::SurfaceOutdoorEnvironment(error) => Display::fmt(error, formatter),
            Self::CtfSurfaceLifecycle(error) => write!(
                formatter,
                "selected public CTF/history lifecycle unavailable: {error:?}"
            ),
            Self::WarmupUnavailable { reason } => {
                write!(formatter, "warmup owner unavailable: {reason}")
            }
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

impl std::error::Error for RuntimeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CtfInitialization(error) => Some(error.as_ref()),
            Self::WarmupOutput(error) => Some(error.as_ref()),
            Self::SurfaceOutdoorEnvironment(error) => Some(error.as_ref()),
            _ => None,
        }
    }
}
impl From<crate::heat_balance::ctf_initialization_context::CtfInitializationError>
    for RuntimeError
{
    fn from(
        error: crate::heat_balance::ctf_initialization_context::CtfInitializationError,
    ) -> Self {
        Self::CtfInitialization(Box::new(error))
    }
}

impl From<crate::weather::day::WeatherDayError> for RuntimeError {
    fn from(error: crate::weather::day::WeatherDayError) -> Self {
        Self::WeatherDay {
            source_fatal: error.is_source_fatal(),
            reason: error.to_string(),
        }
    }
}

/// Genuine ordinary warmup report/backend errors; no Native fatal is inferred.
#[derive(Debug)]
pub enum WarmupOutputError {
    /// Missing actual caller or allocated owner.
    Unavailable(String),
    /// Selected Source R formatting is outside its defined domain.
    Format(String),
    /// Real caller-owned EIO failure, retaining the original OS cause.
    Io(std::io::Error),
    /// Real selected UtilityRoutines backend stop.
    Backend(crate::heat_balance::error_backend::SourceErrorBackendFailure),
}
impl Display for WarmupOutputError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable(value) | Self::Format(value) => f.write_str(value),
            Self::Io(error) => Display::fmt(error, f),
            Self::Backend(error) => Display::fmt(error, f),
        }
    }
}
impl std::error::Error for WarmupOutputError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Backend(error) => Some(error),
            _ => None,
        }
    }
}
impl PartialEq for WarmupOutputError {
    fn eq(&self, other: &Self) -> bool {
        use crate::heat_balance::error_backend::SourceErrorBackendFailure as B;
        let io_equal = |a: &std::io::Error, b: &std::io::Error| {
            a.kind() == b.kind()
                && a.raw_os_error() == b.raw_os_error()
                && a.to_string() == b.to_string()
        };
        match (self, other) {
            (Self::Unavailable(a), Self::Unavailable(b)) | (Self::Format(a), Self::Format(b)) => {
                a == b
            }
            (Self::Io(a), Self::Io(b)) => io_equal(a, b),
            (
                Self::Backend(B::Effect {
                    operation: a,
                    source: x,
                }),
                Self::Backend(B::Effect {
                    operation: b,
                    source: y,
                }),
            ) => a == b && io_equal(x, y),
            (
                Self::Backend(B::UndefinedCounterIncrement { counter: a }),
                Self::Backend(B::UndefinedCounterIncrement { counter: b }),
            ) => a == b,
            _ => false,
        }
    }
}
impl From<crate::heat_balance::error_backend::SourceErrorBackendFailure> for RuntimeError {
    fn from(error: crate::heat_balance::error_backend::SourceErrorBackendFailure) -> Self {
        Self::WarmupOutput(Box::new(WarmupOutputError::Backend(error)))
    }
}
