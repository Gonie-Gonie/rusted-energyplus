//! Runtime state, execution-plan shells, and first trace helpers.

use ep_model::{
    AutoOrNumber, AutosizeOrNumber, BranchId, BranchListId, ConstructionId, IdealLoadsAirSystem,
    IdealLoadsAirSystemId, LoopId, MaterialId, NodeId, NormalizedName, OtherEquipment,
    OutputHandle, OutsideBoundaryCondition, PlantBranchComponent, PlantLoop, Point3, RunPeriod,
    RunPeriodId, ScheduleCompactSegment, ScheduleId, SimulationModel, Surface, SurfaceId,
    SurfaceType, TypedModel, Zone, ZoneEquipmentConnection, ZoneId, ZoneThermostatId,
};
use std::fmt::{Display, Formatter};
use std::path::Path;

const AIR_DENSITY_KG_PER_M3: f64 = 1.2;
const AIR_SPECIFIC_HEAT_J_PER_KG_K: f64 = 1006.0;
const SECONDS_PER_HOUR: f64 = 3600.0;
const ENERGYPLUS_ZONE_INITIAL_TEMP_C: f64 = 23.0;
const DEFAULT_RUN_PERIOD_YEAR: u32 = 2013;

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
    /// Resolved construction ID.
    pub construction_id: ConstructionId,
    /// EnergyPlus-normalized construction name.
    pub construction_name: String,
    /// Outside layer material ID.
    pub outside_layer_material_id: MaterialId,
    /// EnergyPlus-normalized outside layer material name.
    pub outside_layer_material_name: String,
    /// Surface area in square meters.
    pub area_m2: f64,
    /// Area-normalized thermal resistance in m2-K/W.
    pub thermal_resistance_m2_k_per_w: f64,
    /// Area-normalized heat capacity in J/m2-K when available.
    pub heat_capacity_j_per_m2_k: Option<f64>,
    /// Surface conductance in W/K.
    pub conductance_w_per_k: f64,
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

/// Options for the heat-balance zone-air diagnostic trace.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HeatBalanceSimulationOptions {
    /// Number of hourly weather samples to execute.
    pub sample_count: usize,
    /// Initial zone mean air temperature in C.
    pub initial_zone_air_temperature_c: f64,
}

impl HeatBalanceSimulationOptions {
    /// Creates options with a fixed hourly sample count.
    #[must_use]
    pub const fn hourly_samples(sample_count: usize) -> Self {
        Self {
            sample_count,
            initial_zone_air_temperature_c: ENERGYPLUS_ZONE_INITIAL_TEMP_C,
        }
    }
}

