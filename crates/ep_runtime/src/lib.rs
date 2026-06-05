//! Runtime state, execution-plan shells, and first trace helpers.

use ep_model::{
    AutoOrNumber, OtherEquipment, OutputHandle, OutsideBoundaryCondition, Point3, RunPeriod,
    RunPeriodId, ScheduleCompactSegment, ScheduleId, SimulationModel, SurfaceId, SurfaceType,
    TypedModel, Zone, ZoneId,
};
use std::fmt::{Display, Formatter};
use std::path::Path;

const AIR_DENSITY_KG_PER_M3: f64 = 1.2;
const AIR_SPECIFIC_HEAT_J_PER_KG_K: f64 = 1006.0;
const SECONDS_PER_HOUR: f64 = 3600.0;
const DEFAULT_RUN_PERIOD_YEAR: u32 = 2013;

/// Runtime execution mode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SimulationMode {
    /// EnergyPlus compatibility-first deterministic scalar path.
    #[default]
    Compatibility,
    /// Trace-heavy diagnostics mode.
    Diagnostic,
    /// Future Rust-only optimized mode.
    Fast,
    /// Future isolated algorithm experiments.
    Experimental,
}

/// Minimal execution step set for v0.1 architecture boundaries.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ExecutionStep {
    /// Update weather-derived state.
    UpdateWeather,
    /// Evaluate one schedule.
    EvaluateSchedule(ScheduleId),
    /// Solve one zone.
    SolveZone(ZoneId),
    /// Write one output handle.
    WriteOutput(OutputHandle),
}

/// Named runtime execution stage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionStage {
    /// Stage name.
    pub name: String,
    /// Ordered execution steps in this stage.
    pub steps: Vec<ExecutionStep>,
}

/// Minimal deterministic execution plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionPlan {
    /// Ordered stages.
    pub stages: Vec<ExecutionStage>,
}

impl ExecutionPlan {
    /// Returns the total step count across all stages.
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.stages.iter().map(|stage| stage.steps.len()).sum()
    }
}

/// Builds the first deterministic execution plan for the typed subset.
#[must_use]
pub fn build_execution_plan(model: &SimulationModel) -> ExecutionPlan {
    let mut setup_steps = vec![ExecutionStep::UpdateWeather];
    setup_steps.extend(schedule_ids(&model.typed).map(ExecutionStep::EvaluateSchedule));

    let zone_steps = model
        .typed
        .zones
        .iter()
        .map(|zone| ExecutionStep::SolveZone(zone.id))
        .collect();

    ExecutionPlan {
        stages: vec![
            ExecutionStage {
                name: "environment".to_string(),
                steps: setup_steps,
            },
            ExecutionStage {
                name: "zone".to_string(),
                steps: zone_steps,
            },
            ExecutionStage {
                name: "output".to_string(),
                steps: vec![ExecutionStep::WriteOutput(OutputHandle(0))],
            },
        ],
    }
}

/// One hourly timestamp aligned to EnergyPlus run-period reporting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimePoint {
    /// Zero-based sample index.
    pub sample_index: usize,
    /// Calendar year used for date arithmetic.
    pub year: u32,
    /// Month number, 1-12.
    pub month: u32,
    /// Day of month.
    pub day_of_month: u32,
    /// EnergyPlus-style hour ending, 1-24.
    pub hour: u32,
}

/// Hourly time axis for one run period.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimeAxis {
    /// Run period name.
    pub run_period_name: String,
    /// Hourly samples in output order.
    pub points: Vec<TimePoint>,
}

impl TimeAxis {
    /// Returns the number of hourly samples.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.points.len()
    }
}

/// Error returned while building a run-period time axis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimeAxisError {
    /// A run-period date was invalid.
    InvalidDate {
        /// Run period name.
        run_period_name: String,
        /// Field group, such as begin or end.
        field: &'static str,
        /// Calendar year.
        year: u32,
        /// Month number.
        month: u32,
        /// Day of month.
        day_of_month: u32,
    },
    /// The end date came before the begin date.
    InvalidRange {
        /// Run period name.
        run_period_name: String,
    },
}

impl Display for TimeAxisError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidDate {
                run_period_name,
                field,
                year,
                month,
                day_of_month,
            } => write!(
                formatter,
                "run period {run_period_name} has invalid {field} date {year:04}-{month:02}-{day_of_month:02}"
            ),
            Self::InvalidRange { run_period_name } => {
                write!(
                    formatter,
                    "run period {run_period_name} ends before it begins"
                )
            }
        }
    }
}

impl std::error::Error for TimeAxisError {}

/// Builds the first hourly time axis from the model `RunPeriod` list.
///
/// If no `RunPeriod` is present, a one-day default axis is returned so early
/// diagnostic runtime paths remain explicit and deterministic.
pub fn build_hourly_time_axis(model: &TypedModel) -> Result<TimeAxis, TimeAxisError> {
    let fallback;
    let run_period = if let Some(run_period) = model.run_periods.first() {
        run_period
    } else {
        fallback = default_run_period();
        &fallback
    };

    build_hourly_time_axis_for_run_period(run_period)
}

/// Builds an hourly time axis for one run period.
pub fn build_hourly_time_axis_for_run_period(
    run_period: &RunPeriod,
) -> Result<TimeAxis, TimeAxisError> {
    let begin_year = run_period
        .begin_year
        .or(run_period.end_year)
        .unwrap_or(DEFAULT_RUN_PERIOD_YEAR);
    let end_year = run_period
        .end_year
        .or(run_period.begin_year)
        .unwrap_or(begin_year);
    let begin = Date {
        year: begin_year,
        month: run_period.begin_month,
        day_of_month: run_period.begin_day_of_month,
    };
    let end = Date {
        year: end_year,
        month: run_period.end_month,
        day_of_month: run_period.end_day_of_month,
    };

    let begin_ordinal = date_ordinal(begin).ok_or_else(|| TimeAxisError::InvalidDate {
        run_period_name: run_period.name.0.clone(),
        field: "begin",
        year: begin.year,
        month: begin.month,
        day_of_month: begin.day_of_month,
    })?;
    let end_ordinal = date_ordinal(end).ok_or_else(|| TimeAxisError::InvalidDate {
        run_period_name: run_period.name.0.clone(),
        field: "end",
        year: end.year,
        month: end.month,
        day_of_month: end.day_of_month,
    })?;
    if end_ordinal < begin_ordinal {
        return Err(TimeAxisError::InvalidRange {
            run_period_name: run_period.name.0.clone(),
        });
    }

    let mut points = Vec::new();
    let mut date = begin;
    let mut ordinal = begin_ordinal;
    while ordinal <= end_ordinal {
        for hour in 1..=24 {
            points.push(TimePoint {
                sample_index: points.len(),
                year: date.year,
                month: date.month,
                day_of_month: date.day_of_month,
                hour,
            });
        }
        if ordinal == end_ordinal {
            break;
        }
        date = next_day(date);
        ordinal += 1;
    }

    Ok(TimeAxis {
        run_period_name: run_period.name.0.clone(),
        points,
    })
}

