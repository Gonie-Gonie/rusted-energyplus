//! Fail-closed CP438 release errors.

use ep_model::IdealLoadsAirSystemId;

/// Errors returned before CP438 commits state or its private witness.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementError {
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
    HeatingOutdoorAirMaximumFlowFirstWarningGuardSnapshotMismatch {
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

impl std::fmt::Display
    for PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementError
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP438 heating outdoor-air maximum-flow first-warning counter increment release failed: {self:?}"
        )
    }
}

impl std::error::Error
    for PurchasedAirCalcHeatingOutdoorAirMaximumFlowFirstWarningCounterIncrementError
{
}
