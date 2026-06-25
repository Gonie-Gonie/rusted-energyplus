//! Heat-balance compatibility and diagnostic selection APIs.

use crate::diagnostic_probes::DiagnosticHeatBalanceProbe;

/// Zone-air temperature algorithm used by heat-balance compatibility and diagnostic traces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceZoneAirAlgorithm {
    /// Existing simplified analytical diagnostic shell.
    SimplifiedAnalytical,
    /// Named official dynamic source-order compatibility lane.
    EnergyPlusHeatBalanceCompatCandidate,
    /// Concrete source-order execution variant for the official opaque 1Zone compatibility lane.
    EnergyPlusSourceOrder1ZoneOpaqueCompatibility,
    /// Experimental EnergyPlus analytical predictor path for diagnostics.
    EnergyPlusAnalyticalProbe,
    /// Experimental EnergyPlus analytical correction after the surface pass.
    EnergyPlusAnalyticalSurfaceFirstProbe,
    /// Experimental analytical correction with a same-timestep surface rebalance.
    EnergyPlusAnalyticalCoupledProbe,
    /// Experimental coupled analytical path using previous inside surface temperature for outdoor CTF boundary solves.
    EnergyPlusAnalyticalCoupledPreviousInsideProbe,
    /// Experimental previous-inside path using EnergyPlus DOE-2 exterior convection without quick outside conduction.
    EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe,
    /// Experimental previous-inside coupled path using EnergyPlus quick-conduction outside face solves.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe,
    /// Experimental quick-outside path that interleaves zone-air correction between surface passes.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe,
    /// Experimental interleaved quick-outside path with grey interior longwave exchange and EnergyPlus-style adiabatic CTF outside reporting order.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe,
    /// Experimental interleaved grey longwave path that freezes inside convection coefficients for the timestep.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe,
    /// Experimental frozen-hconv path that samples interior longwave from current pass temperatures.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe,
    /// Experimental frozen-hconv path that lets adiabatic outside CTF history follow the current inside solve.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe,
    /// Experimental interleaved grey longwave path with EnergyPlus third-order zone-air correction.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe,
    /// Experimental frozen-hconv path with EnergyPlus third-order zone-air correction.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe,
    /// Experimental frozen-hconv third-order path with EnergyPlus moist-air storage reporting.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe,
    /// Experimental weather-storage third-order path reporting surface convection against previous MAT.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe,
    /// Experimental weather-storage third-order path reporting surface convection from air-balance closure.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe,
    /// Experimental balance-surface-convection path with timestep-start reference air for surface solves.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe,
    /// Experimental frozen-reference-air path that samples interior longwave from current pass temperatures.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe,
    /// Experimental current-longwave path with EnergyPlus inside-surface convergence cutoff.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe,
    /// Experimental converged-surface path freezing outside-face balance snapshots through inside-surface iterations.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe,
    /// Experimental converged-surface path freezing only inside CTF outside-temperature history snapshots.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe,
    /// Experimental inside-CTF outside-history path also committing the frozen snapshot into CTF histories.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe,
    /// Experimental inside-CTF outside-history path with EnergyPlus ScriptF interior longwave exchange.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe,
    /// Experimental inside-CTF outside-history path with EnergyPlus ScriptF longwave using the EnergyPlus flat lSR access order.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe,
    /// Experimental ScriptF-flat path that keeps the surface reference air current during interleaved solves.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe,
    /// Experimental ScriptF-flat path that refreshes inside convection coefficients during interleaved solves.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe,
    /// Experimental ScriptF-flat path reporting surface convection from the surface reference-air snapshot.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe,
    /// Experimental ScriptF-flat path reporting inside convection from final surface temperatures.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe,
    /// Experimental ScriptF-flat path reporting conduction from the inside-CTF outside snapshot.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe,
    /// Experimental ScriptF-flat path syncing adiabatic outside faces for reports while preserving the pre-sync CTF history commit.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe,
    /// Experimental converged-surface path committing adiabatic CTF history from current inside face without mutating report state.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe,
    /// Experimental balance-surface-convection path syncing adiabatic outside history after the inside solve.
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe,
    /// Experimental interleaved quick-outside path with EnergyPlus ScriptF interior longwave exchange.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe,
    /// Experimental quick-outside path using EnergyPlus DOE-2 exterior convection.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe,
    /// Experimental quick-outside path with a grey interior longwave exchange probe.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe,
    /// Experimental quick-outside path combining DOE-2 exterior convection and grey interior longwave exchange.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe,
    /// Experimental quick-outside path with EnergyPlus ScriptF interior longwave exchange.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe,
    /// Experimental quick-outside path combining DOE-2 exterior convection and ScriptF interior longwave exchange.
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe,
    /// Experimental coupled analytical path using previous inside surface temperature for outdoor and adiabatic boundary solves.
    EnergyPlusAnalyticalCoupledPreviousBoundaryProbe,
    /// Experimental EnergyPlus third-order predictor path for diagnostics.
    EnergyPlusThirdOrderProbe,
}

