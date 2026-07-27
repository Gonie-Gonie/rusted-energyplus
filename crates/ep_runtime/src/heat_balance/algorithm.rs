//! Probe-agnostic heat-balance compatibility configuration.

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

    /// Explicit runtime configuration for this compatibility algorithm.
    #[must_use]
    pub(crate) const fn runtime_config(self) -> HeatBalanceRuntimeConfig {
        match self {
            Self::SourceOrder1ZoneOpaqueCompat => HeatBalanceRuntimeConfig {
                zone_air_update: HeatBalanceZoneAirUpdate::Deferred,
                timestep: HeatBalanceTimestepAlgorithmFlags {
                    correct_zone_air_after_surface_pass: true,
                    rebalance_surfaces_after_zone_air_correction: false,
                    interleave_zone_air_surface_passes: true,
                    use_previous_inside_for_outdoor_boundary: true,
                    use_previous_inside_for_adiabatic_boundary: true,
                    use_quick_outside_conduction: true,
                },
                use_third_order_zone_air_correction: true,
                use_energyplus_adaptive_system_timestep_zone_air_correction: true,
                report_zone_timestep_averages: true,
                interior_longwave_mode:
                    HeatBalanceInteriorLongwaveMode::EnergyPlusScriptFFlatAccess,
                preserve_surface_inside_temperature_for_first_longwave: true,
                use_current_inside_for_first_longwave: true,
                freeze_inside_convection_coefficients: true,
                freeze_surface_reference_air: true,
                converge_interleaved_surface_iterations_to_energyplus_tolerance: true,
                use_doe2_outside_convection: false,
                use_cached_exterior_report_terms: true,
                freeze_outside_balance_for_surface_iterations: true,
                freeze_inside_ctf_outside_temperature_for_surface_iterations: true,
                use_inside_ctf_outside_temperature_for_conduction_report: false,
                commit_inside_ctf_outside_temperature_to_history: false,
                sync_adiabatic_outside_to_current_inside_before_history: false,
                sync_adiabatic_outside_to_current_inside_for_report_only: false,
                commit_adiabatic_current_inside_to_history_only: false,
                use_weather_air_storage_report: true,
                use_previous_mat_surface_convection_report: false,
                use_balance_surface_convection_report: false,
                use_surface_reference_air_convection_report: false,
                use_surface_reference_air_surface_convection_report: true,
                use_final_inside_convection_report: false,
            },
        }
    }
}

/// Fixed-system-timestep ThirdOrder runtime choices for the bounded
/// direct-Zone IdealLoads/PurchasedAir coupling.
///
/// This is deliberately separate from the opaque one-Zone compatibility
/// selector: that lane owns adaptive system-timestep correction, while the
/// first model-bound PurchasedAir caller requires one nominal system step and
/// system-timestep history on every call.
#[must_use]
pub(crate) const fn direct_zone_purchased_air_fixed_step_runtime_config() -> HeatBalanceRuntimeConfig
{
    let mut config =
        CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat.runtime_config();
    config.use_energyplus_adaptive_system_timestep_zone_air_correction = false;
    config.report_zone_timestep_averages = false;
    config
}

/// Classification for heat-balance zone-air algorithms in reports and gates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeatBalanceAlgorithmLane {
    /// Source-order compatibility candidate lane.
    CompatibilitySourceOrder,
    /// Baseline shell that is not conformance-safe.
    DiagnosticOnly,
    /// Experimental runtime variant.
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

/// Zone-air update performed before the shared correction stages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HeatBalanceZoneAirUpdate {
    /// Existing simplified analytical update.
    SimplifiedAnalytical,
    /// EnergyPlus analytical predictor update.
    EnergyPlusAnalytical,
    /// EnergyPlus third-order predictor update.
    EnergyPlusThirdOrder,
    /// Keep the previous value and defer the update to a later correction stage.
    Deferred,
}

/// Interior longwave implementation selected for a heat-balance run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum HeatBalanceInteriorLongwaveMode {
    /// No interior longwave exchange override.
    None,
    /// Grey direct-view-factor exchange.
    GreyEnergyPlusDirectViewFactor,
    /// EnergyPlus ScriptF exchange.
    EnergyPlusScriptF,
    /// EnergyPlus ScriptF exchange with flat lSR access order.
    EnergyPlusScriptFFlatAccess,
}

