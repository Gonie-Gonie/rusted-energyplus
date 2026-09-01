//! Fail-closed CP437 release errors.

use ep_model::IdealLoadsAirSystemId;

/// Errors returned before CP437 commits state or its private witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardError {
    UnknownSystem {
        system: IdealLoadsAirSystemId,
    },
    SystemIdentityMismatch {
        expected: IdealLoadsAirSystemId,
        actual: IdealLoadsAirSystemId,
    },
    SystemOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    InitializationNotReady {
        system: IdealLoadsAirSystemId,
    },
    HeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        predecessor_transition_count: usize,
        transition_count: usize,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
    ExactReleaseReductionViolated {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display for PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP437 heating outdoor-air maximum-flow first-warning guard release failed: {self:?}"
        )
    }
}

impl std::error::Error for PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningGuardError {}