/// Weather state for the current simulation timestep.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WeatherState {
    /// Outdoor dry-bulb temperature in C.
    pub outdoor_dry_bulb_c: f64,
}

/// Per-zone dynamic state.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneState {
    /// Zone ID.
    pub zone_id: ZoneId,
    /// Zone mean air temperature in C.
    pub air_temperature_c: f64,
}

/// Minimal explicit simulation state.
#[derive(Clone, Debug, PartialEq)]
pub struct SimulationState {
    /// Selected mode.
    pub mode: SimulationMode,
    /// Current zero-based timestep index.
    pub timestep_index: u64,
    /// Current weather state.
    pub weather: WeatherState,
    /// Current zone states.
    pub zones: Vec<ZoneState>,
}

impl SimulationState {
    /// Creates a new explicit simulation state.
    #[must_use]
    pub fn new(mode: SimulationMode) -> Self {
        Self {
            mode,
            timestep_index: 0,
            weather: WeatherState {
                outdoor_dry_bulb_c: 0.0,
            },
            zones: Vec::new(),
        }
    }
}

/// One output series stored by the runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputSeries {
    /// Stable output handle for the current run.
    pub handle: OutputHandle,
    /// EnergyPlus-style output key.
    pub key: String,
    /// Output variable name.
    pub variable_name: String,
    /// Display units.
    pub units: String,
    /// Sampled output values.
    pub values: Vec<f64>,
}

/// Structured output store for runtime-native results.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResultStore {
    /// Output series in handle order.
    pub series: Vec<OutputSeries>,
}

impl ResultStore {
    /// Creates an empty result store.
    #[must_use]
    pub fn new() -> Self {
        Self { series: Vec::new() }
    }

    /// Adds a complete output series.
    pub fn add_series(&mut self, series: OutputSeries) {
        self.series.push(series);
    }

    /// Returns the maximum sample count across all output series.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.series
            .iter()
            .map(|series| series.values.len())
            .max()
            .unwrap_or(0)
    }

    /// Finds one output series by EnergyPlus-style key and variable name.
    #[must_use]
    pub fn find_series(&self, key: &str, variable_name: &str) -> Option<&OutputSeries> {
        self.series.iter().find(|series| {
            series.key.eq_ignore_ascii_case(key)
                && series.variable_name.eq_ignore_ascii_case(variable_name)
        })
    }
}

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

/// Initial heat-balance state shell for the EnergyPlus porting path.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceState {
    /// Current zone timestep index.
    pub timestep_index: usize,
    /// Per-zone heat-balance state.
    pub zones: Vec<ZoneHeatBalanceState>,
    /// Per-surface heat-balance state.
    pub surfaces: Vec<SurfaceHeatBalanceState>,
}

/// Per-zone heat-balance state shell.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneHeatBalanceState {
    /// Zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// Current mean air temperature in C.
    pub mean_air_temperature_c: f64,
    /// Previous mean air temperature history in C.
    pub previous_mean_air_temperatures_c: [f64; 3],
    /// Zone volume in cubic meters.
    pub volume_m3: f64,
    /// Air heat capacity in J/K.
    pub air_heat_capacity_j_per_k: f64,
    /// First hour-ending convective internal gain in W.
    pub convective_internal_gain_w: f64,
}

/// Per-surface heat-balance state shell.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceHeatBalanceState {
    /// Surface ID.
    pub surface_id: SurfaceId,
    /// Owning zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized surface name.
    pub surface_name: String,
    /// Surface type.
    pub surface_type: SurfaceType,
    /// Surface area in square meters.
    pub area_m2: f64,
    /// Current inside face temperature in C.
    pub inside_face_temperature_c: f64,
    /// Current outside face temperature in C.
    pub outside_face_temperature_c: f64,
}

/// Runtime error for the first simulation subset.
#[derive(Debug, PartialEq)]
pub enum RuntimeError {
    /// No zones were available to simulate.
    NoZones,
    /// No weather data was supplied.
    NoWeatherData,
    /// Requested more hourly samples than the weather series contains.
    SampleCountExceedsWeather {
        /// Requested sample count.
        requested: usize,
        /// Available weather samples.
        available: usize,
    },
    /// Zone volume could not be derived from inputs.
    MissingZoneVolume {
        /// Zone name.
        zone_name: String,
    },
    /// A surface references a construction that is not available.
    MissingConstruction {
        /// Surface name.
        surface_name: String,
    },
    /// A construction references a material that is not available.
    MissingMaterial {
        /// Construction name.
        construction_name: String,
    },
    /// A material has no usable thermal resistance.
    MissingThermalResistance {
        /// Material name.
        material_name: String,
    },
}

impl Display for RuntimeError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoZones => write!(
                formatter,
                "first-zone simulation requires at least one Zone"
            ),
            Self::NoWeatherData => write!(formatter, "first-zone simulation requires weather data"),
            Self::SampleCountExceedsWeather {
                requested,
                available,
            } => write!(
                formatter,
                "requested {requested} weather samples but only {available} are available"
            ),
            Self::MissingZoneVolume { zone_name } => write!(
                formatter,
                "could not derive a positive volume for zone {zone_name}"
            ),
            Self::MissingConstruction { surface_name } => write!(
                formatter,
                "surface {surface_name} references a missing construction"
            ),
            Self::MissingMaterial { construction_name } => write!(
                formatter,
                "construction {construction_name} references a missing material"
            ),
            Self::MissingThermalResistance { material_name } => write!(
                formatter,
                "material {material_name} has no positive thermal resistance"
            ),
        }
    }
}