/// Probe-agnostic runtime choices consumed by heat-balance compatibility code.
///
/// Compatibility algorithms declare these values directly. Legacy diagnostic
/// selectors are converted to this type at the diagnostic namespace boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HeatBalanceRuntimeConfig {
    pub(crate) zone_air_update: HeatBalanceZoneAirUpdate,
    pub(crate) timestep: HeatBalanceTimestepAlgorithmFlags,
    pub(crate) use_third_order_zone_air_correction: bool,
    pub(crate) use_energyplus_adaptive_system_timestep_zone_air_correction: bool,
    pub(crate) report_zone_timestep_averages: bool,
    pub(crate) interior_longwave_mode: HeatBalanceInteriorLongwaveMode,
    pub(crate) preserve_surface_inside_temperature_for_first_longwave: bool,
    pub(crate) use_current_inside_for_first_longwave: bool,
    pub(crate) freeze_inside_convection_coefficients: bool,
    pub(crate) freeze_surface_reference_air: bool,
    pub(crate) converge_interleaved_surface_iterations_to_energyplus_tolerance: bool,
    pub(crate) use_doe2_outside_convection: bool,
    pub(crate) use_cached_exterior_report_terms: bool,
    pub(crate) freeze_outside_balance_for_surface_iterations: bool,
    pub(crate) freeze_inside_ctf_outside_temperature_for_surface_iterations: bool,
    pub(crate) use_inside_ctf_outside_temperature_for_conduction_report: bool,
    pub(crate) commit_inside_ctf_outside_temperature_to_history: bool,
    pub(crate) sync_adiabatic_outside_to_current_inside_before_history: bool,
    pub(crate) sync_adiabatic_outside_to_current_inside_for_report_only: bool,
    pub(crate) commit_adiabatic_current_inside_to_history_only: bool,
    pub(crate) use_weather_air_storage_report: bool,
    pub(crate) use_previous_mat_surface_convection_report: bool,
    pub(crate) use_balance_surface_convection_report: bool,
    pub(crate) use_surface_reference_air_convection_report: bool,
    pub(crate) use_surface_reference_air_surface_convection_report: bool,
    pub(crate) use_final_inside_convection_report: bool,
}

/// Timestep sequencing choices shared by all runtime lanes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct HeatBalanceTimestepAlgorithmFlags {
    pub(crate) correct_zone_air_after_surface_pass: bool,
    pub(crate) rebalance_surfaces_after_zone_air_correction: bool,
    pub(crate) interleave_zone_air_surface_passes: bool,
    pub(crate) use_previous_inside_for_outdoor_boundary: bool,
    pub(crate) use_previous_inside_for_adiabatic_boundary: bool,
    pub(crate) use_quick_outside_conduction: bool,
}

#[cfg(test)]
mod tests {
    use super::{
        CompatibilityHeatBalanceAlgorithm, HeatBalanceInteriorLongwaveMode,
        HeatBalanceZoneAirUpdate,
    };

    #[test]
    fn compatibility_algorithm_declares_explicit_runtime_config() {
        let compatibility = CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat;
        let config = compatibility.runtime_config();

        assert_eq!(compatibility.id(), "source-order-1zone-opaque-compat");
        assert_eq!(config.zone_air_update, HeatBalanceZoneAirUpdate::Deferred);
        assert!(config.timestep.interleave_zone_air_surface_passes);
        assert!(config.use_third_order_zone_air_correction);
        assert!(config.use_energyplus_adaptive_system_timestep_zone_air_correction);
        assert_eq!(
            config.interior_longwave_mode,
            HeatBalanceInteriorLongwaveMode::EnergyPlusScriptFFlatAccess
        );
        assert!(config.freeze_outside_balance_for_surface_iterations);
        assert!(config.freeze_inside_ctf_outside_temperature_for_surface_iterations);
        assert!(config.use_surface_reference_air_surface_convection_report);

        let coupled_config = super::direct_zone_purchased_air_fixed_step_runtime_config();
        assert_eq!(
            coupled_config.zone_air_update,
            HeatBalanceZoneAirUpdate::Deferred
        );
        assert!(coupled_config.use_third_order_zone_air_correction);
        assert!(!coupled_config.use_energyplus_adaptive_system_timestep_zone_air_correction);
        assert!(!coupled_config.report_zone_timestep_averages);
    }
}
