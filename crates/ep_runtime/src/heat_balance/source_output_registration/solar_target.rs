//! Stable Solar field identities and dispatch over borrowed live owner groups.
use super::{OrdinaryRadiationError, SourceRealOutputOwners, SourceRealOutputTarget};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceSolarOutputField {
    ZoneTransSolar,
    ZoneBmSolFrExtWinsRep,
    ZoneBmSolFrIntWinsRep,
    ZoneDifSolFrExtWinsRep,
    ZoneDifSolFrIntWinsRep,
    ZoneTransSolarEnergy,
    ZoneBmSolFrExtWinsRepEnergy,
    ZoneBmSolFrIntWinsRepEnergy,
    ZoneDifSolFrExtWinsRepEnergy,
    ZoneDifSolFrIntWinsRepEnergy,
    ZoneWinHeatGainRep,
    ZoneWinHeatLossRep,
    ZoneWinHeatGainRepEnergy,
    ZoneWinHeatLossRepEnergy,
    ZoneOpaqSurfInsFaceCond,
    ZoneOpaqSurfInsFaceCondGainRep,
    ZoneOpaqSurfInsFaceCondLossRep,
    ZoneOpaqSurfExtFaceCond,
    ZoneOpaqSurfExtFaceCondGainRep,
    ZoneOpaqSurfExtFaceCondLossRep,
    ZnOpqSurfInsFaceCondGnRepEnrg,
    ZnOpqSurfInsFaceCondLsRepEnrg,
    ZnOpqSurfExtFaceCondGnRepEnrg,
    ZnOpqSurfExtFaceCondLsRepEnrg,
    SurfaceAzimuth,
    SurfaceSunlitArea,
    SurfaceSunlitFrac,
    SurfQRadSWOutIncident,
    SurfQRadSWOutIncidentBeam,
    SurfQRadSWOutIncidentSkyDiffuse,
    SurfQRadSWOutIncidentGndDiffuse,
    SurfCosIncidenceAngle,
    SurfQRadSWOutIncSkyDiffReflGnd,
    SurfQRadSWOutIncSkyDiffReflObs,
    SurfQRadSWOutIncBmToBmReflObs,
    SurfQRadSWOutIncBmToDiffReflObs,
    SurfQRadSWOutIncBmToDiffReflGnd,
    SurfAnisoSkyMult,
    SurfWinBSDFBeamThetaRep,
    SurfWinBSDFBeamPhiRep,
    SurfBmIncInsSurfIntensRep,
    SurfBmIncInsSurfAmountRep,
    SurfIntBmIncInsSurfIntensRep,
    SurfIntBmIncInsSurfAmountRep,
    SurfInitialDifSolInAbsReport,
    SurfWinInitialDifSolInTransReport,
    SurfSWInAbsTotalReport,
    SurfBmIncInsSurfAmountRepEnergy,
    SurfIntBmIncInsSurfAmountRepEnergy,
    SurfCurDifShdgRatioIsoSky,
    SurfDifShdgRatioIsoSky,
    SurfDifShdgRatioHoriz,
    SurfWithShdgIsoSky,
    SurfWoShdgIsoSky,
}

/// The actual reporting consumer borrows both live groups; no value is retained.
#[allow(
    dead_code,
    reason = "Retain the combined live surface/solar resolver interface; narrower active caller views preserve the same borrowed owners without copying registered values."
)]
pub(crate) struct SourceCombinedOutputOwners<'a> {
    pub(crate) surface: &'a dyn SourceRealOutputOwners,
    pub(crate) solar: &'a dyn SourceRealOutputOwners,
}
impl SourceRealOutputOwners for SourceCombinedOutputOwners<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        match target {
            #[cfg(all(test, feature = "native-controlled-air-cohort"))]
            SourceRealOutputTarget::CohortReal { .. } => {
                Err(super::OrdinaryRadiationUnavailable::StorageDomain.into())
            }
            SourceRealOutputTarget::SurfaceHeatBalance { .. } => self.surface.real_output(target),
            SourceRealOutputTarget::Solar { .. } => self.solar.real_output(target),
            SourceRealOutputTarget::Air { .. } => {
                Err(super::OrdinaryRadiationUnavailable::StorageDomain.into())
            }
            SourceRealOutputTarget::InternalGain { .. }
            | SourceRealOutputTarget::PurchasedAir { .. }
            | SourceRealOutputTarget::Node { .. }
            | SourceRealOutputTarget::Weather { .. }
            | SourceRealOutputTarget::Schedule { .. }
            | SourceRealOutputTarget::ReportingSchedule { .. }
            | SourceRealOutputTarget::SystemReports { .. }
            | SourceRealOutputTarget::ExteriorLight { .. }
            | SourceRealOutputTarget::Comfort { .. }
            | SourceRealOutputTarget::Hvac { .. }
            | SourceRealOutputTarget::Oscillation { .. }
            | SourceRealOutputTarget::Pollution { .. }
            | SourceRealOutputTarget::Facility { .. } => {
                Err(super::OrdinaryRadiationUnavailable::StorageDomain.into())
            }
        }
    }
}
