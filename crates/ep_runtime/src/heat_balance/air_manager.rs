//! HeatBalanceAirManager source-order stage contract.

use crate::execution_plan::{EnergyPlusCompatibilityStage, ExecutionStageKind};
use crate::heat_balance::algorithm::{
    HeatBalanceZoneAirAlgorithm, heat_balance_zone_air_algorithm_feature_base,
};
use crate::heat_balance::state::{HeatBalanceState, ZoneHeatBalanceState};
use crate::psychrometrics::energyplus_zone_air_heat_capacity_j_per_k;
use crate::weather::{
    EpwRecord, HeatBalanceWeatherContext, energyplus_weather_atmospheric_pressure_for_context,
    weather_context_outdoor_humidity_ratio,
};
use ep_model::FirstHourInterpolationStartingValues;

/// EnergyPlus `HeatBalanceAirManager::ManageAirHeatBalance`.
#[must_use]
pub const fn manage_air_heat_balance_stage() -> EnergyPlusCompatibilityStage {
    EnergyPlusCompatibilityStage {
        kind: ExecutionStageKind::ManageAirHeatBalance,
        stage_name: "manage-air-heat-balance",
        source_file: "src/EnergyPlus/HeatBalanceAirManager.cc",
        source_routine: "ManageAirHeatBalance",
    }
}

pub(crate) fn weather_proxy_zone_air_heat_capacity_j_per_k(
    zone: &ZoneHeatBalanceState,
    context: Option<HeatBalanceWeatherContext<'_>>,
    _fallback_dry_bulb_c: f64,
) -> Option<f64> {
    weather_context_zone_air_heat_capacity_j_per_k(zone, context)
}

pub(crate) fn weather_context_zone_air_heat_capacity_j_per_k(
    zone: &ZoneHeatBalanceState,
    context: Option<HeatBalanceWeatherContext<'_>>,
) -> Option<f64> {
    let context = context?;
    let record = context.records.get(context.record_index)?;
    let atmospheric_pressure_pa = energyplus_weather_atmospheric_pressure_for_context(
        context,
        record.atmospheric_pressure_pa,
    );

    energyplus_zone_air_heat_capacity_j_per_k(
        zone.volume_m3,
        atmospheric_pressure_pa,
        zone.mean_air_temperature_c,
        zone.air_humidity_ratio,
    )
}

pub(crate) fn update_single_zone_air_heat_capacity_from_weather_context(
    zone: &mut ZoneHeatBalanceState,
    context: Option<HeatBalanceWeatherContext<'_>>,
    _fallback_dry_bulb_c: f64,
) {
    if let Some(air_heat_capacity_j_per_k) =
        weather_context_zone_air_heat_capacity_j_per_k(zone, context)
    {
        zone.air_heat_capacity_j_per_k = air_heat_capacity_j_per_k;
    }
}

pub(crate) fn update_zone_air_heat_capacities_from_weather_context(
    zones: &mut [ZoneHeatBalanceState],
    context: Option<HeatBalanceWeatherContext<'_>>,
    _fallback_dry_bulb_c: f64,
) {
    for zone in zones {
        if let Some(air_heat_capacity_j_per_k) =
            weather_context_zone_air_heat_capacity_j_per_k(zone, context)
        {
            zone.air_heat_capacity_j_per_k = air_heat_capacity_j_per_k;
        }
    }
}

pub(crate) fn seed_zone_air_humidity_ratios_from_weather_records(
    state: &mut HeatBalanceState,
    weather_records: Option<&[EpwRecord]>,
    fallback_dry_bulb_c: f64,
    zone_steps_per_hour: u32,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
) {
    let Some(records) = weather_records else {
        return;
    };
    let Some(humidity_ratio) = weather_context_outdoor_humidity_ratio(
        HeatBalanceWeatherContext {
            records,
            record_index: 0,
            zone_steps_per_hour,
            zone_timestep: Some(1),
            first_hour_interpolation_starting_values,
        },
        fallback_dry_bulb_c,
    ) else {
        return;
    };

    for zone in &mut state.zones {
        zone.air_humidity_ratio = humidity_ratio;
        zone.zone_timestep_average_air_humidity_ratio = humidity_ratio;
        zone.previous_air_humidity_ratios = [humidity_ratio; 3];
        zone.previous_system_air_humidity_ratios = [humidity_ratio; 3];
    }
}

pub(crate) fn zone_air_heat_balance_air_storage_rate_w(
    zone_state: &ZoneHeatBalanceState,
    timestep_seconds: f64,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    third_order_report_air_heat_capacity_j_per_k: Option<f64>,
) -> f64 {
    let zone_air_algorithm = heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);
    match zone_air_algorithm {
        HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe => {
            zone_state
                .zone_air_temperature_coefficients
                .temp_independent_coefficient_w
                - zone_state
                    .zone_air_temperature_coefficients
                    .temp_dependent_coefficient_w_per_k
                    * zone_state.mean_air_temperature_c
        }
        HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate
        | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
        | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe => {
            if timestep_seconds > 0.0 {
                third_order_report_air_heat_capacity_j_per_k
                    .unwrap_or(zone_state.air_heat_capacity_j_per_k)
                    * (zone_state.mean_air_temperature_c
                        - zone_state.previous_mean_air_temperatures_c[0])
                    / timestep_seconds
            } else {
                0.0
            }
        }
    }
}
