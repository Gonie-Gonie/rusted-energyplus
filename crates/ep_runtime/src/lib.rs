//! Runtime state, execution-plan shells, and first trace helpers.

use ep_model::{OutputHandle, ScheduleId, SimulationModel, TypedModel, ZoneId};
use std::fmt::{Display, Formatter};
use std::path::Path;

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
    /// Evaluate one constant schedule.
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
    setup_steps.extend(
        model
            .typed
            .schedules
            .iter()
            .map(|schedule| ExecutionStep::EvaluateSchedule(schedule.id)),
    );

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

/// Minimal explicit simulation state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimulationState {
    /// Selected mode.
    pub mode: SimulationMode,
    /// Current zero-based timestep index.
    pub timestep_index: u64,
}

impl SimulationState {
    /// Creates a new explicit simulation state.
    #[must_use]
    pub const fn new(mode: SimulationMode) -> Self {
        Self {
            mode,
            timestep_index: 0,
        }
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

/// Error returned while reading EPW weather data.
#[derive(Debug)]
pub enum EpwError {
    /// File read failed.
    Io(std::io::Error),
    /// EPW data row was missing the dry-bulb column.
    MissingDryBulb {
        /// One-based line number.
        line: usize,
    },
    /// EPW dry-bulb value could not be parsed.
    InvalidDryBulb {
        /// One-based line number.
        line: usize,
        /// Raw field text.
        value: String,
    },
}

impl Display for EpwError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(error) => write!(formatter, "failed to read EPW: {error}"),
            Self::MissingDryBulb { line } => {
                write!(
                    formatter,
                    "EPW row at line {line} is missing dry-bulb value"
                )
            }
            Self::InvalidDryBulb { line, value } => {
                write!(
                    formatter,
                    "EPW row at line {line} has invalid dry-bulb value '{value}'"
                )
            }
        }
    }
}

impl std::error::Error for EpwError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::MissingDryBulb { .. } | Self::InvalidDryBulb { .. } => None,
        }
    }
}

impl From<std::io::Error> for EpwError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

/// Loads hourly outdoor dry-bulb values from an EPW file.
pub fn load_epw_dry_bulb_series(path: impl AsRef<Path>) -> Result<Vec<f64>, EpwError> {
    let contents = std::fs::read_to_string(path)?;
    parse_epw_dry_bulb_series(&contents)
}

/// Parses hourly outdoor dry-bulb values from EPW text.
pub fn parse_epw_dry_bulb_series(contents: &str) -> Result<Vec<f64>, EpwError> {
    let mut values = Vec::new();

    for (line_index, line) in contents.lines().enumerate().skip(8) {
        let line_number = line_index + 1;
        if line.trim().is_empty() {
            continue;
        }
        let mut fields = line.split(',');
        let dry_bulb = fields
            .nth(6)
            .ok_or(EpwError::MissingDryBulb { line: line_number })?;
        let value = dry_bulb
            .trim()
            .parse::<f64>()
            .map_err(|_error| EpwError::InvalidDryBulb {
                line: line_number,
                value: dry_bulb.to_string(),
            })?;
        values.push(value);
    }

    Ok(values)
}

#[cfg(test)]
mod tests {
    use super::{
        ExecutionStep, SimulationMode, SimulationState, build_execution_plan,
        parse_epw_dry_bulb_series, simulate_constant_schedules,
    };
    use ep_model::{
        NormalizedName, ScheduleConstant, ScheduleId, SimulationModel, TypedModel, Zone, ZoneId,
    };

    #[test]
    fn state_defaults_to_first_timestep() {
        let state = SimulationState::new(SimulationMode::Compatibility);

        assert_eq!(state.timestep_index, 0);
        assert_eq!(state.mode, SimulationMode::Compatibility);
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
1999,1,1,1,0,Source,-3.0,-4.0
1999,1,1,2,0,Source,-2.0,-3.0
"#,
        )?;

        assert_eq!(values, vec![-3.0, -2.0]);

        Ok(())
    }
}
