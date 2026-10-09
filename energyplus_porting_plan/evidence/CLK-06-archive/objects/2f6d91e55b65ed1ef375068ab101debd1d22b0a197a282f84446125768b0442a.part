//! Heat-balance diagnostic selectors and their runtime-config conversion boundary.

use crate::heat_balance::algorithm::{
    CompatibilityHeatBalanceAlgorithm, HeatBalanceAlgorithmLane, HeatBalanceInteriorLongwaveMode,
    HeatBalanceRuntimeConfig, HeatBalanceTimestepAlgorithmFlags, HeatBalanceZoneAirUpdate,
};

/// Structured metadata for a diagnostic-only heat-balance selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DiagnosticProbeMetadata {
    /// Stable selector name used in reports and audits.
    pub name: &'static str,
    /// Short purpose statement for the selector.
    pub purpose: &'static str,
    /// Expected bottleneck family this selector should isolate.
    pub expected_bottleneck: &'static str,
    /// Why the selector is kept in the runtime.
    pub why_it_exists: &'static str,
    /// The mismatch family this selector investigates.
    pub mismatch_investigated: &'static str,
}

/// Typed heat-balance runtime selection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceZoneAirSelection {
    /// Compatibility-mode source-order algorithm.
    Compatibility(CompatibilityHeatBalanceAlgorithm),
    /// Diagnostic-only selector or baseline.
    Diagnostic(DiagnosticHeatBalanceProbe),
}

macro_rules! diagnostic_heat_balance_selectors {
    ($($variant:ident => $cli_name:literal),+ $(,)?) => {
        /// Legacy zone-air selector retained at the diagnostic and CLI boundary.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[allow(missing_docs)]
        pub enum HeatBalanceZoneAirAlgorithm {
            EnergyPlusHeatBalanceCompatCandidate,
            EnergyPlusSourceOrder1ZoneOpaqueCompatibility,
            $($variant,)+
        }

        /// Diagnostic-only heat-balance selectors and non-claim baselines.
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        #[allow(missing_docs)]
        pub enum DiagnosticHeatBalanceProbe {
            $($variant,)+
        }

        impl DiagnosticHeatBalanceProbe {
            /// Every diagnostic selector, used by boundary and regression checks.
            pub const ALL: &'static [Self] = &[$(Self::$variant,)+];

            /// Legacy runtime selector used at the diagnostic boundary.
            #[must_use]
            pub const fn zone_air_algorithm(self) -> HeatBalanceZoneAirAlgorithm {
                match self {
                    $(Self::$variant => HeatBalanceZoneAirAlgorithm::$variant,)+
                }
            }

            /// Explicit runtime choices represented by this selector.
            #[must_use]
            #[cfg(test)]
            pub(crate) const fn runtime_config(self) -> HeatBalanceRuntimeConfig {
                diagnostic_runtime_config(self.zone_air_algorithm())
            }

            /// Returns non-claim metadata explaining the selector boundary.
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
            /// Every legacy selector, used by exhaustive boundary checks.
            pub const ALL: &'static [Self] = &[
                Self::EnergyPlusHeatBalanceCompatCandidate,
                Self::EnergyPlusSourceOrder1ZoneOpaqueCompatibility,
                $(Self::$variant,)+
            ];

            /// Stable CLI and report name for this selector.
            #[must_use]
            pub const fn cli_name(self) -> &'static str {
                match self {
                    Self::EnergyPlusHeatBalanceCompatCandidate =>
                        "energyplus-heat-balance-compat-candidate",
                    Self::EnergyPlusSourceOrder1ZoneOpaqueCompatibility =>
                        "energyplus-source-order-1zone-opaque-compatibility",
                    $(Self::$variant => $cli_name,)+
                }
            }

            /// Parses a CLI name without exposing selector matching to the CLI crate.
            #[must_use]
            pub fn from_cli_name(value: &str) -> Option<Self> {
                let normalized = value.trim().to_ascii_lowercase();
                match normalized.as_str() {
                    "" => Some(Self::SimplifiedAnalytical),
                    "energyplus-heat-balance-compat-candidate" =>
                        Some(Self::EnergyPlusHeatBalanceCompatCandidate),
                    "energyplus-source-order-1zone-opaque-compatibility" =>
                        Some(Self::EnergyPlusSourceOrder1ZoneOpaqueCompatibility),
                    $($cli_name => Some(Self::$variant),)+
                    _ => None,
                }
            }

            /// Returns the compatibility algorithm represented by this selector.
            #[must_use]
            pub const fn compatibility_algorithm(self) -> Option<CompatibilityHeatBalanceAlgorithm> {
                match self {
                    Self::EnergyPlusHeatBalanceCompatCandidate
                    | Self::EnergyPlusSourceOrder1ZoneOpaqueCompatibility => {
                        Some(CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat)
                    }
                    $(Self::$variant => None,)+
                }
            }

            /// Returns the diagnostic selector represented by this legacy value.
            #[must_use]
            pub const fn diagnostic_probe(self) -> Option<DiagnosticHeatBalanceProbe> {
                match self {
                    Self::EnergyPlusHeatBalanceCompatCandidate
                    | Self::EnergyPlusSourceOrder1ZoneOpaqueCompatibility => None,
                    $(Self::$variant => Some(DiagnosticHeatBalanceProbe::$variant),)+
                }
            }

            /// Returns the typed compatibility/diagnostic selection.
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

            /// Returns the source-order/diagnostic lane for this selector.
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

            /// Returns whether this selector represents a compatibility lane.
            #[must_use]
            pub const fn is_compatibility_source_order(self) -> bool {
                matches!(self.lane(), HeatBalanceAlgorithmLane::CompatibilitySourceOrder)
            }

            /// Returns whether this selector is diagnostic-only.
            #[must_use]
            pub const fn is_diagnostic_lane(self) -> bool {
                !self.is_compatibility_source_order()
            }

            /// Returns whether this selector may participate in conformance promotion.
            #[must_use]
            pub const fn allows_conformance_promotion(self) -> bool {
                self.lane().allows_conformance_promotion()
            }

            /// Converts the boundary selector to probe-agnostic runtime choices.
            #[must_use]
            pub(crate) const fn runtime_config(self) -> HeatBalanceRuntimeConfig {
                match self {
                    Self::EnergyPlusHeatBalanceCompatCandidate
                    | Self::EnergyPlusSourceOrder1ZoneOpaqueCompatibility => {
                        CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat
                            .runtime_config()
                    }
                    _ => diagnostic_runtime_config(self),
                }
            }
        }
    };
}

