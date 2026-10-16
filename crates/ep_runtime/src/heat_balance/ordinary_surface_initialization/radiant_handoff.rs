//! Exact borrowed Main449 / Inside8809 radiation arguments.
//! The shared InitSurface flag remains owned by the complete caller suffix620.
use super::OrdinarySurfaceInitialization;
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::radiation::ordinary::{OrdinarySolarRadiation, RadiantExchangeCall};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::weather::day::WeatherGlobalState;

fn call(
    globals: &SelectedHeatBalanceGlobals,
    clock: &WeatherGlobalState,
    geometry: &SurfaceGeometryInputState,
    carroll_method: bool,
    surface_iterations: i32,
) -> RadiantExchangeCall {
    RadiantExchangeCall {
        surface_iterations,
        begin_environment: clock.begin_envrn_flag,
        kickoff_simulation: globals.kickoff_simulation,
        kickoff_sizing: globals.kickoff_sizing,
        any_movable_insulation: geometry.any_movable_insulation,
        carroll_method,
    }
}

impl OrdinarySurfaceInitialization {
    /// Source444-449. A transient immutable view projects the SAME persistent
    /// normal history term1 into final Source order. No mutable shadow is kept.
    /// The literal Source function argument is zero, independently of the
    /// DataHeatBalance InsideSurfIterations owner left by the preceding call.
    pub(crate) fn exchange_source_main(
        &mut self,
        radiation: &mut OrdinarySolarRadiation,
        geometry: &mut SurfaceGeometryInputState,
        constructions: &SelectedHeatBalanceInputState,
        history: &CtfThermalHistoryState,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        carroll_method: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let surfaces = geometry
            .final_surfaces
            .as_ref()
            .ok_or(Error::Unavailable("Main actual final Surface allocation"))?;
        let mut temperatures = Vec::with_capacity(surfaces.len());
        for surface in surfaces {
            let id = surface
                .typed_id
                .ok_or(Error::Unavailable("Main actual Source Surface custody"))?;
            let h = history
                .surface_ids
                .iter()
                .position(|candidate| *candidate == id)
                .ok_or(Error::Unavailable("Main same normal history owner"))?;
            temperatures.push(
                *history.normal.inside_temperature[0]
                    .get(h)
                    .ok_or(Error::Unavailable("Main actual SurfInsideTempHist(1)"))?,
            );
        }
        let controls = call(globals, clock, geometry, carroll_method, 0);
        radiation
            .exchange_source_radiant(
                geometry,
                constructions,
                &temperatures,
                self.arrays
                    .surf_abs_thermal_int
                    .as_deref()
                    .ok_or(Error::Unavailable("Main actual SurfAbsThermalInt"))?,
                self.arrays
                    .surf_qdot_rad_net_lw_in_per_area
                    .as_deref_mut()
                    .ok_or(Error::Unavailable("Main sole NetLW output array"))?,
                controls,
                backend,
            )
            .map_err(Error::Output)
    }

    /// Source8807-8816. Whole-array Old assignment precedes radiation; the
    /// actual global counter is passed before any 30-iteration convection call.
    /// The subsequent solver reads this very NetLW allocation directly.
    pub(crate) fn exchange_source_inside_iteration(
        &mut self,
        radiation: &mut OrdinarySolarRadiation,
        geometry: &mut SurfaceGeometryInputState,
        constructions: &SelectedHeatBalanceInputState,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        carroll_method: bool,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        let inside = self
            .arrays
            .surf_temp_in
            .as_deref()
            .ok_or(Error::Unavailable("Inside actual SurfTempIn"))?;
        let old = self
            .arrays
            .surf_temp_ins_old
            .as_deref_mut()
            .ok_or(Error::Unavailable("Inside actual SurfTempInsOld"))?;
        if old.len() != inside.len() {
            return Err(Error::Unavailable("Inside whole-array Old shape"));
        }
        old.clone_from_slice(inside);
        let controls = call(
            globals,
            clock,
            geometry,
            carroll_method,
            self.inside_surface_iterations,
        );
        radiation
            .exchange_source_radiant(
                geometry,
                constructions,
                inside,
                self.arrays
                    .surf_abs_thermal_int
                    .as_deref()
                    .ok_or(Error::Unavailable("Inside actual SurfAbsThermalInt"))?,
                self.arrays
                    .surf_qdot_rad_net_lw_in_per_area
                    .as_deref_mut()
                    .ok_or(Error::Unavailable("Inside sole NetLW output array"))?,
                controls,
                backend,
            )
            .map_err(Error::Output)
    }
}
