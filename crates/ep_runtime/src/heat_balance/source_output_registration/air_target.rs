//! Typed Air output identities, never a cached numeric snapshot.
use super::{OrdinaryRadiationError, SourceRealOutputOwners, SourceRealOutputTarget};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceAirOutputField {
    AirTemperature,
    AirHumidityRatio,
    AirRelativeHumidity,
    MeanRadiantTemperature,
    ZoneListSensibleHeatingEnergy,
    ZoneListSensibleCoolingEnergy,
    ZoneListSensibleHeatingRate,
    ZoneListSensibleCoolingRate,
    SensibleHeatingEnergy,
    SensibleCoolingEnergy,
    SensibleHeatingRate,
    SensibleCoolingRate,
    PredictedSensibleRate,
    PredictedHeatingSetpointRate,
    PredictedCoolingSetpointRate,
    SystemPredictedSensibleRate,
    SystemPredictedHeatingSetpointRate,
    SystemPredictedCoolingSetpointRate,
    PredictedMoistureRate,
    PredictedHumidifyingSetpointRate,
    PredictedDehumidifyingSetpointRate,
    SystemPredictedMoistureRate,
    SystemPredictedHumidifyingSetpointRate,
    SystemPredictedDehumidifyingSetpointRate,
    ThermostatAirTemperature,
    ThermostatHeatingSetpoint,
    ThermostatCoolingSetpoint,
    AdaptiveComfortCoolingSetpoint,
    LoadCorrectionFactor,
    // DataHeatBalance205–300 and HeatBalanceAirManager297–382 live report fields.
    MeanAirTemp,
    WetbulbGlobeTemp,
    OperativeTemp,
    MeanAirDewPointTemp,
    MeanAirHumRat,
    SumIntGains,
    SumHADTsurfs,
    SumMCpDTzones,
    SumMCpDtInfil,
    SumMCpDTsystem,
    SumNonAirSystem,
    CzdTdt,
    ImBalance,
    SumEnthalpyM,
    SumEnthalpyH,
    ExfilTotalLoss,
    ExfilSensiLoss,
    ExfilLatentLoss,
    ExhTotalLoss,
    ExhSensiLoss,
    ExhLatentLoss,
    ZoneTotalExfiltrationHeatLoss,
    ZoneTotalExhaustHeatLoss,
    ZoneHeatIndex,
    ZoneHumidex,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceComfortOutputField {
    PeopleFangerPmv,
    PeopleFangerPpd,
    PeopleAnkleDraftPpd,
    PeopleClothingSurfaceTemperature,
    PeopleMeanRadiantTemperature,
    PeopleOperativeTemperature,
    PeopleClothingValue,
    ZoneSimpleSummer,
    ZoneSimpleWinter,
    ZoneSimpleEither,
    FacilitySimpleSummer,
    FacilitySimpleWinter,
    FacilitySimpleEither,
    ZoneNotMetHeating,
    ZoneNotMetHeatingOccupied,
    ZoneNotMetCooling,
    ZoneNotMetCoolingOccupied,
    FacilityNotMetHeating,
    FacilityNotMetCooling,
    FacilityNotMetHeatingOccupied,
    FacilityNotMetCoolingOccupied,
}
/// Reporting borrows each actual owner group on every read.
#[allow(
    dead_code,
    reason = "Retain the all-owner borrowed output resolver interface; live production may choose narrower owner views without replacing registered Which targets with copied values."
)]
pub(crate) struct SourceAllOutputOwners<'a> {
    pub(crate) weather: &'a dyn SourceRealOutputOwners,
    pub(crate) schedules: &'a dyn SourceRealOutputOwners,
    pub(crate) system_reports: &'a dyn SourceRealOutputOwners,
    pub(crate) exterior: &'a dyn SourceRealOutputOwners,
    pub(crate) facility: &'a dyn SourceRealOutputOwners,
    pub(crate) surface: &'a dyn SourceRealOutputOwners,
    pub(crate) solar: &'a dyn SourceRealOutputOwners,
    pub(crate) air: &'a dyn SourceRealOutputOwners,
    pub(crate) comfort: &'a dyn SourceRealOutputOwners,
    pub(crate) hvac: &'a dyn SourceRealOutputOwners,
    pub(crate) oscillation: &'a dyn SourceRealOutputOwners,
    pub(crate) pollution: &'a dyn SourceRealOutputOwners,
}
impl SourceRealOutputOwners for SourceAllOutputOwners<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        match target {
            #[cfg(all(test, feature = "native-controlled-air-cohort"))]
            SourceRealOutputTarget::CohortReal { .. } => {
                Err(super::OrdinaryRadiationUnavailable::StorageDomain.into())
            }
            SourceRealOutputTarget::InternalGain { .. }
            | SourceRealOutputTarget::Node { .. }
            | SourceRealOutputTarget::PurchasedAir { .. } => {
                Err(super::OrdinaryRadiationUnavailable::StorageDomain.into())
            }
            SourceRealOutputTarget::Weather { .. } => self.weather.real_output(target),
            SourceRealOutputTarget::Schedule { .. }
            | SourceRealOutputTarget::ReportingSchedule { .. } => {
                self.schedules.real_output(target)
            }
            SourceRealOutputTarget::SystemReports { .. } => self.system_reports.real_output(target),
            SourceRealOutputTarget::ExteriorLight { .. } => self.exterior.real_output(target),
            SourceRealOutputTarget::Facility { .. } => self.facility.real_output(target),
            SourceRealOutputTarget::SurfaceHeatBalance { .. } => self.surface.real_output(target),
            SourceRealOutputTarget::Solar { .. } => self.solar.real_output(target),
            SourceRealOutputTarget::Air { .. } => self.air.real_output(target),
            SourceRealOutputTarget::Comfort { .. } => self.comfort.real_output(target),
            SourceRealOutputTarget::Hvac { .. } => self.hvac.real_output(target),
            SourceRealOutputTarget::Oscillation { .. } => self.oscillation.real_output(target),
            SourceRealOutputTarget::Pollution { .. } => self.pollution.real_output(target),
        }
    }
}