diagnostic_heat_balance_selectors! {
    SimplifiedAnalytical => "simplified-analytical",
    EnergyPlusAnalyticalProbe => "energyplus-analytical-probe",
    EnergyPlusAnalyticalSurfaceFirstProbe => "energyplus-analytical-surface-first-probe",
    EnergyPlusAnalyticalCoupledProbe => "energyplus-analytical-coupled-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideProbe => "energyplus-analytical-coupled-previous-inside-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe => "energyplus-analytical-coupled-previous-inside-doe2-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-longwave-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-current-adiabatic-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-previous-mat-surface-convection-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-frozen-outside-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-commit-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-live-reference-air-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-live-hconv-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-surface-reference-air-report-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-final-hconv-report-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-inside-ctf-report-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-inside-ctf-outside-history-scriptf-flat-adiabatic-report-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-frozen-reference-air-current-longwave-converged-surface-adiabatic-history-commit-probe",
    EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe => "energyplus-third-order-coupled-previous-inside-quick-outside-interleaved-interior-longwave-frozen-hconv-weather-air-storage-balance-surface-convection-current-adiabatic-history-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-interleaved-scriptf-interior-longwave-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe => "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-interior-longwave-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-interior-longwave-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-scriptf-interior-longwave-probe",
    EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe => "energyplus-analytical-coupled-previous-inside-quick-outside-doe2-scriptf-interior-longwave-probe",
    EnergyPlusAnalyticalCoupledPreviousBoundaryProbe => "energyplus-analytical-coupled-previous-boundary-probe",
    EnergyPlusThirdOrderProbe => "energyplus-third-order-probe",
}

impl CompatibilityHeatBalanceAlgorithm {
    /// Legacy selector conversion kept at the diagnostic boundary.
    #[must_use]
    pub const fn zone_air_algorithm(self) -> HeatBalanceZoneAirAlgorithm {
        match self {
            Self::SourceOrder1ZoneOpaqueCompat => {
                HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
            }
        }
    }
}

