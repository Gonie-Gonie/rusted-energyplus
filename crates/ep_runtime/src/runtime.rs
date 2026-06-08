//! Runtime state, execution-plan shells, and first trace helpers.

use crate::{OutputSeries, ResultStore, RuntimeOutputRegistry};
use ep_model::{
    AutoOrNumber, AutosizeOrNumber, BranchId, BranchListId, ConstructionId, IdealLoadsAirSystem,
    IdealLoadsAirSystemId, LoopId, MaterialId, MaterialSurfaceRoughness, NodeId, NormalizedName,
    OtherEquipment, OutputHandle, OutsideBoundaryCondition, PlantBranchComponent, PlantLoop,
    Point3, RunPeriod, RunPeriodId, ScheduleCompactSegment, ScheduleId, SimulationModel,
    SiteLocation, SunExposure, Surface, SurfaceId, SurfaceType, TypedModel, Zone,
    ZoneEquipmentConnection, ZoneId, ZoneThermostatId,
};
use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::path::Path;

const AIR_DENSITY_KG_PER_M3: f64 = 1.2;
const AIR_SPECIFIC_HEAT_J_PER_KG_K: f64 = 1006.0;
const SECONDS_PER_HOUR: f64 = 3600.0;
const ENERGYPLUS_ZONE_INITIAL_TEMP_C: f64 = 23.0;
const DEFAULT_RUN_PERIOD_YEAR: u32 = 2013;
const DEFAULT_SOLAR_GROUND_REFLECTANCE: f64 = 0.2;
const DEFAULT_MATERIAL_THERMAL_ABSORPTANCE: f64 = 0.9;
const DEFAULT_MATERIAL_SOLAR_ABSORPTANCE: f64 = 0.7;
const EXTERIOR_LONGWAVE_COEFFICIENT_W_PER_M2_K: f64 = 4.0;
const EXTERIOR_SOLAR_FORCING_THRESHOLD_W_PER_M2: f64 = 300.0;
const EXTERIOR_RAIN_FALLBACK_DEPTH_MM: f64 = 2.0;
const STEFAN_BOLTZMANN_W_PER_M2_K4: f64 = 5.6697e-8;
const KELVIN_OFFSET: f64 = 273.15;
const ENERGYPLUS_SUN_IS_UP_COS_ZENITH: f64 = 0.00001;
const ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS: usize = 20;
const ENERGYPLUS_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K: f64 = 5.0;
const ENERGYPLUS_INITIAL_CONVECTION_COEFFICIENT_W_PER_M2_K: f64 = 3.076;
const ENERGYPLUS_LOW_CONVECTION_LIMIT_W_PER_M2_K: f64 = 0.1;
const ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K: f64 = 1000.0;
const ENERGYPLUS_QUICK_CONDUCTION_CROSS_THRESHOLD_W_PER_M2_K: f64 = 0.01;

/// EnergyPlus `SensedNodeFlagValue` used for unset node temperature setpoints.
pub const NODE_TEMPERATURE_SETPOINT_SENTINEL_C: f64 = -999.0;
/// Source map that owns node-state output registration and update paths.
pub const NODE_STATE_SOURCE_MAP_PATH: &str = "docs/src/porting-map/node-state-source-map.md";
/// Timestamp rule for the diagnostic node-state projection.
pub const NODE_STATE_TIMESTAMP_RULE: &str =
    "hour-ending hourly samples aligned to the run-period time axis";
/// Warmup handling rule for the diagnostic node-state projection.
pub const NODE_STATE_WARMUP_RULE: &str =
    "EnergyPlus warmup samples are not represented in this diagnostic projection";
/// Sentinel handling rule for excluded node setpoint output.
pub const NODE_STATE_SENTINEL_RULE: &str = "System Node Setpoint Temperature remains excluded; EnergyPlus SensedNodeFlagValue (-999) is represented as None";
/// Node output variable excluded until setpoint ownership and sentinel filtering are ported.
pub const NODE_STATE_EXCLUDED_SETPOINT_VARIABLE: &str = "System Node Setpoint Temperature";
/// Source map that owns plant diagnostic output registration and future update paths.
pub const PLANT_STATE_SOURCE_MAP_PATH: &str = "docs/src/porting-map/plant-source-map.md";
/// Timestamp rule for the diagnostic plant-state projection.
pub const PLANT_STATE_TIMESTAMP_RULE: &str =
    "hour-ending hourly samples aligned to the plant diagnostic case time axis";
/// Warmup handling rule for the diagnostic plant-state projection.
pub const PLANT_STATE_WARMUP_RULE: &str =
    "EnergyPlus warmup samples are not represented in this diagnostic projection";
/// Sizing/design-day boundary for the diagnostic plant-state projection.
pub const PLANT_STATE_SIZING_RULE: &str = "PlantLoop sizing-period baseline rows remain diagnostic-only until plant loop algorithms are ported";

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
    /// Evaluate one zone thermostat control.
    EvaluateZoneThermostat(ZoneThermostatId),
    /// Solve one zone.
    SolveZone(ZoneId),
    /// Evaluate one IdealLoads air system assigned to a zone.
    EvaluateIdealLoadsAirSystem(IdealLoadsAirSystemId),
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

    let mut zone_steps = Vec::new();
    for zone in &model.typed.zones {
        zone_steps.extend(
            model
                .graph
                .zone_thermostats
                .iter()
                .filter(|edge| edge.zone == zone.id)
                .map(|edge| ExecutionStep::EvaluateZoneThermostat(edge.thermostat)),
        );
        zone_steps.push(ExecutionStep::SolveZone(zone.id));
        zone_steps.extend(
            model
                .graph
                .zone_ideal_loads
                .iter()
                .filter(|edge| edge.zone == zone.id)
                .map(|edge| {
                    ExecutionStep::EvaluateIdealLoadsAirSystem(edge.ideal_loads_air_system)
                }),
        );
    }

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
                steps: RuntimeOutputRegistry::from_model(model)
                    .outputs()
                    .iter()
                    .map(|output| ExecutionStep::WriteOutput(output.handle))
                    .collect(),
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
    /// Sum of opaque surface conductance for this zone in W/K.
    pub opaque_surface_conductance_w_per_k: f64,
    /// Current opaque surface heat gain to the zone in W.
    pub opaque_surface_heat_gain_w: f64,
    /// EnergyPlus `SumHA`: inside convection conductance sum in W/K.
    pub sum_ha_w_per_k: f64,
    /// EnergyPlus `SumHATsurf`: inside convection temperature sum in W.
    pub sum_hat_surf_w: f64,
    /// EnergyPlus `SumHATref`: reference-air convection temperature sum in W.
    pub sum_hat_ref_w: f64,
    /// EnergyPlus zone-air temperature coefficient snapshot for diagnostics.
    pub zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients,
}

/// EnergyPlus zone-air temperature coefficient snapshot.
///
/// These fields mirror the predictor/corrector coefficient names in
/// `ZoneTempPredictorCorrector.cc`. They are diagnostic state until the full
/// zone-air predictor is wired into the heat-balance timestep.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoneAirTemperatureCoefficients {
    /// EnergyPlus `TempDepCoef` in W/K.
    pub temp_dependent_coefficient_w_per_k: f64,
    /// EnergyPlus `TempIndCoef` in W.
    pub temp_independent_coefficient_w: f64,
    /// EnergyPlus `AirPowerCap = C_air / dt` in W/K.
    pub air_power_cap_w_per_k: f64,
    /// EnergyPlus third-order `TempHistoryTerm` in W.
    pub third_order_history_term_w: f64,
    /// EnergyPlus third-order `tempDepLoad` in W/K.
    pub third_order_temp_dependent_load_w_per_k: f64,
    /// EnergyPlus third-order `tempIndLoad` in W.
    pub third_order_temp_independent_load_w: f64,
}

impl ZoneAirTemperatureCoefficients {
    const ZERO: Self = Self {
        temp_dependent_coefficient_w_per_k: 0.0,
        temp_independent_coefficient_w: 0.0,
        air_power_cap_w_per_k: 0.0,
        third_order_history_term_w: 0.0,
        third_order_temp_dependent_load_w_per_k: 0.0,
        third_order_temp_independent_load_w: 0.0,
    };
}

/// Surface CTF coefficients and history constants.
#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceCtfState {
    /// CTF outside/X coefficient at time zero in W/m2-K.
    pub outside_0_w_per_m2_k: f64,
    /// CTF cross/Y coefficient at time zero in W/m2-K.
    pub cross_0_w_per_m2_k: f64,
    /// CTF inside/Z coefficient at time zero in W/m2-K.
    pub inside_0_w_per_m2_k: f64,
    /// Inside CTF history constant part in W/m2.
    pub const_in_part_w_per_m2: f64,
    /// Outside CTF history constant part in W/m2.
    pub const_out_part_w_per_m2: f64,
    /// CTF outside/X history coefficients in W/m2-K.
    pub outside_history_w_per_m2_k: Vec<f64>,
    /// CTF cross/Y history coefficients in W/m2-K.
    pub cross_history_w_per_m2_k: Vec<f64>,
    /// CTF inside/Z history coefficients in W/m2-K.
    pub inside_history_w_per_m2_k: Vec<f64>,
    /// CTF flux history coefficients.
    pub flux_history: Vec<f64>,
    /// Previous outside face temperature history in C.
    pub outside_temperature_history_c: Vec<f64>,
    /// Previous inside face temperature history in C.
    pub inside_temperature_history_c: Vec<f64>,
    /// Previous outside conduction flux history in W/m2.
    pub outside_flux_history_w_per_m2: Vec<f64>,
    /// Previous inside conduction flux history in W/m2.
    pub inside_flux_history_w_per_m2: Vec<f64>,
}

/// Per-construction CTF coefficient row used to seed diagnostic surface histories.
#[derive(Clone, Debug, PartialEq)]
pub struct ConstructionCtfCoefficientOverride {
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// EnergyPlus CTF time/history index. Time zero is the current coefficient row.
    pub time_index: usize,
    /// CTF outside/X coefficient in W/m2-K.
    pub outside_w_per_m2_k: f64,
    /// CTF cross/Y coefficient in W/m2-K.
    pub cross_w_per_m2_k: f64,
    /// CTF inside/Z coefficient in W/m2-K.
    pub inside_w_per_m2_k: f64,
    /// CTF flux coefficient for history rows.
    pub flux: Option<f64>,
}

/// Inputs for the EnergyPlus CTF inside-face temperature balance subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfInsideFaceBalanceInput {
    /// Reference zone air temperature used by inside convection in C.
    pub reference_air_temperature_c: f64,
    /// Inside convection coefficient in W/m2-K.
    pub inside_convection_coefficient_w_per_m2_k: f64,
    /// Previous inside-face temperature from the current inside-surface iteration in C.
    pub previous_inside_face_temperature_c: f64,
    /// Net inside radiant/source term in W/m2 from EnergyPlus `SurfTempTerm` inputs.
    pub net_inside_source_w_per_m2: f64,
}

/// Inputs for the EnergyPlus CTF outside-face environmental balance subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfOutsideFaceBalanceInput {
    /// Outdoor air temperature used by exterior convection in C.
    pub outdoor_air_temperature_c: f64,
    /// Linearized outside radiant temperature in C.
    pub radiant_temperature_c: f64,
    /// Outside convection coefficient in W/m2-K.
    pub outside_convection_coefficient_w_per_m2_k: f64,
    /// Linearized outside radiation coefficient in W/m2-K.
    pub outside_radiation_coefficient_w_per_m2_k: f64,
    /// Shortwave/source term absorbed at the outside face in W/m2.
    pub absorbed_outside_source_w_per_m2: f64,
}

/// Inputs for the EnergyPlus quick-conduction outside-face balance subset.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CtfOutsideQuickConductionBalanceInput {
    /// Outside environmental/source balance inputs.
    pub environmental: CtfOutsideFaceBalanceInput,
    /// Reference zone air temperature used by inside convection in C.
    pub reference_air_temperature_c: f64,
    /// Inside convection coefficient in W/m2-K.
    pub inside_convection_coefficient_w_per_m2_k: f64,
    /// Net inside radiant/source term in W/m2 from EnergyPlus `SurfTempTerm` inputs.
    pub net_inside_source_w_per_m2: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct QuickOutsideConductionContext {
    reference_air_temperature_c: f64,
    inside_convection_coefficient_w_per_m2_k: f64,
    net_inside_source_w_per_m2: f64,
    use_doe2_outside_convection: bool,
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
    /// Outside boundary condition.
    pub outside_boundary_condition: OutsideBoundaryCondition,
    /// Optional outside boundary object name.
    pub outside_boundary_condition_object_name: Option<String>,
    /// Resolved adjacent surface for interzone surface boundaries.
    pub outside_boundary_target_surface_id: Option<SurfaceId>,
    /// Resolved adjacent zone for interzone surface, zone, or space boundaries.
    pub outside_boundary_target_zone_id: Option<ZoneId>,
    /// Resolved construction ID.
    pub construction_id: ConstructionId,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// Outside layer material ID.
    pub outside_layer_material_id: MaterialId,
    /// EnergyPlus-normalized outside layer material name.
    pub outside_layer_material_name: String,
    /// Outside layer surface roughness used by EnergyPlus exterior convection.
    pub outside_layer_roughness: MaterialSurfaceRoughness,
    /// Surface area in square meters.
    pub area_m2: f64,
    /// Surface tilt in degrees using EnergyPlus orientation conventions.
    pub tilt_deg: f64,
    /// Area-normalized thermal resistance in m2-K/W.
    pub thermal_resistance_m2_k_per_w: f64,
    /// Area-normalized heat capacity in J/m2-K when available.
    pub heat_capacity_j_per_m2_k: Option<f64>,
    /// Outside layer thermal absorptance used by exterior diagnostic forcing.
    pub thermal_absorptance: f64,
    /// Outside layer solar absorptance used by exterior diagnostic forcing.
    pub solar_absorptance: f64,
    /// Surface conductance in W/K.
    pub conductance_w_per_k: f64,
    /// Current inside convection coefficient in W/m2-K.
    pub inside_convection_coefficient_w_per_m2_k: f64,
    /// EnergyPlus `SurfQdotRadIntGainsInPerArea` source term in W/m2.
    pub inside_radiant_internal_gain_w_per_m2: f64,
    /// EnergyPlus `SurfOpaqQRadSWInAbs` absorbed inside shortwave term in W/m2.
    pub inside_shortwave_absorbed_w_per_m2: f64,
    /// EnergyPlus `SurfQAdditionalHeatSourceInside` term in W/m2.
    pub inside_additional_heat_source_w_per_m2: f64,
    /// EnergyPlus `SurfQdotRadHVACInPerArea` source term in W/m2.
    pub inside_radiant_hvac_w_per_m2: f64,
    /// EnergyPlus `SurfQdotRadNetLWInPerArea` source term in W/m2.
    pub inside_net_longwave_w_per_m2: f64,
    /// Surface CTF coefficients and history constants.
    pub ctf: SurfaceCtfState,
    /// Current opaque heat transfer to the owning zone in W.
    pub heat_gain_to_zone_w: f64,
    /// Current inside face temperature in C.
    pub inside_face_temperature_c: f64,
    /// Current outside face temperature in C.
    pub outside_face_temperature_c: f64,
}

/// Inputs for advancing the first heat-balance timestep shell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceStepInput {
    /// Outdoor dry-bulb temperature in C for this timestep.
    pub outdoor_dry_bulb_c: f64,
    /// EnergyPlus-style hour ending, 1-24.
    pub hour_ending: u32,
    /// Timestep duration in seconds.
    pub timestep_seconds: f64,
}

/// Zone-air temperature algorithm used by diagnostic heat-balance traces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceZoneAirAlgorithm {
    /// Existing simplified analytical diagnostic shell.
    SimplifiedAnalytical,
    /// Experimental EnergyPlus analytical predictor path for diagnostics.
    EnergyPlusAnalyticalProbe,
    /// Experimental EnergyPlus analytical correction after the surface pass.
    EnergyPlusAnalyticalSurfaceFirstProbe,
    /// Experimental analytical correction with a same-timestep surface rebalance.
    EnergyPlusAnalyticalCoupledProbe,
    /// Experimental coupled analytical path using previous inside surface temperature for outdoor CTF boundary solves.
    EnergyPlusAnalyticalCoupledPreviousInsideProbe,
    /// Experimental previous-inside coupled path using EnergyPlus quick-conduction outside face solves.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe,
    /// Experimental quick-outside path using EnergyPlus DOE-2 exterior convection.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe,
    /// Experimental quick-outside path with a grey interior longwave exchange probe.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe,
    /// Experimental quick-outside path combining DOE-2 exterior convection and grey interior longwave exchange.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe,
    /// Experimental coupled analytical path using previous inside surface temperature for outdoor and adiabatic boundary solves.
    EnergyPlusAnalyticalCoupledPreviousBoundaryProbe,
    /// Experimental EnergyPlus third-order predictor path for diagnostics.
    EnergyPlusThirdOrderProbe,
}

/// Initial CTF temperature/flux history seeding used by diagnostic heat-balance traces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceCtfInitialHistoryPolicy {
    /// Existing Rust diagnostic seed: current boundary temperature and steady U-value flux.
    BoundaryTemperatureAndUValue,
    /// EnergyPlus 26.1 style seed: SurfInitialTemp histories and zero flux histories.
    EnergyPlusSurfInitial,
}

/// Options for the heat-balance zone-air diagnostic trace.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceSimulationOptions {
    /// Number of hourly weather samples to execute.
    pub sample_count: usize,
    /// Initial zone mean air temperature in C.
    pub initial_zone_air_temperature_c: f64,
    /// Optional run-period warmup loop.
    pub warmup: HeatBalanceWarmupOptions,
    /// Zone-air temperature algorithm for diagnostic probes.
    pub zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    /// Number of inside/outside surface-balance passes per zone timestep.
    pub surface_iteration_count: u32,
    /// Initial CTF temperature/flux history seeding policy.
    pub ctf_initial_history_policy: HeatBalanceCtfInitialHistoryPolicy,
}

/// Warmup settings for heat-balance diagnostic traces.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceWarmupOptions {
    /// Whether to execute a warmup loop before reported samples are recorded.
    pub enabled: bool,
    /// Minimum number of repeated warmup days.
    pub minimum_days: u32,
    /// Maximum number of repeated warmup days.
    pub maximum_days: u32,
    /// Zone end-state convergence tolerance in delta C.
    pub temperature_convergence_tolerance_delta_c: f64,
}

impl HeatBalanceWarmupOptions {
    /// Creates disabled warmup settings.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            minimum_days: 0,
            maximum_days: 0,
            temperature_convergence_tolerance_delta_c: 0.0,
        }
    }
}

/// Summary of the executed heat-balance warmup loop.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceWarmupSummary {
    /// Whether warmup was requested.
    pub enabled: bool,
    /// Number of warmup days actually executed.
    pub day_count: u32,
    /// Number of timesteps executed during warmup.
    pub timestep_count: usize,
    /// Number of weather hours repeated for one warmup day.
    pub hours_per_day: usize,
    /// Whether the repeated-day end state converged before max days.
    pub converged: bool,
    /// Final max zone air temperature delta between repeated-day end states.
    pub final_max_zone_temperature_delta_c: f64,
}

impl HeatBalanceWarmupSummary {
    /// Creates a disabled warmup summary.
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            enabled: false,
            day_count: 0,
            timestep_count: 0,
            hours_per_day: 0,
            converged: false,
            final_max_zone_temperature_delta_c: 0.0,
        }
    }
}

impl HeatBalanceSimulationOptions {
    /// Creates options with a fixed hourly sample count.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            initial_zone_air_temperature_c: ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            warmup: HeatBalanceWarmupOptions::disabled(),
            zone_air_algorithm: HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            surface_iteration_count: 1,
            ctf_initial_history_policy:
                HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue,
        }
    }

    /// Creates options with a run-period warmup loop based on typed Building settings.
    #[must_use]
    pub fn hourly_samples_with_model_warmup(model: &SimulationModel, sample_count: usize) -> Self {
        let Some(building) = model.typed.building.as_ref() else {
            return Self::hourly_samples(sample_count);
        };
        let minimum_days = building.minimum_number_of_warmup_days;
        let maximum_days = building.maximum_number_of_warmup_days.max(minimum_days);
        Self {
            sample_count,
            initial_zone_air_temperature_c: ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            warmup: HeatBalanceWarmupOptions {
                enabled: maximum_days > 0,
                minimum_days,
                maximum_days,
                temperature_convergence_tolerance_delta_c: building
                    .temperature_convergence_tolerance_delta_c,
            },
            zone_air_algorithm: HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            surface_iteration_count: 1,
            ctf_initial_history_policy:
                HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue,
        }
    }

    /// Returns options with an explicit zone-air diagnostic algorithm.
    #[must_use]
    pub const fn with_zone_air_algorithm(
        mut self,
        zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    ) -> Self {
        self.zone_air_algorithm = zone_air_algorithm;
        self
    }

    /// Returns options with an elevated warmup minimum day count for diagnostics.
    #[must_use]
    pub fn with_warmup_minimum_days(mut self, minimum_days: u32) -> Self {
        if self.warmup.enabled {
            self.warmup.minimum_days = minimum_days;
            self.warmup.maximum_days = self.warmup.maximum_days.max(minimum_days);
        }
        self
    }

    /// Returns options with an explicit surface-balance iteration count.
    #[must_use]
    pub const fn with_surface_iteration_count(mut self, surface_iteration_count: u32) -> Self {
        self.surface_iteration_count = if surface_iteration_count == 0 {
            1
        } else {
            surface_iteration_count
        };
        self
    }

    /// Returns options with an explicit initial CTF history seed policy.
    #[must_use]
    pub const fn with_ctf_initial_history_policy(
        mut self,
        ctf_initial_history_policy: HeatBalanceCtfInitialHistoryPolicy,
    ) -> Self {
        self.ctf_initial_history_policy = ctf_initial_history_policy;
        self
    }
}

/// Summary for the heat-balance zone-air diagnostic trace.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSimulationSummary {
    /// Hourly output sample count.
    pub samples: usize,
    /// Number of executed zone timesteps.
    pub timestep_count: usize,
    /// Number of reported run-period zone timesteps.
    pub run_period_timestep_count: usize,
    /// Warmup execution summary.
    pub warmup: HeatBalanceWarmupSummary,
    /// Number of zones represented in the state.
    pub zone_count: usize,
    /// Number of surfaces represented in the state.
    pub surface_count: usize,
    /// Number of surface-balance passes used per zone timestep.
    pub surface_iteration_count: u32,
    /// Initial CTF temperature/flux history seeding policy.
    pub ctf_initial_history_policy: HeatBalanceCtfInitialHistoryPolicy,
}

/// Result of the heat-balance zone-air diagnostic trace.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSimulation {
    /// Final heat-balance state.
    pub state: HeatBalanceState,
    /// Native output results.
    pub results: ResultStore,
    /// Trace summary.
    pub summary: HeatBalanceSimulationSummary,
}

struct SurfaceHeatBalanceTrace {
    surface_id: SurfaceId,
    surface_name: String,
    inside_face_temperature_c: Vec<f64>,
    outside_face_temperature_c: Vec<f64>,
    inside_conduction_rate_w: Vec<f64>,
    inside_conduction_gain_rate_w: Vec<f64>,
    inside_conduction_loss_rate_w: Vec<f64>,
    inside_conduction_rate_per_area_w_per_m2: Vec<f64>,
    outside_conduction_rate_w: Vec<f64>,
    outside_conduction_gain_rate_w: Vec<f64>,
    outside_conduction_loss_rate_w: Vec<f64>,
    outside_conduction_rate_per_area_w_per_m2: Vec<f64>,
    heat_storage_rate_w: Vec<f64>,
}

/// Appends diagnostic surface incident solar radiation series for sun-exposed
/// surfaces with a declared site location.
///
/// The calculation is intentionally a forcing diagnostic: direct normal
/// radiation is projected with EnergyPlus-style weather timestep interpolation
/// and shadowing-period solar position coefficients. Diffuse sky remains
/// isotropic, and ground reflection uses a fixed default reflectance. It is not
/// a full EnergyPlus solar distribution or shadowing claim.
pub fn append_surface_incident_solar_radiation_series(
    results: &mut ResultStore,
    model: &SimulationModel,
    weather_records: &[EpwRecord],
    sample_count: usize,
) -> usize {
    let Some(site) = model.typed.site.as_ref() else {
        return 0;
    };
    if weather_records.is_empty() || sample_count == 0 {
        return 0;
    }

    let mut added = 0;
    let mut handle_index = results
        .series
        .iter()
        .map(|series| series.handle.0)
        .max()
        .map_or(0, |handle| handle + 1);

    let zone_steps_per_hour = model.typed.timestep.number_of_timesteps_per_hour.max(1);
    for surface in &model.typed.surfaces {
        if surface.sun_exposure != SunExposure::SunExposed
            || surface.outside_boundary_condition != OutsideBoundaryCondition::Outdoors
        {
            continue;
        }
        let values = weather_records
            .iter()
            .enumerate()
            .take(sample_count)
            .map(|(record_index, _record)| {
                surface_incident_solar_radiation_hourly_average_w_per_m2(
                    surface,
                    site,
                    weather_records,
                    record_index,
                    zone_steps_per_hour,
                )
            })
            .collect::<Vec<_>>();
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: surface.name.0.clone(),
            variable_name: "Surface Outside Face Incident Solar Radiation Rate per Area"
                .to_string(),
            units: "W/m2".to_string(),
            values,
        });
        handle_index += 1;
        added += 1;
    }

    added
}

/// Diagnostic role assigned to one plant equipment projection row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlantEquipmentRole {
    /// Pump component row.
    Pump,
    /// Purchased heating source row.
    PurchasedHeating,
    /// Plant load profile row.
    LoadProfile,
}

impl PlantEquipmentRole {
    /// Stable diagnostic label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Pump => "pump",
            Self::PurchasedHeating => "purchased-heating",
            Self::LoadProfile => "load-profile",
        }
    }
}

/// Options for the diagnostic PlantLoadProfile plant-state projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PlantStateProjectionOptions {
    /// Number of hourly samples to write.
    pub sample_count: usize,
    /// Fallback plant supply-side cooling demand in W.
    pub default_cooling_demand_w: f64,
    /// Fallback plant supply-side heating demand in W.
    pub default_heating_demand_w: f64,
    /// Fallback plant supply inlet mass flow rate in kg/s.
    pub default_inlet_mass_flow_rate_kg_per_s: f64,
    /// Fallback plant supply inlet temperature in C.
    pub default_inlet_temperature_c: f64,
    /// Fallback plant supply outlet temperature in C.
    pub default_outlet_temperature_c: f64,
    /// Fallback pump electricity rate in W.
    pub default_pump_electricity_rate_w: f64,
    /// Fallback purchased heating water rate in W.
    pub default_district_heating_rate_w: f64,
    /// Fallback load profile heat transfer rate in W.
    pub default_load_profile_heat_transfer_rate_w: f64,
}

impl PlantStateProjectionOptions {
    /// Creates options with a fixed hourly sample count.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            default_cooling_demand_w: 500.0,
            default_heating_demand_w: 1_000.0,
            default_inlet_mass_flow_rate_kg_per_s: 1.0,
            default_inlet_temperature_c: 60.0,
            default_outlet_temperature_c: 65.0,
            default_pump_electricity_rate_w: 250.0,
            default_district_heating_rate_w: 1_000.0,
            default_load_profile_heat_transfer_rate_w: 1_000.0,
        }
    }
}

/// Evidence policy attached to diagnostic plant-state projection artifacts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlantStateProjectionEvidencePolicy {
    /// Source map that owns the EnergyPlus routine and field mapping.
    pub source_map_path: &'static str,
    /// Timestamp alignment rule for samples written by the projection.
    pub timestamp_rule: &'static str,
    /// Warmup handling rule for samples written by the projection.
    pub warmup_rule: &'static str,
    /// Sizing/design-day boundary for the projection.
    pub sizing_rule: &'static str,
}

