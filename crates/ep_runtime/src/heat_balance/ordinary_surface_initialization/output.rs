//! Live semantic output targets over the sole Source arrays/history owner.
use super::SourceSurfaceHeatBalanceArrays;
use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError as Error, OrdinaryRadiationUnavailable as Unavailable,
};
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::source_output_registration::{
    SourceRealOutputOwners, SourceRealOutputTarget, SourceSurfaceOutputField,
};

/// Only borrowed owners. Registration and sampling retain no numeric snapshot.
pub(crate) struct SourceSurfaceOutputOwners<'a> {
    pub(crate) arrays: &'a SourceSurfaceHeatBalanceArrays,
    pub(crate) sum_surface_heat_emission: &'a f64,
    pub(crate) geometry: &'a SurfaceGeometryInputState,
    pub(crate) history: &'a CtfThermalHistoryState,
    pub(crate) physical_surfaces: &'a [SourceSurfaceOutdoorState],
}

pub(super) fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|number| usize::try_from(number).ok())
        .ok_or_else(|| Unavailable::StorageDomain.into())
}

impl SourceSurfaceOutputOwners<'_> {
    pub(super) fn history_index(&self, source_surface: usize) -> Result<usize, Error> {
        let id = self
            .geometry
            .final_surfaces
            .as_ref()
            .and_then(|rows| rows.get(source_surface))
            .and_then(|row| row.typed_id)
            .ok_or(Unavailable::StorageDomain)?;
        self.history
            .surface_ids
            .iter()
            .position(|candidate| *candidate == id)
            .ok_or_else(|| Unavailable::StorageDomain.into())
    }

    pub(super) fn physical_surface(
        &self,
        source_surface: usize,
    ) -> Result<&SourceSurfaceOutdoorState, Error> {
        let id = self
            .geometry
            .final_surfaces
            .as_ref()
            .and_then(|rows| rows.get(source_surface))
            .and_then(|row| row.typed_id)
            .ok_or(Unavailable::StorageDomain)?;
        self.physical_surfaces
            .iter()
            .find(|surface| surface.surface_id == id)
            .ok_or_else(|| Unavailable::StorageDomain.into())
    }
}

impl SourceRealOutputOwners for SourceSurfaceOutputOwners<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, Error> {
        let SourceRealOutputTarget::SurfaceHeatBalance {
            field,
            index: number,
        } = target
        else {
            return Err(Unavailable::StorageDomain.into());
        };
        let i = index(number)?;
        let arrays = self.arrays;
        let value = match field {
            SourceSurfaceOutputField::SurfTempIn => arrays
                .surf_temp_in
                .as_ref()
                .and_then(|values| values.get(i)),
            SourceSurfaceOutputField::SurfTempOut => arrays
                .surf_temp_out
                .as_ref()
                .and_then(|values| values.get(i)),
            SourceSurfaceOutputField::SurfOpaqInsFaceCond => self
                .history
                .flux_reports
                .inside_conduction
                .get(self.history_index(i)?),
            SourceSurfaceOutputField::SurfOpaqInsFaceCondFlux => self
                .history
                .flux_reports
                .inside_flux
                .get(self.history_index(i)?),
            SourceSurfaceOutputField::SurfOpaqOutFaceCond => self
                .history
                .flux_reports
                .outside_conduction
                .get(self.history_index(i)?),
            SourceSurfaceOutputField::SurfOpaqOutFaceCondFlux => self
                .history
                .flux_reports
                .outside_flux
                .get(self.history_index(i)?),
            SourceSurfaceOutputField::SurfOpaqStorageCond => arrays
                .surf_opaq_storage_cond
                .as_ref()
                .and_then(|values| values.get(i)),
            SourceSurfaceOutputField::SurfOpaqStorageCondFlux => arrays
                .surf_opaq_storage_cond_flux
                .as_ref()
                .and_then(|values| values.get(i)),
        };
        value.ok_or_else(|| Unavailable::StorageDomain.into())
    }
}
