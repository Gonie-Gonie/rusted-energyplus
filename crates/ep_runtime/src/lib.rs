//! Runtime state and execution-plan shells.

use ep_model::{OutputHandle, ZoneId};

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
    /// Solve one zone.
    SolveZone(ZoneId),
    /// Write one output handle.
    WriteOutput(OutputHandle),
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

#[cfg(test)]
mod tests {
    use super::{SimulationMode, SimulationState};

    #[test]
    fn state_defaults_to_first_timestep() {
        let state = SimulationState::new(SimulationMode::Compatibility);

        assert_eq!(state.timestep_index, 0);
        assert_eq!(state.mode, SimulationMode::Compatibility);
    }
}