impl PlantStateProjectionEvidencePolicy {
    /// Returns the diagnostic-only plant-state evidence policy.
    #[must_use]
    pub const fn diagnostic() -> Self {
        Self {
            source_map_path: PLANT_STATE_SOURCE_MAP_PATH,
            timestamp_rule: PLANT_STATE_TIMESTAMP_RULE,
            warmup_rule: PLANT_STATE_WARMUP_RULE,
            sizing_rule: PLANT_STATE_SIZING_RULE,
        }
    }
}

/// Runtime scalar state for one plant loop diagnostic row set.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantLoopState {
    /// Plant loop ID.
    pub loop_id: LoopId,
    /// EnergyPlus-normalized plant loop key.
    pub loop_name: String,
    /// Supply-side inlet node name.
    pub supply_inlet_node_name: String,
    /// Supply-side outlet node name.
    pub supply_outlet_node_name: String,
    /// Current supply-side cooling demand in W.
    pub supply_side_cooling_demand_w: f64,
    /// Current supply-side heating demand in W.
    pub supply_side_heating_demand_w: f64,
    /// Current supply-side inlet mass flow rate in kg/s.
    pub supply_side_inlet_mass_flow_rate_kg_per_s: f64,
    /// Current supply-side inlet temperature in C.
    pub supply_side_inlet_temperature_c: f64,
    /// Current supply-side outlet temperature in C.
    pub supply_side_outlet_temperature_c: f64,
}

/// Runtime scalar state for one plant diagnostic equipment row.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantEquipmentState {
    /// Component object type.
    pub object_type: String,
    /// EnergyPlus-normalized equipment key.
    pub equipment_name: String,
    /// Diagnostic role.
    pub role: PlantEquipmentRole,
    /// Projected equipment output value.
    pub output_rate_w: f64,
}

/// Diagnostic plant state store.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantStateStore {
    /// Plant loop states in typed-loop order.
    pub loops: Vec<PlantLoopState>,
    /// Plant equipment states in plant graph order.
    pub equipment: Vec<PlantEquipmentState>,
}

impl PlantStateStore {
    /// Number of stored plant loops.
    #[must_use]
    pub fn loop_count(&self) -> usize {
        self.loops.len()
    }

    /// Number of stored plant equipment rows.
    #[must_use]
    pub fn equipment_count(&self) -> usize {
        self.equipment.len()
    }
}

/// One resolved plant loop represented by the projection.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantStateProjectionLoop {
    /// Plant loop ID.
    pub loop_id: LoopId,
    /// EnergyPlus-normalized loop key.
    pub loop_name: String,
    /// Supply-side inlet node name.
    pub supply_inlet_node_name: String,
    /// Supply-side outlet node name.
    pub supply_outlet_node_name: String,
}

/// One resolved plant equipment represented by the projection.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantStateProjectionEquipment {
    /// Component object type.
    pub object_type: String,
    /// EnergyPlus-normalized equipment key.
    pub equipment_name: String,
    /// Diagnostic role.
    pub role: PlantEquipmentRole,
}

/// Summary for the diagnostic PlantLoadProfile plant-state projection.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantStateProjectionSummary {
    /// Hourly output sample count.
    pub samples: usize,
    /// Number of plant loops represented.
    pub loop_count: usize,
    /// Number of equipment rows represented.
    pub equipment_count: usize,
    /// Number of output series written.
    pub series_count: usize,
    /// Diagnostic evidence policy attached to output artifacts.
    pub evidence_policy: PlantStateProjectionEvidencePolicy,
    /// Resolved loops in output order.
    pub loops: Vec<PlantStateProjectionLoop>,
    /// Resolved equipment rows in output order.
    pub equipment: Vec<PlantStateProjectionEquipment>,
}

/// Result of the diagnostic PlantLoadProfile plant-state projection.
#[derive(Clone, Debug, PartialEq)]
pub struct PlantStateProjection {
    /// Final diagnostic plant state.
    pub state: PlantStateStore,
    /// Native output results.
    pub results: ResultStore,
    /// Projection summary.
    pub summary: PlantStateProjectionSummary,
}

/// Role assigned to a node-state projection row.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeStateRole {
    /// Zone inlet or IdealLoads supply node.
    Supply,
    /// Zone air node.
    ZoneAir,
    /// Zone return node.
    ReturnAir,
}

impl NodeStateRole {
    /// Stable diagnostic label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Supply => "supply",
            Self::ZoneAir => "zone-air",
            Self::ReturnAir => "return-air",
        }
    }
}

/// Options for the diagnostic IdealLoads node-state projection.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeStateProjectionOptions {
    /// Number of hourly samples to write.
    pub sample_count: usize,
    /// Fallback zone-air temperature in C.
    pub default_zone_air_temperature_c: f64,
    /// Fallback zone-air humidity ratio in kgWater/kgDryAir.
    pub default_zone_air_humidity_ratio: f64,
    /// Fallback supply-air temperature in C when no IdealLoads value exists.
    pub default_supply_air_temperature_c: f64,
    /// Fallback supply-air humidity ratio in kgWater/kgDryAir.
    pub default_supply_air_humidity_ratio: f64,
    /// Fallback supply-air mass flow rate in kg/s when no design flow exists.
    pub default_supply_air_mass_flow_rate_kg_per_s: f64,
}

impl NodeStateProjectionOptions {
    /// Creates options with a fixed hourly sample count.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            default_zone_air_temperature_c: ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            default_zone_air_humidity_ratio: 0.008,
            default_supply_air_temperature_c: 50.0,
            default_supply_air_humidity_ratio: 0.0156,
            default_supply_air_mass_flow_rate_kg_per_s: 0.5,
        }
    }
}

/// Evidence policy attached to diagnostic node-state projection artifacts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NodeStateProjectionEvidencePolicy {
    /// Source map that owns the EnergyPlus routine and field mapping.
    pub source_map_path: &'static str,
    /// Timestamp alignment rule for samples written by the projection.
    pub timestamp_rule: &'static str,
    /// Warmup handling rule for samples written by the projection.
    pub warmup_rule: &'static str,
    /// Sentinel filtering rule for future setpoint sampling.
    pub sentinel_rule: &'static str,
    /// Output variable intentionally excluded by the sentinel rule.
    pub excluded_variable: &'static str,
}

impl NodeStateProjectionEvidencePolicy {
    /// Returns the diagnostic-only v0.12 node-state evidence policy.
    #[must_use]
    pub const fn diagnostic() -> Self {
        Self {
            source_map_path: NODE_STATE_SOURCE_MAP_PATH,
            timestamp_rule: NODE_STATE_TIMESTAMP_RULE,
            warmup_rule: NODE_STATE_WARMUP_RULE,
            sentinel_rule: NODE_STATE_SENTINEL_RULE,
            excluded_variable: NODE_STATE_EXCLUDED_SETPOINT_VARIABLE,
        }
    }
}

/// Runtime scalar state for one air-side node.
#[derive(Clone, Debug, PartialEq)]
pub struct AirNodeState {
    /// Resolved typed node ID.
    pub node_id: NodeId,
    /// EnergyPlus-normalized node key.
    pub node_name: String,
    /// Current node temperature in C.
    pub temperature_c: f64,
    /// Current node humidity ratio in kgWater/kgDryAir.
    pub humidity_ratio: f64,
    /// Current node mass flow rate in kg/s.
    pub mass_flow_rate_kg_per_s: f64,
    /// Optional node temperature setpoint in C.
    pub temperature_setpoint_c: Option<f64>,
}

/// Diagnostic air-side node state store.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeStateStore {
    /// Air-side node states in typed-node order.
    pub air_nodes: Vec<AirNodeState>,
}

impl NodeStateStore {
    /// Initializes one diagnostic air-node state for each typed model node.
    #[must_use]
    pub fn from_typed_model(
        model: &TypedModel,
        default_temperature_c: f64,
        default_humidity_ratio: f64,
    ) -> Self {
        Self {
            air_nodes: model
                .nodes
                .iter()
                .map(|node| AirNodeState {
                    node_id: node.id,
                    node_name: node.name.0.clone(),
                    temperature_c: default_temperature_c,
                    humidity_ratio: default_humidity_ratio,
                    mass_flow_rate_kg_per_s: 0.0,
                    temperature_setpoint_c: None,
                })
                .collect(),
        }
    }

    /// Number of stored air nodes.
    #[must_use]
    pub fn len(&self) -> usize {
        self.air_nodes.len()
    }

    /// Returns true when no air nodes are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.air_nodes.is_empty()
    }

    /// Finds an air-node state by typed node ID.
    #[must_use]
    pub fn find_by_id(&self, node_id: NodeId) -> Option<&AirNodeState> {
        self.air_nodes.iter().find(|node| node.node_id == node_id)
    }

    /// Finds an air-node state by EnergyPlus key.
    #[must_use]
    pub fn find_by_key(&self, key: &str) -> Option<&AirNodeState> {
        let normalized = NormalizedName::new(key);
        self.air_nodes
            .iter()
            .find(|node| node.node_name == normalized.0)
    }

    fn find_mut_by_id(&mut self, node_id: NodeId) -> Option<&mut AirNodeState> {
        self.air_nodes
            .iter_mut()
            .find(|node| node.node_id == node_id)
    }
}

/// Converts an EnergyPlus node temperature setpoint scalar into diagnostic state.
#[must_use]
pub fn node_temperature_setpoint_from_energyplus(value_c: f64) -> Option<f64> {
    if (value_c - NODE_TEMPERATURE_SETPOINT_SENTINEL_C).abs() < 1.0e-9 {
        None
    } else {
        Some(value_c)
    }
}

/// One resolved node represented by the node-state projection.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeStateProjectionNode {
    /// Resolved typed node ID.
    pub node_id: NodeId,
    /// EnergyPlus-normalized node key.
    pub node_name: String,
    /// Diagnostic role for the node.
    pub role: NodeStateRole,
}

/// Summary for the diagnostic IdealLoads node-state projection.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeStateProjectionSummary {
    /// Hourly output sample count.
    pub samples: usize,
    /// Number of unique nodes represented.
    pub node_count: usize,
    /// Number of output series written.
    pub series_count: usize,
    /// Number of air nodes initialized in the runtime state store.
    pub state_node_count: usize,
    /// Diagnostic evidence policy attached to output artifacts.
    pub evidence_policy: NodeStateProjectionEvidencePolicy,
    /// Resolved nodes in output order.
    pub nodes: Vec<NodeStateProjectionNode>,
}

/// Result of the diagnostic IdealLoads node-state projection.
#[derive(Clone, Debug, PartialEq)]
pub struct NodeStateProjection {
    /// Final diagnostic node state.
    pub state: NodeStateStore,
    /// Native output results.
    pub results: ResultStore,
    /// Projection summary.
    pub summary: NodeStateProjectionSummary,
}

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

/// Writes a deterministic diagnostic projection of IdealLoads-related node
/// state outputs.
///
/// This function intentionally does not claim EnergyPlus algorithm parity. It
/// maps the typed air-side node graph to native `ResultStore` series so the
/// port can exercise NodeList expansion, node output registration, and result
/// artifact plumbing before the full HVAC manager is ported.
pub fn simulate_ideal_loads_node_state_projection(
    model: &SimulationModel,
    options: NodeStateProjectionOptions,
) -> Result<NodeStateProjection, RuntimeError> {
    let mut state = NodeStateStore::from_typed_model(
        &model.typed,
        options.default_zone_air_temperature_c,
        options.default_zone_air_humidity_ratio,
    );
    let mut projected_nodes = Vec::new();

    for connection in &model.typed.zone_equipment_connections {
        let ideal_loads = ideal_loads_for_connection(&model.typed, connection);
        let supply_temperature_c = ideal_loads
            .map(|system| system.maximum_heating_supply_air_temperature_c)
            .unwrap_or(options.default_supply_air_temperature_c);
        let supply_humidity_ratio = ideal_loads
            .map(|system| system.maximum_heating_supply_air_humidity_ratio)
            .unwrap_or(options.default_supply_air_humidity_ratio);
        let supply_mass_flow_rate_kg_per_s = ideal_loads
            .and_then(ideal_loads_design_mass_flow_rate_kg_per_s)
            .unwrap_or(options.default_supply_air_mass_flow_rate_kg_per_s);

        let supply_nodes = connection
            .zone_air_inlet_node_or_nodelist_name
            .as_ref()
            .map(|name| resolve_node_or_nodelist(&model.typed, name))
            .unwrap_or_default();
        let supply_node_count = supply_nodes.len().max(1) as f64;
        for node_id in supply_nodes {
            if let Some(node_name) = node_name_for_id(&model.typed, node_id) {
                apply_node_state_update(
                    &mut state,
                    node_id,
                    supply_temperature_c,
                    supply_humidity_ratio,
                    supply_mass_flow_rate_kg_per_s / supply_node_count,
                );
                push_projected_node_assignment(
                    &mut projected_nodes,
                    ProjectedNodeAssignment {
                        node_id,
                        node_name,
                        role: NodeStateRole::Supply,
                    },
                );
            }
        }

        if let Some(zone_air_node_id) = model
            .typed
            .node_names
            .resolve(&connection.zone_air_node_name.0)
            && let Some(node_name) = node_name_for_id(&model.typed, zone_air_node_id)
        {
            apply_node_state_update(
                &mut state,
                zone_air_node_id,
                options.default_zone_air_temperature_c,
                options.default_zone_air_humidity_ratio,
                supply_mass_flow_rate_kg_per_s,
            );
            push_projected_node_assignment(
                &mut projected_nodes,
                ProjectedNodeAssignment {
                    node_id: zone_air_node_id,
                    node_name,
                    role: NodeStateRole::ZoneAir,
                },
            );
        }

        let return_nodes = connection
            .zone_return_air_node_or_nodelist_name
            .as_ref()
            .map(|name| resolve_node_or_nodelist(&model.typed, name))
            .unwrap_or_default();
        for node_id in return_nodes {
            if let Some(node_name) = node_name_for_id(&model.typed, node_id) {
                apply_node_state_update(
                    &mut state,
                    node_id,
                    options.default_zone_air_temperature_c,
                    options.default_zone_air_humidity_ratio,
                    supply_mass_flow_rate_kg_per_s,
                );
                push_projected_node_assignment(
                    &mut projected_nodes,
                    ProjectedNodeAssignment {
                        node_id,
                        node_name,
                        role: NodeStateRole::ReturnAir,
                    },
                );
            }
        }
    }

    if projected_nodes.is_empty() {
        return Err(RuntimeError::NoNodeStateProjectionNodes);
    }

    let mut results = ResultStore::new();
    let mut handle_index = 0_u32;
    for node in &projected_nodes {
        let Some(node_state) = state.find_by_id(node.node_id) else {
            continue;
        };
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &node.node_name,
            "System Node Temperature",
            "C",
            node_state.temperature_c,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &node.node_name,
            "System Node Humidity Ratio",
            "kgWater/kgDryAir",
            node_state.humidity_ratio,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &node.node_name,
            "System Node Mass Flow Rate",
            "kg/s",
            node_state.mass_flow_rate_kg_per_s,
            options.sample_count,
        );
    }

    Ok(NodeStateProjection {
        summary: NodeStateProjectionSummary {
            samples: options.sample_count,
            node_count: projected_nodes.len(),
            series_count: results.series.len(),
            state_node_count: state.len(),
            evidence_policy: NodeStateProjectionEvidencePolicy::diagnostic(),
            nodes: projected_nodes
                .iter()
                .map(|node| NodeStateProjectionNode {
                    node_id: node.node_id,
                    node_name: node.node_name.clone(),
                    role: node.role,
                })
                .collect(),
        },
        state,
        results,
    })
}

/// Writes a deterministic diagnostic projection of plant loop and first plant
/// equipment output rows.
///
/// This function intentionally does not claim EnergyPlus algorithm parity. It
/// maps the typed plant-loop graph and branch component order to native
/// `ResultStore` series so the port can exercise plant output registration and
/// artifact plumbing before plant loop manager algorithms are ported.
pub fn simulate_plant_state_projection(
    model: &SimulationModel,
    options: PlantStateProjectionOptions,
) -> Result<PlantStateProjection, RuntimeError> {
    if model.typed.plant_loops.is_empty() {
        return Err(RuntimeError::NoPlantStateProjectionLoops);
    }

    let loop_states: Vec<_> = model
        .typed
        .plant_loops
        .iter()
        .map(|plant_loop| plant_loop_state(model, plant_loop, options))
        .collect();
    let equipment_states = plant_equipment_states(model, options);

    let state = PlantStateStore {
        loops: loop_states,
        equipment: equipment_states,
    };

    let mut results = ResultStore::new();
    let mut handle_index = 0_u32;
    for loop_state in &state.loops {
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &loop_state.loop_name,
            "Plant Supply Side Cooling Demand Rate",
            "W",
            loop_state.supply_side_cooling_demand_w,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &loop_state.loop_name,
            "Plant Supply Side Heating Demand Rate",
            "W",
            loop_state.supply_side_heating_demand_w,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &loop_state.loop_name,
            "Plant Supply Side Inlet Mass Flow Rate",
            "kg/s",
            loop_state.supply_side_inlet_mass_flow_rate_kg_per_s,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &loop_state.loop_name,
            "Plant Supply Side Inlet Temperature",
            "C",
            loop_state.supply_side_inlet_temperature_c,
            options.sample_count,
        );
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &loop_state.loop_name,
            "Plant Supply Side Outlet Temperature",
            "C",
            loop_state.supply_side_outlet_temperature_c,
            options.sample_count,
        );
    }

    for equipment_state in &state.equipment {
        add_constant_output_series(
            &mut results,
            &mut handle_index,
            &equipment_state.equipment_name,
            plant_equipment_variable_name(equipment_state.role),
            "W",
            equipment_state.output_rate_w,
            options.sample_count,
        );
    }

    Ok(PlantStateProjection {
        summary: PlantStateProjectionSummary {
            samples: options.sample_count,
            loop_count: state.loop_count(),
            equipment_count: state.equipment_count(),
            series_count: results.series.len(),
            evidence_policy: PlantStateProjectionEvidencePolicy::diagnostic(),
            loops: state
                .loops
                .iter()
                .map(|loop_state| PlantStateProjectionLoop {
                    loop_id: loop_state.loop_id,
                    loop_name: loop_state.loop_name.clone(),
                    supply_inlet_node_name: loop_state.supply_inlet_node_name.clone(),
                    supply_outlet_node_name: loop_state.supply_outlet_node_name.clone(),
                })
                .collect(),
            equipment: state
                .equipment
                .iter()
                .map(|equipment| PlantStateProjectionEquipment {
                    object_type: equipment.object_type.clone(),
                    equipment_name: equipment.equipment_name.clone(),
                    role: equipment.role,
                })
                .collect(),
        },
        state,
        results,
    })
}

fn plant_loop_state(
    model: &SimulationModel,
    plant_loop: &PlantLoop,
    options: PlantStateProjectionOptions,
) -> PlantLoopState {
    PlantLoopState {
        loop_id: plant_loop.id,
        loop_name: plant_loop.name.0.clone(),
        supply_inlet_node_name: node_name_for_id(&model.typed, plant_loop.plant_side_inlet_node)
            .unwrap_or_else(|| format!("NODE {}", plant_loop.plant_side_inlet_node.0)),
        supply_outlet_node_name: node_name_for_id(&model.typed, plant_loop.plant_side_outlet_node)
            .unwrap_or_else(|| format!("NODE {}", plant_loop.plant_side_outlet_node.0)),
        supply_side_cooling_demand_w: options.default_cooling_demand_w,
        supply_side_heating_demand_w: options.default_heating_demand_w,
        supply_side_inlet_mass_flow_rate_kg_per_s: options.default_inlet_mass_flow_rate_kg_per_s,
        supply_side_inlet_temperature_c: options.default_inlet_temperature_c,
        supply_side_outlet_temperature_c: options.default_outlet_temperature_c,
    }
}

fn plant_equipment_states(
    model: &SimulationModel,
    options: PlantStateProjectionOptions,
) -> Vec<PlantEquipmentState> {
    let mut equipment = Vec::new();
    for plant_loop in &model.typed.plant_loops {
        for branch_list in [
            plant_loop.plant_side_branch_list,
            plant_loop.demand_side_branch_list,
        ] {
            for branch_id in plant_branch_ids_for_list(&model.typed, branch_list) {
                let Some(branch) = model
                    .typed
                    .plant_branches
                    .iter()
                    .find(|branch| branch.id == branch_id)
                else {
                    continue;
                };
                for component in &branch.components {
                    push_plant_equipment_state(&mut equipment, component, options);
                }
            }
        }
    }
    equipment
}

fn plant_branch_ids_for_list(model: &TypedModel, branch_list_id: BranchListId) -> Vec<BranchId> {
    model
        .plant_branch_lists
        .iter()
        .find(|list| list.id == branch_list_id)
        .map(|list| list.branches.clone())
        .unwrap_or_default()
}

fn push_plant_equipment_state(
    equipment: &mut Vec<PlantEquipmentState>,
    component: &PlantBranchComponent,
    options: PlantStateProjectionOptions,
) {
    let Some(role) = plant_equipment_role(&component.object_type.0) else {
        return;
    };
    if equipment
        .iter()
        .any(|existing| existing.equipment_name == component.name.0 && existing.role == role)
    {
        return;
    }

    equipment.push(PlantEquipmentState {
        object_type: component.object_type.0.clone(),
        equipment_name: component.name.0.clone(),
        role,
        output_rate_w: plant_equipment_output_rate_w(role, options),
    });
}

fn plant_equipment_role(object_type: &str) -> Option<PlantEquipmentRole> {
    match object_type.to_ascii_lowercase().as_str() {
        "pump:constantspeed" | "pump:variablespeed" => Some(PlantEquipmentRole::Pump),
        "districtheating:water" => Some(PlantEquipmentRole::PurchasedHeating),
        "loadprofile:plant" => Some(PlantEquipmentRole::LoadProfile),
        _ => None,
    }
}

fn plant_equipment_output_rate_w(
    role: PlantEquipmentRole,
    options: PlantStateProjectionOptions,
) -> f64 {
    match role {
        PlantEquipmentRole::Pump => options.default_pump_electricity_rate_w,
        PlantEquipmentRole::PurchasedHeating => options.default_district_heating_rate_w,
        PlantEquipmentRole::LoadProfile => options.default_load_profile_heat_transfer_rate_w,
    }
}

fn plant_equipment_variable_name(role: PlantEquipmentRole) -> &'static str {
    match role {
        PlantEquipmentRole::Pump => "Pump Electricity Rate",
        PlantEquipmentRole::PurchasedHeating => "District Heating Water Rate",
        PlantEquipmentRole::LoadProfile => "Plant Load Profile Heat Transfer Rate",
    }
}

#[derive(Clone, Debug, PartialEq)]
struct ProjectedNodeAssignment {
    node_id: NodeId,
    node_name: String,
    role: NodeStateRole,
}

fn push_projected_node_assignment(
    nodes: &mut Vec<ProjectedNodeAssignment>,
    node: ProjectedNodeAssignment,
) {
    if let Some(existing) = nodes
        .iter_mut()
        .find(|existing| existing.node_id == node.node_id)
    {
        existing.role = merged_node_state_role(existing.role, node.role);
        return;
    }

    nodes.push(node);
}

fn merged_node_state_role(existing: NodeStateRole, next: NodeStateRole) -> NodeStateRole {
    if existing == next {
        return existing;
    }

    match (existing, next) {
        (NodeStateRole::ZoneAir, _) | (_, NodeStateRole::ZoneAir) => NodeStateRole::ZoneAir,
        (NodeStateRole::Supply, NodeStateRole::ReturnAir)
        | (NodeStateRole::ReturnAir, NodeStateRole::Supply) => NodeStateRole::Supply,
        _ => existing,
    }
}

fn apply_node_state_update(
    state: &mut NodeStateStore,
    node_id: NodeId,
    temperature_c: f64,
    humidity_ratio: f64,
    mass_flow_rate_kg_per_s: f64,
) {
    let Some(node_state) = state.find_mut_by_id(node_id) else {
        return;
    };

    let previous_flow = node_state.mass_flow_rate_kg_per_s;
    let total_flow = previous_flow + mass_flow_rate_kg_per_s;
    if previous_flow > 0.0 && total_flow > 0.0 {
        node_state.temperature_c = weighted_value(
            node_state.temperature_c,
            previous_flow,
            temperature_c,
            mass_flow_rate_kg_per_s,
            total_flow,
        );
        node_state.humidity_ratio = weighted_value(
            node_state.humidity_ratio,
            previous_flow,
            humidity_ratio,
            mass_flow_rate_kg_per_s,
            total_flow,
        );
    } else {
        node_state.temperature_c = temperature_c;
        node_state.humidity_ratio = humidity_ratio;
    }
    node_state.mass_flow_rate_kg_per_s = total_flow;
}

fn weighted_value(
    existing_value: f64,
    existing_weight: f64,
    new_value: f64,
    new_weight: f64,
    total_weight: f64,
) -> f64 {
    (existing_value * existing_weight + new_value * new_weight) / total_weight
}

fn add_constant_output_series(
    results: &mut ResultStore,
    handle_index: &mut u32,
    key: &str,
    variable_name: &str,
    units: &str,
    value: f64,
    sample_count: usize,
) {
    results.add_series(OutputSeries {
        handle: OutputHandle(*handle_index),
        key: key.to_string(),
        variable_name: variable_name.to_string(),
        units: units.to_string(),
        values: vec![value; sample_count],
    });
    *handle_index += 1;
}

fn resolve_node_or_nodelist(model: &TypedModel, name: &NormalizedName) -> Vec<NodeId> {
    if let Some(node_id) = model.node_names.resolve(&name.0) {
        return vec![node_id];
    }

    if let Some(node_list_id) = model.node_list_names.resolve(&name.0)
        && let Some(node_list) = model
            .node_lists
            .iter()
            .find(|node_list| node_list.id == node_list_id)
    {
        return node_list.nodes.clone();
    }

    Vec::new()
}

fn node_name_for_id(model: &TypedModel, node_id: NodeId) -> Option<String> {
    model
        .nodes
        .iter()
        .find(|node| node.id == node_id)
        .map(|node| node.name.0.clone())
}

fn ideal_loads_for_connection<'a>(
    model: &'a TypedModel,
    connection: &ZoneEquipmentConnection,
) -> Option<&'a IdealLoadsAirSystem> {
    let list = model
        .zone_equipment_lists
        .iter()
        .find(|list| list.id == connection.equipment_list)?;
    let entry = list.equipment.iter().min_by_key(|entry| {
        (
            entry.heating_or_no_load_sequence,
            entry.cooling_sequence,
            entry.ideal_loads_air_system,
        )
    })?;
    model
        .ideal_loads_air_systems
        .iter()
        .find(|system| system.id == entry.ideal_loads_air_system)
}

fn ideal_loads_design_mass_flow_rate_kg_per_s(system: &IdealLoadsAirSystem) -> Option<f64> {
    let heating_flow_m3_per_s =
        autosized_or_numeric_value(system.maximum_heating_air_flow_rate_m3_per_s);
    let cooling_flow_m3_per_s =
        autosized_or_numeric_value(system.maximum_cooling_air_flow_rate_m3_per_s);
    heating_flow_m3_per_s
        .into_iter()
        .chain(cooling_flow_m3_per_s)
        .filter(|value| *value > 0.0)
        .reduce(f64::max)
        .map(|flow_m3_per_s| flow_m3_per_s * AIR_DENSITY_KG_PER_M3)
}

fn autosized_or_numeric_value(value: Option<AutosizeOrNumber>) -> Option<f64> {
    match value {
        Some(AutosizeOrNumber::Value(value)) => Some(value),
        Some(AutosizeOrNumber::Autosize) | None => None,
    }
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
    initialize_heat_balance_state_with_ctf_coefficients(model, initial_zone_air_temperature_c, &[])
}

