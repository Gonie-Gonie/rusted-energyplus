//! First-zone simulation and geometry summary value types.

use crate::geometry::{surface_area_m2, zone_volume_m3};
use crate::heat_balance::{step_zone_air_temperature, surface_thermal_properties};
use crate::schedules::internal_gain_w;
use crate::{OutputSeries, ResultStore, RuntimeError, SimulationMode, SimulationState, ZoneState};
use ep_model::{
    OutputHandle, OutsideBoundaryCondition, SimulationModel, SurfaceId, SurfaceType, Zone, ZoneId,
};

const AIR_DENSITY_KG_PER_M3: f64 = 1.2;
const AIR_SPECIFIC_HEAT_J_PER_KG_K: f64 = 1006.0;
const SECONDS_PER_HOUR: f64 = 3600.0;

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

/// Executes the first uncontrolled one-zone thermal simulation subset.
pub fn simulate_first_zone_uncontrolled(
    model: &SimulationModel,
    weather_dry_bulb_c: &[f64],
    options: FirstZoneSimulationOptions,
) -> Result<FirstZoneSimulation, RuntimeError> {
    if weather_dry_bulb_c.is_empty() {
        return Err(RuntimeError::NoWeatherData);
    }
    if options.sample_count > weather_dry_bulb_c.len() {
        return Err(RuntimeError::SampleCountExceedsWeather {
            requested: options.sample_count,
            available: weather_dry_bulb_c.len(),
        });
    }

    let zone = model.typed.zones.first().ok_or(RuntimeError::NoZones)?;
    let characteristics = derive_first_zone_characteristics(model, zone, options.sample_count)?;
    let zone_steps_per_hour = model.typed.timestep.number_of_timesteps_per_hour.max(1);
    let seconds_per_timestep = SECONDS_PER_HOUR / f64::from(zone_steps_per_hour);

    let mut state = SimulationState::new(options.mode);
    let mut zone_air_temperature_c = options.initial_zone_air_temperature_c;
    state.zones.push(ZoneState {
        zone_id: zone.id,
        air_temperature_c: zone_air_temperature_c,
    });

    let mut zone_temperatures = Vec::with_capacity(options.sample_count);
    let mut outdoor_temperatures = Vec::with_capacity(options.sample_count);

    for (hour_index, outdoor_dry_bulb_c) in weather_dry_bulb_c
        .iter()
        .copied()
        .take(options.sample_count)
        .enumerate()
    {
        state.weather.outdoor_dry_bulb_c = outdoor_dry_bulb_c;
        let hour_ending = u32::try_from(hour_index % 24 + 1).unwrap_or(24);
        let internal_gain_w = internal_gain_w(&model.typed, zone.id, hour_ending);
        for _substep in 0..zone_steps_per_hour {
            zone_air_temperature_c = step_zone_air_temperature(
                zone_air_temperature_c,
                outdoor_dry_bulb_c,
                internal_gain_w,
                characteristics.conductance_w_per_k,
                characteristics.air_heat_capacity_j_per_k,
                seconds_per_timestep,
            );
            state.timestep_index += 1;
        }
        zone_temperatures.push(zone_air_temperature_c);
        outdoor_temperatures.push(outdoor_dry_bulb_c);
    }

    if let Some(zone_state) = state.zones.first_mut() {
        zone_state.air_temperature_c = zone_air_temperature_c;
    }

    let mut results = ResultStore::new();
    results.add_series(OutputSeries {
        handle: OutputHandle(0),
        key: zone.name.0.clone(),
        variable_name: "Zone Mean Air Temperature".to_string(),
        units: "C".to_string(),
        values: zone_temperatures,
    });
    results.add_series(OutputSeries {
        handle: OutputHandle(1),
        key: "Environment".to_string(),
        variable_name: "Site Outdoor Air Drybulb Temperature".to_string(),
        units: "C".to_string(),
        values: outdoor_temperatures,
    });

    Ok(FirstZoneSimulation {
        state,
        results,
        summary: characteristics,
    })
}

fn derive_first_zone_characteristics(
    model: &SimulationModel,
    zone: &Zone,
    sample_count: usize,
) -> Result<FirstZoneSimulationSummary, RuntimeError> {
    let volume_m3 =
        zone_volume_m3(&model.typed, zone).ok_or_else(|| RuntimeError::MissingZoneVolume {
            zone_name: zone.name.0.clone(),
        })?;
    let (exterior_area_m2, conductance_w_per_k) = exterior_zone_conductance(model, zone)?;
    let multiplier = f64::from(zone.multiplier.max(1));
    let air_heat_capacity_j_per_k =
        volume_m3 * multiplier * AIR_DENSITY_KG_PER_M3 * AIR_SPECIFIC_HEAT_J_PER_KG_K;
    let internal_gain_w = internal_gain_w(&model.typed, zone.id, 1);

    Ok(FirstZoneSimulationSummary {
        zone_id: zone.id,
        zone_name: zone.name.0.clone(),
        samples: sample_count,
        volume_m3,
        exterior_area_m2,
        conductance_w_per_k,
        air_heat_capacity_j_per_k,
        internal_gain_w,
    })
}

fn exterior_zone_conductance(
    model: &SimulationModel,
    zone: &Zone,
) -> Result<(f64, f64), RuntimeError> {
    let mut exterior_area_m2 = 0.0;
    let mut conductance_w_per_k = 0.0;

    for surface in model.typed.surfaces.iter().filter(|surface| {
        surface.zone == zone.id
            && surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors
    }) {
        let area_m2 = surface_area_m2(&surface.vertices);
        if area_m2 <= 0.0 {
            continue;
        }

        let thermal = surface_thermal_properties(&model.typed, surface)?;

        exterior_area_m2 += area_m2;
        conductance_w_per_k += area_m2 / thermal.thermal_resistance_m2_k_per_w;
    }

    Ok((exterior_area_m2, conductance_w_per_k))
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
