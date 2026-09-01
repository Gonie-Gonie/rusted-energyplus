//! Fail-closed CP436 release errors.

use ep_model::IdealLoadsAirSystemId;

/// Errors returned before CP436 commits state or its private witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentError {
    UnknownSystem { system: IdealLoadsAirSystemId },
    SystemIdentityMismatch {
        expected: IdealLoadsAirSystemId,
        actual: IdealLoadsAirSystemId,
    },
    SystemOutsideDirectSubset { system: IdealLoadsAirSystemId },
    InitializationNotReady { system: IdealLoadsAirSystemId },
    HeatingOutdoorAirMaximumFlowGuardSnapshotMismatch { system: IdealLoadsAirSystemId },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
        init_call_count: usize,
        calculation_entry_call_count: usize,
        predecessor_transition_count: usize,
        transition_count: usize,
    },
    RuntimeStateInvariantViolation { system: IdealLoadsAirSystemId },
    StandardAirDensityUnavailableOrInconsistent { system: IdealLoadsAirSystemId },
    ExactReleaseReductionViolated { system: IdealLoadsAirSystemId },
}

impl std::fmt::Display
    for PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP436 heating outdoor-air volume-flow assignment release failed: {self:?}"
        )
    }
}

impl std::error::Error
    for PurchasedAirCalcHeatingOutdoorAirMaximumFlowBodyVolumeFlowAssignmentError
{
}