/// Initializes the heat-balance state shell with diagnostic CTF coefficient rows.
///
/// This is an oracle-isolation hook for heat-balance diagnostics. It does not
/// calculate EnergyPlus CTF coefficients; callers may provide rows already
/// emitted by EnergyPlus so surface history behavior can be tested separately
/// from coefficient generation.
pub fn initialize_heat_balance_state_with_ctf_coefficients(
    model: &SimulationModel,
    initial_zone_air_temperature_c: f64,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
) -> Result<HeatBalanceState, RuntimeError> {
    let ctf_coefficients_by_construction = construction_ctf_coefficients_by_name(ctf_coefficients);
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
            opaque_surface_conductance_w_per_k: 0.0,
            opaque_surface_heat_gain_w: 0.0,
            sum_ha_w_per_k: 0.0,
            sum_hat_surf_w: 0.0,
            sum_hat_ref_w: 0.0,
            zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
        });
    }

    let surfaces = model
        .typed
        .surfaces
        .iter()
        .map(|surface| {
            let area_m2 = surface_area_m2(&surface.vertices);
            let tilt_deg = surface_tilt_deg(surface.surface_type, &surface.vertices);
            let thermal = surface_thermal_properties(&model.typed, surface)?;
            let boundary = resolve_surface_boundary_target(&model.typed, surface)?;
            let conductance_w_per_k = area_m2 / thermal.thermal_resistance_m2_k_per_w;
            let steady_ctf_w_per_m2_k =
                steady_ctf_coefficient_w_per_m2_k(area_m2, thermal.thermal_resistance_m2_k_per_w);
            let ctf = ctf_coefficients_by_construction
                .get(&thermal.construction_name)
                .and_then(|coefficients| {
                    surface_ctf_state_from_coefficients(
                        coefficients,
                        initial_zone_air_temperature_c,
                    )
                })
                .unwrap_or_else(|| {
                    steady_surface_ctf_state(steady_ctf_w_per_m2_k, initial_zone_air_temperature_c)
                });

            Ok(SurfaceHeatBalanceState {
                surface_id: surface.id,
                zone_id: surface.zone,
                surface_name: surface.name.0.clone(),
                surface_type: surface.surface_type,
                outside_boundary_condition: surface.outside_boundary_condition,
                outside_boundary_condition_object_name: surface
                    .outside_boundary_condition_object
                    .as_ref()
                    .map(|name| name.0.clone()),
                outside_boundary_target_surface_id: boundary.surface_id,
                outside_boundary_target_zone_id: boundary.zone_id,
                construction_id: thermal.construction_id,
                construction_name: thermal.construction_name,
                outside_layer_material_id: thermal.outside_layer_material_id,
                outside_layer_material_name: thermal.outside_layer_material_name,
                outside_layer_roughness: thermal.outside_layer_roughness,
                area_m2,
                tilt_deg,
                thermal_resistance_m2_k_per_w: thermal.thermal_resistance_m2_k_per_w,
                heat_capacity_j_per_m2_k: thermal.heat_capacity_j_per_m2_k,
                thermal_absorptance: thermal.thermal_absorptance,
                solar_absorptance: thermal.solar_absorptance,
                conductance_w_per_k,
                inside_convection_coefficient_w_per_m2_k:
                    ENERGYPLUS_INITIAL_CONVECTION_COEFFICIENT_W_PER_M2_K,
                inside_radiant_internal_gain_w_per_m2: 0.0,
                inside_shortwave_absorbed_w_per_m2: 0.0,
                inside_additional_heat_source_w_per_m2: 0.0,
                inside_radiant_hvac_w_per_m2: 0.0,
                inside_net_longwave_w_per_m2: 0.0,
                ctf,
                heat_gain_to_zone_w: 0.0,
                inside_face_temperature_c: initial_zone_air_temperature_c,
                outside_face_temperature_c: initial_zone_air_temperature_c,
            })
        })
        .collect::<Result<Vec<_>, RuntimeError>>()?;

    for zone in &mut zones {
        zone.opaque_surface_conductance_w_per_k = surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone.zone_id)
            .map(|surface| surface.conductance_w_per_k)
            .sum();
        let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
            zone_surface_convection_sums(&surfaces, zone.zone_id);
        zone.sum_ha_w_per_k = sum_ha_w_per_k;
        zone.sum_hat_surf_w = sum_hat_surf_w;
        zone.sum_hat_ref_w = sum_hat_ref_w;
        zone.zone_air_temperature_coefficients = energyplus_zone_air_temperature_coefficients(
            zone.sum_ha_w_per_k,
            zone.sum_hat_surf_w,
            zone.sum_hat_ref_w,
            zone.convective_internal_gain_w,
            0.0,
            0.0,
            zone.air_heat_capacity_j_per_k,
            0.0,
            zone.previous_mean_air_temperatures_c,
        );
    }

    Ok(HeatBalanceState {
        timestep_index: 0,
        zones,
        surfaces,
    })
}

/// Advances the heat-balance state by one timestep without making a
/// conformance claim.
///
/// This is the first zone-air predictor/corrector-shaped state update. It uses
/// the currently supported opaque surface conductance and internal convective
/// gains while keeping the public zone-temperature comparison diagnostic-only.
pub fn advance_heat_balance_state_one_timestep(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
) {
    advance_heat_balance_state_one_timestep_internal(
        model,
        state,
        input,
        None,
        HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
        1,
    );
}

fn advance_heat_balance_state_one_timestep_internal(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    surface_iteration_count: u32,
) {
    let hour_ending = input.hour_ending.clamp(1, 24);
    let previous_zone_temperatures = state
        .zones
        .iter()
        .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
        .collect::<BTreeMap<_, _>>();
    let previous_surface_inside_temperatures = state
        .surfaces
        .iter()
        .map(|surface| (surface.surface_id, surface.inside_face_temperature_c))
        .collect::<BTreeMap<_, _>>();
    let correct_zone_air_after_surface_pass = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
    );
    let rebalance_surfaces_after_zone_air_correction = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
    );
    let use_previous_inside_for_outdoor_boundary = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
    );
    let use_previous_inside_for_adiabatic_boundary = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
    );
    let use_quick_outside_conduction = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
    );
    let use_doe2_outside_convection = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
    );
    let use_inside_longwave_exchange_probe = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
    );

    for surface in &mut state.surfaces {
        let zone_temperature_c = previous_zone_temperatures
            .get(&surface.zone_id)
            .copied()
            .unwrap_or(surface.inside_face_temperature_c);

        surface.inside_face_temperature_c = zone_temperature_c;
        surface.outside_face_temperature_c = heat_balance_surface_boundary_temperature_c(
            model,
            surface,
            &previous_zone_temperatures,
            input.outdoor_dry_bulb_c,
            zone_temperature_c,
            weather_context,
            None,
        );
    }

    for zone in &mut state.zones {
        let previous_temperature_c = zone.mean_air_temperature_c;
        zone.previous_mean_air_temperatures_c = [
            previous_temperature_c,
            zone.previous_mean_air_temperatures_c[0],
            zone.previous_mean_air_temperatures_c[1],
        ];
        zone.convective_internal_gain_w =
            convective_internal_gain_w(model, zone.zone_id, hour_ending);

        let conductance_w_per_k = state
            .surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone.zone_id)
            .map(|surface| surface.conductance_w_per_k)
            .sum::<f64>();
        let conductance_weighted_outside_temperature = state
            .surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone.zone_id)
            .map(|surface| surface.conductance_w_per_k * surface.outside_face_temperature_c)
            .sum::<f64>();
        let equivalent_outside_temperature_c = if conductance_w_per_k > 0.0 {
            conductance_weighted_outside_temperature / conductance_w_per_k
        } else {
            previous_temperature_c
        };

        zone.opaque_surface_conductance_w_per_k = conductance_w_per_k;
        zone.mean_air_temperature_c = match zone_air_algorithm {
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical => step_zone_air_temperature(
                previous_temperature_c,
                equivalent_outside_temperature_c,
                zone.convective_internal_gain_w,
                conductance_w_per_k,
                zone.air_heat_capacity_j_per_k,
                input.timestep_seconds,
            ),
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe => previous_temperature_c,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe => {
                let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
                    zone_surface_convection_sums(&state.surfaces, zone.zone_id);
                let coefficients = energyplus_zone_air_temperature_coefficients(
                    sum_ha_w_per_k,
                    sum_hat_surf_w,
                    sum_hat_ref_w,
                    zone.convective_internal_gain_w,
                    0.0,
                    0.0,
                    zone.air_heat_capacity_j_per_k,
                    input.timestep_seconds,
                    zone.previous_mean_air_temperatures_c,
                );
                energyplus_analytical_zone_air_temperature_c(
                    previous_temperature_c,
                    coefficients.temp_independent_coefficient_w,
                    coefficients.temp_dependent_coefficient_w_per_k,
                    zone.air_heat_capacity_j_per_k,
                    input.timestep_seconds,
                )
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe => {
                let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
                    zone_surface_convection_sums(&state.surfaces, zone.zone_id);
                let coefficients = energyplus_zone_air_temperature_coefficients(
                    sum_ha_w_per_k,
                    sum_hat_surf_w,
                    sum_hat_ref_w,
                    zone.convective_internal_gain_w,
                    0.0,
                    0.0,
                    zone.air_heat_capacity_j_per_k,
                    input.timestep_seconds,
                    zone.previous_mean_air_temperatures_c,
                );
                energyplus_third_order_zone_air_temperature_from_coefficients(
                    previous_temperature_c,
                    coefficients,
                )
            }
        };
    }

    let current_zone_temperatures = state
        .zones
        .iter()
        .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
        .collect::<BTreeMap<_, _>>();

    run_surface_balance_passes(
        model,
        &mut state.surfaces,
        Some(&previous_surface_inside_temperatures),
        &current_zone_temperatures,
        input,
        weather_context,
        surface_iteration_count,
        use_previous_inside_for_outdoor_boundary,
        use_previous_inside_for_adiabatic_boundary,
        use_quick_outside_conduction,
        use_doe2_outside_convection,
        use_inside_longwave_exchange_probe,
    );

    if rebalance_surfaces_after_zone_air_correction {
        correct_zone_air_temperatures_from_current_surfaces(
            &state.surfaces,
            &mut state.zones,
            input.timestep_seconds,
            true,
        );
        let corrected_zone_temperatures = state
            .zones
            .iter()
            .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
            .collect::<BTreeMap<_, _>>();
        run_surface_balance_passes(
            model,
            &mut state.surfaces,
            None,
            &corrected_zone_temperatures,
            input,
            weather_context,
            surface_iteration_count,
            use_previous_inside_for_outdoor_boundary,
            use_previous_inside_for_adiabatic_boundary,
            use_quick_outside_conduction,
            use_doe2_outside_convection,
            use_inside_longwave_exchange_probe,
        );
    }

    for surface in &mut state.surfaces {
        advance_surface_ctf_histories(surface);
    }

    correct_zone_air_temperatures_from_current_surfaces(
        &state.surfaces,
        &mut state.zones,
        input.timestep_seconds,
        correct_zone_air_after_surface_pass,
    );

    state.timestep_index += 1;
}

fn run_surface_balance_passes(
    model: &TypedModel,
    surfaces: &mut [SurfaceHeatBalanceState],
    first_pass_inside_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    zone_temperatures: &BTreeMap<ZoneId, f64>,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    surface_iteration_count: u32,
    use_previous_inside_for_outdoor_boundary: bool,
    use_previous_inside_for_adiabatic_boundary: bool,
    use_quick_outside_conduction: bool,
    use_doe2_outside_convection: bool,
    use_inside_longwave_exchange_probe: bool,
) {
    for surface_iteration_index in 0..surface_iteration_count.max(1) {
        if use_inside_longwave_exchange_probe {
            let temperature_overrides = if surface_iteration_index == 0 {
                first_pass_inside_temperatures
            } else {
                None
            };
            update_surface_inside_longwave_exchange_probe(surfaces, temperature_overrides);
        }
        for surface in surfaces.iter_mut() {
            let previous_inside_face_temperature_c = if surface_iteration_index == 0 {
                first_pass_inside_temperatures
                    .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                    .unwrap_or(surface.inside_face_temperature_c)
            } else {
                surface.inside_face_temperature_c
            };
            let zone_temperature_c = zone_temperatures
                .get(&surface.zone_id)
                .copied()
                .unwrap_or(surface.inside_face_temperature_c);
            let inside_convection_coefficient_w_per_m2_k =
                energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
                    surface,
                    previous_inside_face_temperature_c,
                    zone_temperature_c,
                );
            surface.inside_convection_coefficient_w_per_m2_k =
                inside_convection_coefficient_w_per_m2_k;

            update_surface_ctf_history_constants(surface);
            let use_previous_inside_for_boundary = (use_previous_inside_for_outdoor_boundary
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors)
                || (use_previous_inside_for_adiabatic_boundary
                    && surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic);
            let outside_balance_inside_temperature_c = if use_previous_inside_for_boundary {
                previous_inside_face_temperature_c
            } else {
                zone_temperature_c
            };
            surface.inside_face_temperature_c = outside_balance_inside_temperature_c;
            let net_inside_source_w_per_m2 = surface_inside_ctf_source_terms_w_per_m2(surface);
            let quick_outside_conduction = if use_quick_outside_conduction
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors
            {
                Some(QuickOutsideConductionContext {
                    reference_air_temperature_c: zone_temperature_c,
                    inside_convection_coefficient_w_per_m2_k:
                        inside_convection_coefficient_w_per_m2_k,
                    net_inside_source_w_per_m2,
                    use_doe2_outside_convection,
                })
            } else {
                None
            };
            surface.outside_face_temperature_c = heat_balance_surface_boundary_temperature_c(
                model,
                surface,
                zone_temperatures,
                input.outdoor_dry_bulb_c,
                outside_balance_inside_temperature_c,
                weather_context,
                quick_outside_conduction,
            );
            surface.inside_face_temperature_c = energyplus_ctf_inside_face_temperature_c(
                surface,
                CtfInsideFaceBalanceInput {
                    reference_air_temperature_c: zone_temperature_c,
                    inside_convection_coefficient_w_per_m2_k,
                    previous_inside_face_temperature_c,
                    net_inside_source_w_per_m2,
                },
            );
            if surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic
                && !use_previous_inside_for_adiabatic_boundary
            {
                surface.outside_face_temperature_c = surface.inside_face_temperature_c;
            }
            surface.heat_gain_to_zone_w = surface_inside_conduction_rate_w(surface);
        }
    }
}

fn correct_zone_air_temperatures_from_current_surfaces(
    surfaces: &[SurfaceHeatBalanceState],
    zones: &mut [ZoneHeatBalanceState],
    timestep_seconds: f64,
    update_mean_air_temperature: bool,
) {
    for zone in zones {
        zone.opaque_surface_heat_gain_w = surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone.zone_id)
            .map(|surface| surface.heat_gain_to_zone_w)
            .sum();
        let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
            zone_surface_convection_sums(surfaces, zone.zone_id);
        zone.sum_ha_w_per_k = sum_ha_w_per_k;
        zone.sum_hat_surf_w = sum_hat_surf_w;
        zone.sum_hat_ref_w = sum_hat_ref_w;
        let coefficients = energyplus_zone_air_temperature_coefficients(
            zone.sum_ha_w_per_k,
            zone.sum_hat_surf_w,
            zone.sum_hat_ref_w,
            zone.convective_internal_gain_w,
            0.0,
            0.0,
            zone.air_heat_capacity_j_per_k,
            timestep_seconds,
            zone.previous_mean_air_temperatures_c,
        );
        if update_mean_air_temperature {
            zone.mean_air_temperature_c = energyplus_analytical_zone_air_temperature_c(
                zone.previous_mean_air_temperatures_c[0],
                coefficients.temp_independent_coefficient_w,
                coefficients.temp_dependent_coefficient_w_per_k,
                zone.air_heat_capacity_j_per_k,
                timestep_seconds,
            );
        }
        zone.zone_air_temperature_coefficients = coefficients;
    }
}

fn zone_surface_convection_sums(
    surfaces: &[SurfaceHeatBalanceState],
    zone_id: ZoneId,
) -> (f64, f64, f64) {
    let (sum_ha_w_per_k, sum_hat_surf_w) = surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone_id)
        .map(|surface| {
            let surface_ha_w_per_k =
                surface.inside_convection_coefficient_w_per_m2_k * surface.area_m2;
            (
                surface_ha_w_per_k,
                surface_ha_w_per_k * surface.inside_face_temperature_c,
            )
        })
        .fold((0.0, 0.0), |(sum_ha, sum_hat), (ha, hat)| {
            (sum_ha + ha, sum_hat + hat)
        });

    (sum_ha_w_per_k, sum_hat_surf_w, 0.0)
}

fn zone_air_heat_balance_air_storage_rate_w(
    zone_state: &ZoneHeatBalanceState,
    timestep_seconds: f64,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> f64 {
    match zone_air_algorithm {
        HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe => {
            zone_state
                .zone_air_temperature_coefficients
                .temp_independent_coefficient_w
                - zone_state
                    .zone_air_temperature_coefficients
                    .temp_dependent_coefficient_w_per_k
                    * zone_state.mean_air_temperature_c
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe => {
            if timestep_seconds > 0.0 {
                zone_state.air_heat_capacity_j_per_k
                    * (zone_state.mean_air_temperature_c
                        - zone_state.previous_mean_air_temperatures_c[0])
                    / timestep_seconds
            } else {
                0.0
            }
        }
    }
}

/// Simulates hourly zone mean air temperatures through the heat-balance state
/// shell without making a conformance claim.
///
/// This diagnostic trace runs every configured zone timestep, samples hourly
/// MAT values, and stores EnergyPlus-style result series for all zones.
pub fn simulate_heat_balance_zone_air_temperatures(
    model: &SimulationModel,
    weather_dry_bulb_c: &[f64],
    options: HeatBalanceSimulationOptions,
) -> Result<HeatBalanceSimulation, RuntimeError> {
    simulate_heat_balance_zone_air_temperatures_internal(
        model,
        weather_dry_bulb_c,
        None,
        options,
        &[],
    )
}

/// Simulates hourly zone mean air temperatures with full EPW records available
/// for diagnostic exterior surface forcing.
pub fn simulate_heat_balance_zone_air_temperatures_with_weather_records(
    model: &SimulationModel,
    weather_records: &[EpwRecord],
    options: HeatBalanceSimulationOptions,
) -> Result<HeatBalanceSimulation, RuntimeError> {
    simulate_heat_balance_zone_air_temperatures_with_weather_records_and_ctf_coefficients(
        model,
        weather_records,
        options,
        &[],
    )
}

/// Simulates hourly zone mean air temperatures with diagnostic CTF coefficient rows.
///
/// The coefficient rows are intended for diagnostic isolation with EnergyPlus
/// `eplusout.eio` CTF output. Conformance paths should use the default
/// simulation entry points until native coefficient generation is ported.
pub fn simulate_heat_balance_zone_air_temperatures_with_weather_records_and_ctf_coefficients(
    model: &SimulationModel,
    weather_records: &[EpwRecord],
    options: HeatBalanceSimulationOptions,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
) -> Result<HeatBalanceSimulation, RuntimeError> {
    let weather_dry_bulb_c = weather_records
        .iter()
        .map(|record| record.dry_bulb_c)
        .collect::<Vec<_>>();
    simulate_heat_balance_zone_air_temperatures_internal(
        model,
        &weather_dry_bulb_c,
        Some(weather_records),
        options,
        ctf_coefficients,
    )
}

fn simulate_heat_balance_zone_air_temperatures_internal(
    model: &SimulationModel,
    weather_dry_bulb_c: &[f64],
    weather_records: Option<&[EpwRecord]>,
    options: HeatBalanceSimulationOptions,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
) -> Result<HeatBalanceSimulation, RuntimeError> {
    if weather_dry_bulb_c.is_empty() {
        return Err(RuntimeError::NoWeatherData);
    }
    if options.sample_count > weather_dry_bulb_c.len() {
        return Err(RuntimeError::SampleCountExceedsWeather {
            requested: options.sample_count,
            available: weather_dry_bulb_c.len(),
        });
    }
    if model.typed.zones.is_empty() {
        return Err(RuntimeError::NoZones);
    }

    let zone_steps_per_hour = model.typed.timestep.number_of_timesteps_per_hour.max(1);
    let seconds_per_timestep = SECONDS_PER_HOUR / f64::from(zone_steps_per_hour);
    let mut state = initialize_heat_balance_state_with_ctf_coefficients(
        model,
        options.initial_zone_air_temperature_c,
        ctf_coefficients,
    )?;
    match options.ctf_initial_history_policy {
        HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue => {
            seed_initial_surface_ctf_boundary_histories(&mut state, weather_dry_bulb_c[0]);
        }
        HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial => {
            seed_energyplus_initial_surface_ctf_histories(
                &mut state,
                options.initial_zone_air_temperature_c,
            );
        }
    }
    let warmup = run_heat_balance_run_period_warmup(
        &model.typed,
        &mut state,
        weather_dry_bulb_c,
        weather_records,
        zone_steps_per_hour,
        seconds_per_timestep,
        options.warmup,
        options.zone_air_algorithm,
        options.surface_iteration_count,
    );
    let run_period_timestep_start = state.timestep_index;
    let mut zone_temperatures = state
        .zones
        .iter()
        .map(|zone| {
            (
                zone.zone_id,
                zone.zone_name.clone(),
                Vec::with_capacity(options.sample_count),
            )
        })
        .collect::<Vec<_>>();
    let mut zone_conduction_rates = state
        .zones
        .iter()
        .map(|zone| {
            (
                zone.zone_id,
                zone.zone_name.clone(),
                Vec::with_capacity(options.sample_count),
                Vec::with_capacity(options.sample_count),
                Vec::with_capacity(options.sample_count),
            )
        })
        .collect::<Vec<_>>();
    let mut zone_air_heat_balance_rates = state
        .zones
        .iter()
        .map(|zone| {
            (
                zone.zone_id,
                zone.zone_name.clone(),
                Vec::with_capacity(options.sample_count),
                Vec::with_capacity(options.sample_count),
                Vec::with_capacity(options.sample_count),
            )
        })
        .collect::<Vec<_>>();
    let mut surface_temperatures = state
        .surfaces
        .iter()
        .map(|surface| SurfaceHeatBalanceTrace {
            surface_id: surface.surface_id,
            surface_name: surface.surface_name.clone(),
            inside_face_temperature_c: Vec::with_capacity(options.sample_count),
            outside_face_temperature_c: Vec::with_capacity(options.sample_count),
            inside_conduction_rate_w: Vec::with_capacity(options.sample_count),
            inside_conduction_gain_rate_w: Vec::with_capacity(options.sample_count),
            inside_conduction_loss_rate_w: Vec::with_capacity(options.sample_count),
            inside_conduction_rate_per_area_w_per_m2: Vec::with_capacity(options.sample_count),
            outside_conduction_rate_w: Vec::with_capacity(options.sample_count),
            outside_conduction_gain_rate_w: Vec::with_capacity(options.sample_count),
            outside_conduction_loss_rate_w: Vec::with_capacity(options.sample_count),
            outside_conduction_rate_per_area_w_per_m2: Vec::with_capacity(options.sample_count),
            heat_storage_rate_w: Vec::with_capacity(options.sample_count),
        })
        .collect::<Vec<_>>();
    let mut outdoor_temperatures = Vec::with_capacity(options.sample_count);

    for (hour_index, outdoor_dry_bulb_c) in weather_dry_bulb_c
        .iter()
        .copied()
        .take(options.sample_count)
        .enumerate()
    {
        let hour_ending = u32::try_from(hour_index % 24 + 1).unwrap_or(24);
        let weather_context = weather_records.map(|records| HeatBalanceWeatherContext {
            records,
            record_index: hour_index,
            zone_steps_per_hour,
        });
        for _substep in 0..zone_steps_per_hour {
            advance_heat_balance_state_one_timestep_internal(
                &model.typed,
                &mut state,
                HeatBalanceStepInput {
                    outdoor_dry_bulb_c,
                    hour_ending,
                    timestep_seconds: seconds_per_timestep,
                },
                weather_context,
                options.zone_air_algorithm,
                options.surface_iteration_count,
            );
        }

        for (zone_id, _zone_name, values) in &mut zone_temperatures {
            if let Some(zone_state) = state.zones.iter().find(|zone| zone.zone_id == *zone_id) {
                values.push(zone_state.mean_air_temperature_c);
            }
        }
        for (
            zone_id,
            _zone_name,
            conduction_values,
            conduction_gain_values,
            conduction_loss_values,
        ) in &mut zone_conduction_rates
        {
            if let Some(zone_state) = state.zones.iter().find(|zone| zone.zone_id == *zone_id) {
                let rate = zone_state.opaque_surface_heat_gain_w;
                conduction_values.push(rate);
                conduction_gain_values.push(heat_gain_rate_w(rate));
                conduction_loss_values.push(heat_loss_rate_w(rate));
            }
        }
        for (
            zone_id,
            _zone_name,
            internal_gain_values,
            surface_convection_values,
            air_storage_values,
        ) in &mut zone_air_heat_balance_rates
        {
            if let Some(zone_state) = state.zones.iter().find(|zone| zone.zone_id == *zone_id) {
                internal_gain_values.push(zone_state.convective_internal_gain_w);
                surface_convection_values.push(
                    zone_state.sum_hat_surf_w
                        - zone_state.sum_hat_ref_w
                        - zone_state.sum_ha_w_per_k * zone_state.mean_air_temperature_c,
                );
                air_storage_values.push(zone_air_heat_balance_air_storage_rate_w(
                    zone_state,
                    seconds_per_timestep,
                    options.zone_air_algorithm,
                ));
            }
        }
        for trace in &mut surface_temperatures {
            if let Some(surface_state) = state
                .surfaces
                .iter()
                .find(|surface| surface.surface_id == trace.surface_id)
            {
                let inside_rate = surface_inside_conduction_rate_w(surface_state);
                let outside_rate = surface_outside_conduction_rate_w(surface_state);
                let storage_rate = surface_heat_storage_rate_w(inside_rate, outside_rate);
                let outside_face_temperature_c = reported_surface_outside_face_temperature_c(
                    &model.typed,
                    surface_state,
                    outdoor_dry_bulb_c,
                    surface_state.inside_face_temperature_c,
                    weather_context,
                    options.zone_air_algorithm,
                );
                trace
                    .inside_face_temperature_c
                    .push(surface_state.inside_face_temperature_c);
                trace
                    .outside_face_temperature_c
                    .push(outside_face_temperature_c);
                trace.inside_conduction_rate_w.push(inside_rate);
                trace
                    .inside_conduction_gain_rate_w
                    .push(heat_gain_rate_w(inside_rate));
                trace
                    .inside_conduction_loss_rate_w
                    .push(heat_loss_rate_w(inside_rate));
                trace.inside_conduction_rate_per_area_w_per_m2.push(
                    surface_rate_per_area_w_per_m2(inside_rate, surface_state.area_m2),
                );
                trace.outside_conduction_rate_w.push(outside_rate);
                trace
                    .outside_conduction_gain_rate_w
                    .push(heat_gain_rate_w(outside_rate));
                trace
                    .outside_conduction_loss_rate_w
                    .push(heat_loss_rate_w(outside_rate));
                trace.outside_conduction_rate_per_area_w_per_m2.push(
                    surface_rate_per_area_w_per_m2(outside_rate, surface_state.area_m2),
                );
                trace.heat_storage_rate_w.push(storage_rate);
            }
        }
        outdoor_temperatures.push(outdoor_dry_bulb_c);
    }

    let mut results = ResultStore::new();
    let mut handle_index = 0;
    for (_zone_id, zone_name, values) in zone_temperatures {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name,
            variable_name: "Zone Mean Air Temperature".to_string(),
            units: "C".to_string(),
            values,
        });
        handle_index += 1;
    }
    for (_zone_id, zone_name, conduction_values, conduction_gain_values, conduction_loss_values) in
        zone_conduction_rates
    {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name.clone(),
            variable_name: "Zone Opaque Surface Inside Faces Conduction Rate".to_string(),
            units: "W".to_string(),
            values: conduction_values,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name.clone(),
            variable_name: "Zone Opaque Surface Inside Faces Conduction Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: conduction_gain_values,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name,
            variable_name: "Zone Opaque Surface Inside Faces Conduction Heat Loss Rate".to_string(),
            units: "W".to_string(),
            values: conduction_loss_values,
        });
        handle_index += 1;
    }
    for (
        _zone_id,
        zone_name,
        internal_gain_values,
        surface_convection_values,
        air_storage_values,
    ) in zone_air_heat_balance_rates
    {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name.clone(),
            variable_name: "Zone Air Heat Balance Internal Convective Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: internal_gain_values,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name.clone(),
            variable_name: "Zone Air Heat Balance Surface Convection Rate".to_string(),
            units: "W".to_string(),
            values: surface_convection_values,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name,
            variable_name: "Zone Air Heat Balance Air Energy Storage Rate".to_string(),
            units: "W".to_string(),
            values: air_storage_values,
        });
        handle_index += 1;
    }
    for trace in surface_temperatures {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Temperature".to_string(),
            units: "C".to_string(),
            values: trace.inside_face_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Temperature".to_string(),
            units: "C".to_string(),
            values: trace.outside_face_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Transfer Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Loss Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_loss_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Transfer Rate per Area".to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_conduction_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Transfer Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Loss Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_loss_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Transfer Rate per Area"
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_conduction_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name,
            variable_name: "Surface Heat Storage Rate".to_string(),
            units: "W".to_string(),
            values: trace.heat_storage_rate_w,
        });
        handle_index += 1;
    }
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Outdoor Air Drybulb Temperature".to_string(),
        units: "C".to_string(),
        values: outdoor_temperatures,
    });

    let summary = HeatBalanceSimulationSummary {
        samples: options.sample_count,
        timestep_count: state.timestep_index,
        run_period_timestep_count: state.timestep_index - run_period_timestep_start,
        warmup,
        zone_count: state.zones.len(),
        surface_count: state.surfaces.len(),
        surface_iteration_count: options.surface_iteration_count,
        ctf_initial_history_policy: options.ctf_initial_history_policy,
    };

    Ok(HeatBalanceSimulation {
        state,
        results,
        summary,
    })
}

