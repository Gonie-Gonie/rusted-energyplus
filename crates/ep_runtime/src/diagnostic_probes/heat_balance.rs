//! Heat-balance diagnostic probe selectors.

use crate::heat_balance::{
    CompatibilityHeatBalanceAlgorithm, HeatBalanceZoneAirAlgorithm, HeatBalanceZoneAirSelection,
};

/// Structured metadata for a diagnostic-only heat-balance probe.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticProbeMetadata {
    /// Stable probe name used in reports and audits.
    pub name: &'static str,
    /// Short purpose statement for the probe.
    pub purpose: &'static str,
    /// Expected bottleneck family this probe should isolate.
    pub expected_bottleneck: &'static str,
    /// Why the probe is kept in the runtime.
    pub why_it_exists: &'static str,
    /// The mismatch family this probe investigates.
    pub mismatch_investigated: &'static str,
}

macro_rules! diagnostic_heat_balance_probes {
    ($($variant:ident),+ $(,)?) => {
        /// Diagnostic-only heat-balance probes and non-claim baselines.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[allow(missing_docs)]
        pub enum DiagnosticHeatBalanceProbe {
            $($variant,)+
        }

        impl DiagnosticHeatBalanceProbe {
            /// Legacy runtime selector used while diagnostic probes are split out.
            #[must_use]
            pub const fn zone_air_algorithm(self) -> HeatBalanceZoneAirAlgorithm {
                match self {
                    $(Self::$variant => HeatBalanceZoneAirAlgorithm::$variant,)+
                }
            }

            /// Returns non-claim metadata explaining the probe boundary.
            #[must_use]
            pub const fn metadata(self) -> DiagnosticProbeMetadata {
                match self {
                    $(
                        Self::$variant => DiagnosticProbeMetadata {
                            name: stringify!($variant),
                            purpose: "Isolate one heat-balance source-order hypothesis without expanding conformance claims.",
                            expected_bottleneck: "A timing, state-history, surface-balance, zone-air, or report-row delta that must be promoted through a compatibility lane before conformance.",
                            why_it_exists: "Diagnostic-only selector for isolating heat-balance source-order deltas before compatibility promotion.",
                            mismatch_investigated: "EnergyPlus/Rust heat-balance timing, state-history, or report-row mismatch; not conformance evidence.",
                        },
                    )+
                }
            }
        }

        impl HeatBalanceZoneAirAlgorithm {
            /// Returns the diagnostic probe represented by this legacy selector.
            #[must_use]
            pub const fn diagnostic_probe(self) -> Option<DiagnosticHeatBalanceProbe> {
                match self {
                    $(Self::$variant => Some(DiagnosticHeatBalanceProbe::$variant),)+
                    Self::EnergyPlusHeatBalanceCompatCandidate
                    | Self::EnergyPlusSourceOrder1ZoneOpaqueCompatibility => None,
                }
            }

            /// Returns the typed compatibility/diagnostic selection for this selector.
            #[must_use]
            pub const fn selection(self) -> HeatBalanceZoneAirSelection {
                match self {
                    Self::EnergyPlusHeatBalanceCompatCandidate
                    | Self::EnergyPlusSourceOrder1ZoneOpaqueCompatibility => {
                        HeatBalanceZoneAirSelection::Compatibility(
                            CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat,
                        )
                    }
                    $(
                        Self::$variant => HeatBalanceZoneAirSelection::Diagnostic(
                            DiagnosticHeatBalanceProbe::$variant,
                        ),
                    )+
                }
            }
        }
    };
}

diagnostic_heat_balance_probes! {
    SimplifiedAnalytical,
    EnergyPlusAnalyticalProbe,
    EnergyPlusAnalyticalSurfaceFirstProbe,
    EnergyPlusAnalyticalCoupledProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe,
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe,
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe,
    EnergyPlusAnalyticalCoupledPreviousBoundaryProbe,
    EnergyPlusThirdOrderProbe,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diagnostic_probe_metadata_declares_non_claim_boundary() {
        let metadata = DiagnosticHeatBalanceProbe::EnergyPlusThirdOrderProbe.metadata();

        assert_eq!(metadata.name, "EnergyPlusThirdOrderProbe");
        assert!(metadata.purpose.contains("without expanding conformance"));
        assert!(metadata.expected_bottleneck.contains("delta"));
        assert!(metadata.why_it_exists.contains("Diagnostic-only"));
        assert!(metadata.mismatch_investigated.contains("mismatch"));
    }
}
