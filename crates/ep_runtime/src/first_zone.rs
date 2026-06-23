//! First-zone simulation and geometry summary value types.

use crate::{ResultStore, SimulationMode, SimulationState};
use ep_model::{SurfaceId, SurfaceType, ZoneId};

/// Options for the first uncontrolled one-zone simulation subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FirstZoneSimulationOptions {
    /// Number of hourly weather samples to execute.
    pub sample_count: usize,
    /// Initial zone mean air temperature in C.
    pub initial_zone_air_temperature_c: f64,
    /// Runtime mode.
    pub mode: SimulationMode,
}

impl FirstZoneSimulationOptions {
    /// Creates options with a fixed hourly sample count.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            initial_zone_air_temperature_c: 20.0,
            mode: SimulationMode::Compatibility,
        }
    }
}

/// Summary of the derived first-zone thermal model.
#[derive(Clone, Debug, PartialEq)]
pub struct FirstZoneSimulationSummary {
    /// Zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// Hourly output sample count.
    pub samples: usize,
    /// Zone volume used by the air capacitance model.
    pub volume_m3: f64,
    /// Exterior opaque surface area used by the UA model.
    pub exterior_area_m2: f64,
    /// Envelope conductance in W/K.
    pub conductance_w_per_k: f64,
    /// Air heat capacity in J/K.
    pub air_heat_capacity_j_per_k: f64,
    /// First-hour internal sensible gain in W.
    pub internal_gain_w: f64,
}

/// Result of the first uncontrolled one-zone simulation subset.
#[derive(Clone, Debug, PartialEq)]
pub struct FirstZoneSimulation {
    /// Final mutable state.
    pub state: SimulationState,
    /// Native output results.
    pub results: ResultStore,
    /// Derived model summary.
    pub summary: FirstZoneSimulationSummary,
}

/// Zone geometry summary used for EnergyPlus EIO/internal-variable comparisons.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneGeometrySummary {
    /// Zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// Number of surfaces assigned to the zone.
    pub surface_count: usize,
    /// Sum of floor surface areas in square meters.
    pub floor_area_m2: f64,
    /// Derived or declared zone volume in cubic meters.
    pub volume_m3: Option<f64>,
    /// Gross exterior wall area in square meters.
    pub exterior_wall_area_m2: f64,
}

/// Surface geometry summary used for EnergyPlus EIO static-input comparisons.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceGeometrySummary {
    /// Surface ID.
    pub surface_id: SurfaceId,
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// Surface type.
    pub surface_type: SurfaceType,
    /// Net surface area in square meters.
    pub area_m2: f64,
    /// Surface azimuth in degrees clockwise from north.
    pub azimuth_deg: f64,
    /// Surface tilt in degrees.
    pub tilt_deg: f64,
}