/// Summary for the heat-balance zone-air diagnostic trace.
#[derive(Clone, Debug, PartialEq)]
pub struct HeatBalanceSimulationSummary {
    /// Hourly output sample count.
    pub samples: usize,
    /// Number of executed zone timesteps.
    pub timestep_count: usize,
    /// Number of zones represented in the state.
    pub zone_count: usize,
    /// Number of surfaces represented in the state.
    pub surface_count: usize,
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
    /// Returns the diagnostic-only v0.16 plant-state evidence policy.
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
        });
    }

    let surfaces = model
        .typed
        .surfaces
        .iter()
        .map(|surface| {
            let area_m2 = surface_area_m2(&surface.vertices);
            let thermal = surface_thermal_properties(&model.typed, surface)?;

            Ok(SurfaceHeatBalanceState {
                surface_id: surface.id,
                zone_id: surface.zone,
                surface_name: surface.name.0.clone(),
                surface_type: surface.surface_type,
                outside_boundary_condition: surface.outside_boundary_condition,
                construction_id: thermal.construction_id,
                construction_name: thermal.construction_name,
                outside_layer_material_id: thermal.outside_layer_material_id,
                outside_layer_material_name: thermal.outside_layer_material_name,
                area_m2,
                thermal_resistance_m2_k_per_w: thermal.thermal_resistance_m2_k_per_w,
                heat_capacity_j_per_m2_k: thermal.heat_capacity_j_per_m2_k,
                conductance_w_per_k: area_m2 / thermal.thermal_resistance_m2_k_per_w,
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
    let hour_ending = input.hour_ending.clamp(1, 24);
    let previous_zone_temperatures = state
        .zones
        .iter()
        .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
        .collect::<Vec<_>>();

    for surface in &mut state.surfaces {
        let zone_temperature_c = previous_zone_temperatures
            .iter()
            .find(|(zone_id, _temperature)| *zone_id == surface.zone_id)
            .map(|(_zone_id, temperature)| *temperature)
            .unwrap_or(surface.inside_face_temperature_c);

        surface.inside_face_temperature_c = zone_temperature_c;
        if surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors {
            surface.outside_face_temperature_c = input.outdoor_dry_bulb_c;
        }
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
        zone.mean_air_temperature_c = step_zone_air_temperature(
            previous_temperature_c,
            equivalent_outside_temperature_c,
            zone.convective_internal_gain_w,
            conductance_w_per_k,
            zone.air_heat_capacity_j_per_k,
            input.timestep_seconds,
        );
    }

    for surface in &mut state.surfaces {
        let zone_temperature_c = state
            .zones
            .iter()
            .find(|zone| zone.zone_id == surface.zone_id)
            .map(|zone| zone.mean_air_temperature_c)
            .unwrap_or(surface.inside_face_temperature_c);

        surface.inside_face_temperature_c = zone_temperature_c;
        surface.heat_gain_to_zone_w =
            surface.conductance_w_per_k * (surface.outside_face_temperature_c - zone_temperature_c);
    }

    for zone in &mut state.zones {
        zone.opaque_surface_heat_gain_w = state
            .surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone.zone_id)
            .map(|surface| surface.heat_gain_to_zone_w)
            .sum();
    }

    state.timestep_index += 1;
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
    let mut state = initialize_heat_balance_state(model, options.initial_zone_air_temperature_c)?;
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
    let mut surface_temperatures = state
        .surfaces
        .iter()
        .map(|surface| {
            (
                surface.surface_id,
                surface.surface_name.clone(),
                Vec::with_capacity(options.sample_count),
                Vec::with_capacity(options.sample_count),
            )
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
        for _substep in 0..zone_steps_per_hour {
            advance_heat_balance_state_one_timestep(
                &model.typed,
                &mut state,
                HeatBalanceStepInput {
                    outdoor_dry_bulb_c,
                    hour_ending,
                    timestep_seconds: seconds_per_timestep,
                },
            );
        }

        for (zone_id, _zone_name, values) in &mut zone_temperatures {
            if let Some(zone_state) = state.zones.iter().find(|zone| zone.zone_id == *zone_id) {
                values.push(zone_state.mean_air_temperature_c);
            }
        }
        for (surface_id, _surface_name, inside_values, outside_values) in &mut surface_temperatures
        {
            if let Some(surface_state) = state
                .surfaces
                .iter()
                .find(|surface| surface.surface_id == *surface_id)
            {
                inside_values.push(surface_state.inside_face_temperature_c);
                outside_values.push(surface_state.outside_face_temperature_c);
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
    for (_surface_id, surface_name, inside_values, outside_values) in surface_temperatures {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: surface_name.clone(),
            variable_name: "Surface Inside Face Temperature".to_string(),
            units: "C".to_string(),
            values: inside_values,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: surface_name,
            variable_name: "Surface Outside Face Temperature".to_string(),
            units: "C".to_string(),
            values: outside_values,
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
        zone_count: state.zones.len(),
        surface_count: state.surfaces.len(),
    };

    Ok(HeatBalanceSimulation {
        state,
        results,
        summary,
    })
}

#[derive(Clone, Debug, PartialEq)]
struct SurfaceThermalProperties {
    construction_id: ConstructionId,
    construction_name: String,
    outside_layer_material_id: MaterialId,
    outside_layer_material_name: String,
    thermal_resistance_m2_k_per_w: f64,
    heat_capacity_j_per_m2_k: Option<f64>,
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
    let material = model
        .materials
        .iter()
        .find(|material| material.id == construction.outside_layer)
        .ok_or_else(|| RuntimeError::MissingMaterial {
            construction_name: construction.name.0.clone(),
        })?;
    let thermal_resistance_m2_k_per_w =
        material
            .thermal_resistance()
            .ok_or_else(|| RuntimeError::MissingThermalResistance {
                material_name: material.name.0.clone(),
            })?;

    Ok(SurfaceThermalProperties {
        construction_id: construction.id,
        construction_name: construction.name.0.clone(),
        outside_layer_material_id: material.id,
        outside_layer_material_name: material.name.0.clone(),
        thermal_resistance_m2_k_per_w,
        heat_capacity_j_per_m2_k: material.heat_capacity_per_area(),
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
        ExecutionStep, FirstZoneSimulationOptions, HeatBalanceSimulationOptions,
        HeatBalanceStepInput, NODE_STATE_EXCLUDED_SETPOINT_VARIABLE, NODE_STATE_SOURCE_MAP_PATH,
        NODE_TEMPERATURE_SETPOINT_SENTINEL_C, NodeStateProjectionOptions, NodeStateRole,
        OutputSeries, PLANT_STATE_SOURCE_MAP_PATH, PlantEquipmentRole, PlantStateProjectionOptions,
        ResultStore, SimulationMode, SimulationState, advance_heat_balance_state_one_timestep,
        build_execution_plan, build_hourly_time_axis, build_hourly_time_axis_for_run_period,
        initialize_heat_balance_state, node_temperature_setpoint_from_energyplus,
        parse_epw_dry_bulb_series, parse_epw_records, simulate_constant_schedules,
        simulate_first_zone_uncontrolled, simulate_heat_balance_zone_air_temperatures,
        simulate_ideal_loads_node_state_projection, simulate_plant_state_projection,
        simulate_schedule_values, simulate_zone_internal_convective_gains, surface_area_m2,
        surface_geometry_summaries, zone_geometry_summaries,
    };
    use ep_model::{
        AutoOrNumber, AutosizeOrNumber, BranchId, BranchListId, Construction, ConstructionId,
        DehumidificationControlType, DemandControlledVentilationType, HeatRecoveryType,
        HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
        IdealLoadsLimit, InternalGainId, LoadDistributionScheme, LoopId, Material, MaterialId,
        MaterialKind, Node, NodeId, NodeList, NodeListId, NormalizedName, OtherEquipment,
        OutdoorAirEconomizerType, OutputHandle, OutsideBoundaryCondition, PlantBranch,
        PlantBranchComponent, PlantBranchList, PlantLoop, Point3, RunPeriod, RunPeriodId,
        ScheduleCompact, ScheduleCompactSegment, ScheduleConstant, ScheduleId, SimulationModel,
        Surface, SurfaceId, SurfaceType, ThermostatControlObjectType, ThermostatDualSetpoint,
        ThermostatSetpointId, TimestepConfig, TypedModel, Zone, ZoneEquipmentConnection,
        ZoneEquipmentConnectionId, ZoneEquipmentList, ZoneEquipmentListEntry, ZoneEquipmentListId,
        ZoneEquipmentObjectType, ZoneId, ZoneThermostat, ZoneThermostatControl, ZoneThermostatId,
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
        assert_eq!(state.surfaces.len(), 6);
        assert_eq!(
            state.surfaces[0].outside_boundary_condition,
            OutsideBoundaryCondition::Outdoors
        );
        assert_eq!(state.surfaces[0].construction_name, "WALL");
        assert_eq!(state.surfaces[0].outside_layer_material_name, "R1");
        assert_eq!(state.surfaces[0].area_m2, 1.0);
        assert_eq!(state.surfaces[0].thermal_resistance_m2_k_per_w, 1.0);
        assert_eq!(state.surfaces[0].heat_capacity_j_per_m2_k, None);
        assert_eq!(state.surfaces[0].conductance_w_per_k, 1.0);
        assert_eq!(state.surfaces[0].heat_gain_to_zone_w, 0.0);
        assert_eq!(state.surfaces[0].inside_face_temperature_c, 20.0);
        assert_eq!(state.surfaces[0].outside_face_temperature_c, 20.0);

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
        assert_eq!(
            state.surfaces[0].inside_face_temperature_c,
            state.zones[0].mean_air_temperature_c
        );
        assert!(state.surfaces[0].heat_gain_to_zone_w < 0.0);

        Ok(())
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
        assert_eq!(simulation.results.series.len(), 14);

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
        assert_eq!(inside_surface_series.values[0], zone_series.values[0]);

        let Some(outside_surface_series) = simulation
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing outside surface series").into());
        };
        assert_eq!(outside_surface_series.values, vec![10.0, 12.0]);

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

    fn point(x_m: f64, y_m: f64, z_m: f64) -> Point3 {
        Point3 { x_m, y_m, z_m }
    }
}