impl std::error::Error for RuntimeError {}

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

        let construction = model
            .typed
            .constructions
            .iter()
            .find(|construction| construction.id == surface.construction)
            .ok_or_else(|| RuntimeError::MissingConstruction {
                surface_name: surface.name.0.clone(),
            })?;
        let material = model
            .typed
            .materials
            .iter()
            .find(|material| material.id == construction.outside_layer)
            .ok_or_else(|| RuntimeError::MissingMaterial {
                construction_name: construction.name.0.clone(),
            })?;
        let resistance = material.thermal_resistance().ok_or_else(|| {
            RuntimeError::MissingThermalResistance {
                material_name: material.name.0.clone(),
            }
        })?;

        exterior_area_m2 += area_m2;
        conductance_w_per_k += area_m2 / resistance;
    }

    Ok((exterior_area_m2, conductance_w_per_k))
}

fn internal_gain_w(model: &TypedModel, zone_id: ZoneId, hour_ending: u32) -> f64 {
    model
        .other_equipment
        .iter()
        .filter(|equipment| equipment.zone == zone_id)
        .map(|equipment| internal_gain_for_equipment_w(model, equipment, hour_ending))
        .sum()
}

fn internal_gain_for_equipment_w(
    model: &TypedModel,
    equipment: &OtherEquipment,
    hour_ending: u32,
) -> f64 {
    let schedule_multiplier = equipment
        .schedule
        .and_then(|schedule_id| schedule_value(model, schedule_id, hour_ending))
        .unwrap_or(1.0);
    let sensible_fraction = (1.0 - equipment.fraction_latent - equipment.fraction_lost).max(0.0);

    equipment.design_level_w * schedule_multiplier * sensible_fraction
}

/// Initializes the heat-balance state shell without advancing the solver.
pub fn initialize_heat_balance_state(
    model: &SimulationModel,
    initial_zone_air_temperature_c: f64,
) -> Result<HeatBalanceState, RuntimeError> {
    let mut zones = Vec::with_capacity(model.typed.zones.len());
    for zone in &model.typed.zones {
        let volume_m3 =
            zone_volume_m3(&model.typed, zone).ok_or_else(|| RuntimeError::MissingZoneVolume {
                zone_name: zone.name.0.clone(),
            })?;
        zones.push(ZoneHeatBalanceState {
            zone_id: zone.id,
            zone_name: zone.name.0.clone(),
            mean_air_temperature_c: initial_zone_air_temperature_c,
            previous_mean_air_temperatures_c: [initial_zone_air_temperature_c; 3],
            volume_m3,
            air_heat_capacity_j_per_k: volume_m3
                * AIR_DENSITY_KG_PER_M3
                * AIR_SPECIFIC_HEAT_J_PER_KG_K,
            convective_internal_gain_w: convective_internal_gain_w(&model.typed, zone.id, 1),
        });
    }

    let surfaces = model
        .typed
        .surfaces
        .iter()
        .map(|surface| SurfaceHeatBalanceState {
            surface_id: surface.id,
            zone_id: surface.zone,
            surface_name: surface.name.0.clone(),
            surface_type: surface.surface_type,
            area_m2: surface_area_m2(&surface.vertices),
            inside_face_temperature_c: initial_zone_air_temperature_c,
            outside_face_temperature_c: initial_zone_air_temperature_c,
        })
        .collect();

    Ok(HeatBalanceState {
        timestep_index: 0,
        zones,
        surfaces,
    })
}

fn convective_internal_gain_w(model: &TypedModel, zone_id: ZoneId, hour_ending: u32) -> f64 {
    model
        .other_equipment
        .iter()
        .filter(|equipment| equipment.zone == zone_id)
        .map(|equipment| convective_internal_gain_for_equipment_w(model, equipment, hour_ending))
        .sum()
}

fn convective_internal_gain_for_equipment_w(
    model: &TypedModel,
    equipment: &OtherEquipment,
    hour_ending: u32,
) -> f64 {
    let schedule_multiplier = equipment
        .schedule
        .and_then(|schedule_id| schedule_value(model, schedule_id, hour_ending))
        .unwrap_or(1.0);
    let convective_fraction =
        (1.0 - equipment.fraction_latent - equipment.fraction_radiant - equipment.fraction_lost)
            .max(0.0);

    equipment.design_level_w * schedule_multiplier * convective_fraction
}

fn schedule_ids(model: &TypedModel) -> impl Iterator<Item = ScheduleId> + '_ {
    model
        .schedules
        .iter()
        .map(|schedule| schedule.id)
        .chain(model.compact_schedules.iter().map(|schedule| schedule.id))
}

fn schedule_value(model: &TypedModel, schedule_id: ScheduleId, hour_ending: u32) -> Option<f64> {
    if let Some(schedule) = model
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
    {
        return Some(schedule.hourly_value);
    }

    let minute_of_day = hour_ending.clamp(1, 24) * 60;
    model
        .compact_schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .and_then(|schedule| compact_schedule_value(&schedule.segments, minute_of_day))
}

fn compact_schedule_value(segments: &[ScheduleCompactSegment], minute_of_day: u32) -> Option<f64> {
    segments
        .iter()
        .find(|segment| minute_of_day <= segment.until_minute_of_day)
        .map(|segment| segment.value)
        .or_else(|| segments.last().map(|segment| segment.value))
}

/// Builds per-zone geometry summaries from the typed model.
#[must_use]
pub fn zone_geometry_summaries(model: &TypedModel) -> Vec<ZoneGeometrySummary> {
    model
        .zones
        .iter()
        .map(|zone| ZoneGeometrySummary {
            zone_id: zone.id,
            zone_name: zone.name.0.clone(),
            surface_count: model
                .surfaces
                .iter()
                .filter(|surface| surface.zone == zone.id)
                .count(),
            floor_area_m2: zone_floor_area_m2(model, zone),
            volume_m3: zone_volume_m3(model, zone),
            exterior_wall_area_m2: exterior_wall_area_m2(model, zone),
        })
        .collect()
}

