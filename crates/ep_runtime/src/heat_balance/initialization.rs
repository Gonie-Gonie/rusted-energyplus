//! Heat-balance state initialization and diagnostic CTF seeding helpers.

use crate::error::RuntimeError;
use crate::geometry::{surface_area_m2, surface_azimuth_deg, surface_tilt_deg, zone_volume_m3};
use crate::heat_balance::ctf::{
    ConstructionCtfCoefficientOverride, construction_ctf_coefficients_by_name,
    steady_ctf_coefficient_w_per_m2_k, steady_surface_ctf_state,
    surface_ctf_state_from_coefficients,
};
use crate::heat_balance::inside_convection::zone_surface_convection_sums_for_indices;
use crate::heat_balance::state::{
    HeatBalanceState, HeatBalanceSurfaceIndexes, SurfaceExteriorReportTerms,
    SurfaceHeatBalanceState, SurfaceOutsideBalanceDiagnostics, ZoneAirTemperatureCoefficients,
    ZoneHeatBalanceState,
};
use crate::heat_balance::surface_boundary::resolve_surface_boundary_target;
use crate::heat_balance::zone_air_correction::ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO;
use crate::heat_balance::zone_predictor_corrector::energyplus_zone_air_temperature_coefficients;
use crate::heat_balance::{ConstructionThermalData, surface_thermal_properties};
use crate::schedules::{
    convective_internal_gain_w, update_surface_radiant_internal_gain_source_terms,
};
use ep_model::{ConstructionId, SimulationModel};
use std::collections::BTreeMap;

const AIR_DENSITY_KG_PER_M3: f64 = 1.2;
const AIR_SPECIFIC_HEAT_J_PER_KG_K: f64 = 1006.0;
const ENERGYPLUS_INITIAL_CONVECTION_COEFFICIENT_W_PER_M2_K: f64 = 3.076;
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

    let mut construction_thermal_data = BTreeMap::<ConstructionId, ConstructionThermalData>::new();
    let mut surfaces = model
        .typed
        .surfaces
        .iter()
        .map(|surface| {
            let area_m2 = surface_area_m2(&surface.vertices);
            let azimuth_deg = surface_azimuth_deg(&surface.vertices);
            let tilt_deg = surface_tilt_deg(surface.surface_type, &surface.vertices);
            let thermal = match construction_thermal_data.get(&surface.construction) {
                Some(thermal) => thermal.clone(),
                None => {
                    let thermal = surface_thermal_properties(&model.typed, surface)?;
                    construction_thermal_data.insert(surface.construction, thermal.clone());
                    thermal
                }
            };
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
    let surface_indexes = HeatBalanceSurfaceIndexes::from_model_surfaces(model, &surfaces);

    for zone in &mut zones {
        let zone_surface_indexes = surface_indexes.surfaces_for_zone(zone.zone_id);
        zone.opaque_surface_conductance_w_per_k = zone_surface_indexes
            .iter()
            .filter_map(|surface_index| surfaces.get(*surface_index))
            .map(|surface| surface.conductance_w_per_k)
            .sum();
        let (sum_ha_w_per_k, sum_hat_surf_w, sum_hat_ref_w) =
            zone_surface_convection_sums_for_indices(&surfaces, zone_surface_indexes);
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
        surface_indexes,
        last_ctf_history_slot_terms: Vec::new(),
        last_ctf_history_slot_terms_after_advance: Vec::new(),
        last_inside_surface_iteration_count: 0,
        last_inside_surface_iteration_max_delta_c: f64::NAN,
        last_inside_surface_iteration_max_delta_surface_name: None,
    })
}
