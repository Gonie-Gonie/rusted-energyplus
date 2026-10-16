//! Stable semantic fields on the sole actual whole-building electric facility.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceFacilityOutputField {
    PurchasedElectricityRate,
    PurchasedElectricityEnergy,
    SurplusElectricityRate,
    SurplusElectricityEnergy,
    NetPurchasedElectricityRate,
    NetPurchasedElectricityEnergy,
    BuildingElectricityDemandRate,
    HvacElectricityDemandRate,
    ElectricityDemandRate,
    ProducedElectricityRate,
    ProducedElectricityEnergy,
}

/// Temporary actual other-owner view for a mutable facility callback. The own
/// facility is deliberately borrowed only inside each instantaneous read.
pub(crate) struct SourceNonFacilityOutputOwners<'a> {
    pub(crate) weather: &'a dyn super::SourceRealOutputOwners,
    pub(crate) schedules: &'a dyn super::SourceRealOutputOwners,
    pub(crate) system_reports: &'a dyn super::SourceRealOutputOwners,
    pub(crate) exterior: &'a dyn super::SourceRealOutputOwners,
    pub(crate) surface: &'a dyn super::SourceRealOutputOwners,
    pub(crate) solar: &'a dyn super::SourceRealOutputOwners,
    pub(crate) air: &'a dyn super::SourceRealOutputOwners,
    pub(crate) comfort: &'a dyn super::SourceRealOutputOwners,
    pub(crate) hvac: &'a dyn super::SourceRealOutputOwners,
    pub(crate) oscillation: &'a dyn super::SourceRealOutputOwners,
    pub(crate) pollution: &'a dyn super::SourceRealOutputOwners,
}
impl super::SourceRealOutputOwners for SourceNonFacilityOutputOwners<'_> {
    fn real_output(
        &self,
        target: super::SourceRealOutputTarget,
    ) -> Result<&f64, super::OrdinaryRadiationError> {
        use super::SourceRealOutputTarget as T;
        match target {
            T::Weather { .. } => self.weather.real_output(target),
            T::Schedule { .. } => self.schedules.real_output(target),
            T::SystemReports { .. } => self.system_reports.real_output(target),
            T::ExteriorLight { .. } => self.exterior.real_output(target),
            T::SurfaceHeatBalance { .. } => self.surface.real_output(target),
            T::Solar { .. } => self.solar.real_output(target),
            T::Air { .. } => self.air.real_output(target),
            T::Comfort { .. } => self.comfort.real_output(target),
            T::Hvac { .. } => self.hvac.real_output(target),
            T::Oscillation { .. } => self.oscillation.real_output(target),
            T::Pollution { .. } => self.pollution.real_output(target),
            T::Facility { .. } => Err(super::OrdinaryRadiationUnavailable::StorageDomain.into()),
        }
    }
}