fn run_heat_balance_run_period_warmup(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    weather_dry_bulb_c: &[f64],
    weather_records: Option<&[EpwRecord]>,
    zone_steps_per_hour: u32,
    seconds_per_timestep: f64,
    options: HeatBalanceWarmupOptions,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    surface_iteration_count: u32,
) -> HeatBalanceWarmupSummary {
    if !options.enabled || options.maximum_days == 0 || weather_dry_bulb_c.is_empty() {
        return HeatBalanceWarmupSummary::disabled();
    }

    let hours_per_day = weather_dry_bulb_c.len().min(24);
    let maximum_days = options.maximum_days.max(options.minimum_days).max(1);
    let tolerance = options.temperature_convergence_tolerance_delta_c.max(0.0);
    let timestep_start = state.timestep_index;
    let mut previous_day_end_temperatures: Option<Vec<f64>> = None;
    let mut final_delta = f64::INFINITY;

    for day in 1..=maximum_days {
        for (hour_index, outdoor_dry_bulb_c) in weather_dry_bulb_c
            .iter()
            .copied()
            .take(hours_per_day)
            .enumerate()
        {
            let hour_ending = u32::try_from(hour_index % 24 + 1).unwrap_or(24);
            let weather_context = weather_records.map(|records| HeatBalanceWeatherContext {
                records,
                record_index: hour_index,
                zone_steps_per_hour,
            });
            for _substep in 0..zone_steps_per_hour {
                advance_heat_balance_state_one_timestep_internal(
                    model,
                    state,
                    HeatBalanceStepInput {
                        outdoor_dry_bulb_c,
                        hour_ending,
                        timestep_seconds: seconds_per_timestep,
                    },
                    weather_context,
                    zone_air_algorithm,
                    surface_iteration_count,
                );
            }
        }

        let day_end_temperatures = heat_balance_zone_temperature_snapshot(state);
        if let Some(previous_temperatures) = &previous_day_end_temperatures {
            final_delta = max_abs_pair_delta(
                previous_temperatures.as_slice(),
                day_end_temperatures.as_slice(),
            );
            if day >= options.minimum_days && final_delta <= tolerance {
                return HeatBalanceWarmupSummary {
                    enabled: true,
                    day_count: day,
                    timestep_count: state.timestep_index - timestep_start,
                    hours_per_day,
                    converged: true,
                    final_max_zone_temperature_delta_c: final_delta,
                };
            }
        }
        previous_day_end_temperatures = Some(day_end_temperatures);
    }

    HeatBalanceWarmupSummary {
        enabled: true,
        day_count: maximum_days,
        timestep_count: state.timestep_index - timestep_start,
        hours_per_day,
        converged: false,
        final_max_zone_temperature_delta_c: final_delta,
    }
}

fn heat_balance_zone_temperature_snapshot(state: &HeatBalanceState) -> Vec<f64> {
    state
        .zones
        .iter()
        .map(|zone| zone.mean_air_temperature_c)
        .collect()
}

fn max_abs_pair_delta(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .fold(0.0, f64::max)
}

#[derive(Clone, Debug, PartialEq)]
struct SurfaceThermalProperties {
    construction_id: ConstructionId,
    construction_name: String,
    outside_layer_material_id: MaterialId,
    outside_layer_material_name: String,
    outside_layer_roughness: MaterialSurfaceRoughness,
    thermal_resistance_m2_k_per_w: f64,
    heat_capacity_j_per_m2_k: Option<f64>,
    thermal_absorptance: f64,
    solar_absorptance: f64,
}

#[derive(Clone, Copy)]
struct HeatBalanceWeatherContext<'a> {
    records: &'a [EpwRecord],
    record_index: usize,
    zone_steps_per_hour: u32,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct SurfaceBoundaryTarget {
    surface_id: Option<SurfaceId>,
    zone_id: Option<ZoneId>,
}

fn surface_thermal_properties(
    model: &TypedModel,
    surface: &Surface,
) -> Result<SurfaceThermalProperties, RuntimeError> {
    let construction = model
        .constructions
        .iter()
        .find(|construction| construction.id == surface.construction)
        .ok_or_else(|| RuntimeError::MissingConstruction {
            surface_name: surface.name.0.clone(),
        })?;
    let layer_ids = if construction.layers.is_empty() {
        std::slice::from_ref(&construction.outside_layer)
    } else {
        construction.layers.as_slice()
    };
    let mut layer_materials = Vec::with_capacity(layer_ids.len());
    for layer_id in layer_ids {
        let material = model
            .materials
            .iter()
            .find(|material| material.id == *layer_id)
            .ok_or_else(|| RuntimeError::MissingMaterial {
                construction_name: construction.name.0.clone(),
            })?;
        layer_materials.push(material);
    }
    let material = layer_materials
        .first()
        .ok_or_else(|| RuntimeError::MissingMaterial {
            construction_name: construction.name.0.clone(),
        })?;
    let mut thermal_resistance_m2_k_per_w = 0.0;
    for material in &layer_materials {
        thermal_resistance_m2_k_per_w += material.thermal_resistance().ok_or_else(|| {
            RuntimeError::MissingThermalResistance {
                material_name: material.name.0.clone(),
            }
        })?;
    }
    let heat_capacity_j_per_m2_k = layer_materials
        .iter()
        .filter_map(|material| material.heat_capacity_per_area())
        .sum::<f64>();
    let heat_capacity_j_per_m2_k = if heat_capacity_j_per_m2_k > 0.0 {
        Some(heat_capacity_j_per_m2_k)
    } else {
        None
    };

    Ok(SurfaceThermalProperties {
        construction_id: construction.id,
        construction_name: construction.name.0.clone(),
        outside_layer_material_id: material.id,
        outside_layer_material_name: material.name.0.clone(),
        outside_layer_roughness: material
            .roughness
            .unwrap_or(MaterialSurfaceRoughness::MediumRough),
        thermal_resistance_m2_k_per_w,
        heat_capacity_j_per_m2_k,
        thermal_absorptance: material
            .thermal_absorptance
            .unwrap_or(DEFAULT_MATERIAL_THERMAL_ABSORPTANCE),
        solar_absorptance: material
            .solar_absorptance
            .unwrap_or(DEFAULT_MATERIAL_SOLAR_ABSORPTANCE),
    })
}

fn steady_ctf_coefficient_w_per_m2_k(area_m2: f64, thermal_resistance_m2_k_per_w: f64) -> f64 {
    if area_m2 > 0.0 && thermal_resistance_m2_k_per_w > 0.0 {
        1.0 / thermal_resistance_m2_k_per_w
    } else {
        0.0
    }
}

fn steady_surface_ctf_state(
    coefficient_w_per_m2_k: f64,
    initial_temperature_c: f64,
) -> SurfaceCtfState {
    SurfaceCtfState {
        outside_0_w_per_m2_k: coefficient_w_per_m2_k,
        cross_0_w_per_m2_k: coefficient_w_per_m2_k,
        inside_0_w_per_m2_k: coefficient_w_per_m2_k,
        const_in_part_w_per_m2: 0.0,
        const_out_part_w_per_m2: 0.0,
        outside_history_w_per_m2_k: Vec::new(),
        cross_history_w_per_m2_k: Vec::new(),
        inside_history_w_per_m2_k: Vec::new(),
        flux_history: Vec::new(),
        outside_temperature_history_c: vec![initial_temperature_c],
        inside_temperature_history_c: vec![initial_temperature_c],
        outside_flux_history_w_per_m2: vec![0.0],
        inside_flux_history_w_per_m2: vec![0.0],
    }
}

fn construction_ctf_coefficients_by_name(
    coefficients: &[ConstructionCtfCoefficientOverride],
) -> BTreeMap<String, Vec<&ConstructionCtfCoefficientOverride>> {
    let mut by_construction = BTreeMap::new();
    for coefficient in coefficients {
        by_construction
            .entry(NormalizedName::new(&coefficient.construction_name).0)
            .or_insert_with(Vec::new)
            .push(coefficient);
    }
    for coefficients in by_construction.values_mut() {
        coefficients.sort_by_key(|coefficient| coefficient.time_index);
    }
    by_construction
}

fn surface_ctf_state_from_coefficients(
    coefficients: &[&ConstructionCtfCoefficientOverride],
    initial_temperature_c: f64,
) -> Option<SurfaceCtfState> {
    let zero = coefficients
        .iter()
        .copied()
        .find(|coefficient| coefficient.time_index == 0)?;
    let history = coefficients
        .iter()
        .copied()
        .filter(|coefficient| coefficient.time_index > 0)
        .collect::<Vec<_>>();
    let history_terms = history.len();

    Some(SurfaceCtfState {
        outside_0_w_per_m2_k: zero.outside_w_per_m2_k,
        cross_0_w_per_m2_k: zero.cross_w_per_m2_k,
        inside_0_w_per_m2_k: zero.inside_w_per_m2_k,
        const_in_part_w_per_m2: 0.0,
        const_out_part_w_per_m2: 0.0,
        outside_history_w_per_m2_k: history
            .iter()
            .map(|coefficient| coefficient.outside_w_per_m2_k)
            .collect(),
        cross_history_w_per_m2_k: history
            .iter()
            .map(|coefficient| coefficient.cross_w_per_m2_k)
            .collect(),
        inside_history_w_per_m2_k: history
            .iter()
            .map(|coefficient| coefficient.inside_w_per_m2_k)
            .collect(),
        flux_history: history
            .iter()
            .map(|coefficient| coefficient.flux.unwrap_or(0.0))
            .collect(),
        outside_temperature_history_c: vec![initial_temperature_c; history_terms],
        inside_temperature_history_c: vec![initial_temperature_c; history_terms],
        outside_flux_history_w_per_m2: vec![0.0; history_terms],
        inside_flux_history_w_per_m2: vec![0.0; history_terms],
    })
}

fn seed_initial_surface_ctf_boundary_histories(
    state: &mut HeatBalanceState,
    initial_outdoor_dry_bulb_c: f64,
) {
    let zone_temperatures = state
        .zones
        .iter()
        .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
        .collect::<BTreeMap<_, _>>();

    for surface in &mut state.surfaces {
        let inside_temperature_c = zone_temperatures
            .get(&surface.zone_id)
            .copied()
            .unwrap_or(surface.inside_face_temperature_c);
        let outside_temperature_c = initial_surface_ctf_boundary_temperature_c(
            surface,
            &zone_temperatures,
            initial_outdoor_dry_bulb_c,
            inside_temperature_c,
        );
        let initial_flux_w_per_m2 = surface_steady_u_value_w_per_m2_k(surface)
            * (outside_temperature_c - inside_temperature_c);

        surface.inside_face_temperature_c = inside_temperature_c;
        surface.outside_face_temperature_c = outside_temperature_c;
        surface
            .ctf
            .inside_temperature_history_c
            .fill(inside_temperature_c);
        surface
            .ctf
            .outside_temperature_history_c
            .fill(outside_temperature_c);
        surface
            .ctf
            .inside_flux_history_w_per_m2
            .fill(initial_flux_w_per_m2);
        surface
            .ctf
            .outside_flux_history_w_per_m2
            .fill(initial_flux_w_per_m2);
    }
}

fn seed_energyplus_initial_surface_ctf_histories(
    state: &mut HeatBalanceState,
    initial_surface_temperature_c: f64,
) {
    for surface in &mut state.surfaces {
        surface.inside_face_temperature_c = initial_surface_temperature_c;
        surface.outside_face_temperature_c = initial_surface_temperature_c;
        surface
            .ctf
            .inside_temperature_history_c
            .fill(initial_surface_temperature_c);
        surface
            .ctf
            .outside_temperature_history_c
            .fill(initial_surface_temperature_c);
        surface.ctf.inside_flux_history_w_per_m2.fill(0.0);
        surface.ctf.outside_flux_history_w_per_m2.fill(0.0);
    }
}

fn initial_surface_ctf_boundary_temperature_c(
    surface: &SurfaceHeatBalanceState,
    zone_temperatures: &BTreeMap<ZoneId, f64>,
    initial_outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
) -> f64 {
    match surface.outside_boundary_condition {
        OutsideBoundaryCondition::Outdoors => initial_outdoor_dry_bulb_c,
        OutsideBoundaryCondition::Adiabatic => owning_zone_temperature_c,
        _ => surface_boundary_temperature_c(
            surface,
            zone_temperatures,
            initial_outdoor_dry_bulb_c,
            owning_zone_temperature_c,
        ),
    }
}

fn surface_steady_u_value_w_per_m2_k(surface: &SurfaceHeatBalanceState) -> f64 {
    if surface.thermal_resistance_m2_k_per_w > 0.0 {
        1.0 / surface.thermal_resistance_m2_k_per_w
    } else {
        0.0
    }
}

fn resolve_surface_boundary_target(
    model: &TypedModel,
    surface: &Surface,
) -> Result<SurfaceBoundaryTarget, RuntimeError> {
    match surface.outside_boundary_condition {
        OutsideBoundaryCondition::Surface => {
            let target_name = boundary_object_name(surface);
            let target_surface = model
                .surfaces
                .iter()
                .find(|candidate| candidate.name == NormalizedName::new(&target_name))
                .ok_or_else(|| RuntimeError::MissingSurfaceBoundaryTarget {
                    surface_name: surface.name.0.clone(),
                    target_name: target_name.clone(),
                })?;
            Ok(SurfaceBoundaryTarget {
                surface_id: Some(target_surface.id),
                zone_id: Some(target_surface.zone),
            })
        }
        OutsideBoundaryCondition::Zone | OutsideBoundaryCondition::Space => {
            let target_name = boundary_object_name(surface);
            let target_zone = model
                .zones
                .iter()
                .find(|zone| zone.name == NormalizedName::new(&target_name))
                .ok_or_else(|| RuntimeError::MissingZoneBoundaryTarget {
                    surface_name: surface.name.0.clone(),
                    target_name: target_name.clone(),
                })?;
            Ok(SurfaceBoundaryTarget {
                surface_id: None,
                zone_id: Some(target_zone.id),
            })
        }
        OutsideBoundaryCondition::Adiabatic
        | OutsideBoundaryCondition::Foundation
        | OutsideBoundaryCondition::Ground
        | OutsideBoundaryCondition::Outdoors
        | OutsideBoundaryCondition::Other => Ok(SurfaceBoundaryTarget::default()),
    }
}

fn boundary_object_name(surface: &Surface) -> String {
    surface
        .outside_boundary_condition_object
        .as_ref()
        .map(|name| name.0.clone())
        .unwrap_or_default()
}

fn surface_boundary_temperature_c(
    surface: &SurfaceHeatBalanceState,
    previous_zone_temperatures: &BTreeMap<ZoneId, f64>,
    outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
) -> f64 {
    match surface.outside_boundary_condition {
        OutsideBoundaryCondition::Outdoors => outdoor_dry_bulb_c,
        OutsideBoundaryCondition::Adiabatic => owning_zone_temperature_c,
        OutsideBoundaryCondition::Surface
        | OutsideBoundaryCondition::Zone
        | OutsideBoundaryCondition::Space => surface
            .outside_boundary_target_zone_id
            .and_then(|target_zone_id| previous_zone_temperatures.get(&target_zone_id).copied())
            .unwrap_or(owning_zone_temperature_c),
        OutsideBoundaryCondition::Foundation
        | OutsideBoundaryCondition::Ground
        | OutsideBoundaryCondition::Other => surface.outside_face_temperature_c,
    }
}

fn heat_balance_surface_boundary_temperature_c(
    model: &TypedModel,
    surface: &SurfaceHeatBalanceState,
    previous_zone_temperatures: &BTreeMap<ZoneId, f64>,
    outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    quick_outside_conduction: Option<QuickOutsideConductionContext>,
) -> f64 {
    if surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors {
        return exterior_surface_boundary_temperature_c(
            model,
            surface,
            outdoor_dry_bulb_c,
            owning_zone_temperature_c,
            weather_context,
            quick_outside_conduction,
        );
    }

    surface_boundary_temperature_c(
        surface,
        previous_zone_temperatures,
        outdoor_dry_bulb_c,
        owning_zone_temperature_c,
    )
}

fn exterior_surface_boundary_temperature_c(
    model: &TypedModel,
    surface_state: &SurfaceHeatBalanceState,
    outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    quick_outside_conduction: Option<QuickOutsideConductionContext>,
) -> f64 {
    let Some(context) = weather_context else {
        return outdoor_dry_bulb_c;
    };
    let Some(record) = context.records.get(context.record_index) else {
        return outdoor_dry_bulb_c;
    };
    let Some(typed_surface) = model
        .surfaces
        .iter()
        .find(|surface| surface.id == surface_state.surface_id)
    else {
        return outdoor_dry_bulb_c;
    };
    if !matches!(
        typed_surface.surface_type,
        SurfaceType::Roof | SurfaceType::Wall
    ) {
        return outdoor_dry_bulb_c;
    }

    let incident_solar_w_per_m2 = if typed_surface.sun_exposure == SunExposure::SunExposed {
        let Some(site) = model.site.as_ref() else {
            return exterior_surface_energy_balance_temperature_c(
                surface_state,
                typed_surface,
                record,
                outdoor_dry_bulb_c,
                owning_zone_temperature_c,
                0.0,
                quick_outside_conduction,
            );
        };
        surface_incident_solar_radiation_hourly_average_w_per_m2(
            typed_surface,
            site,
            context.records,
            context.record_index,
            context.zone_steps_per_hour,
        )
    } else {
        0.0
    };
    exterior_surface_energy_balance_temperature_c(
        surface_state,
        typed_surface,
        record,
        outdoor_dry_bulb_c,
        owning_zone_temperature_c,
        incident_solar_w_per_m2,
        quick_outside_conduction,
    )
}

fn reported_surface_outside_face_temperature_c(
    model: &TypedModel,
    surface_state: &SurfaceHeatBalanceState,
    outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> f64 {
    if surface_state.outside_boundary_condition != OutsideBoundaryCondition::Outdoors {
        return surface_state.outside_face_temperature_c;
    }
    if matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
    ) {
        return surface_state.outside_face_temperature_c;
    }

    exterior_surface_boundary_temperature_c(
        model,
        surface_state,
        outdoor_dry_bulb_c,
        owning_zone_temperature_c,
        weather_context,
        None,
    )
}

/// EnergyPlus ASHRAE TARP inside natural convection coefficient for one surface.
#[must_use]
pub fn energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
    surface: &SurfaceHeatBalanceState,
    surface_temperature_c: f64,
    air_temperature_c: f64,
) -> f64 {
    let inside_cos_tilt = -surface.tilt_deg.to_radians().cos();
    let coefficient = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(
        surface_temperature_c,
        air_temperature_c,
        inside_cos_tilt,
    );
    if !coefficient.is_finite() {
        return ENERGYPLUS_LOW_CONVECTION_LIMIT_W_PER_M2_K;
    }

    coefficient.clamp(
        ENERGYPLUS_LOW_CONVECTION_LIMIT_W_PER_M2_K,
        ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K,
    )
}

fn energyplus_ashrae_tarp_natural_convection_w_per_m2_k(
    surface_temperature_c: f64,
    air_temperature_c: f64,
    cos_tilt: f64,
) -> f64 {
    let delta_temperature_c = surface_temperature_c - air_temperature_c;
    if delta_temperature_c.abs() <= f64::EPSILON || cos_tilt.abs() <= 1.0e-12 {
        return energyplus_ashrae_vertical_wall_convection_w_per_m2_k(delta_temperature_c);
    }

    if (delta_temperature_c < 0.0 && cos_tilt < 0.0)
        || (delta_temperature_c > 0.0 && cos_tilt > 0.0)
    {
        energyplus_walton_unstable_horizontal_or_tilt_convection_w_per_m2_k(
            delta_temperature_c,
            cos_tilt,
        )
    } else {
        energyplus_walton_stable_horizontal_or_tilt_convection_w_per_m2_k(
            delta_temperature_c,
            cos_tilt,
        )
    }
}

fn energyplus_ashrae_vertical_wall_convection_w_per_m2_k(delta_temperature_c: f64) -> f64 {
    1.31 * delta_temperature_c.abs().powf(1.0 / 3.0)
}

fn energyplus_walton_unstable_horizontal_or_tilt_convection_w_per_m2_k(
    delta_temperature_c: f64,
    cos_tilt: f64,
) -> f64 {
    9.482 * delta_temperature_c.abs().powf(1.0 / 3.0) / (7.238 - cos_tilt.abs())
}

fn energyplus_walton_stable_horizontal_or_tilt_convection_w_per_m2_k(
    delta_temperature_c: f64,
    cos_tilt: f64,
) -> f64 {
    1.810 * delta_temperature_c.abs().powf(1.0 / 3.0) / (1.382 + cos_tilt.abs())
}