fn zone_floor_area_m2(model: &TypedModel, zone: &Zone) -> f64 {
    model
        .surfaces
        .iter()
        .filter(|surface| surface.zone == zone.id && surface.surface_type == SurfaceType::Floor)
        .map(|surface| surface_area_m2(&surface.vertices))
        .sum()
}

fn exterior_wall_area_m2(model: &TypedModel, zone: &Zone) -> f64 {
    model
        .surfaces
        .iter()
        .filter(|surface| {
            surface.zone == zone.id
                && surface.surface_type == SurfaceType::Wall
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors
        })
        .map(|surface| surface_area_m2(&surface.vertices))
        .sum()
}

fn zone_volume_m3(model: &TypedModel, zone: &Zone) -> Option<f64> {
    if let AutoOrNumber::Value(volume_m3) = zone.volume
        && volume_m3 > 0.0
    {
        return Some(volume_m3);
    }

    if let Some(volume_m3) = bounding_box_volume_m3(model, zone)
        && volume_m3 > 0.0
    {
        return Some(volume_m3);
    }

    let AutoOrNumber::Value(ceiling_height_m) = zone.ceiling_height else {
        return None;
    };
    if ceiling_height_m <= 0.0 {
        return None;
    }
    let floor_area_m2 = zone_floor_area_m2(model, zone);
    if floor_area_m2 > 0.0 {
        Some(floor_area_m2 * ceiling_height_m)
    } else {
        None
    }
}

fn bounding_box_volume_m3(model: &TypedModel, zone: &Zone) -> Option<f64> {
    let mut bounds: Option<(f64, f64, f64, f64, f64, f64)> = None;
    for surface in model
        .surfaces
        .iter()
        .filter(|surface| surface.zone == zone.id)
    {
        for vertex in &surface.vertices {
            let x = vertex.x_m + zone.origin.x_m;
            let y = vertex.y_m + zone.origin.y_m;
            let z = vertex.z_m + zone.origin.z_m;
            bounds = Some(match bounds {
                Some((min_x, max_x, min_y, max_y, min_z, max_z)) => (
                    min_x.min(x),
                    max_x.max(x),
                    min_y.min(y),
                    max_y.max(y),
                    min_z.min(z),
                    max_z.max(z),
                ),
                None => (x, x, y, y, z, z),
            });
        }
    }

    let (min_x, max_x, min_y, max_y, min_z, max_z) = bounds?;
    let volume_m3 = (max_x - min_x) * (max_y - min_y) * (max_z - min_z);
    if volume_m3 > 0.0 {
        Some(volume_m3)
    } else {
        None
    }
}

/// Calculates a polygon surface area from 3D vertices in square meters.
#[must_use]
pub fn surface_area_m2(vertices: &[Point3]) -> f64 {
    if vertices.len() < 3 {
        return 0.0;
    }

    let origin = vertices[0];
    vertices[1..]
        .windows(2)
        .map(|window| {
            let first = vector_between(origin, window[0]);
            let second = vector_between(origin, window[1]);
            cross(first, second).magnitude() * 0.5
        })
        .sum()
}

fn step_zone_air_temperature(
    current_temperature_c: f64,
    outdoor_temperature_c: f64,
    internal_gain_w: f64,
    conductance_w_per_k: f64,
    heat_capacity_j_per_k: f64,
    timestep_seconds: f64,
) -> f64 {
    if heat_capacity_j_per_k <= 0.0 || timestep_seconds <= 0.0 {
        return current_temperature_c;
    }
    if conductance_w_per_k <= 0.0 {
        return current_temperature_c + internal_gain_w * timestep_seconds / heat_capacity_j_per_k;
    }

    let equilibrium_temperature_c = outdoor_temperature_c + internal_gain_w / conductance_w_per_k;
    let decay = (-conductance_w_per_k * timestep_seconds / heat_capacity_j_per_k).exp();
    equilibrium_temperature_c + (current_temperature_c - equilibrium_temperature_c) * decay
}

#[derive(Clone, Copy)]
struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3 {
    fn magnitude(self) -> f64 {
        (self
            .x
            .mul_add(self.x, self.y.mul_add(self.y, self.z * self.z)))
        .sqrt()
    }
}

fn vector_between(origin: Point3, point: Point3) -> Vector3 {
    Vector3 {
        x: point.x_m - origin.x_m,
        y: point.y_m - origin.y_m,
        z: point.z_m - origin.z_m,
    }
}

fn cross(left: Vector3, right: Vector3) -> Vector3 {
    Vector3 {
        x: left.y * right.z - left.z * right.y,
        y: left.z * right.x - left.x * right.z,
        z: left.x * right.y - left.y * right.x,
    }
}

#[derive(Clone, Copy)]
struct Date {
    year: u32,
    month: u32,
    day_of_month: u32,
}

fn default_run_period() -> RunPeriod {
    RunPeriod {
        id: RunPeriodId(0),
        name: ep_model::NormalizedName::new("Default Run Period"),
        begin_month: 1,
        begin_day_of_month: 1,
        begin_year: Some(DEFAULT_RUN_PERIOD_YEAR),
        end_month: 1,
        end_day_of_month: 1,
        end_year: Some(DEFAULT_RUN_PERIOD_YEAR),
        day_of_week_for_start_day: None,
    }
}

fn date_ordinal(date: Date) -> Option<i64> {
    let day_of_year = day_of_year(date.year, date.month, date.day_of_month)?;
    Some(days_before_year(date.year) + i64::from(day_of_year - 1))
}

fn days_before_year(year: u32) -> i64 {
    let previous = i64::from(year.saturating_sub(1));
    365 * previous + previous / 4 - previous / 100 + previous / 400
}

fn day_of_year(year: u32, month: u32, day_of_month: u32) -> Option<u32> {
    if !(1..=12).contains(&month) {
        return None;
    }
    let month_days = days_in_month(year, month);
    if day_of_month == 0 || day_of_month > month_days {
        return None;
    }
    let before_month = (1..month)
        .map(|value| days_in_month(year, value))
        .sum::<u32>();
    Some(before_month + day_of_month)
}