/// Compatibility-mode heat-balance algorithms that may participate in claims.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompatibilityHeatBalanceAlgorithm {
    /// Source-order compatibility lane for the official opaque 1Zone heat-balance target.
    SourceOrder1ZoneOpaqueCompat,
}

impl CompatibilityHeatBalanceAlgorithm {
    /// Stable report identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::SourceOrder1ZoneOpaqueCompat => "source-order-1zone-opaque-compat",
        }
    }

    /// Legacy runtime selector used while the runtime is split into source-order modules.
    #[must_use]
    pub const fn zone_air_algorithm(self) -> HeatBalanceZoneAirAlgorithm {
        match self {
            Self::SourceOrder1ZoneOpaqueCompat => {
                HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
            }
        }
    }
}

/// Typed heat-balance runtime selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceZoneAirSelection {
    /// Compatibility-mode source-order algorithm.
    Compatibility(CompatibilityHeatBalanceAlgorithm),
    /// Diagnostic-only probe or baseline.
    Diagnostic(DiagnosticHeatBalanceProbe),
}

/// Classification for heat-balance zone-air algorithms in reports and gates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceAlgorithmLane {
    /// Source-order compatibility candidate lane.
    CompatibilitySourceOrder,
    /// Diagnostic-only baseline shell, not a probe and not conformance-safe.
    DiagnosticOnly,
    /// Diagnostic probe or experimental variant.
    DiagnosticProbe,
}

impl HeatBalanceAlgorithmLane {
    /// Stable report identifier.
    #[must_use]
    pub const fn id(self) -> &'static str {
        match self {
            Self::CompatibilitySourceOrder => "compatibility-source-order",
            Self::DiagnosticOnly => "diagnostic-only",
            Self::DiagnosticProbe => "diagnostic-probe",
        }
    }

    /// Returns whether the lane can be used for a conformance promotion.
    #[must_use]
    pub const fn allows_conformance_promotion(self) -> bool {
        matches!(self, Self::CompatibilitySourceOrder)
    }
}

impl HeatBalanceZoneAirAlgorithm {
    /// Returns the compatibility algorithm represented by this legacy selector.
    #[must_use]
    pub const fn compatibility_algorithm(self) -> Option<CompatibilityHeatBalanceAlgorithm> {
        match self {
            Self::EnergyPlusHeatBalanceCompatCandidate => {
                Some(CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat)
            }
            Self::EnergyPlusSourceOrder1ZoneOpaqueCompatibility => {
                Some(CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat)
            }
            _ => None,
        }
    }

    /// Returns the source-order/diagnostic lane for this algorithm.
    #[must_use]
    pub const fn lane(self) -> HeatBalanceAlgorithmLane {
        match self {
            Self::EnergyPlusHeatBalanceCompatCandidate
            | Self::EnergyPlusSourceOrder1ZoneOpaqueCompatibility => {
                HeatBalanceAlgorithmLane::CompatibilitySourceOrder
            }
            Self::SimplifiedAnalytical => HeatBalanceAlgorithmLane::DiagnosticOnly,
            _ => HeatBalanceAlgorithmLane::DiagnosticProbe,
        }
    }

    /// Returns whether the algorithm represents a source-order compatibility lane.
    #[must_use]
    pub const fn is_compatibility_source_order(self) -> bool {
        matches!(
            self.lane(),
            HeatBalanceAlgorithmLane::CompatibilitySourceOrder
        )
    }

    /// Returns whether the algorithm is a diagnostic probe or diagnostic-only shell.
    #[must_use]
    pub const fn is_diagnostic_lane(self) -> bool {
        !self.is_compatibility_source_order()
    }

    /// Returns whether this algorithm is allowed for a conformance promotion.
    #[must_use]
    pub const fn allows_conformance_promotion(self) -> bool {
        self.lane().allows_conformance_promotion()
    }
}