const fn diagnostic_runtime_config(
    selector: HeatBalanceZoneAirAlgorithm,
) -> HeatBalanceRuntimeConfig {
    let feature = feature_base(selector);
    let use_quick_outside_conduction = uses_quick_outside_conduction(feature);

    HeatBalanceRuntimeConfig {
        zone_air_update: zone_air_update(feature),
        timestep: HeatBalanceTimestepAlgorithmFlags {
            correct_zone_air_after_surface_pass: corrects_zone_air_after_surface_pass(feature),
            rebalance_surfaces_after_zone_air_correction: rebalances_surfaces(feature),
            interleave_zone_air_surface_passes: interleaves_surface_passes(feature),
            use_previous_inside_for_outdoor_boundary: uses_previous_inside_for_outdoor(feature),
            use_previous_inside_for_adiabatic_boundary: uses_previous_inside_for_adiabatic(feature),
            use_quick_outside_conduction,
        },
        use_third_order_zone_air_correction: uses_third_order_correction(feature),
        use_energyplus_adaptive_system_timestep_zone_air_correction: false,
        report_zone_timestep_averages: false,
        interior_longwave_mode: interior_longwave_mode(selector, feature),
        preserve_surface_inside_temperature_for_first_longwave:
            preserves_inside_temperature_for_first_longwave(feature),
        use_current_inside_for_first_longwave: uses_current_inside_for_first_longwave(feature),
        freeze_inside_convection_coefficients: freezes_inside_convection(selector, feature),
        freeze_surface_reference_air: freezes_surface_reference_air(selector, feature),
        converge_interleaved_surface_iterations_to_energyplus_tolerance:
            converges_surface_iterations(feature),
        use_doe2_outside_convection: uses_doe2_outside_convection(feature),
        use_cached_exterior_report_terms: uses_cached_exterior_report_terms(feature),
        freeze_outside_balance_for_surface_iterations: freezes_outside_balance(selector),
        freeze_inside_ctf_outside_temperature_for_surface_iterations:
            freezes_inside_ctf_outside_temperature(selector),
        use_inside_ctf_outside_temperature_for_conduction_report:
            uses_inside_ctf_outside_temperature_for_report(selector),
        commit_inside_ctf_outside_temperature_to_history: commits_inside_ctf_outside_temperature(
            selector,
        ),
        sync_adiabatic_outside_to_current_inside_before_history:
            syncs_adiabatic_outside_before_history(selector),
        sync_adiabatic_outside_to_current_inside_for_report_only:
            syncs_adiabatic_outside_for_report(selector),
        commit_adiabatic_current_inside_to_history_only: commits_adiabatic_inside_to_history(
            selector,
        ),
        use_weather_air_storage_report: uses_weather_air_storage_report(feature),
        use_previous_mat_surface_convection_report: uses_previous_mat_surface_convection_report(
            feature,
        ),
        use_balance_surface_convection_report: uses_balance_surface_convection_report(selector),
        use_surface_reference_air_convection_report: uses_surface_reference_air_convection_report(
            selector,
        ),
        use_surface_reference_air_surface_convection_report:
            uses_surface_reference_air_surface_convection_report(selector),
        use_final_inside_convection_report: uses_final_inside_convection_report(selector),
    }
}

const fn feature_base(selector: HeatBalanceZoneAirAlgorithm) -> HeatBalanceZoneAirAlgorithm {
    use HeatBalanceZoneAirAlgorithm as A;
    match selector {
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
        | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe
        | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
        | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
        | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
        | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
        | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
        | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
        | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
        | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe => {
            A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
        }
        _ => selector,
    }
}

const fn zone_air_update(selector: HeatBalanceZoneAirAlgorithm) -> HeatBalanceZoneAirUpdate {
    use HeatBalanceZoneAirAlgorithm as A;
    match selector {
        A::SimplifiedAnalytical => HeatBalanceZoneAirUpdate::SimplifiedAnalytical,
        A::EnergyPlusAnalyticalProbe => HeatBalanceZoneAirUpdate::EnergyPlusAnalytical,
        A::EnergyPlusThirdOrderProbe => HeatBalanceZoneAirUpdate::EnergyPlusThirdOrder,
        _ => HeatBalanceZoneAirUpdate::Deferred,
    }
}