fn surface_inside_ctf_source_terms_w_per_m2(surface: &SurfaceHeatBalanceState) -> f64 {
    surface.inside_radiant_internal_gain_w_per_m2
        + surface.inside_shortwave_absorbed_w_per_m2
        + surface.inside_additional_heat_source_w_per_m2
        + surface.inside_radiant_hvac_w_per_m2
        + surface.inside_net_longwave_w_per_m2
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct InteriorLongwaveSurfaceSnapshot {
    zone_id: ZoneId,
    area_m2: f64,
    temperature_k4: f64,
    thermal_absorptance: f64,
}

fn update_surface_inside_longwave_exchange_probe(
    surfaces: &mut [SurfaceHeatBalanceState],
    temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) {
    let snapshots = surfaces
        .iter()
        .map(|surface| {
            let temperature_c = temperature_overrides
                .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                .unwrap_or(surface.inside_face_temperature_c);
            let temperature_k = (temperature_c + KELVIN_OFFSET).max(0.0);
            InteriorLongwaveSurfaceSnapshot {
                zone_id: surface.zone_id,
                area_m2: surface.area_m2.max(0.0),
                temperature_k4: temperature_k.powi(4),
                thermal_absorptance: surface.thermal_absorptance.clamp(0.0, 1.0),
            }
        })
        .collect::<Vec<_>>();
    let mut zone_area_m2 = BTreeMap::<ZoneId, f64>::new();
    for snapshot in &snapshots {
        *zone_area_m2.entry(snapshot.zone_id).or_insert(0.0) += snapshot.area_m2;
    }

    let mut longwave_terms_w_per_m2 = vec![0.0; surfaces.len()];
    for (receiver_index, receiver) in snapshots.iter().enumerate() {
        let Some(zone_area_m2) = zone_area_m2.get(&receiver.zone_id).copied() else {
            continue;
        };
        if zone_area_m2 <= f64::EPSILON || receiver.area_m2 <= f64::EPSILON {
            continue;
        }

        let mut net_longwave_w_per_m2 = 0.0;
        for (sender_index, sender) in snapshots.iter().enumerate() {
            if sender_index == receiver_index
                || sender.zone_id != receiver.zone_id
                || sender.area_m2 <= f64::EPSILON
            {
                continue;
            }
            let exchange_emissivity = grey_pair_exchange_emissivity(
                receiver.thermal_absorptance,
                sender.thermal_absorptance,
            );
            if exchange_emissivity <= f64::EPSILON {
                continue;
            }
            let view_factor = sender.area_m2 / zone_area_m2;
            net_longwave_w_per_m2 += STEFAN_BOLTZMANN_W_PER_M2_K4
                * exchange_emissivity
                * view_factor
                * (sender.temperature_k4 - receiver.temperature_k4);
        }
        longwave_terms_w_per_m2[receiver_index] = net_longwave_w_per_m2;
    }

    for (surface, net_longwave_w_per_m2) in
        surfaces.iter_mut().zip(longwave_terms_w_per_m2.into_iter())
    {
        surface.inside_net_longwave_w_per_m2 = net_longwave_w_per_m2;
    }
}

fn grey_pair_exchange_emissivity(receiver_emissivity: f64, sender_emissivity: f64) -> f64 {
    let receiver = receiver_emissivity.clamp(0.0, 1.0);
    let sender = sender_emissivity.clamp(0.0, 1.0);
    if receiver <= f64::EPSILON || sender <= f64::EPSILON {
        return 0.0;
    }
    1.0 / ((1.0 / receiver) + (1.0 / sender) - 1.0)
}

/// EnergyPlus-shaped CTF inside-face temperature balance for the opaque subset.
///
/// This covers the no-pool/no-movable-insulation branch documented in
/// `CalcHeatBalanceInsideSurf2CTFOnly`. Inside shortwave, radiant, additional
/// heat-source, HVAC radiant, and net longwave terms are passed through the
/// source-map slots on `SurfaceHeatBalanceState`.
#[must_use]
pub fn energyplus_ctf_inside_face_temperature_c(
    surface: &SurfaceHeatBalanceState,
    input: CtfInsideFaceBalanceInput,
) -> f64 {
    let adiabatic_cross =
        if surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic {
            surface.ctf.cross_0_w_per_m2_k
        } else {
            0.0
        };
    let outside_temperature_term =
        if surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic {
            0.0
        } else {
            surface.ctf.cross_0_w_per_m2_k * surface.outside_face_temperature_c
        };
    let denominator = surface.ctf.inside_0_w_per_m2_k - adiabatic_cross
        + input.inside_convection_coefficient_w_per_m2_k
        + ENERGYPLUS_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K;
    if denominator.abs() <= f64::EPSILON {
        return surface.inside_face_temperature_c;
    }

    (surface.ctf.const_in_part_w_per_m2
        + input.net_inside_source_w_per_m2
        + input.inside_convection_coefficient_w_per_m2_k * input.reference_air_temperature_c
        + ENERGYPLUS_INSIDE_SURFACE_ITER_DAMP_W_PER_M2_K * input.previous_inside_face_temperature_c
        + outside_temperature_term)
        / denominator
}

/// EnergyPlus-shaped CTF outside-face environmental balance for the opaque subset.
#[must_use]
pub fn energyplus_ctf_outside_face_temperature_c(
    surface: &SurfaceHeatBalanceState,
    input: CtfOutsideFaceBalanceInput,
) -> f64 {
    let denominator = surface.ctf.outside_0_w_per_m2_k
        + input.outside_convection_coefficient_w_per_m2_k
        + input.outside_radiation_coefficient_w_per_m2_k;
    if denominator.abs() <= f64::EPSILON {
        return input.outdoor_air_temperature_c;
    }

    (-surface.ctf.const_out_part_w_per_m2
        + input.absorbed_outside_source_w_per_m2
        + input.outside_convection_coefficient_w_per_m2_k * input.outdoor_air_temperature_c
        + input.outside_radiation_coefficient_w_per_m2_k * input.radiant_temperature_c
        + surface.ctf.cross_0_w_per_m2_k * surface.inside_face_temperature_c)
        / denominator
}

/// EnergyPlus-shaped quick-conduction outside-face balance for the opaque subset.
#[must_use]
pub fn energyplus_ctf_outside_face_temperature_quick_conduction_c(
    surface: &SurfaceHeatBalanceState,
    input: CtfOutsideQuickConductionBalanceInput,
) -> f64 {
    let inside_denominator =
        surface.ctf.inside_0_w_per_m2_k + input.inside_convection_coefficient_w_per_m2_k;
    if surface.ctf.cross_0_w_per_m2_k <= ENERGYPLUS_QUICK_CONDUCTION_CROSS_THRESHOLD_W_PER_M2_K
        || inside_denominator.abs() <= f64::EPSILON
    {
        return energyplus_ctf_outside_face_temperature_c(surface, input.environmental);
    }

    let f1 = surface.ctf.cross_0_w_per_m2_k / inside_denominator;
    let denominator = surface.ctf.outside_0_w_per_m2_k
        + input
            .environmental
            .outside_convection_coefficient_w_per_m2_k
        + input.environmental.outside_radiation_coefficient_w_per_m2_k
        - f1 * surface.ctf.cross_0_w_per_m2_k;
    if denominator.abs() <= f64::EPSILON {
        return energyplus_ctf_outside_face_temperature_c(surface, input.environmental);
    }

    let inside_balance_term = surface.ctf.const_in_part_w_per_m2
        + input.net_inside_source_w_per_m2
        + input.inside_convection_coefficient_w_per_m2_k * input.reference_air_temperature_c;
    (-surface.ctf.const_out_part_w_per_m2
        + input.environmental.absorbed_outside_source_w_per_m2
        + input
            .environmental
            .outside_convection_coefficient_w_per_m2_k
            * input.environmental.outdoor_air_temperature_c
        + input.environmental.outside_radiation_coefficient_w_per_m2_k
            * input.environmental.radiant_temperature_c
        + f1 * inside_balance_term)
        / denominator
}

fn exterior_surface_energy_balance_temperature_c(
    surface_state: &SurfaceHeatBalanceState,
    typed_surface: &Surface,
    record: &EpwRecord,
    outdoor_dry_bulb_c: f64,
    _owning_zone_temperature_c: f64,
    incident_solar_w_per_m2: f64,
    quick_outside_conduction: Option<QuickOutsideConductionContext>,
) -> f64 {
    if quick_outside_conduction.is_none()
        && (record.liquid_precipitation_depth_mm >= EXTERIOR_RAIN_FALLBACK_DEPTH_MM
            || incident_solar_w_per_m2 < EXTERIOR_SOLAR_FORCING_THRESHOLD_W_PER_M2)
    {
        return outdoor_dry_bulb_c;
    }

    let thermal_absorptance = surface_state.thermal_absorptance.clamp(0.0, 1.0);
    let solar_absorptance = surface_state.solar_absorptance.clamp(0.0, 1.0);
    let tilt_rad =
        surface_tilt_deg(typed_surface.surface_type, &typed_surface.vertices).to_radians();
    let use_doe2_outside_convection = quick_outside_conduction
        .map(|context| context.use_doe2_outside_convection)
        .unwrap_or(false);
    let convection_coefficient = if use_doe2_outside_convection {
        energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            surface_state.outside_face_temperature_c,
            outdoor_dry_bulb_c,
            tilt_rad.cos(),
            surface_azimuth_deg(&typed_surface.vertices),
            record.wind_direction_deg,
            record.wind_speed_m_per_s,
            surface_state.outside_layer_roughness,
        )
    } else {
        exterior_convection_coefficient_w_per_m2_k(record.wind_speed_m_per_s)
    };
    let longwave_coefficient = EXTERIOR_LONGWAVE_COEFFICIENT_W_PER_M2_K * thermal_absorptance;
    let sky_temperature_c = horizontal_infrared_sky_temperature_c(
        record.horizontal_infrared_radiation_wh_per_m2,
        outdoor_dry_bulb_c,
    );
    let sky_view_factor = surface_sky_view_factor(typed_surface, tilt_rad);
    let ground_view_factor = surface_ground_view_factor(typed_surface, tilt_rad);
    let longwave_radiant_temperature_c = sky_view_factor * sky_temperature_c
        + ground_view_factor * outdoor_dry_bulb_c
        + (1.0 - sky_view_factor - ground_view_factor).max(0.0) * outdoor_dry_bulb_c;

    let environmental = CtfOutsideFaceBalanceInput {
        outdoor_air_temperature_c: outdoor_dry_bulb_c,
        radiant_temperature_c: longwave_radiant_temperature_c,
        outside_convection_coefficient_w_per_m2_k: convection_coefficient,
        outside_radiation_coefficient_w_per_m2_k: longwave_coefficient,
        absorbed_outside_source_w_per_m2: solar_absorptance * incident_solar_w_per_m2.max(0.0),
    };
    if let Some(context) = quick_outside_conduction {
        energyplus_ctf_outside_face_temperature_quick_conduction_c(
            surface_state,
            CtfOutsideQuickConductionBalanceInput {
                environmental,
                reference_air_temperature_c: context.reference_air_temperature_c,
                inside_convection_coefficient_w_per_m2_k: context
                    .inside_convection_coefficient_w_per_m2_k,
                net_inside_source_w_per_m2: context.net_inside_source_w_per_m2,
            },
        )
    } else {
        energyplus_ctf_outside_face_temperature_c(surface_state, environmental)
    }
}

fn exterior_convection_coefficient_w_per_m2_k(wind_speed_m_per_s: f64) -> f64 {
    13.0 + 2.5 * wind_speed_m_per_s.max(0.0)
}

/// EnergyPlus DOE-2 outside convection coefficient for future exterior balance wiring.
#[must_use]
pub fn energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
    surface_temperature_c: f64,
    air_temperature_c: f64,
    cos_tilt: f64,
    surface_azimuth_deg: f64,
    wind_direction_deg: f64,
    wind_speed_m_per_s: f64,
    roughness: MaterialSurfaceRoughness,
) -> f64 {
    let h_n = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(
        surface_temperature_c,
        air_temperature_c,
        cos_tilt,
    );
    let h_f_smooth =
        if energyplus_surface_is_windward(cos_tilt, surface_azimuth_deg, wind_direction_deg) {
            energyplus_mowitt_forced_windward_w_per_m2_k(wind_speed_m_per_s)
        } else {
            energyplus_mowitt_forced_leeward_w_per_m2_k(wind_speed_m_per_s)
        };
    let h_c_smooth = (h_n.powi(2) + h_f_smooth.powi(2)).sqrt();
    let h_f = energyplus_roughness_multiplier(roughness) * (h_c_smooth - h_n);
    h_n + h_f
}

fn energyplus_surface_is_windward(
    cos_tilt: f64,
    surface_azimuth_deg: f64,
    wind_direction_deg: f64,
) -> bool {
    if cos_tilt.abs() >= 0.98 {
        return true;
    }

    let mut diff = (wind_direction_deg - surface_azimuth_deg).abs();
    if diff - 180.0 > 0.001 {
        diff -= 360.0;
    }
    diff.abs() - 90.0 <= 0.001
}

fn energyplus_mowitt_forced_windward_w_per_m2_k(wind_speed_m_per_s: f64) -> f64 {
    3.26 * wind_speed_m_per_s.max(0.0).powf(0.89)
}

fn energyplus_mowitt_forced_leeward_w_per_m2_k(wind_speed_m_per_s: f64) -> f64 {
    3.55 * wind_speed_m_per_s.max(0.0).powf(0.617)
}

fn energyplus_roughness_multiplier(roughness: MaterialSurfaceRoughness) -> f64 {
    match roughness {
        MaterialSurfaceRoughness::VeryRough => 2.17,
        MaterialSurfaceRoughness::Rough => 1.67,
        MaterialSurfaceRoughness::MediumRough => 1.52,
        MaterialSurfaceRoughness::MediumSmooth => 1.13,
        MaterialSurfaceRoughness::Smooth => 1.11,
        MaterialSurfaceRoughness::VerySmooth => 1.0,
    }
}

fn horizontal_infrared_sky_temperature_c(
    horizontal_infrared_radiation_w_per_m2: f64,
    fallback_air_temperature_c: f64,
) -> f64 {
    if horizontal_infrared_radiation_w_per_m2 <= 0.0 {
        return fallback_air_temperature_c;
    }

    (horizontal_infrared_radiation_w_per_m2 / STEFAN_BOLTZMANN_W_PER_M2_K4).powf(0.25)
        - KELVIN_OFFSET
}

fn surface_inside_conduction_rate_w(surface: &SurfaceHeatBalanceState) -> f64 {
    surface.area_m2 * surface_inside_conduction_flux_w_per_m2(surface)
}

fn surface_outside_conduction_rate_w(surface: &SurfaceHeatBalanceState) -> f64 {
    -surface.area_m2 * surface_outside_conduction_flux_w_per_m2(surface)
}

fn surface_heat_storage_rate_w(inside_rate_w: f64, outside_rate_w: f64) -> f64 {
    -(inside_rate_w + outside_rate_w)
}

fn surface_inside_conduction_flux_w_per_m2(surface: &SurfaceHeatBalanceState) -> f64 {
    if surface.area_m2 <= 0.0 {
        return 0.0;
    }

    surface.outside_face_temperature_c * surface.ctf.cross_0_w_per_m2_k
        - surface.inside_face_temperature_c * surface.ctf.inside_0_w_per_m2_k
        + surface.ctf.const_in_part_w_per_m2
}

fn surface_outside_conduction_flux_w_per_m2(surface: &SurfaceHeatBalanceState) -> f64 {
    if surface.area_m2 <= 0.0 {
        return 0.0;
    }

    surface.outside_face_temperature_c * surface.ctf.outside_0_w_per_m2_k
        - surface.inside_face_temperature_c * surface.ctf.cross_0_w_per_m2_k
        + surface.ctf.const_out_part_w_per_m2
}

fn update_surface_ctf_history_constants(surface: &mut SurfaceHeatBalanceState) {
    surface.ctf.const_in_part_w_per_m2 = 0.0;
    surface.ctf.const_out_part_w_per_m2 = 0.0;
    let terms = surface
        .ctf
        .outside_history_w_per_m2_k
        .len()
        .max(surface.ctf.cross_history_w_per_m2_k.len())
        .max(surface.ctf.inside_history_w_per_m2_k.len())
        .max(surface.ctf.flux_history.len());

    for term in 0..terms {
        let outside_temperature_c = surface
            .ctf
            .outside_temperature_history_c
            .get(term)
            .copied()
            .unwrap_or(surface.outside_face_temperature_c);
        let inside_temperature_c = surface
            .ctf
            .inside_temperature_history_c
            .get(term)
            .copied()
            .unwrap_or(surface.inside_face_temperature_c);
        let inside_flux_w_per_m2 = surface
            .ctf
            .inside_flux_history_w_per_m2
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let outside_flux_w_per_m2 = surface
            .ctf
            .outside_flux_history_w_per_m2
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let cross = surface
            .ctf
            .cross_history_w_per_m2_k
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let inside = surface
            .ctf
            .inside_history_w_per_m2_k
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let outside = surface
            .ctf
            .outside_history_w_per_m2_k
            .get(term)
            .copied()
            .unwrap_or(0.0);
        let flux = surface.ctf.flux_history.get(term).copied().unwrap_or(0.0);

        surface.ctf.const_in_part_w_per_m2 += cross * outside_temperature_c
            - inside * inside_temperature_c
            + flux * inside_flux_w_per_m2;
        surface.ctf.const_out_part_w_per_m2 += outside * outside_temperature_c
            - cross * inside_temperature_c
            + flux * outside_flux_w_per_m2;
    }
}

fn advance_surface_ctf_histories(surface: &mut SurfaceHeatBalanceState) {
    let history_terms = surface
        .ctf
        .outside_history_w_per_m2_k
        .len()
        .max(surface.ctf.cross_history_w_per_m2_k.len())
        .max(surface.ctf.inside_history_w_per_m2_k.len())
        .max(surface.ctf.flux_history.len());
    if history_terms == 0 {
        return;
    }

    let inside_flux_w_per_m2 = surface_inside_conduction_flux_w_per_m2(surface);
    let outside_flux_w_per_m2 = surface_outside_conduction_flux_w_per_m2(surface);
    push_surface_history(
        &mut surface.ctf.outside_temperature_history_c,
        surface.outside_face_temperature_c,
        history_terms,
    );
    push_surface_history(
        &mut surface.ctf.inside_temperature_history_c,
        surface.inside_face_temperature_c,
        history_terms,
    );
    push_surface_history(
        &mut surface.ctf.inside_flux_history_w_per_m2,
        inside_flux_w_per_m2,
        history_terms,
    );
    push_surface_history(
        &mut surface.ctf.outside_flux_history_w_per_m2,
        outside_flux_w_per_m2,
        history_terms,
    );
}

fn push_surface_history(history: &mut Vec<f64>, value: f64, limit: usize) {
    history.insert(0, value);
    history.truncate(limit);
}

fn surface_rate_per_area_w_per_m2(rate_w: f64, area_m2: f64) -> f64 {
    if area_m2 > 0.0 { rate_w / area_m2 } else { 0.0 }
}

fn surface_incident_solar_radiation_hourly_average_w_per_m2(
    surface: &Surface,
    site: &SiteLocation,
    weather_records: &[EpwRecord],
    record_index: usize,
    zone_steps_per_hour: u32,
) -> f64 {
    let Some(record) = weather_records.get(record_index) else {
        return 0.0;
    };
    let Some((sin_declination, cos_declination, equation_of_time_hours)) =
        energyplus_shadowing_period_solar_coefficients(weather_records, record_index)
    else {
        return 0.0;
    };
    let steps = zone_steps_per_hour.max(1);
    let mut total = 0.0;
    for timestep in 1..=steps {
        let (previous_weight, current_weight, next_weight) =
            solar_weather_interpolation_weights(steps, timestep);
        let previous = previous_weather_record(weather_records, record_index);
        let next = next_weather_record(weather_records, record_index);
        let direct_normal = weighted_solar_value(
            previous.direct_normal_radiation_wh_per_m2,
            record.direct_normal_radiation_wh_per_m2,
            next.direct_normal_radiation_wh_per_m2,
            previous_weight,
            current_weight,
            next_weight,
        );
        let diffuse_horizontal = weighted_solar_value(
            previous.diffuse_horizontal_radiation_wh_per_m2,
            record.diffuse_horizontal_radiation_wh_per_m2,
            next.diffuse_horizontal_radiation_wh_per_m2,
            previous_weight,
            current_weight,
            next_weight,
        );
        let local_hour =
            f64::from(record.hour.saturating_sub(1)) + f64::from(timestep) / f64::from(steps);
        let actual_sun_is_up = solar_position_rad_at_local_hour(site, record, local_hour).is_some();
        total += surface_incident_solar_radiation_at_local_hour_w_per_m2(
            surface,
            site,
            SurfaceSolarTimestepInput {
                local_hour,
                actual_sun_is_up,
                sin_declination,
                cos_declination,
                equation_of_time_hours,
                direct_normal_radiation_w_per_m2: direct_normal,
                diffuse_horizontal_radiation_w_per_m2: diffuse_horizontal,
            },
        );
    }

    total / f64::from(steps)
}

#[derive(Clone, Copy)]
struct SurfaceSolarTimestepInput {
    local_hour: f64,
    actual_sun_is_up: bool,
    sin_declination: f64,
    cos_declination: f64,
    equation_of_time_hours: f64,
    direct_normal_radiation_w_per_m2: f64,
    diffuse_horizontal_radiation_w_per_m2: f64,
}

fn surface_incident_solar_radiation_at_local_hour_w_per_m2(
    surface: &Surface,
    site: &SiteLocation,
    input: SurfaceSolarTimestepInput,
) -> f64 {
    if !input.actual_sun_is_up {
        return 0.0;
    }

    let Some((solar_altitude_rad, solar_azimuth_rad)) = solar_position_rad_from_coefficients(
        site,
        input.local_hour,
        input.sin_declination,
        input.cos_declination,
        input.equation_of_time_hours,
    ) else {
        return 0.0;
    };
    if solar_altitude_rad <= 0.0 {
        return 0.0;
    }

    let tilt_rad = surface_tilt_deg(surface.surface_type, &surface.vertices).to_radians();
    let surface_azimuth_rad = surface_azimuth_deg(&surface.vertices).to_radians();
    let direct_normal = input.direct_normal_radiation_w_per_m2.max(0.0);
    let diffuse_horizontal = input.diffuse_horizontal_radiation_w_per_m2.max(0.0);

    let cos_incidence = solar_altitude_rad.sin() * tilt_rad.cos()
        + solar_altitude_rad.cos()
            * tilt_rad.sin()
            * (solar_azimuth_rad - surface_azimuth_rad).cos();
    let beam = direct_normal * cos_incidence.max(0.0);
    let sky_diffuse = diffuse_horizontal * (1.0 + tilt_rad.cos()) * 0.5;
    let ground_horizontal =
        (direct_normal * solar_altitude_rad.sin() + diffuse_horizontal).max(0.0);
    let ground_reflected = ground_horizontal
        * DEFAULT_SOLAR_GROUND_REFLECTANCE
        * surface_ground_view_factor(surface, tilt_rad);

    beam + sky_diffuse + ground_reflected
}

fn previous_weather_record(records: &[EpwRecord], record_index: usize) -> &EpwRecord {
    if record_index == 0 {
        &records[records.len() - 1]
    } else {
        &records[record_index - 1]
    }
}

fn next_weather_record(records: &[EpwRecord], record_index: usize) -> &EpwRecord {
    let next_index = if record_index + 1 >= records.len() {
        0
    } else {
        record_index + 1
    };
    &records[next_index]
}

fn weighted_solar_value(
    previous: f64,
    current: f64,
    next: f64,
    previous_weight: f64,
    current_weight: f64,
    next_weight: f64,
) -> f64 {
    previous.max(0.0) * previous_weight
        + current.max(0.0) * current_weight
        + next.max(0.0) * next_weight
}

fn solar_weather_interpolation_weights(zone_steps_per_hour: u32, timestep: u32) -> (f64, f64, f64) {
    let steps = zone_steps_per_hour.max(1);
    let timestep = timestep.clamp(1, steps);
    let current_weight = solar_interpolation_weight(steps, timestep);
    if steps == 1 {
        return (0.0, current_weight, 0.0);
    }
    let timestep_fraction = 1.0 / f64::from(steps);
    if (current_weight - 1.0).abs() <= f64::EPSILON {
        (0.0, current_weight, 0.0)
    } else if f64::from(timestep) * timestep_fraction < 0.5 {
        (1.0 - current_weight, current_weight, 0.0)
    } else {
        (0.0, current_weight, 1.0 - current_weight)
    }
}

fn solar_interpolation_weight(zone_steps_per_hour: u32, timestep: u32) -> f64 {
    let steps = zone_steps_per_hour.max(1);
    let timestep = timestep.clamp(1, steps);
    if steps.is_multiple_of(2) {
        let halfpoint = steps / 2;
        let distance = timestep.abs_diff(halfpoint);
        return 1.0 - f64::from(distance) / f64::from(steps);
    }

    if steps == 1 {
        0.5
    } else if steps == 3 {
        match timestep {
            1 | 2 => 5.0 / 6.0,
            _ => 0.5,
        }
    } else {
        let timestep_weight = 1.0 / f64::from(steps);
        let halfpoint = steps / 2;
        let peak_weight = 1.0 - timestep_weight / 2.0;
        if timestep == halfpoint || timestep == halfpoint + 1 {
            peak_weight
        } else if timestep > halfpoint + 1 {
            peak_weight - f64::from(timestep - (halfpoint + 1)) * timestep_weight
        } else {
            peak_weight - f64::from(halfpoint - timestep) * timestep_weight
        }
    }
}

fn surface_ground_view_factor(surface: &Surface, tilt_rad: f64) -> f64 {
    match surface.view_factor_to_ground {
        AutoOrNumber::Value(value) => value.clamp(0.0, 1.0),
        AutoOrNumber::AutoCalculate => ((1.0 - tilt_rad.cos()) * 0.5).clamp(0.0, 1.0),
    }
}

fn surface_sky_view_factor(surface: &Surface, tilt_rad: f64) -> f64 {
    match surface.view_factor_to_ground {
        AutoOrNumber::Value(value) => (1.0 - value).clamp(0.0, 1.0),
        AutoOrNumber::AutoCalculate => ((1.0 + tilt_rad.cos()) * 0.5).clamp(0.0, 1.0),
    }
}

fn solar_position_rad_at_local_hour(
    site: &SiteLocation,
    record: &EpwRecord,
    local_hour: f64,
) -> Option<(f64, f64)> {
    let day = day_of_year(record.year, record.month, record.day)?;
    let (sin_declination, cos_declination, equation_of_time_hours) =
        energyplus_daily_solar_coefficients(day);
    solar_position_rad_from_coefficients(
        site,
        local_hour,
        sin_declination,
        cos_declination,
        equation_of_time_hours,
    )
}

fn solar_position_rad_from_coefficients(
    site: &SiteLocation,
    local_hour: f64,
    sin_declination: f64,
    cos_declination: f64,
    equation_of_time_hours: f64,
) -> Option<(f64, f64)> {
    let latitude_rad = site.latitude_deg.to_radians();
    let sin_latitude = latitude_rad.sin();
    let cos_latitude = latitude_rad.cos();
    let time_zone_meridian_deg = 15.0 * site.time_zone_hours;
    let hour_angle_deg = 15.0 * (12.0 - (local_hour + equation_of_time_hours))
        + (time_zone_meridian_deg - site.longitude_deg);
    let hour_angle_rad = hour_angle_deg.to_radians();

    let cos_zenith =
        sin_declination * sin_latitude + cos_declination * cos_latitude * hour_angle_rad.cos();
    if cos_zenith < ENERGYPLUS_SUN_IS_UP_COS_ZENITH {
        return None;
    }

    let altitude_rad = cos_zenith.clamp(-1.0, 1.0).asin();
    let solar_zenith_rad = cos_zenith.clamp(-1.0, 1.0).acos();
    let azimuth_denominator = cos_latitude * solar_zenith_rad.sin();
    let mut azimuth_rad = if azimuth_denominator.abs() > 1.0e-12 {
        let cos_azimuth = -((sin_latitude * cos_zenith - sin_declination) / azimuth_denominator);
        cos_azimuth.clamp(-1.0, 1.0).acos()
    } else {
        0.0
    };
    if hour_angle_deg < 0.0 {
        azimuth_rad = 2.0 * std::f64::consts::PI - azimuth_rad;
    }

    Some((altitude_rad, azimuth_rad))
}

fn energyplus_shadowing_period_solar_coefficients(
    weather_records: &[EpwRecord],
    record_index: usize,
) -> Option<(f64, f64, f64)> {
    if weather_records.is_empty() {
        return None;
    }

    let total_days = weather_records.len().div_ceil(24);
    let day_of_sim_zero = record_index / 24;
    let period_start_day_zero = (day_of_sim_zero / ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS)
        * ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS;
    let period_length = ENERGYPLUS_SHADOWING_CALC_FREQUENCY_DAYS
        .min(total_days.saturating_sub(period_start_day_zero))
        .max(1);
    let period_start_record = weather_records.get(period_start_day_zero * 24)?;
    let period_start_day_of_year = day_of_year(
        period_start_record.year,
        period_start_record.month,
        period_start_record.day,
    )?;

    Some(energyplus_average_solar_coefficients(
        period_start_day_of_year,
        period_length,
    ))
}

fn energyplus_average_solar_coefficients(
    start_day_of_year: u32,
    day_count: usize,
) -> (f64, f64, f64) {
    let day_count = day_count.max(1);
    let mut sin_declination_sum = 0.0;
    let mut equation_of_time_sum = 0.0;
    for offset in 0..day_count {
        let (sin_declination, _cos_declination, equation_of_time_hours) =
            energyplus_daily_solar_coefficients(start_day_of_year + offset as u32);
        sin_declination_sum += sin_declination;
        equation_of_time_sum += equation_of_time_hours;
    }

    let sin_declination = sin_declination_sum / day_count as f64;
    let cos_declination = (1.0 - sin_declination.powi(2)).sqrt();
    let equation_of_time_hours = equation_of_time_sum / day_count as f64;

    (sin_declination, cos_declination, equation_of_time_hours)
}

fn energyplus_daily_solar_coefficients(day_of_year: u32) -> (f64, f64, f64) {
    const SINE_SOLAR_DECLINATION_COEFFICIENTS: [f64; 9] = [
        0.00561800,
        0.0657911,
        -0.392779,
        0.00064440,
        -0.00618495,
        -0.00010101,
        -0.00007951,
        -0.00011691,
        0.00002096,
    ];
    const EQUATION_OF_TIME_COEFFICIENTS: [f64; 9] = [
        0.00021971,
        -0.122649,
        0.00762856,
        -0.156308,
        -0.0530028,
        -0.00388702,
        -0.00123978,
        -0.00270502,
        -0.00167992,
    ];

    let angle = 2.0 * std::f64::consts::PI * f64::from(day_of_year) / 366.0;
    let sin_x = angle.sin();
    let cos_x = angle.cos();
    let sin_2x = sin_x * cos_x * 2.0;
    let cos_2x = cos_x.powi(2) - sin_x.powi(2);
    let sin_3x = sin_x * cos_2x + cos_x * sin_2x;
    let cos_3x = cos_x * cos_2x - sin_x * sin_2x;
    let sin_4x = 2.0 * sin_2x * cos_2x;
    let cos_4x = cos_2x.powi(2) - sin_2x.powi(2);
    let basis = [
        1.0, sin_x, cos_x, sin_2x, cos_2x, sin_3x, cos_3x, sin_4x, cos_4x,
    ];

    let sin_declination = SINE_SOLAR_DECLINATION_COEFFICIENTS
        .iter()
        .zip(basis)
        .map(|(coefficient, term)| coefficient * term)
        .sum::<f64>();
    let cos_declination = (1.0 - sin_declination.powi(2)).sqrt();
    let equation_of_time_hours = EQUATION_OF_TIME_COEFFICIENTS
        .iter()
        .zip(basis)
        .map(|(coefficient, term)| coefficient * term)
        .sum::<f64>();

    (sin_declination, cos_declination, equation_of_time_hours)
}

fn heat_gain_rate_w(rate_w: f64) -> f64 {
    rate_w.max(0.0)
}

