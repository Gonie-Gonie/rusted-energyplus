//! Persistent selected temperatures for the existing one-pass ordinary A caller.
//!
//! This unit retains actual solver returns. It neither changes that solver nor
//! activates InitIntConvCoeff, its readiness inputs, or its iteration cadence.

use super::algorithm::{HeatBalanceRuntimeConfig, HeatBalanceZoneAirUpdate};
use super::ctf_surface_lifecycle::CtfSurfaceLifecycleUnavailable as Unavailable;
use super::inside_convection_dispatcher::{NoUserConvectionInput, SpaceAirTemperatures};
use super::inside_convection_layout::InsideConvectionLayout;
use super::state::{HeatBalanceState, SurfaceHeatBalanceState, ZoneHeatBalanceState};
use ep_model::{SurfaceId, TypedModel, ZoneId};
use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct OrdinaryInsideTemperatures {
    layout: InsideConvectionLayout,
    /// Separate current ZT, never reconstructed from the initialization DTO.
    zone_zt: (ZoneId, f64),
    pub(crate) space_air: SpaceAirTemperatures,
    /// Actual raw no-user input admission retained for the future dispatcher.
    pub(crate) no_user_convection_input: NoUserConvectionInput,
    /// Actual project input owner; this unit does not activate its consumer.
    pub(crate) low_hconv_limit_w_per_m2_k: f64,
    /// Separate SurfTempInsOld allocation; BeginEnvrn does not reset it.
    pub(crate) inside_old: Vec<(SurfaceId, f64)>,
}

impl OrdinaryInsideTemperatures {
    fn allocate(
        layout: InsideConvectionLayout,
        no_user_convection_input: NoUserConvectionInput,
        low_hconv_limit_w_per_m2_k: f64,
    ) -> Result<Self, Unavailable> {
        validate_convection_inputs(no_user_convection_input, low_hconv_limit_w_per_m2_k)?;
        let [zone_id] = layout.zones() else {
            return Err(Unavailable::OrdinaryTemperatureOwner);
        };
        Ok(Self {
            zone_zt: (*zone_id, 23.0),
            space_air: SpaceAirTemperatures::allocate(&layout),
            no_user_convection_input,
            low_hconv_limit_w_per_m2_k,
            inside_old: layout
                .surfaces()
                .iter()
                .map(|s| (s.surface_id, 0.0))
                .collect(),
            layout,
        })
    }

    fn validate_zones(&self, zones: &[ZoneHeatBalanceState]) -> Result<(), Unavailable> {
        if zones.len() != 1 || zones[0].zone_id != self.zone_zt.0 {
            return Err(Unavailable::OrdinaryTemperatureOwner);
        }
        Ok(())
    }

    pub(crate) fn begin_environment(
        &mut self,
        zones: &mut [ZoneHeatBalanceState],
    ) -> Result<(), Unavailable> {
        self.validate_zones(zones)?;
        // HBSM2231-2250 reconstructs both Zone and Space owners, even with
        // space simulation disabled. Other constructor fields remain separate.
        self.zone_zt.1 = 23.0;
        zones[0].mean_air_temperature_c = 23.0;
        zones[0].zone_timestep_average_air_temperature_c = 23.0;
        self.space_air
            .reset_begin_environment(&self.layout)
            .map_err(|_| Unavailable::OrdinaryTemperatureOwner)
    }

    pub(crate) fn retain_solution(&mut self, id: ZoneId, zt: f64) -> Result<(), Unavailable> {
        if id != self.zone_zt.0 {
            return Err(Unavailable::OrdinaryTemperatureOwner);
        }
        self.zone_zt.1 = zt;
        Ok(())
    }

    pub(crate) fn commit_zone_mat(
        &self,
        zones: &mut [ZoneHeatBalanceState],
    ) -> Result<(), Unavailable> {
        self.validate_zones(zones)?;
        // Original4118: actual retained ZT -> MAT before sensible report/humidity.
        zones[0].mean_air_temperature_c = self.zone_zt.1;
        Ok(())
    }

    pub(crate) fn after_full_correction(
        &mut self,
        zones: &[ZoneHeatBalanceState],
    ) -> Result<(), Unavailable> {
        self.validate_zones(zones)?;
        // Original3843-3845: disabled-space copy after the full correction returns.
        // ZTAV remains its independent previous owner.
        self.space_air
            .copy_zone_after_correction(
                &self.layout,
                self.zone_zt.0,
                self.zone_zt.1,
                zones[0].mean_air_temperature_c,
            )
            .map_err(|_| Unavailable::OrdinaryTemperatureOwner)
    }

    fn reference_air_by_layout(&self) -> Result<BTreeMap<SurfaceId, f64>, Unavailable> {
        let rows = self.space_air.rows();
        if rows.len() != self.layout.spaces().len()
            || rows
                .iter()
                .zip(self.layout.spaces())
                .any(|(row, &(space, zone))| (row.space_id, row.zone_id) != (space, zone))
        {
            return Err(Unavailable::OrdinaryTemperatureOwner);
        }
        self.layout
            .surfaces()
            .iter()
            .map(|surface| {
                let row = rows
                    .iter()
                    .find(|row| row.space_id == surface.space_id && row.zone_id == surface.zone_id)
                    .ok_or(Unavailable::OrdinaryTemperatureOwner)?;
                Ok((surface.surface_id, row.mat))
            })
            .collect()
    }