const fn corrects_zone_air_after_surface_pass(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    !matches!(
        selector,
        A::SimplifiedAnalytical
            | A::EnergyPlusAnalyticalProbe
            | A::EnergyPlusThirdOrderProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
    )
}

const fn rebalances_surfaces(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusAnalyticalCoupledProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
    )
}

const fn interleaves_surface_passes(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    )
}

const fn uses_previous_inside_for_outdoor(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    !matches!(
        selector,
        A::SimplifiedAnalytical
            | A::EnergyPlusAnalyticalProbe
            | A::EnergyPlusAnalyticalSurfaceFirstProbe
            | A::EnergyPlusAnalyticalCoupledProbe
            | A::EnergyPlusThirdOrderProbe
    )
}

const fn uses_previous_inside_for_adiabatic(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    )
}

const fn uses_quick_outside_conduction(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    !matches!(
        selector,
        A::SimplifiedAnalytical
            | A::EnergyPlusAnalyticalProbe
            | A::EnergyPlusAnalyticalSurfaceFirstProbe
            | A::EnergyPlusAnalyticalCoupledProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
            | A::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
            | A::EnergyPlusThirdOrderProbe
    )
}

const fn uses_cached_exterior_report_terms(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    uses_quick_outside_conduction(selector)
        && !matches!(
            selector,
            A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
        )
}

const fn uses_third_order_correction(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusThirdOrderProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    )
}

const fn interior_longwave_mode(
    selector: HeatBalanceZoneAirAlgorithm,
    feature: HeatBalanceZoneAirAlgorithm,
) -> HeatBalanceInteriorLongwaveMode {
    use HeatBalanceInteriorLongwaveMode as L;
    use HeatBalanceZoneAirAlgorithm as A;
    if matches!(
        selector,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
    ) {
        L::EnergyPlusScriptF
    } else if matches!(
        selector,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
    ) {
        L::EnergyPlusScriptFFlatAccess
    } else if matches!(
        feature,
        A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
    ) {
        L::EnergyPlusScriptF
    } else if interleaves_surface_passes(feature)
        && !matches!(feature, A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe)
        || matches!(
            feature,
            A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
                | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
        )
    {
        L::GreyEnergyPlusDirectViewFactor
    } else {
        L::None
    }
}

const fn preserves_inside_temperature_for_first_longwave(
    selector: HeatBalanceZoneAirAlgorithm,
) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    )
}

const fn uses_current_inside_for_first_longwave(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    )
}

const fn freezes_inside_convection(
    selector: HeatBalanceZoneAirAlgorithm,
    feature: HeatBalanceZoneAirAlgorithm,
) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    !matches!(
        selector,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
    ) && matches!(
        feature,
        A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    )
}

const fn freezes_surface_reference_air(
    selector: HeatBalanceZoneAirAlgorithm,
    feature: HeatBalanceZoneAirAlgorithm,
) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    !matches!(
        selector,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
    ) && matches!(
        feature,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    )
}

const fn converges_surface_iterations(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    )
}

const fn uses_doe2_outside_convection(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
            | A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
    )
}

const fn freezes_outside_balance(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
    )
}

const fn freezes_inside_ctf_outside_temperature(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
    )
}

const fn uses_inside_ctf_outside_temperature_for_report(
    selector: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        selector,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
    )
}

const fn commits_inside_ctf_outside_temperature(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    matches!(
        selector,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe
    )
}

const fn syncs_adiabatic_outside_before_history(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    matches!(
        selector,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    )
}

const fn syncs_adiabatic_outside_for_report(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    matches!(
        selector,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
    )
}

const fn commits_adiabatic_inside_to_history(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    matches!(
        selector,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    )
}

const fn uses_weather_air_storage_report(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    )
}

const fn uses_previous_mat_surface_convection_report(
    selector: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        selector,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
    )
}

const fn uses_balance_surface_convection_report(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    )
}

const fn uses_surface_reference_air_convection_report(
    selector: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        selector,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
    )
}

const fn uses_surface_reference_air_surface_convection_report(
    selector: HeatBalanceZoneAirAlgorithm,
) -> bool {
    use HeatBalanceZoneAirAlgorithm as A;
    matches!(
        selector,
        A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
            | A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
    )
}