fn heat_loss_rate_w(rate_w: f64) -> f64 {
    (-rate_w).max(0.0)
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

/// Builds per-surface geometry summaries from the typed model.
#[must_use]
pub fn surface_geometry_summaries(model: &TypedModel) -> Vec<SurfaceGeometrySummary> {
    model
        .surfaces
        .iter()
        .map(|surface| {
            let zone_name = model
                .zones
                .iter()
                .find(|zone| zone.id == surface.zone)
                .map(|zone| zone.name.0.clone())
                .unwrap_or_else(|| "UNKNOWN".to_string());

            SurfaceGeometrySummary {
                surface_id: surface.id,
                surface_name: surface.name.0.clone(),
                zone_name,
                surface_type: surface.surface_type,
                area_m2: surface_area_m2(&surface.vertices),
                azimuth_deg: surface_azimuth_deg(&surface.vertices),
                tilt_deg: surface_tilt_deg(surface.surface_type, &surface.vertices),
            }
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

fn surface_azimuth_deg(vertices: &[Point3]) -> f64 {
    let Some(normal) = polygon_normal(vertices) else {
        return 0.0;
    };

    let horizontal_magnitude = normal.x.hypot(normal.y);
    if horizontal_magnitude > 1.0e-12 {
        return normalize_degrees(normal.x.atan2(normal.y).to_degrees());
    }

    first_horizontal_edge(vertices)
        .map(|edge| normalize_degrees((-edge.x).atan2(edge.y).to_degrees()))
        .unwrap_or(0.0)
}

fn surface_tilt_deg(surface_type: SurfaceType, vertices: &[Point3]) -> f64 {
    let Some(normal) = polygon_normal(vertices) else {
        return 0.0;
    };
    let magnitude = normal.magnitude();
    if magnitude <= 1.0e-12 {
        return 0.0;
    }
    if (normal.z.abs() / magnitude) > 1.0 - 1.0e-12 {
        return match surface_type {
            SurfaceType::Floor => 180.0,
            SurfaceType::Roof | SurfaceType::Ceiling => 0.0,
            SurfaceType::Wall => 90.0,
        };
    }

    (-normal.z / magnitude).clamp(-1.0, 1.0).acos().to_degrees()
}

fn polygon_normal(vertices: &[Point3]) -> Option<Vector3> {
    if vertices.len() < 3 {
        return None;
    }

    let origin = vertices[0];
    let mut normal = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    for window in vertices[1..].windows(2) {
        let first = vector_between(origin, window[0]);
        let second = vector_between(origin, window[1]);
        let triangle_normal = cross(first, second);
        normal.x += triangle_normal.x;
        normal.y += triangle_normal.y;
        normal.z += triangle_normal.z;
    }

    if normal.magnitude() > 1.0e-12 {
        Some(normal)
    } else {
        None
    }
}

fn first_horizontal_edge(vertices: &[Point3]) -> Option<Vector3> {
    vertices
        .windows(2)
        .map(|window| vector_between(window[0], window[1]))
        .chain(
            vertices
                .first()
                .zip(vertices.last())
                .map(|(first, last)| vector_between(*last, *first)),
        )
        .find(|edge| edge.x.hypot(edge.y) > 1.0e-12)
}

fn normalize_degrees(value: f64) -> f64 {
    value.rem_euclid(360.0)
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

    energyplus_analytical_zone_air_temperature_c(
        current_temperature_c,
        internal_gain_w + conductance_w_per_k * outdoor_temperature_c,
        conductance_w_per_k,
        heat_capacity_j_per_k,
        timestep_seconds,
    )
}

/// Builds EnergyPlus zone-air temperature coefficients for an uncontrolled zone.
///
/// This mirrors the coefficient assembly in `correctZoneAirTemps` for the
/// current diagnostic subset:
/// `TempDepCoef = SumHA + SumMCp` and
/// `TempIndCoef = SumIntGain + SumHATsurf - SumHATref + SumMCpT`.
#[must_use]
pub fn energyplus_zone_air_temperature_coefficients(
    sum_ha_w_per_k: f64,
    sum_hat_surf_w: f64,
    sum_hat_ref_w: f64,
    sum_internal_gain_w: f64,
    sum_mcp_w_per_k: f64,
    sum_mcp_t_w: f64,
    air_heat_capacity_j_per_k: f64,
    timestep_seconds: f64,
    previous_mean_air_temperatures_c: [f64; 3],
) -> ZoneAirTemperatureCoefficients {
    let temp_dependent_coefficient_w_per_k = sum_ha_w_per_k + sum_mcp_w_per_k;
    let temp_independent_coefficient_w =
        sum_internal_gain_w + sum_hat_surf_w - sum_hat_ref_w + sum_mcp_t_w;
    energyplus_zone_air_temperature_coefficients_from_terms(
        temp_dependent_coefficient_w_per_k,
        temp_independent_coefficient_w,
        air_heat_capacity_j_per_k,
        timestep_seconds,
        previous_mean_air_temperatures_c,
    )
}

fn energyplus_zone_air_temperature_coefficients_from_terms(
    temp_dependent_coefficient_w_per_k: f64,
    temp_independent_coefficient_w: f64,
    air_heat_capacity_j_per_k: f64,
    timestep_seconds: f64,
    previous_mean_air_temperatures_c: [f64; 3],
) -> ZoneAirTemperatureCoefficients {
    let air_power_cap_w_per_k = if air_heat_capacity_j_per_k > 0.0 && timestep_seconds > 0.0 {
        air_heat_capacity_j_per_k / timestep_seconds
    } else {
        0.0
    };
    let third_order_history_term_w = air_power_cap_w_per_k
        * (3.0 * previous_mean_air_temperatures_c[0]
            - (3.0 / 2.0) * previous_mean_air_temperatures_c[1]
            + (1.0 / 3.0) * previous_mean_air_temperatures_c[2]);

    ZoneAirTemperatureCoefficients {
        temp_dependent_coefficient_w_per_k,
        temp_independent_coefficient_w,
        air_power_cap_w_per_k,
        third_order_history_term_w,
        third_order_temp_dependent_load_w_per_k: (11.0 / 6.0) * air_power_cap_w_per_k
            + temp_dependent_coefficient_w_per_k,
        third_order_temp_independent_load_w: third_order_history_term_w
            + temp_independent_coefficient_w,
    }
}

/// EnergyPlus third-order zone-air temperature solution for one timestep.
///
/// This mirrors the `ThirdOrder` branch in `correctZoneAirTemps`:
/// `ZT = (TempIndCoef + TempHistoryTerm) /
///       ((11/6) * AirPowerCap + TempDepCoef)`.
#[must_use]
pub fn energyplus_third_order_zone_air_temperature_c(
    previous_temperature_c: f64,
    temp_independent_coefficient_w: f64,
    temp_dependent_coefficient_w_per_k: f64,
    air_heat_capacity_j_per_k: f64,
    timestep_seconds: f64,
    previous_mean_air_temperatures_c: [f64; 3],
) -> f64 {
    let coefficients = energyplus_zone_air_temperature_coefficients_from_terms(
        temp_dependent_coefficient_w_per_k,
        temp_independent_coefficient_w,
        air_heat_capacity_j_per_k,
        timestep_seconds,
        previous_mean_air_temperatures_c,
    );
    energyplus_third_order_zone_air_temperature_from_coefficients(
        previous_temperature_c,
        coefficients,
    )
}

fn energyplus_third_order_zone_air_temperature_from_coefficients(
    previous_temperature_c: f64,
    coefficients: ZoneAirTemperatureCoefficients,
) -> f64 {
    let denominator = coefficients.third_order_temp_dependent_load_w_per_k;
    if denominator.abs() <= f64::EPSILON {
        previous_temperature_c
    } else {
        coefficients.third_order_temp_independent_load_w / denominator
    }
}

/// EnergyPlus analytical zone-air temperature solution for one timestep.
///
/// This mirrors the `AnalyticalSolution` branch in
/// `ZoneTempPredictorCorrector.cc`, using `TempIndCoef`, `TempDepCoef`, and
/// `AirPowerCap = C_air / dt`.
#[must_use]
pub fn energyplus_analytical_zone_air_temperature_c(
    previous_temperature_c: f64,
    temp_independent_coefficient_w: f64,
    temp_dependent_coefficient_w_per_k: f64,
    air_heat_capacity_j_per_k: f64,
    timestep_seconds: f64,
) -> f64 {
    if air_heat_capacity_j_per_k <= 0.0 || timestep_seconds <= 0.0 {
        return previous_temperature_c;
    }

    let air_power_cap_w_per_k = air_heat_capacity_j_per_k / timestep_seconds;
    if temp_dependent_coefficient_w_per_k.abs() <= f64::EPSILON {
        return previous_temperature_c + temp_independent_coefficient_w / air_power_cap_w_per_k;
    }

    let equilibrium_temperature_c =
        temp_independent_coefficient_w / temp_dependent_coefficient_w_per_k;
    let exponent = (-temp_dependent_coefficient_w_per_k / air_power_cap_w_per_k).min(700.0);
    (previous_temperature_c - equilibrium_temperature_c) * exponent.exp()
        + equilibrium_temperature_c
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
    /// Liquid precipitation depth in mm for the hour when present.
    pub liquid_precipitation_depth_mm: f64,
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
            liquid_precipitation_depth_mm: parse_epw_liquid_precipitation_depth_mm(&fields, 33),
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

fn parse_epw_optional_f64_default(fields: &[&str], index: usize, default: f64) -> f64 {
    let Some(value) = fields.get(index).map(|value| value.trim()) else {
        return default;
    };
    if value.is_empty() {
        default
    } else {
        value.parse::<f64>().unwrap_or(default)
    }
}

fn parse_epw_liquid_precipitation_depth_mm(fields: &[&str], index: usize) -> f64 {
    let value = parse_epw_optional_f64_default(fields, index, 0.0);
    if value >= 99.0 { 0.0 } else { value.max(0.0) }
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
        ConstructionCtfCoefficientOverride, CtfInsideFaceBalanceInput, CtfOutsideFaceBalanceInput,
        CtfOutsideQuickConductionBalanceInput, Date, ENERGYPLUS_ZONE_INITIAL_TEMP_C, EpwRecord,
        ExecutionStep, FirstZoneSimulationOptions, HeatBalanceCtfInitialHistoryPolicy,
        HeatBalanceSimulationOptions, HeatBalanceStepInput, HeatBalanceWarmupOptions,
        HeatBalanceWarmupSummary, HeatBalanceZoneAirAlgorithm,
        NODE_STATE_EXCLUDED_SETPOINT_VARIABLE, NODE_STATE_SOURCE_MAP_PATH,
        NODE_TEMPERATURE_SETPOINT_SENTINEL_C, NodeStateProjectionOptions, NodeStateRole,
        OutputSeries, PLANT_STATE_SOURCE_MAP_PATH, PlantEquipmentRole, PlantStateProjectionOptions,
        ResultStore, RuntimeError, RuntimeOutputRegistry, SECONDS_PER_HOUR, SimulationMode,
        SimulationState, advance_heat_balance_state_one_timestep, advance_surface_ctf_histories,
        append_surface_incident_solar_radiation_series, build_execution_plan,
        build_hourly_time_axis, build_hourly_time_axis_for_run_period,
        energyplus_analytical_zone_air_temperature_c,
        energyplus_ashrae_tarp_natural_convection_w_per_m2_k,
        energyplus_average_solar_coefficients, energyplus_ctf_inside_face_temperature_c,
        energyplus_ctf_outside_face_temperature_c,
        energyplus_ctf_outside_face_temperature_quick_conduction_c,
        energyplus_daily_solar_coefficients,
        energyplus_doe2_outside_convection_coefficient_w_per_m2_k,
        energyplus_shadowing_period_solar_coefficients,
        energyplus_tarp_inside_convection_coefficient_w_per_m2_k,
        energyplus_third_order_zone_air_temperature_c,
        energyplus_zone_air_temperature_coefficients, initialize_heat_balance_state,
        initialize_heat_balance_state_with_ctf_coefficients, next_day,
        node_temperature_setpoint_from_energyplus, parse_epw_dry_bulb_series, parse_epw_records,
        run_heat_balance_run_period_warmup, seed_energyplus_initial_surface_ctf_histories,
        seed_initial_surface_ctf_boundary_histories, simulate_constant_schedules,
        simulate_first_zone_uncontrolled, simulate_heat_balance_zone_air_temperatures,
        simulate_heat_balance_zone_air_temperatures_with_weather_records,
        simulate_ideal_loads_node_state_projection, simulate_plant_state_projection,
        simulate_schedule_values, simulate_zone_internal_convective_gains,
        solar_position_rad_at_local_hour, solar_weather_interpolation_weights, surface_area_m2,
        surface_geometry_summaries, surface_inside_conduction_flux_w_per_m2,
        surface_inside_ctf_source_terms_w_per_m2, surface_outside_conduction_flux_w_per_m2,
        update_surface_ctf_history_constants, update_surface_inside_longwave_exchange_probe,
        zone_air_heat_balance_air_storage_rate_w, zone_geometry_summaries,
    };
    use crate::{RuntimeDiagnosticCode, RuntimeMeterRequest, RuntimeOutputRequest};
    use ep_model::{
        AutoOrNumber, AutosizeOrNumber, BranchId, BranchListId, Construction, ConstructionId,
        DehumidificationControlType, DemandControlledVentilationType, HeatRecoveryType,
        HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
        IdealLoadsLimit, InternalGainId, LoadDistributionScheme, LoopId, Material, MaterialId,
        MaterialKind, MaterialSurfaceRoughness, Node, NodeId, NodeList, NodeListId, NormalizedName,
        OtherEquipment, OutdoorAirEconomizerType, OutputHandle, OutsideBoundaryCondition,
        PlantBranch, PlantBranchComponent, PlantBranchList, PlantLoop, Point3, RunPeriod,
        RunPeriodId, ScheduleCompact, ScheduleCompactSegment, ScheduleConstant, ScheduleId,
        SimulationModel, SiteLocation, SunExposure, Surface, SurfaceId, SurfaceType,
        ThermostatControlObjectType, ThermostatDualSetpoint, ThermostatSetpointId, TimestepConfig,
        TypedModel, WindExposure, Zone, ZoneEquipmentConnection, ZoneEquipmentConnectionId,
        ZoneEquipmentList, ZoneEquipmentListEntry, ZoneEquipmentListId, ZoneEquipmentObjectType,
        ZoneId, ZoneThermostat, ZoneThermostatControl, ZoneThermostatId,
    };

    #[test]
    fn state_defaults_to_first_timestep() {
        let state = SimulationState::new(SimulationMode::Compatibility);

        assert_eq!(state.timestep_index, 0);
        assert_eq!(state.mode, SimulationMode::Compatibility);
        assert!(state.zones.is_empty());
    }

    #[test]
    fn solar_weather_interpolation_matches_energyplus_even_timestep_weights() {
        assert_eq!(solar_weather_interpolation_weights(4, 1), (0.25, 0.75, 0.0));
        assert_eq!(solar_weather_interpolation_weights(4, 2), (0.0, 1.0, 0.0));
        assert_eq!(solar_weather_interpolation_weights(4, 3), (0.0, 0.75, 0.25));
        assert_eq!(solar_weather_interpolation_weights(4, 4), (0.0, 0.5, 0.5));
    }

    #[test]
    fn energyplus_daily_solar_coefficients_match_reference_day() {
        let (sin_declination, _cos_declination, equation_of_time_hours) =
            energyplus_daily_solar_coefficients(1);

        assert!((sin_declination - -0.392204631085).abs() < 1.0e-12);
        assert!((equation_of_time_hours - -0.055895327979).abs() < 1.0e-12);
    }

    #[test]
    fn energyplus_average_solar_coefficients_match_shadowing_period() {
        let (sin_declination, cos_declination, equation_of_time_hours) =
            energyplus_average_solar_coefficients(61, 20);

        assert!((sin_declination - -0.065802703719632).abs() < 1.0e-12);
        assert!((cos_declination - 0.997832653395942).abs() < 1.0e-12);
        assert!((equation_of_time_hours - -0.168373861452452).abs() < 1.0e-12);
    }

    #[test]
    fn shadowing_period_solar_coefficients_use_energyplus_update_frequency() {
        let mut records = Vec::new();
        let mut date = Date {
            year: 2013,
            month: 1,
            day_of_month: 1,
        };
        for _day in 0..80 {
            for hour in 1..=24 {
                records.push(EpwRecord {
                    year: date.year,
                    month: date.month,
                    day: date.day_of_month,
                    hour,
                    minute: 60,
                    dry_bulb_c: 0.0,
                    dew_point_c: 0.0,
                    relative_humidity_percent: 0.0,
                    atmospheric_pressure_pa: 101_325.0,
                    horizontal_infrared_radiation_wh_per_m2: 0.0,
                    global_horizontal_radiation_wh_per_m2: 0.0,
                    direct_normal_radiation_wh_per_m2: 0.0,
                    diffuse_horizontal_radiation_wh_per_m2: 0.0,
                    wind_direction_deg: 0.0,
                    wind_speed_m_per_s: 0.0,
                    liquid_precipitation_depth_mm: 0.0,
                });
            }
            date = next_day(date);
        }

        let coefficients = energyplus_shadowing_period_solar_coefficients(&records, 1450);
        assert!(coefficients.is_some());
        let (sin_declination, cos_declination, equation_of_time_hours) =
            coefficients.unwrap_or((0.0, 0.0, 0.0));

        assert!((sin_declination - -0.065802703719632).abs() < 1.0e-12);
        assert!((cos_declination - 0.997832653395942).abs() < 1.0e-12);
        assert!((equation_of_time_hours - -0.168373861452452).abs() < 1.0e-12);
    }

    #[test]
    fn solar_position_uses_energyplus_hour_angle_convention() {
        let site = SiteLocation {
            name: NormalizedName::new("Chicago"),
            latitude_deg: 41.78,
            longitude_deg: -87.75,
            time_zone_hours: -6.0,
            elevation_m: 190.0,
        };
        let record = EpwRecord {
            year: 2013,
            month: 1,
            day: 1,
            hour: 12,
            minute: 60,
            dry_bulb_c: 0.0,
            dew_point_c: 0.0,
            relative_humidity_percent: 0.0,
            atmospheric_pressure_pa: 101_325.0,
            horizontal_infrared_radiation_wh_per_m2: 0.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 0.0,
            wind_speed_m_per_s: 0.0,
            liquid_precipitation_depth_mm: 0.0,
        };

        let position = solar_position_rad_at_local_hour(&site, &record, 12.0);
        assert!(position.is_some());
        let (altitude_rad, azimuth_rad) = position.unwrap_or((0.0, 0.0));

        assert!((altitude_rad.to_degrees() - 25.115079268192).abs() < 1.0e-12);
        assert!((azimuth_rad.to_degrees() - 181.434056277464).abs() < 1.0e-12);
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
        assert_eq!(plan.step_count(), 9);
        assert_eq!(plan.stages[0].steps[0], ExecutionStep::UpdateWeather);
        assert_eq!(
            plan.stages[0].steps[1],
            ExecutionStep::EvaluateSchedule(ScheduleId(0))
        );
        assert_eq!(plan.stages[1].steps[0], ExecutionStep::SolveZone(ZoneId(0)));
        assert_eq!(plan.stages[2].steps.len(), 6);
        assert_eq!(
            plan.stages[2].steps[0],
            ExecutionStep::WriteOutput(OutputHandle(0))
        );
        assert_eq!(
            plan.stages[2].steps[1],
            ExecutionStep::WriteOutput(OutputHandle(1))
        );
        assert_eq!(
            plan.stages[2].steps[2],
            ExecutionStep::WriteOutput(OutputHandle(2))
        );
        assert_eq!(
            plan.stages[2].steps[5],
            ExecutionStep::WriteOutput(OutputHandle(5))
        );
    }

    #[test]
    fn execution_plan_includes_thermostat_and_ideal_loads_steps() {
        let mut typed = TypedModel::default();
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("Control Type"),
            schedule_type_limits: None,
            hourly_value: 4.0,
        });
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(1),
            name: NormalizedName::new("Heating Setpoint"),
            schedule_type_limits: None,
            hourly_value: 21.0,
        });
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(2),
            name: NormalizedName::new("Cooling Setpoint"),
            schedule_type_limits: None,
            hourly_value: 24.0,
        });
        typed.zones.push(Zone {
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
        typed
            .thermostat_dual_setpoints
            .push(ThermostatDualSetpoint {
                id: ThermostatSetpointId(0),
                name: NormalizedName::new("Dual Setpoints"),
                heating_setpoint_schedule: ScheduleId(1),
                cooling_setpoint_schedule: ScheduleId(2),
            });
        typed.zone_thermostats.push(ZoneThermostat {
            id: ZoneThermostatId(0),
            name: NormalizedName::new("Zone Thermostat"),
            zone: ZoneId(0),
            control_type_schedule: ScheduleId(0),
            controls: vec![ZoneThermostatControl {
                object_type: ThermostatControlObjectType::DualSetpoint,
                dual_setpoint: ThermostatSetpointId(0),
            }],
            temperature_difference_between_cutout_and_setpoint_delta_c: 0.0,
        });
        typed.ideal_loads_air_systems.push(IdealLoadsAirSystem {
            id: IdealLoadsAirSystemId(0),
            name: NormalizedName::new("Zone Ideal Loads"),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new("Zone Inlet"),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: None,
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        });
        typed.zone_equipment_lists.push(ZoneEquipmentList {
            id: ZoneEquipmentListId(0),
            name: NormalizedName::new("Zone Equipment"),
            load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
            equipment: vec![ZoneEquipmentListEntry {
                object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
                ideal_loads_air_system: IdealLoadsAirSystemId(0),
                cooling_sequence: 1,
                heating_or_no_load_sequence: 1,
                sequential_cooling_fraction_schedule: None,
                sequential_heating_fraction_schedule: None,
            }],
        });
        typed
            .zone_equipment_connections
            .push(ZoneEquipmentConnection {
                id: ZoneEquipmentConnectionId(0),
                zone: ZoneId(0),
                equipment_list: ZoneEquipmentListId(0),
                zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new("Zone Inlet")),
                zone_air_exhaust_node_or_nodelist_name: None,
                zone_air_node_name: NormalizedName::new("Zone Air Node"),
                zone_return_air_node_or_nodelist_name: Some(NormalizedName::new("Zone Return")),
                zone_return_air_node_1_flow_rate_fraction_schedule: None,
                zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
            });
        let model = SimulationModel::from_typed(typed);

        let plan = build_execution_plan(&model);

        assert_eq!(model.graph.zone_thermostats.len(), 1);
        assert_eq!(model.graph.zone_ideal_loads.len(), 1);
        assert_eq!(plan.stages[1].steps.len(), 3);
        assert_eq!(
            plan.stages[1].steps[0],
            ExecutionStep::EvaluateZoneThermostat(ZoneThermostatId(0))
        );
        assert_eq!(plan.stages[1].steps[1], ExecutionStep::SolveZone(ZoneId(0)));
        assert_eq!(
            plan.stages[1].steps[2],
            ExecutionStep::EvaluateIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );
    }

    #[test]
    fn ideal_loads_node_state_projection_expands_nodelist_and_writes_series()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = ideal_loads_node_state_model();

        let projection = simulate_ideal_loads_node_state_projection(
            &model,
            NodeStateProjectionOptions::hourly_samples(4),
        )?;

        assert_eq!(projection.summary.samples, 4);
        assert_eq!(projection.summary.node_count, 3);
        assert_eq!(projection.summary.series_count, 9);
        assert_eq!(projection.summary.state_node_count, 3);
        assert_eq!(
            projection.summary.evidence_policy.source_map_path,
            NODE_STATE_SOURCE_MAP_PATH
        );
        assert_eq!(
            projection.summary.evidence_policy.excluded_variable,
            NODE_STATE_EXCLUDED_SETPOINT_VARIABLE
        );
        assert_eq!(
            node_temperature_setpoint_from_energyplus(NODE_TEMPERATURE_SETPOINT_SENTINEL_C),
            None
        );
        assert_eq!(node_temperature_setpoint_from_energyplus(21.0), Some(21.0));
        assert_eq!(projection.state.len(), 3);
        assert_eq!(
            projection
                .summary
                .nodes
                .iter()
                .map(|node| (node.node_name.as_str(), node.role))
                .collect::<Vec<_>>(),
            vec![
                ("ZONE ONE INLET", NodeStateRole::Supply),
                ("ZONE ONE AIR NODE", NodeStateRole::ZoneAir),
                ("ZONE ONE RETURN", NodeStateRole::ReturnAir),
            ]
        );

        let inlet_temperature = projection
            .results
            .find_series("ZONE ONE INLET", "System Node Temperature")
            .ok_or_else(|| std::io::Error::other("missing inlet temperature series"))?;
        assert_eq!(inlet_temperature.values, vec![50.0; 4]);

        let inlet_humidity = projection
            .results
            .find_series("ZONE ONE INLET", "System Node Humidity Ratio")
            .ok_or_else(|| std::io::Error::other("missing inlet humidity series"))?;
        assert_eq!(inlet_humidity.values, vec![0.0156; 4]);

        let inlet_mass_flow = projection
            .results
            .find_series("ZONE ONE INLET", "System Node Mass Flow Rate")
            .ok_or_else(|| std::io::Error::other("missing inlet mass flow series"))?;
        assert!(
            inlet_mass_flow
                .values
                .iter()
                .all(|value| (*value - 0.3).abs() < 1.0e-12)
        );
        let inlet_state = projection
            .state
            .find_by_key("ZONE ONE INLET")
            .ok_or_else(|| std::io::Error::other("missing inlet node state"))?;
        assert!((inlet_state.mass_flow_rate_kg_per_s - 0.3).abs() < 1.0e-12);
        assert!((inlet_state.temperature_c - 50.0).abs() < 1.0e-12);
        assert_eq!(inlet_state.temperature_setpoint_c, None);

        let zone_air_temperature = projection
            .results
            .find_series("ZONE ONE AIR NODE", "System Node Temperature")
            .ok_or_else(|| std::io::Error::other("missing zone air temperature series"))?;
        assert_eq!(zone_air_temperature.values, vec![23.0; 4]);
        let zone_air_state = projection
            .state
            .find_by_key("ZONE ONE AIR NODE")
            .ok_or_else(|| std::io::Error::other("missing zone air node state"))?;
        assert!((zone_air_state.humidity_ratio - 0.008).abs() < 1.0e-12);

        let return_mass_flow = projection
            .results
            .find_series("ZONE ONE RETURN", "System Node Mass Flow Rate")
            .ok_or_else(|| std::io::Error::other("missing return mass flow series"))?;
        assert!(
            return_mass_flow
                .values
                .iter()
                .all(|value| (*value - 0.3).abs() < 1.0e-12)
        );

        Ok(())
    }

    #[test]
    fn plant_state_projection_writes_diagnostic_loop_and_equipment_series()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = plant_state_projection_model();

        let projection = simulate_plant_state_projection(
            &model,
            PlantStateProjectionOptions::hourly_samples(48),
        )?;

        assert_eq!(projection.summary.samples, 48);
        assert_eq!(projection.summary.loop_count, 1);
        assert_eq!(projection.summary.equipment_count, 3);
        assert_eq!(projection.summary.series_count, 8);
        assert_eq!(projection.results.sample_count(), 48);
        assert_eq!(projection.results.series.len(), 8);
        assert_eq!(
            projection.summary.evidence_policy.source_map_path,
            PLANT_STATE_SOURCE_MAP_PATH
        );
        assert_eq!(projection.summary.loops[0].loop_name, "MAIN LOOP");
        assert_eq!(
            projection.summary.loops[0].supply_inlet_node_name,
            "SUPPLY INLET NODE"
        );
        assert_eq!(
            projection.summary.loops[0].supply_outlet_node_name,
            "SUPPLY OUTLET NODE"
        );

        let roles: Vec<_> = projection
            .summary
            .equipment
            .iter()
            .map(|equipment| (equipment.equipment_name.as_str(), equipment.role))
            .collect();
        assert_eq!(
            roles,
            vec![
                ("PUMP", PlantEquipmentRole::Pump),
                ("PURCHASED HEATING", PlantEquipmentRole::PurchasedHeating),
                ("LOAD PROFILE 1", PlantEquipmentRole::LoadProfile),
            ]
        );

        for (key, variable) in [
            ("MAIN LOOP", "Plant Supply Side Cooling Demand Rate"),
            ("MAIN LOOP", "Plant Supply Side Heating Demand Rate"),
            ("MAIN LOOP", "Plant Supply Side Inlet Mass Flow Rate"),
            ("MAIN LOOP", "Plant Supply Side Inlet Temperature"),
            ("MAIN LOOP", "Plant Supply Side Outlet Temperature"),
            ("PUMP", "Pump Electricity Rate"),
            ("PURCHASED HEATING", "District Heating Water Rate"),
            ("LOAD PROFILE 1", "Plant Load Profile Heat Transfer Rate"),
        ] {
            let Some(series) = projection.results.find_series(key, variable) else {
                return Err(std::io::Error::other(format!("missing {key} / {variable}")).into());
            };
            assert_eq!(series.values.len(), 48);
            assert!(series.values.iter().all(|value| value.abs() > 1.0e-9));
        }

        Ok(())
    }

    fn ideal_loads_node_state_model() -> SimulationModel {
        let mut typed = TypedModel::default();
        typed.zones.push(Zone {
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
        typed.nodes.push(Node {
            id: NodeId(0),
            name: NormalizedName::new("Zone One Inlet"),
        });
        typed.nodes.push(Node {
            id: NodeId(1),
            name: NormalizedName::new("Zone One Air Node"),
        });
        typed.nodes.push(Node {
            id: NodeId(2),
            name: NormalizedName::new("Zone One Return"),
        });
        typed.node_names.insert("Zone One Inlet", NodeId(0));
        typed.node_names.insert("Zone One Air Node", NodeId(1));
        typed.node_names.insert("Zone One Return", NodeId(2));
        typed.node_lists.push(NodeList {
            id: NodeListId(0),
            name: NormalizedName::new("Zone One Inlets"),
            nodes: vec![NodeId(0)],
        });
        typed
            .node_list_names
            .insert("Zone One Inlets", NodeListId(0));
        typed.ideal_loads_air_systems.push(IdealLoadsAirSystem {
            id: IdealLoadsAirSystemId(0),
            name: NormalizedName::new("Zone One Ideal Loads"),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new("Zone One Inlets"),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: Some(AutosizeOrNumber::Value(0.25)),
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        });
        typed.zone_equipment_lists.push(ZoneEquipmentList {
            id: ZoneEquipmentListId(0),
            name: NormalizedName::new("Zone One Equipment"),
            load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
            equipment: vec![ZoneEquipmentListEntry {
                object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
                ideal_loads_air_system: IdealLoadsAirSystemId(0),
                cooling_sequence: 1,
                heating_or_no_load_sequence: 1,
                sequential_cooling_fraction_schedule: None,
                sequential_heating_fraction_schedule: None,
            }],
        });
        typed
            .zone_equipment_connections
            .push(ZoneEquipmentConnection {
                id: ZoneEquipmentConnectionId(0),
                zone: ZoneId(0),
                equipment_list: ZoneEquipmentListId(0),
                zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new("Zone One Inlets")),
                zone_air_exhaust_node_or_nodelist_name: None,
                zone_air_node_name: NormalizedName::new("Zone One Air Node"),
                zone_return_air_node_or_nodelist_name: Some(NormalizedName::new("Zone One Return")),
                zone_return_air_node_1_flow_rate_fraction_schedule: None,
                zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
            });

        SimulationModel::from_typed(typed)
    }

    fn plant_state_projection_model() -> SimulationModel {
        let mut typed = TypedModel::default();
        let supply_inlet = push_node(&mut typed, "Supply Inlet Node");
        let pump_outlet = push_node(&mut typed, "Supply Pump-Heating Node");
        let supply_outlet = push_node(&mut typed, "Supply Outlet Node");
        let demand_inlet = push_node(&mut typed, "Demand Inlet Node");
        let load_profile_inlet = push_node(&mut typed, "Demand Load Profile 1 Inlet Node");
        let load_profile_outlet = push_node(&mut typed, "Demand Load Profile 1 Outlet Node");
        let demand_outlet = push_node(&mut typed, "Demand Outlet Node");

        typed.plant_branches.extend([
            PlantBranch {
                id: BranchId(0),
                name: NormalizedName::new("Supply Inlet Branch"),
                components: vec![PlantBranchComponent {
                    object_type: NormalizedName::new("Pump:VariableSpeed"),
                    name: NormalizedName::new("Pump"),
                    inlet_node: supply_inlet,
                    outlet_node: pump_outlet,
                }],
            },
            PlantBranch {
                id: BranchId(1),
                name: NormalizedName::new("Heating Branch"),
                components: vec![PlantBranchComponent {
                    object_type: NormalizedName::new("DistrictHeating:Water"),
                    name: NormalizedName::new("Purchased Heating"),
                    inlet_node: pump_outlet,
                    outlet_node: supply_outlet,
                }],
            },
            PlantBranch {
                id: BranchId(2),
                name: NormalizedName::new("Load Profile Branch 1"),
                components: vec![PlantBranchComponent {
                    object_type: NormalizedName::new("LoadProfile:Plant"),
                    name: NormalizedName::new("Load Profile 1"),
                    inlet_node: load_profile_inlet,
                    outlet_node: load_profile_outlet,
                }],
            },
        ]);
        typed.plant_branch_lists.extend([
            PlantBranchList {
                id: BranchListId(0),
                name: NormalizedName::new("Supply Branches"),
                branches: vec![BranchId(0), BranchId(1)],
            },
            PlantBranchList {
                id: BranchListId(1),
                name: NormalizedName::new("Demand Branches"),
                branches: vec![BranchId(2)],
            },
        ]);
        typed.plant_loops.push(PlantLoop {
            id: LoopId(0),
            name: NormalizedName::new("Main Loop"),
            fluid_type: NormalizedName::new("Water"),
            plant_side_inlet_node: supply_inlet,
            plant_side_outlet_node: supply_outlet,
            plant_side_branch_list: BranchListId(0),
            plant_side_connector_list: None,
            demand_side_inlet_node: demand_inlet,
            demand_side_outlet_node: demand_outlet,
            demand_side_branch_list: BranchListId(1),
            demand_side_connector_list: None,
            load_distribution_scheme: Some(NormalizedName::new("SequentialLoad")),
        });

        SimulationModel::from_typed(typed)
    }

    fn push_node(model: &mut TypedModel, name: &str) -> NodeId {
        let id = NodeId(model.nodes.len() as u32);
        model.nodes.push(Node {
            id,
            name: NormalizedName::new(name),
        });
        model.node_names.insert(name, id);
        id
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
1999,1,1,2,0,Source,-2.0,-3.0,51,82100,0,0,301,11,21,31,0,0,0,0,190,2.6,0,0,0,0,0,0,0,0,0,0,0,2.0,1.0
"#,
        )?;

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].dry_bulb_c, -3.0);
        assert_eq!(records[0].dew_point_c, -4.0);
        assert_eq!(records[0].relative_humidity_percent, 50.0);
        assert_eq!(records[0].atmospheric_pressure_pa, 82_000.0);
        assert_eq!(records[0].wind_direction_deg, 180.0);
        assert_eq!(records[0].wind_speed_m_per_s, 2.5);
        assert_eq!(records[0].liquid_precipitation_depth_mm, 0.0);
        assert_eq!(records[1].liquid_precipitation_depth_mm, 2.0);

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
    fn surface_geometry_summary_reports_cube_orientation() -> Result<(), Box<dyn std::error::Error>>
    {
        let summaries = surface_geometry_summaries(&cube_model());

        assert_eq!(summaries.len(), 6);
        let floor = summaries
            .iter()
            .find(|surface| surface.surface_name == "FLOOR")
            .ok_or_else(|| std::io::Error::other("missing floor surface"))?;
        assert_eq!(floor.zone_name, "ZONE ONE");
        assert_eq!(floor.surface_type, SurfaceType::Floor);
        assert_eq!(floor.area_m2, 1.0);
        assert!((floor.azimuth_deg - 270.0).abs() < 1.0e-9);
        assert!((floor.tilt_deg - 180.0).abs() < 1.0e-9);

        let roof = summaries
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof surface"))?;
        assert_eq!(roof.surface_type, SurfaceType::Roof);
        assert_eq!(roof.area_m2, 1.0);
        assert!((roof.azimuth_deg - 0.0).abs() < 1.0e-9);
        assert!((roof.tilt_deg - 0.0).abs() < 1.0e-9);

        let wall_azimuths = [
            ("WALL X0", 90.0),
            ("WALL X1", 270.0),
            ("WALL Y0", 0.0),
            ("WALL Y1", 180.0),
        ];
        for (surface_name, azimuth_deg) in wall_azimuths {
            let wall = summaries
                .iter()
                .find(|surface| surface.surface_name == surface_name)
                .ok_or_else(|| std::io::Error::other(format!("missing {surface_name} surface")))?;
            assert_eq!(wall.surface_type, SurfaceType::Wall);
            assert_eq!(wall.area_m2, 1.0);
            assert!((wall.azimuth_deg - azimuth_deg).abs() < 1.0e-9);
            assert!((wall.tilt_deg - 90.0).abs() < 1.0e-9);
        }

        Ok(())
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
        assert_eq!(state.zones[0].opaque_surface_conductance_w_per_k, 6.0);
        assert_eq!(state.zones[0].opaque_surface_heat_gain_w, 0.0);
        assert!((state.zones[0].sum_ha_w_per_k - 18.456).abs() < 1.0e-12);
        assert!((state.zones[0].sum_hat_surf_w - 369.12).abs() < 1.0e-12);
        assert_eq!(state.zones[0].sum_hat_ref_w, 0.0);
        assert!(
            (state.zones[0]
                .zone_air_temperature_coefficients
                .temp_dependent_coefficient_w_per_k
                - 18.456)
                .abs()
                < 1.0e-12
        );
        assert!(
            (state.zones[0]
                .zone_air_temperature_coefficients
                .temp_independent_coefficient_w
                - 381.12)
                .abs()
                < 1.0e-12
        );
        assert_eq!(
            state.zones[0]
                .zone_air_temperature_coefficients
                .air_power_cap_w_per_k,
            0.0
        );
        assert_eq!(state.surfaces.len(), 6);
        let floor = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "FLOOR")
            .ok_or_else(|| std::io::Error::other("missing floor surface"))?;
        assert!((floor.tilt_deg - 180.0).abs() < 1.0e-9);
        let roof = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof surface"))?;
        assert!((roof.tilt_deg - 0.0).abs() < 1.0e-9);
        let wall = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "WALL Y0")
            .ok_or_else(|| std::io::Error::other("missing wall surface"))?;
        assert!((wall.tilt_deg - 90.0).abs() < 1.0e-9);
        assert_eq!(
            state.surfaces[0].outside_boundary_condition,
            OutsideBoundaryCondition::Outdoors
        );
        assert_eq!(state.surfaces[0].construction_name, "WALL");
        assert_eq!(state.surfaces[0].outside_layer_material_name, "R1");
        assert_eq!(
            state.surfaces[0].outside_layer_roughness,
            MaterialSurfaceRoughness::Rough
        );
        assert_eq!(state.surfaces[0].area_m2, 1.0);
        assert_eq!(state.surfaces[0].thermal_resistance_m2_k_per_w, 1.0);
        assert_eq!(state.surfaces[0].heat_capacity_j_per_m2_k, None);
        assert_eq!(state.surfaces[0].conductance_w_per_k, 1.0);
        assert_eq!(
            state.surfaces[0].inside_convection_coefficient_w_per_m2_k,
            3.076
        );
        assert_eq!(state.surfaces[0].ctf.outside_0_w_per_m2_k, 1.0);
        assert_eq!(state.surfaces[0].ctf.cross_0_w_per_m2_k, 1.0);
        assert_eq!(state.surfaces[0].ctf.inside_0_w_per_m2_k, 1.0);
        assert_eq!(state.surfaces[0].ctf.const_in_part_w_per_m2, 0.0);
        assert_eq!(state.surfaces[0].ctf.const_out_part_w_per_m2, 0.0);
        assert_eq!(
            state.surfaces[0].ctf.outside_temperature_history_c,
            vec![20.0]
        );
        assert_eq!(state.surfaces[0].heat_gain_to_zone_w, 0.0);
        assert_eq!(state.surfaces[0].inside_face_temperature_c, 20.0);
        assert_eq!(state.surfaces[0].outside_face_temperature_c, 20.0);

        Ok(())
    }

    #[test]
    fn energyplus_zone_air_temperature_coefficients_match_predictor_terms() {
        let coefficients = energyplus_zone_air_temperature_coefficients(
            18.456,
            369.12,
            2.0,
            12.0,
            3.0,
            45.0,
            1207.2,
            600.0,
            [20.0, 19.0, 18.0],
        );

        assert!((coefficients.temp_dependent_coefficient_w_per_k - 21.456).abs() < 1.0e-12);
        assert!((coefficients.temp_independent_coefficient_w - 424.12).abs() < 1.0e-12);
        assert!((coefficients.air_power_cap_w_per_k - 2.012).abs() < 1.0e-12);
        let expected_history = 2.012 * (3.0 * 20.0 - 1.5 * 19.0 + (1.0 / 3.0) * 18.0);
        assert!((coefficients.third_order_history_term_w - expected_history).abs() < 1.0e-12);
        assert!(
            (coefficients.third_order_temp_dependent_load_w_per_k
                - ((11.0 / 6.0) * 2.012 + 21.456))
                .abs()
                < 1.0e-12
        );
        assert!(
            (coefficients.third_order_temp_independent_load_w - (expected_history + 424.12)).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn energyplus_third_order_zone_air_temperature_matches_predictor_branch() {
        let temperature = energyplus_third_order_zone_air_temperature_c(
            20.0,
            424.12,
            21.456,
            1207.2,
            600.0,
            [20.0, 19.0, 18.0],
        );
        let air_power_cap = 1207.2 / 600.0;
        let history_term = air_power_cap * (3.0 * 20.0 - 1.5 * 19.0 + (1.0 / 3.0) * 18.0);
        let expected = (424.12 + history_term) / ((11.0 / 6.0) * air_power_cap + 21.456);
        assert!((temperature - expected).abs() < 1.0e-12);

        let fallback =
            energyplus_third_order_zone_air_temperature_c(20.0, 1.0, 0.0, 0.0, 600.0, [20.0; 3]);
        assert_eq!(fallback, 20.0);
    }

    #[test]
    fn energyplus_analytical_zone_air_temperature_matches_predictor_branch() {
        let zero_dependency =
            energyplus_analytical_zone_air_temperature_c(20.0, 12.0, 0.0, 1207.2, 600.0);
        assert!((zero_dependency - (20.0 + 12.0 * 600.0 / 1207.2)).abs() < 1.0e-12);

        let temperature =
            energyplus_analytical_zone_air_temperature_c(20.0, 72.0, 6.0, 1207.2, 600.0);
        let expected = 12.0 + (20.0 - 12.0) * (-6.0 * 600.0 / 1207.2_f64).exp();
        assert!((temperature - expected).abs() < 1.0e-12);
    }

    #[test]
    fn energyplus_tarp_natural_convection_matches_ashrae_branches() {
        let vertical = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(28.0, 20.0, 0.0);
        assert!((vertical - 2.62).abs() < 1.0e-12);

        let unstable_delta = 2.0_f64.powf(1.0 / 3.0);
        let unstable = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(22.0, 20.0, 1.0);
        let expected_unstable = 9.482 * unstable_delta / (7.238 - 1.0);
        assert!((unstable - expected_unstable).abs() < 1.0e-12);

        let stable = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(22.0, 20.0, -1.0);
        let expected_stable = 1.810 * unstable_delta / (1.382 + 1.0);
        assert!((stable - expected_stable).abs() < 1.0e-12);

        let zero_delta = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(20.0, 20.0, 1.0);
        assert_eq!(zero_delta, 0.0);
    }

    #[test]
    fn energyplus_tarp_inside_convection_uses_surface_orientation_and_limits()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let state = initialize_heat_balance_state(&model, 20.0)?;
        let floor = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "FLOOR")
            .ok_or_else(|| std::io::Error::other("missing floor surface"))?;
        let roof = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof surface"))?;
        let wall = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "WALL Y0")
            .ok_or_else(|| std::io::Error::other("missing wall surface"))?;

        let delta_term = 2.0_f64.powf(1.0 / 3.0);
        let floor_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(floor, 22.0, 20.0);
        let expected_floor = 9.482 * delta_term / (7.238 - 1.0);
        assert!((floor_coefficient - expected_floor).abs() < 1.0e-12);

        let roof_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(roof, 22.0, 20.0);
        let expected_roof = 1.810 * delta_term / (1.382 + 1.0);
        assert!((roof_coefficient - expected_roof).abs() < 1.0e-12);

        let wall_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(wall, 22.0, 20.0);
        let expected_wall = 1.31 * delta_term;
        assert!((wall_coefficient - expected_wall).abs() < 1.0e-12);

        let zero_delta_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(floor, 20.0, 20.0);
        assert_eq!(zero_delta_coefficient, 0.1);

        Ok(())
    }

    #[test]
    fn energyplus_doe2_outside_convection_uses_wind_side_and_roughness() {
        let windward = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            35.0,
            20.0,
            0.0,
            180.0,
            180.0,
            4.0,
            MaterialSurfaceRoughness::MediumRough,
        );
        let leeward = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            35.0,
            20.0,
            0.0,
            180.0,
            0.0,
            4.0,
            MaterialSurfaceRoughness::MediumRough,
        );
        let smoother = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            35.0,
            20.0,
            0.0,
            180.0,
            180.0,
            4.0,
            MaterialSurfaceRoughness::VerySmooth,
        );

        assert!((windward - 16.031846262998357).abs() < 1.0e-12);
        assert!((leeward - 11.929263692153699).abs() < 1.0e-12);
        assert!(windward > leeward);
        assert!(smoother < windward);
    }

    #[test]
    fn surface_ctf_history_terms_update_flux_constants() -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.inside_face_temperature_c = 20.0;
        surface.outside_face_temperature_c = 10.0;
        surface.ctf.cross_history_w_per_m2_k = vec![0.2];
        surface.ctf.inside_history_w_per_m2_k = vec![0.3];
        surface.ctf.outside_history_w_per_m2_k = vec![0.4];
        surface.ctf.flux_history = vec![0.5];
        surface.ctf.outside_temperature_history_c = vec![8.0];
        surface.ctf.inside_temperature_history_c = vec![18.0];
        surface.ctf.inside_flux_history_w_per_m2 = vec![1.2];
        surface.ctf.outside_flux_history_w_per_m2 = vec![-0.4];

        update_surface_ctf_history_constants(surface);

        assert!((surface.ctf.const_in_part_w_per_m2 - (-3.2)).abs() < 1.0e-12);
        assert!((surface.ctf.const_out_part_w_per_m2 - (-0.6)).abs() < 1.0e-12);

        let inside_flux = surface_inside_conduction_flux_w_per_m2(surface);
        let outside_flux = surface_outside_conduction_flux_w_per_m2(surface);
        advance_surface_ctf_histories(surface);

        assert_eq!(surface.ctf.outside_temperature_history_c, vec![10.0]);
        assert_eq!(surface.ctf.inside_temperature_history_c, vec![20.0]);
        assert_eq!(surface.ctf.inside_flux_history_w_per_m2, vec![inside_flux]);
        assert_eq!(
            surface.ctf.outside_flux_history_w_per_m2,
            vec![outside_flux]
        );

        Ok(())
    }

    #[test]
    fn heat_balance_state_applies_construction_ctf_coefficients()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            20.0,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 2,
                    outside_w_per_m2_k: -0.4,
                    cross_w_per_m2_k: 0.2,
                    inside_w_per_m2_k: -0.3,
                    flux: Some(-0.5),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 2.0,
                    cross_w_per_m2_k: 0.5,
                    inside_w_per_m2_k: 3.0,
                    flux: None,
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: 0.4,
                    cross_w_per_m2_k: 0.1,
                    inside_w_per_m2_k: 0.3,
                    flux: Some(0.5),
                },
            ],
        )?;

        let ctf = &state.surfaces[0].ctf;
        assert_eq!(ctf.outside_0_w_per_m2_k, 2.0);
        assert_eq!(ctf.cross_0_w_per_m2_k, 0.5);
        assert_eq!(ctf.inside_0_w_per_m2_k, 3.0);
        assert_eq!(ctf.outside_history_w_per_m2_k, vec![0.4, -0.4]);
        assert_eq!(ctf.cross_history_w_per_m2_k, vec![0.1, 0.2]);
        assert_eq!(ctf.inside_history_w_per_m2_k, vec![0.3, -0.3]);
        assert_eq!(ctf.flux_history, vec![0.5, -0.5]);
        assert_eq!(ctf.outside_temperature_history_c, vec![20.0, 20.0]);
        assert_eq!(ctf.inside_temperature_history_c, vec![20.0, 20.0]);
        assert_eq!(ctf.outside_flux_history_w_per_m2, vec![0.0, 0.0]);
        assert_eq!(ctf.inside_flux_history_w_per_m2, vec![0.0, 0.0]);

        Ok(())
    }

    #[test]
    fn initial_ctf_history_seeding_uses_boundary_temperature_and_u_value()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 2.0,
                    cross_w_per_m2_k: 0.5,
                    inside_w_per_m2_k: 3.0,
                    flux: None,
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: 0.4,
                    cross_w_per_m2_k: 0.1,
                    inside_w_per_m2_k: 0.3,
                    flux: Some(0.5),
                },
            ],
        )?;

        seed_initial_surface_ctf_boundary_histories(&mut state, 5.0);

        let surface = &state.surfaces[0];
        let expected_u_value = 1.0 / surface.thermal_resistance_m2_k_per_w;
        let expected_flux = expected_u_value * (5.0 - ENERGYPLUS_ZONE_INITIAL_TEMP_C);
        assert_eq!(surface.ctf.outside_temperature_history_c, vec![5.0]);
        assert_eq!(
            surface.ctf.inside_temperature_history_c,
            vec![ENERGYPLUS_ZONE_INITIAL_TEMP_C]
        );
        assert!((surface.ctf.outside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);
        assert!((surface.ctf.inside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn energyplus_initial_ctf_history_seeding_uses_surf_initial_temp_and_zero_flux()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 2.0,
                    cross_w_per_m2_k: 0.5,
                    inside_w_per_m2_k: 3.0,
                    flux: None,
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: 0.4,
                    cross_w_per_m2_k: 0.1,
                    inside_w_per_m2_k: 0.3,
                    flux: Some(0.5),
                },
            ],
        )?;
        seed_initial_surface_ctf_boundary_histories(&mut state, 5.0);

        seed_energyplus_initial_surface_ctf_histories(&mut state, ENERGYPLUS_ZONE_INITIAL_TEMP_C);

        let surface = &state.surfaces[0];
        assert_eq!(
            surface.ctf.outside_temperature_history_c,
            vec![ENERGYPLUS_ZONE_INITIAL_TEMP_C]
        );
        assert_eq!(
            surface.ctf.inside_temperature_history_c,
            vec![ENERGYPLUS_ZONE_INITIAL_TEMP_C]
        );
        assert_eq!(surface.ctf.outside_flux_history_w_per_m2, vec![0.0]);
        assert_eq!(surface.ctf.inside_flux_history_w_per_m2, vec![0.0]);
        assert_eq!(
            surface.inside_face_temperature_c,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C
        );
        assert_eq!(
            surface.outside_face_temperature_c,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C
        );

        Ok(())
    }

    #[test]
    fn heat_balance_options_track_initial_ctf_history_policy() {
        let options = HeatBalanceSimulationOptions::hourly_samples(24)
            .with_ctf_initial_history_policy(
                HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial,
            );

        assert_eq!(
            options.ctf_initial_history_policy,
            HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial
        );
    }

    #[test]
    fn energyplus_ctf_inside_face_balance_handles_standard_and_adiabatic()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.outside_face_temperature_c = 10.0;
        surface.inside_face_temperature_c = 19.0;
        surface.ctf.inside_0_w_per_m2_k = 3.0;
        surface.ctf.cross_0_w_per_m2_k = 0.5;
        surface.ctf.const_in_part_w_per_m2 = 1.0;

        let standard = energyplus_ctf_inside_face_temperature_c(
            surface,
            CtfInsideFaceBalanceInput {
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 2.0,
                previous_inside_face_temperature_c: 18.0,
                net_inside_source_w_per_m2: 4.0,
            },
        );
        assert!((standard - 14.0).abs() < 1.0e-12);

        surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        let adiabatic = energyplus_ctf_inside_face_temperature_c(
            surface,
            CtfInsideFaceBalanceInput {
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 2.0,
                previous_inside_face_temperature_c: 18.0,
                net_inside_source_w_per_m2: 4.0,
            },
        );
        assert!((adiabatic - (135.0 / 9.5)).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn surface_inside_ctf_source_terms_follow_energyplus_temp_term_slots()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.outside_face_temperature_c = 10.0;
        surface.inside_face_temperature_c = 19.0;
        surface.ctf.inside_0_w_per_m2_k = 3.0;
        surface.ctf.cross_0_w_per_m2_k = 0.5;
        surface.ctf.const_in_part_w_per_m2 = 1.0;
        surface.inside_radiant_internal_gain_w_per_m2 = 1.0;
        surface.inside_shortwave_absorbed_w_per_m2 = 2.0;
        surface.inside_additional_heat_source_w_per_m2 = 3.0;
        surface.inside_radiant_hvac_w_per_m2 = 4.0;
        surface.inside_net_longwave_w_per_m2 = 5.0;

        let source_terms = surface_inside_ctf_source_terms_w_per_m2(surface);
        assert!((source_terms - 15.0).abs() < 1.0e-12);

        let temperature = energyplus_ctf_inside_face_temperature_c(
            surface,
            CtfInsideFaceBalanceInput {
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 2.0,
                previous_inside_face_temperature_c: 18.0,
                net_inside_source_w_per_m2: source_terms,
            },
        );
        assert!((temperature - 15.1).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn interior_longwave_probe_is_zero_for_equal_surface_temperatures()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        for surface in &mut state.surfaces {
            surface.inside_face_temperature_c = 21.0;
            surface.inside_net_longwave_w_per_m2 = 12.0;
        }

        update_surface_inside_longwave_exchange_probe(&mut state.surfaces, None);

        for surface in &state.surfaces {
            assert!(surface.inside_net_longwave_w_per_m2.abs() < 1.0e-12);
        }

        Ok(())
    }

    #[test]
    fn interior_longwave_probe_conserves_zone_exchange_signs()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        for surface in &mut state.surfaces {
            surface.inside_face_temperature_c = 20.0;
        }
        state.surfaces[0].inside_face_temperature_c = 30.0;

        update_surface_inside_longwave_exchange_probe(&mut state.surfaces, None);

        assert!(state.surfaces[0].inside_net_longwave_w_per_m2 < 0.0);
        for surface in state.surfaces.iter().skip(1) {
            assert!(surface.inside_net_longwave_w_per_m2 > 0.0);
        }
        let zone_exchange_w = state
            .surfaces
            .iter()
            .map(|surface| surface.inside_net_longwave_w_per_m2 * surface.area_m2)
            .sum::<f64>();
        assert!(zone_exchange_w.abs() < 1.0e-9);

        Ok(())
    }

    #[test]
    fn energyplus_ctf_outside_face_balance_uses_ctf_zero_terms()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.inside_face_temperature_c = 20.0;
        surface.ctf.outside_0_w_per_m2_k = 1.0;
        surface.ctf.cross_0_w_per_m2_k = 1.0;
        surface.ctf.const_out_part_w_per_m2 = 0.0;

        let temperature = energyplus_ctf_outside_face_temperature_c(
            surface,
            CtfOutsideFaceBalanceInput {
                outdoor_air_temperature_c: 10.0,
                radiant_temperature_c: 5.0,
                outside_convection_coefficient_w_per_m2_k: 3.0,
                outside_radiation_coefficient_w_per_m2_k: 2.0,
                absorbed_outside_source_w_per_m2: 7.0,
            },
        );

        assert!((temperature - (67.0 / 6.0)).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn energyplus_ctf_quick_outside_face_balance_uses_inside_balance_term()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.ctf.outside_0_w_per_m2_k = 3.0;
        surface.ctf.cross_0_w_per_m2_k = 2.0;
        surface.ctf.inside_0_w_per_m2_k = 4.0;
        surface.ctf.const_out_part_w_per_m2 = 11.0;
        surface.ctf.const_in_part_w_per_m2 = 13.0;

        let temperature = energyplus_ctf_outside_face_temperature_quick_conduction_c(
            surface,
            CtfOutsideQuickConductionBalanceInput {
                environmental: CtfOutsideFaceBalanceInput {
                    outdoor_air_temperature_c: 10.0,
                    radiant_temperature_c: 5.0,
                    outside_convection_coefficient_w_per_m2_k: 3.0,
                    outside_radiation_coefficient_w_per_m2_k: 2.0,
                    absorbed_outside_source_w_per_m2: 7.0,
                },
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 6.0,
                net_inside_source_w_per_m2: 17.0,
            },
        );

        assert!((temperature - (66.0 / 7.6)).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn heat_balance_timestep_advances_zone_air_state() -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: 10.0,
                hour_ending: 1,
                timestep_seconds: 600.0,
            },
        );

        assert_eq!(state.timestep_index, 1);
        assert_eq!(state.zones[0].previous_mean_air_temperatures_c, [20.0; 3]);
        assert_eq!(state.zones[0].convective_internal_gain_w, 12.0);
        assert_eq!(state.zones[0].opaque_surface_conductance_w_per_k, 6.0);
        assert!(state.zones[0].mean_air_temperature_c > 12.0);
        assert!(state.zones[0].mean_air_temperature_c < 20.0);
        assert!(state.zones[0].opaque_surface_heat_gain_w < 0.0);
        assert_eq!(state.surfaces[0].outside_face_temperature_c, 10.0);
        assert!(
            state.surfaces[0].inside_face_temperature_c > state.zones[0].mean_air_temperature_c
        );
        assert!(state.surfaces[0].inside_face_temperature_c < 20.0);
        assert!(state.surfaces[0].heat_gain_to_zone_w < 0.0);
        let expected_sum_ha = state
            .surfaces
            .iter()
            .map(|surface| surface.inside_convection_coefficient_w_per_m2_k * surface.area_m2)
            .sum::<f64>();
        let expected_sum_hat_surf = state
            .surfaces
            .iter()
            .map(|surface| {
                surface.inside_convection_coefficient_w_per_m2_k
                    * surface.area_m2
                    * surface.inside_face_temperature_c
            })
            .sum::<f64>();
        assert!((state.zones[0].sum_ha_w_per_k - expected_sum_ha).abs() < 1.0e-12);
        assert!((state.zones[0].sum_hat_surf_w - expected_sum_hat_surf).abs() < 1.0e-12);
        assert_eq!(state.zones[0].sum_hat_ref_w, 0.0);
        let coefficients = state.zones[0].zone_air_temperature_coefficients;
        assert!(
            (coefficients.temp_dependent_coefficient_w_per_k - expected_sum_ha).abs() < 1.0e-12
        );
        assert!(
            (coefficients.temp_independent_coefficient_w
                - (state.zones[0].convective_internal_gain_w + expected_sum_hat_surf))
                .abs()
                < 1.0e-12
        );
        assert!((coefficients.air_power_cap_w_per_k - (1207.2 / 600.0)).abs() < 1.0e-12);
        let expected_history = (1207.2 / 600.0) * (3.0 * 20.0 - 1.5 * 20.0 + 20.0 / 3.0);
        assert!((coefficients.third_order_history_term_w - expected_history).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn zone_air_heat_balance_storage_rate_uses_source_algorithm_branch()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone = &mut state.zones[0];
        zone.mean_air_temperature_c = 21.0;
        zone.previous_mean_air_temperatures_c = [20.0, 19.0, 18.0];
        zone.air_heat_capacity_j_per_k = 1200.0;
        zone.zone_air_temperature_coefficients
            .temp_dependent_coefficient_w_per_k = 5.0;
        zone.zone_air_temperature_coefficients
            .temp_independent_coefficient_w = 200.0;

        let analytical = zone_air_heat_balance_air_storage_rate_w(
            zone,
            60.0,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
        );
        assert!((analytical - 95.0).abs() < 1.0e-12);

        let third_order = zone_air_heat_balance_air_storage_rate_w(
            zone,
            60.0,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe,
        );
        assert!((third_order - 20.0).abs() < 1.0e-12);

        let invalid_timestep = zone_air_heat_balance_air_storage_rate_w(
            zone,
            0.0,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe,
        );
        assert_eq!(invalid_timestep, 0.0);

        Ok(())
    }

    #[test]
    fn heat_balance_timestep_uses_previous_surface_temperature_for_ctf_damping()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        state.surfaces[0].inside_face_temperature_c = 40.0;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: 20.0,
                hour_ending: 1,
                timestep_seconds: 60.0,
            },
        );

        assert!(
            state.surfaces[0].inside_face_temperature_c > 25.0,
            "CTF damping should use the previous surface temperature, not the overwritten zone temperature"
        );

        Ok(())
    }

    #[test]
    fn heat_balance_adiabatic_surfaces_do_not_create_artificial_losses()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        for surface in &mut typed.surfaces {
            surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
            surface.outside_boundary_condition_object = None;
        }
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: -10.0,
                hour_ending: 1,
                timestep_seconds: 600.0,
            },
        );

        assert!(state.zones[0].mean_air_temperature_c > 20.0);
        assert!((state.zones[0].opaque_surface_heat_gain_w).abs() < 1.0e-9);
        for surface in &state.surfaces {
            assert_eq!(
                surface.outside_boundary_condition,
                OutsideBoundaryCondition::Adiabatic
            );
            assert_eq!(
                surface.outside_face_temperature_c,
                surface.inside_face_temperature_c
            );
            assert!(surface.heat_gain_to_zone_w.abs() < 1.0e-9);
        }

        Ok(())
    }

    #[test]
    fn heat_balance_interzone_surface_uses_adjacent_zone_temperature()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = two_zone_interzone_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        state.zones[0].mean_air_temperature_c = 20.0;
        state.zones[1].mean_air_temperature_c = 10.0;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: 0.0,
                hour_ending: 1,
                timestep_seconds: 60.0,
            },
        );

        let warm_zone = state
            .zones
            .iter()
            .find(|zone| zone.zone_name == "ZONE A")
            .ok_or_else(|| std::io::Error::other("missing warm zone"))?;
        let cool_zone = state
            .zones
            .iter()
            .find(|zone| zone.zone_name == "ZONE B")
            .ok_or_else(|| std::io::Error::other("missing cool zone"))?;
        assert!(warm_zone.mean_air_temperature_c < 20.0);
        assert!(cool_zone.mean_air_temperature_c > 10.0);

        let warm_surface = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "A WALL")
            .ok_or_else(|| std::io::Error::other("missing A WALL"))?;
        assert_eq!(
            warm_surface.outside_boundary_target_surface_id,
            Some(SurfaceId(1))
        );
        assert_eq!(
            warm_surface.outside_boundary_target_zone_id,
            Some(ZoneId(1))
        );
        assert_eq!(
            warm_surface.outside_face_temperature_c,
            cool_zone.mean_air_temperature_c
        );
        assert!(warm_surface.heat_gain_to_zone_w < 0.0);

        let cool_surface = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "B WALL")
            .ok_or_else(|| std::io::Error::other("missing B WALL"))?;
        assert_eq!(
            cool_surface.outside_face_temperature_c,
            warm_zone.mean_air_temperature_c
        );
        assert!(cool_surface.heat_gain_to_zone_w > 0.0);

        Ok(())
    }

    #[test]
    fn heat_balance_missing_interzone_surface_target_fails() {
        let mut typed = two_zone_interzone_model();
        typed.surfaces[0].outside_boundary_condition_object =
            Some(NormalizedName::new("Missing Surface"));
        let model = SimulationModel::from_typed(typed);

        assert!(matches!(
            initialize_heat_balance_state(&model, 20.0),
            Err(RuntimeError::MissingSurfaceBoundaryTarget { .. })
        ));
    }

    #[test]
    fn heat_balance_trace_writes_zone_air_temperature_results()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());

        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;

        assert_eq!(simulation.summary.samples, 2);
        assert_eq!(simulation.summary.timestep_count, 12);
        assert_eq!(simulation.summary.zone_count, 1);
        assert_eq!(simulation.summary.surface_count, 6);
        assert_eq!(simulation.state.timestep_index, 12);
        assert_eq!(simulation.results.sample_count(), 2);
        assert_eq!(simulation.results.series.len(), 74);

        let Some(zone_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing zone series").into());
        };
        assert!(zone_series.values[0] > 11.9);
        assert!(zone_series.values[0] < 20.0);
        assert!(zone_series.values[1] > zone_series.values[0]);

        let Some(weather_series) = simulation
            .results
            .find_series("Environment", "Site Outdoor Air Drybulb Temperature")
        else {
            return Err(std::io::Error::other("missing weather series").into());
        };
        assert_eq!(weather_series.values, vec![10.0, 12.0]);

        let Some(inside_surface_series) = simulation
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing inside surface series").into());
        };
        assert_eq!(inside_surface_series.values.len(), 2);
        assert!(inside_surface_series.values[0].is_finite());
        assert_ne!(inside_surface_series.values[0], zone_series.values[0]);

        let Some(outside_surface_series) = simulation
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing outside surface series").into());
        };
        assert_eq!(outside_surface_series.values, vec![10.0, 12.0]);

        let Some(inside_conduction_series) = simulation
            .results
            .find_series("FLOOR", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing inside conduction series").into());
        };
        assert_eq!(inside_conduction_series.values.len(), 2);
        assert!(inside_conduction_series.values[0] < 0.0);

        let Some(outside_conduction_series) = simulation.results.find_series(
            "FLOOR",
            "Surface Outside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing outside conduction series").into());
        };
        assert_eq!(
            outside_conduction_series.values[0],
            -inside_conduction_series.values[0]
        );

        let Some(storage_series) = simulation
            .results
            .find_series("FLOOR", "Surface Heat Storage Rate")
        else {
            return Err(std::io::Error::other("missing surface heat storage series").into());
        };
        assert_eq!(storage_series.values.len(), 2);
        assert!(
            (storage_series.values[0]
                + inside_conduction_series.values[0]
                + outside_conduction_series.values[0])
                .abs()
                < 1.0e-9
        );

        let Some(zone_conduction_series) = simulation.results.find_series(
            "ZONE ONE",
            "Zone Opaque Surface Inside Faces Conduction Rate",
        ) else {
            return Err(std::io::Error::other("missing zone conduction series").into());
        };
        assert!(zone_conduction_series.values[0] < 0.0);

        let Some(surface_convection_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate")
        else {
            return Err(std::io::Error::other("missing zone air surface convection series").into());
        };
        assert_eq!(surface_convection_series.values.len(), 2);
        assert!(surface_convection_series.values[0].is_finite());

        Ok(())
    }

    #[test]
    fn heat_balance_zone_air_algorithm_option_defaults_to_simplified() {
        let options = HeatBalanceSimulationOptions::hourly_samples(2);

        assert_eq!(
            options.zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        );
        assert_eq!(options.surface_iteration_count, 1);
        assert_eq!(
            options
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe)
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe,
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe)
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe
        );
        assert_eq!(
            options
                .with_surface_iteration_count(0)
                .surface_iteration_count,
            1
        );
        assert_eq!(
            options
                .with_surface_iteration_count(3)
                .surface_iteration_count,
            3
        );
    }

    #[test]
    fn heat_balance_warmup_minimum_override_preserves_disabled_boundary() {
        let disabled = HeatBalanceSimulationOptions::hourly_samples(3).with_warmup_minimum_days(20);
        assert!(!disabled.warmup.enabled);
        assert_eq!(disabled.warmup.minimum_days, 0);

        let mut enabled = HeatBalanceSimulationOptions::hourly_samples(3);
        enabled.warmup = HeatBalanceWarmupOptions {
            enabled: true,
            minimum_days: 6,
            maximum_days: 10,
            temperature_convergence_tolerance_delta_c: 0.1,
        };
        let overridden = enabled.with_warmup_minimum_days(20);
        assert_eq!(overridden.warmup.minimum_days, 20);
        assert_eq!(overridden.warmup.maximum_days, 20);
    }

    #[test]
    fn heat_balance_warmup_uses_weather_context_for_exterior_forcing()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.timestep = TimestepConfig {
            number_of_timesteps_per_hour: 1,
        };
        typed.site = Some(SiteLocation {
            name: NormalizedName::new("Golden"),
            latitude_deg: 39.75,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        });
        let model = SimulationModel::from_typed(typed.clone());
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
2013,6,21,12,0,Source,25.0,5.0,30,82000,0,0,300,900,800,100,0,0,0,0,180,2.5
2013,6,21,13,0,Source,26.0,5.0,30,82000,0,0,300,920,820,100,0,0,0,0,180,2.5
"#,
        )?;
        let weather_dry_bulb_c = records
            .iter()
            .map(|record| record.dry_bulb_c)
            .collect::<Vec<_>>();
        let options = HeatBalanceWarmupOptions {
            enabled: true,
            minimum_days: 1,
            maximum_days: 1,
            temperature_convergence_tolerance_delta_c: 0.0,
        };
        let mut dry_only_state = initialize_heat_balance_state(&model, 20.0)?;
        let mut weather_context_state = initialize_heat_balance_state(&model, 20.0)?;

        let dry_only_summary = run_heat_balance_run_period_warmup(
            &typed,
            &mut dry_only_state,
            &weather_dry_bulb_c,
            None,
            1,
            SECONDS_PER_HOUR,
            options,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            1,
        );
        let weather_context_summary = run_heat_balance_run_period_warmup(
            &typed,
            &mut weather_context_state,
            &weather_dry_bulb_c,
            Some(&records),
            1,
            SECONDS_PER_HOUR,
            options,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            1,
        );

        assert_eq!(dry_only_summary.day_count, 1);
        assert_eq!(weather_context_summary.day_count, 1);
        let dry_only_roof = dry_only_state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing dry-only roof"))?;
        let weather_context_roof = weather_context_state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing weather-context roof"))?;

        assert!(
            weather_context_roof.outside_face_temperature_c
                > dry_only_roof.outside_face_temperature_c + 1.0
        );

        Ok(())
    }

    #[test]
    fn heat_balance_third_order_probe_runs_as_diagnostic_option()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe),
        )?;

        assert_eq!(simulation.summary.samples, 2);
        assert_eq!(simulation.summary.timestep_count, 12);
        let Some(zone_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing zone series").into());
        };
        assert_eq!(zone_series.values.len(), 2);
        assert!(zone_series.values.iter().all(|value| value.is_finite()));
        assert_eq!(
            simulation.summary.warmup,
            HeatBalanceWarmupSummary::disabled()
        );

        Ok(())
    }

    #[test]
    fn heat_balance_surface_first_probe_uses_distinct_zone_air_order()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let analytical = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe),
        )?;
        let surface_first = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2).with_zone_air_algorithm(
                HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe,
            ),
        )?;
        let coupled = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2).with_zone_air_algorithm(
                HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe,
            ),
        )?;

        let Some(analytical_zone_series) = analytical
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing analytical zone series").into());
        };
        let Some(surface_first_zone_series) = surface_first
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing surface-first zone series").into());
        };
        let Some(coupled_zone_series) = coupled
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing coupled zone series").into());
        };

        assert_eq!(analytical_zone_series.values.len(), 2);
        assert_eq!(surface_first_zone_series.values.len(), 2);
        assert_eq!(coupled_zone_series.values.len(), 2);
        assert!(
            analytical_zone_series
                .values
                .iter()
                .chain(surface_first_zone_series.values.iter())
                .chain(coupled_zone_series.values.iter())
                .all(|value| value.is_finite())
        );
        assert!(
            (analytical_zone_series.values[0] - surface_first_zone_series.values[0]).abs() > 1.0e-6
        );
        assert!(
            (surface_first_zone_series.values[0] - coupled_zone_series.values[0]).abs() > 1.0e-6
        );

        Ok(())
    }

    #[test]
    fn surface_incident_solar_diagnostic_appends_roof_series()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.site = Some(SiteLocation {
            name: NormalizedName::new("Solar Test Site"),
            latitude_deg: 39.75,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        });
        let model = SimulationModel::from_typed(typed);
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
2013,6,21,12,0,Source,25.0,5.0,30,82000,0,0,300,900,800,100,0,0,0,0,180,2.5
2013,6,21,13,0,Source,26.0,5.0,30,82000,0,0,300,920,820,100,0,0,0,0,180,2.5
"#,
        )?;
        let weather_values = records
            .iter()
            .map(|record| record.dry_bulb_c)
            .collect::<Vec<_>>();
        let mut simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;

        let added = append_surface_incident_solar_radiation_series(
            &mut simulation.results,
            &model,
            &records,
            2,
        );

        assert_eq!(added, 5);
        assert!(
            simulation
                .results
                .find_series(
                    "FLOOR",
                    "Surface Outside Face Incident Solar Radiation Rate per Area"
                )
                .is_none()
        );
        let Some(roof_solar) = simulation.results.find_series(
            "ROOF",
            "Surface Outside Face Incident Solar Radiation Rate per Area",
        ) else {
            return Err(std::io::Error::other("missing roof solar series").into());
        };
        assert_eq!(roof_solar.units, "W/m2");
        assert_eq!(roof_solar.values.len(), 2);
        assert!(roof_solar.values[0].is_finite());
        assert!(roof_solar.values[0] > 600.0);

        Ok(())
    }

    #[test]
    fn weather_record_exterior_balance_forces_exterior_conduction()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.site = Some(SiteLocation {
            name: NormalizedName::new("Solar Test Site"),
            latitude_deg: 39.75,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        });
        let model = SimulationModel::from_typed(typed);
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
2013,6,21,12,0,Source,25.0,5.0,30,82000,0,0,300,900,800,100,0,0,0,0,180,2.5
2013,6,21,13,0,Source,26.0,5.0,30,82000,0,0,300,920,820,100,0,0,0,0,180,2.5
"#,
        )?;
        let weather_values = records
            .iter()
            .map(|record| record.dry_bulb_c)
            .collect::<Vec<_>>();
        let dry_bulb_only = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;
        let weather_forced = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;
        let coupled = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let previous_inside = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(dry_roof_conduction) = dry_bulb_only
            .results
            .find_series("ROOF", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing dry roof conduction series").into());
        };
        let Some(forced_roof_conduction) = weather_forced
            .results
            .find_series("ROOF", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing forced roof conduction series").into());
        };
        let Some(dry_wall_conduction) = dry_bulb_only.results.find_series(
            "WALL Y0",
            "Surface Inside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing dry wall conduction series").into());
        };
        let Some(forced_wall_conduction) = weather_forced.results.find_series(
            "WALL Y0",
            "Surface Inside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing forced wall conduction series").into());
        };
        let Some(coupled_roof_temperature) = coupled
            .results
            .find_series("ROOF", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing coupled roof temperature series").into());
        };
        let Some(previous_inside_roof_temperature) = previous_inside
            .results
            .find_series("ROOF", "Surface Outside Face Temperature")
        else {
            return Err(
                std::io::Error::other("missing previous-inside roof temperature series").into(),
            );
        };

        assert_eq!(dry_roof_conduction.values.len(), 2);
        assert_eq!(forced_roof_conduction.values.len(), 2);
        assert_eq!(dry_wall_conduction.values.len(), 2);
        assert_eq!(forced_wall_conduction.values.len(), 2);
        assert_eq!(coupled_roof_temperature.values.len(), 2);
        assert_eq!(previous_inside_roof_temperature.values.len(), 2);
        assert!((dry_roof_conduction.values[0] - forced_roof_conduction.values[0]).abs() > 1.0e-3);
        assert!((dry_wall_conduction.values[0] - forced_wall_conduction.values[0]).abs() > 1.0e-3);
        assert!(
            (coupled_roof_temperature.values[0] - previous_inside_roof_temperature.values[0]).abs()
                > 1.0e-6
        );

        Ok(())
    }

    #[test]
    fn previous_boundary_probe_keeps_adiabatic_outside_face_history()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![10.0, 12.0];

        let coupled = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let previous_boundary = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(coupled_floor_outside_temperature) = coupled
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing coupled floor outside temperature").into());
        };
        let Some(coupled_floor_inside_temperature) = coupled
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing coupled floor inside temperature").into());
        };
        let Some(previous_boundary_floor_outside_temperature) = previous_boundary
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other(
                "missing previous-boundary floor outside temperature",
            )
            .into());
        };

        assert_eq!(coupled_floor_outside_temperature.values.len(), 2);
        assert_eq!(previous_boundary_floor_outside_temperature.values.len(), 2);
        assert_eq!(
            coupled_floor_outside_temperature.values[0],
            coupled_floor_inside_temperature.values[0]
        );
        assert!(
            (coupled_floor_outside_temperature.values[0]
                - previous_boundary_floor_outside_temperature.values[0])
                .abs()
                > 1.0e-6
        );

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
    fn runtime_output_registry_resolves_declared_model_outputs() {
        let model = SimulationModel::from_typed(cube_model());
        let registry = RuntimeOutputRegistry::from_model(&model);

        assert_eq!(registry.len(), 78);
        assert!(registry.meter_registry().is_empty());

        let resolution = registry.resolve_output_requests(&[
            RuntimeOutputRequest::hourly("zone one", "Zone Mean Air Temperature"),
            RuntimeOutputRequest::hourly("floor", "Surface Inside Face Temperature"),
            RuntimeOutputRequest::hourly(
                "floor",
                "Surface Inside Face Conduction Heat Transfer Rate",
            ),
            RuntimeOutputRequest::hourly(
                "zone one",
                "Zone Opaque Surface Inside Faces Conduction Rate",
            ),
            RuntimeOutputRequest::hourly("floor", "Surface Heat Storage Rate"),
            RuntimeOutputRequest::hourly(
                "floor",
                "Surface Outside Face Incident Solar Radiation Rate per Area",
            ),
            RuntimeOutputRequest::hourly("environment", "Site Outdoor Air Drybulb Temperature"),
        ]);

        assert!(resolution.diagnostics.is_empty());
        assert_eq!(resolution.resolved.len(), 7);
        assert_eq!(resolution.resolved[0].definition.handle, OutputHandle(0));
        assert_eq!(resolution.resolved[1].definition.key, "FLOOR");
    }

    #[test]
    fn runtime_output_registry_skips_no_sun_surface_solar_output() {
        let mut typed = cube_model();
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        let model = SimulationModel::from_typed(typed);
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry.resolve_output_requests(&[RuntimeOutputRequest::hourly(
            "floor",
            "Surface Outside Face Incident Solar Radiation Rate per Area",
        )]);

        assert!(resolution.resolved.is_empty());
        assert!(resolution.diagnostics.has_errors());
        assert_eq!(
            resolution.diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::OutputVariableUnavailable
        );
    }

    #[test]
    fn runtime_output_registry_diagnoses_unavailable_output() {
        let model = SimulationModel::from_typed(cube_model());
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry.resolve_output_requests(&[RuntimeOutputRequest::hourly(
            "ZONE ONE",
            "Zone Lights Electricity Energy",
        )]);

        assert!(resolution.resolved.is_empty());
        assert!(resolution.diagnostics.has_errors());
        assert_eq!(
            resolution.diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::OutputVariableUnavailable
        );
    }

    #[test]
    fn runtime_meter_registry_diagnoses_unavailable_meter() {
        let model = SimulationModel::from_typed(cube_model());
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry
            .meter_registry()
            .resolve_meter_requests(&[RuntimeMeterRequest::hourly("Electricity:Facility")]);

        assert!(resolution.resolved.is_empty());
        assert!(resolution.diagnostics.has_errors());
        assert_eq!(
            resolution.diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::MeterUnavailable
        );
    }

    #[test]
    fn result_store_diagnostics_report_duplicate_handles() {
        let mut store = ResultStore::new();
        store.add_series(OutputSeries {
            handle: OutputHandle(0),
            key: "ZONE ONE".to_string(),
            variable_name: "Zone Mean Air Temperature".to_string(),
            units: "C".to_string(),
            values: vec![20.0],
        });
        store.add_series(OutputSeries {
            handle: OutputHandle(0),
            key: "Environment".to_string(),
            variable_name: "Site Outdoor Air Drybulb Temperature".to_string(),
            units: "C".to_string(),
            values: vec![10.0],
        });

        let diagnostics = store.diagnostics();

        assert!(diagnostics.has_errors());
        assert_eq!(
            diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::DuplicateOutputHandle
        );
        assert_eq!(store.profile().series_count, 2);
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
            roughness: Some(MaterialSurfaceRoughness::Rough),
            conductivity_w_per_m_k: None,
            density_kg_per_m3: None,
            specific_heat_j_per_kg_k: None,
            thickness_m: None,
            thermal_resistance_m2_k_per_w: Some(1.0),
            thermal_absorptance: Some(0.9),
            solar_absorptance: Some(0.75),
            visible_absorptance: Some(0.75),
        });
        model.constructions.push(Construction {
            id: ConstructionId(0),
            name: NormalizedName::new("Wall"),
            outside_layer: MaterialId(0),
            layers: vec![MaterialId(0)],
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

    fn two_zone_interzone_model() -> TypedModel {
        let mut model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 1,
            },
            ..TypedModel::default()
        };
        model.materials.push(Material {
            id: MaterialId(0),
            name: NormalizedName::new("R1"),
            kind: MaterialKind::NoMass,
            roughness: Some(MaterialSurfaceRoughness::Rough),
            conductivity_w_per_m_k: None,
            density_kg_per_m3: None,
            specific_heat_j_per_kg_k: None,
            thickness_m: None,
            thermal_resistance_m2_k_per_w: Some(1.0),
            thermal_absorptance: Some(0.9),
            solar_absorptance: Some(0.75),
            visible_absorptance: Some(0.75),
        });
        model.constructions.push(Construction {
            id: ConstructionId(0),
            name: NormalizedName::new("Wall"),
            outside_layer: MaterialId(0),
            layers: vec![MaterialId(0)],
        });
        model.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone A"),
            direction_of_relative_north_deg: 0.0,
            origin: point(0.0, 0.0, 0.0),
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::Value(1.0),
        });
        model.zones.push(Zone {
            id: ZoneId(1),
            name: NormalizedName::new("Zone B"),
            direction_of_relative_north_deg: 0.0,
            origin: point(1.0, 0.0, 0.0),
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::Value(1.0),
        });
        model.surfaces.push(interzone_surface(
            0,
            "A Wall",
            ZoneId(0),
            "B Wall",
            [
                point(1.0, 0.0, 0.0),
                point(1.0, 1.0, 0.0),
                point(1.0, 1.0, 1.0),
                point(1.0, 0.0, 1.0),
            ],
        ));
        model.surfaces.push(interzone_surface(
            1,
            "B Wall",
            ZoneId(1),
            "A Wall",
            [
                point(0.0, 0.0, 0.0),
                point(0.0, 0.0, 1.0),
                point(0.0, 1.0, 1.0),
                point(0.0, 1.0, 0.0),
            ],
        ));
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
                    point(0.0, 1.0, 1.0),
                    point(1.0, 1.0, 1.0),
                    point(1.0, 0.0, 1.0),
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
                    point(1.0, 0.0, 1.0),
                    point(1.0, 1.0, 1.0),
                    point(1.0, 1.0, 0.0),
                ],
            ),
            surface(
                4,
                "Wall Y0",
                SurfaceType::Wall,
                [
                    point(0.0, 0.0, 0.0),
                    point(0.0, 0.0, 1.0),
                    point(1.0, 0.0, 1.0),
                    point(1.0, 0.0, 0.0),
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

    fn interzone_surface(
        id: u32,
        name: &str,
        zone: ZoneId,
        target_surface: &str,
        vertices: [Point3; 4],
    ) -> Surface {
        Surface {
            id: SurfaceId(id),
            name: NormalizedName::new(name),
            surface_type: SurfaceType::Wall,
            construction: ConstructionId(0),
            zone,
            outside_boundary_condition: OutsideBoundaryCondition::Surface,
            outside_boundary_condition_object: Some(NormalizedName::new(target_surface)),
            sun_exposure: ep_model::SunExposure::NoSun,
            wind_exposure: ep_model::WindExposure::NoWind,
            view_factor_to_ground: AutoOrNumber::AutoCalculate,
            vertices: vertices.to_vec(),
        }
    }

    fn point(x_m: f64, y_m: f64, z_m: f64) -> Point3 {
        Point3 { x_m, y_m, z_m }
    }
}
