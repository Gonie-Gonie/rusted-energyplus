//! Heat-balance state initialization and diagnostic CTF seeding helpers.

mod schedule_cache;
mod state_shell;

#[cfg(test)]
pub(crate) use schedule_cache::initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache;
pub use schedule_cache::{
    initialize_heat_balance_state, initialize_heat_balance_state_with_ctf_coefficients,
    initialize_heat_balance_state_with_initialization_context,
};
pub(crate) use schedule_cache::{
    initialize_heat_balance_state_with_ctf_coefficients_and_schedule_cache_profiled,
    initialize_heat_balance_state_with_initialization_context_and_equipment_profiled,
    initialize_heat_balance_state_with_initialization_context_profiled,
};

use crate::error::RuntimeError;
use crate::geometry::{
    InitializedGeometryOwners, InitializedZoneGeometry, opaque_geometry_owners,
    zone_geometry_properties,
};
use crate::heat_balance::ctf::{
    ConstructionCtfCoefficientOverride, all_resistive_surface_ctf_state,
    construction_ctf_coefficients_by_name, steady_ctf_coefficient_w_per_m2_k,
    steady_surface_ctf_state, surface_ctf_state_from_coefficient_rows,
};
use crate::heat_balance::inside_convection::zone_surface_convection_sums_for_indices;
use crate::heat_balance::state::{
    HeatBalanceState, HeatBalanceSurfaceIndexes, SurfaceExteriorReportTerms,
    SurfaceHeatBalanceState, SurfaceOutsideBalanceDiagnostics, ZoneAirTemperatureCoefficients,
    ZoneHeatBalanceState,
};
use crate::heat_balance::surface_boundary::resolve_surface_boundary_target;
use crate::heat_balance::surface_manager::ConstructionThermalDataCache;
use crate::heat_balance::zone_air_correction::ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO;
use crate::heat_balance::zone_predictor_corrector::energyplus_zone_air_temperature_coefficients;
use crate::psychrometrics::energyplus_standard_zone_air_heat_capacity_j_per_k;
use crate::schedules::{InternalGainSchedulePhaseOperations, ScheduleSeriesCache};
use ep_model::SimulationModel;
use state_shell::finish_heat_balance_state;

/// Actual optional raw-admission projections for the one existing initializer.
#[derive(Default)]
pub(crate) struct OrdinaryInitializationInputs<'a> {
    pub(crate) equipment: Option<super::ordinary_equipment::OrdinaryEquipmentInput>,
    pub(crate) source: Option<SourceHeatBalanceInputOwners<'a>>,
    pub(crate) radiation: Option<super::radiation::ordinary::OrdinaryRadiationInput>,
}

pub(crate) struct SourceHeatBalanceInputOwners<'a> {
    pub(crate) environment: &'a mut crate::weather::day::WeatherEnvironmentState,
    pub(crate) minutes: i32,
    pub(crate) output_host: &'a mut super::source_output_registration::SourceMtdFileHost,
}