fn days_in_month(year: u32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

fn is_leap_year(year: u32) -> bool {
    year.is_multiple_of(4) && (!year.is_multiple_of(100) || year.is_multiple_of(400))
}

fn next_day(date: Date) -> Date {
    let month_days = days_in_month(date.year, date.month);
    if date.day_of_month < month_days {
        return Date {
            day_of_month: date.day_of_month + 1,
            ..date
        };
    }
    if date.month < 12 {
        return Date {
            month: date.month + 1,
            day_of_month: 1,
            ..date
        };
    }
    Date {
        year: date.year + 1,
        month: 1,
        day_of_month: 1,
    }
}

/// One sampled schedule output series.
#[derive(Clone, Debug, PartialEq)]
pub struct ScheduleTrace {
    /// Typed schedule ID.
    pub schedule_id: ScheduleId,
    /// EnergyPlus-normalized schedule name.
    pub schedule_name: String,
    /// Sampled schedule values.
    pub values: Vec<f64>,
}

/// One sampled zone internal-gain output series.
#[derive(Clone, Debug, PartialEq)]
pub struct ZoneInternalGainTrace {
    /// Typed zone ID.
    pub zone_id: ZoneId,
    /// EnergyPlus-normalized zone name.
    pub zone_name: String,
    /// Sampled convective internal gain values in W.
    pub values_w: Vec<f64>,
}

/// Simulates constant schedules for a fixed number of samples.
#[must_use]
pub fn simulate_constant_schedules(model: &TypedModel, sample_count: usize) -> Vec<ScheduleTrace> {
    model
        .schedules
        .iter()
        .map(|schedule| ScheduleTrace {
            schedule_id: schedule.id,
            schedule_name: schedule.name.0.clone(),
            values: vec![schedule.hourly_value; sample_count],
        })
        .collect()
}

/// Simulates constant and supported compact schedules for a fixed number of hourly samples.
#[must_use]
pub fn simulate_schedule_values(model: &TypedModel, sample_count: usize) -> Vec<ScheduleTrace> {
    schedule_ids(model)
        .filter_map(|schedule_id| {
            let schedule_name = schedule_name(model, schedule_id)?;
            let values = (0..sample_count)
                .map(|index| {
                    let hour_ending = u32::try_from(index % 24 + 1).unwrap_or(24);
                    schedule_value(model, schedule_id, hour_ending).unwrap_or(0.0)
                })
                .collect();
            Some(ScheduleTrace {
                schedule_id,
                schedule_name,
                values,
            })
        })
        .collect()
}

/// Simulates zone total internal convective heating rates for hourly samples.
#[must_use]
pub fn simulate_zone_internal_convective_gains(
    model: &TypedModel,
    sample_count: usize,
) -> Vec<ZoneInternalGainTrace> {
    model
        .zones
        .iter()
        .map(|zone| {
            let values_w = (0..sample_count)
                .map(|index| {
                    let hour_ending = u32::try_from(index % 24 + 1).unwrap_or(24);
                    convective_internal_gain_w(model, zone.id, hour_ending)
                })
                .collect();
            ZoneInternalGainTrace {
                zone_id: zone.id,
                zone_name: zone.name.0.clone(),
                values_w,
            }
        })
        .collect()
}

fn schedule_name(model: &TypedModel, schedule_id: ScheduleId) -> Option<String> {
    model
        .schedules
        .iter()
        .find(|schedule| schedule.id == schedule_id)
        .map(|schedule| schedule.name.0.clone())
        .or_else(|| {
            model
                .compact_schedules
                .iter()
                .find(|schedule| schedule.id == schedule_id)
                .map(|schedule| schedule.name.0.clone())
        })
}

/// Error returned while reading EPW weather data.
#[derive(Debug)]
pub enum EpwError {
    /// File read failed.
    Io(std::io::Error),
    /// EPW data row was missing a required column.
    MissingField {
        /// One-based line number.
        line: usize,
        /// EPW field name.
        field: &'static str,
    },
    /// EPW numeric field could not be parsed.
    InvalidNumber {
        /// One-based line number.
        line: usize,
        /// EPW field name.
        field: &'static str,
        /// Raw field text.
        value: String,
    },
}

impl Display for EpwError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "failed to read EPW: {error}"),
            Self::MissingField { line, field } => {
                write!(formatter, "EPW row at line {line} is missing {field}")
            }
            Self::InvalidNumber { line, field, value } => {
                write!(
                    formatter,
                    "EPW row at line {line} has invalid {field} value '{value}'"
                )
            }
        }
    }
}

impl std::error::Error for EpwError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::MissingField { .. } | Self::InvalidNumber { .. } => None,
        }
    }
}

impl From<std::io::Error> for EpwError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// One hourly EPW weather record for the current compatibility subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EpwRecord {
    /// Calendar year.
    pub year: u32,
    /// Month number, 1-12.
    pub month: u32,
    /// Day of month.
    pub day: u32,
    /// Hour ending, 1-24.
    pub hour: u32,
    /// Minute field from EPW.
    pub minute: u32,
    /// Outdoor dry-bulb temperature in C.
    pub dry_bulb_c: f64,
    /// Outdoor dew-point temperature in C.
    pub dew_point_c: f64,
    /// Relative humidity in percent.
    pub relative_humidity_percent: f64,
    /// Atmospheric station pressure in Pa.
    pub atmospheric_pressure_pa: f64,
    /// Horizontal infrared radiation intensity in Wh/m2.
    pub horizontal_infrared_radiation_wh_per_m2: f64,
    /// Global horizontal radiation in Wh/m2.
    pub global_horizontal_radiation_wh_per_m2: f64,
    /// Direct normal radiation in Wh/m2.
    pub direct_normal_radiation_wh_per_m2: f64,
    /// Diffuse horizontal radiation in Wh/m2.
    pub diffuse_horizontal_radiation_wh_per_m2: f64,
    /// Wind direction in degrees.
    pub wind_direction_deg: f64,
    /// Wind speed in m/s.
    pub wind_speed_m_per_s: f64,
}

