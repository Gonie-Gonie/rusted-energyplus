//! Surface boundary target and initial CTF boundary history helpers.

use crate::error::RuntimeError;
use crate::heat_balance::state::{HeatBalanceState, SurfaceHeatBalanceState};
use ep_model::{NormalizedName, OutsideBoundaryCondition, Surface, SurfaceId, TypedModel, ZoneId};
use std::collections::BTreeMap;

/// EnergyPlus source-order owner for surface boundary setup.
pub const SURFACE_BOUNDARY_OWNER_STAGE: &str = "InitSurfaceHeatBalance";

/// EnergyPlus default ground temperature used for building-surface CTF history seeding.
pub(crate) const ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C: f64 = 18.0;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct SurfaceBoundaryTarget {
    pub(crate) surface_id: Option<SurfaceId>,
    pub(crate) zone_id: Option<ZoneId>,
}

pub(crate) fn seed_initial_surface_ctf_boundary_histories(
    state: &mut HeatBalanceState,
    initial_outdoor_dry_bulb_c: f64,
) {
    let zone_temperatures = state
        .zones
        .iter()
        .map(|zone| (zone.zone_id, zone.mean_air_temperature_c))
        .collect::<BTreeMap<_, _>>();

    for surface in &mut state.surfaces {
        let inside_temperature_c = zone_temperatures
            .get(&surface.zone_id)
            .copied()
            .unwrap_or(surface.inside_face_temperature_c);
        let outside_temperature_c = initial_surface_ctf_boundary_temperature_c(
            surface,
            &zone_temperatures,
            initial_outdoor_dry_bulb_c,
            inside_temperature_c,
        );
        let initial_flux_w_per_m2 = surface_steady_u_value_w_per_m2_k(surface)
            * (outside_temperature_c - inside_temperature_c);

        surface.inside_face_temperature_c = inside_temperature_c;
        surface.outside_face_temperature_c = outside_temperature_c;
        surface
            .ctf
            .inside_temperature_history_c
            .fill(inside_temperature_c);
        surface
            .ctf
            .outside_temperature_history_c
            .fill(outside_temperature_c);
        surface
            .ctf
            .inside_flux_history_w_per_m2
            .fill(initial_flux_w_per_m2);
        surface
            .ctf
            .outside_flux_history_w_per_m2
            .fill(initial_flux_w_per_m2);
    }
}

pub(crate) fn seed_energyplus_initial_surface_ctf_histories(
    state: &mut HeatBalanceState,
    initial_surface_temperature_c: f64,
    initial_outdoor_dry_bulb_c: f64,
) {
    let zone_temperatures = state
        .zones
        .iter()
        .map(|zone| (zone.zone_id, initial_surface_temperature_c))
        .collect::<BTreeMap<_, _>>();

    for surface in &mut state.surfaces {
        let outside_temperature_c = initial_surface_ctf_boundary_temperature_c(
            surface,
            &zone_temperatures,
            initial_outdoor_dry_bulb_c,
            initial_surface_temperature_c,
        );
        let initial_flux_w_per_m2 = surface_steady_u_value_w_per_m2_k(surface)
            * (outside_temperature_c - initial_surface_temperature_c);

        surface.inside_face_temperature_c = initial_surface_temperature_c;
        surface.outside_face_temperature_c = outside_temperature_c;
        surface
            .ctf
            .inside_temperature_history_c
            .fill(initial_surface_temperature_c);
        surface
            .ctf
            .outside_temperature_history_c
            .fill(outside_temperature_c);
        surface
            .ctf
            .inside_flux_history_w_per_m2
            .fill(initial_flux_w_per_m2);
        surface
            .ctf
            .outside_flux_history_w_per_m2
            .fill(initial_flux_w_per_m2);
    }
}

