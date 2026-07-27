//! Persistent `InitPurchasedAir` lifecycle state.

mod manager_plan;
mod state;
mod summary;
mod topology_plan;
mod topology_transition;
mod transition;

pub use manager_plan::*;
pub use state::*;
pub use summary::*;
pub use topology_plan::*;
pub use transition::*;

#[cfg(test)]
mod lifecycle_tests;
#[cfg(test)]
mod manager_plan_tests;
#[cfg(test)]
mod manager_scan_tests;
#[cfg(test)]
mod topology_plan_tests;
#[cfg(test)]
mod topology_transition_tests;
#[cfg(test)]
mod warning_tests;

/// Stable provenance for the Rust-owned persistent initialization lifecycle.
pub const PURCHASED_AIR_INIT_LIFECYCLE_SOURCE: &str = "rust-persistent-init-purchased-air";

/// One-time flags mirrored from `InitPurchasedAir` source concepts.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IdealLoadsInitFlags {
    /// Whether these values came from the persistent lifecycle state machine.
    pub state_machine_used: bool,
    /// One-time input and object lookup checks have run.
    pub one_time_checked: bool,
    /// Selected-unit topology reached a release-usable recirculation result.
    pub topology_ready: bool,
    /// Begin-environment initialization has run.
    pub environment_initialized: bool,
    /// Source `MyEnvrnFlag`; a later begin-environment write is pending.
    pub environment_initialization_needed: bool,
    /// Sizing branch has been checked.
    pub sizing_checked: bool,
    /// Zone equipment list membership has been checked.
    pub equipment_list_checked: bool,
    /// Return plenum branch is inactive for the first subset.
    pub return_plenum_inactive: bool,
}

impl IdealLoadsInitFlags {
    /// Returns assumed-ready flags used only by legacy diagnostic adapters.
    ///
    /// Release compatibility must use [`init_purchased_air_runtime`] and may
    /// not promote this descriptive snapshot.
    #[must_use]
    pub const fn diagnostic_adapter_assumed_ready() -> Self {
        Self {
            state_machine_used: false,
            one_time_checked: true,
            topology_ready: true,
            environment_initialized: true,
            environment_initialization_needed: false,
            sizing_checked: true,
            equipment_list_checked: true,
            return_plenum_inactive: true,
        }
    }

    /// Returns the initialized flag set for the no-OA/no-limit candidate.
    #[must_use]
    pub const fn no_oa_no_limit_candidate() -> Self {
        Self::diagnostic_adapter_assumed_ready()
    }
}
