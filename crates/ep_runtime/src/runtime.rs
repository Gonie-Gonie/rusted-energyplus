//! Runtime state, heat-balance execution, weather, and trace helpers.

pub use crate::diagnostics::*;
pub use crate::error::*;
pub use crate::first_zone::*;
pub use crate::geometry::{surface_area_m2, surface_geometry_summaries, zone_geometry_summaries};
pub(crate) use crate::geometry::{surface_azimuth_deg, surface_tilt_deg, zone_volume_m3};
use crate::heat_balance::air_manager::{
    seed_zone_air_humidity_ratios_from_weather_records,
    update_single_zone_air_heat_capacity_from_weather_context,
    update_zone_air_heat_capacities_from_weather_context,
    weather_proxy_zone_air_heat_capacity_j_per_k, zone_air_heat_balance_air_storage_rate_w,
};
#[cfg(test)]
use crate::heat_balance::convection::{
    ENERGYPLUS_DEFAULT_WEATHER_FILE_TEMPERATURE_SENSOR_HEIGHT_M,
    ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K,
    energyplus_ashrae_tarp_natural_convection_w_per_m2_k,
    energyplus_doe2_outside_convection_coefficient_w_per_m2_k,
};
use crate::heat_balance::convection::{
    energyplus_building_terrain, energyplus_surface_outdoor_air_temperature_c,
    energyplus_surface_outside_wind_speed_m_per_s,
    energyplus_tarp_inside_convection_coefficient_w_per_m2_k,
    heat_balance_uses_doe2_outside_convection,
};
pub use crate::heat_balance::ctf::ConstructionCtfCoefficientOverride;
use crate::heat_balance::ctf::CtfInsideFaceBalanceInput;
#[cfg(test)]
use crate::heat_balance::ctf::{
    CtfOutsideFaceBalanceInput, CtfOutsideQuickConductionBalanceInput,
    energyplus_ctf_inside_face_temperature_c, energyplus_ctf_outside_face_temperature_c,
    energyplus_ctf_outside_face_temperature_quick_conduction_c, surface_ctf_history_slot_samples,
    surface_inside_conduction_flux_w_per_m2, surface_inside_conduction_rate_w,
    surface_outside_conduction_flux_w_per_m2, surface_outside_conduction_rate_w,
};
use crate::heat_balance::ctf::{
    advance_surface_ctf_histories, advance_surface_ctf_histories_with_outside_temperature_override,
    heat_balance_ctf_history_slot_inside_flux_term_rate_w,
    heat_balance_ctf_history_slot_inside_temperature_term_rate_w,
    heat_balance_ctf_history_slot_samples, surface_ctf_inside_current_inside_term_rate_w,
    surface_ctf_inside_current_outside_term_rate_w_for_report,
    surface_ctf_inside_history_term_rate_w, surface_ctf_outside_current_inside_term_rate_w,
    surface_ctf_outside_current_outside_term_rate_w_for_report,
    surface_ctf_outside_history_term_rate_w, surface_heat_storage_rate_w,
    surface_inside_conduction_rate_w_for_report, surface_outside_conduction_rate_w_for_report,
    surface_rate_per_area_w_per_m2, update_surface_ctf_history_constants,
};
pub(crate) use crate::heat_balance::ctf::{
    construction_ctf_coefficients_by_name,
    energyplus_ctf_inside_face_temperature_c_with_outside_temperature,
    steady_ctf_coefficient_w_per_m2_k, steady_surface_ctf_state,
    surface_ctf_state_from_coefficients,
};
use crate::heat_balance::longwave::horizontal_infrared_sky_temperature_c;
#[cfg(test)]
use crate::heat_balance::longwave::{
    energyplus_exterior_longwave_terms, energyplus_linearized_radiation_coefficient_w_per_m2_k,
};
#[cfg(test)]
pub(crate) use crate::heat_balance::radiation::surface_incident_solar_radiation_for_weather_context_w_per_m2;
use crate::heat_balance::radiation::{
    InteriorLongwaveExchangeProbe, update_surface_inside_longwave_exchange_probe,
    update_surface_inside_scriptf_flat_access_longwave_exchange_probe,
    update_surface_inside_scriptf_longwave_exchange_probe,
};
#[cfg(test)]
use crate::heat_balance::radiation::{
    InteriorLongwaveSurfaceSnapshot, KELVIN_OFFSET, STEFAN_BOLTZMANN_W_PER_M2_K4,
    append_surface_incident_solar_radiation_series, energyplus_approximate_view_factors,
    energyplus_scriptf_from_view_factors, fix_energyplus_approximate_view_factors,
    surface_incident_solar_components_hourly_average_w_per_m2,
};
pub use crate::heat_balance::state::*;
pub(crate) use crate::heat_balance::state::{
    InsideConvectionCoefficientInputState, SurfaceBoundaryBalanceResult,
    SurfaceExteriorReportTerms, SurfaceOutsideBalanceDiagnostics,
};
#[cfg(test)]
use crate::heat_balance::surface_balance::exterior_surface_energy_balance;
use crate::heat_balance::surface_balance::{
    QuickOutsideConductionContext, heat_balance_surface_boundary_balance,
    reported_surface_outside_face_temperature_c, surface_exterior_report_terms,
    surface_inside_ctf_source_terms_w_per_m2,
};
#[cfg(test)]
use crate::heat_balance::surface_boundary::{
    ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C, surface_steady_u_value_w_per_m2_k,
};
use crate::heat_balance::surface_boundary::{
    resolve_surface_boundary_target, seed_energyplus_initial_surface_ctf_histories,
    seed_initial_surface_ctf_boundary_histories,
};
pub(crate) use crate::heat_balance::surface_thermal_properties;
#[cfg(test)]
use crate::heat_balance::surface_weather::{
    energyplus_exterior_wet_context_fraction, energyplus_exterior_wet_timestep_fraction,
    energyplus_weather_record_is_rain_at_timestep,
};
use crate::heat_balance::surface_weather::{
    energyplus_exterior_wet_reference_temperature_c,
    energyplus_weather_record_is_rain_at_timestep_with_starting_values,
};
pub(crate) use crate::heat_balance::trace::*;
use crate::heat_balance::{
    HeatBalanceZoneAirAlgorithm, energyplus_analytical_zone_air_temperature_c,
    energyplus_zone_air_temperature_coefficients,
    heat_balance_zone_air_algorithm_execution_variant,
    heat_balance_zone_air_algorithm_feature_base,
};
#[cfg(test)]
use crate::heat_balance::{
    energyplus_anisotropic_sky_multiplier, energyplus_average_solar_coefficients,
    energyplus_daily_solar_coefficients, energyplus_shadowing_period_solar_coefficients,
    energyplus_third_order_zone_air_temperature_c, energyplus_weather_record_day_of_year,
    solar_position_rad_at_local_hour, solar_weather_interpolation_weights,
};
pub(crate) use crate::heat_balance::{
    energyplus_third_order_zone_air_temperature_from_coefficients, step_zone_air_temperature,
};
#[cfg(test)]
pub(crate) use crate::heat_balance::{surface_air_sky_radiation_split, surface_sky_view_factor};
#[cfg(test)]
pub(crate) use crate::psychrometrics::energyplus_outdoor_wet_bulb_c;
pub use crate::psychrometrics::{
    energyplus_moist_air_density_kg_per_m3, energyplus_moist_air_specific_heat_j_per_kg_k,
    energyplus_psychrometric_humidity_ratio_from_rh, energyplus_water_vapor_gas_enthalpy_j_per_kg,
    energyplus_zone_air_heat_capacity_j_per_k,
};
pub use crate::schedules::{
    ScheduleTrace, ZoneInternalGainTrace, simulate_constant_schedules, simulate_schedule_values,
    simulate_zone_internal_convective_gains,
};
use crate::schedules::{
    convective_internal_gain_w, update_surface_radiant_internal_gain_source_terms,
};
use crate::time_axis::run_period_first_hour_interpolation_starting_values;
pub use crate::weather::*;
pub(crate) use crate::weather::{
    HeatBalanceWeatherContext, energyplus_weather_atmospheric_pressure_for_context,
    energyplus_weather_dry_bulb_at_timestep_with_starting_values,
    energyplus_weather_horizontal_infrared_for_context,
    energyplus_weather_wind_direction_for_context, energyplus_weather_wind_speed_for_context,
    heat_balance_weather_context_for_timestep,
};
#[cfg(test)]
use crate::weather::{
    energyplus_weather_atmospheric_pressure_at_timestep, energyplus_weather_dry_bulb_at_timestep,
    energyplus_weather_horizontal_infrared_at_timestep,
    energyplus_weather_relative_humidity_at_timestep,
    energyplus_weather_wind_direction_at_timestep, energyplus_weather_wind_speed_at_timestep,
};
use crate::{OutputSeries, ResultStore};
#[cfg(test)]
use crate::{SimulationMode, SimulationState};
use ep_model::{
    FirstHourInterpolationStartingValues, OutputHandle, OutsideBoundaryCondition, SimulationModel,
    SurfaceId, TypedModel, ZoneId,
};
use std::collections::BTreeMap;

const AIR_DENSITY_KG_PER_M3: f64 = 1.2;
const AIR_SPECIFIC_HEAT_J_PER_KG_K: f64 = 1006.0;
const SECONDS_PER_HOUR: f64 = 3600.0;
#[cfg(test)]
const ENERGYPLUS_ZONE_INITIAL_TEMP_C: f64 = 23.0;
const ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO: f64 = 0.008;
const ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA: f64 = 101_325.0;
const ENERGYPLUS_MAX_ALLOWED_INSIDE_SURFACE_DELTA_C: f64 = 0.002;
const ENERGYPLUS_MAX_ZONE_TEMP_DIFF_C: f64 = 0.3;
const ENERGYPLUS_MIN_SYSTEM_TIMESTEP_SECONDS: f64 = 60.0;
const ENERGYPLUS_INITIAL_CONVECTION_COEFFICIENT_W_PER_M2_K: f64 = 3.076;
const ENERGYPLUS_MIN_HUMIDITY_RATIO: f64 = 1.0e-5;

/// Initializes the heat-balance state shell without advancing the solver.
pub fn initialize_heat_balance_state(
    model: &SimulationModel,
    initial_zone_air_temperature_c: f64,
) -> Result<HeatBalanceState, RuntimeError> {
    initialize_heat_balance_state_with_ctf_coefficients(model, initial_zone_air_temperature_c, &[])
}

/// Initializes the heat-balance state shell with diagnostic CTF coefficient rows.
///
/// This is an oracle-isolation hook for heat-balance diagnostics. It does not
/// calculate EnergyPlus CTF coefficients; callers may provide rows already
/// emitted by EnergyPlus so surface history behavior can be tested separately
/// from coefficient generation.
pub fn initialize_heat_balance_state_with_ctf_coefficients(
    model: &SimulationModel,
    initial_zone_air_temperature_c: f64,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
) -> Result<HeatBalanceState, RuntimeError> {
    let ctf_coefficients_by_construction = construction_ctf_coefficients_by_name(ctf_coefficients);
    let mut zones = Vec::with_capacity(model.typed.zones.len());
    for zone in &model.typed.zones {
        let volume_m3 =
            zone_volume_m3(&model.typed, zone).ok_or_else(|| RuntimeError::MissingZoneVolume {
                zone_name: zone.name.0.clone(),
            })?;
        zones.push(ZoneHeatBalanceState {
            zone_id: zone.id,
            zone_name: zone.name.0.clone(),
            mean_air_temperature_c: initial_zone_air_temperature_c,
            zone_timestep_average_air_temperature_c: initial_zone_air_temperature_c,
            previous_mean_air_temperatures_c: [initial_zone_air_temperature_c; 3],
            previous_system_mean_air_temperatures_c: [initial_zone_air_temperature_c; 3],
            previous_system_timestep_count: 1,
            air_humidity_ratio: ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO,
            zone_timestep_average_air_humidity_ratio: ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO,
            previous_air_humidity_ratios: [ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO; 3],
            previous_system_air_humidity_ratios: [ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO; 3],
            volume_m3,
            air_heat_capacity_j_per_k: volume_m3
                * AIR_DENSITY_KG_PER_M3
                * AIR_SPECIFIC_HEAT_J_PER_KG_K,
            convective_internal_gain_w: convective_internal_gain_w(&model.typed, zone.id, 1),
            opaque_surface_conductance_w_per_k: 0.0,
            opaque_surface_heat_gain_w: 0.0,
            opaque_surface_outside_conduction_w: 0.0,
            sum_ha_w_per_k: 0.0,
            sum_hat_surf_w: 0.0,
            sum_hat_ref_w: 0.0,
            zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
            system_timestep_average_surface_convection_report_w: None,
            system_timestep_average_air_storage_report_w: None,
        });
    }

    let mut surfaces = model
        .typed
        .surfaces
        .iter()
        .map(|surface| {
            let area_m2 = surface_area_m2(&surface.vertices);
            let azimuth_deg = surface_azimuth_deg(&surface.vertices);
            let tilt_deg = surface_tilt_deg(surface.surface_type, &surface.vertices);
            let thermal = surface_thermal_properties(&model.typed, surface)?;
            let boundary = resolve_surface_boundary_target(&model.typed, surface)?;
            let conductance_w_per_k = area_m2 / thermal.thermal_resistance_m2_k_per_w;
            let steady_ctf_w_per_m2_k =
                steady_ctf_coefficient_w_per_m2_k(area_m2, thermal.thermal_resistance_m2_k_per_w);
            let ctf = ctf_coefficients_by_construction
                .get(&thermal.construction_name)
                .and_then(|coefficients| {
                    surface_ctf_state_from_coefficients(
                        coefficients,
                        initial_zone_air_temperature_c,
                    )
                })
                .unwrap_or_else(|| {
                    steady_surface_ctf_state(steady_ctf_w_per_m2_k, initial_zone_air_temperature_c)
                });

            Ok(SurfaceHeatBalanceState {
                surface_id: surface.id,
                zone_id: surface.zone,
                surface_name: surface.name.0.clone(),
                surface_type: surface.surface_type,
                outside_boundary_condition: surface.outside_boundary_condition,
                outside_boundary_condition_object_name: surface
                    .outside_boundary_condition_object
                    .as_ref()
                    .map(|name| name.0.clone()),
                outside_boundary_target_surface_id: boundary.surface_id,
                outside_boundary_target_zone_id: boundary.zone_id,
                construction_id: thermal.construction_id,
                construction_name: thermal.construction_name,
                outside_layer_material_id: thermal.outside_layer_material_id,
                outside_layer_material_name: thermal.outside_layer_material_name,
                outside_layer_roughness: thermal.outside_layer_roughness,
                area_m2,
                azimuth_deg,
                tilt_deg,
                thermal_resistance_m2_k_per_w: thermal.thermal_resistance_m2_k_per_w,
                heat_capacity_j_per_m2_k: thermal.heat_capacity_j_per_m2_k,
                thermal_absorptance: thermal.thermal_absorptance,
                inside_thermal_absorptance: thermal.inside_thermal_absorptance,
                solar_absorptance: thermal.solar_absorptance,
                conductance_w_per_k,
                inside_convection_coefficient_w_per_m2_k:
                    ENERGYPLUS_INITIAL_CONVECTION_COEFFICIENT_W_PER_M2_K,
                inside_convection_input_inside_face_temperature_c: initial_zone_air_temperature_c,
                inside_convection_input_reference_air_temperature_c: initial_zone_air_temperature_c,
                inside_reference_air_temperature_c: initial_zone_air_temperature_c,
                inside_ctf_outside_temperature_c: initial_zone_air_temperature_c,
                inside_radiant_internal_gain_w_per_m2: 0.0,
                inside_shortwave_absorbed_w_per_m2: 0.0,
                inside_additional_heat_source_w_per_m2: 0.0,
                inside_radiant_hvac_w_per_m2: 0.0,
                inside_net_longwave_w_per_m2: 0.0,
                ctf,
                heat_gain_to_zone_w: 0.0,
                outside_report_terms: SurfaceExteriorReportTerms::default(),
                outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics::default(),
                inside_face_temperature_c: initial_zone_air_temperature_c,
                outside_face_temperature_c: initial_zone_air_temperature_c,
            })
        })
        .collect::<Result<Vec<_>, RuntimeError>>()?;
    update_surface_radiant_internal_gain_source_terms(&model.typed, &mut surfaces, 1);

    for zone in &mut zones {
        zone.opaque_surface_conductance_w_per_k = surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone.zone_id)
            .map(|surface| surface.conductance_w_per_k)
            .sum();
        let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
            zone_surface_convection_sums(&surfaces, zone.zone_id);
        zone.sum_ha_w_per_k = sum_ha_w_per_k;
        zone.sum_hat_surf_w = sum_hat_surf_w;
        zone.sum_hat_ref_w = sum_hat_ref_w;
        zone.zone_air_temperature_coefficients = energyplus_zone_air_temperature_coefficients(
            zone.sum_ha_w_per_k,
            zone.sum_hat_surf_w,
            zone.sum_hat_ref_w,
            zone.convective_internal_gain_w,
            0.0,
            0.0,
            zone.air_heat_capacity_j_per_k,
            0.0,
            zone.previous_mean_air_temperatures_c,
        );
    }

    Ok(HeatBalanceState {
        timestep_index: 0,
        zones,
        surfaces,
        last_ctf_history_slot_terms: Vec::new(),
        last_ctf_history_slot_terms_after_advance: Vec::new(),
        last_inside_surface_iteration_count: 0,
        last_inside_surface_iteration_max_delta_c: f64::NAN,
        last_inside_surface_iteration_max_delta_surface_name: None,
    })
}

/// Advances the heat-balance state by one timestep without making a
/// conformance claim.
///
/// This is the first zone-air predictor/corrector-shaped state update. It uses
/// the currently supported opaque surface conductance and internal convective
/// gains while keeping the public zone-temperature comparison diagnostic-only.
pub fn advance_heat_balance_state_one_timestep(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
) {
    advance_heat_balance_state_one_timestep_internal(
        model,
        state,
        input,
        None,
        HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
        1,
        None,
        HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
    );
}

fn advance_heat_balance_state_one_timestep_internal(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
) {
    let hour_ending = input.hour_ending.clamp(1, 24);
    let previous_zone_temperatures = state
        .zones
        .iter()
        .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
        .collect::<BTreeMap<_, _>>();
    let previous_surface_inside_temperatures = state
        .surfaces
        .iter()
        .map(|surface| (surface.surface_id, surface.inside_face_temperature_c))
        .collect::<BTreeMap<_, _>>();
    let previous_surface_outside_temperatures = state
        .surfaces
        .iter()
        .map(|surface| (surface.surface_id, surface.outside_face_temperature_c))
        .collect::<BTreeMap<_, _>>();
    let requested_zone_air_algorithm = zone_air_algorithm;
    let zone_air_algorithm = heat_balance_zone_air_algorithm_execution_variant(zone_air_algorithm);
    let feature_zone_air_algorithm =
        heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);
    let use_energyplus_adaptive_system_timestep_zone_air_correction = matches!(
        requested_zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate
            | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
    );
    let correct_zone_air_after_surface_pass = matches!(
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
    );
    let rebalance_surfaces_after_zone_air_correction = matches!(
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
    );
    let interleave_zone_air_surface_passes = matches!(
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
    );
    let use_previous_inside_for_outdoor_boundary = matches!(
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
    );
    let use_previous_inside_for_adiabatic_boundary = matches!(
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
    );
    let use_quick_outside_conduction = matches!(
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
    );
    let use_doe2_outside_convection =
        heat_balance_uses_doe2_outside_convection(model, feature_zone_air_algorithm);
    let sync_adiabatic_outside_to_current_inside_before_history = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
    );
    let sync_adiabatic_outside_to_current_inside_for_report_only = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
    );
    let commit_adiabatic_current_inside_to_history_only = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    );
    let interior_longwave_exchange_probe = if matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
    ) {
        InteriorLongwaveExchangeProbe::EnergyPlusScriptF
    } else if matches!(
        zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
    ) {
        InteriorLongwaveExchangeProbe::EnergyPlusScriptFFlatAccess
    } else {
        match feature_zone_air_algorithm {
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
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
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe => {
                InteriorLongwaveExchangeProbe::GreyEnergyPlusDirectViewFactor
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe => {
                InteriorLongwaveExchangeProbe::EnergyPlusScriptF
            }
            _ => InteriorLongwaveExchangeProbe::None,
        }
    };

    let preserve_surface_inside_temperature_for_first_longwave =
        heat_balance_preserves_surface_inside_temperature_for_first_longwave(
            feature_zone_air_algorithm,
        );

    for surface in &mut state.surfaces {
        let zone_temperature_c = previous_zone_temperatures
            .get(&surface.zone_id)
            .copied()
            .unwrap_or(surface.inside_face_temperature_c);

        let initial_inside_face_temperature_c =
            if preserve_surface_inside_temperature_for_first_longwave {
                previous_surface_inside_temperatures
                    .get(&surface.surface_id)
                    .copied()
                    .unwrap_or(zone_temperature_c)
            } else {
                zone_temperature_c
            };
        surface.inside_face_temperature_c = initial_inside_face_temperature_c;
        let boundary_balance = heat_balance_surface_boundary_balance(
            model,
            surface,
            &previous_zone_temperatures,
            input.outdoor_dry_bulb_c,
            initial_inside_face_temperature_c,
            weather_context,
            None,
            use_doe2_outside_convection,
        );
        surface.outside_face_temperature_c = boundary_balance.temperature_c;
        surface.outside_report_terms = boundary_balance.exterior_report_terms;
        surface.outside_balance_diagnostics = boundary_balance.outside_balance_diagnostics;
    }
    for zone in &mut state.zones {
        let previous_temperature_c = zone.mean_air_temperature_c;
        let previous_zone_history_temperature_c =
            if use_energyplus_adaptive_system_timestep_zone_air_correction {
                zone.zone_timestep_average_air_temperature_c
            } else {
                previous_temperature_c
            };
        zone.previous_mean_air_temperatures_c = [
            previous_zone_history_temperature_c,
            zone.previous_mean_air_temperatures_c[0],
            zone.previous_mean_air_temperatures_c[1],
        ];
        let previous_humidity_ratio = zone.air_humidity_ratio;
        let previous_zone_history_humidity_ratio =
            if use_energyplus_adaptive_system_timestep_zone_air_correction {
                zone.zone_timestep_average_air_humidity_ratio
            } else {
                previous_humidity_ratio
            };
        zone.previous_air_humidity_ratios = [
            previous_zone_history_humidity_ratio,
            zone.previous_air_humidity_ratios[0],
            zone.previous_air_humidity_ratios[1],
        ];
        zone.convective_internal_gain_w =
            convective_internal_gain_w(model, zone.zone_id, hour_ending);

        let conductance_w_per_k = state
            .surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone.zone_id)
            .map(|surface| surface.conductance_w_per_k)
            .sum::<f64>();
        let conductance_weighted_outside_temperature = state
            .surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone.zone_id)
            .map(|surface| surface.conductance_w_per_k * surface.outside_face_temperature_c)
            .sum::<f64>();
        let equivalent_outside_temperature_c = if conductance_w_per_k > 0.0 {
            conductance_weighted_outside_temperature / conductance_w_per_k
        } else {
            previous_temperature_c
        };

        zone.opaque_surface_conductance_w_per_k = conductance_w_per_k;
        zone.mean_air_temperature_c = match feature_zone_air_algorithm {
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical => step_zone_air_temperature(
                previous_temperature_c,
                equivalent_outside_temperature_c,
                zone.convective_internal_gain_w,
                conductance_w_per_k,
                zone.air_heat_capacity_j_per_k,
                input.timestep_seconds,
            ),
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe => previous_temperature_c,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe => {
                previous_temperature_c
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe => {
                let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
                    zone_surface_convection_sums(&state.surfaces, zone.zone_id);
                let coefficients = energyplus_zone_air_temperature_coefficients(
                    sum_ha_w_per_k,
                    sum_hat_surf_w,
                    sum_hat_ref_w,
                    zone.convective_internal_gain_w,
                    0.0,
                    0.0,
                    zone.air_heat_capacity_j_per_k,
                    input.timestep_seconds,
                    zone.previous_mean_air_temperatures_c,
                );
                energyplus_analytical_zone_air_temperature_c(
                    previous_temperature_c,
                    coefficients.temp_independent_coefficient_w,
                    coefficients.temp_dependent_coefficient_w_per_k,
                    zone.air_heat_capacity_j_per_k,
                    input.timestep_seconds,
                )
            }
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe => {
                let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
                    zone_surface_convection_sums(&state.surfaces, zone.zone_id);
                let coefficients = energyplus_zone_air_temperature_coefficients(
                    sum_ha_w_per_k,
                    sum_hat_surf_w,
                    sum_hat_ref_w,
                    zone.convective_internal_gain_w,
                    0.0,
                    0.0,
                    zone.air_heat_capacity_j_per_k,
                    input.timestep_seconds,
                    zone.previous_mean_air_temperatures_c,
                );
                energyplus_third_order_zone_air_temperature_from_coefficients(
                    previous_temperature_c,
                    coefficients,
                )
            }
        };
    }
    update_surface_radiant_internal_gain_source_terms(model, &mut state.surfaces, hour_ending);
    let use_current_inside_for_first_longwave = matches!(
        feature_zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    );
    let converge_interleaved_surface_iterations_to_energyplus_tolerance = matches!(
        feature_zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
    );
    let freeze_outside_balance_for_surface_iterations = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
    );
    let freeze_inside_ctf_outside_temperature_for_surface_iterations = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
    );
    let use_inside_ctf_outside_temperature_for_conduction_report = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
    );
    let commit_inside_ctf_outside_temperature_to_history = matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryCommitProbe
    );

    let interleaved_surface_zone_balance_result = if interleave_zone_air_surface_passes {
        Some(run_interleaved_surface_zone_balance(
            model,
            &mut state.surfaces,
            &mut state.zones,
            Some(&previous_surface_inside_temperatures),
            input,
            weather_context,
            surface_iteration_count,
            use_previous_inside_for_outdoor_boundary,
            use_previous_inside_for_adiabatic_boundary,
            use_quick_outside_conduction,
            Some(&previous_surface_outside_temperatures),
            use_doe2_outside_convection,
            interior_longwave_exchange_probe,
            matches!(
                feature_zone_air_algorithm,
                HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
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
            ) && !matches!(
                zone_air_algorithm,
                HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
            ),
            use_current_inside_for_first_longwave,
            heat_balance_uses_third_order_zone_air_correction(feature_zone_air_algorithm),
            matches!(
                feature_zone_air_algorithm,
                HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
                    | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
            ) && !matches!(
                zone_air_algorithm,
                HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
            ),
            converge_interleaved_surface_iterations_to_energyplus_tolerance,
            freeze_outside_balance_for_surface_iterations,
            freeze_inside_ctf_outside_temperature_for_surface_iterations,
            use_inside_ctf_outside_temperature_for_conduction_report,
            inside_hconv_reevaluation_interval,
            surface_loop_zone_air_correction,
        ))
    } else {
        let current_zone_temperatures = state
            .zones
            .iter()
            .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
            .collect::<BTreeMap<_, _>>();
        run_surface_balance_passes(
            model,
            &mut state.surfaces,
            Some(&previous_surface_inside_temperatures),
            Some(&previous_surface_inside_temperatures),
            None,
            &current_zone_temperatures,
            input,
            weather_context,
            surface_iteration_count,
            use_previous_inside_for_outdoor_boundary,
            use_previous_inside_for_adiabatic_boundary,
            use_quick_outside_conduction,
            if use_quick_outside_conduction {
                Some(&previous_surface_outside_temperatures)
            } else {
                None
            },
            use_doe2_outside_convection,
            interior_longwave_exchange_probe,
            None,
            None,
            None,
            None,
            false,
        );

        if rebalance_surfaces_after_zone_air_correction {
            correct_zone_air_temperatures_from_current_surfaces(
                &state.surfaces,
                &mut state.zones,
                input.timestep_seconds,
                weather_context,
                input.outdoor_dry_bulb_c,
                true,
                heat_balance_uses_third_order_zone_air_correction(feature_zone_air_algorithm),
                false,
            );
            let corrected_zone_temperatures = state
                .zones
                .iter()
                .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
                .collect::<BTreeMap<_, _>>();
            run_surface_balance_passes(
                model,
                &mut state.surfaces,
                None,
                None,
                None,
                &corrected_zone_temperatures,
                input,
                weather_context,
                surface_iteration_count,
                use_previous_inside_for_outdoor_boundary,
                use_previous_inside_for_adiabatic_boundary,
                use_quick_outside_conduction,
                if use_quick_outside_conduction {
                    Some(&previous_surface_outside_temperatures)
                } else {
                    None
                },
                use_doe2_outside_convection,
                interior_longwave_exchange_probe,
                None,
                None,
                None,
                None,
                false,
            );
        }
        None
    };

    if interleave_zone_air_surface_passes
        && matches!(
            surface_loop_zone_air_correction,
            HeatBalanceSurfaceLoopZoneAirCorrection::AfterSurfaceLoop
        )
    {
        correct_zone_air_temperatures_from_current_surfaces(
            &state.surfaces,
            &mut state.zones,
            input.timestep_seconds,
            weather_context,
            input.outdoor_dry_bulb_c,
            true,
            heat_balance_uses_third_order_zone_air_correction(feature_zone_air_algorithm),
            use_inside_ctf_outside_temperature_for_conduction_report,
        );
    }

    if sync_adiabatic_outside_to_current_inside_before_history {
        sync_adiabatic_outside_faces_to_inside_faces(&mut state.surfaces);
    }
    let adiabatic_report_history_outside_temperature_snapshots =
        if sync_adiabatic_outside_to_current_inside_for_report_only {
            let snapshots = state
                .surfaces
                .iter()
                .filter(|surface| {
                    surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic
                })
                .map(|surface| (surface.surface_id, surface.outside_face_temperature_c))
                .collect::<BTreeMap<_, _>>();
            sync_adiabatic_outside_faces_to_inside_faces(&mut state.surfaces);
            Some(snapshots)
        } else {
            None
        };

    state.last_ctf_history_slot_terms = heat_balance_ctf_history_slot_samples(&state.surfaces);
    let inside_ctf_outside_temperature_history_commit_snapshots =
        interleaved_surface_zone_balance_result
            .as_ref()
            .and_then(|result| result.inside_ctf_outside_temperature_snapshots.as_ref());
    for surface in &mut state.surfaces {
        if commit_adiabatic_current_inside_to_history_only
            && surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic
        {
            advance_surface_ctf_histories_with_outside_temperature_override(
                surface,
                Some(surface.inside_face_temperature_c),
            );
        } else if let Some(outside_temperature_c) =
            adiabatic_report_history_outside_temperature_snapshots
                .as_ref()
                .and_then(|snapshots| snapshots.get(&surface.surface_id).copied())
        {
            advance_surface_ctf_histories_with_outside_temperature_override(
                surface,
                Some(outside_temperature_c),
            );
        } else if let Some(outside_temperature_c) =
            inside_ctf_outside_temperature_history_commit_override_c(
                surface,
                commit_inside_ctf_outside_temperature_to_history,
                inside_ctf_outside_temperature_history_commit_snapshots,
            )
        {
            advance_surface_ctf_histories_with_outside_temperature_override(
                surface,
                Some(outside_temperature_c),
            );
        } else {
            advance_surface_ctf_histories(surface);
        }
    }
    state.last_ctf_history_slot_terms_after_advance =
        heat_balance_ctf_history_slot_samples(&state.surfaces);

    correct_zone_air_temperatures_from_current_surfaces(
        &state.surfaces,
        &mut state.zones,
        input.timestep_seconds,
        weather_context,
        input.outdoor_dry_bulb_c,
        correct_zone_air_after_surface_pass && !interleave_zone_air_surface_passes,
        heat_balance_uses_third_order_zone_air_correction(feature_zone_air_algorithm),
        use_inside_ctf_outside_temperature_for_conduction_report,
    );
    correct_zone_air_humidity_ratios_from_current_state(
        &mut state.zones,
        input.timestep_seconds,
        weather_context,
        heat_balance_uses_third_order_zone_air_correction(feature_zone_air_algorithm),
    );
    if use_energyplus_adaptive_system_timestep_zone_air_correction {
        apply_energyplus_adaptive_system_timestep_zone_air_correction(
            &state.surfaces,
            &mut state.zones,
            input.timestep_seconds,
            weather_context,
            input.outdoor_dry_bulb_c,
            use_inside_ctf_outside_temperature_for_conduction_report,
        );
    } else {
        for zone in &mut state.zones {
            zone.zone_timestep_average_air_temperature_c = zone.mean_air_temperature_c;
            zone.zone_timestep_average_air_humidity_ratio = zone.air_humidity_ratio;
            synchronize_single_system_timestep_history(zone);
            zone.system_timestep_average_surface_convection_report_w = None;
            zone.system_timestep_average_air_storage_report_w = None;
        }
    }
    state.last_inside_surface_iteration_count = interleaved_surface_zone_balance_result
        .as_ref()
        .map(|result| result.inside_surface_iteration_count)
        .unwrap_or_else(|| surface_iteration_count.max(1));
    state.last_inside_surface_iteration_max_delta_c = interleaved_surface_zone_balance_result
        .as_ref()
        .map(|result| result.max_inside_surface_delta_c)
        .unwrap_or(f64::NAN);
    state.last_inside_surface_iteration_max_delta_surface_name =
        interleaved_surface_zone_balance_result
            .as_ref()
            .and_then(|result| result.max_delta_surface_name.clone());
    state.timestep_index += 1;
}

#[derive(Default)]
struct InterleavedSurfaceZoneBalanceResult {
    inside_ctf_outside_temperature_snapshots: Option<BTreeMap<SurfaceId, f64>>,
    inside_surface_iteration_count: u32,
    max_inside_surface_delta_c: f64,
    max_delta_surface_name: Option<String>,
}

fn run_interleaved_surface_zone_balance(
    model: &TypedModel,
    surfaces: &mut [SurfaceHeatBalanceState],
    zones: &mut [ZoneHeatBalanceState],
    first_pass_inside_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    surface_iteration_count: u32,
    use_previous_inside_for_outdoor_boundary: bool,
    use_previous_inside_for_adiabatic_boundary: bool,
    use_quick_outside_conduction: bool,
    exterior_coefficient_surface_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    use_doe2_outside_convection: bool,
    interior_longwave_exchange_probe: InteriorLongwaveExchangeProbe,
    freeze_inside_convection_for_timestep: bool,
    use_current_inside_for_first_longwave: bool,
    use_third_order_zone_air_correction: bool,
    freeze_surface_reference_air_for_timestep: bool,
    converge_surface_iterations_to_energyplus_tolerance: bool,
    freeze_outside_balance_for_surface_iterations: bool,
    freeze_inside_ctf_outside_temperature_for_surface_iterations: bool,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
) -> InterleavedSurfaceZoneBalanceResult {
    let inside_hconv_reevaluation_interval =
        inside_hconv_reevaluation_interval.filter(|interval| *interval > 0);
    let mut inside_convection_coefficients =
        if freeze_inside_convection_for_timestep || inside_hconv_reevaluation_interval.is_some() {
            let zone_temperatures = heat_balance_zone_temperature_map(zones);
            Some(heat_balance_inside_convection_coefficients(
                surfaces,
                &zone_temperatures,
                first_pass_inside_temperatures,
            ))
        } else {
            None
        };
    let mut inside_convection_coefficient_inputs =
        if freeze_inside_convection_for_timestep || inside_hconv_reevaluation_interval.is_some() {
            let zone_temperatures = heat_balance_zone_temperature_map(zones);
            Some(heat_balance_inside_convection_coefficient_inputs(
                surfaces,
                &zone_temperatures,
                first_pass_inside_temperatures,
            ))
        } else {
            None
        };
    let frozen_surface_reference_air_temperatures = if freeze_surface_reference_air_for_timestep {
        Some(
            zones
                .iter()
                .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
                .collect::<BTreeMap<_, _>>(),
        )
    } else {
        None
    };
    let mut frozen_outside_boundary_balances: Option<
        BTreeMap<SurfaceId, SurfaceBoundaryBalanceResult>,
    > = None;
    let mut frozen_inside_ctf_outside_temperatures: Option<BTreeMap<SurfaceId, f64>> = None;
    let mut inside_surface_iteration_count = 0;
    let mut final_max_inside_surface_delta_c = f64::NAN;
    let mut final_max_delta_surface_name = None;

    for surface_iteration_index in 0..surface_iteration_count.max(1) {
        inside_surface_iteration_count = surface_iteration_index + 1;
        let pass_start_inside_temperatures = if converge_surface_iterations_to_energyplus_tolerance
        {
            Some(
                surfaces
                    .iter()
                    .map(|surface| (surface.surface_id, surface.inside_face_temperature_c))
                    .collect::<BTreeMap<_, _>>(),
            )
        } else {
            None
        };
        let current_zone_temperatures = heat_balance_zone_temperature_map(zones);
        if let Some(interval) = inside_hconv_reevaluation_interval {
            if surface_iteration_index > 0 && surface_iteration_index % interval == 0 {
                inside_convection_coefficients = Some(heat_balance_inside_convection_coefficients(
                    surfaces,
                    &current_zone_temperatures,
                    None,
                ));
                inside_convection_coefficient_inputs =
                    Some(heat_balance_inside_convection_coefficient_inputs(
                        surfaces,
                        &current_zone_temperatures,
                        None,
                    ));
            }
        }
        let zone_temperatures = frozen_surface_reference_air_temperatures
            .as_ref()
            .unwrap_or(&current_zone_temperatures);
        let first_pass_temperatures = if surface_iteration_index == 0 {
            first_pass_inside_temperatures
        } else {
            None
        };
        let first_pass_longwave_temperatures = if use_current_inside_for_first_longwave {
            None
        } else {
            first_pass_temperatures
        };
        run_surface_balance_passes(
            model,
            surfaces,
            first_pass_temperatures,
            first_pass_longwave_temperatures,
            // EnergyPlus sets regular adiabatic/partition outside-face CTF state
            // during the outside balance before the inside surface loop, then
            // reports outside flux from that frozen state in UpdateThermalHistories.
            if use_previous_inside_for_adiabatic_boundary {
                first_pass_inside_temperatures
            } else {
                None
            },
            zone_temperatures,
            input,
            weather_context,
            1,
            use_previous_inside_for_outdoor_boundary,
            use_previous_inside_for_adiabatic_boundary,
            use_quick_outside_conduction,
            exterior_coefficient_surface_temperatures,
            use_doe2_outside_convection,
            interior_longwave_exchange_probe,
            inside_convection_coefficients.as_ref(),
            inside_convection_coefficient_inputs.as_ref(),
            frozen_outside_boundary_balances.as_ref(),
            frozen_inside_ctf_outside_temperatures.as_ref(),
            use_inside_ctf_outside_temperature_for_conduction_report,
        );
        if freeze_outside_balance_for_surface_iterations
            && frozen_outside_boundary_balances.is_none()
        {
            frozen_outside_boundary_balances = Some(
                surfaces
                    .iter()
                    .map(|surface| {
                        (
                            surface.surface_id,
                            SurfaceBoundaryBalanceResult {
                                temperature_c: surface.outside_face_temperature_c,
                                exterior_report_terms: surface.outside_report_terms,
                                outside_balance_diagnostics: surface.outside_balance_diagnostics,
                            },
                        )
                    })
                    .collect(),
            );
        }
        if freeze_inside_ctf_outside_temperature_for_surface_iterations
            && frozen_inside_ctf_outside_temperatures.is_none()
        {
            frozen_inside_ctf_outside_temperatures = Some(
                surfaces
                    .iter()
                    .map(|surface| (surface.surface_id, surface.outside_face_temperature_c))
                    .collect(),
            );
        }
        let (max_inside_surface_delta_c, max_delta_surface_name) = pass_start_inside_temperatures
            .as_ref()
            .map(|temperatures| {
                surfaces.iter().fold((0.0, None), |best, surface| {
                    let Some(previous) = temperatures.get(&surface.surface_id) else {
                        return best;
                    };
                    let delta = (surface.inside_face_temperature_c - previous).abs();
                    if delta > best.0 {
                        (delta, Some(surface.surface_name.clone()))
                    } else {
                        best
                    }
                })
            })
            .unwrap_or((f64::INFINITY, None));
        final_max_inside_surface_delta_c = max_inside_surface_delta_c;
        final_max_delta_surface_name = max_delta_surface_name;
        if matches!(
            surface_loop_zone_air_correction,
            HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration
        ) {
            correct_zone_air_temperatures_from_current_surfaces(
                surfaces,
                zones,
                input.timestep_seconds,
                weather_context,
                input.outdoor_dry_bulb_c,
                true,
                use_third_order_zone_air_correction,
                use_inside_ctf_outside_temperature_for_conduction_report,
            );
        }
        if converge_surface_iterations_to_energyplus_tolerance
            && max_inside_surface_delta_c <= ENERGYPLUS_MAX_ALLOWED_INSIDE_SURFACE_DELTA_C
        {
            break;
        }
    }

    InterleavedSurfaceZoneBalanceResult {
        inside_ctf_outside_temperature_snapshots: frozen_inside_ctf_outside_temperatures,
        inside_surface_iteration_count,
        max_inside_surface_delta_c: final_max_inside_surface_delta_c,
        max_delta_surface_name: final_max_delta_surface_name,
    }
}

fn run_surface_balance_passes(
    model: &TypedModel,
    surfaces: &mut [SurfaceHeatBalanceState],
    first_pass_inside_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    first_pass_longwave_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    adiabatic_boundary_inside_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    zone_temperatures: &BTreeMap<ZoneId, f64>,
    input: HeatBalanceStepInput,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    surface_iteration_count: u32,
    use_previous_inside_for_outdoor_boundary: bool,
    use_previous_inside_for_adiabatic_boundary: bool,
    use_quick_outside_conduction: bool,
    exterior_coefficient_surface_temperatures: Option<&BTreeMap<SurfaceId, f64>>,
    use_doe2_outside_convection: bool,
    interior_longwave_exchange_probe: InteriorLongwaveExchangeProbe,
    inside_convection_coefficient_overrides: Option<&BTreeMap<SurfaceId, f64>>,
    inside_convection_coefficient_input_overrides: Option<
        &BTreeMap<SurfaceId, InsideConvectionCoefficientInputState>,
    >,
    outside_boundary_balance_snapshots: Option<&BTreeMap<SurfaceId, SurfaceBoundaryBalanceResult>>,
    inside_ctf_outside_temperature_snapshots: Option<&BTreeMap<SurfaceId, f64>>,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) {
    for surface_iteration_index in 0..surface_iteration_count.max(1) {
        let temperature_overrides = if surface_iteration_index == 0 {
            first_pass_longwave_temperatures
        } else {
            None
        };
        match interior_longwave_exchange_probe {
            InteriorLongwaveExchangeProbe::None => {}
            InteriorLongwaveExchangeProbe::GreyEnergyPlusDirectViewFactor => {
                update_surface_inside_longwave_exchange_probe(surfaces, temperature_overrides);
            }
            InteriorLongwaveExchangeProbe::EnergyPlusScriptF => {
                update_surface_inside_scriptf_longwave_exchange_probe(
                    surfaces,
                    temperature_overrides,
                );
            }
            InteriorLongwaveExchangeProbe::EnergyPlusScriptFFlatAccess => {
                update_surface_inside_scriptf_flat_access_longwave_exchange_probe(
                    surfaces,
                    temperature_overrides,
                );
            }
        }
        for surface in surfaces.iter_mut() {
            let previous_inside_face_temperature_c = if surface_iteration_index == 0 {
                first_pass_inside_temperatures
                    .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                    .unwrap_or(surface.inside_face_temperature_c)
            } else {
                surface.inside_face_temperature_c
            };
            let zone_temperature_c = zone_temperatures
                .get(&surface.zone_id)
                .copied()
                .unwrap_or(surface.inside_face_temperature_c);
            surface.inside_reference_air_temperature_c = zone_temperature_c;
            let inside_convection_coefficient_w_per_m2_k = inside_convection_coefficient_overrides
                .and_then(|coefficients| coefficients.get(&surface.surface_id).copied())
                .unwrap_or_else(|| {
                    energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
                        surface,
                        previous_inside_face_temperature_c,
                        zone_temperature_c,
                    )
                });
            surface.inside_convection_coefficient_w_per_m2_k =
                inside_convection_coefficient_w_per_m2_k;
            let inside_convection_input = inside_convection_coefficient_input_overrides
                .and_then(|inputs| inputs.get(&surface.surface_id).copied())
                .unwrap_or(InsideConvectionCoefficientInputState {
                    inside_face_temperature_c: previous_inside_face_temperature_c,
                    reference_air_temperature_c: zone_temperature_c,
                });
            surface.inside_convection_input_inside_face_temperature_c =
                inside_convection_input.inside_face_temperature_c;
            surface.inside_convection_input_reference_air_temperature_c =
                inside_convection_input.reference_air_temperature_c;

            update_surface_ctf_history_constants(surface);
            let use_previous_inside_for_boundary = (use_previous_inside_for_outdoor_boundary
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors)
                || (use_previous_inside_for_adiabatic_boundary
                    && surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic);
            let outside_balance_inside_temperature_c = if use_previous_inside_for_adiabatic_boundary
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic
            {
                adiabatic_boundary_inside_temperatures
                    .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                    .unwrap_or(previous_inside_face_temperature_c)
            } else if use_previous_inside_for_boundary {
                previous_inside_face_temperature_c
            } else {
                zone_temperature_c
            };
            surface.inside_face_temperature_c = outside_balance_inside_temperature_c;
            let net_inside_source_w_per_m2 = surface_inside_ctf_source_terms_w_per_m2(surface);
            let quick_outside_conduction = if use_quick_outside_conduction
                && surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors
            {
                Some(QuickOutsideConductionContext {
                    reference_air_temperature_c: zone_temperature_c,
                    inside_convection_coefficient_w_per_m2_k:
                        inside_convection_coefficient_w_per_m2_k,
                    net_inside_source_w_per_m2,
                    exterior_coefficient_surface_temperature_c:
                        exterior_coefficient_surface_temperatures
                            .and_then(|temperatures| temperatures.get(&surface.surface_id))
                            .copied(),
                    use_doe2_outside_convection,
                })
            } else {
                None
            };
            let boundary_balance = outside_boundary_balance_snapshots
                .and_then(|snapshots| snapshots.get(&surface.surface_id).copied())
                .unwrap_or_else(|| {
                    heat_balance_surface_boundary_balance(
                        model,
                        surface,
                        zone_temperatures,
                        input.outdoor_dry_bulb_c,
                        outside_balance_inside_temperature_c,
                        weather_context,
                        quick_outside_conduction,
                        use_doe2_outside_convection,
                    )
                });
            surface.outside_face_temperature_c = boundary_balance.temperature_c;
            surface.outside_report_terms = boundary_balance.exterior_report_terms;
            surface.outside_balance_diagnostics = boundary_balance.outside_balance_diagnostics;
            let inside_ctf_outside_temperature_c = inside_ctf_outside_temperature_snapshots
                .and_then(|snapshots| snapshots.get(&surface.surface_id).copied());
            surface.inside_ctf_outside_temperature_c =
                inside_ctf_outside_temperature_c.unwrap_or(surface.outside_face_temperature_c);
            surface.inside_face_temperature_c =
                energyplus_ctf_inside_face_temperature_c_with_outside_temperature(
                    surface,
                    CtfInsideFaceBalanceInput {
                        reference_air_temperature_c: zone_temperature_c,
                        inside_convection_coefficient_w_per_m2_k,
                        previous_inside_face_temperature_c,
                        net_inside_source_w_per_m2,
                    },
                    inside_ctf_outside_temperature_c,
                );
            if surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic
                && !use_previous_inside_for_adiabatic_boundary
            {
                surface.outside_face_temperature_c = surface.inside_face_temperature_c;
                surface.inside_ctf_outside_temperature_c = surface.inside_face_temperature_c;
            }
            surface.heat_gain_to_zone_w = surface_inside_conduction_rate_w_for_report(
                surface,
                use_inside_ctf_outside_temperature_for_conduction_report,
            );
        }
    }
}

fn heat_balance_zone_temperature_map(zones: &[ZoneHeatBalanceState]) -> BTreeMap<ZoneId, f64> {
    zones
        .iter()
        .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
        .collect()
}

fn heat_balance_inside_convection_coefficients(
    surfaces: &[SurfaceHeatBalanceState],
    zone_temperatures: &BTreeMap<ZoneId, f64>,
    inside_surface_temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) -> BTreeMap<SurfaceId, f64> {
    surfaces
        .iter()
        .map(|surface| {
            let inside_face_temperature_c = inside_surface_temperature_overrides
                .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                .unwrap_or(surface.inside_face_temperature_c);
            let zone_temperature_c = zone_temperatures
                .get(&surface.zone_id)
                .copied()
                .unwrap_or(surface.inside_face_temperature_c);
            (
                surface.surface_id,
                energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
                    surface,
                    inside_face_temperature_c,
                    zone_temperature_c,
                ),
            )
        })
        .collect()
}

fn heat_balance_inside_convection_coefficient_inputs(
    surfaces: &[SurfaceHeatBalanceState],
    zone_temperatures: &BTreeMap<ZoneId, f64>,
    inside_surface_temperature_overrides: Option<&BTreeMap<SurfaceId, f64>>,
) -> BTreeMap<SurfaceId, InsideConvectionCoefficientInputState> {
    surfaces
        .iter()
        .map(|surface| {
            let inside_face_temperature_c = inside_surface_temperature_overrides
                .and_then(|temperatures| temperatures.get(&surface.surface_id).copied())
                .unwrap_or(surface.inside_face_temperature_c);
            let reference_air_temperature_c = zone_temperatures
                .get(&surface.zone_id)
                .copied()
                .unwrap_or(surface.inside_face_temperature_c);
            (
                surface.surface_id,
                InsideConvectionCoefficientInputState {
                    inside_face_temperature_c,
                    reference_air_temperature_c,
                },
            )
        })
        .collect()
}

fn correct_zone_air_temperatures_from_current_surfaces(
    surfaces: &[SurfaceHeatBalanceState],
    zones: &mut [ZoneHeatBalanceState],
    timestep_seconds: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    fallback_dry_bulb_c: f64,
    update_mean_air_temperature: bool,
    use_third_order_zone_air_correction: bool,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) {
    update_zone_air_heat_capacities_from_weather_context(
        zones,
        weather_context,
        fallback_dry_bulb_c,
    );

    for zone in zones {
        zone.opaque_surface_heat_gain_w = surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone.zone_id)
            .map(|surface| surface.heat_gain_to_zone_w)
            .sum();
        zone.opaque_surface_outside_conduction_w = surfaces
            .iter()
            .filter(|surface| surface.zone_id == zone.zone_id)
            .map(|surface| {
                surface_outside_conduction_rate_w_for_report(
                    surface,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                )
            })
            .sum();
        let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
            zone_surface_convection_sums(surfaces, zone.zone_id);
        zone.sum_ha_w_per_k = sum_ha_w_per_k;
        zone.sum_hat_surf_w = sum_hat_surf_w;
        zone.sum_hat_ref_w = sum_hat_ref_w;
        let coefficients = energyplus_zone_air_temperature_coefficients(
            zone.sum_ha_w_per_k,
            zone.sum_hat_surf_w,
            zone.sum_hat_ref_w,
            zone.convective_internal_gain_w,
            0.0,
            0.0,
            zone.air_heat_capacity_j_per_k,
            timestep_seconds,
            zone.previous_mean_air_temperatures_c,
        );
        if update_mean_air_temperature {
            zone.mean_air_temperature_c = if use_third_order_zone_air_correction {
                energyplus_third_order_zone_air_temperature_from_coefficients(
                    zone.previous_mean_air_temperatures_c[0],
                    coefficients,
                )
            } else {
                energyplus_analytical_zone_air_temperature_c(
                    zone.previous_mean_air_temperatures_c[0],
                    coefficients.temp_independent_coefficient_w,
                    coefficients.temp_dependent_coefficient_w_per_k,
                    zone.air_heat_capacity_j_per_k,
                    timestep_seconds,
                )
            };
        }
        zone.zone_air_temperature_coefficients = coefficients;
    }
}

fn correct_zone_air_humidity_ratios_from_current_state(
    zones: &mut [ZoneHeatBalanceState],
    timestep_seconds: f64,
    context: Option<HeatBalanceWeatherContext<'_>>,
    use_third_order_zone_air_correction: bool,
) {
    for zone in zones {
        let humidity_ratio = if use_third_order_zone_air_correction {
            let history_term = 3.0 * zone.previous_air_humidity_ratios[0]
                - (3.0 / 2.0) * zone.previous_air_humidity_ratios[1]
                + (1.0 / 3.0) * zone.previous_air_humidity_ratios[2];
            history_term / (11.0 / 6.0)
        } else {
            zone.previous_air_humidity_ratios[0]
        };
        let saturation_humidity_ratio = context
            .and_then(|context| {
                let atmospheric_pressure_pa = energyplus_weather_atmospheric_pressure_for_context(
                    context,
                    context
                        .records
                        .get(context.record_index)?
                        .atmospheric_pressure_pa,
                );
                energyplus_psychrometric_humidity_ratio_from_rh(
                    zone.mean_air_temperature_c,
                    1.0,
                    atmospheric_pressure_pa,
                )
            })
            .or_else(|| {
                energyplus_psychrometric_humidity_ratio_from_rh(
                    zone.mean_air_temperature_c,
                    1.0,
                    ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA,
                )
            })
            .unwrap_or(f64::INFINITY);
        zone.air_humidity_ratio = humidity_ratio
            .clamp(0.0, saturation_humidity_ratio)
            .max(ENERGYPLUS_MIN_HUMIDITY_RATIO);
        if timestep_seconds <= 0.0 || !zone.air_humidity_ratio.is_finite() {
            zone.air_humidity_ratio = ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO;
        }
    }
}

fn synchronize_single_system_timestep_history(zone: &mut ZoneHeatBalanceState) {
    zone.previous_system_mean_air_temperatures_c = [
        zone.mean_air_temperature_c,
        zone.previous_mean_air_temperatures_c[0],
        zone.previous_mean_air_temperatures_c[1],
    ];
    zone.previous_system_air_humidity_ratios = [
        zone.air_humidity_ratio,
        zone.previous_air_humidity_ratios[0],
        zone.previous_air_humidity_ratios[1],
    ];
    zone.previous_system_timestep_count = 1;
}
fn apply_energyplus_adaptive_system_timestep_zone_air_correction(
    surfaces: &[SurfaceHeatBalanceState],
    zones: &mut [ZoneHeatBalanceState],
    zone_timestep_seconds: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    fallback_dry_bulb_c: f64,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) {
    if zone_timestep_seconds <= 0.0 {
        return;
    }

    let limit_system_timestep_count = (zone_timestep_seconds
        / ENERGYPLUS_MIN_SYSTEM_TIMESTEP_SECONDS)
        .floor()
        .max(1.0) as u32;

    for zone in zones {
        let zone_temp_change_c =
            (zone.mean_air_temperature_c - zone.previous_mean_air_temperatures_c[0]).abs();
        let system_timestep_count = if zone_temp_change_c > ENERGYPLUS_MAX_ZONE_TEMP_DIFF_C {
            let requested = (zone_temp_change_c / ENERGYPLUS_MAX_ZONE_TEMP_DIFF_C + 1.0) as u32;
            requested.clamp(1, limit_system_timestep_count)
        } else {
            1
        };

        if system_timestep_count <= 1 {
            zone.zone_timestep_average_air_temperature_c = zone.mean_air_temperature_c;
            zone.zone_timestep_average_air_humidity_ratio = zone.air_humidity_ratio;
            synchronize_single_system_timestep_history(zone);
            zone.system_timestep_average_surface_convection_report_w = None;
            zone.system_timestep_average_air_storage_report_w = None;
            continue;
        }

        let system_timestep_seconds = zone_timestep_seconds / f64::from(system_timestep_count);
        let mut system_temperature_history =
            if system_timestep_count == zone.previous_system_timestep_count {
                zone.previous_system_mean_air_temperatures_c
            } else {
                energyplus_down_interpolate_three_history_values(
                    zone_timestep_seconds,
                    system_timestep_seconds,
                    zone.previous_mean_air_temperatures_c,
                )
            };
        let mut system_humidity_history =
            if system_timestep_count == zone.previous_system_timestep_count {
                zone.previous_system_air_humidity_ratios
            } else {
                energyplus_down_interpolate_three_history_values(
                    zone_timestep_seconds,
                    system_timestep_seconds,
                    zone.previous_air_humidity_ratios,
                )
            };

        let reset_zone_air_state_from_system_history =
            system_timestep_count != zone.previous_system_timestep_count;
        if reset_zone_air_state_from_system_history {
            zone.mean_air_temperature_c = system_temperature_history[0];
            zone.air_humidity_ratio = system_humidity_history[0];
        }
        let mut zone_temperature_average_c = 0.0;
        let mut zone_humidity_average = 0.0;
        let mut surface_convection_report_average_w = 0.0;
        let mut air_storage_report_average_w = 0.0;
        let system_timestep_fraction = 1.0 / f64::from(system_timestep_count);

        for _ in 0..system_timestep_count {
            correct_single_zone_air_temperature_from_current_surfaces(
                surfaces,
                zone,
                system_timestep_seconds,
                system_temperature_history,
                weather_context,
                fallback_dry_bulb_c,
                use_inside_ctf_outside_temperature_for_conduction_report,
            );
            correct_single_zone_air_humidity_ratio_from_history(
                zone,
                system_timestep_seconds,
                system_humidity_history,
                weather_context,
            );
            let air_storage_rate_w = zone_air_system_timestep_storage_report_rate_w(
                zone,
                system_temperature_history[0],
                system_timestep_seconds,
                weather_context,
                fallback_dry_bulb_c,
            );
            let surface_convection_rate_w = zone_air_heat_balance_surface_convection_rate_w(zone);
            zone_temperature_average_c += zone.mean_air_temperature_c * system_timestep_fraction;
            zone_humidity_average += zone.air_humidity_ratio * system_timestep_fraction;
            surface_convection_report_average_w +=
                surface_convection_rate_w * system_timestep_fraction;
            air_storage_report_average_w += air_storage_rate_w * system_timestep_fraction;
            system_temperature_history = [
                zone.mean_air_temperature_c,
                system_temperature_history[0],
                system_temperature_history[1],
            ];
            system_humidity_history = [
                zone.air_humidity_ratio,
                system_humidity_history[0],
                system_humidity_history[1],
            ];
        }

        zone.zone_timestep_average_air_temperature_c = zone_temperature_average_c;
        zone.zone_timestep_average_air_humidity_ratio = zone_humidity_average;
        zone.previous_system_mean_air_temperatures_c = system_temperature_history;
        zone.previous_system_air_humidity_ratios = system_humidity_history;
        zone.previous_system_timestep_count = system_timestep_count;
        zone.system_timestep_average_surface_convection_report_w =
            Some(surface_convection_report_average_w);
        zone.system_timestep_average_air_storage_report_w = Some(air_storage_report_average_w);
    }
}

fn zone_air_system_timestep_storage_report_rate_w(
    zone: &ZoneHeatBalanceState,
    previous_system_temperature_c: f64,
    system_timestep_seconds: f64,
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    fallback_dry_bulb_c: f64,
) -> f64 {
    if system_timestep_seconds <= 0.0 {
        return 0.0;
    }

    let report_air_heat_capacity_j_per_k =
        weather_proxy_zone_air_heat_capacity_j_per_k(zone, weather_context, fallback_dry_bulb_c)
            .unwrap_or(zone.air_heat_capacity_j_per_k);
    report_air_heat_capacity_j_per_k * (zone.mean_air_temperature_c - previous_system_temperature_c)
        / system_timestep_seconds
}

fn correct_single_zone_air_temperature_from_current_surfaces(
    surfaces: &[SurfaceHeatBalanceState],
    zone: &mut ZoneHeatBalanceState,
    timestep_seconds: f64,
    previous_mean_air_temperatures_c: [f64; 3],
    weather_context: Option<HeatBalanceWeatherContext<'_>>,
    fallback_dry_bulb_c: f64,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) {
    update_single_zone_air_heat_capacity_from_weather_context(
        zone,
        weather_context,
        fallback_dry_bulb_c,
    );
    zone.opaque_surface_heat_gain_w = surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone.zone_id)
        .map(|surface| surface.heat_gain_to_zone_w)
        .sum();
    zone.opaque_surface_outside_conduction_w = surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone.zone_id)
        .map(|surface| {
            surface_outside_conduction_rate_w_for_report(
                surface,
                use_inside_ctf_outside_temperature_for_conduction_report,
            )
        })
        .sum();
    let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
        zone_surface_convection_sums(surfaces, zone.zone_id);
    zone.sum_ha_w_per_k = sum_ha_w_per_k;
    zone.sum_hat_surf_w = sum_hat_surf_w;
    zone.sum_hat_ref_w = sum_hat_ref_w;
    let coefficients = energyplus_zone_air_temperature_coefficients(
        zone.sum_ha_w_per_k,
        zone.sum_hat_surf_w,
        zone.sum_hat_ref_w,
        zone.convective_internal_gain_w,
        0.0,
        0.0,
        zone.air_heat_capacity_j_per_k,
        timestep_seconds,
        previous_mean_air_temperatures_c,
    );
    zone.mean_air_temperature_c = energyplus_third_order_zone_air_temperature_from_coefficients(
        previous_mean_air_temperatures_c[0],
        coefficients,
    );
    zone.zone_air_temperature_coefficients = coefficients;
}

fn correct_single_zone_air_humidity_ratio_from_history(
    zone: &mut ZoneHeatBalanceState,
    timestep_seconds: f64,
    previous_air_humidity_ratios: [f64; 3],
    context: Option<HeatBalanceWeatherContext<'_>>,
) {
    let history_term = 3.0 * previous_air_humidity_ratios[0]
        - (3.0 / 2.0) * previous_air_humidity_ratios[1]
        + (1.0 / 3.0) * previous_air_humidity_ratios[2];
    let humidity_ratio = history_term / (11.0 / 6.0);
    let atmospheric_pressure_pa = context
        .and_then(|context| {
            Some(energyplus_weather_atmospheric_pressure_for_context(
                context,
                context
                    .records
                    .get(context.record_index)?
                    .atmospheric_pressure_pa,
            ))
        })
        .unwrap_or(ENERGYPLUS_STANDARD_ATMOSPHERIC_PRESSURE_PA);
    let saturation_humidity_ratio = energyplus_psychrometric_humidity_ratio_from_rh(
        zone.mean_air_temperature_c,
        1.0,
        atmospheric_pressure_pa,
    )
    .unwrap_or(f64::INFINITY);
    zone.air_humidity_ratio = humidity_ratio
        .clamp(0.0, saturation_humidity_ratio)
        .max(ENERGYPLUS_MIN_HUMIDITY_RATIO);
    if timestep_seconds <= 0.0 || !zone.air_humidity_ratio.is_finite() {
        zone.air_humidity_ratio = ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO;
    }
}

fn energyplus_down_interpolate_three_history_values(
    old_timestep_seconds: f64,
    new_timestep_seconds: f64,
    old_values: [f64; 3],
) -> [f64; 3] {
    if old_timestep_seconds <= 0.0 || new_timestep_seconds <= 0.0 {
        return old_values;
    }

    let down_step_ratio = old_timestep_seconds / new_timestep_seconds;
    let mut new_values = [old_values[0]; 3];
    if (down_step_ratio - 2.0).abs() < 0.01 {
        new_values[1] = (old_values[0] + old_values[1]) / 2.0;
        new_values[2] = old_values[1];
    } else if (down_step_ratio - 3.0).abs() < 0.01 {
        let delta = (old_values[1] - old_values[0]) / 3.0;
        new_values[1] = old_values[0] + delta;
        new_values[2] = new_values[1] + delta;
    } else {
        let delta = (old_values[1] - old_values[0]) / down_step_ratio;
        new_values[1] = old_values[0] + delta;
        new_values[2] = new_values[1] + delta;
    }
    new_values
}

fn zone_surface_report_conduction_rates_w(
    surfaces: &[SurfaceHeatBalanceState],
    zone_id: ZoneId,
    use_inside_ctf_outside_temperature_for_conduction_report: bool,
) -> (f64, f64) {
    surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone_id)
        .map(|surface| {
            (
                surface_inside_conduction_rate_w_for_report(
                    surface,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                ),
                surface_outside_conduction_rate_w_for_report(
                    surface,
                    use_inside_ctf_outside_temperature_for_conduction_report,
                ),
            )
        })
        .fold(
            (0.0, 0.0),
            |(inside_sum, outside_sum), (inside, outside)| {
                (inside_sum + inside, outside_sum + outside)
            },
        )
}

fn sync_adiabatic_outside_faces_to_inside_faces(surfaces: &mut [SurfaceHeatBalanceState]) {
    for surface in surfaces {
        if surface.outside_boundary_condition == OutsideBoundaryCondition::Adiabatic {
            surface.outside_face_temperature_c = surface.inside_face_temperature_c;
        }
    }
}

fn inside_ctf_outside_temperature_history_commit_override_c(
    surface: &SurfaceHeatBalanceState,
    commit_inside_ctf_outside_temperature_to_history: bool,
    snapshots: Option<&BTreeMap<SurfaceId, f64>>,
) -> Option<f64> {
    if !commit_inside_ctf_outside_temperature_to_history
        || surface.outside_boundary_condition != OutsideBoundaryCondition::Outdoors
    {
        return None;
    }

    snapshots.and_then(|snapshots| snapshots.get(&surface.surface_id).copied())
}

fn heat_balance_uses_third_order_zone_air_correction(
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

fn heat_balance_preserves_surface_inside_temperature_for_first_longwave(
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

fn heat_balance_uses_weather_air_storage_report(
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

fn heat_balance_uses_previous_mat_surface_convection_report(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    let zone_air_algorithm = heat_balance_zone_air_algorithm_feature_base(zone_air_algorithm);
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
    )
}

fn heat_balance_uses_balance_surface_convection_report(
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

fn heat_balance_uses_surface_reference_air_convection_report(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
    )
}

fn heat_balance_uses_surface_reference_air_surface_convection_report(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
            | HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
    )
}

fn heat_balance_uses_final_inside_convection_report(
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
) -> bool {
    matches!(
        zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
    )
}

fn zone_surface_convection_sums(
    surfaces: &[SurfaceHeatBalanceState],
    zone_id: ZoneId,
) -> (f64, f64, f64) {
    let (sum_ha_w_per_k, sum_hat_surf_w) = surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone_id)
        .map(|surface| {
            let surface_ha_w_per_k =
                surface.inside_convection_coefficient_w_per_m2_k * surface.area_m2;
            (
                surface_ha_w_per_k,
                surface_ha_w_per_k * surface.inside_face_temperature_c,
            )
        })
        .fold((0.0, 0.0), |(sum_ha, sum_hat), (ha, hat)| {
            (sum_ha + ha, sum_hat + hat)
        });

    (sum_ha_w_per_k, sum_hat_surf_w, 0.0)
}

fn zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w(
    surfaces: &[SurfaceHeatBalanceState],
    zone_id: ZoneId,
) -> f64 {
    surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone_id)
        .map(|surface| {
            surface.inside_convection_coefficient_w_per_m2_k
                * surface.area_m2
                * (surface.inside_face_temperature_c - surface.inside_reference_air_temperature_c)
        })
        .sum()
}

fn surface_inside_convection_reference_air_temperature_c(
    surface: &SurfaceHeatBalanceState,
    zones: &[ZoneHeatBalanceState],
    use_surface_reference_air_report: bool,
) -> f64 {
    if use_surface_reference_air_report {
        surface.inside_reference_air_temperature_c
    } else {
        zones
            .iter()
            .find(|zone| zone.zone_id == surface.zone_id)
            .map(|zone| zone.mean_air_temperature_c)
            .unwrap_or(surface.inside_face_temperature_c)
    }
}

fn surface_inside_convection_report_coefficient_w_per_m2_k(
    surface: &SurfaceHeatBalanceState,
    zones: &[ZoneHeatBalanceState],
    use_surface_reference_air_report: bool,
    use_final_inside_convection_report: bool,
) -> f64 {
    if use_final_inside_convection_report {
        let reference_air_temperature_c = surface_inside_convection_reference_air_temperature_c(
            surface,
            zones,
            use_surface_reference_air_report,
        );
        energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
            surface,
            surface.inside_face_temperature_c,
            reference_air_temperature_c,
        )
    } else {
        surface.inside_convection_coefficient_w_per_m2_k
    }
}

fn surface_inside_convection_heat_gain_rate_per_area_w_per_m2(
    surface: &SurfaceHeatBalanceState,
    zones: &[ZoneHeatBalanceState],
    use_surface_reference_air_report: bool,
    use_final_inside_convection_report: bool,
) -> f64 {
    let reference_air_temperature_c = surface_inside_convection_reference_air_temperature_c(
        surface,
        zones,
        use_surface_reference_air_report,
    );
    surface_inside_convection_report_coefficient_w_per_m2_k(
        surface,
        zones,
        use_surface_reference_air_report,
        use_final_inside_convection_report,
    ) * (reference_air_temperature_c - surface.inside_face_temperature_c)
}

fn zone_air_heat_balance_surface_convection_rate_from_final_inside_hconv_report_w(
    surfaces: &[SurfaceHeatBalanceState],
    zones: &[ZoneHeatBalanceState],
    zone_id: ZoneId,
    use_surface_reference_air_report: bool,
) -> f64 {
    surfaces
        .iter()
        .filter(|surface| surface.zone_id == zone_id)
        .map(|surface| {
            let reference_air_temperature_c = surface_inside_convection_reference_air_temperature_c(
                surface,
                zones,
                use_surface_reference_air_report,
            );
            let coefficient_w_per_m2_k = surface_inside_convection_report_coefficient_w_per_m2_k(
                surface,
                zones,
                use_surface_reference_air_report,
                true,
            );
            coefficient_w_per_m2_k
                * surface.area_m2
                * (surface.inside_face_temperature_c - reference_air_temperature_c)
        })
        .sum()
}

fn zone_air_heat_balance_surface_convection_rate_w(zone_state: &ZoneHeatBalanceState) -> f64 {
    zone_air_heat_balance_surface_convection_rate_at_air_temperature_w(
        zone_state,
        zone_state.mean_air_temperature_c,
    )
}

fn zone_air_heat_balance_surface_convection_rate_at_air_temperature_w(
    zone_state: &ZoneHeatBalanceState,
    reference_air_temperature_c: f64,
) -> f64 {
    zone_state.sum_hat_surf_w
        - zone_state.sum_hat_ref_w
        - zone_state.sum_ha_w_per_k * reference_air_temperature_c
}

fn zone_air_heat_balance_surface_convection_rate_from_balance_w(
    zone_state: &ZoneHeatBalanceState,
    air_storage_rate_w: f64,
) -> f64 {
    air_storage_rate_w - zone_state.convective_internal_gain_w
}

/// Simulates hourly zone mean air temperatures through the heat-balance state
/// shell without making a conformance claim.
///
/// This diagnostic trace runs every configured zone timestep, samples hourly
/// MAT values, and stores EnergyPlus-style result series for all zones.
pub fn simulate_heat_balance_zone_air_temperatures(
    model: &SimulationModel,
    weather_dry_bulb_c: &[f64],
    options: HeatBalanceSimulationOptions,
) -> Result<HeatBalanceSimulation, RuntimeError> {
    simulate_heat_balance_zone_air_temperatures_internal(
        model,
        weather_dry_bulb_c,
        None,
        options,
        &[],
    )
}

/// Simulates hourly zone mean air temperatures with full EPW records available
/// for diagnostic exterior surface forcing.
pub fn simulate_heat_balance_zone_air_temperatures_with_weather_records(
    model: &SimulationModel,
    weather_records: &[EpwRecord],
    options: HeatBalanceSimulationOptions,
) -> Result<HeatBalanceSimulation, RuntimeError> {
    simulate_heat_balance_zone_air_temperatures_with_weather_records_and_ctf_coefficients(
        model,
        weather_records,
        options,
        &[],
    )
}

/// Simulates hourly zone mean air temperatures with diagnostic CTF coefficient rows.
///
/// The coefficient rows are intended for diagnostic isolation with EnergyPlus
/// `eplusout.eio` CTF output. Conformance paths should use the default
/// simulation entry points until native coefficient generation is ported.
pub fn simulate_heat_balance_zone_air_temperatures_with_weather_records_and_ctf_coefficients(
    model: &SimulationModel,
    weather_records: &[EpwRecord],
    options: HeatBalanceSimulationOptions,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
) -> Result<HeatBalanceSimulation, RuntimeError> {
    let weather_dry_bulb_c = weather_records
        .iter()
        .map(|record| record.dry_bulb_c)
        .collect::<Vec<_>>();
    simulate_heat_balance_zone_air_temperatures_internal(
        model,
        &weather_dry_bulb_c,
        Some(weather_records),
        options,
        ctf_coefficients,
    )
}

fn simulate_heat_balance_zone_air_temperatures_internal(
    model: &SimulationModel,
    weather_dry_bulb_c: &[f64],
    weather_records: Option<&[EpwRecord]>,
    options: HeatBalanceSimulationOptions,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
) -> Result<HeatBalanceSimulation, RuntimeError> {
    if weather_dry_bulb_c.is_empty() {
        return Err(RuntimeError::NoWeatherData);
    }
    if options.sample_count > weather_dry_bulb_c.len() {
        return Err(RuntimeError::SampleCountExceedsWeather {
            requested: options.sample_count,
            available: weather_dry_bulb_c.len(),
        });
    }
    if model.typed.zones.is_empty() {
        return Err(RuntimeError::NoZones);
    }

    let zone_steps_per_hour = model.typed.timestep.number_of_timesteps_per_hour.max(1);
    let seconds_per_timestep = SECONDS_PER_HOUR / f64::from(zone_steps_per_hour);
    let first_hour_interpolation_starting_values =
        run_period_first_hour_interpolation_starting_values(&model.typed);
    let mut state = initialize_heat_balance_state_with_ctf_coefficients(
        model,
        options.initial_zone_air_temperature_c,
        ctf_coefficients,
    )?;
    seed_zone_air_humidity_ratios_from_weather_records(
        &mut state,
        weather_records,
        weather_dry_bulb_c[0],
        zone_steps_per_hour,
        first_hour_interpolation_starting_values,
    );
    match options.ctf_initial_history_policy {
        HeatBalanceCtfInitialHistoryPolicy::BoundaryTemperatureAndUValue => {
            seed_initial_surface_ctf_boundary_histories(&mut state, weather_dry_bulb_c[0]);
        }
        HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial => {
            seed_energyplus_initial_surface_ctf_histories(
                &mut state,
                options.initial_zone_air_temperature_c,
                weather_dry_bulb_c[0],
            );
        }
    }
    let mut warmup_day_end_zone_air_states = Vec::new();
    let warmup = run_heat_balance_run_period_warmup(
        &model.typed,
        &mut state,
        weather_dry_bulb_c,
        weather_records,
        zone_steps_per_hour,
        seconds_per_timestep,
        options.warmup,
        options.zone_air_algorithm,
        options.surface_iteration_count,
        options.inside_hconv_reevaluation_interval,
        options.surface_loop_zone_air_correction,
        first_hour_interpolation_starting_values,
        &mut warmup_day_end_zone_air_states,
    );
    let run_period_initial_zone_air_states = state
        .zones
        .iter()
        .map(heat_balance_zone_air_state_sample)
        .collect::<Vec<_>>();
    let run_period_initial_ctf_history_slots =
        heat_balance_ctf_history_slot_samples(&state.surfaces);
    let run_period_timestep_start = state.timestep_index;
    let mut zone_temperatures = state
        .zones
        .iter()
        .map(|zone| {
            (
                zone.zone_id,
                zone.zone_name.clone(),
                Vec::with_capacity(options.sample_count),
            )
        })
        .collect::<Vec<_>>();
    let mut zone_humidity_ratios = state
        .zones
        .iter()
        .map(|zone| {
            (
                zone.zone_id,
                zone.zone_name.clone(),
                Vec::with_capacity(options.sample_count),
            )
        })
        .collect::<Vec<_>>();
    let mut zone_conduction_rates = state
        .zones
        .iter()
        .map(|zone| ZoneConductionTrace {
            zone_id: zone.zone_id,
            zone_name: zone.zone_name.clone(),
            inside_conduction_rate_w: Vec::with_capacity(options.sample_count),
            inside_conduction_gain_rate_w: Vec::with_capacity(options.sample_count),
            inside_conduction_loss_rate_w: Vec::with_capacity(options.sample_count),
            outside_conduction_rate_w: Vec::with_capacity(options.sample_count),
            outside_conduction_gain_rate_w: Vec::with_capacity(options.sample_count),
            outside_conduction_loss_rate_w: Vec::with_capacity(options.sample_count),
        })
        .collect::<Vec<_>>();
    let mut inside_surface_iteration_counts = Vec::with_capacity(options.sample_count);
    let mut zone_air_heat_balance_rates = state
        .zones
        .iter()
        .map(|zone| {
            (
                zone.zone_id,
                zone.zone_name.clone(),
                Vec::with_capacity(options.sample_count),
                Vec::with_capacity(options.sample_count),
                Vec::with_capacity(options.sample_count),
            )
        })
        .collect::<Vec<_>>();
    let mut zone_air_debug_traces = state
        .zones
        .iter()
        .map(|zone| ZoneAirDebugTrace {
            zone_id: zone.zone_id,
            zone_name: zone.zone_name.clone(),
            current_temperature_c: Vec::with_capacity(options.sample_count),
            zone_timestep_average_temperature_c: Vec::with_capacity(options.sample_count),
            previous_temperature_1_c: Vec::with_capacity(options.sample_count),
            previous_temperature_2_c: Vec::with_capacity(options.sample_count),
            previous_temperature_3_c: Vec::with_capacity(options.sample_count),
            previous_system_temperature_1_c: Vec::with_capacity(options.sample_count),
            system_timestep_count: Vec::with_capacity(options.sample_count),
            humidity_ratio: Vec::with_capacity(options.sample_count),
            zone_timestep_average_humidity_ratio: Vec::with_capacity(options.sample_count),
            air_heat_capacity_j_per_k: Vec::with_capacity(options.sample_count),
            zone_timestep_air_power_cap_w_per_k: Vec::with_capacity(options.sample_count),
            last_correction_air_power_cap_w_per_k: Vec::with_capacity(options.sample_count),
        })
        .collect::<Vec<_>>();
    let mut surface_temperatures = state
        .surfaces
        .iter()
        .map(|surface| SurfaceHeatBalanceTrace {
            surface_id: surface.surface_id,
            surface_name: surface.surface_name.clone(),
            inside_face_temperature_c: Vec::with_capacity(options.sample_count),
            inside_adjacent_air_temperature_c: Vec::with_capacity(options.sample_count),
            outside_face_temperature_c: Vec::with_capacity(options.sample_count),
            outside_outdoor_air_dry_bulb_temperature_c: Vec::with_capacity(options.sample_count),
            outside_outdoor_air_wet_bulb_temperature_c: Vec::with_capacity(options.sample_count),
            outside_outdoor_air_wind_speed_m_per_s: Vec::with_capacity(options.sample_count),
            outside_outdoor_air_wind_direction_deg: Vec::with_capacity(options.sample_count),
            inside_convection_heat_gain_rate_w: Vec::with_capacity(options.sample_count),
            inside_convection_heat_gain_rate_per_area_w_per_m2: Vec::with_capacity(
                options.sample_count,
            ),
            inside_convection_coefficient_w_per_m2_k: Vec::with_capacity(options.sample_count),
            inside_net_surface_thermal_radiation_heat_gain_rate_w: Vec::with_capacity(
                options.sample_count,
            ),
            inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2:
                Vec::with_capacity(options.sample_count),
            outside_convection_heat_gain_rate_w: Vec::with_capacity(options.sample_count),
            outside_convection_heat_gain_rate_per_area_w_per_m2: Vec::with_capacity(
                options.sample_count,
            ),
            outside_convection_coefficient_w_per_m2_k: Vec::with_capacity(options.sample_count),
            outside_net_thermal_radiation_heat_gain_rate_w: Vec::with_capacity(
                options.sample_count,
            ),
            outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2: Vec::with_capacity(
                options.sample_count,
            ),
            outside_thermal_radiation_to_air_coefficient_w_per_m2_k: Vec::with_capacity(
                options.sample_count,
            ),
            outside_thermal_radiation_to_sky_coefficient_w_per_m2_k: Vec::with_capacity(
                options.sample_count,
            ),
            outside_thermal_radiation_to_ground_coefficient_w_per_m2_k: Vec::with_capacity(
                options.sample_count,
            ),
            outside_solar_radiation_heat_gain_rate_w: Vec::with_capacity(options.sample_count),
            outside_solar_radiation_heat_gain_rate_per_area_w_per_m2: Vec::with_capacity(
                options.sample_count,
            ),
            outside_balance_report_temperature_c: Vec::with_capacity(options.sample_count),
            outside_balance_coefficient_temperature_c: Vec::with_capacity(options.sample_count),
            outside_balance_convection_reference_temperature_c: Vec::with_capacity(
                options.sample_count,
            ),
            outside_balance_equivalent_radiant_temperature_c: Vec::with_capacity(
                options.sample_count,
            ),
            outside_balance_radiation_coefficient_w_per_m2_k: Vec::with_capacity(
                options.sample_count,
            ),
            outside_quick_balance_inside_source_term_w_per_m2: Vec::with_capacity(
                options.sample_count,
            ),
            outside_quick_balance_inside_balance_term_w_per_m2: Vec::with_capacity(
                options.sample_count,
            ),
            outside_quick_balance_numerator_w_per_m2: Vec::with_capacity(options.sample_count),
            outside_quick_balance_denominator_w_per_m2_k: Vec::with_capacity(options.sample_count),
            outside_quick_balance_coupling_factor: Vec::with_capacity(options.sample_count),
            inside_conduction_rate_w: Vec::with_capacity(options.sample_count),
            inside_conduction_gain_rate_w: Vec::with_capacity(options.sample_count),
            inside_conduction_loss_rate_w: Vec::with_capacity(options.sample_count),
            inside_conduction_rate_per_area_w_per_m2: Vec::with_capacity(options.sample_count),
            ctf_inside_current_outside_term_rate_w: Vec::with_capacity(options.sample_count),
            ctf_inside_current_inside_term_rate_w: Vec::with_capacity(options.sample_count),
            ctf_inside_history_term_rate_w: Vec::with_capacity(options.sample_count),
            ctf_inside_history_temperature_term_rate_w: Vec::with_capacity(options.sample_count),
            ctf_inside_history_flux_term_rate_w: Vec::with_capacity(options.sample_count),
            outside_conduction_rate_w: Vec::with_capacity(options.sample_count),
            outside_conduction_gain_rate_w: Vec::with_capacity(options.sample_count),
            outside_conduction_loss_rate_w: Vec::with_capacity(options.sample_count),
            outside_conduction_rate_per_area_w_per_m2: Vec::with_capacity(options.sample_count),
            ctf_outside_current_outside_term_rate_w: Vec::with_capacity(options.sample_count),
            ctf_outside_current_inside_term_rate_w: Vec::with_capacity(options.sample_count),
            ctf_outside_history_term_rate_w: Vec::with_capacity(options.sample_count),
            heat_storage_rate_w: Vec::with_capacity(options.sample_count),
            heat_storage_rate_per_area_w_per_m2: Vec::with_capacity(options.sample_count),
        })
        .collect::<Vec<_>>();
    let mut outdoor_temperatures = Vec::with_capacity(options.sample_count);
    let mut outdoor_wet_bulb_temperatures = Vec::with_capacity(options.sample_count);
    let mut sky_temperatures = Vec::with_capacity(options.sample_count);
    let mut horizontal_infrared_radiation_rates = Vec::with_capacity(options.sample_count);
    let mut rain_statuses = Vec::with_capacity(options.sample_count);
    let mut first_sample_ctf_history_slot_accumulators =
        BTreeMap::<(String, usize), HeatBalanceCtfHistorySlotFirstSampleAccumulator>::new();
    let mut hourly_ctf_history_slots = Vec::new();
    let mut hourly_ctf_history_slots_after_advance = Vec::new();
    let mut surface_first_sample_trace = Vec::new();
    let mut zone_air_first_sample_trace = Vec::new();
    let mut surface_iteration_first_sample_trace = Vec::new();
    let mut surface_iteration_sample_trace = Vec::new();
    let report_zone_air_algorithm =
        heat_balance_zone_air_algorithm_execution_variant(options.zone_air_algorithm);
    let use_surface_reference_air_zone_convection_report =
        heat_balance_uses_surface_reference_air_convection_report(report_zone_air_algorithm);
    let use_surface_reference_air_surface_convection_report =
        heat_balance_uses_surface_reference_air_surface_convection_report(
            report_zone_air_algorithm,
        );
    let use_final_inside_convection_report =
        heat_balance_uses_final_inside_convection_report(report_zone_air_algorithm);
    let use_inside_ctf_outside_temperature_for_conduction_report = matches!(
        report_zone_air_algorithm,
        HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
    );
    let use_surface_report_zone_conduction_rates = matches!(
        options.zone_conduction_report_source,
        HeatBalanceZoneConductionReportSource::SurfaceReport
    );

    for (hour_index, outdoor_dry_bulb_c) in weather_dry_bulb_c
        .iter()
        .copied()
        .take(options.sample_count)
        .enumerate()
    {
        let hour_ending = u32::try_from(hour_index % 24 + 1).unwrap_or(24);
        let steps = zone_steps_per_hour.max(1);
        let mut zone_temperature_sums = vec![0.0; zone_temperatures.len()];
        let mut zone_humidity_ratio_sums = vec![0.0; zone_humidity_ratios.len()];
        let mut zone_conduction_sums =
            vec![(0.0, 0.0, 0.0, 0.0, 0.0, 0.0); zone_conduction_rates.len()];
        let mut inside_surface_iteration_count_sum = 0.0;
        let mut zone_air_heat_balance_sums =
            vec![(0.0, 0.0, 0.0); zone_air_heat_balance_rates.len()];
        let mut zone_air_heat_balance_last =
            vec![(0.0, 0.0, 0.0); zone_air_heat_balance_rates.len()];
        let mut zone_air_debug_sums =
            vec![ZoneAirDebugTraceSums::default(); zone_air_debug_traces.len()];
        let mut surface_sums =
            vec![SurfaceHeatBalanceTraceSums::default(); surface_temperatures.len()];
        let mut outdoor_temperature_sum = 0.0;
        let mut outdoor_wet_bulb_temperature_sum = 0.0;
        let mut sky_temperature_sum = 0.0;
        let mut horizontal_infrared_radiation_sum = 0.0;
        let mut rain_status_sum = 0.0;
        let mut hourly_ctf_history_slot_accumulators =
            BTreeMap::<(String, usize), HeatBalanceCtfHistorySlotFirstSampleAccumulator>::new();
        let mut hourly_ctf_history_slot_after_advance_accumulators =
            BTreeMap::<(String, usize), HeatBalanceCtfHistorySlotFirstSampleAccumulator>::new();

        for substep in 1..=steps {
            let timestep_outdoor_dry_bulb_c =
                energyplus_weather_dry_bulb_at_timestep_with_starting_values(
                    weather_records,
                    hour_index,
                    outdoor_dry_bulb_c,
                    steps,
                    substep,
                    first_hour_interpolation_starting_values,
                );
            let weather_context = heat_balance_weather_context_for_timestep(
                weather_records,
                hour_index,
                steps,
                substep,
                first_hour_interpolation_starting_values,
            );
            let timestep_outdoor_wet_bulb_c = weather_context
                .map(|context| {
                    energyplus_exterior_wet_reference_temperature_c(
                        context,
                        timestep_outdoor_dry_bulb_c,
                    )
                })
                .unwrap_or(timestep_outdoor_dry_bulb_c);
            let timestep_horizontal_infrared_radiation_w_per_m2 = weather_context
                .and_then(|context| {
                    context.records.get(context.record_index).map(|record| {
                        energyplus_weather_horizontal_infrared_for_context(
                            context,
                            record.horizontal_infrared_radiation_wh_per_m2,
                        )
                    })
                })
                .unwrap_or(0.0);
            let timestep_sky_temperature_c = horizontal_infrared_sky_temperature_c(
                timestep_horizontal_infrared_radiation_w_per_m2,
                timestep_outdoor_dry_bulb_c,
            );
            let timestep_rain_status = weather_context
                .map(|context| {
                    if energyplus_weather_record_is_rain_at_timestep_with_starting_values(
                        context.records,
                        context.record_index,
                        substep,
                        steps,
                        context.first_hour_interpolation_starting_values,
                    ) {
                        1.0
                    } else {
                        0.0
                    }
                })
                .unwrap_or(0.0);
            advance_heat_balance_state_one_timestep_internal(
                &model.typed,
                &mut state,
                HeatBalanceStepInput {
                    outdoor_dry_bulb_c: timestep_outdoor_dry_bulb_c,
                    hour_ending,
                    timestep_seconds: seconds_per_timestep,
                },
                weather_context,
                options.zone_air_algorithm,
                options.surface_iteration_count,
                options.inside_hconv_reevaluation_interval,
                options.surface_loop_zone_air_correction,
            );

            for sample in &state.last_ctf_history_slot_terms {
                hourly_ctf_history_slot_accumulators
                    .entry((sample.surface_name.clone(), sample.slot_index))
                    .or_insert_with(|| {
                        HeatBalanceCtfHistorySlotFirstSampleAccumulator::from_sample(sample)
                    })
                    .push(sample);
            }
            for sample in &state.last_ctf_history_slot_terms_after_advance {
                hourly_ctf_history_slot_after_advance_accumulators
                    .entry((sample.surface_name.clone(), sample.slot_index))
                    .or_insert_with(|| {
                        HeatBalanceCtfHistorySlotFirstSampleAccumulator::from_sample(sample)
                    })
                    .push(sample);
            }

            if hour_index == 0 {
                for sample in &state.last_ctf_history_slot_terms {
                    first_sample_ctf_history_slot_accumulators
                        .entry((sample.surface_name.clone(), sample.slot_index))
                        .or_insert_with(|| {
                            HeatBalanceCtfHistorySlotFirstSampleAccumulator::from_sample(sample)
                        })
                        .push(sample);
                }
            }

            outdoor_temperature_sum += timestep_outdoor_dry_bulb_c;
            outdoor_wet_bulb_temperature_sum += timestep_outdoor_wet_bulb_c;
            sky_temperature_sum += timestep_sky_temperature_c;
            horizontal_infrared_radiation_sum += timestep_horizontal_infrared_radiation_w_per_m2;
            rain_status_sum += timestep_rain_status;
            for (index, (zone_id, _zone_name, _values)) in zone_temperatures.iter().enumerate() {
                if let Some(zone_state) = state.zones.iter().find(|zone| zone.zone_id == *zone_id) {
                    let reported_zone_temperature_c = if matches!(
                        options.zone_air_algorithm,
                        HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate
                            | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
                    ) {
                        zone_state.zone_timestep_average_air_temperature_c
                    } else {
                        zone_state.mean_air_temperature_c
                    };
                    zone_temperature_sums[index] += reported_zone_temperature_c;
                }
            }
            for (index, (zone_id, _zone_name, _values)) in zone_humidity_ratios.iter().enumerate() {
                if let Some(zone_state) = state.zones.iter().find(|zone| zone.zone_id == *zone_id) {
                    let reported_zone_humidity_ratio = if matches!(
                        options.zone_air_algorithm,
                        HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate
                            | HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
                    ) {
                        zone_state.zone_timestep_average_air_humidity_ratio
                    } else {
                        zone_state.air_humidity_ratio
                    };
                    zone_humidity_ratio_sums[index] += reported_zone_humidity_ratio;
                }
            }
            for (index, trace) in zone_air_debug_traces.iter().enumerate() {
                if let Some(zone_state) = state
                    .zones
                    .iter()
                    .find(|zone| zone.zone_id == trace.zone_id)
                {
                    let zone_timestep_air_power_cap_w_per_k = if seconds_per_timestep > 0.0 {
                        zone_state.air_heat_capacity_j_per_k / seconds_per_timestep
                    } else {
                        0.0
                    };
                    if hour_index == 0 {
                        let coefficients = zone_state.zone_air_temperature_coefficients;
                        let third_order_solution_temperature_c =
                            if coefficients.third_order_temp_dependent_load_w_per_k.abs()
                                <= f64::EPSILON
                            {
                                zone_state.mean_air_temperature_c
                            } else {
                                coefficients.third_order_temp_independent_load_w
                                    / coefficients.third_order_temp_dependent_load_w_per_k
                            };
                        zone_air_first_sample_trace.push(HeatBalanceZoneAirFirstSampleTrace {
                            zone_id: zone_state.zone_id,
                            zone_name: zone_state.zone_name.clone(),
                            timestep_index: substep,
                            outdoor_dry_bulb_c: timestep_outdoor_dry_bulb_c,
                            timestep_seconds: seconds_per_timestep,
                            mean_air_temperature_c: zone_state.mean_air_temperature_c,
                            zone_timestep_average_air_temperature_c: zone_state
                                .zone_timestep_average_air_temperature_c,
                            previous_mean_air_temperatures_c: zone_state
                                .previous_mean_air_temperatures_c,
                            previous_system_mean_air_temperatures_c: zone_state
                                .previous_system_mean_air_temperatures_c,
                            previous_system_timestep_count: zone_state
                                .previous_system_timestep_count,
                            air_humidity_ratio: zone_state.air_humidity_ratio,
                            zone_timestep_average_air_humidity_ratio: zone_state
                                .zone_timestep_average_air_humidity_ratio,
                            air_heat_capacity_j_per_k: zone_state.air_heat_capacity_j_per_k,
                            zone_timestep_air_power_cap_w_per_k,
                            zone_air_temperature_coefficients: coefficients,
                            third_order_solution_numerator_w: coefficients
                                .third_order_temp_independent_load_w,
                            third_order_solution_denominator_w_per_k: coefficients
                                .third_order_temp_dependent_load_w_per_k,
                            third_order_solution_temperature_c,
                        });
                    }
                    zone_air_debug_sums[index].current_temperature_c +=
                        zone_state.mean_air_temperature_c;
                    zone_air_debug_sums[index].zone_timestep_average_temperature_c +=
                        zone_state.zone_timestep_average_air_temperature_c;
                    zone_air_debug_sums[index].previous_temperature_1_c +=
                        zone_state.previous_mean_air_temperatures_c[0];
                    zone_air_debug_sums[index].previous_temperature_2_c +=
                        zone_state.previous_mean_air_temperatures_c[1];
                    zone_air_debug_sums[index].previous_temperature_3_c +=
                        zone_state.previous_mean_air_temperatures_c[2];
                    zone_air_debug_sums[index].previous_system_temperature_1_c +=
                        zone_state.previous_system_mean_air_temperatures_c[0];
                    zone_air_debug_sums[index].system_timestep_count +=
                        f64::from(zone_state.previous_system_timestep_count);
                    zone_air_debug_sums[index].humidity_ratio += zone_state.air_humidity_ratio;
                    zone_air_debug_sums[index].zone_timestep_average_humidity_ratio +=
                        zone_state.zone_timestep_average_air_humidity_ratio;
                    zone_air_debug_sums[index].air_heat_capacity_j_per_k +=
                        zone_state.air_heat_capacity_j_per_k;
                    zone_air_debug_sums[index].zone_timestep_air_power_cap_w_per_k +=
                        zone_timestep_air_power_cap_w_per_k;
                    zone_air_debug_sums[index].last_correction_air_power_cap_w_per_k += zone_state
                        .zone_air_temperature_coefficients
                        .air_power_cap_w_per_k;
                }
            }
            for (index, trace) in zone_conduction_rates.iter().enumerate() {
                if use_surface_report_zone_conduction_rates {
                    let (inside_rate, outside_rate) = zone_surface_report_conduction_rates_w(
                        &state.surfaces,
                        trace.zone_id,
                        use_inside_ctf_outside_temperature_for_conduction_report,
                    );
                    zone_conduction_sums[index].0 += inside_rate;
                    zone_conduction_sums[index].1 += heat_gain_rate_w(inside_rate);
                    zone_conduction_sums[index].2 += heat_loss_rate_w(inside_rate);
                    zone_conduction_sums[index].3 += outside_rate;
                    zone_conduction_sums[index].4 += heat_gain_rate_w(outside_rate);
                    zone_conduction_sums[index].5 += heat_loss_rate_w(outside_rate);
                } else if let Some(zone_state) = state
                    .zones
                    .iter()
                    .find(|zone| zone.zone_id == trace.zone_id)
                {
                    let inside_rate = zone_state.opaque_surface_heat_gain_w;
                    let outside_rate = zone_state.opaque_surface_outside_conduction_w;
                    zone_conduction_sums[index].0 += inside_rate;
                    zone_conduction_sums[index].1 += heat_gain_rate_w(inside_rate);
                    zone_conduction_sums[index].2 += heat_loss_rate_w(inside_rate);
                    zone_conduction_sums[index].3 += outside_rate;
                    zone_conduction_sums[index].4 += heat_gain_rate_w(outside_rate);
                    zone_conduction_sums[index].5 += heat_loss_rate_w(outside_rate);
                }
            }
            inside_surface_iteration_count_sum +=
                f64::from(state.last_inside_surface_iteration_count);
            surface_iteration_sample_trace.push(HeatBalanceSurfaceIterationSampleTrace {
                sample_index: hour_index,
                timestep_index: substep,
                inside_surface_iteration_count: state.last_inside_surface_iteration_count,
                max_inside_surface_delta_c: state.last_inside_surface_iteration_max_delta_c,
                max_delta_surface_name: state
                    .last_inside_surface_iteration_max_delta_surface_name
                    .clone(),
            });
            if hour_index == 0 {
                surface_iteration_first_sample_trace.push(
                    HeatBalanceSurfaceIterationFirstSampleTrace {
                        timestep_index: substep,
                        inside_surface_iteration_count: state.last_inside_surface_iteration_count,
                        max_inside_surface_delta_c: state.last_inside_surface_iteration_max_delta_c,
                        max_delta_surface_name: state
                            .last_inside_surface_iteration_max_delta_surface_name
                            .clone(),
                    },
                );
            }
            for (index, (zone_id, _zone_name, _internal, _surface, _storage)) in
                zone_air_heat_balance_rates.iter().enumerate()
            {
                if let Some(zone_state) = state.zones.iter().find(|zone| zone.zone_id == *zone_id) {
                    let third_order_report_air_heat_capacity_j_per_k =
                        if heat_balance_uses_weather_air_storage_report(options.zone_air_algorithm)
                        {
                            weather_proxy_zone_air_heat_capacity_j_per_k(
                                zone_state,
                                weather_context,
                                timestep_outdoor_dry_bulb_c,
                            )
                        } else {
                            None
                        };
                    let air_storage_rate_w = zone_air_heat_balance_air_storage_rate_w(
                        zone_state,
                        seconds_per_timestep,
                        options.zone_air_algorithm,
                        third_order_report_air_heat_capacity_j_per_k,
                    );
                    let air_storage_rate_w = zone_state
                        .system_timestep_average_air_storage_report_w
                        .unwrap_or(air_storage_rate_w);
                    let surface_convection_rate_w = if use_final_inside_convection_report {
                        zone_air_heat_balance_surface_convection_rate_from_final_inside_hconv_report_w(
                                &state.surfaces,
                                &state.zones,
                                *zone_id,
                                use_surface_reference_air_zone_convection_report,
                            )
                    } else if use_surface_reference_air_zone_convection_report {
                        zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w(
                            &state.surfaces,
                            *zone_id,
                        )
                    } else if heat_balance_uses_balance_surface_convection_report(
                        options.zone_air_algorithm,
                    ) {
                        zone_air_heat_balance_surface_convection_rate_from_balance_w(
                            zone_state,
                            air_storage_rate_w,
                        )
                    } else if heat_balance_uses_previous_mat_surface_convection_report(
                        options.zone_air_algorithm,
                    ) {
                        zone_air_heat_balance_surface_convection_rate_at_air_temperature_w(
                            zone_state,
                            zone_state.previous_mean_air_temperatures_c[0],
                        )
                    } else {
                        zone_air_heat_balance_surface_convection_rate_w(zone_state)
                    };
                    let surface_convection_rate_w = zone_state
                        .system_timestep_average_surface_convection_report_w
                        .unwrap_or(surface_convection_rate_w);
                    let values = (
                        zone_state.convective_internal_gain_w,
                        surface_convection_rate_w,
                        air_storage_rate_w,
                    );
                    zone_air_heat_balance_sums[index].0 += values.0;
                    zone_air_heat_balance_sums[index].1 += values.1;
                    zone_air_heat_balance_sums[index].2 += values.2;
                    zone_air_heat_balance_last[index] = values;
                }
            }
            for (index, trace) in surface_temperatures.iter().enumerate() {
                if let Some(surface_state) = state
                    .surfaces
                    .iter()
                    .find(|surface| surface.surface_id == trace.surface_id)
                {
                    let inside_convection_heat_gain_rate_per_area =
                        surface_inside_convection_heat_gain_rate_per_area_w_per_m2(
                            surface_state,
                            &state.zones,
                            use_surface_reference_air_surface_convection_report,
                            use_final_inside_convection_report,
                        );
                    let inside_convection_heat_gain_rate =
                        surface_state.area_m2 * inside_convection_heat_gain_rate_per_area;
                    let inside_convection_coefficient_w_per_m2_k =
                        surface_inside_convection_report_coefficient_w_per_m2_k(
                            surface_state,
                            &state.zones,
                            use_surface_reference_air_surface_convection_report,
                            use_final_inside_convection_report,
                        );
                    let inside_net_surface_thermal_radiation_heat_gain_rate =
                        surface_state.area_m2 * surface_state.inside_net_longwave_w_per_m2;
                    let inside_rate = surface_inside_conduction_rate_w_for_report(
                        surface_state,
                        use_inside_ctf_outside_temperature_for_conduction_report,
                    );
                    let outside_rate = surface_outside_conduction_rate_w_for_report(
                        surface_state,
                        use_inside_ctf_outside_temperature_for_conduction_report,
                    );
                    let storage_rate = surface_heat_storage_rate_w(inside_rate, outside_rate);
                    let storage_rate_per_area =
                        surface_rate_per_area_w_per_m2(storage_rate, surface_state.area_m2);
                    let outside_face_temperature_c = reported_surface_outside_face_temperature_c(
                        &model.typed,
                        surface_state,
                        timestep_outdoor_dry_bulb_c,
                        surface_state.inside_face_temperature_c,
                        weather_context,
                        options.zone_air_algorithm,
                    );
                    let exterior_terms = surface_exterior_report_terms(
                        &model.typed,
                        surface_state,
                        timestep_outdoor_dry_bulb_c,
                        outside_face_temperature_c,
                        weather_context,
                        options.zone_air_algorithm,
                    );
                    let typed_surface = model
                        .typed
                        .surfaces
                        .iter()
                        .find(|surface| surface.id == surface_state.surface_id);
                    let surface_outdoor_air_dry_bulb_temperature_c = typed_surface
                        .map(|surface| {
                            energyplus_surface_outdoor_air_temperature_c(
                                surface,
                                timestep_outdoor_dry_bulb_c,
                            )
                        })
                        .unwrap_or(timestep_outdoor_dry_bulb_c);
                    let surface_outdoor_air_wet_bulb_temperature_c = typed_surface
                        .map(|surface| {
                            energyplus_surface_outdoor_air_temperature_c(
                                surface,
                                timestep_outdoor_wet_bulb_c,
                            )
                        })
                        .unwrap_or(timestep_outdoor_wet_bulb_c);
                    let (weather_file_wind_speed_m_per_s, surface_outdoor_air_wind_direction_deg) =
                        weather_context
                            .and_then(|context| {
                                context.records.get(context.record_index).map(|record| {
                                    (
                                        energyplus_weather_wind_speed_for_context(
                                            context,
                                            record.wind_speed_m_per_s,
                                        ),
                                        energyplus_weather_wind_direction_for_context(
                                            context,
                                            record.wind_direction_deg,
                                        ),
                                    )
                                })
                            })
                            .unwrap_or((0.0, 0.0));
                    let surface_outdoor_air_wind_speed_m_per_s = typed_surface
                        .map(|surface| {
                            energyplus_surface_outside_wind_speed_m_per_s(
                                surface,
                                energyplus_building_terrain(&model.typed),
                                weather_file_wind_speed_m_per_s,
                            )
                        })
                        .unwrap_or(weather_file_wind_speed_m_per_s);
                    if hour_index == 0 {
                        let zone_mean_air_temperature_c = state
                            .zones
                            .iter()
                            .find(|zone| zone.zone_id == surface_state.zone_id)
                            .map(|zone| zone.mean_air_temperature_c)
                            .unwrap_or(f64::NAN);
                        surface_first_sample_trace.push(HeatBalanceSurfaceFirstSampleTrace {
                            surface_name: surface_state.surface_name.clone(),
                            construction_name: surface_state.construction_name.clone(),
                            timestep_index: substep,
                            outdoor_dry_bulb_c: timestep_outdoor_dry_bulb_c,
                            zone_mean_air_temperature_c,
                            inside_face_temperature_c: surface_state.inside_face_temperature_c,
                            inside_convection_input_inside_face_temperature_c: surface_state
                                .inside_convection_input_inside_face_temperature_c,
                            inside_convection_input_reference_air_temperature_c: surface_state
                                .inside_convection_input_reference_air_temperature_c,
                            outside_face_temperature_c,
                            inside_convection_heat_gain_rate_w: inside_convection_heat_gain_rate,
                            inside_net_surface_thermal_radiation_heat_gain_rate_w:
                                inside_net_surface_thermal_radiation_heat_gain_rate,
                            inside_conduction_rate_w: inside_rate,
                            outside_conduction_rate_w: outside_rate,
                            heat_storage_rate_w: storage_rate,
                            outside_convection_heat_gain_rate_w: exterior_terms
                                .convection_heat_gain_rate_w,
                            outside_net_thermal_radiation_heat_gain_rate_w: exterior_terms
                                .net_thermal_radiation_heat_gain_rate_w,
                            outside_solar_radiation_heat_gain_rate_w: exterior_terms
                                .solar_radiation_heat_gain_rate_w,
                        });
                    }
                    let sums = &mut surface_sums[index];
                    sums.inside_face_temperature_c += surface_state.inside_face_temperature_c;
                    sums.inside_adjacent_air_temperature_c +=
                        surface_state.inside_reference_air_temperature_c;
                    sums.outside_face_temperature_c += outside_face_temperature_c;
                    sums.outside_outdoor_air_dry_bulb_temperature_c +=
                        surface_outdoor_air_dry_bulb_temperature_c;
                    sums.outside_outdoor_air_wet_bulb_temperature_c +=
                        surface_outdoor_air_wet_bulb_temperature_c;
                    sums.outside_outdoor_air_wind_speed_m_per_s +=
                        surface_outdoor_air_wind_speed_m_per_s;
                    sums.outside_outdoor_air_wind_direction_deg +=
                        surface_outdoor_air_wind_direction_deg;
                    sums.inside_convection_heat_gain_rate_w += inside_convection_heat_gain_rate;
                    sums.inside_convection_heat_gain_rate_per_area_w_per_m2 +=
                        inside_convection_heat_gain_rate_per_area;
                    sums.inside_convection_coefficient_w_per_m2_k +=
                        inside_convection_coefficient_w_per_m2_k;
                    sums.inside_net_surface_thermal_radiation_heat_gain_rate_w +=
                        inside_net_surface_thermal_radiation_heat_gain_rate;
                    sums.inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2 +=
                        surface_state.inside_net_longwave_w_per_m2;
                    sums.outside_convection_heat_gain_rate_w +=
                        exterior_terms.convection_heat_gain_rate_w;
                    sums.outside_convection_heat_gain_rate_per_area_w_per_m2 +=
                        exterior_terms.convection_heat_gain_rate_per_area_w_per_m2;
                    sums.outside_convection_coefficient_w_per_m2_k +=
                        exterior_terms.convection_coefficient_w_per_m2_k;
                    sums.outside_net_thermal_radiation_heat_gain_rate_w +=
                        exterior_terms.net_thermal_radiation_heat_gain_rate_w;
                    sums.outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2 +=
                        exterior_terms.net_thermal_radiation_heat_gain_rate_per_area_w_per_m2;
                    sums.outside_thermal_radiation_to_air_coefficient_w_per_m2_k +=
                        exterior_terms.thermal_radiation_to_air_coefficient_w_per_m2_k;
                    sums.outside_thermal_radiation_to_sky_coefficient_w_per_m2_k +=
                        exterior_terms.thermal_radiation_to_sky_coefficient_w_per_m2_k;
                    sums.outside_thermal_radiation_to_ground_coefficient_w_per_m2_k +=
                        exterior_terms.thermal_radiation_to_ground_coefficient_w_per_m2_k;
                    sums.outside_solar_radiation_heat_gain_rate_w +=
                        exterior_terms.solar_radiation_heat_gain_rate_w;
                    sums.outside_solar_radiation_heat_gain_rate_per_area_w_per_m2 +=
                        exterior_terms.solar_radiation_heat_gain_rate_per_area_w_per_m2;
                    let outside_balance = surface_state.outside_balance_diagnostics;
                    sums.outside_balance_report_temperature_c +=
                        outside_balance.report_temperature_c;
                    sums.outside_balance_coefficient_temperature_c +=
                        outside_balance.coefficient_surface_temperature_c;
                    sums.outside_balance_convection_reference_temperature_c +=
                        outside_balance.convection_reference_temperature_c;
                    sums.outside_balance_equivalent_radiant_temperature_c +=
                        outside_balance.equivalent_radiant_temperature_c;
                    sums.outside_balance_radiation_coefficient_w_per_m2_k +=
                        outside_balance.outside_radiation_coefficient_w_per_m2_k;
                    sums.outside_quick_balance_inside_source_term_w_per_m2 +=
                        outside_balance.quick_net_inside_source_w_per_m2;
                    sums.outside_quick_balance_inside_balance_term_w_per_m2 +=
                        outside_balance.quick_inside_balance_term_w_per_m2;
                    sums.outside_quick_balance_numerator_w_per_m2 +=
                        outside_balance.quick_numerator_w_per_m2;
                    sums.outside_quick_balance_denominator_w_per_m2_k +=
                        outside_balance.quick_denominator_w_per_m2_k;
                    sums.outside_quick_balance_coupling_factor +=
                        outside_balance.quick_coupling_factor;
                    sums.inside_conduction_rate_w += inside_rate;
                    sums.inside_conduction_gain_rate_w += heat_gain_rate_w(inside_rate);
                    sums.inside_conduction_loss_rate_w += heat_loss_rate_w(inside_rate);
                    sums.inside_conduction_rate_per_area_w_per_m2 +=
                        surface_rate_per_area_w_per_m2(inside_rate, surface_state.area_m2);
                    sums.ctf_inside_current_outside_term_rate_w +=
                        surface_ctf_inside_current_outside_term_rate_w_for_report(
                            surface_state,
                            use_inside_ctf_outside_temperature_for_conduction_report,
                        );
                    sums.ctf_inside_current_inside_term_rate_w +=
                        surface_ctf_inside_current_inside_term_rate_w(surface_state);
                    sums.ctf_inside_history_term_rate_w +=
                        surface_ctf_inside_history_term_rate_w(surface_state);
                    sums.ctf_inside_history_temperature_term_rate_w +=
                        heat_balance_ctf_history_slot_inside_temperature_term_rate_w(
                            &state.last_ctf_history_slot_terms,
                            &surface_state.surface_name,
                        );
                    sums.ctf_inside_history_flux_term_rate_w +=
                        heat_balance_ctf_history_slot_inside_flux_term_rate_w(
                            &state.last_ctf_history_slot_terms,
                            &surface_state.surface_name,
                        );
                    sums.outside_conduction_rate_w += outside_rate;
                    sums.outside_conduction_gain_rate_w += heat_gain_rate_w(outside_rate);
                    sums.outside_conduction_loss_rate_w += heat_loss_rate_w(outside_rate);
                    sums.outside_conduction_rate_per_area_w_per_m2 +=
                        surface_rate_per_area_w_per_m2(outside_rate, surface_state.area_m2);
                    sums.ctf_outside_current_outside_term_rate_w +=
                        surface_ctf_outside_current_outside_term_rate_w_for_report(
                            surface_state,
                            use_inside_ctf_outside_temperature_for_conduction_report,
                        );
                    sums.ctf_outside_current_inside_term_rate_w +=
                        surface_ctf_outside_current_inside_term_rate_w(surface_state);
                    sums.ctf_outside_history_term_rate_w +=
                        surface_ctf_outside_history_term_rate_w(surface_state);
                    sums.heat_storage_rate_w += storage_rate;
                    sums.heat_storage_rate_per_area_w_per_m2 += storage_rate_per_area;
                }
            }
        }

        hourly_ctf_history_slots.extend(
            hourly_ctf_history_slot_accumulators
                .into_values()
                .map(|accumulator| accumulator.finalize_hourly(hour_index)),
        );
        hourly_ctf_history_slots_after_advance.extend(
            hourly_ctf_history_slot_after_advance_accumulators
                .into_values()
                .map(|accumulator| accumulator.finalize_hourly(hour_index)),
        );

        let divisor = f64::from(steps);
        for (index, (_zone_id, _zone_name, values)) in zone_temperatures.iter_mut().enumerate() {
            values.push(zone_temperature_sums[index] / divisor);
        }
        for (index, (_zone_id, _zone_name, values)) in zone_humidity_ratios.iter_mut().enumerate() {
            values.push(zone_humidity_ratio_sums[index] / divisor);
        }
        for (index, trace) in zone_conduction_rates.iter_mut().enumerate() {
            let sums = zone_conduction_sums[index];
            trace.inside_conduction_rate_w.push(sums.0 / divisor);
            trace.inside_conduction_gain_rate_w.push(sums.1 / divisor);
            trace.inside_conduction_loss_rate_w.push(sums.2 / divisor);
            trace.outside_conduction_rate_w.push(sums.3 / divisor);
            trace.outside_conduction_gain_rate_w.push(sums.4 / divisor);
            trace.outside_conduction_loss_rate_w.push(sums.5 / divisor);
        }
        inside_surface_iteration_counts.push(inside_surface_iteration_count_sum);
        for (
            index,
            (
                _zone_id,
                _zone_name,
                internal_gain_values,
                surface_convection_values,
                air_storage_values,
            ),
        ) in zone_air_heat_balance_rates.iter_mut().enumerate()
        {
            let values = match options.zone_air_report_sampling {
                HeatBalanceZoneAirReportSampling::Average => {
                    let sums = zone_air_heat_balance_sums[index];
                    (sums.0 / divisor, sums.1 / divisor, sums.2 / divisor)
                }
                HeatBalanceZoneAirReportSampling::LastSystemState => {
                    zone_air_heat_balance_last[index]
                }
            };
            internal_gain_values.push(values.0);
            surface_convection_values.push(values.1);
            air_storage_values.push(values.2);
        }
        for (index, trace) in zone_air_debug_traces.iter_mut().enumerate() {
            let sums = zone_air_debug_sums[index];
            trace
                .current_temperature_c
                .push(sums.current_temperature_c / divisor);
            trace
                .zone_timestep_average_temperature_c
                .push(sums.zone_timestep_average_temperature_c / divisor);
            trace
                .previous_temperature_1_c
                .push(sums.previous_temperature_1_c / divisor);
            trace
                .previous_temperature_2_c
                .push(sums.previous_temperature_2_c / divisor);
            trace
                .previous_temperature_3_c
                .push(sums.previous_temperature_3_c / divisor);
            trace
                .previous_system_temperature_1_c
                .push(sums.previous_system_temperature_1_c / divisor);
            trace
                .system_timestep_count
                .push(sums.system_timestep_count / divisor);
            trace.humidity_ratio.push(sums.humidity_ratio / divisor);
            trace
                .zone_timestep_average_humidity_ratio
                .push(sums.zone_timestep_average_humidity_ratio / divisor);
            trace
                .air_heat_capacity_j_per_k
                .push(sums.air_heat_capacity_j_per_k / divisor);
            trace
                .zone_timestep_air_power_cap_w_per_k
                .push(sums.zone_timestep_air_power_cap_w_per_k / divisor);
            trace
                .last_correction_air_power_cap_w_per_k
                .push(sums.last_correction_air_power_cap_w_per_k / divisor);
        }
        for (index, trace) in surface_temperatures.iter_mut().enumerate() {
            let sums = surface_sums[index];
            trace
                .inside_face_temperature_c
                .push(sums.inside_face_temperature_c / divisor);
            trace
                .inside_adjacent_air_temperature_c
                .push(sums.inside_adjacent_air_temperature_c / divisor);
            trace
                .outside_face_temperature_c
                .push(sums.outside_face_temperature_c / divisor);
            trace
                .outside_outdoor_air_dry_bulb_temperature_c
                .push(sums.outside_outdoor_air_dry_bulb_temperature_c / divisor);
            trace
                .outside_outdoor_air_wet_bulb_temperature_c
                .push(sums.outside_outdoor_air_wet_bulb_temperature_c / divisor);
            trace
                .outside_outdoor_air_wind_speed_m_per_s
                .push(sums.outside_outdoor_air_wind_speed_m_per_s / divisor);
            trace
                .outside_outdoor_air_wind_direction_deg
                .push(sums.outside_outdoor_air_wind_direction_deg / divisor);
            trace
                .inside_convection_heat_gain_rate_w
                .push(sums.inside_convection_heat_gain_rate_w / divisor);
            trace
                .inside_convection_heat_gain_rate_per_area_w_per_m2
                .push(sums.inside_convection_heat_gain_rate_per_area_w_per_m2 / divisor);
            trace
                .inside_convection_coefficient_w_per_m2_k
                .push(sums.inside_convection_coefficient_w_per_m2_k / divisor);
            trace
                .inside_net_surface_thermal_radiation_heat_gain_rate_w
                .push(sums.inside_net_surface_thermal_radiation_heat_gain_rate_w / divisor);
            trace
                .inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2
                .push(
                    sums.inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2
                        / divisor,
                );
            trace
                .outside_convection_heat_gain_rate_w
                .push(sums.outside_convection_heat_gain_rate_w / divisor);
            trace
                .outside_convection_heat_gain_rate_per_area_w_per_m2
                .push(sums.outside_convection_heat_gain_rate_per_area_w_per_m2 / divisor);
            trace
                .outside_convection_coefficient_w_per_m2_k
                .push(sums.outside_convection_coefficient_w_per_m2_k / divisor);
            trace
                .outside_net_thermal_radiation_heat_gain_rate_w
                .push(sums.outside_net_thermal_radiation_heat_gain_rate_w / divisor);
            trace
                .outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2
                .push(
                    sums.outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2 / divisor,
                );
            trace
                .outside_thermal_radiation_to_air_coefficient_w_per_m2_k
                .push(sums.outside_thermal_radiation_to_air_coefficient_w_per_m2_k / divisor);
            trace
                .outside_thermal_radiation_to_sky_coefficient_w_per_m2_k
                .push(sums.outside_thermal_radiation_to_sky_coefficient_w_per_m2_k / divisor);
            trace
                .outside_thermal_radiation_to_ground_coefficient_w_per_m2_k
                .push(sums.outside_thermal_radiation_to_ground_coefficient_w_per_m2_k / divisor);
            trace
                .outside_solar_radiation_heat_gain_rate_w
                .push(sums.outside_solar_radiation_heat_gain_rate_w / divisor);
            trace
                .outside_solar_radiation_heat_gain_rate_per_area_w_per_m2
                .push(sums.outside_solar_radiation_heat_gain_rate_per_area_w_per_m2 / divisor);
            trace
                .outside_balance_report_temperature_c
                .push(sums.outside_balance_report_temperature_c / divisor);
            trace
                .outside_balance_coefficient_temperature_c
                .push(sums.outside_balance_coefficient_temperature_c / divisor);
            trace
                .outside_balance_convection_reference_temperature_c
                .push(sums.outside_balance_convection_reference_temperature_c / divisor);
            trace
                .outside_balance_equivalent_radiant_temperature_c
                .push(sums.outside_balance_equivalent_radiant_temperature_c / divisor);
            trace
                .outside_balance_radiation_coefficient_w_per_m2_k
                .push(sums.outside_balance_radiation_coefficient_w_per_m2_k / divisor);
            trace
                .outside_quick_balance_inside_source_term_w_per_m2
                .push(sums.outside_quick_balance_inside_source_term_w_per_m2 / divisor);
            trace
                .outside_quick_balance_inside_balance_term_w_per_m2
                .push(sums.outside_quick_balance_inside_balance_term_w_per_m2 / divisor);
            trace
                .outside_quick_balance_numerator_w_per_m2
                .push(sums.outside_quick_balance_numerator_w_per_m2 / divisor);
            trace
                .outside_quick_balance_denominator_w_per_m2_k
                .push(sums.outside_quick_balance_denominator_w_per_m2_k / divisor);
            trace
                .outside_quick_balance_coupling_factor
                .push(sums.outside_quick_balance_coupling_factor / divisor);
            trace
                .inside_conduction_rate_w
                .push(sums.inside_conduction_rate_w / divisor);
            trace
                .inside_conduction_gain_rate_w
                .push(sums.inside_conduction_gain_rate_w / divisor);
            trace
                .inside_conduction_loss_rate_w
                .push(sums.inside_conduction_loss_rate_w / divisor);
            trace
                .inside_conduction_rate_per_area_w_per_m2
                .push(sums.inside_conduction_rate_per_area_w_per_m2 / divisor);
            trace
                .ctf_inside_current_outside_term_rate_w
                .push(sums.ctf_inside_current_outside_term_rate_w / divisor);
            trace
                .ctf_inside_current_inside_term_rate_w
                .push(sums.ctf_inside_current_inside_term_rate_w / divisor);
            trace
                .ctf_inside_history_term_rate_w
                .push(sums.ctf_inside_history_term_rate_w / divisor);
            trace
                .ctf_inside_history_temperature_term_rate_w
                .push(sums.ctf_inside_history_temperature_term_rate_w / divisor);
            trace
                .ctf_inside_history_flux_term_rate_w
                .push(sums.ctf_inside_history_flux_term_rate_w / divisor);
            trace
                .outside_conduction_rate_w
                .push(sums.outside_conduction_rate_w / divisor);
            trace
                .outside_conduction_gain_rate_w
                .push(sums.outside_conduction_gain_rate_w / divisor);
            trace
                .outside_conduction_loss_rate_w
                .push(sums.outside_conduction_loss_rate_w / divisor);
            trace
                .outside_conduction_rate_per_area_w_per_m2
                .push(sums.outside_conduction_rate_per_area_w_per_m2 / divisor);
            trace
                .ctf_outside_current_outside_term_rate_w
                .push(sums.ctf_outside_current_outside_term_rate_w / divisor);
            trace
                .ctf_outside_current_inside_term_rate_w
                .push(sums.ctf_outside_current_inside_term_rate_w / divisor);
            trace
                .ctf_outside_history_term_rate_w
                .push(sums.ctf_outside_history_term_rate_w / divisor);
            trace
                .heat_storage_rate_w
                .push(sums.heat_storage_rate_w / divisor);
            trace
                .heat_storage_rate_per_area_w_per_m2
                .push(sums.heat_storage_rate_per_area_w_per_m2 / divisor);
        }
        outdoor_temperatures.push(outdoor_temperature_sum / divisor);
        outdoor_wet_bulb_temperatures.push(outdoor_wet_bulb_temperature_sum / divisor);
        sky_temperatures.push(sky_temperature_sum / divisor);
        horizontal_infrared_radiation_rates.push(horizontal_infrared_radiation_sum / divisor);
        rain_statuses.push(rain_status_sum / divisor);
    }

    let mut results = ResultStore::new();
    let mut handle_index = 0;
    for (_zone_id, zone_name, values) in zone_temperatures {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name,
            variable_name: "Zone Mean Air Temperature".to_string(),
            units: "C".to_string(),
            values,
        });
        handle_index += 1;
    }
    for (_zone_id, zone_name, values) in zone_humidity_ratios {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name,
            variable_name: "Zone Mean Air Humidity Ratio".to_string(),
            units: "kgWater/kgDryAir".to_string(),
            values,
        });
        handle_index += 1;
    }
    for trace in zone_conduction_rates {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: "Zone Opaque Surface Inside Faces Conduction Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: "Zone Opaque Surface Inside Faces Conduction Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: "Zone Opaque Surface Inside Faces Conduction Heat Loss Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_loss_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: "Zone Opaque Surface Outside Faces Conduction Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: "Zone Opaque Surface Outside Faces Conduction Heat Gain Rate"
                .to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name,
            variable_name: "Zone Opaque Surface Outside Faces Conduction Heat Loss Rate"
                .to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_loss_rate_w,
        });
        handle_index += 1;
    }
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Simulation".to_string(),
        variable_name: SURFACE_INSIDE_HEAT_BALANCE_ITERATION_COUNT_VARIABLE.to_string(),
        units: String::new(),
        values: inside_surface_iteration_counts,
    });
    handle_index += 1;
    for (
        _zone_id,
        zone_name,
        internal_gain_values,
        surface_convection_values,
        air_storage_values,
    ) in zone_air_heat_balance_rates
    {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name.clone(),
            variable_name: "Zone Air Heat Balance Internal Convective Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: internal_gain_values,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name.clone(),
            variable_name: "Zone Air Heat Balance Surface Convection Rate".to_string(),
            units: "W".to_string(),
            values: surface_convection_values,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: zone_name,
            variable_name: "Zone Air Heat Balance Air Energy Storage Rate".to_string(),
            units: "W".to_string(),
            values: air_storage_values,
        });
        handle_index += 1;
    }
    for trace in zone_air_debug_traces {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_CURRENT_TEMPERATURE_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.current_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_ZONE_TIMESTEP_AVERAGE_TEMPERATURE_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.zone_timestep_average_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_PREVIOUS_TEMPERATURE_1_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.previous_temperature_1_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_PREVIOUS_TEMPERATURE_2_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.previous_temperature_2_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_PREVIOUS_TEMPERATURE_3_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.previous_temperature_3_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_PREVIOUS_SYSTEM_TEMPERATURE_1_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.previous_system_temperature_1_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_SYSTEM_TIMESTEP_COUNT_VARIABLE.to_string(),
            units: String::new(),
            values: trace.system_timestep_count,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_HUMIDITY_RATIO_VARIABLE.to_string(),
            units: "kgWater/kgDryAir".to_string(),
            values: trace.humidity_ratio,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_ZONE_TIMESTEP_AVERAGE_HUMIDITY_RATIO_VARIABLE.to_string(),
            units: "kgWater/kgDryAir".to_string(),
            values: trace.zone_timestep_average_humidity_ratio,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_HEAT_CAPACITY_VARIABLE.to_string(),
            units: "J/K".to_string(),
            values: trace.air_heat_capacity_j_per_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name.clone(),
            variable_name: RUST_ZONE_AIR_ZONE_TIMESTEP_AIR_POWER_CAP_VARIABLE.to_string(),
            units: "W/K".to_string(),
            values: trace.zone_timestep_air_power_cap_w_per_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.zone_name,
            variable_name: RUST_ZONE_AIR_LAST_CORRECTION_AIR_POWER_CAP_VARIABLE.to_string(),
            units: "W/K".to_string(),
            values: trace.last_correction_air_power_cap_w_per_k,
        });
        handle_index += 1;
    }
    for trace in surface_temperatures {
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Temperature".to_string(),
            units: "C".to_string(),
            values: trace.inside_face_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Adjacent Air Temperature".to_string(),
            units: "C".to_string(),
            values: trace.inside_adjacent_air_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Temperature".to_string(),
            units: "C".to_string(),
            values: trace.outside_face_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Outdoor Air Drybulb Temperature".to_string(),
            units: "C".to_string(),
            values: trace.outside_outdoor_air_dry_bulb_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Outdoor Air Wetbulb Temperature".to_string(),
            units: "C".to_string(),
            values: trace.outside_outdoor_air_wet_bulb_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Outdoor Air Wind Speed".to_string(),
            units: "m/s".to_string(),
            values: trace.outside_outdoor_air_wind_speed_m_per_s,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Outdoor Air Wind Direction".to_string(),
            units: "deg".to_string(),
            values: trace.outside_outdoor_air_wind_direction_deg,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Convection Heat Transfer Coefficient".to_string(),
            units: "W/m2-K".to_string(),
            values: trace.inside_convection_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Convection Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_convection_heat_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Convection Heat Gain Rate per Area".to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_convection_heat_gain_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate"
                .to_string(),
            units: "W".to_string(),
            values: trace.inside_net_surface_thermal_radiation_heat_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name:
                "Surface Inside Face Net Surface Thermal Radiation Heat Gain Rate per Area"
                    .to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_net_surface_thermal_radiation_heat_gain_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Convection Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_convection_heat_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Convection Heat Gain Rate per Area".to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_convection_heat_gain_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Convection Heat Transfer Coefficient".to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_convection_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Net Thermal Radiation Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_net_thermal_radiation_heat_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Net Thermal Radiation Heat Gain Rate per Area"
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_net_thermal_radiation_heat_gain_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name:
                "Surface Outside Face Thermal Radiation to Air Heat Transfer Coefficient"
                    .to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_thermal_radiation_to_air_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name:
                "Surface Outside Face Thermal Radiation to Sky Heat Transfer Coefficient"
                    .to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_thermal_radiation_to_sky_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name:
                "Surface Outside Face Thermal Radiation to Ground Heat Transfer Coefficient"
                    .to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_thermal_radiation_to_ground_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Solar Radiation Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_solar_radiation_heat_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Solar Radiation Heat Gain Rate per Area"
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_solar_radiation_heat_gain_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_BALANCE_REPORT_TEMPERATURE_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.outside_balance_report_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_BALANCE_COEFFICIENT_TEMPERATURE_VARIABLE.to_string(),
            units: "C".to_string(),
            values: trace.outside_balance_coefficient_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_BALANCE_CONVECTION_REFERENCE_TEMPERATURE_VARIABLE
                .to_string(),
            units: "C".to_string(),
            values: trace.outside_balance_convection_reference_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_BALANCE_EQUIVALENT_RADIANT_TEMPERATURE_VARIABLE
                .to_string(),
            units: "C".to_string(),
            values: trace.outside_balance_equivalent_radiant_temperature_c,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_BALANCE_RADIATION_COEFFICIENT_VARIABLE.to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_balance_radiation_coefficient_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_QUICK_BALANCE_INSIDE_SOURCE_TERM_VARIABLE.to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_quick_balance_inside_source_term_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_QUICK_BALANCE_INSIDE_BALANCE_TERM_VARIABLE.to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_quick_balance_inside_balance_term_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_QUICK_BALANCE_NUMERATOR_VARIABLE.to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_quick_balance_numerator_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_QUICK_BALANCE_DENOMINATOR_VARIABLE.to_string(),
            units: "W/m2-K".to_string(),
            values: trace.outside_quick_balance_denominator_w_per_m2_k,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_OUTSIDE_QUICK_BALANCE_COUPLING_FACTOR_VARIABLE.to_string(),
            units: String::new(),
            values: trace.outside_quick_balance_coupling_factor,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Transfer Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Loss Rate".to_string(),
            units: "W".to_string(),
            values: trace.inside_conduction_loss_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Inside Face Conduction Heat Transfer Rate per Area".to_string(),
            units: "W/m2".to_string(),
            values: trace.inside_conduction_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_INSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_inside_current_outside_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_INSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_inside_current_inside_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_inside_history_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_INSIDE_HISTORY_TEMPERATURE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_inside_history_temperature_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_INSIDE_HISTORY_FLUX_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_inside_history_flux_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Transfer Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Gain Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_gain_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Loss Rate".to_string(),
            units: "W".to_string(),
            values: trace.outside_conduction_loss_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Outside Face Conduction Heat Transfer Rate per Area"
                .to_string(),
            units: "W/m2".to_string(),
            values: trace.outside_conduction_rate_per_area_w_per_m2,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_OUTSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_outside_current_outside_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_OUTSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_outside_current_inside_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: SURFACE_CTF_OUTSIDE_HISTORY_TERM_RATE_VARIABLE.to_string(),
            units: "W".to_string(),
            values: trace.ctf_outside_history_term_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name.clone(),
            variable_name: "Surface Heat Storage Rate".to_string(),
            units: "W".to_string(),
            values: trace.heat_storage_rate_w,
        });
        handle_index += 1;
        results.add_series(OutputSeries {
            handle: OutputHandle(handle_index),
            key: trace.surface_name,
            variable_name: "Surface Heat Storage Rate per Area".to_string(),
            units: "W/m2".to_string(),
            values: trace.heat_storage_rate_per_area_w_per_m2,
        });
        handle_index += 1;
    }
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Outdoor Air Drybulb Temperature".to_string(),
        units: "C".to_string(),
        values: outdoor_temperatures,
    });
    handle_index += 1;
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Outdoor Air Wetbulb Temperature".to_string(),
        units: "C".to_string(),
        values: outdoor_wet_bulb_temperatures,
    });
    handle_index += 1;
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Sky Temperature".to_string(),
        units: "C".to_string(),
        values: sky_temperatures,
    });
    handle_index += 1;
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Horizontal Infrared Radiation Rate per Area".to_string(),
        units: "W/m2".to_string(),
        values: horizontal_infrared_radiation_rates,
    });
    handle_index += 1;
    results.add_series(OutputSeries {
        handle: OutputHandle(handle_index),
        key: "Environment".to_string(),
        variable_name: "Site Rain Status".to_string(),
        units: String::new(),
        values: rain_statuses,
    });

    let summary = HeatBalanceSimulationSummary {
        samples: options.sample_count,
        timestep_count: state.timestep_index,
        run_period_timestep_count: state.timestep_index - run_period_timestep_start,
        warmup,
        zone_count: state.zones.len(),
        surface_count: state.surfaces.len(),
        surface_iteration_count: options.surface_iteration_count,
        inside_hconv_reevaluation_interval: options.inside_hconv_reevaluation_interval,
        ctf_initial_history_policy: options.ctf_initial_history_policy,
        zone_conduction_report_source: options.zone_conduction_report_source,
        zone_air_report_sampling: options.zone_air_report_sampling,
        surface_loop_zone_air_correction: options.surface_loop_zone_air_correction,
        run_period_initial_zone_air_states,
        warmup_day_end_zone_air_states,
        run_period_initial_ctf_history_slots,
        first_sample_ctf_history_slots: first_sample_ctf_history_slot_accumulators
            .into_values()
            .map(HeatBalanceCtfHistorySlotFirstSampleAccumulator::finalize)
            .collect(),
        hourly_ctf_history_slots,
        hourly_ctf_history_slots_after_advance,
        surface_first_sample_trace,
        zone_air_first_sample_trace,
        surface_iteration_first_sample_trace,
        surface_iteration_sample_trace,
    };

    Ok(HeatBalanceSimulation {
        state,
        results,
        summary,
    })
}

fn run_heat_balance_run_period_warmup(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    weather_dry_bulb_c: &[f64],
    weather_records: Option<&[EpwRecord]>,
    zone_steps_per_hour: u32,
    seconds_per_timestep: f64,
    options: HeatBalanceWarmupOptions,
    zone_air_algorithm: HeatBalanceZoneAirAlgorithm,
    surface_iteration_count: u32,
    inside_hconv_reevaluation_interval: Option<u32>,
    surface_loop_zone_air_correction: HeatBalanceSurfaceLoopZoneAirCorrection,
    first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues,
    day_end_zone_air_states: &mut Vec<HeatBalanceWarmupDayEndZoneAirStateSample>,
) -> HeatBalanceWarmupSummary {
    if !options.enabled || options.maximum_days == 0 || weather_dry_bulb_c.is_empty() {
        return HeatBalanceWarmupSummary::disabled();
    }

    let hours_per_day = weather_dry_bulb_c.len().min(24);
    let maximum_days = options.maximum_days.max(options.minimum_days).max(1);
    let tolerance = options.temperature_convergence_tolerance_delta_c.max(0.0);
    let timestep_start = state.timestep_index;
    let mut previous_day_end_temperatures: Option<Vec<f64>> = None;
    let mut final_delta = f64::INFINITY;

    for day in 1..=maximum_days {
        for (hour_index, outdoor_dry_bulb_c) in weather_dry_bulb_c
            .iter()
            .copied()
            .take(hours_per_day)
            .enumerate()
        {
            let hour_ending = u32::try_from(hour_index % 24 + 1).unwrap_or(24);
            let steps = zone_steps_per_hour.max(1);
            for substep in 1..=steps {
                let timestep_outdoor_dry_bulb_c =
                    energyplus_weather_dry_bulb_at_timestep_with_starting_values(
                        weather_records,
                        hour_index,
                        outdoor_dry_bulb_c,
                        steps,
                        substep,
                        first_hour_interpolation_starting_values,
                    );
                let weather_context = heat_balance_weather_context_for_timestep(
                    weather_records,
                    hour_index,
                    steps,
                    substep,
                    first_hour_interpolation_starting_values,
                );
                advance_heat_balance_state_one_timestep_internal(
                    model,
                    state,
                    HeatBalanceStepInput {
                        outdoor_dry_bulb_c: timestep_outdoor_dry_bulb_c,
                        hour_ending,
                        timestep_seconds: seconds_per_timestep,
                    },
                    weather_context,
                    zone_air_algorithm,
                    surface_iteration_count,
                    inside_hconv_reevaluation_interval,
                    surface_loop_zone_air_correction,
                );
            }
        }

        let day_end_temperatures = heat_balance_zone_temperature_snapshot(state);
        day_end_zone_air_states.extend(state.zones.iter().map(|zone| {
            HeatBalanceWarmupDayEndZoneAirStateSample {
                day_index: day,
                state: heat_balance_zone_air_state_sample(zone),
            }
        }));
        if let Some(previous_temperatures) = &previous_day_end_temperatures {
            final_delta = max_abs_pair_delta(
                previous_temperatures.as_slice(),
                day_end_temperatures.as_slice(),
            );
            if day >= options.minimum_days && final_delta <= tolerance {
                return HeatBalanceWarmupSummary {
                    enabled: true,
                    day_count: day,
                    timestep_count: state.timestep_index - timestep_start,
                    hours_per_day,
                    converged: true,
                    final_max_zone_temperature_delta_c: final_delta,
                };
            }
        }
        previous_day_end_temperatures = Some(day_end_temperatures);
    }

    HeatBalanceWarmupSummary {
        enabled: true,
        day_count: maximum_days,
        timestep_count: state.timestep_index - timestep_start,
        hours_per_day,
        converged: false,
        final_max_zone_temperature_delta_c: final_delta,
    }
}

fn heat_balance_zone_temperature_snapshot(state: &HeatBalanceState) -> Vec<f64> {
    state
        .zones
        .iter()
        .map(|zone| zone.mean_air_temperature_c)
        .collect()
}

fn max_abs_pair_delta(left: &[f64], right: &[f64]) -> f64 {
    left.iter()
        .zip(right.iter())
        .map(|(left, right)| (left - right).abs())
        .fold(0.0, f64::max)
}

fn heat_gain_rate_w(rate_w: f64) -> f64 {
    rate_w.max(0.0)
}

fn heat_loss_rate_w(rate_w: f64) -> f64 {
    (-rate_w).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::{
        ConstructionCtfCoefficientOverride, CtfInsideFaceBalanceInput, CtfOutsideFaceBalanceInput,
        CtfOutsideQuickConductionBalanceInput,
        ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C,
        ENERGYPLUS_DEFAULT_WEATHER_FILE_TEMPERATURE_SENSOR_HEIGHT_M,
        ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K, ENERGYPLUS_ZONE_INITIAL_TEMP_C, EpwRecord,
        FirstZoneSimulationOptions, HeatBalanceCtfInitialHistoryPolicy,
        HeatBalanceSimulationOptions, HeatBalanceStepInput,
        HeatBalanceSurfaceLoopZoneAirCorrection, HeatBalanceWarmupOptions,
        HeatBalanceWarmupSummary, HeatBalanceWeatherContext, HeatBalanceZoneAirReportSampling,
        HeatBalanceZoneConductionReportSource, InteriorLongwaveExchangeProbe,
        InteriorLongwaveSurfaceSnapshot, KELVIN_OFFSET, OutputSeries,
        QuickOutsideConductionContext, ResultStore, RuntimeError, SECONDS_PER_HOUR,
        STEFAN_BOLTZMANN_W_PER_M2_K4, SimulationMode, SimulationState,
        SurfaceBoundaryBalanceResult, SurfaceCtfState, SurfaceExteriorReportTerms,
        SurfaceOutsideBalanceDiagnostics, advance_heat_balance_state_one_timestep,
        advance_heat_balance_state_one_timestep_internal, advance_surface_ctf_histories,
        advance_surface_ctf_histories_with_outside_temperature_override,
        append_surface_incident_solar_radiation_series,
        apply_energyplus_adaptive_system_timestep_zone_air_correction,
        energyplus_analytical_zone_air_temperature_c, energyplus_anisotropic_sky_multiplier,
        energyplus_approximate_view_factors, energyplus_ashrae_tarp_natural_convection_w_per_m2_k,
        energyplus_average_solar_coefficients, energyplus_ctf_inside_face_temperature_c,
        energyplus_ctf_outside_face_temperature_c,
        energyplus_ctf_outside_face_temperature_quick_conduction_c,
        energyplus_daily_solar_coefficients,
        energyplus_doe2_outside_convection_coefficient_w_per_m2_k,
        energyplus_exterior_longwave_terms, energyplus_exterior_wet_context_fraction,
        energyplus_exterior_wet_timestep_fraction,
        energyplus_linearized_radiation_coefficient_w_per_m2_k,
        energyplus_moist_air_density_kg_per_m3, energyplus_moist_air_specific_heat_j_per_kg_k,
        energyplus_outdoor_wet_bulb_c, energyplus_scriptf_from_view_factors,
        energyplus_shadowing_period_solar_coefficients,
        energyplus_surface_outdoor_air_temperature_c,
        energyplus_surface_outside_wind_speed_m_per_s,
        energyplus_tarp_inside_convection_coefficient_w_per_m2_k,
        energyplus_third_order_zone_air_temperature_c,
        energyplus_weather_atmospheric_pressure_at_timestep,
        energyplus_weather_dry_bulb_at_timestep,
        energyplus_weather_dry_bulb_at_timestep_with_starting_values,
        energyplus_weather_horizontal_infrared_at_timestep, energyplus_weather_record_day_of_year,
        energyplus_weather_record_is_rain_at_timestep,
        energyplus_weather_relative_humidity_at_timestep,
        energyplus_weather_wind_direction_at_timestep, energyplus_weather_wind_speed_at_timestep,
        energyplus_zone_air_heat_capacity_j_per_k, energyplus_zone_air_temperature_coefficients,
        exterior_surface_energy_balance, fix_energyplus_approximate_view_factors,
        heat_balance_uses_balance_surface_convection_report,
        heat_balance_uses_doe2_outside_convection,
        heat_balance_uses_surface_reference_air_convection_report,
        heat_balance_uses_surface_reference_air_surface_convection_report,
        horizontal_infrared_sky_temperature_c, initialize_heat_balance_state,
        initialize_heat_balance_state_with_ctf_coefficients,
        inside_ctf_outside_temperature_history_commit_override_c, parse_epw_dry_bulb_series,
        parse_epw_records, run_heat_balance_run_period_warmup, run_surface_balance_passes,
        seed_energyplus_initial_surface_ctf_histories, seed_initial_surface_ctf_boundary_histories,
        simulate_constant_schedules, simulate_first_zone_uncontrolled,
        simulate_heat_balance_zone_air_temperatures,
        simulate_heat_balance_zone_air_temperatures_internal,
        simulate_heat_balance_zone_air_temperatures_with_weather_records, simulate_schedule_values,
        simulate_zone_internal_convective_gains, solar_position_rad_at_local_hour,
        solar_weather_interpolation_weights, surface_air_sky_radiation_split, surface_area_m2,
        surface_azimuth_deg, surface_ctf_history_slot_samples, surface_exterior_report_terms,
        surface_geometry_summaries, surface_heat_storage_rate_w,
        surface_incident_solar_components_hourly_average_w_per_m2,
        surface_incident_solar_radiation_for_weather_context_w_per_m2,
        surface_inside_conduction_flux_w_per_m2, surface_inside_conduction_rate_w,
        surface_inside_convection_heat_gain_rate_per_area_w_per_m2,
        surface_inside_convection_report_coefficient_w_per_m2_k,
        surface_inside_ctf_source_terms_w_per_m2, surface_outside_conduction_flux_w_per_m2,
        surface_outside_conduction_rate_w, surface_sky_view_factor,
        surface_steady_u_value_w_per_m2_k, surface_tilt_deg, update_surface_ctf_history_constants,
        update_surface_inside_longwave_exchange_probe,
        update_surface_inside_scriptf_longwave_exchange_probe,
        update_surface_radiant_internal_gain_source_terms,
        update_zone_air_heat_capacities_from_weather_context,
        zone_air_heat_balance_air_storage_rate_w,
        zone_air_heat_balance_surface_convection_rate_at_air_temperature_w,
        zone_air_heat_balance_surface_convection_rate_from_balance_w,
        zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w,
        zone_air_heat_balance_surface_convection_rate_w,
        zone_air_system_timestep_storage_report_rate_w, zone_geometry_summaries,
        zone_surface_report_conduction_rates_w,
    };
    use crate::heat_balance::{HeatBalanceAlgorithmLane, HeatBalanceZoneAirAlgorithm};
    use crate::node::{
        NODE_STATE_EXCLUDED_SETPOINT_VARIABLE, NODE_STATE_SOURCE_MAP_PATH,
        NODE_TEMPERATURE_SETPOINT_SENTINEL_C, NodeStateProjectionOptions, NodeStateRole,
        NodeStateStore, node_temperature_setpoint_from_energyplus,
        simulate_ideal_loads_node_state_projection,
    };
    use crate::time_axis::{Date, next_day};
    use crate::{
        ExecutionStage, ExecutionStageKind, ExecutionStep, RuntimeOutputRegistry,
        build_execution_plan, build_hourly_time_axis, build_hourly_time_axis_for_run_period,
        energyplus_heat_balance_compatibility_stages,
    };
    use crate::{
        RuntimeDiagnosticCode, RuntimeMeterRequest, RuntimeOutputFrequency, RuntimeOutputRequest,
    };
    use ep_model::{
        AutoOrNumber, AutosizeOrNumber, Construction, ConstructionId, DehumidificationControlType,
        DemandControlledVentilationType, FirstHourInterpolationStartingValues, HeatRecoveryType,
        HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsFuelType,
        IdealLoadsLimit, InternalGainId, LoadDistributionScheme, Material, MaterialId,
        MaterialKind, MaterialSurfaceRoughness, Node, NodeId, NodeList, NodeListId, NormalizedName,
        OtherEquipment, OutdoorAirEconomizerType, OutputHandle, OutsideBoundaryCondition,
        OutsideSurfaceConvectionAlgorithm, Point3, RunPeriod, RunPeriodId, ScheduleCompact,
        ScheduleCompactSegment, ScheduleConstant, ScheduleId, SimulationModel, SiteLocation,
        SunExposure, Surface, SurfaceId, SurfaceType, Terrain, ThermostatControlObjectType,
        ThermostatDualSetpoint, ThermostatSetpointId, TimestepConfig, TypedModel, WindExposure,
        Zone, ZoneEquipmentConnection, ZoneEquipmentConnectionId, ZoneEquipmentList,
        ZoneEquipmentListEntry, ZoneEquipmentListId, ZoneEquipmentObjectType, ZoneId,
        ZoneThermostat, ZoneThermostatControl, ZoneThermostatId,
    };
    use std::collections::BTreeMap;

    #[test]
    fn state_defaults_to_first_timestep() {
        let state = SimulationState::new(SimulationMode::Compatibility);

        assert_eq!(state.timestep_index, 0);
        assert_eq!(state.mode, SimulationMode::Compatibility);
        assert!(state.zones.is_empty());
    }

    #[test]
    fn solar_weather_interpolation_matches_energyplus_even_timestep_weights() {
        assert_eq!(solar_weather_interpolation_weights(4, 1), (0.25, 0.75, 0.0));
        assert_eq!(solar_weather_interpolation_weights(4, 2), (0.0, 1.0, 0.0));
        assert_eq!(solar_weather_interpolation_weights(4, 3), (0.0, 0.75, 0.25));
        assert_eq!(solar_weather_interpolation_weights(4, 4), (0.0, 0.5, 0.5));
    }

    #[test]
    fn energyplus_daily_solar_coefficients_match_reference_day() {
        let (sin_declination, _cos_declination, equation_of_time_hours) =
            energyplus_daily_solar_coefficients(1);

        assert!((sin_declination - -0.392204631085).abs() < 1.0e-12);
        assert!((equation_of_time_hours - -0.055895327979).abs() < 1.0e-12);
    }

    #[test]
    fn energyplus_weather_record_day_of_year_ignores_tmy_source_leap_year() {
        let mut record = EpwRecord {
            year: 2004,
            month: 3,
            day: 1,
            hour: 1,
            minute: 60,
            dry_bulb_c: 0.0,
            dew_point_c: 0.0,
            relative_humidity_percent: 0.0,
            atmospheric_pressure_pa: 101_325.0,
            horizontal_infrared_radiation_wh_per_m2: 0.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 0.0,
            wind_speed_m_per_s: 0.0,
            liquid_precipitation_depth_mm: 0.0,
        };

        assert_eq!(energyplus_weather_record_day_of_year(&record), Some(60));

        record.month = 4;
        record.day = 6;
        assert_eq!(energyplus_weather_record_day_of_year(&record), Some(96));

        record.year = 2013;
        assert_eq!(energyplus_weather_record_day_of_year(&record), Some(96));
    }

    #[test]
    fn energyplus_average_solar_coefficients_match_shadowing_period() {
        let (sin_declination, cos_declination, equation_of_time_hours) =
            energyplus_average_solar_coefficients(61, 20);

        assert!((sin_declination - -0.065802703719632).abs() < 1.0e-12);
        assert!((cos_declination - 0.997832653395942).abs() < 1.0e-12);
        assert!((equation_of_time_hours - -0.168373861452452).abs() < 1.0e-12);
    }

    #[test]
    fn shadowing_period_solar_coefficients_use_energyplus_update_frequency() {
        let mut records = Vec::new();
        let mut date = Date {
            year: 2013,
            month: 1,
            day_of_month: 1,
        };
        for _day in 0..80 {
            for hour in 1..=24 {
                records.push(EpwRecord {
                    year: date.year,
                    month: date.month,
                    day: date.day_of_month,
                    hour,
                    minute: 60,
                    dry_bulb_c: 0.0,
                    dew_point_c: 0.0,
                    relative_humidity_percent: 0.0,
                    atmospheric_pressure_pa: 101_325.0,
                    horizontal_infrared_radiation_wh_per_m2: 0.0,
                    global_horizontal_radiation_wh_per_m2: 0.0,
                    direct_normal_radiation_wh_per_m2: 0.0,
                    diffuse_horizontal_radiation_wh_per_m2: 0.0,
                    wind_direction_deg: 0.0,
                    wind_speed_m_per_s: 0.0,
                    liquid_precipitation_depth_mm: 0.0,
                });
            }
            date = next_day(date);
        }

        let coefficients = energyplus_shadowing_period_solar_coefficients(&records, 1450);
        assert!(coefficients.is_some());
        let (sin_declination, cos_declination, equation_of_time_hours) =
            coefficients.unwrap_or((0.0, 0.0, 0.0));

        assert!((sin_declination - -0.065802703719632).abs() < 1.0e-12);
        assert!((cos_declination - 0.997832653395942).abs() < 1.0e-12);
        assert!((equation_of_time_hours - -0.168373861452452).abs() < 1.0e-12);
    }

    #[test]
    fn solar_position_uses_energyplus_hour_angle_convention() {
        let site = SiteLocation {
            name: NormalizedName::new("Chicago"),
            latitude_deg: 41.78,
            longitude_deg: -87.75,
            time_zone_hours: -6.0,
            elevation_m: 190.0,
        };
        let record = EpwRecord {
            year: 2013,
            month: 1,
            day: 1,
            hour: 12,
            minute: 60,
            dry_bulb_c: 0.0,
            dew_point_c: 0.0,
            relative_humidity_percent: 0.0,
            atmospheric_pressure_pa: 101_325.0,
            horizontal_infrared_radiation_wh_per_m2: 0.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 0.0,
            wind_speed_m_per_s: 0.0,
            liquid_precipitation_depth_mm: 0.0,
        };

        let position = solar_position_rad_at_local_hour(&site, &record, 12.0);
        assert!(position.is_some());
        let (altitude_rad, azimuth_rad) = position.unwrap_or((0.0, 0.0));

        assert!((altitude_rad.to_degrees() - 25.115079268192).abs() < 1.0e-12);
        assert!((azimuth_rad.to_degrees() - 181.434056277464).abs() < 1.0e-12);
    }

    #[test]
    fn surface_solar_uses_shadowing_sunlit_fraction_at_sunrise_edge() {
        let site = SiteLocation {
            name: NormalizedName::new("Golden"),
            latitude_deg: 39.74,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        };
        let mut records = Vec::new();
        let mut record_index = None;
        let mut date = Date {
            year: 2004,
            month: 1,
            day_of_month: 1,
        };
        for _day in 0..340 {
            for hour in 1..=24 {
                if date.month == 11 && date.day_of_month == 19 && hour == 7 {
                    record_index = Some(records.len());
                }
                let (direct_normal_radiation_wh_per_m2, diffuse_horizontal_radiation_wh_per_m2) =
                    if date.month == 11 && date.day_of_month == 19 && hour == 8 {
                        (279.0, 56.0)
                    } else {
                        (0.0, 0.0)
                    };
                records.push(EpwRecord {
                    year: date.year,
                    month: date.month,
                    day: date.day_of_month,
                    hour,
                    minute: 0,
                    dry_bulb_c: 0.0,
                    dew_point_c: 0.0,
                    relative_humidity_percent: 50.0,
                    atmospheric_pressure_pa: 82_000.0,
                    horizontal_infrared_radiation_wh_per_m2: 0.0,
                    global_horizontal_radiation_wh_per_m2: 0.0,
                    direct_normal_radiation_wh_per_m2,
                    diffuse_horizontal_radiation_wh_per_m2,
                    wind_direction_deg: 0.0,
                    wind_speed_m_per_s: 0.0,
                    liquid_precipitation_depth_mm: 0.0,
                });
            }
            date = next_day(date);
        }
        let roof = surface(
            100,
            "Sunrise Roof",
            SurfaceType::Roof,
            [
                point(0.0, 0.0, 1.0),
                point(0.0, 1.0, 1.0),
                point(1.0, 1.0, 1.0),
                point(1.0, 0.0, 1.0),
            ],
        );

        let incident = surface_incident_solar_radiation_for_weather_context_w_per_m2(
            &roof,
            &site,
            &records,
            record_index.unwrap_or(0),
            4,
            None,
            FirstHourInterpolationStartingValues::Hour24,
        );

        assert!((incident - 6.003845309857875).abs() < 1.0e-9);
    }

    #[test]
    fn horizontal_roof_sky_diffuse_matches_energyplus_shadowing_sunrise_edge() {
        let site = SiteLocation {
            name: NormalizedName::new("Golden"),
            latitude_deg: 39.74,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        };
        let mut records = Vec::new();
        let mut record_index = None;
        let mut date = Date {
            year: 2004,
            month: 1,
            day_of_month: 1,
        };
        for _day in 0..117 {
            for hour in 1..=24 {
                if date.month == 4 && date.day_of_month == 26 && hour == 6 {
                    record_index = Some(records.len());
                }
                let (direct_normal_radiation_wh_per_m2, diffuse_horizontal_radiation_wh_per_m2) =
                    if date.month == 4 && date.day_of_month == 26 && hour == 6 {
                        (0.0, 42.0)
                    } else if date.month == 4 && date.day_of_month == 26 && hour == 7 {
                        (626.0, 70.0)
                    } else {
                        (0.0, 0.0)
                    };
                records.push(EpwRecord {
                    year: date.year,
                    month: date.month,
                    day: date.day_of_month,
                    hour,
                    minute: 0,
                    dry_bulb_c: 0.0,
                    dew_point_c: 0.0,
                    relative_humidity_percent: 50.0,
                    atmospheric_pressure_pa: 82_000.0,
                    horizontal_infrared_radiation_wh_per_m2: 0.0,
                    global_horizontal_radiation_wh_per_m2: 0.0,
                    direct_normal_radiation_wh_per_m2,
                    diffuse_horizontal_radiation_wh_per_m2,
                    wind_direction_deg: 0.0,
                    wind_speed_m_per_s: 0.0,
                    liquid_precipitation_depth_mm: 0.0,
                });
            }
            date = next_day(date);
        }
        let roof = surface(
            100,
            "Spring Sunrise Roof",
            SurfaceType::Roof,
            [
                point(0.0, 0.0, 1.0),
                point(0.0, 1.0, 1.0),
                point(1.0, 1.0, 1.0),
                point(1.0, 0.0, 1.0),
            ],
        );

        let components = surface_incident_solar_components_hourly_average_w_per_m2(
            &roof,
            &site,
            &records,
            record_index.unwrap_or(0),
            4,
        );

        assert!((components.sky_diffuse_w_per_m2 - 42.517992377816).abs() < 1.0e-9);
    }

    #[test]
    fn anisotropic_sky_circumsolar_uses_sunlit_fraction() {
        let site = SiteLocation {
            name: NormalizedName::new("Golden"),
            latitude_deg: 39.74,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        };
        let wall = surface(
            101,
            "South Wall",
            SurfaceType::Wall,
            [
                point(0.0, 0.0, 0.0),
                point(1.0, 0.0, 0.0),
                point(1.0, 0.0, 1.0),
                point(0.0, 0.0, 1.0),
            ],
        );

        let shadowed = energyplus_anisotropic_sky_multiplier(
            &wall,
            &site,
            90.0_f64.to_radians(),
            20.0_f64.to_radians(),
            500.0,
            100.0,
            0.6,
            0.0,
        );
        let sunlit = energyplus_anisotropic_sky_multiplier(
            &wall,
            &site,
            90.0_f64.to_radians(),
            20.0_f64.to_radians(),
            500.0,
            100.0,
            0.6,
            1.0,
        );

        assert!(shadowed > 0.0);
        assert!(sunlit > shadowed);
    }

    #[test]
    fn constant_schedule_trace_repeats_hourly_value() {
        let mut model = TypedModel::default();
        model.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("AlwaysOn"),
            schedule_type_limits: None,
            hourly_value: 1.0,
        });

        let traces = simulate_constant_schedules(&model, 3);

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].schedule_name, "ALWAYSON");
        assert_eq!(traces[0].values, vec![1.0, 1.0, 1.0]);
    }

    #[test]
    fn compact_schedule_trace_uses_until_segments() {
        let mut model = TypedModel::default();
        model.compact_schedules.push(ScheduleCompact {
            id: ScheduleId(0),
            name: NormalizedName::new("Office Occupancy"),
            schedule_type_limits: None,
            segments: vec![
                ScheduleCompactSegment {
                    until_minute_of_day: 8 * 60,
                    value: 0.0,
                },
                ScheduleCompactSegment {
                    until_minute_of_day: 18 * 60,
                    value: 1.0,
                },
                ScheduleCompactSegment {
                    until_minute_of_day: 24 * 60,
                    value: 0.0,
                },
            ],
        });

        let traces = simulate_schedule_values(&model, 24);

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].values[7], 0.0);
        assert_eq!(traces[0].values[8], 1.0);
        assert_eq!(traces[0].values[17], 1.0);
        assert_eq!(traces[0].values[18], 0.0);
    }

    #[test]
    fn zone_internal_convective_gain_trace_excludes_radiant_fraction() {
        let mut model = cube_model();
        model.other_equipment[0].fraction_radiant = 0.25;

        let traces = simulate_zone_internal_convective_gains(&model, 2);

        assert_eq!(traces.len(), 1);
        assert_eq!(traces[0].zone_name, "ZONE ONE");
        assert_eq!(traces[0].values_w, vec![9.0, 9.0]);
    }

    #[test]
    fn default_time_axis_has_one_day() -> Result<(), Box<dyn std::error::Error>> {
        let axis = build_hourly_time_axis(&TypedModel::default())?;

        assert_eq!(axis.sample_count(), 24);
        assert_eq!(axis.points[0].hour, 1);
        assert_eq!(axis.points[23].hour, 24);

        Ok(())
    }

    #[test]
    fn run_period_time_axis_counts_inclusive_days() -> Result<(), Box<dyn std::error::Error>> {
        let axis = build_hourly_time_axis_for_run_period(&RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("Three Days"),
            begin_month: 1,
            begin_day_of_month: 1,
            begin_year: Some(2013),
            end_month: 1,
            end_day_of_month: 3,
            end_year: Some(2013),
            day_of_week_for_start_day: None,
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        })?;

        assert_eq!(axis.sample_count(), 72);
        assert_eq!(axis.points[0].day_of_month, 1);
        assert_eq!(axis.points[71].day_of_month, 3);
        assert_eq!(axis.points[71].hour, 24);

        Ok(())
    }

    #[test]
    fn run_period_time_axis_handles_leap_year() -> Result<(), Box<dyn std::error::Error>> {
        let axis = build_hourly_time_axis_for_run_period(&RunPeriod {
            id: RunPeriodId(0),
            name: NormalizedName::new("Leap Window"),
            begin_month: 2,
            begin_day_of_month: 28,
            begin_year: Some(2020),
            end_month: 3,
            end_day_of_month: 1,
            end_year: Some(2020),
            day_of_week_for_start_day: None,
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        })?;

        assert_eq!(axis.sample_count(), 72);
        assert_eq!(axis.points[24].day_of_month, 29);

        Ok(())
    }

    fn stage_with_kind(stages: &[ExecutionStage], kind: ExecutionStageKind) -> &ExecutionStage {
        stages
            .iter()
            .find(|stage| stage.kind == kind)
            .expect("execution stage kind should exist")
    }

    #[test]
    fn execution_plan_uses_heat_balance_source_order_stages() {
        let mut typed = TypedModel::default();
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("AlwaysOn"),
            schedule_type_limits: None,
            hourly_value: 1.0,
        });
        typed.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: ep_model::Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            ceiling_height: ep_model::AutoOrNumber::AutoCalculate,
            volume: ep_model::AutoOrNumber::AutoCalculate,
        });
        let model = SimulationModel::from_typed(typed);

        let plan = build_execution_plan(&model);

        assert_eq!(plan.stages.len(), 18);
        assert_eq!(plan.step_count(), 16);
        assert!(
            plan.stages
                .iter()
                .all(|stage| stage.kind.is_source_order_barrier())
        );
        assert_eq!(
            plan.stages
                .iter()
                .map(|stage| stage.kind)
                .collect::<Vec<_>>(),
            plan.compatibility_stages
                .iter()
                .map(|stage| stage.kind)
                .collect::<Vec<_>>()
        );
        assert!(plan.source_order_stages_match());
        assert_eq!(
            plan.expected_source_order_stage_ids(),
            plan.actual_source_order_stage_ids()
        );

        let init_heat_balance = stage_with_kind(&plan.stages, ExecutionStageKind::InitHeatBalance);
        assert_eq!(init_heat_balance.steps[0], ExecutionStep::UpdateWeather);
        assert_eq!(
            init_heat_balance.steps[1],
            ExecutionStep::EvaluateSchedule(ScheduleId(0))
        );

        let manage_zone_air_updates =
            stage_with_kind(&plan.stages, ExecutionStageKind::ManageZoneAirUpdates);
        assert_eq!(
            manage_zone_air_updates.steps[0],
            ExecutionStep::SolveZone(ZoneId(0))
        );

        let report_heat_balance =
            stage_with_kind(&plan.stages, ExecutionStageKind::ReportHeatBalance);
        assert_eq!(report_heat_balance.steps.len(), 13);
        assert_eq!(
            report_heat_balance.steps[0],
            ExecutionStep::WriteOutput(OutputHandle(0))
        );
        assert_eq!(
            report_heat_balance.steps[1],
            ExecutionStep::WriteOutput(OutputHandle(1))
        );
        assert_eq!(
            report_heat_balance.steps[2],
            ExecutionStep::WriteOutput(OutputHandle(2))
        );
        assert_eq!(
            report_heat_balance.steps[10],
            ExecutionStep::WriteOutput(OutputHandle(10))
        );
        assert_eq!(
            plan.compatibility_stages,
            energyplus_heat_balance_compatibility_stages()
        );
    }

    #[test]
    fn heat_balance_compatibility_stages_follow_energyplus_source_order() {
        let stages = energyplus_heat_balance_compatibility_stages();

        assert_eq!(stages.len(), 18);
        assert!(
            stages
                .iter()
                .all(|stage| stage.kind.is_source_order_barrier())
        );
        assert_eq!(stages[0].kind, ExecutionStageKind::GetHeatBalanceInput);
        assert_eq!(stages[0].stage_name, "get-heat-balance-input");
        assert_eq!(stages[0].source_routine, "GetHeatBalanceInput");
        assert_eq!(stages[4].kind, ExecutionStageKind::ManageSurfaceHeatBalance);
        assert_eq!(stages[4].source_routine, "ManageSurfaceHeatBalance");
        assert_eq!(stages[5].source_routine, "InitSurfaceHeatBalance");
        assert_eq!(stages[6].source_routine, "CalcHeatBalanceOutsideSurf");
        assert_eq!(stages[7].source_routine, "CalcHeatBalanceInsideSurf");
        assert_eq!(stages[8].source_routine, "ManageAirHeatBalance");
        assert_eq!(stages[9].source_routine, "ManageZoneAirUpdates");
        assert_eq!(stages[11].source_routine, "UpdateThermalHistories");
        assert_eq!(stages[12].source_routine, "ReportSurfaceHeatBalance");
        assert_eq!(stages[15].source_routine, "ReportHeatBalance");
        assert_eq!(stages[17].source_routine, "CheckWarmupConvergence");
    }

    #[test]
    fn heat_balance_zone_air_algorithm_lanes_separate_compatibility_and_diagnostics() {
        let candidate = HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate;
        assert_eq!(
            candidate.lane(),
            HeatBalanceAlgorithmLane::CompatibilitySourceOrder
        );
        assert_eq!(candidate.lane().id(), "compatibility-source-order");
        assert!(candidate.is_compatibility_source_order());
        assert!(candidate.allows_conformance_promotion());

        let diagnostic_only = HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical;
        assert_eq!(
            diagnostic_only.lane(),
            HeatBalanceAlgorithmLane::DiagnosticOnly
        );
        assert_eq!(diagnostic_only.lane().id(), "diagnostic-only");
        assert!(diagnostic_only.is_diagnostic_lane());
        assert!(!diagnostic_only.allows_conformance_promotion());

        let probe = HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe;
        assert_eq!(probe.lane(), HeatBalanceAlgorithmLane::DiagnosticProbe);
        assert_eq!(probe.lane().id(), "diagnostic-probe");
        assert!(probe.is_diagnostic_lane());
        assert!(!probe.allows_conformance_promotion());
    }

    #[test]
    fn execution_plan_includes_thermostat_and_ideal_loads_steps() {
        let mut typed = TypedModel::default();
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("Control Type"),
            schedule_type_limits: None,
            hourly_value: 4.0,
        });
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(1),
            name: NormalizedName::new("Heating Setpoint"),
            schedule_type_limits: None,
            hourly_value: 21.0,
        });
        typed.schedules.push(ScheduleConstant {
            id: ScheduleId(2),
            name: NormalizedName::new("Cooling Setpoint"),
            schedule_type_limits: None,
            hourly_value: 24.0,
        });
        typed.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::AutoCalculate,
        });
        typed
            .thermostat_dual_setpoints
            .push(ThermostatDualSetpoint {
                id: ThermostatSetpointId(0),
                name: NormalizedName::new("Dual Setpoints"),
                heating_setpoint_schedule: ScheduleId(1),
                cooling_setpoint_schedule: ScheduleId(2),
            });
        typed.zone_thermostats.push(ZoneThermostat {
            id: ZoneThermostatId(0),
            name: NormalizedName::new("Zone Thermostat"),
            zone: ZoneId(0),
            control_type_schedule: ScheduleId(0),
            controls: vec![ZoneThermostatControl {
                object_type: ThermostatControlObjectType::DualSetpoint,
                dual_setpoint: ThermostatSetpointId(0),
            }],
            temperature_difference_between_cutout_and_setpoint_delta_c: 0.0,
        });
        typed.ideal_loads_air_systems.push(IdealLoadsAirSystem {
            id: IdealLoadsAirSystemId(0),
            name: NormalizedName::new("Zone Ideal Loads"),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new("Zone Inlet"),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: None,
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        });
        typed.zone_equipment_lists.push(ZoneEquipmentList {
            id: ZoneEquipmentListId(0),
            name: NormalizedName::new("Zone Equipment"),
            load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
            equipment: vec![ZoneEquipmentListEntry {
                object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
                ideal_loads_air_system: IdealLoadsAirSystemId(0),
                cooling_sequence: 1,
                heating_or_no_load_sequence: 1,
                sequential_cooling_fraction_schedule: None,
                sequential_heating_fraction_schedule: None,
            }],
        });
        typed
            .zone_equipment_connections
            .push(ZoneEquipmentConnection {
                id: ZoneEquipmentConnectionId(0),
                zone: ZoneId(0),
                equipment_list: ZoneEquipmentListId(0),
                zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new("Zone Inlet")),
                zone_air_exhaust_node_or_nodelist_name: None,
                zone_air_node_name: NormalizedName::new("Zone Air Node"),
                zone_return_air_node_or_nodelist_name: Some(NormalizedName::new("Zone Return")),
                zone_return_air_node_1_flow_rate_fraction_schedule: None,
                zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
            });
        let model = SimulationModel::from_typed(typed);

        let plan = build_execution_plan(&model);

        assert_eq!(model.graph.zone_thermostats.len(), 1);
        assert_eq!(model.graph.zone_ideal_loads.len(), 1);
        assert_eq!(plan.stages.len(), 25);
        assert_eq!(plan.compatibility_stages.len(), 25);
        assert!(plan.source_order_stages_match());
        assert_eq!(
            plan.expected_source_order_stage_ids(),
            plan.actual_source_order_stage_ids()
        );
        assert!(
            plan.expected_source_order_stage_ids()
                .contains(&"sim-purchased-air")
        );
        assert!(
            plan.expected_source_order_stage_ids()
                .contains(&"get-purchased-air")
        );
        assert!(
            plan.expected_source_order_stage_ids()
                .contains(&"calc-purch-air-loads")
        );

        let manage_zone_air_updates =
            stage_with_kind(&plan.stages, ExecutionStageKind::ManageZoneAirUpdates);
        assert_eq!(manage_zone_air_updates.steps.len(), 2);
        assert_eq!(
            manage_zone_air_updates.steps[0],
            ExecutionStep::EvaluateZoneThermostat(ZoneThermostatId(0))
        );
        assert_eq!(
            manage_zone_air_updates.steps[1],
            ExecutionStep::SolveZone(ZoneId(0))
        );

        let zone_equipment =
            stage_with_kind(&plan.stages, ExecutionStageKind::ZoneEquipmentManager);
        assert_eq!(zone_equipment.name, "zone-equipment-manager");
        assert_eq!(zone_equipment.steps.len(), 2);
        assert_eq!(
            zone_equipment.steps[0],
            ExecutionStep::ManageZoneEquipment(ZoneId(0))
        );
        assert_eq!(
            zone_equipment.steps[1],
            ExecutionStep::SimZoneEquipment(ZoneEquipmentListId(0))
        );

        let purchased_air_sim =
            stage_with_kind(&plan.stages, ExecutionStageKind::PurchasedAirManagerSim);
        assert_eq!(
            purchased_air_sim.steps[0],
            ExecutionStep::SimPurchasedAir(IdealLoadsAirSystemId(0))
        );

        let purchased_air_get =
            stage_with_kind(&plan.stages, ExecutionStageKind::PurchasedAirManagerGet);
        assert_eq!(
            purchased_air_get.steps[0],
            ExecutionStep::GetIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );

        let purchased_air_init =
            stage_with_kind(&plan.stages, ExecutionStageKind::PurchasedAirManagerInit);
        assert_eq!(
            purchased_air_init.steps[0],
            ExecutionStep::InitIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );

        let purchased_air_calc =
            stage_with_kind(&plan.stages, ExecutionStageKind::PurchasedAirManagerCalc);
        assert_eq!(
            purchased_air_calc.steps[0],
            ExecutionStep::EvaluateIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );

        let purchased_air_update =
            stage_with_kind(&plan.stages, ExecutionStageKind::PurchasedAirManagerUpdate);
        assert_eq!(
            purchased_air_update.steps[0],
            ExecutionStep::UpdateIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );

        let purchased_air_report =
            stage_with_kind(&plan.stages, ExecutionStageKind::PurchasedAirManagerReport);
        assert_eq!(
            purchased_air_report.steps[0],
            ExecutionStep::ReportIdealLoadsAirSystem(IdealLoadsAirSystemId(0))
        );
    }

    #[test]
    fn ideal_loads_node_state_projection_expands_nodelist_and_writes_series()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = ideal_loads_node_state_model();

        let projection = simulate_ideal_loads_node_state_projection(
            &model,
            NodeStateProjectionOptions::hourly_samples(4),
        )?;

        assert_eq!(projection.summary.samples, 4);
        assert_eq!(projection.summary.node_count, 3);
        assert_eq!(projection.summary.series_count, 9);
        assert_eq!(projection.summary.state_node_count, 3);
        assert_eq!(
            projection.summary.evidence_policy.source_map_path,
            NODE_STATE_SOURCE_MAP_PATH
        );
        assert_eq!(
            projection.summary.evidence_policy.excluded_variable,
            NODE_STATE_EXCLUDED_SETPOINT_VARIABLE
        );
        assert_eq!(
            node_temperature_setpoint_from_energyplus(NODE_TEMPERATURE_SETPOINT_SENTINEL_C),
            None
        );
        assert_eq!(node_temperature_setpoint_from_energyplus(21.0), Some(21.0));
        assert_eq!(projection.state.len(), 3);
        assert_eq!(
            projection
                .summary
                .nodes
                .iter()
                .map(|node| (node.node_name.as_str(), node.role))
                .collect::<Vec<_>>(),
            vec![
                ("ZONE ONE INLET", NodeStateRole::Supply),
                ("ZONE ONE AIR NODE", NodeStateRole::ZoneAir),
                ("ZONE ONE RETURN", NodeStateRole::ReturnAir),
            ]
        );

        let inlet_temperature = projection
            .results
            .find_series("ZONE ONE INLET", "System Node Temperature")
            .ok_or_else(|| std::io::Error::other("missing inlet temperature series"))?;
        assert_eq!(inlet_temperature.values, vec![50.0; 4]);

        let inlet_humidity = projection
            .results
            .find_series("ZONE ONE INLET", "System Node Humidity Ratio")
            .ok_or_else(|| std::io::Error::other("missing inlet humidity series"))?;
        assert_eq!(inlet_humidity.values, vec![0.0156; 4]);

        let inlet_mass_flow = projection
            .results
            .find_series("ZONE ONE INLET", "System Node Mass Flow Rate")
            .ok_or_else(|| std::io::Error::other("missing inlet mass flow series"))?;
        assert!(
            inlet_mass_flow
                .values
                .iter()
                .all(|value| (*value - 0.3).abs() < 1.0e-12)
        );
        let inlet_state = projection
            .state
            .find_by_key("ZONE ONE INLET")
            .ok_or_else(|| std::io::Error::other("missing inlet node state"))?;
        assert!((inlet_state.mass_flow_rate_kg_per_s - 0.3).abs() < 1.0e-12);
        assert!((inlet_state.temperature_c - 50.0).abs() < 1.0e-12);
        assert_eq!(inlet_state.temperature_setpoint_c, None);

        let zone_air_temperature = projection
            .results
            .find_series("ZONE ONE AIR NODE", "System Node Temperature")
            .ok_or_else(|| std::io::Error::other("missing zone air temperature series"))?;
        assert_eq!(zone_air_temperature.values, vec![23.0; 4]);
        let zone_air_state = projection
            .state
            .find_by_key("ZONE ONE AIR NODE")
            .ok_or_else(|| std::io::Error::other("missing zone air node state"))?;
        assert!((zone_air_state.humidity_ratio - 0.008).abs() < 1.0e-12);

        let return_mass_flow = projection
            .results
            .find_series("ZONE ONE RETURN", "System Node Mass Flow Rate")
            .ok_or_else(|| std::io::Error::other("missing return mass flow series"))?;
        assert!(
            return_mass_flow
                .values
                .iter()
                .all(|value| (*value - 0.3).abs() < 1.0e-12)
        );

        Ok(())
    }

    #[test]
    fn ideal_loads_node_state_projection_resolves_supply_zone_and_return_node_ids()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = ideal_loads_node_state_model();

        let projection = simulate_ideal_loads_node_state_projection(
            &model,
            NodeStateProjectionOptions::hourly_samples(1),
        )?;

        assert_eq!(
            projection
                .summary
                .nodes
                .iter()
                .map(|node| (node.node_id, node.node_name.as_str(), node.role))
                .collect::<Vec<_>>(),
            vec![
                (NodeId(0), "ZONE ONE INLET", NodeStateRole::Supply),
                (NodeId(1), "ZONE ONE AIR NODE", NodeStateRole::ZoneAir),
                (NodeId(2), "ZONE ONE RETURN", NodeStateRole::ReturnAir),
            ]
        );
        assert_eq!(
            projection
                .state
                .find_by_key("zone one inlet")
                .unwrap()
                .node_id,
            NodeId(0)
        );
        assert_eq!(
            projection
                .state
                .find_by_key("Zone One Air Node")
                .unwrap()
                .node_id,
            NodeId(1)
        );
        assert_eq!(
            projection
                .state
                .find_by_id(NodeId(2))
                .unwrap()
                .node_name
                .as_str(),
            "ZONE ONE RETURN"
        );

        Ok(())
    }

    #[test]
    fn node_state_store_initializes_without_ideal_loads_result_structs() {
        let mut typed = TypedModel::default();
        let node_id = push_node(&mut typed, "Standalone Air Node");

        let state = NodeStateStore::from_typed_model(&typed, 21.5, 0.0085);

        assert!(typed.ideal_loads_air_systems.is_empty());
        assert_eq!(state.len(), 1);
        let node = state.find_by_key("standalone air node").unwrap();
        assert_eq!(node.node_id, node_id);
        assert_eq!(node.temperature_c, 21.5);
        assert_eq!(node.humidity_ratio, 0.0085);
        assert_eq!(node.mass_flow_rate_kg_per_s, 0.0);
    }

    fn ideal_loads_node_state_model() -> SimulationModel {
        let mut typed = TypedModel::default();
        typed.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::AutoCalculate,
        });
        typed.nodes.push(Node {
            id: NodeId(0),
            name: NormalizedName::new("Zone One Inlet"),
        });
        typed.nodes.push(Node {
            id: NodeId(1),
            name: NormalizedName::new("Zone One Air Node"),
        });
        typed.nodes.push(Node {
            id: NodeId(2),
            name: NormalizedName::new("Zone One Return"),
        });
        typed.node_names.insert("Zone One Inlet", NodeId(0));
        typed.node_names.insert("Zone One Air Node", NodeId(1));
        typed.node_names.insert("Zone One Return", NodeId(2));
        typed.node_lists.push(NodeList {
            id: NodeListId(0),
            name: NormalizedName::new("Zone One Inlets"),
            nodes: vec![NodeId(0)],
        });
        typed
            .node_list_names
            .insert("Zone One Inlets", NodeListId(0));
        typed.ideal_loads_air_systems.push(IdealLoadsAirSystem {
            id: IdealLoadsAirSystemId(0),
            name: NormalizedName::new("Zone One Ideal Loads"),
            availability_schedule: None,
            zone_supply_air_node_name: NormalizedName::new("Zone One Inlets"),
            zone_exhaust_air_node_name: None,
            system_inlet_air_node_name: None,
            maximum_heating_supply_air_temperature_c: 50.0,
            minimum_cooling_supply_air_temperature_c: 13.0,
            maximum_heating_supply_air_humidity_ratio: 0.0156,
            minimum_cooling_supply_air_humidity_ratio: 0.0077,
            heating_limit: IdealLoadsLimit::NoLimit,
            maximum_heating_air_flow_rate_m3_per_s: Some(AutosizeOrNumber::Value(0.25)),
            maximum_sensible_heating_capacity_w: None,
            cooling_limit: IdealLoadsLimit::NoLimit,
            maximum_cooling_air_flow_rate_m3_per_s: None,
            maximum_total_cooling_capacity_w: None,
            heating_availability_schedule: None,
            cooling_availability_schedule: None,
            dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
            cooling_sensible_heat_ratio: 0.7,
            humidification_control_type: HumidificationControlType::None,
            design_specification_outdoor_air_object_name: None,
            outdoor_air_inlet_node_name: None,
            demand_controlled_ventilation_type: DemandControlledVentilationType::None,
            outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
            heat_recovery_type: HeatRecoveryType::None,
            sensible_heat_recovery_effectiveness: 0.7,
            latent_heat_recovery_effectiveness: 0.65,
            design_specification_zonehvac_sizing_object_name: None,
            heating_fuel_efficiency_schedule: None,
            heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
            cooling_fuel_efficiency_schedule: None,
            cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
        });
        typed.zone_equipment_lists.push(ZoneEquipmentList {
            id: ZoneEquipmentListId(0),
            name: NormalizedName::new("Zone One Equipment"),
            load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
            equipment: vec![ZoneEquipmentListEntry {
                object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
                ideal_loads_air_system: IdealLoadsAirSystemId(0),
                cooling_sequence: 1,
                heating_or_no_load_sequence: 1,
                sequential_cooling_fraction_schedule: None,
                sequential_heating_fraction_schedule: None,
            }],
        });
        typed
            .zone_equipment_connections
            .push(ZoneEquipmentConnection {
                id: ZoneEquipmentConnectionId(0),
                zone: ZoneId(0),
                equipment_list: ZoneEquipmentListId(0),
                zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new("Zone One Inlets")),
                zone_air_exhaust_node_or_nodelist_name: None,
                zone_air_node_name: NormalizedName::new("Zone One Air Node"),
                zone_return_air_node_or_nodelist_name: Some(NormalizedName::new("Zone One Return")),
                zone_return_air_node_1_flow_rate_fraction_schedule: None,
                zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
            });

        SimulationModel::from_typed(typed)
    }

    fn push_node(model: &mut TypedModel, name: &str) -> NodeId {
        let id = NodeId(model.nodes.len() as u32);
        model.nodes.push(Node {
            id,
            name: NormalizedName::new(name),
        });
        model.node_names.insert(name, id);
        id
    }

    #[test]
    fn parses_epw_records_after_header() -> Result<(), Box<dyn std::error::Error>> {
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
1999,1,1,1,0,Source,-3.0,-4.0,50,82000,0,0,300,10,20,30,0,0,0,0,180,2.5
1999,1,1,2,0,Source,-2.0,-3.0,51,82100,0,0,301,11,21,31,0,0,0,0,190,2.6,0,0,0,0,0,0,0,0,0,0,0,2.0,1.0
"#,
        )?;

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].dry_bulb_c, -3.0);
        assert_eq!(records[0].dew_point_c, -4.0);
        assert_eq!(records[0].relative_humidity_percent, 50.0);
        assert_eq!(records[0].atmospheric_pressure_pa, 82_000.0);
        assert_eq!(records[0].wind_direction_deg, 180.0);
        assert_eq!(records[0].wind_speed_m_per_s, 2.5);
        assert_eq!(records[0].liquid_precipitation_depth_mm, 0.0);
        assert_eq!(records[1].liquid_precipitation_depth_mm, 2.0);

        Ok(())
    }

    #[test]
    fn parses_epw_dry_bulb_values_after_header() -> Result<(), Box<dyn std::error::Error>> {
        let values = parse_epw_dry_bulb_series(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
1999,1,1,1,0,Source,-3.0,-4.0,50,82000,0,0,300,10,20,30,0,0,0,0,180,2.5
1999,1,1,2,0,Source,-2.0,-3.0,51,82100,0,0,301,11,21,31,0,0,0,0,190,2.6
"#,
        )?;

        assert_eq!(values, vec![-3.0, -2.0]);

        Ok(())
    }

    #[test]
    fn surface_area_handles_3d_rectangles() {
        let vertices = vec![
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 2.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 2.0,
                y_m: 0.0,
                z_m: 3.0,
            },
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 3.0,
            },
        ];

        assert_eq!(surface_area_m2(&vertices), 6.0);
    }

    #[test]
    fn zone_geometry_summary_reports_cube_metrics() {
        let summaries = zone_geometry_summaries(&cube_model());

        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].zone_name, "ZONE ONE");
        assert_eq!(summaries[0].surface_count, 6);
        assert_eq!(summaries[0].floor_area_m2, 1.0);
        assert_eq!(summaries[0].volume_m3, Some(1.0));
        assert_eq!(summaries[0].exterior_wall_area_m2, 4.0);
    }

    #[test]
    fn surface_geometry_summary_reports_cube_orientation() -> Result<(), Box<dyn std::error::Error>>
    {
        let summaries = surface_geometry_summaries(&cube_model());

        assert_eq!(summaries.len(), 6);
        let floor = summaries
            .iter()
            .find(|surface| surface.surface_name == "FLOOR")
            .ok_or_else(|| std::io::Error::other("missing floor surface"))?;
        assert_eq!(floor.zone_name, "ZONE ONE");
        assert_eq!(floor.surface_type, SurfaceType::Floor);
        assert_eq!(floor.area_m2, 1.0);
        assert!((floor.azimuth_deg - 270.0).abs() < 1.0e-9);
        assert!((floor.tilt_deg - 180.0).abs() < 1.0e-9);

        let roof = summaries
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof surface"))?;
        assert_eq!(roof.surface_type, SurfaceType::Roof);
        assert_eq!(roof.area_m2, 1.0);
        assert!((roof.azimuth_deg - 0.0).abs() < 1.0e-9);
        assert!((roof.tilt_deg - 0.0).abs() < 1.0e-9);

        let wall_azimuths = [
            ("WALL X0", 90.0),
            ("WALL X1", 270.0),
            ("WALL Y0", 0.0),
            ("WALL Y1", 180.0),
        ];
        for (surface_name, azimuth_deg) in wall_azimuths {
            let wall = summaries
                .iter()
                .find(|surface| surface.surface_name == surface_name)
                .ok_or_else(|| std::io::Error::other(format!("missing {surface_name} surface")))?;
            assert_eq!(wall.surface_type, SurfaceType::Wall);
            assert_eq!(wall.area_m2, 1.0);
            assert!((wall.azimuth_deg - azimuth_deg).abs() < 1.0e-9);
            assert!((wall.tilt_deg - 90.0).abs() < 1.0e-9);
        }

        Ok(())
    }

    #[test]
    fn single_system_timestep_syncs_adaptive_history() -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone = &mut state.zones[0];
        zone.mean_air_temperature_c = 20.1;
        zone.air_humidity_ratio = 0.004;
        zone.previous_mean_air_temperatures_c = [20.0, 19.0, 18.0];
        zone.previous_air_humidity_ratios = [0.003, 0.002, 0.001];
        zone.previous_system_mean_air_temperatures_c = [9.0, 8.0, 7.0];
        zone.previous_system_air_humidity_ratios = [0.009, 0.008, 0.007];
        zone.previous_system_timestep_count = 4;

        apply_energyplus_adaptive_system_timestep_zone_air_correction(
            &state.surfaces,
            &mut state.zones,
            900.0,
            None,
            20.0,
            false,
        );

        let zone = &state.zones[0];
        assert_eq!(zone.previous_system_timestep_count, 1);
        assert_eq!(
            zone.previous_system_mean_air_temperatures_c,
            [zone.mean_air_temperature_c, 20.0, 19.0]
        );
        assert_eq!(
            zone.previous_system_air_humidity_ratios,
            [zone.air_humidity_ratio, 0.003, 0.002]
        );
        assert_eq!(
            zone.zone_timestep_average_air_temperature_c,
            zone.mean_air_temperature_c
        );
        assert_eq!(
            zone.zone_timestep_average_air_humidity_ratio,
            zone.air_humidity_ratio
        );

        Ok(())
    }
    #[test]
    fn heat_balance_state_shell_initializes_cube_metrics() -> Result<(), Box<dyn std::error::Error>>
    {
        let model = SimulationModel::from_typed(cube_model());

        let state = initialize_heat_balance_state(&model, 20.0)?;

        assert_eq!(state.timestep_index, 0);
        assert_eq!(state.zones.len(), 1);
        assert_eq!(state.zones[0].zone_name, "ZONE ONE");
        assert_eq!(state.zones[0].mean_air_temperature_c, 20.0);
        assert_eq!(state.zones[0].zone_timestep_average_air_temperature_c, 20.0);
        assert_eq!(state.zones[0].previous_mean_air_temperatures_c, [20.0; 3]);
        assert_eq!(
            state.zones[0].previous_system_mean_air_temperatures_c,
            [20.0; 3]
        );
        assert_eq!(state.zones[0].previous_system_timestep_count, 1);
        assert_eq!(state.zones[0].volume_m3, 1.0);
        assert!((state.zones[0].air_heat_capacity_j_per_k - 1207.2).abs() < 1.0e-9);
        assert_eq!(state.zones[0].convective_internal_gain_w, 12.0);
        assert_eq!(state.zones[0].opaque_surface_conductance_w_per_k, 6.0);
        assert_eq!(state.zones[0].opaque_surface_heat_gain_w, 0.0);
        assert!((state.zones[0].sum_ha_w_per_k - 18.456).abs() < 1.0e-12);
        assert!((state.zones[0].sum_hat_surf_w - 369.12).abs() < 1.0e-12);
        assert_eq!(state.zones[0].sum_hat_ref_w, 0.0);
        assert!(
            (state.zones[0]
                .zone_air_temperature_coefficients
                .temp_dependent_coefficient_w_per_k
                - 18.456)
                .abs()
                < 1.0e-12
        );
        assert!(
            (state.zones[0]
                .zone_air_temperature_coefficients
                .temp_independent_coefficient_w
                - 381.12)
                .abs()
                < 1.0e-12
        );
        assert_eq!(
            state.zones[0]
                .zone_air_temperature_coefficients
                .air_power_cap_w_per_k,
            0.0
        );
        assert_eq!(state.surfaces.len(), 6);
        let floor = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "FLOOR")
            .ok_or_else(|| std::io::Error::other("missing floor surface"))?;
        assert!((floor.tilt_deg - 180.0).abs() < 1.0e-9);
        let roof = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof surface"))?;
        assert!((roof.tilt_deg - 0.0).abs() < 1.0e-9);
        let wall = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "WALL Y0")
            .ok_or_else(|| std::io::Error::other("missing wall surface"))?;
        assert!((wall.tilt_deg - 90.0).abs() < 1.0e-9);
        assert_eq!(
            state.surfaces[0].outside_boundary_condition,
            OutsideBoundaryCondition::Outdoors
        );
        assert_eq!(state.surfaces[0].construction_name, "WALL");
        assert_eq!(state.surfaces[0].outside_layer_material_name, "R1");
        assert_eq!(
            state.surfaces[0].outside_layer_roughness,
            MaterialSurfaceRoughness::Rough
        );
        assert_eq!(state.surfaces[0].area_m2, 1.0);
        assert_eq!(state.surfaces[0].thermal_resistance_m2_k_per_w, 1.0);
        assert_eq!(state.surfaces[0].heat_capacity_j_per_m2_k, None);
        assert_eq!(state.surfaces[0].thermal_absorptance, 0.9);
        assert_eq!(state.surfaces[0].inside_thermal_absorptance, 0.9);
        assert_eq!(state.surfaces[0].conductance_w_per_k, 1.0);
        assert_eq!(
            state.surfaces[0].inside_convection_coefficient_w_per_m2_k,
            3.076
        );
        assert_eq!(state.surfaces[0].ctf.outside_0_w_per_m2_k, 1.0);
        assert_eq!(state.surfaces[0].ctf.cross_0_w_per_m2_k, 1.0);
        assert_eq!(state.surfaces[0].ctf.inside_0_w_per_m2_k, 1.0);
        assert_eq!(state.surfaces[0].ctf.const_in_part_w_per_m2, 0.0);
        assert_eq!(state.surfaces[0].ctf.const_out_part_w_per_m2, 0.0);
        assert_eq!(
            state.surfaces[0].ctf.outside_temperature_history_c,
            vec![20.0]
        );
        assert_eq!(state.surfaces[0].heat_gain_to_zone_w, 0.0);
        assert_eq!(state.surfaces[0].inside_face_temperature_c, 20.0);
        assert_eq!(state.surfaces[0].outside_face_temperature_c, 20.0);

        Ok(())
    }

    #[test]
    fn heat_balance_state_uses_inside_layer_absorptance_for_interior_sources()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.materials.push(Material {
            id: MaterialId(1),
            name: NormalizedName::new("Inside Low Absorptance"),
            kind: MaterialKind::NoMass,
            roughness: Some(MaterialSurfaceRoughness::Smooth),
            conductivity_w_per_m_k: None,
            density_kg_per_m3: None,
            specific_heat_j_per_kg_k: None,
            thickness_m: None,
            thermal_resistance_m2_k_per_w: Some(1.0),
            thermal_absorptance: Some(0.2),
            solar_absorptance: Some(0.2),
            visible_absorptance: Some(0.2),
        });
        typed.materials.push(Material {
            id: MaterialId(2),
            name: NormalizedName::new("Inside High Absorptance"),
            kind: MaterialKind::NoMass,
            roughness: Some(MaterialSurfaceRoughness::Smooth),
            conductivity_w_per_m_k: None,
            density_kg_per_m3: None,
            specific_heat_j_per_kg_k: None,
            thickness_m: None,
            thermal_resistance_m2_k_per_w: Some(1.0),
            thermal_absorptance: Some(0.8),
            solar_absorptance: Some(0.8),
            visible_absorptance: Some(0.8),
        });
        typed.constructions[0].layers = vec![MaterialId(0), MaterialId(1)];
        typed.constructions.push(Construction {
            id: ConstructionId(1),
            name: NormalizedName::new("High Inside Wall"),
            outside_layer: MaterialId(0),
            layers: vec![MaterialId(0), MaterialId(2)],
        });
        typed.surfaces[0].construction = ConstructionId(1);
        typed.other_equipment[0].fraction_radiant = 0.25;
        let model = SimulationModel::from_typed(typed);
        let state = initialize_heat_balance_state(&model, 20.0)?;

        let high_inside = &state.surfaces[0];
        assert_eq!(high_inside.thermal_absorptance, 0.9);
        assert_eq!(high_inside.inside_thermal_absorptance, 0.8);
        let low_inside = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_id != high_inside.surface_id)
            .ok_or_else(|| std::io::Error::other("missing low-inside surface"))?;
        assert_eq!(low_inside.thermal_absorptance, 0.9);
        assert_eq!(low_inside.inside_thermal_absorptance, 0.2);

        let denominator = 0.8 + 5.0 * 0.2;
        let multiplier = 3.0 / denominator;
        assert!(
            (high_inside.inside_radiant_internal_gain_w_per_m2 - multiplier * 0.8).abs() < 1.0e-12
        );
        assert!(
            (low_inside.inside_radiant_internal_gain_w_per_m2 - multiplier * 0.2).abs() < 1.0e-12
        );

        Ok(())
    }

    #[test]
    fn energyplus_zone_air_temperature_coefficients_match_predictor_terms() {
        let coefficients = energyplus_zone_air_temperature_coefficients(
            18.456,
            369.12,
            2.0,
            12.0,
            3.0,
            45.0,
            1207.2,
            600.0,
            [20.0, 19.0, 18.0],
        );

        assert!((coefficients.temp_dependent_coefficient_w_per_k - 21.456).abs() < 1.0e-12);
        assert!((coefficients.temp_independent_coefficient_w - 424.12).abs() < 1.0e-12);
        assert!((coefficients.air_power_cap_w_per_k - 2.012).abs() < 1.0e-12);
        let expected_history = 2.012 * (3.0 * 20.0 - 1.5 * 19.0 + (1.0 / 3.0) * 18.0);
        assert!((coefficients.third_order_history_term_w - expected_history).abs() < 1.0e-12);
        assert!(
            (coefficients.third_order_temp_dependent_load_w_per_k
                - ((11.0 / 6.0) * 2.012 + 21.456))
                .abs()
                < 1.0e-12
        );
        assert!(
            (coefficients.third_order_temp_independent_load_w - (expected_history + 424.12)).abs()
                < 1.0e-12
        );
    }

    #[test]
    fn energyplus_third_order_zone_air_temperature_matches_predictor_branch() {
        let temperature = energyplus_third_order_zone_air_temperature_c(
            20.0,
            424.12,
            21.456,
            1207.2,
            600.0,
            [20.0, 19.0, 18.0],
        );
        let air_power_cap = 1207.2 / 600.0;
        let history_term = air_power_cap * (3.0 * 20.0 - 1.5 * 19.0 + (1.0 / 3.0) * 18.0);
        let expected = (424.12 + history_term) / ((11.0 / 6.0) * air_power_cap + 21.456);
        assert!((temperature - expected).abs() < 1.0e-12);

        let fallback =
            energyplus_third_order_zone_air_temperature_c(20.0, 1.0, 0.0, 0.0, 600.0, [20.0; 3]);
        assert_eq!(fallback, 20.0);
    }

    #[test]
    fn energyplus_analytical_zone_air_temperature_matches_predictor_branch() {
        let zero_dependency =
            energyplus_analytical_zone_air_temperature_c(20.0, 12.0, 0.0, 1207.2, 600.0);
        assert!((zero_dependency - (20.0 + 12.0 * 600.0 / 1207.2)).abs() < 1.0e-12);

        let temperature =
            energyplus_analytical_zone_air_temperature_c(20.0, 72.0, 6.0, 1207.2, 600.0);
        let expected = 12.0 + (20.0 - 12.0) * (-6.0 * 600.0 / 1207.2_f64).exp();
        assert!((temperature - expected).abs() < 1.0e-12);
    }

    #[test]
    fn energyplus_tarp_natural_convection_matches_ashrae_branches() {
        let vertical = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(28.0, 20.0, 0.0);
        assert!((vertical - 2.62).abs() < 1.0e-12);

        let unstable_delta = 2.0_f64.powf(1.0 / 3.0);
        let unstable = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(22.0, 20.0, 1.0);
        let expected_unstable = 9.482 * unstable_delta / (7.238 - 1.0);
        assert!((unstable - expected_unstable).abs() < 1.0e-12);

        let stable = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(22.0, 20.0, -1.0);
        let expected_stable = 1.810 * unstable_delta / (1.382 + 1.0);
        assert!((stable - expected_stable).abs() < 1.0e-12);

        let zero_delta = energyplus_ashrae_tarp_natural_convection_w_per_m2_k(20.0, 20.0, 1.0);
        assert_eq!(zero_delta, 0.0);
    }

    #[test]
    fn energyplus_tarp_inside_convection_uses_surface_orientation_and_limits()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let state = initialize_heat_balance_state(&model, 20.0)?;
        let floor = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "FLOOR")
            .ok_or_else(|| std::io::Error::other("missing floor surface"))?;
        let roof = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof surface"))?;
        let wall = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "WALL Y0")
            .ok_or_else(|| std::io::Error::other("missing wall surface"))?;

        let delta_term = 2.0_f64.powf(1.0 / 3.0);
        let floor_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(floor, 22.0, 20.0);
        let expected_floor = 9.482 * delta_term / (7.238 - 1.0);
        assert!((floor_coefficient - expected_floor).abs() < 1.0e-12);

        let roof_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(roof, 22.0, 20.0);
        let expected_roof = 1.810 * delta_term / (1.382 + 1.0);
        assert!((roof_coefficient - expected_roof).abs() < 1.0e-12);

        let wall_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(wall, 22.0, 20.0);
        let expected_wall = 1.31 * delta_term;
        assert!((wall_coefficient - expected_wall).abs() < 1.0e-12);

        let zero_delta_coefficient =
            energyplus_tarp_inside_convection_coefficient_w_per_m2_k(floor, 20.0, 20.0);
        assert_eq!(zero_delta_coefficient, 0.1);

        Ok(())
    }

    #[test]
    fn energyplus_doe2_outside_convection_uses_wind_side_and_roughness() {
        let windward = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            35.0,
            20.0,
            0.0,
            180.0,
            180.0,
            4.0,
            MaterialSurfaceRoughness::MediumRough,
        );
        let leeward = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            35.0,
            20.0,
            0.0,
            180.0,
            0.0,
            4.0,
            MaterialSurfaceRoughness::MediumRough,
        );
        let smoother = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            35.0,
            20.0,
            0.0,
            180.0,
            180.0,
            4.0,
            MaterialSurfaceRoughness::VerySmooth,
        );

        assert!((windward - 16.031846262998357).abs() < 1.0e-12);
        assert!((leeward - 11.929263692153699).abs() < 1.0e-12);
        assert!(windward > leeward);
        assert!(smoother < windward);
    }

    #[test]
    fn energyplus_surface_wind_speed_uses_terrain_and_centroid_height() {
        let typed = cube_model();
        let roof = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .expect("roof test surface");
        let expected_weather_mod = (270.0_f64 / 10.0).powf(0.14);
        let roof_height_m =
            roof.vertices.iter().map(|vertex| vertex.z_m).sum::<f64>() / roof.vertices.len() as f64;
        let expected_roof_wind = 4.0 * expected_weather_mod * (roof_height_m / 370.0).powf(0.22);

        let expected_roof_air_temperature = 20.0
            - 0.0065
                * (roof_height_m - ENERGYPLUS_DEFAULT_WEATHER_FILE_TEMPERATURE_SENSOR_HEIGHT_M);
        assert!(
            (energyplus_surface_outdoor_air_temperature_c(roof, 20.0)
                - expected_roof_air_temperature)
                .abs()
                < 1.0e-12
        );

        assert!(
            (energyplus_surface_outside_wind_speed_m_per_s(roof, Terrain::Suburbs, 4.0)
                - expected_roof_wind)
                .abs()
                < 1.0e-12
        );

        let mut no_wind_roof = roof.clone();
        no_wind_roof.wind_exposure = WindExposure::NoWind;
        assert_eq!(
            energyplus_surface_outside_wind_speed_m_per_s(&no_wind_roof, Terrain::Suburbs, 4.0),
            0.0
        );
    }

    #[test]
    fn surface_ctf_history_terms_update_flux_constants() -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.inside_face_temperature_c = 20.0;
        surface.outside_face_temperature_c = 10.0;
        surface.ctf.cross_history_w_per_m2_k = vec![0.2];
        surface.ctf.inside_history_w_per_m2_k = vec![0.3];
        surface.ctf.outside_history_w_per_m2_k = vec![0.4];
        surface.ctf.flux_history = vec![0.5];
        surface.ctf.outside_temperature_history_c = vec![8.0];
        surface.ctf.inside_temperature_history_c = vec![18.0];
        surface.ctf.inside_flux_history_w_per_m2 = vec![1.2];
        surface.ctf.outside_flux_history_w_per_m2 = vec![-0.4];

        update_surface_ctf_history_constants(surface);

        assert!((surface.ctf.const_in_part_w_per_m2 - (-3.2)).abs() < 1.0e-12);
        assert!((surface.ctf.const_out_part_w_per_m2 - (-0.6)).abs() < 1.0e-12);

        let slot_samples = surface_ctf_history_slot_samples(surface);
        assert_eq!(slot_samples.len(), 1);
        let slot = &slot_samples[0];
        assert_eq!(slot.slot_index, 1);
        assert!(
            (slot.inside_total_term_w - surface.area_m2 * surface.ctf.const_in_part_w_per_m2).abs()
                < 1.0e-12
        );
        assert!(
            (slot.outside_total_term_w + surface.area_m2 * surface.ctf.const_out_part_w_per_m2)
                .abs()
                < 1.0e-12
        );

        let inside_flux = surface_inside_conduction_flux_w_per_m2(surface);
        let outside_flux = surface_outside_conduction_flux_w_per_m2(surface);
        advance_surface_ctf_histories(surface);

        assert_eq!(surface.ctf.outside_temperature_history_c, vec![10.0]);
        assert_eq!(surface.ctf.inside_temperature_history_c, vec![20.0]);
        assert_eq!(surface.ctf.inside_flux_history_w_per_m2, vec![inside_flux]);
        assert_eq!(
            surface.ctf.outside_flux_history_w_per_m2,
            vec![outside_flux]
        );

        Ok(())
    }

    #[test]
    fn surface_ctf_conduction_report_signs_match_energyplus_storage_convention()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.inside_face_temperature_c = 20.0;
        surface.outside_face_temperature_c = 10.0;
        surface.ctf.outside_0_w_per_m2_k = 0.7;
        surface.ctf.cross_0_w_per_m2_k = 0.2;
        surface.ctf.inside_0_w_per_m2_k = 0.5;
        surface.ctf.const_in_part_w_per_m2 = 1.0;
        surface.ctf.const_out_part_w_per_m2 = -0.3;

        let inside_flux = surface_inside_conduction_flux_w_per_m2(surface);
        let outside_ctf_flux = surface_outside_conduction_flux_w_per_m2(surface);
        let inside_rate = surface_inside_conduction_rate_w(surface);
        let outside_report_rate = surface_outside_conduction_rate_w(surface);
        let storage_rate = surface_heat_storage_rate_w(inside_rate, outside_report_rate);

        assert!((inside_rate - surface.area_m2 * inside_flux).abs() < 1.0e-12);
        assert!(
            (outside_report_rate + surface.area_m2 * outside_ctf_flux).abs() < 1.0e-12,
            "EnergyPlus flips Qout to SurfOpaqOutFaceCondFlux before reporting"
        );
        assert!((storage_rate + inside_rate + outside_report_rate).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn energyplus_down_interpolate_three_history_values_matches_source_ratios() {
        assert_eq!(
            super::energyplus_down_interpolate_three_history_values(
                3600.0,
                1800.0,
                [12.0, 9.0, 3.0]
            ),
            [12.0, 10.5, 9.0]
        );
        assert_eq!(
            super::energyplus_down_interpolate_three_history_values(
                3600.0,
                1200.0,
                [12.0, 9.0, 3.0]
            ),
            [12.0, 11.0, 10.0]
        );
        assert_eq!(
            super::energyplus_down_interpolate_three_history_values(
                3600.0,
                900.0,
                [12.0, 8.0, 2.0]
            ),
            [12.0, 11.0, 10.0]
        );
    }

    #[test]
    fn heat_balance_state_applies_construction_ctf_coefficients()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            20.0,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 2,
                    outside_w_per_m2_k: -0.4,
                    cross_w_per_m2_k: 0.2,
                    inside_w_per_m2_k: -0.3,
                    flux: Some(-0.5),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 2.0,
                    cross_w_per_m2_k: 0.5,
                    inside_w_per_m2_k: 3.0,
                    flux: None,
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: 0.4,
                    cross_w_per_m2_k: 0.1,
                    inside_w_per_m2_k: 0.3,
                    flux: Some(0.5),
                },
            ],
        )?;

        let ctf = &state.surfaces[0].ctf;
        assert_eq!(ctf.outside_0_w_per_m2_k, 2.0);
        assert_eq!(ctf.cross_0_w_per_m2_k, 0.5);
        assert_eq!(ctf.inside_0_w_per_m2_k, 3.0);
        assert_eq!(ctf.outside_history_w_per_m2_k, vec![0.4, -0.4]);
        assert_eq!(ctf.cross_history_w_per_m2_k, vec![0.1, 0.2]);
        assert_eq!(ctf.inside_history_w_per_m2_k, vec![0.3, -0.3]);
        assert_eq!(ctf.flux_history, vec![0.5, -0.5]);
        assert_eq!(ctf.outside_temperature_history_c, vec![20.0, 20.0]);
        assert_eq!(ctf.inside_temperature_history_c, vec![20.0, 20.0]);
        assert_eq!(ctf.outside_flux_history_w_per_m2, vec![0.0, 0.0]);
        assert_eq!(ctf.inside_flux_history_w_per_m2, vec![0.0, 0.0]);

        Ok(())
    }

    #[test]
    fn heat_balance_state_orders_energyplus_ctf_history_indices_for_runtime_slots()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            20.0,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 5,
                    outside_w_per_m2_k: -4.1142049e-08,
                    cross_w_per_m2_k: 1.5543709e-08,
                    inside_w_per_m2_k: -4.1142049e-08,
                    flux: Some(1.2297289e-11),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 4,
                    outside_w_per_m2_k: 0.00057884701,
                    cross_w_per_m2_k: 0.00022976293,
                    inside_w_per_m2_k: 0.00057884701,
                    flux: Some(-4.0580373e-07),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 3,
                    outside_w_per_m2_k: -0.33051123,
                    cross_w_per_m2_k: 0.091914804,
                    inside_w_per_m2_k: -0.33051123,
                    flux: Some(0.0006592243),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 2,
                    outside_w_per_m2_k: 12.566595,
                    cross_w_per_m2_k: 2.1743923,
                    inside_w_per_m2_k: 12.566595,
                    flux: Some(-0.058066613),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: -62.622544,
                    cross_w_per_m2_k: 4.7096437,
                    inside_w_per_m2_k: -62.622544,
                    flux: Some(0.60555731),
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 58.08561,
                    cross_w_per_m2_k: 0.72354869,
                    inside_w_per_m2_k: 58.08561,
                    flux: None,
                },
            ],
        )?;

        let ctf = &state.surfaces[0].ctf;
        assert_eq!(ctf.outside_0_w_per_m2_k, 58.08561);
        assert_eq!(ctf.cross_0_w_per_m2_k, 0.72354869);
        assert_eq!(ctf.inside_0_w_per_m2_k, 58.08561);
        assert_eq!(
            ctf.outside_history_w_per_m2_k,
            vec![
                -62.622544,
                12.566595,
                -0.33051123,
                0.00057884701,
                -4.1142049e-08
            ]
        );
        assert_eq!(
            ctf.cross_history_w_per_m2_k,
            vec![
                4.7096437,
                2.1743923,
                0.091914804,
                0.00022976293,
                1.5543709e-08
            ]
        );
        assert_eq!(
            ctf.inside_history_w_per_m2_k,
            vec![
                -62.622544,
                12.566595,
                -0.33051123,
                0.00057884701,
                -4.1142049e-08
            ]
        );
        assert_eq!(
            ctf.flux_history,
            vec![
                0.60555731,
                -0.058066613,
                0.0006592243,
                -4.0580373e-07,
                1.2297289e-11
            ]
        );

        Ok(())
    }

    #[test]
    fn heat_balance_summary_captures_run_period_initial_ctf_history_slots()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let simulation = simulate_heat_balance_zone_air_temperatures_internal(
            &model,
            &[5.0],
            None,
            HeatBalanceSimulationOptions::hourly_samples(1),
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 2.0,
                    cross_w_per_m2_k: 0.5,
                    inside_w_per_m2_k: 3.0,
                    flux: None,
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: 0.4,
                    cross_w_per_m2_k: 0.1,
                    inside_w_per_m2_k: 0.3,
                    flux: Some(0.5),
                },
            ],
        )?;

        let floor_initial_slots = simulation
            .summary
            .run_period_initial_ctf_history_slots
            .iter()
            .filter(|sample| sample.surface_name == "FLOOR")
            .collect::<Vec<_>>();
        assert_eq!(floor_initial_slots.len(), 1);
        assert_eq!(floor_initial_slots[0].slot_index, 1);
        assert!(floor_initial_slots[0].inside_total_term_w.is_finite());
        assert!(floor_initial_slots[0].outside_total_term_w.is_finite());

        let floor_first_sample_slots = simulation
            .summary
            .first_sample_ctf_history_slots
            .iter()
            .filter(|sample| sample.surface_name == "FLOOR")
            .collect::<Vec<_>>();
        assert_eq!(floor_first_sample_slots.len(), 1);
        assert_eq!(floor_first_sample_slots[0].slot_index, 1);
        assert!(floor_first_sample_slots[0].timestep_count > 0);

        let floor_hourly_slots = simulation
            .summary
            .hourly_ctf_history_slots
            .iter()
            .filter(|sample| sample.surface_name == "FLOOR")
            .collect::<Vec<_>>();
        assert_eq!(floor_hourly_slots.len(), 1);
        assert_eq!(floor_hourly_slots[0].sample_index, 0);
        assert_eq!(floor_hourly_slots[0].slot_index, 1);
        assert_eq!(
            floor_hourly_slots[0].inside_total_term_w,
            floor_first_sample_slots[0].inside_total_term_w
        );

        Ok(())
    }

    #[test]
    fn initial_ctf_history_seeding_uses_boundary_temperature_and_u_value()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 2.0,
                    cross_w_per_m2_k: 0.5,
                    inside_w_per_m2_k: 3.0,
                    flux: None,
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: 0.4,
                    cross_w_per_m2_k: 0.1,
                    inside_w_per_m2_k: 0.3,
                    flux: Some(0.5),
                },
            ],
        )?;

        seed_initial_surface_ctf_boundary_histories(&mut state, 5.0);

        let surface = &state.surfaces[0];
        let expected_u_value = 1.0 / surface.thermal_resistance_m2_k_per_w;
        let expected_flux = expected_u_value * (5.0 - ENERGYPLUS_ZONE_INITIAL_TEMP_C);
        assert_eq!(surface.ctf.outside_temperature_history_c, vec![5.0]);
        assert_eq!(
            surface.ctf.inside_temperature_history_c,
            vec![ENERGYPLUS_ZONE_INITIAL_TEMP_C]
        );
        assert!((surface.ctf.outside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);
        assert!((surface.ctf.inside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn ground_ctf_history_seeding_uses_energyplus_building_surface_default()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 2.0,
                    cross_w_per_m2_k: 0.5,
                    inside_w_per_m2_k: 3.0,
                    flux: None,
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: 0.4,
                    cross_w_per_m2_k: 0.1,
                    inside_w_per_m2_k: 0.3,
                    flux: Some(0.5),
                },
            ],
        )?;
        state.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Ground;

        seed_initial_surface_ctf_boundary_histories(&mut state, 5.0);

        let surface = &state.surfaces[0];
        let expected_flux = surface_steady_u_value_w_per_m2_k(surface)
            * (ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C
                - ENERGYPLUS_ZONE_INITIAL_TEMP_C);
        assert_eq!(
            surface.outside_face_temperature_c,
            ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C
        );
        assert_eq!(
            surface.ctf.outside_temperature_history_c,
            vec![ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C]
        );
        assert!((surface.ctf.outside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);
        assert!((surface.ctf.inside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn energyplus_initial_ctf_history_seeding_applies_boundary_reset_and_steady_flux()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state_with_ctf_coefficients(
            &model,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            &[
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 0,
                    outside_w_per_m2_k: 2.0,
                    cross_w_per_m2_k: 0.5,
                    inside_w_per_m2_k: 3.0,
                    flux: None,
                },
                ConstructionCtfCoefficientOverride {
                    construction_name: "Wall".to_string(),
                    time_index: 1,
                    outside_w_per_m2_k: 0.4,
                    cross_w_per_m2_k: 0.1,
                    inside_w_per_m2_k: 0.3,
                    flux: Some(0.5),
                },
            ],
        )?;
        seed_initial_surface_ctf_boundary_histories(&mut state, 5.0);

        seed_energyplus_initial_surface_ctf_histories(
            &mut state,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C,
            5.0,
        );

        let surface = &state.surfaces[0];
        let expected_flux =
            surface_steady_u_value_w_per_m2_k(surface) * (5.0 - ENERGYPLUS_ZONE_INITIAL_TEMP_C);
        assert_eq!(surface.ctf.outside_temperature_history_c, vec![5.0]);
        assert_eq!(
            surface.ctf.inside_temperature_history_c,
            vec![ENERGYPLUS_ZONE_INITIAL_TEMP_C]
        );
        assert!((surface.ctf.outside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);
        assert!((surface.ctf.inside_flux_history_w_per_m2[0] - expected_flux).abs() < 1.0e-12);
        assert_eq!(
            surface.inside_face_temperature_c,
            ENERGYPLUS_ZONE_INITIAL_TEMP_C
        );
        assert_eq!(surface.outside_face_temperature_c, 5.0);

        Ok(())
    }

    #[test]
    fn heat_balance_options_track_initial_ctf_history_policy() {
        let options = HeatBalanceSimulationOptions::hourly_samples(24)
            .with_ctf_initial_history_policy(
                HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial,
            );

        assert_eq!(
            options.ctf_initial_history_policy,
            HeatBalanceCtfInitialHistoryPolicy::EnergyPlusSurfInitial
        );
    }

    #[test]
    fn energyplus_ctf_inside_face_balance_handles_standard_and_adiabatic()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.outside_face_temperature_c = 10.0;
        surface.inside_face_temperature_c = 19.0;
        surface.ctf.inside_0_w_per_m2_k = 3.0;
        surface.ctf.cross_0_w_per_m2_k = 0.5;
        surface.ctf.const_in_part_w_per_m2 = 1.0;

        let standard = energyplus_ctf_inside_face_temperature_c(
            surface,
            CtfInsideFaceBalanceInput {
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 2.0,
                previous_inside_face_temperature_c: 18.0,
                net_inside_source_w_per_m2: 4.0,
            },
        );
        assert!((standard - 14.0).abs() < 1.0e-12);

        surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        let adiabatic = energyplus_ctf_inside_face_temperature_c(
            surface,
            CtfInsideFaceBalanceInput {
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 2.0,
                previous_inside_face_temperature_c: 18.0,
                net_inside_source_w_per_m2: 4.0,
            },
        );
        assert!((adiabatic - (135.0 / 9.5)).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn surface_balance_pass_can_freeze_outside_snapshot_for_inside_ctf_solve()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_id = state.surfaces[0].surface_id;
        let zone_id = state.surfaces[0].zone_id;
        let surface = &mut state.surfaces[0];
        surface.outside_boundary_condition = OutsideBoundaryCondition::Outdoors;
        surface.outside_face_temperature_c = 30.0;
        surface.inside_face_temperature_c = 18.0;
        surface.inside_radiant_internal_gain_w_per_m2 = 0.0;
        surface.inside_shortwave_absorbed_w_per_m2 = 0.0;
        surface.inside_additional_heat_source_w_per_m2 = 0.0;
        surface.inside_radiant_hvac_w_per_m2 = 0.0;
        surface.inside_net_longwave_w_per_m2 = 0.0;
        surface.ctf = SurfaceCtfState {
            outside_0_w_per_m2_k: 4.0,
            cross_0_w_per_m2_k: 0.5,
            inside_0_w_per_m2_k: 3.0,
            const_in_part_w_per_m2: 0.0,
            const_out_part_w_per_m2: 0.0,
            outside_history_w_per_m2_k: Vec::new(),
            cross_history_w_per_m2_k: Vec::new(),
            inside_history_w_per_m2_k: Vec::new(),
            flux_history: Vec::new(),
            outside_temperature_history_c: Vec::new(),
            inside_temperature_history_c: Vec::new(),
            outside_flux_history_w_per_m2: Vec::new(),
            inside_flux_history_w_per_m2: Vec::new(),
        };

        let first_pass_inside_temperatures = BTreeMap::from([(surface_id, 18.0)]);
        let zone_temperatures = BTreeMap::from([(zone_id, 20.0)]);
        let inside_convection_coefficients = BTreeMap::from([(surface_id, 2.0)]);
        let outside_snapshots = BTreeMap::from([(
            surface_id,
            SurfaceBoundaryBalanceResult {
                temperature_c: 12.0,
                exterior_report_terms: SurfaceExteriorReportTerms {
                    convection_heat_gain_rate_w: 77.0,
                    ..SurfaceExteriorReportTerms::default()
                },
                outside_balance_diagnostics: SurfaceOutsideBalanceDiagnostics::default(),
            },
        )]);

        run_surface_balance_passes(
            &model.typed,
            &mut state.surfaces,
            Some(&first_pass_inside_temperatures),
            None,
            None,
            &zone_temperatures,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: -20.0,
                hour_ending: 1,
                timestep_seconds: SECONDS_PER_HOUR,
            },
            None,
            1,
            false,
            false,
            false,
            None,
            false,
            InteriorLongwaveExchangeProbe::None,
            Some(&inside_convection_coefficients),
            None,
            Some(&outside_snapshots),
            None,
            false,
        );

        let surface = &state.surfaces[0];
        assert_eq!(surface.outside_face_temperature_c, 12.0);
        assert_eq!(
            surface.outside_report_terms.convection_heat_gain_rate_w,
            77.0
        );
        assert!((surface.inside_face_temperature_c - 13.6).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn surface_balance_pass_can_freeze_inside_ctf_outside_snapshot_without_mutating_report_state()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_id = state.surfaces[0].surface_id;
        let zone_id = state.surfaces[0].zone_id;
        let surface = &mut state.surfaces[0];
        surface.outside_boundary_condition = OutsideBoundaryCondition::Outdoors;
        surface.outside_face_temperature_c = 30.0;
        surface.inside_face_temperature_c = 18.0;
        surface.inside_radiant_internal_gain_w_per_m2 = 0.0;
        surface.inside_shortwave_absorbed_w_per_m2 = 0.0;
        surface.inside_additional_heat_source_w_per_m2 = 0.0;
        surface.inside_radiant_hvac_w_per_m2 = 0.0;
        surface.inside_net_longwave_w_per_m2 = 0.0;
        surface.ctf = SurfaceCtfState {
            outside_0_w_per_m2_k: 4.0,
            cross_0_w_per_m2_k: 0.5,
            inside_0_w_per_m2_k: 3.0,
            const_in_part_w_per_m2: 0.0,
            const_out_part_w_per_m2: 0.0,
            outside_history_w_per_m2_k: Vec::new(),
            cross_history_w_per_m2_k: Vec::new(),
            inside_history_w_per_m2_k: Vec::new(),
            flux_history: Vec::new(),
            outside_temperature_history_c: Vec::new(),
            inside_temperature_history_c: Vec::new(),
            outside_flux_history_w_per_m2: Vec::new(),
            inside_flux_history_w_per_m2: Vec::new(),
        };

        let first_pass_inside_temperatures = BTreeMap::from([(surface_id, 18.0)]);
        let zone_temperatures = BTreeMap::from([(zone_id, 20.0)]);
        let inside_convection_coefficients = BTreeMap::from([(surface_id, 2.0)]);
        let inside_ctf_outside_temperature_snapshots = BTreeMap::from([(surface_id, 12.0)]);

        run_surface_balance_passes(
            &model.typed,
            &mut state.surfaces,
            Some(&first_pass_inside_temperatures),
            None,
            None,
            &zone_temperatures,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: -20.0,
                hour_ending: 1,
                timestep_seconds: SECONDS_PER_HOUR,
            },
            None,
            1,
            false,
            false,
            false,
            None,
            false,
            InteriorLongwaveExchangeProbe::None,
            Some(&inside_convection_coefficients),
            None,
            None,
            Some(&inside_ctf_outside_temperature_snapshots),
            false,
        );

        let surface = &state.surfaces[0];
        assert!((surface.outside_face_temperature_c - 12.0).abs() > 1.0e-6);
        assert!((surface.inside_ctf_outside_temperature_c - 12.0).abs() < 1.0e-12);
        assert_ne!(
            surface.outside_report_terms.convection_heat_gain_rate_w,
            77.0
        );
        assert!((surface.inside_face_temperature_c - 13.6).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn surface_inside_ctf_source_terms_follow_energyplus_temp_term_slots()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.outside_face_temperature_c = 10.0;
        surface.inside_face_temperature_c = 19.0;
        surface.ctf.inside_0_w_per_m2_k = 3.0;
        surface.ctf.cross_0_w_per_m2_k = 0.5;
        surface.ctf.const_in_part_w_per_m2 = 1.0;
        surface.inside_radiant_internal_gain_w_per_m2 = 1.0;
        surface.inside_shortwave_absorbed_w_per_m2 = 2.0;
        surface.inside_additional_heat_source_w_per_m2 = 3.0;
        surface.inside_radiant_hvac_w_per_m2 = 4.0;
        surface.inside_net_longwave_w_per_m2 = 5.0;

        let source_terms = surface_inside_ctf_source_terms_w_per_m2(surface);
        assert!((source_terms - 15.0).abs() < 1.0e-12);

        let temperature = energyplus_ctf_inside_face_temperature_c(
            surface,
            CtfInsideFaceBalanceInput {
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 2.0,
                previous_inside_face_temperature_c: 18.0,
                net_inside_source_w_per_m2: source_terms,
            },
        );
        assert!((temperature - 15.1).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn radiant_internal_gains_follow_energyplus_area_absorptance_distribution()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.other_equipment[0].fraction_radiant = 0.25;
        let model = SimulationModel::from_typed(typed);
        let mut state = initialize_heat_balance_state(&model, 20.0)?;

        let absorbed_radiant_gain_w = state
            .surfaces
            .iter()
            .map(|surface| surface.inside_radiant_internal_gain_w_per_m2 * surface.area_m2)
            .sum::<f64>();
        assert!((absorbed_radiant_gain_w - 3.0).abs() < 1.0e-12);
        for surface in &state.surfaces {
            assert!((surface.inside_radiant_internal_gain_w_per_m2 - 0.5).abs() < 1.0e-12);
        }

        state.surfaces[0].inside_radiant_internal_gain_w_per_m2 = 10.0;
        update_surface_radiant_internal_gain_source_terms(&model.typed, &mut state.surfaces, 1);
        assert!((state.surfaces[0].inside_radiant_internal_gain_w_per_m2 - 0.5).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn interior_longwave_probe_is_zero_for_equal_surface_temperatures()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        for surface in &mut state.surfaces {
            surface.inside_face_temperature_c = 21.0;
            surface.inside_net_longwave_w_per_m2 = 12.0;
        }

        update_surface_inside_longwave_exchange_probe(&mut state.surfaces, None);

        for surface in &state.surfaces {
            assert!(surface.inside_net_longwave_w_per_m2.abs() < 1.0e-12);
        }

        Ok(())
    }

    #[test]
    fn interior_longwave_probe_conserves_zone_exchange_signs()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        for surface in &mut state.surfaces {
            surface.inside_face_temperature_c = 20.0;
        }
        state.surfaces[0].inside_face_temperature_c = 30.0;

        update_surface_inside_longwave_exchange_probe(&mut state.surfaces, None);

        assert!(state.surfaces[0].inside_net_longwave_w_per_m2 < 0.0);
        for surface in state.surfaces.iter().skip(1) {
            assert!(surface.inside_net_longwave_w_per_m2 > 0.0);
        }
        let zone_exchange_w = state
            .surfaces
            .iter()
            .map(|surface| surface.inside_net_longwave_w_per_m2 * surface.area_m2)
            .sum::<f64>();
        assert!(zone_exchange_w.abs() < 1.0e-9);

        Ok(())
    }

    #[test]
    fn scriptf_interior_longwave_probe_is_zero_for_equal_surface_temperatures()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        for surface in &mut state.surfaces {
            surface.inside_face_temperature_c = 21.0;
            surface.inside_net_longwave_w_per_m2 = 12.0;
        }

        update_surface_inside_scriptf_longwave_exchange_probe(&mut state.surfaces, None);

        for surface in &state.surfaces {
            assert!(surface.inside_net_longwave_w_per_m2.abs() < 1.0e-9);
        }

        Ok(())
    }

    #[test]
    fn scriptf_interior_longwave_probe_conserves_zone_exchange_signs()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        for surface in &mut state.surfaces {
            surface.inside_face_temperature_c = 20.0;
        }
        state.surfaces[0].inside_face_temperature_c = 30.0;

        update_surface_inside_scriptf_longwave_exchange_probe(&mut state.surfaces, None);

        assert!(state.surfaces[0].inside_net_longwave_w_per_m2 < 0.0);
        let zone_exchange_w = state
            .surfaces
            .iter()
            .map(|surface| surface.inside_net_longwave_w_per_m2 * surface.area_m2)
            .sum::<f64>();
        assert!(zone_exchange_w.abs() < 1.0e-8);

        Ok(())
    }

    #[test]
    fn scriptf_from_view_factors_matches_energyplus_1zone_eio_orientation() {
        let areas = [69.6773, 69.6773, 69.6773, 69.6773, 232.2576, 232.2576];
        let printed_final_view_factors = [
            [0.0000, 0.078565, 0.078565, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.0000, 0.078565, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.078565, 0.0000, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.078565, 0.078565, 0.0000, 0.3823, 0.3823],
            [0.1147, 0.1147, 0.1147, 0.1147, 0.0000, 0.5410],
            [0.1147, 0.1147, 0.1147, 0.1147, 0.5410, 0.0000],
        ];
        let surface_count = areas.len();
        let mut internal_view_factors = vec![0.0; surface_count * surface_count];
        for from_index in 0..surface_count {
            for to_index in 0..surface_count {
                internal_view_factors[to_index * surface_count + from_index] =
                    printed_final_view_factors[from_index][to_index];
            }
        }
        let mut emissivities = vec![0.9; surface_count];

        let script_f =
            energyplus_scriptf_from_view_factors(&areas, &internal_view_factors, &mut emissivities)
                .expect("script F matrix");
        let dimensionless = |sender_index: usize, receiver_index: usize| {
            script_f[sender_index * surface_count + receiver_index] / STEFAN_BOLTZMANN_W_PER_M2_K4
        };

        assert!((dimensionless(0, 4) - 0.3366).abs() < 5.0e-4);
        assert!((dimensionless(4, 0) - 0.1010).abs() < 5.0e-4);
        assert!((dimensionless(4, 5) - 0.4559).abs() < 5.0e-4);
        assert!((dimensionless(0, 0) - 0.0094307).abs() < 5.0e-5);
    }

    #[test]
    fn approximate_view_factors_match_energyplus_1zone_eio() {
        let areas = [69.6773, 69.6773, 69.6773, 69.6773, 232.2576, 232.2576];
        let surface_types = [
            SurfaceType::Wall,
            SurfaceType::Wall,
            SurfaceType::Wall,
            SurfaceType::Wall,
            SurfaceType::Floor,
            SurfaceType::Roof,
        ];
        let azimuths = [180.0, 90.0, 0.0, 270.0, 0.0, 0.0];
        let tilts = [90.0, 90.0, 90.0, 90.0, 180.0, 0.0];
        let snapshots = areas
            .iter()
            .copied()
            .zip(surface_types)
            .zip(azimuths)
            .zip(tilts)
            .map(|(((area_m2, surface_type), azimuth_deg), tilt_deg)| {
                InteriorLongwaveSurfaceSnapshot {
                    zone_id: ZoneId(0),
                    surface_type,
                    area_m2,
                    azimuth_deg,
                    tilt_deg,
                    temperature_k4: 293.15_f64.powi(4),
                    thermal_absorptance: 0.9,
                }
            })
            .collect::<Vec<_>>();
        let view_factors = fix_energyplus_approximate_view_factors(
            &areas,
            &energyplus_approximate_view_factors(&snapshots),
        );
        let printed_final_view_factors = [
            [0.0000, 0.078565, 0.078565, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.0000, 0.078565, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.078565, 0.0000, 0.078565, 0.3823, 0.3823],
            [0.078565, 0.078565, 0.078565, 0.0000, 0.3823, 0.3823],
            [0.1147, 0.1147, 0.1147, 0.1147, 0.0000, 0.5410],
            [0.1147, 0.1147, 0.1147, 0.1147, 0.5410, 0.0000],
        ];
        let surface_count = areas.len();
        for from_index in 0..surface_count {
            for to_index in 0..surface_count {
                let actual = view_factors[to_index * surface_count + from_index];
                let expected = printed_final_view_factors[from_index][to_index];
                assert!(
                    (actual - expected).abs() < 5.0e-4,
                    "view factor {from_index}->{to_index}: actual {actual}, expected {expected}"
                );
            }
        }
    }

    #[test]
    fn energyplus_ctf_outside_face_balance_uses_ctf_zero_terms()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.inside_face_temperature_c = 20.0;
        surface.ctf.outside_0_w_per_m2_k = 1.0;
        surface.ctf.cross_0_w_per_m2_k = 1.0;
        surface.ctf.const_out_part_w_per_m2 = 0.0;

        let temperature = energyplus_ctf_outside_face_temperature_c(
            surface,
            CtfOutsideFaceBalanceInput {
                outdoor_air_temperature_c: 10.0,
                radiant_temperature_c: 5.0,
                outside_convection_coefficient_w_per_m2_k: 3.0,
                outside_radiation_coefficient_w_per_m2_k: 2.0,
                absorbed_outside_source_w_per_m2: 7.0,
            },
        );

        assert!((temperature - (67.0 / 6.0)).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn energyplus_ctf_quick_outside_face_balance_uses_inside_balance_term()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.ctf.outside_0_w_per_m2_k = 3.0;
        surface.ctf.cross_0_w_per_m2_k = 2.0;
        surface.ctf.inside_0_w_per_m2_k = 4.0;
        surface.ctf.const_out_part_w_per_m2 = 11.0;
        surface.ctf.const_in_part_w_per_m2 = 13.0;

        let temperature = energyplus_ctf_outside_face_temperature_quick_conduction_c(
            surface,
            CtfOutsideQuickConductionBalanceInput {
                environmental: CtfOutsideFaceBalanceInput {
                    outdoor_air_temperature_c: 10.0,
                    radiant_temperature_c: 5.0,
                    outside_convection_coefficient_w_per_m2_k: 3.0,
                    outside_radiation_coefficient_w_per_m2_k: 2.0,
                    absorbed_outside_source_w_per_m2: 7.0,
                },
                reference_air_temperature_c: 20.0,
                inside_convection_coefficient_w_per_m2_k: 6.0,
                net_inside_source_w_per_m2: 17.0,
            },
        );

        assert!((temperature - (66.0 / 7.6)).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn heat_balance_timestep_advances_zone_air_state() -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: 10.0,
                hour_ending: 1,
                timestep_seconds: 600.0,
            },
        );

        assert_eq!(state.timestep_index, 1);
        assert_eq!(state.zones[0].previous_mean_air_temperatures_c, [20.0; 3]);
        assert_eq!(state.zones[0].convective_internal_gain_w, 12.0);
        assert_eq!(state.zones[0].opaque_surface_conductance_w_per_k, 6.0);
        assert!(state.zones[0].mean_air_temperature_c > 12.0);
        assert!(state.zones[0].mean_air_temperature_c < 20.0);
        assert!(state.zones[0].opaque_surface_heat_gain_w < 0.0);
        let expected_outside_conduction = state
            .surfaces
            .iter()
            .map(surface_outside_conduction_rate_w)
            .sum::<f64>();
        assert!(
            (state.zones[0].opaque_surface_outside_conduction_w - expected_outside_conduction)
                .abs()
                < 1.0e-12
        );
        assert_eq!(state.surfaces[0].outside_face_temperature_c, 10.0);
        assert!(
            state.surfaces[0].inside_face_temperature_c > state.zones[0].mean_air_temperature_c
        );
        assert!(state.surfaces[0].inside_face_temperature_c < 20.0);
        assert!(state.surfaces[0].heat_gain_to_zone_w < 0.0);
        let expected_sum_ha = state
            .surfaces
            .iter()
            .map(|surface| surface.inside_convection_coefficient_w_per_m2_k * surface.area_m2)
            .sum::<f64>();
        let expected_sum_hat_surf = state
            .surfaces
            .iter()
            .map(|surface| {
                surface.inside_convection_coefficient_w_per_m2_k
                    * surface.area_m2
                    * surface.inside_face_temperature_c
            })
            .sum::<f64>();
        assert!((state.zones[0].sum_ha_w_per_k - expected_sum_ha).abs() < 1.0e-12);
        assert!((state.zones[0].sum_hat_surf_w - expected_sum_hat_surf).abs() < 1.0e-12);
        assert_eq!(state.zones[0].sum_hat_ref_w, 0.0);
        let coefficients = state.zones[0].zone_air_temperature_coefficients;
        assert!(
            (coefficients.temp_dependent_coefficient_w_per_k - expected_sum_ha).abs() < 1.0e-12
        );
        assert!(
            (coefficients.temp_independent_coefficient_w
                - (state.zones[0].convective_internal_gain_w + expected_sum_hat_surf))
                .abs()
                < 1.0e-12
        );
        assert!((coefficients.air_power_cap_w_per_k - (1207.2 / 600.0)).abs() < 1.0e-12);
        let expected_history = (1207.2 / 600.0) * (3.0 * 20.0 - 1.5 * 20.0 + 20.0 / 3.0);
        assert!((coefficients.third_order_history_term_w - expected_history).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn zone_air_heat_balance_storage_rate_uses_source_algorithm_branch()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone = &mut state.zones[0];
        zone.mean_air_temperature_c = 21.0;
        zone.previous_mean_air_temperatures_c = [20.0, 19.0, 18.0];
        zone.air_heat_capacity_j_per_k = 1200.0;
        zone.zone_air_temperature_coefficients
            .temp_dependent_coefficient_w_per_k = 5.0;
        zone.zone_air_temperature_coefficients
            .temp_independent_coefficient_w = 200.0;

        let analytical = zone_air_heat_balance_air_storage_rate_w(
            zone,
            60.0,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            None,
        );
        assert!((analytical - 95.0).abs() < 1.0e-12);

        let third_order = zone_air_heat_balance_air_storage_rate_w(
            zone,
            60.0,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe,
            None,
        );
        assert!((third_order - 20.0).abs() < 1.0e-12);

        let third_order_report_capacity = zone_air_heat_balance_air_storage_rate_w(
            zone,
            60.0,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe,
            Some(600.0),
        );
        assert!((third_order_report_capacity - 10.0).abs() < 1.0e-12);

        let invalid_timestep = zone_air_heat_balance_air_storage_rate_w(
            zone,
            0.0,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe,
            Some(600.0),
        );
        assert_eq!(invalid_timestep, 0.0);

        Ok(())
    }

    #[test]
    fn system_timestep_air_storage_report_uses_weather_proxy_capacity()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone = &mut state.zones[0];
        zone.mean_air_temperature_c = 21.0;
        zone.air_humidity_ratio = 0.012;
        zone.air_heat_capacity_j_per_k = 1200.0;
        let previous_system_temperature_c = 20.0;
        let system_timestep_seconds = 60.0;
        let records = [EpwRecord {
            year: 2026,
            month: 1,
            day: 1,
            hour: 1,
            minute: 60,
            dry_bulb_c: 5.0,
            dew_point_c: 0.0,
            relative_humidity_percent: 50.0,
            atmospheric_pressure_pa: 82_000.0,
            horizontal_infrared_radiation_wh_per_m2: 300.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 0.0,
            wind_speed_m_per_s: 0.0,
            liquid_precipitation_depth_mm: 0.0,
        }];
        let context = HeatBalanceWeatherContext {
            records: &records,
            record_index: 0,
            zone_steps_per_hour: 4,
            zone_timestep: Some(1),
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        };
        let expected_capacity = energyplus_zone_air_heat_capacity_j_per_k(
            zone.volume_m3,
            82_000.0,
            zone.mean_air_temperature_c,
            zone.air_humidity_ratio,
        )
        .ok_or_else(|| std::io::Error::other("missing expected air capacity"))?;

        let storage_rate = zone_air_system_timestep_storage_report_rate_w(
            zone,
            previous_system_temperature_c,
            system_timestep_seconds,
            Some(context),
            records[0].dry_bulb_c,
        );
        let stale_capacity_rate = zone.air_heat_capacity_j_per_k
            * (zone.mean_air_temperature_c - previous_system_temperature_c)
            / system_timestep_seconds;
        let expected_rate = expected_capacity
            * (zone.mean_air_temperature_c - previous_system_temperature_c)
            / system_timestep_seconds;

        assert!((storage_rate - expected_rate).abs() < 1.0e-9);
        assert!((storage_rate - stale_capacity_rate).abs() > 1.0e-3);

        Ok(())
    }
    #[test]
    fn zone_air_heat_balance_surface_convection_can_use_report_air_temperature()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone_id = {
            let zone = &mut state.zones[0];
            zone.mean_air_temperature_c = 21.0;
            zone.previous_mean_air_temperatures_c = [20.0, 19.0, 18.0];
            zone.sum_ha_w_per_k = 10.0;
            zone.sum_hat_surf_w = 250.0;
            zone.sum_hat_ref_w = 5.0;

            assert!((zone_air_heat_balance_surface_convection_rate_w(zone) - 35.0).abs() < 1.0e-12);
            assert!(
                (zone_air_heat_balance_surface_convection_rate_at_air_temperature_w(
                    zone,
                    zone.previous_mean_air_temperatures_c[0]
                ) - 45.0)
                    .abs()
                    < 1.0e-12
            );
            zone.convective_internal_gain_w = 7.0;
            assert!(
                (zone_air_heat_balance_surface_convection_rate_from_balance_w(zone, 45.0) - 38.0)
                    .abs()
                    < 1.0e-12
            );
            zone.zone_id
        };

        state.surfaces[0].inside_convection_coefficient_w_per_m2_k = 2.0;
        state.surfaces[0].area_m2 = 3.0;
        state.surfaces[0].inside_face_temperature_c = 22.0;
        state.surfaces[0].inside_reference_air_temperature_c = 20.0;
        assert!(
            (zone_air_heat_balance_surface_convection_rate_from_surface_reference_air_w(
                &state.surfaces,
                zone_id
            ) - 12.0)
                .abs()
                < 1.0e-12
        );
        assert!(
            (surface_inside_convection_heat_gain_rate_per_area_w_per_m2(
                &state.surfaces[0],
                &state.zones,
                true,
                false,
            ) + 4.0)
                .abs()
                < 1.0e-12
        );
        let scriptf_flat_probe =
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe;
        let converged_surface_probe =
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe;
        assert!(heat_balance_uses_balance_surface_convection_report(
            converged_surface_probe
        ));
        assert!(!heat_balance_uses_balance_surface_convection_report(
            scriptf_flat_probe
        ));
        assert!(
            heat_balance_uses_surface_reference_air_surface_convection_report(scriptf_flat_probe)
        );
        assert!(!heat_balance_uses_surface_reference_air_convection_report(
            scriptf_flat_probe
        ));
        let final_coefficient = surface_inside_convection_report_coefficient_w_per_m2_k(
            &state.surfaces[0],
            &state.zones,
            false,
            true,
        );
        assert!(
            (final_coefficient
                - energyplus_tarp_inside_convection_coefficient_w_per_m2_k(
                    &state.surfaces[0],
                    state.surfaces[0].inside_face_temperature_c,
                    state.zones[0].mean_air_temperature_c,
                ))
            .abs()
                < 1.0e-12
        );

        Ok(())
    }

    #[test]
    fn heat_balance_timestep_uses_previous_surface_temperature_for_ctf_damping()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        state.surfaces[0].inside_face_temperature_c = 40.0;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: 20.0,
                hour_ending: 1,
                timestep_seconds: 60.0,
            },
        );

        assert!(
            state.surfaces[0].inside_face_temperature_c > 25.0,
            "CTF damping should use the previous surface temperature, not the overwritten zone temperature"
        );

        Ok(())
    }

    #[test]
    fn heat_balance_adiabatic_surfaces_do_not_create_artificial_losses()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        for surface in &mut typed.surfaces {
            surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
            surface.outside_boundary_condition_object = None;
        }
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: -10.0,
                hour_ending: 1,
                timestep_seconds: 600.0,
            },
        );

        assert!(state.zones[0].mean_air_temperature_c > 20.0);
        assert!((state.zones[0].opaque_surface_heat_gain_w).abs() < 1.0e-9);
        assert!((state.zones[0].opaque_surface_outside_conduction_w).abs() < 1.0e-9);
        for surface in &state.surfaces {
            assert_eq!(
                surface.outside_boundary_condition,
                OutsideBoundaryCondition::Adiabatic
            );
            assert_eq!(
                surface.outside_face_temperature_c,
                surface.inside_face_temperature_c
            );
            assert!(surface.heat_gain_to_zone_w.abs() < 1.0e-9);
        }

        Ok(())
    }

    #[test]
    fn heat_balance_interzone_surface_uses_adjacent_zone_temperature()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = two_zone_interzone_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        state.zones[0].mean_air_temperature_c = 20.0;
        state.zones[1].mean_air_temperature_c = 10.0;

        advance_heat_balance_state_one_timestep(
            &typed,
            &mut state,
            HeatBalanceStepInput {
                outdoor_dry_bulb_c: 0.0,
                hour_ending: 1,
                timestep_seconds: 60.0,
            },
        );

        let warm_zone = state
            .zones
            .iter()
            .find(|zone| zone.zone_name == "ZONE A")
            .ok_or_else(|| std::io::Error::other("missing warm zone"))?;
        let cool_zone = state
            .zones
            .iter()
            .find(|zone| zone.zone_name == "ZONE B")
            .ok_or_else(|| std::io::Error::other("missing cool zone"))?;
        assert!(warm_zone.mean_air_temperature_c < 20.0);
        assert!(cool_zone.mean_air_temperature_c > 10.0);

        let warm_surface = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "A WALL")
            .ok_or_else(|| std::io::Error::other("missing A WALL"))?;
        assert_eq!(
            warm_surface.outside_boundary_target_surface_id,
            Some(SurfaceId(1))
        );
        assert_eq!(
            warm_surface.outside_boundary_target_zone_id,
            Some(ZoneId(1))
        );
        assert_eq!(
            warm_surface.outside_face_temperature_c,
            cool_zone.mean_air_temperature_c
        );
        assert!(warm_surface.heat_gain_to_zone_w < 0.0);

        let cool_surface = state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "B WALL")
            .ok_or_else(|| std::io::Error::other("missing B WALL"))?;
        assert_eq!(
            cool_surface.outside_face_temperature_c,
            warm_zone.mean_air_temperature_c
        );
        assert!(cool_surface.heat_gain_to_zone_w > 0.0);

        Ok(())
    }

    #[test]
    fn heat_balance_missing_interzone_surface_target_fails() {
        let mut typed = two_zone_interzone_model();
        typed.surfaces[0].outside_boundary_condition_object =
            Some(NormalizedName::new("Missing Surface"));
        let model = SimulationModel::from_typed(typed);

        assert!(matches!(
            initialize_heat_balance_state(&model, 20.0),
            Err(RuntimeError::MissingSurfaceBoundaryTarget { .. })
        ));
    }

    #[test]
    fn heat_balance_trace_writes_zone_air_temperature_results()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());

        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;

        assert_eq!(simulation.summary.samples, 2);
        assert_eq!(simulation.summary.timestep_count, 12);
        assert_eq!(simulation.summary.zone_count, 1);
        assert_eq!(simulation.summary.surface_count, 6);
        assert_eq!(simulation.state.timestep_index, 12);
        assert_eq!(simulation.results.sample_count(), 2);
        assert_eq!(simulation.results.series.len(), 329);
        assert_eq!(
            simulation.summary.run_period_initial_zone_air_states.len(),
            1
        );

        let Some(zone_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing zone series").into());
        };
        assert!(zone_series.values[0] > 11.9);
        assert!(zone_series.values[0] < 20.0);
        assert!(zone_series.values[1] > zone_series.values[0]);

        let Some(zone_humidity_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Humidity Ratio")
        else {
            return Err(std::io::Error::other("missing zone humidity series").into());
        };
        assert_eq!(zone_humidity_series.values.len(), 2);

        let Some(sky_temperature_series) = simulation
            .results
            .find_series("Environment", "Site Sky Temperature")
        else {
            return Err(std::io::Error::other("missing sky temperature series").into());
        };
        assert_eq!(sky_temperature_series.values.len(), 2);

        let Some(horizontal_infrared_series) = simulation.results.find_series(
            "Environment",
            "Site Horizontal Infrared Radiation Rate per Area",
        ) else {
            return Err(std::io::Error::other("missing horizontal infrared series").into());
        };
        assert_eq!(horizontal_infrared_series.values.len(), 2);

        let Some(zone_air_capacity_series) = simulation
            .results
            .find_series("ZONE ONE", super::RUST_ZONE_AIR_HEAT_CAPACITY_VARIABLE)
        else {
            return Err(std::io::Error::other("missing zone-air debug series").into());
        };
        assert_eq!(zone_air_capacity_series.values.len(), 2);

        let Some(inside_convection_series) = simulation.results.find_series(
            "FLOOR",
            "Surface Inside Face Convection Heat Transfer Coefficient",
        ) else {
            return Err(std::io::Error::other("missing inside convection series").into());
        };
        assert_eq!(inside_convection_series.values.len(), 2);
        let Some(adjacent_air_series) = simulation
            .results
            .find_series("FLOOR", "Surface Inside Face Adjacent Air Temperature")
        else {
            return Err(std::io::Error::other("missing adjacent air series").into());
        };
        assert_eq!(adjacent_air_series.values.len(), 2);
        let Some(iteration_count_series) = simulation.results.find_series(
            "Simulation",
            super::SURFACE_INSIDE_HEAT_BALANCE_ITERATION_COUNT_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing inside surface iteration count").into());
        };
        assert_eq!(iteration_count_series.values, vec![6.0, 6.0]);
        let Some(outside_balance_series) = simulation.results.find_series(
            "ROOF",
            super::SURFACE_OUTSIDE_BALANCE_REPORT_TEMPERATURE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing outside balance report temperature").into());
        };
        assert_eq!(outside_balance_series.values.len(), 2);

        let Some(weather_series) = simulation
            .results
            .find_series("Environment", "Site Outdoor Air Drybulb Temperature")
        else {
            return Err(std::io::Error::other("missing weather series").into());
        };
        assert_eq!(weather_series.values, vec![10.0, 12.0]);

        let Some(inside_surface_series) = simulation
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing inside surface series").into());
        };
        assert_eq!(inside_surface_series.values.len(), 2);
        assert!(inside_surface_series.values[0].is_finite());
        assert_ne!(inside_surface_series.values[0], zone_series.values[0]);

        let Some(outside_surface_series) = simulation
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing outside surface series").into());
        };
        assert_eq!(outside_surface_series.values, vec![10.0, 12.0]);

        let Some(inside_conduction_series) = simulation
            .results
            .find_series("FLOOR", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing inside conduction series").into());
        };
        assert_eq!(inside_conduction_series.values.len(), 2);
        assert!(inside_conduction_series.values[0] < 0.0);

        let Some(outside_conduction_series) = simulation.results.find_series(
            "FLOOR",
            "Surface Outside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing outside conduction series").into());
        };
        assert_eq!(
            outside_conduction_series.values[0],
            -inside_conduction_series.values[0]
        );
        let Some(inside_current_outside_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_INSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF inside outside term").into());
        };
        let Some(inside_current_inside_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_INSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF inside inside term").into());
        };
        let Some(inside_history_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_INSIDE_HISTORY_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF inside history term").into());
        };
        let Some(inside_history_temperature_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_INSIDE_HISTORY_TEMPERATURE_TERM_RATE_VARIABLE,
        ) else {
            return Err(
                std::io::Error::other("missing CTF inside history temperature term").into(),
            );
        };
        let Some(inside_history_flux_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_INSIDE_HISTORY_FLUX_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF inside history flux term").into());
        };
        let Some(outside_current_outside_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_OUTSIDE_CURRENT_OUTSIDE_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF outside outside term").into());
        };
        let Some(outside_current_inside_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_OUTSIDE_CURRENT_INSIDE_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF outside inside term").into());
        };
        let Some(outside_history_term) = simulation.results.find_series(
            "FLOOR",
            super::SURFACE_CTF_OUTSIDE_HISTORY_TERM_RATE_VARIABLE,
        ) else {
            return Err(std::io::Error::other("missing CTF outside history term").into());
        };
        assert!(
            (inside_conduction_series.values[0]
                - inside_current_outside_term.values[0]
                - inside_current_inside_term.values[0]
                - inside_history_term.values[0])
                .abs()
                < 1.0e-9
        );
        assert!(
            (inside_history_term.values[0]
                - inside_history_temperature_term.values[0]
                - inside_history_flux_term.values[0])
                .abs()
                < 1.0e-9
        );
        assert!(
            (outside_conduction_series.values[0]
                - outside_current_outside_term.values[0]
                - outside_current_inside_term.values[0]
                - outside_history_term.values[0])
                .abs()
                < 1.0e-9
        );
        let Some(storage_series) = simulation
            .results
            .find_series("FLOOR", "Surface Heat Storage Rate")
        else {
            return Err(std::io::Error::other("missing surface heat storage series").into());
        };
        assert_eq!(storage_series.values.len(), 2);
        assert!(
            (storage_series.values[0]
                + inside_conduction_series.values[0]
                + outside_conduction_series.values[0])
                .abs()
                < 1.0e-9
        );
        let Some(storage_per_area_series) = simulation
            .results
            .find_series("FLOOR", "Surface Heat Storage Rate per Area")
        else {
            return Err(
                std::io::Error::other("missing surface heat storage per-area series").into(),
            );
        };
        assert_eq!(storage_per_area_series.values.len(), 2);
        assert!(
            (storage_per_area_series.values[0] - storage_series.values[0] / 100.0).abs() < 1.0e-9
        );

        let Some(zone_conduction_series) = simulation.results.find_series(
            "ZONE ONE",
            "Zone Opaque Surface Inside Faces Conduction Rate",
        ) else {
            return Err(std::io::Error::other("missing zone conduction series").into());
        };
        assert!(zone_conduction_series.values[0] < 0.0);

        let Some(zone_outside_conduction_series) = simulation.results.find_series(
            "ZONE ONE",
            "Zone Opaque Surface Outside Faces Conduction Rate",
        ) else {
            return Err(std::io::Error::other("missing zone outside conduction series").into());
        };
        assert_eq!(zone_outside_conduction_series.values.len(), 2);
        assert!(zone_outside_conduction_series.values[0].is_finite());

        let Some(surface_convection_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate")
        else {
            return Err(std::io::Error::other("missing zone air surface convection series").into());
        };
        assert_eq!(surface_convection_series.values.len(), 2);
        assert!(surface_convection_series.values[0].is_finite());

        Ok(())
    }

    #[test]
    fn compat_candidate_report_flags_follow_execution_variant() {
        let report_algorithm = super::heat_balance_zone_air_algorithm_execution_variant(
            HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate,
        );

        assert_eq!(
            report_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusSourceOrder1ZoneOpaqueCompatibility
        );
        assert!(
            super::heat_balance_uses_surface_reference_air_surface_convection_report(
                report_algorithm
            )
        );
        assert!(
            !super::heat_balance_uses_surface_reference_air_surface_convection_report(
                HeatBalanceZoneAirAlgorithm::EnergyPlusHeatBalanceCompatCandidate
            )
        );
    }

    #[test]
    fn heat_balance_zone_air_rate_outputs_follow_report_sampling()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let options = HeatBalanceSimulationOptions::hourly_samples(1);
        let simulation = simulate_heat_balance_zone_air_temperatures(&model, &[10.0], options)?;
        assert_eq!(
            simulation.summary.zone_air_report_sampling,
            HeatBalanceZoneAirReportSampling::Average
        );
        let last_state_options = options
            .with_zone_air_report_sampling(HeatBalanceZoneAirReportSampling::LastSystemState);
        let last_state_simulation =
            simulate_heat_balance_zone_air_temperatures(&model, &[10.0], last_state_options)?;
        assert_eq!(
            last_state_simulation.summary.zone_air_report_sampling,
            HeatBalanceZoneAirReportSampling::LastSystemState
        );
        let steps = model.typed.timestep.number_of_timesteps_per_hour.max(1);
        let timestep_seconds = SECONDS_PER_HOUR / f64::from(steps);
        let mut state =
            initialize_heat_balance_state(&model, options.initial_zone_air_temperature_c)?;
        let mut surface_convection_sum = 0.0;
        let mut air_storage_sum = 0.0;
        let mut last_surface_convection = 0.0;
        let mut last_air_storage = 0.0;

        for _substep in 1..=steps {
            advance_heat_balance_state_one_timestep_internal(
                &model.typed,
                &mut state,
                HeatBalanceStepInput {
                    outdoor_dry_bulb_c: 10.0,
                    hour_ending: 1,
                    timestep_seconds,
                },
                None,
                options.zone_air_algorithm,
                options.surface_iteration_count,
                options.inside_hconv_reevaluation_interval,
                options.surface_loop_zone_air_correction,
            );
            let zone = &state.zones[0];
            last_surface_convection = zone_air_heat_balance_surface_convection_rate_w(zone);
            last_air_storage = zone_air_heat_balance_air_storage_rate_w(
                zone,
                timestep_seconds,
                options.zone_air_algorithm,
                None,
            );
            surface_convection_sum += last_surface_convection;
            air_storage_sum += last_air_storage;
        }

        let divisor = f64::from(steps);
        let surface_convection_series = simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate")
            .ok_or_else(|| std::io::Error::other("missing surface convection series"))?;
        assert!(
            (surface_convection_series.values[0] - surface_convection_sum / divisor).abs() < 1.0e-9
        );
        let last_surface_convection_series = last_state_simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Surface Convection Rate")
            .ok_or_else(|| std::io::Error::other("missing last surface convection series"))?;
        assert!(
            (last_surface_convection_series.values[0] - last_surface_convection).abs() < 1.0e-9
        );
        let air_storage_series = simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Air Energy Storage Rate")
            .ok_or_else(|| std::io::Error::other("missing air storage series"))?;
        assert!((air_storage_series.values[0] - air_storage_sum / divisor).abs() < 1.0e-9);
        let last_air_storage_series = last_state_simulation
            .results
            .find_series("ZONE ONE", "Zone Air Heat Balance Air Energy Storage Rate")
            .ok_or_else(|| std::io::Error::other("missing last air storage series"))?;
        assert!((last_air_storage_series.values[0] - last_air_storage).abs() < 1.0e-9);

        Ok(())
    }

    #[test]
    fn zone_surface_report_conduction_rates_sum_surface_report_terms()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let zone_id = state.zones[0].zone_id;
        for surface in &mut state.surfaces {
            surface.ctf = SurfaceCtfState {
                outside_0_w_per_m2_k: 0.0,
                cross_0_w_per_m2_k: 0.0,
                inside_0_w_per_m2_k: 0.0,
                const_in_part_w_per_m2: 0.0,
                const_out_part_w_per_m2: 0.0,
                outside_history_w_per_m2_k: Vec::new(),
                cross_history_w_per_m2_k: Vec::new(),
                inside_history_w_per_m2_k: Vec::new(),
                flux_history: Vec::new(),
                outside_temperature_history_c: Vec::new(),
                inside_temperature_history_c: Vec::new(),
                outside_flux_history_w_per_m2: Vec::new(),
                inside_flux_history_w_per_m2: Vec::new(),
            };
        }

        let [first, second, ..] = state.surfaces.as_mut_slice() else {
            return Err(std::io::Error::other("missing test surfaces").into());
        };
        first.area_m2 = 2.0;
        first.inside_face_temperature_c = 20.0;
        first.outside_face_temperature_c = 10.0;
        first.ctf.cross_0_w_per_m2_k = 1.0;
        first.ctf.outside_0_w_per_m2_k = 0.5;
        first.ctf.const_in_part_w_per_m2 = 3.0;
        first.ctf.const_out_part_w_per_m2 = 4.0;

        second.area_m2 = 3.0;
        second.inside_face_temperature_c = 18.0;
        second.outside_face_temperature_c = 12.0;
        second.ctf.cross_0_w_per_m2_k = 2.0;
        second.ctf.inside_0_w_per_m2_k = 1.0;
        second.ctf.outside_0_w_per_m2_k = 1.5;
        second.ctf.const_in_part_w_per_m2 = -1.0;
        second.ctf.const_out_part_w_per_m2 = 0.5;

        let (inside, outside) =
            zone_surface_report_conduction_rates_w(&state.surfaces, zone_id, false);
        assert!((inside - 41.0).abs() < 1.0e-12);
        assert!((outside - 74.5).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn heat_balance_zone_air_algorithm_option_defaults_to_simplified() {
        let options = HeatBalanceSimulationOptions::hourly_samples(2);

        assert_eq!(
            options.zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        );
        assert_eq!(options.surface_iteration_count, 1);
        assert_eq!(
            options.zone_conduction_report_source,
            HeatBalanceZoneConductionReportSource::ZoneState
        );
        assert_eq!(
            options.zone_air_report_sampling,
            HeatBalanceZoneAirReportSampling::Average
        );
        assert_eq!(
            options.surface_loop_zone_air_correction,
            HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration
        );
        assert_eq!(
            options
                .with_zone_conduction_report_source(
                    HeatBalanceZoneConductionReportSource::SurfaceReport
                )
                .zone_conduction_report_source,
            HeatBalanceZoneConductionReportSource::SurfaceReport
        );
        assert_eq!(
            options
                .with_zone_air_report_sampling(HeatBalanceZoneAirReportSampling::LastSystemState)
                .zone_air_report_sampling,
            HeatBalanceZoneAirReportSampling::LastSystemState
        );
        assert_eq!(
            options
                .with_surface_loop_zone_air_correction(
                    HeatBalanceSurfaceLoopZoneAirCorrection::AfterSurfaceLoop
                )
                .surface_loop_zone_air_correction,
            HeatBalanceSurfaceLoopZoneAirCorrection::AfterSurfaceLoop
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe)
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe,
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvCurrentAdiabaticProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStoragePreviousMatSurfaceConvectionProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceFrozenOutsideProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveReferenceAirProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatLiveHconvProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatSurfaceReferenceAirReportProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatFinalHconvReportProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatInsideCtfReportProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceInsideCtfOutsideHistoryScriptFFlatAdiabaticReportProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceAdiabaticHistoryCommitProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedScriptFInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2Probe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2InteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideScriptFInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideDoe2ScriptFInteriorLongwaveProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
                )
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe
        );
        assert_eq!(
            options
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe)
                .zone_air_algorithm,
            HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe
        );
        assert_eq!(
            options
                .with_surface_iteration_count(0)
                .surface_iteration_count,
            1
        );
        assert_eq!(
            options
                .with_surface_iteration_count(3)
                .surface_iteration_count,
            3
        );
    }

    #[test]
    fn heat_balance_surface_loop_zone_air_correction_runs_after_loop_probe()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[5.0, 35.0],
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedProbe,
                )
                .with_surface_iteration_count(3)
                .with_surface_loop_zone_air_correction(
                    HeatBalanceSurfaceLoopZoneAirCorrection::AfterSurfaceLoop,
                ),
        )?;

        assert_eq!(
            simulation.summary.surface_loop_zone_air_correction,
            HeatBalanceSurfaceLoopZoneAirCorrection::AfterSurfaceLoop
        );
        let zone_temperature = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
            .ok_or_else(|| std::io::Error::other("missing zone temperature series"))?;
        assert_eq!(zone_temperature.values.len(), 2);

        Ok(())
    }

    #[test]
    fn heat_balance_uses_source_declared_doe2_outside_convection() {
        let mut model = TypedModel::default();

        assert!(!heat_balance_uses_doe2_outside_convection(
            &model,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        ));
        assert!(heat_balance_uses_doe2_outside_convection(
            &model,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe
        ));

        model.surface_convection_algorithms.outside = Some(OutsideSurfaceConvectionAlgorithm::Doe2);

        assert!(heat_balance_uses_doe2_outside_convection(
            &model,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical
        ));
    }

    #[test]
    fn quick_outside_probe_reuses_cached_exterior_report_terms()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let Some(surface) = state.surfaces.iter_mut().find(|surface| {
            surface.outside_boundary_condition == OutsideBoundaryCondition::Outdoors
        }) else {
            return Err(std::io::Error::other("missing outdoor surface").into());
        };
        surface.outside_report_terms = SurfaceExteriorReportTerms {
            convection_heat_gain_rate_w: 1.0,
            convection_heat_gain_rate_per_area_w_per_m2: 2.0,
            convection_coefficient_w_per_m2_k: 3.0,
            net_thermal_radiation_heat_gain_rate_w: 4.0,
            net_thermal_radiation_heat_gain_rate_per_area_w_per_m2: 5.0,
            thermal_radiation_to_air_coefficient_w_per_m2_k: 6.0,
            thermal_radiation_to_sky_coefficient_w_per_m2_k: 7.0,
            thermal_radiation_to_ground_coefficient_w_per_m2_k: 8.0,
            solar_radiation_heat_gain_rate_w: 9.0,
            solar_radiation_heat_gain_rate_per_area_w_per_m2: 10.0,
        };

        let cached_terms = surface_exterior_report_terms(
            &model.typed,
            surface,
            10.0,
            surface.outside_face_temperature_c,
            None,
            HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe,
        );
        let fallback_terms = surface_exterior_report_terms(
            &model.typed,
            surface,
            10.0,
            surface.outside_face_temperature_c,
            None,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
        );

        assert_eq!(cached_terms, surface.outside_report_terms);
        assert_eq!(fallback_terms, SurfaceExteriorReportTerms::default());

        Ok(())
    }

    #[test]
    fn quick_outside_balance_freezes_exterior_coefficient_temperature()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_state = state
            .surfaces
            .iter_mut()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof test surface"))?;
        surface_state.outside_face_temperature_c = 60.0;
        let typed_surface = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing typed roof test surface"))?;
        let record = weather_record_with_precipitation(0.0);

        let quick_context = QuickOutsideConductionContext {
            reference_air_temperature_c: 20.0,
            inside_convection_coefficient_w_per_m2_k: 3.0,
            net_inside_source_w_per_m2: 0.0,
            exterior_coefficient_surface_temperature_c: Some(20.0),
            use_doe2_outside_convection: true,
        };
        let frozen = exterior_surface_energy_balance(
            surface_state,
            typed_surface,
            &record,
            10.0,
            20.0,
            0.0,
            Terrain::Suburbs,
            0.0,
            0.0,
            300.0,
            Some(quick_context),
            true,
            10.0,
            0.0,
            quick_context.exterior_coefficient_surface_temperature_c,
        );
        let unfrozen = exterior_surface_energy_balance(
            surface_state,
            typed_surface,
            &record,
            10.0,
            20.0,
            0.0,
            Terrain::Suburbs,
            0.0,
            0.0,
            300.0,
            Some(QuickOutsideConductionContext {
                exterior_coefficient_surface_temperature_c: None,
                ..quick_context
            }),
            true,
            10.0,
            0.0,
            None,
        );
        let expected_coefficient = energyplus_doe2_outside_convection_coefficient_w_per_m2_k(
            20.0,
            10.0,
            surface_tilt_deg(typed_surface.surface_type, &typed_surface.vertices)
                .to_radians()
                .cos(),
            surface_azimuth_deg(&typed_surface.vertices),
            0.0,
            0.0,
            surface_state.outside_layer_roughness,
        );

        assert!(
            (frozen
                .exterior_report_terms
                .convection_coefficient_w_per_m2_k
                - expected_coefficient)
                .abs()
                < 1.0e-12
        );
        assert!(
            unfrozen
                .exterior_report_terms
                .convection_coefficient_w_per_m2_k
                > frozen
                    .exterior_report_terms
                    .convection_coefficient_w_per_m2_k
                    + 1.0
        );

        Ok(())
    }

    #[test]
    fn energyplus_weather_record_is_rain_uses_hourly_threshold() {
        let mut record = weather_record_with_precipitation(0.799);
        assert!(!energyplus_weather_record_is_rain_at_timestep(
            &[record],
            0,
            1,
            1
        ));

        record.liquid_precipitation_depth_mm = 0.8;
        assert!(energyplus_weather_record_is_rain_at_timestep(
            &[record],
            0,
            1,
            1
        ));
    }

    #[test]
    fn energyplus_wet_timestep_fraction_uses_weather_interpolation() {
        let typed = cube_model();
        let typed_surface = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .expect("roof test surface");
        let records = [
            weather_record_with_precipitation(21.0),
            weather_record_with_precipitation(0.0),
        ];

        assert_eq!(
            energyplus_exterior_wet_timestep_fraction(&records, 1, 4, typed_surface),
            0.75
        );
    }

    #[test]
    fn energyplus_weather_context_uses_timestep_rain_and_dry_bulb_interpolation() {
        let typed = cube_model();
        let typed_surface = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .expect("roof test surface");
        let mut previous = weather_record_with_precipitation(0.0);
        previous.dry_bulb_c = 10.0;
        previous.relative_humidity_percent = 40.0;
        previous.atmospheric_pressure_pa = 80_000.0;
        previous.wind_speed_m_per_s = 2.0;
        previous.wind_direction_deg = 350.0;
        let mut current = weather_record_with_precipitation(1.0);
        current.dry_bulb_c = 22.0;
        current.relative_humidity_percent = 80.0;
        current.atmospheric_pressure_pa = 84_000.0;
        current.wind_speed_m_per_s = 10.0;
        current.wind_direction_deg = 10.0;
        previous.horizontal_infrared_radiation_wh_per_m2 = 200.0;
        current.horizontal_infrared_radiation_wh_per_m2 = 600.0;
        let records = [previous, current];

        assert!(
            (energyplus_weather_dry_bulb_at_timestep(Some(&records), 1, 22.0, 4, 2) - 16.0).abs()
                < 1.0e-12
        );
        assert!(
            (energyplus_weather_wind_speed_at_timestep(&records, 1, 10.0, 4, 2) - 6.0).abs()
                < 1.0e-12
        );
        assert!(
            (energyplus_weather_relative_humidity_at_timestep(&records, 1, 80.0, 4, 2) - 60.0)
                .abs()
                < 1.0e-12
        );
        assert!(
            (energyplus_weather_atmospheric_pressure_at_timestep(&records, 1, 84_000.0, 4, 2)
                - 82_000.0)
                .abs()
                < 1.0e-12
        );
        assert!(
            (energyplus_weather_wind_direction_at_timestep(&records, 1, 10.0, 4, 2) - 0.0).abs()
                < 1.0e-12
        );
        assert!(
            (energyplus_weather_horizontal_infrared_at_timestep(&records, 1, 600.0, 4, 2) - 400.0)
                .abs()
                < 1.0e-12
        );
        assert_eq!(
            energyplus_exterior_wet_context_fraction(
                HeatBalanceWeatherContext {
                    records: &records,
                    record_index: 1,
                    zone_steps_per_hour: 4,
                    zone_timestep: Some(3),
                    first_hour_interpolation_starting_values:
                        FirstHourInterpolationStartingValues::Hour24,
                },
                typed_surface,
            ),
            0.0
        );
        assert_eq!(
            energyplus_exterior_wet_context_fraction(
                HeatBalanceWeatherContext {
                    records: &records,
                    record_index: 1,
                    zone_steps_per_hour: 4,
                    zone_timestep: Some(4),
                    first_hour_interpolation_starting_values:
                        FirstHourInterpolationStartingValues::Hour24,
                },
                typed_surface,
            ),
            1.0
        );
        assert_eq!(
            energyplus_exterior_wet_context_fraction(
                HeatBalanceWeatherContext {
                    records: &records,
                    record_index: 1,
                    zone_steps_per_hour: 4,
                    zone_timestep: None,
                    first_hour_interpolation_starting_values:
                        FirstHourInterpolationStartingValues::Hour24,
                },
                typed_surface,
            ),
            0.25
        );
    }

    #[test]
    fn first_hour_weather_interpolation_uses_run_period_day_seed() {
        let mut records = vec![weather_record_with_precipitation(0.0); 25];
        records[0].dry_bulb_c = -3.0;
        records[23].dry_bulb_c = -11.0;
        records[24].dry_bulb_c = 4.0;

        let default_hour24 =
            energyplus_weather_dry_bulb_at_timestep(Some(&records), 0, records[0].dry_bulb_c, 4, 1);
        let explicit_hour1 = energyplus_weather_dry_bulb_at_timestep_with_starting_values(
            Some(&records),
            0,
            records[0].dry_bulb_c,
            4,
            1,
            FirstHourInterpolationStartingValues::Hour1,
        );

        assert!((default_hour24 - -9.0).abs() < 1.0e-12);
        assert!((explicit_hour1 - -3.0).abs() < 1.0e-12);
    }

    #[test]
    fn energyplus_zone_air_heat_capacity_uses_moist_air_psychrometrics() {
        let humidity_ratio = 0.0075;
        let density = energyplus_moist_air_density_kg_per_m3(82_000.0, 20.0, humidity_ratio)
            .expect("valid moist-air density");
        let expected_density =
            82_000.0 / (287.0 * (20.0 + KELVIN_OFFSET) * (1.0 + 1.607_768_7 * humidity_ratio));
        assert!((density - expected_density).abs() < 1.0e-12);

        let specific_heat = energyplus_moist_air_specific_heat_j_per_kg_k(humidity_ratio);
        let expected_specific_heat = 1.004_84e3 + humidity_ratio * 1.858_95e3;
        assert!((specific_heat - expected_specific_heat).abs() < 1.0e-12);

        let volume_m3 = 10.0;
        let heat_capacity =
            energyplus_zone_air_heat_capacity_j_per_k(volume_m3, 82_000.0, 20.0, humidity_ratio)
                .expect("valid zone air heat capacity");
        assert!(
            (heat_capacity - volume_m3 * expected_density * expected_specific_heat).abs() < 1.0e-9
        );
        assert!(heat_capacity < volume_m3 * 1.2 * 1006.0);
    }

    #[test]
    fn weather_context_updates_zone_air_heat_capacity_from_pressure_and_zone_humidity()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed);
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let initial_capacity = state.zones[0].air_heat_capacity_j_per_k;
        state.zones[0].air_humidity_ratio = 0.0025;

        let mut previous = weather_record_with_precipitation(0.0);
        previous.dry_bulb_c = 10.0;
        previous.relative_humidity_percent = 40.0;
        previous.atmospheric_pressure_pa = 80_000.0;
        let mut current = weather_record_with_precipitation(0.0);
        current.dry_bulb_c = 22.0;
        current.relative_humidity_percent = 80.0;
        current.atmospheric_pressure_pa = 84_000.0;
        let records = [previous, current];
        let context = HeatBalanceWeatherContext {
            records: &records,
            record_index: 1,
            zone_steps_per_hour: 4,
            zone_timestep: Some(2),
            first_hour_interpolation_starting_values: FirstHourInterpolationStartingValues::Hour24,
        };

        update_zone_air_heat_capacities_from_weather_context(
            &mut state.zones,
            Some(context),
            current.dry_bulb_c,
        );

        let expected_capacity = energyplus_zone_air_heat_capacity_j_per_k(
            state.zones[0].volume_m3,
            82_000.0,
            20.0,
            0.0025,
        )
        .expect("valid expected capacity");
        assert!((state.zones[0].air_heat_capacity_j_per_k - expected_capacity).abs() < 1.0e-9);
        assert!(state.zones[0].air_heat_capacity_j_per_k < initial_capacity);

        Ok(())
    }

    #[test]
    fn energyplus_outdoor_wet_bulb_uses_psychrometric_formula() {
        let wet_bulb_c = energyplus_outdoor_wet_bulb_c(3.0, 68.0, 82_800.0)
            .expect("valid psychrometric wet-bulb");

        assert!(
            (wet_bulb_c - 0.648_294_941_184).abs() < 1.0e-7,
            "wet_bulb_c={wet_bulb_c}"
        );
    }

    #[test]
    fn energyplus_outdoor_wet_bulb_uses_energyplus_iterate_branch_near_freezing() {
        let wet_bulb_c = energyplus_outdoor_wet_bulb_c(8.0, 20.0, 81_500.0)
            .expect("valid psychrometric wet-bulb");

        assert!(
            (wet_bulb_c - 0.227_141_685_581).abs() < 2.0e-9,
            "wet_bulb_c={wet_bulb_c}"
        );
    }

    #[test]
    fn exterior_report_terms_use_energyplus_wet_surface_rain_override()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_state = state
            .surfaces
            .iter_mut()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof test surface"))?;
        surface_state.outside_face_temperature_c = 10.0;

        let records = [weather_record_with_precipitation(1.0)];
        let reference_temperature_c = energyplus_outdoor_wet_bulb_c(
            records[0].dry_bulb_c,
            records[0].relative_humidity_percent,
            records[0].atmospheric_pressure_pa,
        )
        .unwrap_or(8.0);
        let typed_roof = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing typed roof test surface"))?;
        let expected_reference_temperature_c =
            energyplus_surface_outdoor_air_temperature_c(typed_roof, reference_temperature_c);

        let terms = surface_exterior_report_terms(
            &typed,
            surface_state,
            8.0,
            10.0,
            Some(HeatBalanceWeatherContext {
                records: &records,
                record_index: 0,
                zone_steps_per_hour: 4,
                zone_timestep: None,
                first_hour_interpolation_starting_values:
                    FirstHourInterpolationStartingValues::Hour24,
            }),
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
        );

        assert_eq!(
            terms.convection_coefficient_w_per_m2_k,
            ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K
        );
        assert!(
            expected_reference_temperature_c < 8.0,
            "rain path should use wet-bulb reference below dry-bulb"
        );
        assert!(
            (terms.convection_heat_gain_rate_per_area_w_per_m2
                - -ENERGYPLUS_HIGH_CONVECTION_LIMIT_W_PER_M2_K
                    * (10.0 - expected_reference_temperature_c))
                .abs()
                < 1.0e-9
        );

        Ok(())
    }

    #[test]
    fn exterior_longwave_terms_use_energyplus_sky_air_ground_split()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_state = state
            .surfaces
            .iter_mut()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing roof test surface"))?;
        surface_state.outside_face_temperature_c = 60.0;
        let typed_surface = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing typed roof test surface"))?;
        let record = EpwRecord {
            dry_bulb_c: 24.0,
            horizontal_infrared_radiation_wh_per_m2: 358.0,
            wind_speed_m_per_s: 4.6,
            wind_direction_deg: 310.0,
            ..weather_record_with_precipitation(0.0)
        };
        let tilt_rad =
            surface_tilt_deg(typed_surface.surface_type, &typed_surface.vertices).to_radians();

        let terms = energyplus_exterior_longwave_terms(
            surface_state,
            typed_surface,
            record.horizontal_infrared_radiation_wh_per_m2,
            60.0,
            24.0,
            24.0,
            tilt_rad,
        );
        let expected_sky_temperature_c = horizontal_infrared_sky_temperature_c(
            record.horizontal_infrared_radiation_wh_per_m2,
            24.0,
        );
        let expected_sky_coefficient = energyplus_linearized_radiation_coefficient_w_per_m2_k(
            0.9,
            60.0 + KELVIN_OFFSET,
            expected_sky_temperature_c + KELVIN_OFFSET,
        );
        let expected_gain = -expected_sky_coefficient * (60.0 - expected_sky_temperature_c);

        assert!((terms.sky_coefficient_w_per_m2_k - expected_sky_coefficient).abs() < 1.0e-12);
        assert!(terms.air_coefficient_w_per_m2_k.abs() < 1.0e-12);
        assert!(terms.ground_coefficient_w_per_m2_k.abs() < 1.0e-12);
        assert!((terms.net_heat_gain_per_area_w_per_m2(60.0) - expected_gain).abs() < 1.0e-12);

        Ok(())
    }

    #[test]
    fn exterior_longwave_air_component_uses_air_reference_temperature()
    -> Result<(), Box<dyn std::error::Error>> {
        let typed = cube_model();
        let model = SimulationModel::from_typed(typed.clone());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface_state = state
            .surfaces
            .iter_mut()
            .find(|surface| surface.surface_name == "WALL Y0")
            .ok_or_else(|| std::io::Error::other("missing wall test surface"))?;
        surface_state.outside_face_temperature_c = 30.0;
        let typed_surface = typed
            .surfaces
            .iter()
            .find(|surface| surface.name.0 == "WALL Y0")
            .ok_or_else(|| std::io::Error::other("missing typed wall test surface"))?;
        let tilt_rad =
            surface_tilt_deg(typed_surface.surface_type, &typed_surface.vertices).to_radians();

        let terms = energyplus_exterior_longwave_terms(
            surface_state,
            typed_surface,
            360.0,
            30.0,
            10.0,
            20.0,
            tilt_rad,
        );
        let air_split = surface_air_sky_radiation_split(tilt_rad);
        let expected_air_coefficient = energyplus_linearized_radiation_coefficient_w_per_m2_k(
            surface_state.thermal_absorptance
                * surface_sky_view_factor(typed_surface, tilt_rad)
                * (1.0 - air_split),
            30.0 + KELVIN_OFFSET,
            10.0 + KELVIN_OFFSET,
        );

        assert!((terms.air_coefficient_w_per_m2_k - expected_air_coefficient).abs() < 1.0e-12);
        assert_eq!(terms.air_temperature_c, 10.0);
        assert_eq!(terms.ground_temperature_c, 20.0);

        Ok(())
    }

    #[test]
    fn heat_balance_warmup_minimum_override_preserves_disabled_boundary() {
        let disabled = HeatBalanceSimulationOptions::hourly_samples(3).with_warmup_minimum_days(20);
        assert!(!disabled.warmup.enabled);
        assert_eq!(disabled.warmup.minimum_days, 0);

        let mut enabled = HeatBalanceSimulationOptions::hourly_samples(3);
        enabled.warmup = HeatBalanceWarmupOptions {
            enabled: true,
            minimum_days: 6,
            maximum_days: 10,
            temperature_convergence_tolerance_delta_c: 0.1,
        };
        let overridden = enabled.with_warmup_minimum_days(20);
        assert_eq!(overridden.warmup.minimum_days, 20);
        assert_eq!(overridden.warmup.maximum_days, 20);
    }

    #[test]
    fn heat_balance_warmup_uses_weather_context_for_exterior_forcing()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.timestep = TimestepConfig {
            number_of_timesteps_per_hour: 1,
        };
        typed.site = Some(SiteLocation {
            name: NormalizedName::new("Golden"),
            latitude_deg: 39.75,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        });
        let model = SimulationModel::from_typed(typed.clone());
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
2013,6,21,12,0,Source,25.0,5.0,30,82000,0,0,300,900,800,100,0,0,0,0,180,2.5
2013,6,21,13,0,Source,26.0,5.0,30,82000,0,0,300,920,820,100,0,0,0,0,180,2.5
"#,
        )?;
        let weather_dry_bulb_c = records
            .iter()
            .map(|record| record.dry_bulb_c)
            .collect::<Vec<_>>();
        let options = HeatBalanceWarmupOptions {
            enabled: true,
            minimum_days: 1,
            maximum_days: 1,
            temperature_convergence_tolerance_delta_c: 0.0,
        };
        let mut dry_only_state = initialize_heat_balance_state(&model, 20.0)?;
        let mut weather_context_state = initialize_heat_balance_state(&model, 20.0)?;
        let mut dry_only_warmup_day_end_states = Vec::new();
        let mut weather_context_warmup_day_end_states = Vec::new();

        let dry_only_summary = run_heat_balance_run_period_warmup(
            &typed,
            &mut dry_only_state,
            &weather_dry_bulb_c,
            None,
            1,
            SECONDS_PER_HOUR,
            options,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            1,
            None,
            HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
            FirstHourInterpolationStartingValues::Hour24,
            &mut dry_only_warmup_day_end_states,
        );
        let weather_context_summary = run_heat_balance_run_period_warmup(
            &typed,
            &mut weather_context_state,
            &weather_dry_bulb_c,
            Some(&records),
            1,
            SECONDS_PER_HOUR,
            options,
            HeatBalanceZoneAirAlgorithm::SimplifiedAnalytical,
            1,
            None,
            HeatBalanceSurfaceLoopZoneAirCorrection::EachSurfaceIteration,
            FirstHourInterpolationStartingValues::Hour24,
            &mut weather_context_warmup_day_end_states,
        );

        assert_eq!(dry_only_summary.day_count, 1);
        assert_eq!(weather_context_summary.day_count, 1);
        assert_eq!(dry_only_warmup_day_end_states.len(), 1);
        assert_eq!(weather_context_warmup_day_end_states.len(), 1);
        let dry_only_roof = dry_only_state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing dry-only roof"))?;
        let weather_context_roof = weather_context_state
            .surfaces
            .iter()
            .find(|surface| surface.surface_name == "ROOF")
            .ok_or_else(|| std::io::Error::other("missing weather-context roof"))?;

        assert!(
            weather_context_roof.outside_face_temperature_c
                > dry_only_roof.outside_face_temperature_c + 1.0
        );

        Ok(())
    }

    #[test]
    fn heat_balance_third_order_probe_runs_as_diagnostic_option()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderProbe),
        )?;

        assert_eq!(simulation.summary.samples, 2);
        assert_eq!(simulation.summary.timestep_count, 12);
        let Some(zone_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing zone series").into());
        };
        assert_eq!(zone_series.values.len(), 2);
        assert!(zone_series.values.iter().all(|value| value.is_finite()));
        assert_eq!(
            simulation.summary.warmup,
            HeatBalanceWarmupSummary::disabled()
        );

        Ok(())
    }

    #[test]
    fn heat_balance_surface_first_probe_uses_distinct_zone_air_order()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let analytical = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalProbe),
        )?;
        let surface_first = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2).with_zone_air_algorithm(
                HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalSurfaceFirstProbe,
            ),
        )?;
        let coupled = simulate_heat_balance_zone_air_temperatures(
            &model,
            &[10.0, 12.0],
            HeatBalanceSimulationOptions::hourly_samples(2).with_zone_air_algorithm(
                HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe,
            ),
        )?;

        let Some(analytical_zone_series) = analytical
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing analytical zone series").into());
        };
        let Some(surface_first_zone_series) = surface_first
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing surface-first zone series").into());
        };
        let Some(coupled_zone_series) = coupled
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing coupled zone series").into());
        };

        assert_eq!(analytical_zone_series.values.len(), 2);
        assert_eq!(surface_first_zone_series.values.len(), 2);
        assert_eq!(coupled_zone_series.values.len(), 2);
        assert!(
            analytical_zone_series
                .values
                .iter()
                .chain(surface_first_zone_series.values.iter())
                .chain(coupled_zone_series.values.iter())
                .all(|value| value.is_finite())
        );
        assert!(
            (analytical_zone_series.values[0] - surface_first_zone_series.values[0]).abs() > 1.0e-6
        );
        assert!(
            (surface_first_zone_series.values[0] - coupled_zone_series.values[0]).abs() > 1.0e-6
        );

        Ok(())
    }

    #[test]
    fn surface_incident_solar_diagnostic_appends_roof_series()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.site = Some(SiteLocation {
            name: NormalizedName::new("Solar Test Site"),
            latitude_deg: 39.75,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        });
        let model = SimulationModel::from_typed(typed);
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
2013,6,21,12,0,Source,25.0,5.0,30,82000,0,0,300,900,800,100,0,0,0,0,180,2.5
2013,6,21,13,0,Source,26.0,5.0,30,82000,0,0,300,920,820,100,0,0,0,0,180,2.5
"#,
        )?;
        let weather_values = records
            .iter()
            .map(|record| record.dry_bulb_c)
            .collect::<Vec<_>>();
        let mut simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;

        let added = append_surface_incident_solar_radiation_series(
            &mut simulation.results,
            &model,
            &records,
            2,
        );

        assert_eq!(added, 20);
        assert!(
            simulation
                .results
                .find_series(
                    "FLOOR",
                    "Surface Outside Face Incident Solar Radiation Rate per Area"
                )
                .is_none()
        );
        let Some(roof_solar) = simulation.results.find_series(
            "ROOF",
            "Surface Outside Face Incident Solar Radiation Rate per Area",
        ) else {
            return Err(std::io::Error::other("missing roof solar series").into());
        };
        assert_eq!(roof_solar.units, "W/m2");
        assert_eq!(roof_solar.values.len(), 2);
        assert!(roof_solar.values[0].is_finite());
        assert!(roof_solar.values[0] > 600.0);
        for variable in [
            "Surface Outside Face Incident Beam Solar Radiation Rate per Area",
            "Surface Outside Face Incident Sky Diffuse Solar Radiation Rate per Area",
            "Surface Outside Face Incident Ground Diffuse Solar Radiation Rate per Area",
        ] {
            let Some(series) = simulation.results.find_series("ROOF", variable) else {
                return Err(
                    std::io::Error::other(format!("missing roof {variable} series")).into(),
                );
            };
            assert_eq!(series.units, "W/m2");
            assert_eq!(series.values.len(), 2);
            assert!(series.values[0].is_finite());
        }

        Ok(())
    }

    #[test]
    fn weather_record_exterior_balance_forces_exterior_conduction()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.site = Some(SiteLocation {
            name: NormalizedName::new("Solar Test Site"),
            latitude_deg: 39.75,
            longitude_deg: -105.18,
            time_zone_hours: -7.0,
            elevation_m: 1829.0,
        });
        let model = SimulationModel::from_typed(typed);
        let records = parse_epw_records(
            r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
2013,6,21,12,0,Source,25.0,5.0,30,82000,0,0,300,900,800,100,0,0,0,0,180,2.5
2013,6,21,13,0,Source,26.0,5.0,30,82000,0,0,300,920,820,100,0,0,0,0,180,2.5
"#,
        )?;
        let weather_values = records
            .iter()
            .map(|record| record.dry_bulb_c)
            .collect::<Vec<_>>();
        let dry_bulb_only = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;
        let weather_forced = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2),
        )?;
        let coupled = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let previous_inside = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let previous_inside_doe2 = simulate_heat_balance_zone_air_temperatures_with_weather_records(
            &model,
            &records,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideDoe2Probe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(dry_roof_conduction) = dry_bulb_only
            .results
            .find_series("ROOF", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing dry roof conduction series").into());
        };
        let Some(forced_roof_conduction) = weather_forced
            .results
            .find_series("ROOF", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing forced roof conduction series").into());
        };
        let Some(dry_wall_conduction) = dry_bulb_only.results.find_series(
            "WALL Y0",
            "Surface Inside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing dry wall conduction series").into());
        };
        let Some(forced_wall_conduction) = weather_forced.results.find_series(
            "WALL Y0",
            "Surface Inside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing forced wall conduction series").into());
        };
        let Some(coupled_roof_temperature) = coupled
            .results
            .find_series("ROOF", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing coupled roof temperature series").into());
        };
        let Some(previous_inside_roof_temperature) = previous_inside
            .results
            .find_series("ROOF", "Surface Outside Face Temperature")
        else {
            return Err(
                std::io::Error::other("missing previous-inside roof temperature series").into(),
            );
        };
        let Some(previous_inside_doe2_roof_temperature) = previous_inside_doe2
            .results
            .find_series("ROOF", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other(
                "missing previous-inside DOE-2 roof temperature series",
            )
            .into());
        };

        assert_eq!(dry_roof_conduction.values.len(), 2);
        assert_eq!(forced_roof_conduction.values.len(), 2);
        assert_eq!(dry_wall_conduction.values.len(), 2);
        assert_eq!(forced_wall_conduction.values.len(), 2);
        assert_eq!(coupled_roof_temperature.values.len(), 2);
        assert_eq!(previous_inside_roof_temperature.values.len(), 2);
        assert_eq!(previous_inside_doe2_roof_temperature.values.len(), 2);
        assert!((dry_roof_conduction.values[0] - forced_roof_conduction.values[0]).abs() > 1.0e-3);
        assert!((dry_wall_conduction.values[0] - forced_wall_conduction.values[0]).abs() > 1.0e-3);
        assert!(
            (coupled_roof_temperature.values[0] - previous_inside_roof_temperature.values[0]).abs()
                > 1.0e-6
        );
        assert!(
            (previous_inside_doe2_roof_temperature.values[0]
                - previous_inside_roof_temperature.values[0])
                .abs()
                > 1.0e-6
        );

        Ok(())
    }

    #[test]
    fn previous_boundary_probe_keeps_adiabatic_outside_face_history()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![10.0, 12.0];

        let coupled = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let previous_boundary = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousBoundaryProbe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(coupled_floor_outside_temperature) = coupled
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing coupled floor outside temperature").into());
        };
        let Some(coupled_floor_inside_temperature) = coupled
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing coupled floor inside temperature").into());
        };
        let Some(previous_boundary_floor_outside_temperature) = previous_boundary
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other(
                "missing previous-boundary floor outside temperature",
            )
            .into());
        };

        assert_eq!(coupled_floor_outside_temperature.values.len(), 2);
        assert_eq!(previous_boundary_floor_outside_temperature.values.len(), 2);
        assert_eq!(
            coupled_floor_outside_temperature.values[0],
            coupled_floor_inside_temperature.values[0]
        );
        assert!(
            (coupled_floor_outside_temperature.values[0]
                - previous_boundary_floor_outside_temperature.values[0])
                .abs()
                > 1.0e-6
        );

        Ok(())
    }

    #[test]
    fn interleaved_longwave_probe_freezes_adiabatic_outside_ctf_report_state()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![10.0, 12.0];

        let simulation = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusAnalyticalCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveProbe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(floor_inside_conduction) = simulation
            .results
            .find_series("FLOOR", "Surface Inside Face Conduction Heat Transfer Rate")
        else {
            return Err(std::io::Error::other("missing floor inside conduction").into());
        };
        let Some(floor_outside_conduction) = simulation.results.find_series(
            "FLOOR",
            "Surface Outside Face Conduction Heat Transfer Rate",
        ) else {
            return Err(std::io::Error::other("missing floor outside conduction").into());
        };
        let Some(floor_storage) = simulation
            .results
            .find_series("FLOOR", "Surface Heat Storage Rate")
        else {
            return Err(std::io::Error::other("missing floor heat storage").into());
        };
        let Some(floor_storage_per_area) = simulation
            .results
            .find_series("FLOOR", "Surface Heat Storage Rate per Area")
        else {
            return Err(std::io::Error::other("missing floor heat storage per-area").into());
        };

        assert_eq!(floor_inside_conduction.values.len(), 2);
        assert_eq!(floor_outside_conduction.values.len(), 2);
        assert!(
            (floor_inside_conduction.values[0] - floor_outside_conduction.values[0]).abs() > 1.0e-6
        );
        assert!(
            (floor_storage.values[0]
                + floor_inside_conduction.values[0]
                + floor_outside_conduction.values[0])
                .abs()
                < 1.0e-9
        );
        assert!(
            (floor_storage_per_area.values[0] - floor_storage.values[0] / 100.0).abs() < 1.0e-9
        );

        Ok(())
    }

    #[test]
    fn frozen_reference_air_probe_changes_interleaved_surface_reference_air()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![5.0, 35.0];

        let active = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let frozen_reference_air = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirProbe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(active_floor_inside_temperature) = active
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing active floor inside temperature").into());
        };
        let Some(frozen_floor_inside_temperature) = frozen_reference_air
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing frozen floor inside temperature").into());
        };
        let Some(active_zone_temperature) = active
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing active zone temperature").into());
        };
        let Some(frozen_zone_temperature) = frozen_reference_air
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing frozen zone temperature").into());
        };

        assert_eq!(active_floor_inside_temperature.values.len(), 2);
        assert_eq!(frozen_floor_inside_temperature.values.len(), 2);
        assert!(
            active_floor_inside_temperature
                .values
                .iter()
                .zip(&frozen_floor_inside_temperature.values)
                .any(|(active, frozen)| (active - frozen).abs() > 1.0e-9)
        );
        assert!(
            active_zone_temperature
                .values
                .iter()
                .zip(&frozen_zone_temperature.values)
                .any(|(active, frozen)| (active - frozen).abs() > 1.0e-9)
        );

        Ok(())
    }

    #[test]
    fn converged_surface_probe_changes_fixed_iteration_cap()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![5.0, 35.0];

        let fixed_iterations = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveProbe,
                )
                .with_surface_iteration_count(20),
        )?;
        let converged_iterations = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionFrozenReferenceAirCurrentLongwaveConvergedSurfaceProbe,
                )
                .with_surface_iteration_count(20),
        )?;

        let Some(fixed_floor_temperature) = fixed_iterations
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing fixed floor temperature").into());
        };
        let Some(converged_floor_temperature) = converged_iterations
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing converged floor temperature").into());
        };

        assert_eq!(fixed_floor_temperature.values.len(), 2);
        assert_eq!(converged_floor_temperature.values.len(), 2);
        assert!(
            fixed_floor_temperature
                .values
                .iter()
                .zip(&converged_floor_temperature.values)
                .any(|(fixed, converged)| (fixed - converged).abs() > 1.0e-9)
        );

        Ok(())
    }

    #[test]
    fn current_adiabatic_history_probe_syncs_adiabatic_outside_face_after_solve()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut typed = cube_model();
        typed.surfaces[0].outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        typed.surfaces[0].wind_exposure = WindExposure::NoWind;
        let model = SimulationModel::from_typed(typed);
        let weather_values = vec![10.0, 12.0];

        let active = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionProbe,
                )
                .with_surface_iteration_count(3),
        )?;
        let current_history = simulate_heat_balance_zone_air_temperatures(
            &model,
            &weather_values,
            HeatBalanceSimulationOptions::hourly_samples(2)
                .with_zone_air_algorithm(
                    HeatBalanceZoneAirAlgorithm::EnergyPlusThirdOrderCoupledPreviousInsideQuickOutsideInterleavedInteriorLongwaveFrozenHconvWeatherAirStorageBalanceSurfaceConvectionCurrentAdiabaticHistoryProbe,
                )
                .with_surface_iteration_count(3),
        )?;

        let Some(active_inside_temperature) = active
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing active floor inside temperature").into());
        };
        let Some(active_outside_temperature) = active
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing active floor outside temperature").into());
        };
        let Some(current_inside_temperature) = current_history
            .results
            .find_series("FLOOR", "Surface Inside Face Temperature")
        else {
            return Err(std::io::Error::other("missing current floor inside temperature").into());
        };
        let Some(current_outside_temperature) = current_history
            .results
            .find_series("FLOOR", "Surface Outside Face Temperature")
        else {
            return Err(std::io::Error::other("missing current floor outside temperature").into());
        };

        assert_eq!(current_inside_temperature.values.len(), 2);
        assert_eq!(current_outside_temperature.values.len(), 2);
        assert!(
            (active_inside_temperature.values[0] - active_outside_temperature.values[0]).abs()
                > 1.0e-6
        );
        assert!(
            (current_inside_temperature.values[0] - current_outside_temperature.values[0]).abs()
                < 1.0e-9
        );

        Ok(())
    }

    #[test]
    fn adiabatic_history_commit_override_preserves_report_face_and_uses_inside_for_ctf_history()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        surface.outside_face_temperature_c = 10.0;
        surface.inside_face_temperature_c = 20.0;
        surface.ctf = SurfaceCtfState {
            outside_0_w_per_m2_k: 4.0,
            cross_0_w_per_m2_k: 2.0,
            inside_0_w_per_m2_k: 3.0,
            const_in_part_w_per_m2: 1.0,
            const_out_part_w_per_m2: 5.0,
            outside_history_w_per_m2_k: vec![0.1, 0.2],
            cross_history_w_per_m2_k: vec![0.3, 0.4],
            inside_history_w_per_m2_k: vec![0.5, 0.6],
            flux_history: vec![0.7, 0.8],
            outside_temperature_history_c: vec![7.0, 8.0],
            inside_temperature_history_c: vec![17.0, 18.0],
            outside_flux_history_w_per_m2: vec![70.0, 80.0],
            inside_flux_history_w_per_m2: vec![170.0, 180.0],
        };

        advance_surface_ctf_histories_with_outside_temperature_override(surface, Some(20.0));

        assert_eq!(surface.outside_face_temperature_c, 10.0);
        assert_eq!(surface.ctf.outside_temperature_history_c, vec![20.0, 7.0]);
        assert_eq!(surface.ctf.inside_temperature_history_c, vec![20.0, 17.0]);
        assert_eq!(surface.ctf.inside_flux_history_w_per_m2, vec![-19.0, 170.0]);
        assert_eq!(surface.ctf.outside_flux_history_w_per_m2, vec![45.0, 70.0]);

        Ok(())
    }

    #[test]
    fn inside_ctf_outside_history_commit_override_only_uses_outdoor_snapshots()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());
        let mut state = initialize_heat_balance_state(&model, 20.0)?;
        let surface = &mut state.surfaces[0];
        let surface_id = surface.surface_id;
        surface.outside_boundary_condition = OutsideBoundaryCondition::Outdoors;

        let mut snapshots = BTreeMap::new();
        snapshots.insert(surface_id, 12.5);

        assert_eq!(
            inside_ctf_outside_temperature_history_commit_override_c(
                surface,
                true,
                Some(&snapshots)
            ),
            Some(12.5)
        );
        assert_eq!(
            inside_ctf_outside_temperature_history_commit_override_c(
                surface,
                false,
                Some(&snapshots)
            ),
            None
        );

        snapshots.clear();
        assert_eq!(
            inside_ctf_outside_temperature_history_commit_override_c(
                surface,
                true,
                Some(&snapshots)
            ),
            None
        );

        surface.outside_boundary_condition = OutsideBoundaryCondition::Adiabatic;
        snapshots.insert(surface_id, 15.0);
        assert_eq!(
            inside_ctf_outside_temperature_history_commit_override_c(
                surface,
                true,
                Some(&snapshots)
            ),
            None
        );

        Ok(())
    }

    #[test]
    fn result_store_finds_series_case_insensitively() {
        let mut store = ResultStore::new();
        store.add_series(OutputSeries {
            handle: OutputHandle(0),
            key: "ZONE ONE".to_string(),
            variable_name: "Zone Mean Air Temperature".to_string(),
            units: "C".to_string(),
            values: vec![20.0, 21.0],
        });

        assert_eq!(store.sample_count(), 2);
        assert!(
            store
                .find_series("zone one", "zone mean air temperature")
                .is_some()
        );
    }

    #[test]
    fn runtime_output_registry_resolves_declared_model_outputs() {
        let model = SimulationModel::from_typed(cube_model());
        let registry = RuntimeOutputRegistry::from_model(&model);

        assert_eq!(registry.len(), 157);
        assert!(registry.meter_registry().is_empty());

        let resolution = registry.resolve_output_requests(&[
            RuntimeOutputRequest::hourly("zone one", "Zone Mean Air Temperature"),
            RuntimeOutputRequest::hourly("floor", "Surface Inside Face Temperature"),
            RuntimeOutputRequest::hourly("floor", "Surface Inside Face Adjacent Air Temperature"),
            RuntimeOutputRequest::hourly(
                "floor",
                "Surface Inside Face Conduction Heat Transfer Rate",
            ),
            RuntimeOutputRequest::hourly(
                "zone one",
                "Zone Opaque Surface Inside Faces Conduction Rate",
            ),
            RuntimeOutputRequest::hourly(
                "zone one",
                "Zone Opaque Surface Outside Faces Conduction Rate",
            ),
            RuntimeOutputRequest::hourly("floor", "Surface Heat Storage Rate"),
            RuntimeOutputRequest::hourly("floor", "Surface Heat Storage Rate per Area"),
            RuntimeOutputRequest::hourly(
                "floor",
                "Surface Outside Face Incident Solar Radiation Rate per Area",
            ),
            RuntimeOutputRequest::hourly(
                "floor",
                "Surface Outside Face Convection Heat Transfer Coefficient",
            ),
            RuntimeOutputRequest::hourly("environment", "Site Outdoor Air Drybulb Temperature"),
            RuntimeOutputRequest::hourly("environment", "Site Outdoor Air Wetbulb Temperature"),
            RuntimeOutputRequest::hourly("environment", "Site Rain Status"),
        ]);

        assert!(resolution.diagnostics.is_empty());
        assert_eq!(resolution.resolved.len(), 13);
        assert_eq!(resolution.resolved[0].definition.handle, OutputHandle(0));
        assert_eq!(resolution.resolved[1].definition.key, "FLOOR");
    }

    #[test]
    fn runtime_output_registry_skips_no_sun_surface_solar_output() {
        let mut typed = cube_model();
        typed.surfaces[0].sun_exposure = SunExposure::NoSun;
        let model = SimulationModel::from_typed(typed);
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry.resolve_output_requests(&[RuntimeOutputRequest::hourly(
            "floor",
            "Surface Outside Face Incident Solar Radiation Rate per Area",
        )]);

        assert!(resolution.resolved.is_empty());
        assert!(resolution.diagnostics.has_errors());
        assert_eq!(
            resolution.diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::OutputVariableUnavailable
        );
    }

    #[test]
    fn runtime_output_registry_diagnoses_unavailable_output() {
        let model = SimulationModel::from_typed(cube_model());
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry.resolve_output_requests(&[RuntimeOutputRequest::hourly(
            "ZONE ONE",
            "Zone Lights Electricity Energy",
        )]);

        assert!(resolution.resolved.is_empty());
        assert!(resolution.diagnostics.has_errors());
        assert_eq!(
            resolution.diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::OutputVariableUnavailable
        );
    }

    #[test]
    fn runtime_output_registry_diagnoses_unavailable_system_node_output() {
        let model = ideal_loads_node_state_model();
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry.resolve_output_requests(&[RuntimeOutputRequest::hourly(
            "ZONE ONE INLET",
            NODE_STATE_EXCLUDED_SETPOINT_VARIABLE,
        )]);

        assert!(resolution.resolved.is_empty());
        assert!(resolution.diagnostics.has_errors());
        let diagnostic = &resolution.diagnostics.diagnostics[0];
        assert_eq!(
            diagnostic.code,
            RuntimeDiagnosticCode::OutputVariableUnavailable
        );
        assert_eq!(diagnostic.key.as_deref(), Some("ZONE ONE INLET"));
        assert_eq!(
            diagnostic.variable_name.as_deref(),
            Some(NODE_STATE_EXCLUDED_SETPOINT_VARIABLE)
        );
    }

    #[test]
    fn runtime_meter_registry_diagnoses_unavailable_meter() {
        let model = SimulationModel::from_typed(cube_model());
        let registry = RuntimeOutputRegistry::from_model(&model);

        let resolution = registry
            .meter_registry()
            .resolve_meter_requests(&[RuntimeMeterRequest::hourly("Electricity:Facility")]);

        assert!(resolution.resolved.is_empty());
        assert!(resolution.diagnostics.has_errors());
        assert_eq!(
            resolution.diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::MeterUnavailable
        );
    }

    #[test]
    fn runtime_meter_registry_resolves_ideal_loads_facility_meters() {
        let model = ideal_loads_node_state_model();
        let registry = RuntimeOutputRegistry::from_model(&model);
        let heating_binding =
            crate::ideal_loads_facility_meter_binding(IdealLoadsFuelType::DistrictHeatingWater)
                .expect("district heating meter binding");
        let cooling_binding =
            crate::ideal_loads_facility_meter_binding(IdealLoadsFuelType::DistrictCooling)
                .expect("district cooling meter binding");

        let resolution = registry.meter_registry().resolve_meter_requests(&[
            RuntimeMeterRequest::hourly(heating_binding.meter_name),
            RuntimeMeterRequest::hourly(cooling_binding.meter_name),
            RuntimeMeterRequest::new(heating_binding.meter_name, RuntimeOutputFrequency::Monthly),
            RuntimeMeterRequest::new(heating_binding.meter_name, RuntimeOutputFrequency::Annual),
            RuntimeMeterRequest::new(
                cooling_binding.meter_name,
                RuntimeOutputFrequency::RunPeriod,
            ),
        ]);

        assert_eq!(registry.meter_registry().len(), 8);
        assert_eq!(resolution.resolved.len(), 5);
        assert!(resolution.diagnostics.is_empty());
        assert_eq!(
            resolution.resolved[0].definition.name,
            "DistrictHeatingWater:Facility"
        );
        assert_eq!(
            heating_binding.fuel_energy_variable,
            crate::ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_HEATING_FUEL_ENERGY
        );
        assert_eq!(resolution.resolved[0].definition.units, "J");
        assert_eq!(
            resolution.resolved[1].definition.name,
            "DistrictCooling:Facility"
        );
        assert_eq!(
            cooling_binding.fuel_energy_variable,
            crate::ZONE_IDEAL_LOADS_SUPPLY_AIR_TOTAL_COOLING_FUEL_ENERGY
        );
        assert_eq!(resolution.resolved[1].definition.units, "J");
        assert_eq!(
            resolution.resolved[2].definition.frequency,
            RuntimeOutputFrequency::Monthly
        );
        assert_eq!(
            resolution.resolved[3].definition.frequency,
            RuntimeOutputFrequency::Annual
        );
        assert_eq!(
            resolution.resolved[4].definition.frequency,
            RuntimeOutputFrequency::RunPeriod
        );
    }

    #[test]
    fn result_store_diagnostics_report_duplicate_handles() {
        let mut store = ResultStore::new();
        store.add_series(OutputSeries {
            handle: OutputHandle(0),
            key: "ZONE ONE".to_string(),
            variable_name: "Zone Mean Air Temperature".to_string(),
            units: "C".to_string(),
            values: vec![20.0],
        });
        store.add_series(OutputSeries {
            handle: OutputHandle(0),
            key: "Environment".to_string(),
            variable_name: "Site Outdoor Air Drybulb Temperature".to_string(),
            units: "C".to_string(),
            values: vec![10.0],
        });

        let diagnostics = store.diagnostics();

        assert!(diagnostics.has_errors());
        assert_eq!(
            diagnostics.diagnostics[0].code,
            RuntimeDiagnosticCode::DuplicateOutputHandle
        );
        assert_eq!(store.profile().series_count, 2);
    }

    #[test]
    fn result_store_diagnostics_report_duplicate_system_node_handles() {
        let mut store = ResultStore::new();
        store.add_series(OutputSeries {
            handle: OutputHandle(7),
            key: "ZONE ONE INLET".to_string(),
            variable_name: "System Node Temperature".to_string(),
            units: "C".to_string(),
            values: vec![50.0],
        });
        store.add_series(OutputSeries {
            handle: OutputHandle(7),
            key: "ZONE ONE INLET".to_string(),
            variable_name: "System Node Humidity Ratio".to_string(),
            units: "kgWater/kgDryAir".to_string(),
            values: vec![0.0156],
        });

        let diagnostics = store.diagnostics();

        assert!(diagnostics.has_errors());
        let diagnostic = &diagnostics.diagnostics[0];
        assert_eq!(
            diagnostic.code,
            RuntimeDiagnosticCode::DuplicateOutputHandle
        );
        assert_eq!(diagnostic.key.as_deref(), Some("ZONE ONE INLET"));
        assert_eq!(
            diagnostic.variable_name.as_deref(),
            Some("System Node Humidity Ratio")
        );
        assert_eq!(diagnostic.handle, Some(OutputHandle(7)));
    }

    #[test]
    fn first_zone_simulation_writes_zone_temperature_results()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = SimulationModel::from_typed(cube_model());

        let simulation = simulate_first_zone_uncontrolled(
            &model,
            &[20.0, 20.0],
            FirstZoneSimulationOptions::hourly_samples(2),
        )?;

        assert_eq!(simulation.summary.zone_name, "ZONE ONE");
        assert_eq!(simulation.summary.samples, 2);
        assert_eq!(simulation.summary.volume_m3, 1.0);
        assert_eq!(simulation.summary.exterior_area_m2, 6.0);
        assert_eq!(simulation.summary.conductance_w_per_k, 6.0);
        assert_eq!(simulation.summary.internal_gain_w, 12.0);
        assert_eq!(simulation.results.sample_count(), 2);
        assert_eq!(simulation.results.series.len(), 2);
        assert_eq!(simulation.state.timestep_index, 12);
        let Some(zone_series) = simulation
            .results
            .find_series("ZONE ONE", "Zone Mean Air Temperature")
        else {
            return Err(std::io::Error::other("missing zone series").into());
        };
        assert!(zone_series.values[0] > 20.0);
        assert!(zone_series.values[1] >= zone_series.values[0]);

        Ok(())
    }

    fn cube_model() -> TypedModel {
        let mut model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 6,
            },
            ..TypedModel::default()
        };
        model.materials.push(Material {
            id: MaterialId(0),
            name: NormalizedName::new("R1"),
            kind: MaterialKind::NoMass,
            roughness: Some(MaterialSurfaceRoughness::Rough),
            conductivity_w_per_m_k: None,
            density_kg_per_m3: None,
            specific_heat_j_per_kg_k: None,
            thickness_m: None,
            thermal_resistance_m2_k_per_w: Some(1.0),
            thermal_absorptance: Some(0.9),
            solar_absorptance: Some(0.75),
            visible_absorptance: Some(0.75),
        });
        model.constructions.push(Construction {
            id: ConstructionId(0),
            name: NormalizedName::new("Wall"),
            outside_layer: MaterialId(0),
            layers: vec![MaterialId(0)],
        });
        model.schedules.push(ScheduleConstant {
            id: ScheduleId(0),
            name: NormalizedName::new("Always On"),
            schedule_type_limits: None,
            hourly_value: 1.0,
        });
        model.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone One"),
            direction_of_relative_north_deg: 0.0,
            origin: Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::AutoCalculate,
        });
        model.other_equipment.push(OtherEquipment {
            id: InternalGainId(0),
            name: NormalizedName::new("Plug Load"),
            zone: ZoneId(0),
            schedule: Some(ScheduleId(0)),
            design_level_w: 12.0,
            fraction_latent: 0.0,
            fraction_radiant: 0.0,
            fraction_lost: 0.0,
        });
        model.surfaces.extend(cube_surfaces());
        model
    }

    fn two_zone_interzone_model() -> TypedModel {
        let mut model = TypedModel {
            timestep: TimestepConfig {
                number_of_timesteps_per_hour: 1,
            },
            ..TypedModel::default()
        };
        model.materials.push(Material {
            id: MaterialId(0),
            name: NormalizedName::new("R1"),
            kind: MaterialKind::NoMass,
            roughness: Some(MaterialSurfaceRoughness::Rough),
            conductivity_w_per_m_k: None,
            density_kg_per_m3: None,
            specific_heat_j_per_kg_k: None,
            thickness_m: None,
            thermal_resistance_m2_k_per_w: Some(1.0),
            thermal_absorptance: Some(0.9),
            solar_absorptance: Some(0.75),
            visible_absorptance: Some(0.75),
        });
        model.constructions.push(Construction {
            id: ConstructionId(0),
            name: NormalizedName::new("Wall"),
            outside_layer: MaterialId(0),
            layers: vec![MaterialId(0)],
        });
        model.zones.push(Zone {
            id: ZoneId(0),
            name: NormalizedName::new("Zone A"),
            direction_of_relative_north_deg: 0.0,
            origin: point(0.0, 0.0, 0.0),
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::Value(1.0),
        });
        model.zones.push(Zone {
            id: ZoneId(1),
            name: NormalizedName::new("Zone B"),
            direction_of_relative_north_deg: 0.0,
            origin: point(1.0, 0.0, 0.0),
            zone_type: 1,
            multiplier: 1,
            ceiling_height: AutoOrNumber::AutoCalculate,
            volume: AutoOrNumber::Value(1.0),
        });
        model.surfaces.push(interzone_surface(
            0,
            "A Wall",
            ZoneId(0),
            "B Wall",
            [
                point(1.0, 0.0, 0.0),
                point(1.0, 1.0, 0.0),
                point(1.0, 1.0, 1.0),
                point(1.0, 0.0, 1.0),
            ],
        ));
        model.surfaces.push(interzone_surface(
            1,
            "B Wall",
            ZoneId(1),
            "A Wall",
            [
                point(0.0, 0.0, 0.0),
                point(0.0, 0.0, 1.0),
                point(0.0, 1.0, 1.0),
                point(0.0, 1.0, 0.0),
            ],
        ));
        model
    }

    fn cube_surfaces() -> Vec<Surface> {
        vec![
            surface(
                0,
                "Floor",
                SurfaceType::Floor,
                [
                    point(0.0, 0.0, 0.0),
                    point(1.0, 0.0, 0.0),
                    point(1.0, 1.0, 0.0),
                    point(0.0, 1.0, 0.0),
                ],
            ),
            surface(
                1,
                "Roof",
                SurfaceType::Roof,
                [
                    point(0.0, 0.0, 1.0),
                    point(0.0, 1.0, 1.0),
                    point(1.0, 1.0, 1.0),
                    point(1.0, 0.0, 1.0),
                ],
            ),
            surface(
                2,
                "Wall X0",
                SurfaceType::Wall,
                [
                    point(0.0, 0.0, 0.0),
                    point(0.0, 1.0, 0.0),
                    point(0.0, 1.0, 1.0),
                    point(0.0, 0.0, 1.0),
                ],
            ),
            surface(
                3,
                "Wall X1",
                SurfaceType::Wall,
                [
                    point(1.0, 0.0, 0.0),
                    point(1.0, 0.0, 1.0),
                    point(1.0, 1.0, 1.0),
                    point(1.0, 1.0, 0.0),
                ],
            ),
            surface(
                4,
                "Wall Y0",
                SurfaceType::Wall,
                [
                    point(0.0, 0.0, 0.0),
                    point(0.0, 0.0, 1.0),
                    point(1.0, 0.0, 1.0),
                    point(1.0, 0.0, 0.0),
                ],
            ),
            surface(
                5,
                "Wall Y1",
                SurfaceType::Wall,
                [
                    point(0.0, 1.0, 0.0),
                    point(1.0, 1.0, 0.0),
                    point(1.0, 1.0, 1.0),
                    point(0.0, 1.0, 1.0),
                ],
            ),
        ]
    }

    fn weather_record_with_precipitation(liquid_precipitation_depth_mm: f64) -> EpwRecord {
        EpwRecord {
            year: 2013,
            month: 9,
            day: 18,
            hour: 19,
            minute: 60,
            dry_bulb_c: 8.0,
            dew_point_c: 7.0,
            relative_humidity_percent: 93.0,
            atmospheric_pressure_pa: 81_800.0,
            horizontal_infrared_radiation_wh_per_m2: 330.0,
            global_horizontal_radiation_wh_per_m2: 0.0,
            direct_normal_radiation_wh_per_m2: 0.0,
            diffuse_horizontal_radiation_wh_per_m2: 0.0,
            wind_direction_deg: 0.0,
            wind_speed_m_per_s: 0.0,
            liquid_precipitation_depth_mm,
        }
    }

    fn surface(id: u32, name: &str, surface_type: SurfaceType, vertices: [Point3; 4]) -> Surface {
        Surface {
            id: SurfaceId(id),
            name: NormalizedName::new(name),
            surface_type,
            construction: ConstructionId(0),
            zone: ZoneId(0),
            outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
            outside_boundary_condition_object: None,
            sun_exposure: ep_model::SunExposure::SunExposed,
            wind_exposure: ep_model::WindExposure::WindExposed,
            view_factor_to_ground: AutoOrNumber::AutoCalculate,
            vertices: vertices.to_vec(),
        }
    }

    fn interzone_surface(
        id: u32,
        name: &str,
        zone: ZoneId,
        target_surface: &str,
        vertices: [Point3; 4],
    ) -> Surface {
        Surface {
            id: SurfaceId(id),
            name: NormalizedName::new(name),
            surface_type: SurfaceType::Wall,
            construction: ConstructionId(0),
            zone,
            outside_boundary_condition: OutsideBoundaryCondition::Surface,
            outside_boundary_condition_object: Some(NormalizedName::new(target_surface)),
            sun_exposure: ep_model::SunExposure::NoSun,
            wind_exposure: ep_model::WindExposure::NoWind,
            view_factor_to_ground: AutoOrNumber::AutoCalculate,
            vertices: vertices.to_vec(),
        }
    }

    fn point(x_m: f64, y_m: f64, z_m: f64) -> Point3 {
        Point3 { x_m, y_m, z_m }
    }
}