pub(crate) fn initial_surface_ctf_boundary_temperature_c(
    surface: &SurfaceHeatBalanceState,
    zone_temperatures: &BTreeMap<ZoneId, f64>,
    initial_outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
) -> f64 {
    match surface.outside_boundary_condition {
        OutsideBoundaryCondition::Outdoors => initial_outdoor_dry_bulb_c,
        OutsideBoundaryCondition::Adiabatic => owning_zone_temperature_c,
        _ => surface_boundary_temperature_c(
            surface,
            zone_temperatures,
            initial_outdoor_dry_bulb_c,
            owning_zone_temperature_c,
        ),
    }
}

pub(crate) fn surface_steady_u_value_w_per_m2_k(surface: &SurfaceHeatBalanceState) -> f64 {
    if surface.thermal_resistance_m2_k_per_w > 0.0 {
        1.0 / surface.thermal_resistance_m2_k_per_w
    } else {
        0.0
    }
}

pub(crate) fn resolve_surface_boundary_target(
    model: &TypedModel,
    surface: &Surface,
) -> Result<SurfaceBoundaryTarget, RuntimeError> {
    match surface.outside_boundary_condition {
        OutsideBoundaryCondition::Surface => {
            let target_name = boundary_object_name(surface);
            let target_surface = model
                .surfaces
                .iter()
                .find(|candidate| candidate.name == NormalizedName::new(&target_name))
                .ok_or_else(|| RuntimeError::MissingSurfaceBoundaryTarget {
                    surface_name: surface.name.0.clone(),
                    target_name: target_name.clone(),
                })?;
            Ok(SurfaceBoundaryTarget {
                surface_id: Some(target_surface.id),
                zone_id: Some(target_surface.zone),
            })
        }
        OutsideBoundaryCondition::Zone | OutsideBoundaryCondition::Space => {
            let target_name = boundary_object_name(surface);
            let target_zone = model
                .zones
                .iter()
                .find(|zone| zone.name == NormalizedName::new(&target_name))
                .ok_or_else(|| RuntimeError::MissingZoneBoundaryTarget {
                    surface_name: surface.name.0.clone(),
                    target_name: target_name.clone(),
                })?;
            Ok(SurfaceBoundaryTarget {
                surface_id: None,
                zone_id: Some(target_zone.id),
            })
        }
        OutsideBoundaryCondition::Adiabatic
        | OutsideBoundaryCondition::Foundation
        | OutsideBoundaryCondition::Ground
        | OutsideBoundaryCondition::Outdoors
        | OutsideBoundaryCondition::Other => Ok(SurfaceBoundaryTarget::default()),
    }
}

pub(crate) fn boundary_object_name(surface: &Surface) -> String {
    surface
        .outside_boundary_condition_object
        .as_ref()
        .map(|name| name.0.clone())
        .unwrap_or_default()
}

pub(crate) fn surface_boundary_temperature_c(
    surface: &SurfaceHeatBalanceState,
    previous_zone_temperatures: &BTreeMap<ZoneId, f64>,
    outdoor_dry_bulb_c: f64,
    owning_zone_temperature_c: f64,
) -> f64 {
    match surface.outside_boundary_condition {
        OutsideBoundaryCondition::Outdoors => outdoor_dry_bulb_c,
        OutsideBoundaryCondition::Adiabatic => owning_zone_temperature_c,
        OutsideBoundaryCondition::Surface
        | OutsideBoundaryCondition::Zone
        | OutsideBoundaryCondition::Space => surface
            .outside_boundary_target_zone_id
            .and_then(|target_zone_id| previous_zone_temperatures.get(&target_zone_id).copied())
            .unwrap_or(owning_zone_temperature_c),
        OutsideBoundaryCondition::Ground => {
            ENERGYPLUS_DEFAULT_BUILDING_SURFACE_GROUND_TEMPERATURE_C
        }
        OutsideBoundaryCondition::Foundation | OutsideBoundaryCondition::Other => {
            surface.outside_face_temperature_c
        }
    }
}