const fn uses_final_inside_convection_report(selector: HeatBalanceZoneAirAlgorithm) -> bool {
    matches!(
        selector,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_selector_round_trips_through_cli_and_typed_boundary() {
        for &selector in HeatBalanceZoneAirAlgorithm::ALL {
            assert_eq!(
                HeatBalanceZoneAirAlgorithm::from_cli_name(selector.cli_name()),
                Some(selector)
            );
            match selector.selection() {
                HeatBalanceZoneAirSelection::Compatibility(algorithm) => {
                    assert_eq!(selector.compatibility_algorithm(), Some(algorithm));
                    assert_eq!(selector.diagnostic_probe(), None);
                    assert_eq!(selector.runtime_config(), algorithm.runtime_config());
                }
                HeatBalanceZoneAirSelection::Diagnostic(probe) => {
                    assert_eq!(selector.compatibility_algorithm(), None);
                    assert_eq!(selector.diagnostic_probe(), Some(probe));
                    assert_eq!(selector.runtime_config(), probe.runtime_config());
                }
            }
        }
    }

    #[test]
    fn compatibility_aliases_resolve_to_the_same_explicit_config() {
        let legacy = HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate;
        let source_order =
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility;

        assert_eq!(legacy.runtime_config(), source_order.runtime_config());
        assert_eq!(
            source_order.runtime_config(),
            CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat.runtime_config()
        );

        let mut historical_feature_base = HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe.runtime_config();
        historical_feature_base.use_energyplus_adaptive_system_timestep_zone_air_correction = true;
        historical_feature_base.report_zone_timestep_averages = true;
        assert_eq!(source_order.runtime_config(), historical_feature_base);
    }

    #[test]
    fn cli_boundary_handles_longest_and_invalid_names() {
        use HeatBalanceZoneAirAlgorithm as A;

        let longest = A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe;
        assert_eq!(A::from_cli_name(longest.cli_name()), Some(longest));
        assert_eq!(
            A::from_cli_name("  SIMPLIFIED-ANALYTICAL  "),
            Some(A::SimplifiedAnalytical)
        );
        assert_eq!(A::from_cli_name("not-a-heat-balance-selector"), None);
    }

    #[test]
    fn representative_diagnostic_configs_preserve_feature_differences() {
        use HeatBalanceInteriorLongwaveMode as L;
        use HeatBalanceZoneAirAlgorithm as A;
        use HeatBalanceZoneAirUpdate as U;

        assert_eq!(
            A::SimplifiedAnalytical.runtime_config().zone_air_update,
            U::SimplifiedAnalytical
        );
        assert_eq!(
            A::EnergyPlusAnalyticalProbe
                .runtime_config()
                .zone_air_update,
            U::EnergyPlusAnalytical
        );
        assert_eq!(
            A::EnergyPlusThirdOrderProbe
                .runtime_config()
                .zone_air_update,
            U::EnergyPlusThirdOrder
        );

        let scriptf =
            A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
                .runtime_config();
        assert_eq!(scriptf.interior_longwave_mode, L::EnergyPlusScriptF);
        assert!(scriptf.timestep.use_quick_outside_conduction);

        let interleaved_scriptf = A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe.runtime_config();
        assert!(interleaved_scriptf.timestep.use_quick_outside_conduction);
        assert!(!interleaved_scriptf.use_cached_exterior_report_terms);

        let doe2 =
            A::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe.runtime_config();
        assert!(doe2.use_doe2_outside_convection);

        let report = A::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe.runtime_config();
        assert!(report.use_inside_ctf_outside_temperature_for_conduction_report);
        assert!(report.freeze_inside_ctf_outside_temperature_for_surface_iterations);
        assert_eq!(
            report.interior_longwave_mode,
            L::EnergyPlusScriptFFlatAccess
        );
    }

    #[test]
    fn diagnostic_metadata_declares_non_claim_boundary() {
        let metadata = DiagnosticHeatBalanceProbe::EnergyPlusThirdOrderProbe.metadata();

        assert_eq!(metadata.name, "EnergyPlusThirdOrderProbe");
        assert!(metadata.purpose.contains("without expanding conformance"));
        assert!(metadata.expected_bottleneck.contains("delta"));
        assert!(metadata.why_it_exists.contains("Diagnostic-only"));
        assert!(metadata.mismatch_investigated.contains("mismatch"));
    }
}
