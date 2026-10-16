//! Genuine simulation caller orchestration, requiring same retained Source children.
pub(crate) mod prelude;
pub(crate) mod primary;
mod primary_host;
mod prelude_host;
pub(crate) mod setup;
mod setup_host;
mod through_primary;
mod endrun_prefix;

/// Actual ManageSimulation180 locals retained across setup and the primary loop.
///
/// This owner is constructed at the simulation prologue, before output options183.
/// Its two fields stay outside the report owner borrowed by the concrete host.
#[derive(Debug)]
pub struct SourceSimulationLoopLocals {
    one_time_underwater_boundary_check: bool,
    any_underwater_boundaries: bool,
}

impl SourceSimulationLoopLocals {
    /// Executes the actual local initializers at SimulationManager180-181.
    pub fn at_simulation_prologue() -> Self {
        Self {
            one_time_underwater_boundary_check: true,
            any_underwater_boundaries: false,
        }
    }

    /// Reborrows the original two cells for the literal primary-loop parent.
    pub(crate) fn underwater_borrows(&mut self) -> (&mut bool, &mut bool) {
        (
            &mut self.one_time_underwater_boundary_check,
            &mut self.any_underwater_boundaries,
        )
    }
}
