//! Direct hourly producer bindings into the same radiant enclosure and arrays.
use super::*;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainRadiationOwners;
use crate::heat_balance::radiation::ordinary::{OrdinarySolarRadiation, RadiantRadiationEnclosure};
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;

pub(super) struct SourceSurfaceGainRadiation<'a> {
    pub(super) radiation: &'a mut OrdinarySolarRadiation,
    pub(super) geometry: &'a SurfaceGeometryInputState,
    pub(super) arrays: &'a mut SourceSurfaceHeatBalanceArrays,
}
impl SourceGainRadiationOwners for SourceSurfaceGainRadiation<'_> {
    fn num_radiant_enclosures(&self) -> i32 {
        self.radiation.num_radiant_enclosures()
    }
    fn radiant_enclosure_mut(
        &mut self,
        number: i32,
    ) -> Result<&mut RadiantRadiationEnclosure, Error> {
        self.radiation
            .source_radiant_enclosures_mut()
            .get_mut(prefix::index(number)?)
            .ok_or(Error::Unavailable("same actual gain radiant enclosure"))
    }
    fn surface_radiant_enclosure(&self, surface: i32) -> Result<i32, Error> {
        self.geometry
            .final_surfaces
            .as_deref()
            .and_then(|rows| rows.get(prefix::index(surface).ok()?))
            .map(|row| row.row.rad_encl_index)
            .ok_or(Error::Unavailable("same actual gain Surface RadEnclIndex"))
    }
    fn surface_thermal_absorptance(&self, surface: i32) -> Result<&f64, Error> {
        self.arrays
            .surf_abs_thermal_int
            .as_deref()
            .and_then(|rows| rows.get(prefix::index(surface).ok()?))
            .ok_or(Error::Unavailable("same actual gain SurfAbsThermalInt"))
    }
    fn surface_internal_radiant_gain_mut(&mut self, surface: i32) -> Result<&mut f64, Error> {
        self.arrays
            .surf_qdot_rad_int_gains_in_per_area
            .as_deref_mut()
            .and_then(|rows| rows.get_mut(prefix::index(surface).ok()?))
            .ok_or(Error::Unavailable("same actual gain radiant flux sink"))
    }
}