const ENERGYPLUS_INITIAL_CONVECTION_COEFFICIENT_W_PER_M2_K: f64 = 3.076;
fn initialize_heat_balance_state_with_ctf_coefficients_from_schedule_cache(
    model: &SimulationModel,
    initial_zone_air_temperature_c: f64,
    ctf_coefficients: &[ConstructionCtfCoefficientOverride],
    schedule_cache: &ScheduleSeriesCache,
    mut operations: Option<&mut InternalGainSchedulePhaseOperations>,
    mut initialization: Option<
        &mut crate::heat_balance::ctf_initialization_context::CtfInitializationContext<'_>,
    >,
    inputs: OrdinaryInitializationInputs<'_>,
) -> Result<HeatBalanceState, RuntimeError> {
    let OrdinaryInitializationInputs {
        equipment: equipment_input,
        radiation: radiation_input,
        source,
    } = inputs;
    if radiation_input.is_some() && initialization.is_none() {
        return Err(super::radiation::ordinary::OrdinaryRadiationError::from(
            super::radiation::ordinary::OrdinaryRadiationUnavailable::MissingInitializationContext,
        )
        .into());
    }
    let source_input_reached = source.is_some();
    let mut selected_world_geometry = false;
    let mut selected_world_zone_returns = Vec::new();
    if let Some(source) = source {
        let actual =
            initialization
                .as_deref_mut()
                .ok_or_else(|| RuntimeError::WarmupUnavailable {
                    reason: "source HB input requires its actual context".into(),
                })?;
        actual.initialize_selected_heat_balance_input(
            &model.typed,
            source.environment,
            source.minutes,
            source.output_host,
        )?;
        selected_world_geometry = actual.prepare_selected_world_surface_geometry(
            &model.typed,
            source.environment,
            radiation_input,
            source.minutes,
            &mut *source.output_host,
        )?;
        if selected_world_geometry {
            selected_world_zone_returns = actual.continue_selected_setup_zone_geometry(
                &model.typed,
                equipment_input,
                source.environment,
                source.minutes,
                &mut *source.output_host,
            )?;
            actual.continue_selected_heat_balance_input_before_solar()?;
            actual.initialize_selected_solar_view_factors(&model.typed)?;
        }
    }
    if let Some(actual) = initialization.as_deref_mut() {
        actual.prepare_report_input_cache(&model.typed.report_inputs)?;
    }
    // SurfaceGeometry290->298->548: actual admitted surface-data validation
    // precedes GetZoneEquipment, and volume calculation follows the getter.
    // Retain these real geometry/boundary returns for the later surface owner;
    // no second geometry calculation or Native endpoint is used.
    let admitted_surface_data = equipment_input
        .map(|_| {
            if model.typed.surfaces.is_empty() {
                return Err(RuntimeError::WarmupUnavailable {
                    reason: "missing admitted surface data before GetZoneEquipment".into(),
                });
            }
            super::inside_convection_layout::InsideConvectionLayout::from_typed_model(&model.typed)
                .map_err(|_| RuntimeError::WarmupUnavailable {
                    reason: "invalid surface associations before GetZoneEquipment".into(),
                })?;
            model
                .typed
                .surfaces
                .iter()
                .map(|surface| {
                    if source_input_reached && !selected_world_geometry {
                        initialization
                            .as_deref_mut()
                            .ok_or_else(|| RuntimeError::WarmupUnavailable {
                                reason: "surface construction requires actual input owner".into(),
                            })?
                            .mark_selected_surface_construction_used(surface.construction)?;
                    }
                    let geometry = if selected_world_geometry {
                        initialization
                            .as_deref()
                            .ok_or_else(|| RuntimeError::WarmupUnavailable {
                                reason: "selected World geometry requires its retained context"
                                    .into(),
                            })?
                            .selected_world_surface_properties(surface.id)?
                    } else {
                        crate::geometry::surface_geometry_properties(&surface.vertices).map_err(
                            |error| RuntimeError::InvalidSurfaceGeometry {
                                surface_name: surface.name.0.clone(),
                                reason: error.to_string(),
                            },
                        )?
                    };
                    let boundary = resolve_surface_boundary_target(&model.typed, surface)?;
                    Ok::<_, RuntimeError>((geometry, boundary))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .transpose()?;
    // World GetSurfaceData already wrote the same IsUsedCTF owner through
    // AssignHeatTransferAlgorithm before Setup298; retain the legacy mark route.
    if source_input_reached && !selected_world_geometry {
        let layout =
            super::inside_convection_layout::InsideConvectionLayout::from_typed_model(&model.typed)
                .map_err(|_| RuntimeError::WarmupUnavailable {
                    reason: "actual CTF surface domain".into(),
                })?;
        let actual =
            initialization
                .as_deref_mut()
                .ok_or_else(|| RuntimeError::WarmupUnavailable {
                    reason: "actual input owner before CTF assignment".into(),
                })?;
        for group in layout.groups() {
            for id in &group.surfaces {
                let surface = model
                    .typed
                    .surfaces
                    .iter()
                    .find(|s| s.id == *id)
                    .ok_or_else(|| RuntimeError::WarmupUnavailable {
                        reason: "actual CTF surface identity".into(),
                    })?;
                actual.mark_selected_surface_ctf_construction_used(surface.construction)?;
            }
        }
    }
    let equipment = if selected_world_geometry {
        // Source298 already constructed the sole retained owner; no replay/projection.
        None
    } else {
        equipment_input
            .map(|input| {
                super::ordinary_equipment::OrdinaryEquipment::for_geometry(&model.typed, input)
                    .map_err(|reason| RuntimeError::WarmupUnavailable {
                        reason: format!("ordinary geometry equipment unavailable: {reason}"),
                    })
            })
            .transpose()?
    };
    let ctf_coefficients_by_construction = construction_ctf_coefficients_by_name(ctf_coefficients);
    let mut zones = Vec::with_capacity(model.typed.zones.len());
    let mut zone_geometry_returns = Vec::with_capacity(model.typed.zones.len());
    for zone in &model.typed.zones {
        let geometry = if selected_world_geometry {
            initialization
                .as_deref_mut()
                .ok_or_else(|| RuntimeError::WarmupUnavailable {
                    reason: "retained World zone-volume context".into(),
                })?
                .selected_world_zone_volume(zone.id, &mut selected_world_zone_returns)?
        } else {
            let geometry = zone_geometry_properties(&model.typed, zone);
            crate::geometry::zone_volume_trace::record(zone.id, &zone.name.0, &geometry);
            geometry.map_err(|error| RuntimeError::UnsupportedZoneVolumeGeometry {
                zone_name: zone.name.0.clone(),
                reason: error.to_string(),
            })?
        };
        let volume_m3 = geometry.volume_m3;
        zone_geometry_returns.push(InitializedZoneGeometry {
            zone_id: zone.id,
            returned: geometry,
        });
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
            use_zone_timestep_history: true,
            shorten_timestep_sys: false,
            prior_timestep_seconds: 0.0,
            volume_m3,
            air_heat_capacity_j_per_k: energyplus_standard_zone_air_heat_capacity_j_per_k(
                volume_m3,
                initial_zone_air_temperature_c,
                ENERGYPLUS_DEFAULT_ZONE_AIR_HUMIDITY_RATIO,
            )
            .unwrap_or(0.0),
            // On the admitted solar route, defer the actual physical gain
            // consumer until construction/surface returns and InitSolar.
            // The legacy None route retains its existing call order.
            convective_internal_gain_w: if radiation_input.is_some() {
                0.0
            } else {
                schedule_cache::convective_internal_gain_w_for_initialization(
                    &model.typed,
                    schedule_cache,
                    zone.id,
                    operations.as_deref_mut(),
                )
            },
            opaque_surface_conductance_w_per_k: 0.0,
            opaque_surface_heat_gain_w: 0.0,
            opaque_surface_outside_conduction_w: 0.0,
            sum_ha_w_per_k: 0.0,
            sum_hat_surf_w: 0.0,
            sum_hat_ref_w: 0.0,
            sum_mcp_w_per_k: 0.0,
            sum_mcp_t_w: 0.0,
            sum_sys_mcp_w_per_k: 0.0,
            sum_sys_mcp_t_w: 0.0,
            system_dependent_zone_loads_lagged_w: 0.0,
            zone_air_temperature_coefficients: ZoneAirTemperatureCoefficients::ZERO,
            system_timestep_average_surface_convection_report_w: None,
            system_timestep_average_air_storage_report_w: None,
        });
    }

    let selected_opaque_geometry = if source_input_reached {
        let data =
            admitted_surface_data
                .as_ref()
                .ok_or_else(|| RuntimeError::WarmupUnavailable {
                    reason: "source Zone geometry requires actual admitted surface returns".into(),
                })?;
        let actual_surface_geometry = model
            .typed
            .surfaces
            .iter()
            .zip(data)
            .map(|(surface, (geometry, _))| (surface.id, *geometry))
            .collect::<Vec<_>>();
        let geometry = opaque_geometry_owners(&model.typed, &actual_surface_geometry);
        let actual =
            initialization
                .as_deref_mut()
                .ok_or_else(|| RuntimeError::WarmupUnavailable {
                    reason: "Zone geometry commit requires retained caller owner".into(),
                })?;
        let zone_owner = actual.reports.selected_zone_input.as_mut().ok_or_else(|| {
            RuntimeError::WarmupUnavailable {
                reason: "Zone geometry before real Zone getter".into(),
            }
        })?;
        let rows = zone_owner
            .zones
            .as_mut()
            .ok_or_else(|| RuntimeError::WarmupUnavailable {
                reason: "Zone geometry before actual allocation".into(),
            })?;
        let opaque = geometry
            .as_ref()
            .map_err(|error| RuntimeError::WarmupUnavailable {
                reason: format!("actual Zone centroid unavailable: {error}"),
            })?;
        for zone in rows {
            let id = zone
                .typed_zone_id
                .ok_or_else(|| RuntimeError::WarmupUnavailable {
                    reason: "raw Zone identity".into(),
                })?;
            if !opaque.zones.iter().any(|row| row.zone_id == id) {
                return Err(RuntimeError::WarmupUnavailable {
                    reason: "actual Zone geometry association absent".into(),
                });
            }
            let volume = zone_geometry_returns
                .iter()
                .find(|row| row.zone_id == id)
                .ok_or_else(|| RuntimeError::WarmupUnavailable {
                    reason: "actual Zone volume return absent".into(),
                })?;
            if !selected_world_geometry {
                // Preserve the qualified legacy projection. The selected World
                // caller already wrote the same Source centroid at actual550.
                let layout =
                    super::inside_convection_layout::InsideConvectionLayout::from_typed_model(
                        &model.typed,
                    )
                    .map_err(|_| RuntimeError::WarmupUnavailable {
                        reason: "actual Zone centroid traversal".into(),
                    })?;
                let mut centroid = [0.0; 3];
                let mut total_surface_area = 0.0;
                zone.centroid_m = Some(centroid);
                for group in layout.groups().iter().filter(|group| group.zone_id == id) {
                    for surface_id in &group.surfaces {
                        let (_, geometry) = actual_surface_geometry
                            .iter()
                            .find(|row| row.0 == *surface_id)
                            .ok_or_else(|| RuntimeError::WarmupUnavailable {
                                reason: "actual Zone centroid surface return".into(),
                            })?;
                        centroid[0] += geometry.centroid_m.x_m * geometry.gross_area_m2;
                        centroid[1] += geometry.centroid_m.y_m * geometry.gross_area_m2;
                        centroid[2] += geometry.centroid_m.z_m * geometry.gross_area_m2;
                        total_surface_area += geometry.gross_area_m2;
                        zone.centroid_m = Some(centroid);
                    }
                }
                if total_surface_area > 0.0 {
                    centroid[0] /= total_surface_area;
                    centroid[1] /= total_surface_area;
                    centroid[2] /= total_surface_area;
                    zone.centroid_m = Some(centroid);
                }
                zone.ceiling_height = volume.returned.ceiling_height_m;
                zone.volume = volume.returned.volume_m3;
                zone.floor_area = volume.returned.floor_area_m2;
            }
            // World keeps the genuine earlier area/height/volume and550 centroid
            // writes, including any preceding error prefix.
        }
        Some(geometry)
    } else {
        None
    };

    // Validate actual declared references before selected public generation
    // can reject an unsupported boundary family. Preserve the existing
    // MissingSurface/ZoneBoundaryTarget contract without widening support.
    let boundary_targets = if let Some(data) = &admitted_surface_data {
        data.iter().map(|(_, boundary)| *boundary).collect()
    } else {
        model
            .typed
            .surfaces
            .iter()
            .map(|surface| resolve_surface_boundary_target(&model.typed, surface))
            .collect::<Result<Vec<_>, _>>()?
    };
    let mut construction_thermal_data = if let Some(actual) = initialization.as_deref_mut() {
        if !actual.reports.setup_simulation_registry_prepared {
            // Legacy/None routes retain genuine repeated registration.
            actual.prepare_registry(&model.typed)?;
        }
        if !selected_world_geometry {
            crate::heat_balance::ctf_geometry_reports::report_ordinary_surface_nominal_u(
                &model.typed,
                &actual.reports.registry,
                &mut actual.reports.table,
            )
            .map_err(
                crate::heat_balance::ctf_initialization_context::CtfInitializationError::Owner,
            )?;
        }
        ConstructionThermalDataCache::build_with_initialization_context(
            &model.typed,
            &ctf_coefficients_by_construction,
            actual,
        )?
    } else {
        ConstructionThermalDataCache::build(&model.typed, &ctf_coefficients_by_construction)?
    };
    let construction_cache_token = construction_thermal_data.invalidation_token();
    debug_assert!(!construction_thermal_data.is_invalidated_by(construction_cache_token));
    let ctf_surface_lifecycle = construction_thermal_data
        .take_public_surface_lifecycle()
        .transpose()
        .map_err(RuntimeError::CtfSurfaceLifecycle)?;
    let public_generation = if ctf_surface_lifecycle.is_some() {
        Some(crate::heat_balance::ctf_surface_binding::capture_public_ctf_surface_generation(
            &model.typed,
            construction_thermal_data.public_initialization().ok_or(RuntimeError::CtfSurfaceLifecycle(
                crate::heat_balance::ctf_surface_lifecycle::CtfSurfaceLifecycleUnavailable::MissingPublicOwner))?,
            construction_thermal_data.generation_context,
        ).map_err(|error| RuntimeError::CtfSurfaceLifecycle(error.into()))?)
    } else {
        None
    };
    // Stage every fallible per-surface calculation before moving Source arrays.
    // These closures own constructor inputs; none allocates an outdoor owner.
    let surface_builders = model
        .typed
        .surfaces
        .iter()
        .zip(boundary_targets)
        .enumerate()
        .map(|(index, (surface, boundary))| {
            let geometry = if let Some(data) = &admitted_surface_data {
                data[index].0
            } else if selected_world_geometry {
                initialization.as_deref().ok_or_else(|| RuntimeError::WarmupUnavailable {
                    reason: "selected World surface requires retained geometry".into(),
                })?.selected_world_surface_properties(surface.id)?
            } else {
                crate::geometry::surface_geometry_properties(&surface.vertices)
                    .map_err(|error| RuntimeError::InvalidSurfaceGeometry {
                        surface_name: surface.name.0.clone(), reason: error.to_string(),
                    })?
            };
            let area_m2 = geometry.area_m2;
            let azimuth_deg = geometry.azimuth_deg;
            let tilt_deg = geometry.tilt_deg;
            let thermal = construction_thermal_data.data_for_surface(surface)?;
            let conductance_w_per_k = area_m2 / thermal.thermal_resistance_m2_k_per_w;
            let steady_ctf_w_per_m2_k =
                steady_ctf_coefficient_w_per_m2_k(area_m2, thermal.thermal_resistance_m2_k_per_w);
            let ctf = if let Some(generation) = &public_generation {
                let binding = generation
                    .bind_surface(surface.id)
                    .map_err(|error| RuntimeError::CtfSurfaceLifecycle(error.into()))?;
                crate::heat_balance::ctf::surface_ctf_state_from_public_storage(
                    binding.coefficients,
                )
            } else {
                surface_ctf_state_from_coefficient_rows(
                    &thermal.ctf_coefficients,
                    initial_zone_air_temperature_c,
                )
                .unwrap_or_else(|| {
                    match (
                        thermal.ctf_coefficients.is_empty(),
                        &thermal.ctf02_coefficients.result,
                    ) {
                        (true, Ok(coefficients)) => all_resistive_surface_ctf_state(
                            coefficients,
                            initial_zone_air_temperature_c,
                        ),
                        _ => steady_surface_ctf_state(
                            steady_ctf_w_per_m2_k,
                            initial_zone_air_temperature_c,
                        ),
                    }
                })
            };

            Ok(move |outdoor_environment: Option<super::convection::surface_environment::SurfaceOutdoorEnvironmentState>| SurfaceHeatBalanceState {
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
                construction_thermal_data_index: thermal.cache_index,
                construction_name: thermal.construction_name.clone(),
                outside_layer_material_id: thermal.outside_layer_material_id,
                outside_layer_material_name: thermal.outside_layer_material_name.clone(),
                outside_layer_roughness: thermal.outside_layer_roughness,
                area_m2,
                azimuth_deg,
                tilt_deg,
                geometry,
                outdoor_environment: outdoor_environment.unwrap_or_else(||
                    super::convection::surface_environment::SurfaceOutdoorEnvironmentState::allocate(
                        geometry.centroid_m.z_m, surface.wind_exposure == ep_model::WindExposure::WindExposed)),
                thermal_resistance_m2_k_per_w: thermal.thermal_resistance_m2_k_per_w,
                heat_capacity_j_per_m2_k: thermal.heat_capacity_j_per_m2_k,
                thermal_absorptance: thermal.thermal_absorptance,
                inside_thermal_absorptance: thermal.inside_thermal_absorptance,
                solar_absorptance: thermal.solar_absorptance,
                inside_solar_absorptance: thermal.inside_solar_absorptance,
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
    let mut surfaces = if selected_world_geometry {
        let actual =
            initialization
                .as_deref_mut()
                .ok_or_else(|| RuntimeError::WarmupUnavailable {
                    reason: "Source outdoor handoff requires actual context".into(),
                })?;
        // Validate later sole-owner moves now; no guard failure follows the final move.
        if actual.reports.selected_heat_balance_globals.is_none()
            || actual.reports.selected_zone_input.is_none()
            || (selected_world_geometry && actual.reports.source_zone_equipment.is_none())
        {
            return Err(RuntimeError::WarmupUnavailable {
                reason: "source owners before outdoor handoff".into(),
            });
        }
        let outdoor = actual.take_selected_world_outdoor_rows(&model.typed)?;
        surface_builders
            .into_iter()
            .zip(outdoor)
            .map(|(build, row)| build(Some(row)))
            .collect::<Vec<_>>()
    } else {
        surface_builders
            .into_iter()
            .map(|build| build(None))
            .collect::<Vec<_>>()
    };
    if let Some(input) = radiation_input {
        let actual = initialization.as_deref_mut().ok_or_else(|| {
            RuntimeError::from(super::radiation::ordinary::OrdinaryRadiationError::from(
                super::radiation::ordinary::OrdinaryRadiationUnavailable::MissingInitializationContext,
            ))
        })?;
        let mut owner = if selected_world_geometry {
            let Some(mut owner) = actual.reports.ordinary_solar_radiation.take() else {
                actual.restore_selected_world_outdoor_rows(surfaces);
                return Err(RuntimeError::WarmupUnavailable {
                    reason: "missing same early radiant enclosure container".into(),
                });
            };
            if let Err(error) = owner.bind_source_initialized_solar_geometry(
                &model.typed,
                &surfaces,
                input,
                &actual.reports.surface_geometry_input,
            ) {
                actual.reports.ordinary_solar_radiation = Some(owner);
                actual.restore_selected_world_outdoor_rows(surfaces);
                return Err(error.into());
            }
            owner
        } else {
            super::radiation::ordinary::OrdinarySolarRadiation::for_initialized_geometry(
                &model.typed,
                &surfaces,
                input,
            )?
        };
        // Restore the real reached prefix before propagating any output/domain
        // failure, so the caller's reports/EIO/backend retain the partial owner.
        let result = if selected_world_geometry {
            // Actual316 has already initialized this same owner from Source
            // Surface/Construct operands. Late binding must not fill it again.
            Ok(())
        } else {
            owner.initialize_solar_at_input_boundary(&model.typed, &surfaces, actual)
        };
        actual.reports.ordinary_solar_radiation = Some(owner);
        if let Err(error) = result {
            if selected_world_geometry {
                actual.restore_selected_world_outdoor_rows(surfaces);
            }
            return Err(error.into());
        }
        // These are the present typed-model consumers, not a claim that the
        // full Original ManageInternalHeatGains/GetInternalHeatGainsInput is ported.
        for zone in &mut zones {
            zone.convective_internal_gain_w =
                schedule_cache::convective_internal_gain_w_for_initialization(
                    &model.typed,
                    schedule_cache,
                    zone.zone_id,
                    operations.as_deref_mut(),
                );
        }
    }
    schedule_cache::initialize_surface_radiant_internal_gains(
        &model.typed,
        schedule_cache,
        &mut surfaces,
        operations,
    );
    let surface_indexes = HeatBalanceSurfaceIndexes::from_model_surfaces(model, &surfaces);
    let actual_surface_geometry = surfaces
        .iter()
        .map(|surface| (surface.surface_id, surface.geometry))
        .collect::<Vec<_>>();
    let initialized_geometry = InitializedGeometryOwners {
        zone_returns: zone_geometry_returns,
        opaque_setup: selected_opaque_geometry
            .unwrap_or_else(|| opaque_geometry_owners(&model.typed, &actual_surface_geometry)),
    };

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
        zone.zone_air_temperature_coefficients =
            crate::heat_balance::air_manager::get_air_heat_balance_input_compat(|| {
                energyplus_zone_air_temperature_coefficients(
                    zone.sum_ha_w_per_k,
                    zone.sum_hat_surf_w,
                    zone.sum_hat_ref_w,
                    zone.convective_internal_gain_w,
                    zone.sum_mcp_w_per_k + zone.sum_sys_mcp_w_per_k,
                    zone.sum_mcp_t_w + zone.sum_sys_mcp_t_w,
                    zone.air_heat_capacity_j_per_k,
                    0.0,
                    zone.previous_mean_air_temperatures_c,
                )
            });
    }

    let mut state = finish_heat_balance_state(
        zones,
        initialized_geometry,
        surfaces,
        surface_indexes,
        construction_thermal_data,
        ctf_surface_lifecycle,
    );
    // No fallible initialization work follows the sole-owner move.
    if source_input_reached {
        let actual =
            initialization
                .as_deref_mut()
                .ok_or_else(|| RuntimeError::WarmupUnavailable {
                    reason: "completed source HB initialization requires retained context".into(),
                })?;
        if actual.reports.selected_heat_balance_globals.is_none()
            || actual.reports.selected_zone_input.is_none()
            || (selected_world_geometry && actual.reports.source_zone_equipment.is_none())
        {
            return Err(RuntimeError::WarmupUnavailable {
                reason: "missing source owners after geometry".into(),
            });
        }
        state.selected_heat_balance_globals = actual.reports.selected_heat_balance_globals.take();
        state.selected_zone_input = actual.reports.selected_zone_input.take();
        if selected_world_geometry {
            state.ordinary_equipment = actual.reports.source_zone_equipment.take();
        }
    }
    if !selected_world_geometry {
        state.ordinary_equipment = equipment;
    }
    if radiation_input.is_some() {
        state.ordinary_solar_radiation =
            initialization.and_then(|actual| actual.reports.ordinary_solar_radiation.take());
    }
    Ok(state)
}