/// Loads hourly EPW records from a weather file.
pub fn load_epw_records(path: impl AsRef<Path>) -> Result<Vec<EpwRecord>, EpwError> {
    let contents = std::fs::read_to_string(path)?;
    parse_epw_records(&contents)
}

/// Parses hourly EPW records from weather text.
pub fn parse_epw_records(contents: &str) -> Result<Vec<EpwRecord>, EpwError> {
    let mut records = Vec::new();

    for (line_index, line) in contents.lines().enumerate().skip(8) {
        let line_number = line_index + 1;
        if line.trim().is_empty() {
            continue;
        }
        let fields = line.split(',').collect::<Vec<_>>();
        records.push(EpwRecord {
            year: parse_epw_u32(&fields, line_number, 0, "year")?,
            month: parse_epw_u32(&fields, line_number, 1, "month")?,
            day: parse_epw_u32(&fields, line_number, 2, "day")?,
            hour: parse_epw_u32(&fields, line_number, 3, "hour")?,
            minute: parse_epw_u32(&fields, line_number, 4, "minute")?,
            dry_bulb_c: parse_epw_f64(&fields, line_number, 6, "dry-bulb")?,
            dew_point_c: parse_epw_f64(&fields, line_number, 7, "dew-point")?,
            relative_humidity_percent: parse_epw_f64(&fields, line_number, 8, "relative humidity")?,
            atmospheric_pressure_pa: parse_epw_f64(
                &fields,
                line_number,
                9,
                "atmospheric pressure",
            )?,
            horizontal_infrared_radiation_wh_per_m2: parse_epw_f64(
                &fields,
                line_number,
                12,
                "horizontal infrared radiation",
            )?,
            global_horizontal_radiation_wh_per_m2: parse_epw_f64(
                &fields,
                line_number,
                13,
                "global horizontal radiation",
            )?,
            direct_normal_radiation_wh_per_m2: parse_epw_f64(
                &fields,
                line_number,
                14,
                "direct normal radiation",
            )?,
            diffuse_horizontal_radiation_wh_per_m2: parse_epw_f64(
                &fields,
                line_number,
                15,
                "diffuse horizontal radiation",
            )?,
            wind_direction_deg: parse_epw_f64(&fields, line_number, 20, "wind direction")?,
            wind_speed_m_per_s: parse_epw_f64(&fields, line_number, 21, "wind speed")?,
        });
    }

    Ok(records)
}

/// Loads hourly outdoor dry-bulb values from an EPW file.
pub fn load_epw_dry_bulb_series(path: impl AsRef<Path>) -> Result<Vec<f64>, EpwError> {
    let contents = std::fs::read_to_string(path)?;
    parse_epw_dry_bulb_series(&contents)
}

/// Parses hourly outdoor dry-bulb values from EPW text.
pub fn parse_epw_dry_bulb_series(contents: &str) -> Result<Vec<f64>, EpwError> {
    parse_epw_records(contents).map(|records| {
        records
            .into_iter()
            .map(|record| record.dry_bulb_c)
            .collect()
    })
}

fn parse_epw_u32(
    fields: &[&str],
    line: usize,
    index: usize,
    field: &'static str,
) -> Result<u32, EpwError> {
    let value = epw_field(fields, line, index, field)?;
    value
        .trim()
        .parse::<u32>()
        .map_err(|_error| EpwError::InvalidNumber {
            line,
            field,
            value: value.to_string(),
        })
}

fn parse_epw_f64(
    fields: &[&str],
    line: usize,
    index: usize,
    field: &'static str,
) -> Result<f64, EpwError> {
    let value = epw_field(fields, line, index, field)?;
    value
        .trim()
        .parse::<f64>()
        .map_err(|_error| EpwError::InvalidNumber {
            line,
            field,
            value: value.to_string(),
        })
}

fn epw_field<'a>(
    fields: &'a [&str],
    line: usize,
    index: usize,
    field: &'static str,
) -> Result<&'a str, EpwError> {
    fields
        .get(index)
        .copied()
        .ok_or(EpwError::MissingField { line, field })
}

#[cfg(test)]
mod tests {
    use super::{
        ExecutionStep, FirstZoneSimulationOptions, OutputSeries, ResultStore, SimulationMode,
        SimulationState, build_execution_plan, build_hourly_time_axis,
        build_hourly_time_axis_for_run_period, initialize_heat_balance_state,
        parse_epw_dry_bulb_series, parse_epw_records, simulate_constant_schedules,
        simulate_first_zone_uncontrolled, simulate_schedule_values,
        simulate_zone_internal_convective_gains, surface_area_m2, zone_geometry_summaries,
    };
    use ep_model::{
        AutoOrNumber, Construction, ConstructionId, InternalGainId, Material, MaterialId,
        MaterialKind, NormalizedName, OtherEquipment, OutputHandle, OutsideBoundaryCondition,
        Point3, RunPeriod, RunPeriodId, ScheduleCompact, ScheduleCompactSegment, ScheduleConstant,
        ScheduleId, SimulationModel, Surface, SurfaceId, SurfaceType, TimestepConfig, TypedModel,
        Zone, ZoneId,
    };

    #[test]
    fn state_defaults_to_first_timestep() {
        let state = SimulationState::new(SimulationMode::Compatibility);

        assert_eq!(state.timestep_index, 0);
        assert_eq!(state.mode, SimulationMode::Compatibility);
        assert!(state.zones.is_empty());
    }

