//! Stable actual field identities; all values are borrowed from live owners.
use super::{OrdinaryRadiationError, SourceIntegerOutputOwners, SourceIntegerOutputTarget};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceHvacRealOutputField {
    SystemReliefHeatLoss,
    SystemRejectHeatLoss,
}
pub(crate) struct SourceAllIntegerOutputOwners<'a> {
    pub(crate) equipment: &'a dyn SourceIntegerOutputOwners,
    pub(crate) weather: &'a dyn SourceIntegerOutputOwners,
    pub(crate) air: &'a dyn SourceIntegerOutputOwners,
    pub(crate) surface: &'a dyn SourceIntegerOutputOwners,
    pub(crate) hvac: &'a dyn SourceIntegerOutputOwners,
}
impl SourceIntegerOutputOwners for SourceAllIntegerOutputOwners<'_> {
    fn integer_output(
        &self,
        target: SourceIntegerOutputTarget,
    ) -> Result<&i32, OrdinaryRadiationError> {
        match target {
            #[cfg(all(test, feature = "native-controlled-air-cohort"))]
            SourceIntegerOutputTarget::CohortInteger { .. } => Err(super::OrdinaryRadiationUnavailable::StorageDomain.into()),
            SourceIntegerOutputTarget::PurchasedAirAvailabilityStatus { .. } => self.equipment.integer_output(target),
            SourceIntegerOutputTarget::Weather { .. } => self.weather.integer_output(target),
            SourceIntegerOutputTarget::ZoneThermostatControlType { .. } => {
                self.air.integer_output(target)
            }
            SourceIntegerOutputTarget::SurfaceInsideIterationCount => {
                self.surface.integer_output(target)
            }
            SourceIntegerOutputTarget::HvacSystemSolverIterationCount
            | SourceIntegerOutputTarget::AirSystemSolverIterationCount
            | SourceIntegerOutputTarget::PlantSubIterationCount
            | SourceIntegerOutputTarget::PlantHalfLoopCallCount
            | SourceIntegerOutputTarget::AirLoopMaximumIterations
            | SourceIntegerOutputTarget::AirLoopTotalIterations
            | SourceIntegerOutputTarget::AirLoopTotalCalls => self.hvac.integer_output(target),
        }
    }
}
