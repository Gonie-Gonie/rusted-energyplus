//! IdealLoads initialization state for compatibility mode.

/// One-time flags mirrored from `InitPurchasedAir` source concepts.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct IdealLoadsInitFlags {
    /// One-time input and object lookup checks have run.
    pub one_time_checked: bool,
    /// Begin-environment initialization has run.
    pub environment_initialized: bool,
    /// Sizing branch has been checked.
    pub sizing_checked: bool,
    /// Zone equipment list membership has been checked.
    pub equipment_list_checked: bool,
    /// Return plenum branch is inactive for the first subset.
    pub return_plenum_inactive: bool,
}

impl IdealLoadsInitFlags {
    /// Returns the initialized flag set for the no-OA/no-limit candidate.
    #[must_use]
    pub const fn no_oa_no_limit_candidate() -> Self {
        Self {
            one_time_checked: true,
            environment_initialized: true,
            sizing_checked: true,
            equipment_list_checked: true,
            return_plenum_inactive: true,
        }
    }
}