    #[test]
    fn constant_schedule_trace_repeats_hourly_value() {
        let mut model = TypedModel::default();
        model.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("AlwaysOn"),
            schedule_type_limits: None,
            hourly_value: 1.0,
        });

        let traces = simulate_constant_schedules(&model, 3);

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].schedule_name, "ALWAYSON");
        assert_eq!(traces[0].values, vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn compact_schedule_trace_uses_until_segments() {
        let mut model = TypedModel::default();
        model.compact_schedules.push(ScheduleCompact {
            id: ScheduleId(0),
            name: NormalizedName::new("Office Occupancy"),
            schedule_type_limits: None,
            segments: vec![
                ScheduleCompactSegment {
                    until_minute_of_day: 8 * 60,
                    value: 0.0,
                },
                ScheduleCompactSegment {
                    until_minute_of_day: 18 * 60,
                    value: 1.0,
                },
                ScheduleCompactSegment {
                    until_minute_of_day: 24 * 60,
                    value: 0.0,
                },
            ],
        });

        let traces = simulate_schedule_values(&model, 24);

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].values[7], 0.0);
        assert_eq!(traces[0].values[8], 1.0);
        assert_eq!(traces[0].values[17], 1.0);
        assert_eq!(traces[0].values[18], 0.0);
    }

    #[test]
    fn zone_internal_convective_gain_trace_excludes_radiant_fraction() {
        let mut model = cube_model();
        model.other_equipment[0].fraction_radiant = 0.25;

        let traces = simulate_zone_internal_convective_gains(&model, 2);

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].zone_name, "ZONE ONE");
        assert_eq!(traces[0].values_w, vec![9.0, 9.0]);
    }

    #[test]
    fn default_time_axis_has_one_day() -> Result<(), Box<dyn std::error::Error>> {
        let axis = build_hourly_time_axis(&TypedModel::default())?;

        assert_eq!(axis.sample_count(), 24);
        assert_eq!(axis.points[0].hour, 1);
        assert_eq!(axis.points[23].hour, 24);

        Ok(())
    }

    #[test]
    fn run_period_time_axis_counts_inclusive_days() -> Result<(), Box<dyn std::error::Error>> {
        let axis = build_hourly_time_axis_for_run_period(&RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("Three Days"),
            begin_month: 1,
            begin_day_of_month: 1,
            begin_year: Some(2013),
            end_month: 1,
            end_day_of_month: 3,
            end_year: Some(2013),
            day_of_week_for_start_day: None,
        })?;

        assert_eq!(axis.sample_count(), 72);
        assert_eq!(axis.points[0].day_of_month, 1);
        assert_eq!(axis.points[71].day_of_month, 3);
        assert_eq!(axis.points[71].hour, 24);

        Ok(())
    }

    #[test]
    fn run_period_time_axis_handles_leap_year() -> Result<(), Box<dyn std::error::Error>> {
        let axis = build_hourly_time_axis_for_run_period(&RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("Leap Window"),
            begin_month: 2,
            begin_day_of_month: 28,
            begin_year: Some(2020),
            end_month: 3,
            end_day_of_month: 1,
            end_year: Some(2020),
            day_of_week_for_start_day: None,
        })?;

        assert_eq!(axis.sample_count(), 72);
        assert_eq!(axis.points[24].day_of_month, 29);

        Ok(())
    }

    #[test]
    fn execution_plan_orders_weather_schedule_zone_and_output() {
        let mut typed = TypedModel::default();
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("AlwaysOn"),
            schedule_type_limits: None,
            hourly_value: 1.0,
        });
        typed.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: ep_model::Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            ceiling_height: ep_model::AutoOrNumber::AutoCalculate,
            volume: ep_model::AutoOrNumber::AutoCalculate,
        });
        let model = SimulationModel::from_typed(typed);

        let plan = build_execution_plan(&model);

        assert_eq!(plan.stages.len(), 3);
        assert_eq!(plan.step_count(), 4);
        assert_eq!(plan.stages[0].steps[0], ExecutionStep::UpdateWeather);
        assert_eq!(
            plan.stages[0].steps[1],
            ExecutionStep::EvaluateSchedule(ScheduleId(0))
        );
        assert_eq!(plan.stages[1].steps[0], ExecutionStep::SolveZone(ZoneId(0)));
    }

    #[test]
    fn parses_epw_records_after_header() -> Result<(), Box<dyn std::error::Error>> {
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
1999,1,1,1,0,Source,-3.0,-4.0,50,82000,0,0,300,10,20,30,0,0,0,0,180,2.5
1999,1,1,2,0,Source,-2.0,-3.0,51,82100,0,0,301,11,21,31,0,0,0,0,190,2.6
"#,
        )?;

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].dry_bulb_c, -3.0);
        assert_eq!(records[0].dew_point_c, -4.0);
        assert_eq!(records[0].relative_humidity_percent, 50.0);
        assert_eq!(records[0].atmospheric_pressure_pa, 82_000.0);
        assert_eq!(records[0].wind_direction_deg, 180.0);
        assert_eq!(records[0].wind_speed_m_per_s, 2.5);

        Ok(())
    }

    #[test]
    fn parses_epw_dry_bulb_values_after_header() -> Result<(), Box<dyn std::error::Error>> {
        let values = parse_epw_dry_bulb_series(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
1999,1,1,1,0,Source,-3.0,-4.0,50,82000,0,0,300,10,20,30,0,0,0,0,180,2.5
1999,1,1,2,0,Source,-2.0,-3.0,51,82100,0,0,301,11,21,31,0,0,0,0,190,2.6
"#,
        )?;

        assert_eq!(values, vec![-3.0, -2.0]);

        Ok(())
    }

    #[test]
    fn surface_area_handles_3d_rectangles() {
        let vertices = vec![
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 2.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 2.0,
                y_m: 0.0,
                z_m: 3.0,
            },
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 3.0,
            },
        ];

        assert_eq!(surface_area_m2(&vertices), 6.0);
    }

    #[test]
    fn zone_geometry_summary_reports_cube_metrics() {
        let summaries = zone_geometry_summaries(&cube_model());

        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].zone_name, "ZONE ONE");
        assert_eq!(summaries[0].surface_count, 6);
        assert_eq!(summaries[0].floor_area_m2, 1.0);
        assert_eq!(summaries[0].volume_m3, Some(1.0));
        assert_eq!(summaries[0].exterior_wall_area_m2, 4.0);
    }

    #[test]
    fn heat_balance_state_shell_initializes_cube_metrics() -> Result<(), Box<dyn std::error::Error>>
    {
        let model = SimulationModel::from_typed(cube_model());

        let state = initialize_heat_balance_state(&model, 20.0)?;

        assert_eq!(state.timestep_index, 0);
        assert_eq!(state.zones.len(), 1);
        assert_eq!(state.zones[0].zone_name, "ZONE ONE");
        assert_eq!(state.zones[0].mean_air_temperature_c, 20.0);
        assert_eq!(state.zones[0].previous_mean_air_temperatures_c, [20.0; 3]);
        assert_eq!(state.zones[0].volume_m3, 1.0);
        assert!((state.zones[0].air_heat_capacity_j_per_k - 1207.2).abs() < 1.0e-9);
        assert_eq!(state.zones[0].convective_internal_gain_w, 12.0);
        assert_eq!(state.surfaces.len(), 6);
        assert_eq!(state.surfaces[0].area_m2, 1.0);
        assert_eq!(state.surfaces[0].inside_face_temperature_c, 20.0);
        assert_eq!(state.surfaces[0].outside_face_temperature_c, 20.0);

        Ok(())
    }

    #[test]
    fn result_store_finds_series_case_insensitively() {
        let mut store = ResultStore::new();
        store.add_series(OutputSeries {
            handle: OutputHandle(0),
            key: "ZONE ONE".to_string(),
            variable_name: "Zone Mean Air Temperature".to_string(),
            units: "C".to_string(),
            values: vec![20.0, 21.0],
        });

        assert_eq!(store.sample_count(), 2);
        assert!(
            store
                .find_series("zone one", "zone mean air temperature")
                .is_some()
        );
    }

    #[test]
    fn first_zone_simulation_writes_zone_temperature_results()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());

        let simulation = simulate_first_zone_uncontrolled(
            &model,
            &[20.0, 20.0],
            FirstZoneSimulationOptions::hourly_samples(2),
        )?;

        assert_eq!(simulation.summary.zone_name, "ZONE ONE");
        assert_eq!(simulation.summary.samples, 2);
        assert_eq!(simulation.summary.volume_m3, 1.0);
        assert_eq!(simulation.summary.exterior_area_m2, 6.0);
        assert_eq!(simulation.summary.conductance_w_per_k, 6.0);
        assert_eq!(simulation.summary.internal_gain_w, 12.0);
        assert_eq!(simulation.results.sample_count(), 2);
        assert_eq!(simulation.results.series.len(), 2);
        assert_eq!(simulation.state.timestep_index, 12);
        let Some(zone_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing zone series").into());
        };
        assert!(zone_series.values[0] > 20.0);
        assert!(zone_series.values[1] >= zone_series.values[0]);

        Ok(())
    }

    fn cube_model() -> TypedModel {
        let mut model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 6,
            },
            ..TypedModel::default()
        };
        model.materials.push(Material {
            id: MaterialId(0),
            name: NormalizedName::new("R1"),
            kind: MaterialKind::NoMass,
            conductivity_w_per_m_k: None,
            density_kg_per_m3: None,
            specific_heat_j_per_kg_k: None,
            thickness_m: None,
            thermal_resistance_m2_k_per_w: Some(1.0),
        });
        model.constructions.push(Construction {
            id: ConstructionId(0),
            name: NormalizedName::new("Wall"),
            outside_layer: MaterialId(0),
        });
        model.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("Always On"),
            schedule_type_limits: None,
            hourly_value: 1.0,
        });
        model.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::AutoCalculate,
        });
        model.other_equipment.push(OtherEquipment {
            id: InternalGainId(0),
            name: NormalizedName::new("Plug Load"),
            zone: ZoneId(0),
            schedule: Some(ScheduleId(0)),
            design_level_w: 12.0,
            fraction_latent: 0.0,
            fraction_radiant: 0.0,
            fraction_lost: 0.0,
        });
        model.surfaces.extend(cube_surfaces());
        model
    }

    fn cube_surfaces() -> Vec<Surface> {
        vec![
            surface(
                0,
                "Floor",
                SurfaceType::Floor,
                [
                    point(0.0, 0.0, 0.0),
                    point(1.0, 0.0, 0.0),
                    point(1.0, 1.0, 0.0),
                    point(0.0, 1.0, 0.0),
                ],
            ),
            surface(
                1,
                "Roof",
                SurfaceType::Roof,
                [
                    point(0.0, 0.0, 1.0),
                    point(1.0, 0.0, 1.0),
                    point(1.0, 1.0, 1.0),
                    point(0.0, 1.0, 1.0),
                ],
            ),
            surface(
                2,
                "Wall X0",
                SurfaceType::Wall,
                [
                    point(0.0, 0.0, 0.0),
                    point(0.0, 1.0, 0.0),
                    point(0.0, 1.0, 1.0),
                    point(0.0, 0.0, 1.0),
                ],
            ),
            surface(
                3,
                "Wall X1",
                SurfaceType::Wall,
                [
                    point(1.0, 0.0, 0.0),
                    point(1.0, 1.0, 0.0),
                    point(1.0, 1.0, 1.0),
                    point(1.0, 0.0, 1.0),
                ],
            ),
            surface(
                4,
                "Wall Y0",
                SurfaceType::Wall,
                [
                    point(0.0, 0.0, 0.0),
                    point(1.0, 0.0, 0.0),
                    point(1.0, 0.0, 1.0),
                    point(0.0, 0.0, 1.0),
                ],
            ),
            surface(
                5,
                "Wall Y1",
                SurfaceType::Wall,
                [
                    point(0.0, 1.0, 0.0),
                    point(1.0, 1.0, 0.0),
                    point(1.0, 1.0, 1.0),
                    point(0.0, 1.0, 1.0),
                ],
            ),
        ]
    }

    fn surface(id: u32, name: &str, surface_type: SurfaceType, vertices: [Point3; 4]) -> Surface {
        Surface {
            id: SurfaceId(id),
            name: NormalizedName::new(name),
            surface_type,
            construction: ConstructionId(0),
            zone: ZoneId(0),
            outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
            outside_boundary_condition_object: None,
            sun_exposure: ep_model::SunExposure::SunExposed,
            wind_exposure: ep_model::WindExposure::WindExposed,
            view_factor_to_ground: AutoOrNumber::AutoCalculate,
            vertices: vertices.to_vec(),
        }
    }

    fn point(x_m: f64, y_m: f64, z_m: f64) -> Point3 {
        Point3 { x_m, y_m, z_m }
    }
}