    pub(crate) fn prepare_inside_entry(
        &mut self,
        model: &TypedModel,
        zones: &[ZoneHeatBalanceState],
        surfaces: &mut [SurfaceHeatBalanceState],
    ) -> Result<BTreeMap<SurfaceId, f64>, Unavailable> {
        validate(self, model, zones, surfaces)?;
        let reference_air = self.reference_air_by_layout()?;
        // HBSM8739-8741, DataSurfaces221-225: commit actual SpaceMAT before Old.
        // Complete typed layout and array association are checked before any write.
        for surface in surfaces.iter_mut() {
            surface.inside_reference_air_temperature_c = reference_air[&surface.surface_id];
        }
        self.capture_inside_entry(surfaces)?;
        Ok(reference_air)
    }

    pub(crate) fn capture_inside_entry(
        &mut self,
        surfaces: &[SurfaceHeatBalanceState],
    ) -> Result<(), Unavailable> {
        if self.inside_old.len() != surfaces.len()
            || self
                .inside_old
                .iter()
                .zip(surfaces)
                .any(|(old, s)| old.0 != s.surface_id)
        {
            return Err(Unavailable::OrdinaryTemperatureOwner);
        }
        // Actual inside-pass entry, before radiation or any inside solve writes.
        // Existing outside-stage writes are retained as real incoming owners.
        for (old, surface) in self.inside_old.iter_mut().zip(surfaces) {
            old.1 = surface.inside_face_temperature_c;
        }
        Ok(())
    }
}

fn validate_convection_inputs(
    no_user: NoUserConvectionInput,
    low_limit: f64,
) -> Result<(), Unavailable> {
    if no_user.counts() != [0; 6] || !low_limit.is_finite() || low_limit <= 0.0 {
        return Err(Unavailable::OrdinaryTemperatureOwner);
    }
    Ok(())
}

fn admit_config(config: HeatBalanceRuntimeConfig, passes: u32) -> Result<(), Unavailable> {
    if config.zone_air_update != HeatBalanceZoneAirUpdate::SimplifiedAnalytical
        || config.use_energyplus_adaptive_system_timestep_zone_air_correction
        || config.timestep.correct_zone_air_after_surface_pass
        || config.timestep.rebalance_surfaces_after_zone_air_correction
        || config.timestep.interleave_zone_air_surface_passes
        || passes != 1
    {
        return Err(Unavailable::OrdinaryTemperatureOwner);
    }
    Ok(())
}

pub(crate) fn prepare(
    model: &TypedModel,
    state: &mut HeatBalanceState,
    config: HeatBalanceRuntimeConfig,
    passes: u32,
    space_sizing: bool,
    space_simulation: bool,
    no_user_convection_input: NoUserConvectionInput,
    low_hconv_limit_w_per_m2_k: f64,
) -> Result<(), Unavailable> {
    admit_config(config, passes)?;
    if space_sizing
        || space_simulation
        || state.ctf_surface_lifecycle.is_none()
        || state.ordinary_inside_temperatures.is_some()
    {
        return Err(Unavailable::OrdinaryTemperatureOwner);
    }
    let layout = InsideConvectionLayout::from_typed_model(model)
        .map_err(|_| Unavailable::OrdinaryTemperatureOwner)?;
    let owner = OrdinaryInsideTemperatures::allocate(
        layout,
        no_user_convection_input,
        low_hconv_limit_w_per_m2_k,
    )?;
    validate(&owner, model, &state.zones, &state.surfaces)?;
    state.ordinary_inside_temperatures = Some(owner);
    Ok(())
}

fn validate(
    owner: &OrdinaryInsideTemperatures,
    model: &TypedModel,
    zones: &[ZoneHeatBalanceState],
    surfaces: &[SurfaceHeatBalanceState],
) -> Result<(), Unavailable> {
    owner.validate_zones(zones)?;
    validate_convection_inputs(
        owner.no_user_convection_input,
        owner.low_hconv_limit_w_per_m2_k,
    )?;
    let current = InsideConvectionLayout::from_typed_model(model)
        .map_err(|_| Unavailable::OrdinaryTemperatureOwner)?;
    if current != owner.layout
        || surfaces.len() != owner.layout.surfaces().len()
        || surfaces.len() != owner.inside_old.len()
        || surfaces
            .iter()
            .zip(&owner.inside_old)
            .any(|(s, old)| s.surface_id != old.0)
        || surfaces
            .iter()
            .zip(owner.layout.surfaces())
            .any(|(s, association)| {
                (s.surface_id, s.zone_id) != (association.surface_id, association.zone_id)
            })
    {
        return Err(Unavailable::OrdinaryTemperatureOwner);
    }
    Ok(())
}

pub(crate) fn validate_step(
    model: &TypedModel,
    state: &HeatBalanceState,
    config: HeatBalanceRuntimeConfig,
    passes: u32,
) -> Result<(), Unavailable> {
    if let Some(owner) = &state.ordinary_inside_temperatures {
        admit_config(config, passes)?;
        validate(owner, model, &state.zones, &state.surfaces)?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "ordinary_inside_temperatures_tests.rs"]
mod tests;
