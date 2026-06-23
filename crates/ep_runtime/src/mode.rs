//! Runtime execution mode definitions.

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
