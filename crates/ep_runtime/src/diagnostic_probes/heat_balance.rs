//! Heat-balance diagnostic probe selectors.

use crate::heat_balance::{
    CompatibilityHeatBalanceAlgorithm, HeatBalanceZoneAirAlgorithm, HeatBalanceZoneAirSelection,
};

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
        }

        impl HeatBalanceZoneAirAlgorithm {
            /// Returns the diagnostic probe represented by this legacy selector.
            #[must_use]
            pub const fn diagnostic_probe(self) -> Option<DiagnosticHeatBalanceProbe> {
                match self {
                    $(Self::$variant => Some(DiagnosticHeatBalanceProbe::$variant),)+
                    Self::EnergyPlusHeatBalanceCompatCandidate => None,
                }
            }

            /// Returns the typed compatibility/diagnostic selection for this selector.
            #[must_use]
            pub const fn selection(self) -> HeatBalanceZoneAirSelection {
                match self {
                    Self::EnergyPlusHeatBalanceCompatCandidate => {
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