/// Returns the base feature family for selectors that add report-only diagnostic toggles.
#[must_use]
pub(crate) fn heat_balance_zone_air_algorithm_feature_base(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> HeatBalanceZoneAirAlgorithm {
    let zone_air_algorithm = heat_balance_zone_air_algorithm_execution_variant(zone_air_algorithm);
    match zone_air_algorithm {
        HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe => {
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
        }
        _ => zone_air_algorithm,
    }
}

/// Returns the concrete runtime variant for compatibility selectors.
#[must_use]
pub(crate) fn heat_balance_zone_air_algorithm_execution_variant(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> HeatBalanceZoneAirAlgorithm {
    match zone_air_algorithm {
        HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate => {
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility => {
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
        }
        _ => zone_air_algorithm,
    }
}

pub(crate) fn heat_balance_uses_third_order_zone_air_correction(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    let zone_air_algorithm = heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    )
}

pub(crate) fn heat_balance_preserves_surface_inside_temperature_for_first_longwave(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    let zone_air_algorithm = heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HeatBalanceTimestepAlgorithmFlags {
    pub(crate) correct_zone_air_after_surface_pass: bool,
    pub(crate) rebalance_surfaces_after_zone_air_correction: bool,
    pub(crate) interleave_zone_air_surface_passes: bool,
    pub(crate) use_previous_inside_for_outdoor_boundary: bool,
    pub(crate) use_previous_inside_for_adiabatic_boundary: bool,
    pub(crate) use_quick_outside_conduction: bool,
}

pub(crate) fn heat_balance_timestep_algorithm_flags(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> HeatBalanceTimestepAlgorithmFlags {
    let feature_zone_air_algorithm =
        heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);

    HeatBalanceTimestepAlgorithmFlags {
        correct_zone_air_after_surface_pass: matches!(
            feature_zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
        ),
        rebalance_surfaces_after_zone_air_correction: matches!(
            feature_zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
        ),
        interleave_zone_air_surface_passes: matches!(
            feature_zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
        ),
        use_previous_inside_for_outdoor_boundary: matches!(
            feature_zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
        ),
        use_previous_inside_for_adiabatic_boundary: matches!(
            feature_zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
        ),
        use_quick_outside_conduction: matches!(
            feature_zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
                | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
        ),
    }
}
pub(crate) fn heat_balance_uses_weather_air_storage_report(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    let zone_air_algorithm = heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    )
}

pub(crate) fn heat_balance_uses_previous_mat_surface_convection_report(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    let zone_air_algorithm = heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
    )
}

pub(crate) fn heat_balance_uses_balance_surface_convection_report(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    )
}

pub(crate) fn heat_balance_uses_surface_reference_air_convection_report(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
    )
}

pub(crate) fn heat_balance_uses_surface_reference_air_surface_convection_report(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
    )
}

pub(crate) fn heat_balance_uses_final_inside_convection_report(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
    )
}
#[cfg(test)]
mod tests {
    use super::{
        CompatibilityHeatBalanceAlgorithm, DiagnosticHeatBalanceProbe, HeatBalanceAlgorithmLane,
        HeatBalanceZoneAirAlgorithm, HeatBalanceZoneAirSelection,
    };

    #[test]
    fn compatibility_algorithm_round_trips_to_legacy_selector() {
        let compatibility = CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat;

        assert_eq!(compatibility.id(), "source-order-1zone-opaque-compat");
        assert_eq!(
            compatibility.zone_air_algorithm(),
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
        );
        assert_eq!(
            compatibility.zone_air_algorithm().compatibility_algorithm(),
            Some(compatibility)
        );
        assert_eq!(compatibility.zone_air_algorithm().diagnostic_probe(), None);
        assert_eq!(
            compatibility.zone_air_algorithm().selection(),
            HeatBalanceZoneAirSelection::Compatibility(compatibility)
        );
        assert_eq!(
            compatibility.zone_air_algorithm().lane(),
            HeatBalanceAlgorithmLane::CompatibilitySourceOrder
        );
        assert_eq!(
            super::heat_balance_zone_air_algorithm_execution_variant(
                compatibility.zone_air_algorithm()
            ),
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
        );
        assert_eq!(
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility.selection(),
            HeatBalanceZoneAirSelection::Compatibility(compatibility)
        );
        assert_eq!(
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
                .diagnostic_probe(),
            None
        );
    }

    #[test]
    fn diagnostic_probe_round_trips_to_legacy_selector() {
        let probe = DiagnosticHeatBalanceProbe::EnergyPlusThirdOrderProbe;

        assert_eq!(
            probe.zone_air_algorithm(),
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe
        );
        assert_eq!(probe.zone_air_algorithm().diagnostic_probe(), Some(probe));
        assert_eq!(probe.zone_air_algorithm().compatibility_algorithm(), None);
        assert_eq!(
            probe.zone_air_algorithm().selection(),
            HeatBalanceZoneAirSelection::Diagnostic(probe)
        );
        assert_eq!(
            probe.zone_air_algorithm().lane(),
            HeatBalanceAlgorithmLane::DiagnosticProbe
        );
    }

    #[test]
    fn simplified_analytical_remains_diagnostic_only_baseline() {
        assert_eq!(
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical.diagnostic_probe(),
            Some(DiagnosticHeatBalanceProbe::SimplifiedAnalytical)
        );
        assert_eq!(
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical.lane(),
            HeatBalanceAlgorithmLane::DiagnosticOnly
        );
        assert!(!HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical.allows_conformance_promotion());
    }
}
