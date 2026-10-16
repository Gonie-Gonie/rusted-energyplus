//! Actual UpdateThermalHistories entry over the one persistent history owner.
//! Calls after the real inside/final-average producers; never begin_step/copyback.
use super::OrdinarySurfaceInitialization;
use super::prefix::{index, source_groups};
use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;
use crate::heat_balance::ctf_thermal_history::{
    CtfThermalHistoryAlgorithm, CtfThermalHistoryContext, CtfThermalHistoryState,
    CtfThermalHistorySurface, CtfThermalHistoryUpdateRoute, update_source_ctf_thermal_histories,
};
use crate::heat_balance::ordinary_heat_balance_initialization::ctf::SourceCtfInitialization;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, SelectedHeatTransferModel,
};
use crate::weather::day::WeatherGlobalState;
use ep_model::{OutsideBoundaryCondition, SurfaceId};
use std::collections::BTreeSet;

/// Disjoint live owners supplied by genuine ManageSurface after UpdateFinal.
pub(crate) struct SourceThermalHistoryOwners<'a> {
    pub(crate) geometry: &'a SurfaceGeometryInputState,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) ctf: &'a SourceCtfInitialization,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) history: &'a mut CtfThermalHistoryState,
    /// Truthful initialization-only operand in the shared input view. The Update
    /// body does not read outdoor temperature or select an outside boundary.
    pub(crate) physical_surfaces: &'a [SourceSurfaceOutdoorState],
}
fn value(values: &Option<Vec<f64>>, i: usize) -> Result<f64, Error> {
    values
        .as_deref()
        .and_then(|rows| rows.get(i))
        .copied()
        .ok_or(Error::Unavailable(
            "actual UpdateThermalHistories current surface operand",
        ))
}
fn source_id(geometry: &SurfaceGeometryInputState, number: i32) -> Result<SurfaceId, Error> {
    geometry
        .final_surfaces
        .as_deref()
        .and_then(|rows| index(number).ok().and_then(|at| rows.get(at)))
        .and_then(|row| row.typed_id)
        .ok_or(Error::Unavailable(
            "actual UpdateThermalHistories Source Surface identity",
        ))
}
impl OrdinarySurfaceInitialization {
    /// Genuine opaque/no-source Source update. The complete scientific body is
    /// shared literally with the retained kernel. Normal/master/scratch/four
    /// reports and Update first flag remain in this same supplied history owner.
    pub(crate) fn update_source_thermal_histories(
        &self,
        o: &mut SourceThermalHistoryOwners<'_>,
    ) -> Result<CtfThermalHistoryUpdateRoute, Error> {
        let rows = o
            .geometry
            .final_surfaces
            .as_deref()
            .ok_or(Error::Unavailable(
                "actual UpdateThermalHistories allocated final Surfaces",
            ))?;
        let count = usize::try_from(o.geometry.total_surfaces)
            .map_err(|_| Error::Unavailable("actual UpdateThermalHistories TotSurfaces"))?;
        if rows.len() != count || o.history.surface_ids.len() != count {
            return Err(Error::Unavailable(
                "same UpdateThermalHistories full Source layout",
            ));
        }
        for (row, id) in rows.iter().zip(&o.history.surface_ids) {
            if row.typed_id != Some(*id) {
                return Err(Error::Unavailable(
                    "same UpdateThermalHistories Source allocation order",
                ));
            }
        }
        let groups = |opaque| -> Result<Vec<Vec<SurfaceId>>, Error> {
            source_groups(o.zones, o.globals.num_of_zones, opaque)?
                .iter()
                .map(|group| {
                    group
                        .iter()
                        .map(|&number| source_id(o.geometry, number))
                        .collect()
                })
                .collect()
        };
        let ht_groups = groups(false)?;
        let opaque_groups = groups(true)?;
        let opaque: BTreeSet<_> = opaque_groups.iter().flatten().copied().collect();
        let all_ht = o
            .geometry
            .surface_lists
            .all_ht
            .iter()
            .map(|&number| source_id(o.geometry, number))
            .collect::<Result<Vec<_>, _>>()?;
        let public = &o.ctf.public;
        let mut inputs = Vec::new();
        for &number in &o.geometry.surface_lists.all_ht {
            let i = index(number)?;
            let actual = rows.get(i).ok_or(Error::Unavailable(
                "actual UpdateThermalHistories Source Surface ordinal",
            ))?;
            let surface_id = actual.typed_id.ok_or(Error::Unavailable(
                "actual UpdateThermalHistories Surface typed association",
            ))?;
            let construction = o
                .control
                .construction_usage
                .get(index(actual.row.construction)?)
                .ok_or(Error::Unavailable(
                    "actual UpdateThermalHistories Surface Construction ordinal",
                ))?;
            let construction_id = construction.construction_id.ok_or(Error::Unavailable(
                "actual UpdateThermalHistories Construction typed association",
            ))?;
            let returned = public
                .constructions
                .iter()
                .find(|row| row.construction_id == construction_id)
                .ok_or(Error::Unavailable(
                    "same fresh returned UpdateThermalHistories public Construction",
                ))?;
            let coefficients = &returned
                .result
                .as_ref()
                .map_err(|_| {
                    Error::Unavailable(
                        "actual UpdateThermalHistories completed public coefficient storage",
                    )
                })?
                .storage;
            let physical = o
                .physical_surfaces
                .iter()
                .find(|row| row.surface_id == surface_id)
                .ok_or(Error::Unavailable(
                    "same actual Surface outdoor owner for shared history input view",
                ))?;
            let outside_boundary = if actual.row.exterior_boundary == 0 {
                OutsideBoundaryCondition::Outdoors
            } else if actual.row.exterior_boundary == number {
                OutsideBoundaryCondition::Adiabatic
            } else if actual.row.exterior_boundary == -1 || actual.row.exterior_boundary == -5 {
                OutsideBoundaryCondition::Ground
            } else if actual.row.exterior_boundary > 0 {
                OutsideBoundaryCondition::Surface
            } else {
                OutsideBoundaryCondition::Other
            };
            inputs.push(CtfThermalHistorySurface {
                surface_id,
                construction_id,
                public_owner_construction_id: returned.construction_id,
                coefficients,
                algorithm: if actual.row.heat_transfer_algorithm == SelectedHeatTransferModel::Ctf {
                    CtfThermalHistoryAlgorithm::Ctf
                } else {
                    CtfThermalHistoryAlgorithm::Other
                },
                ordinary_opaque: opaque.contains(&surface_id),
                source_sink_present: construction.source_sink_present,
                outside_boundary,
                outside_dry_bulb: physical.outdoor_environment.row.dry_bulb_c,
                area: actual.row.area,
                inside_surface_temperature: value(&self.arrays.surf_temp_in, i)?,
                inside_constant: value(&self.arrays.surf_ctf_const_in_part, i)?,
                outside_constant: value(&self.arrays.surf_ctf_const_out_part, i)?,
            });
        }
        update_source_ctf_thermal_histories(
            o.history,
            &inputs,
            CtfThermalHistoryContext {
                simple_ctf_only: public.aggregate_after_returned_calls.simple_ctf_only,
                any_ems_in_model: o.globals.any_energy_management_system_in_model,
                any_construction_overrides: o.globals.any_construction_overrides_in_model,
                any_internal_heat_source: o.control.any_internal_heat_source_in_input,
                global_max_ctf_terms: public.aggregate_after_returned_calls.max_ctf_terms,
                time_step_zone_hours: o.clock.time_step_zone,
                heat_transfer_groups: &ht_groups,
                opaque_groups: &opaque_groups,
                all_heat_transfer_surfaces: &all_ht,
            },
        )
        .map_err(Error::from)
    }
}
