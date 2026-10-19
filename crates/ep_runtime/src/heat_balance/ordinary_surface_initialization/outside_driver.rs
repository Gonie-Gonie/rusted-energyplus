//! Genuine no-resimulation opaque CalcHeatBalanceOutsideSurf6951-7737.
//! All mutable temperatures/coefficient/report cells remain in their sole Source owners.
mod boundary;
mod local_environment;
mod scalar;
#[cfg(all(test, feature = "native-controlled-air-cohort"))]
mod native_gap_cohort;
use super::OrdinarySurfaceInitialization;
use super::inside_ctf::{set, value};
use super::prefix::index;
use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;
use crate::heat_balance::ctf_public_initialization::OrderedCtfPublicInitialization;
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::inside_convection_dispatcher::source::SourceInsideConvection;
use crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::report_scan::ReportScanState;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceClass, SurfaceGeometryInputState,
};
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use ep_model::ReportInputs;

/// Disjoint temporary borrows; this bundle owns no numerical state or fallback values.
pub(crate) struct SourceOutsideCtfOwners<'a, 'backend> {
    pub(crate) geometry: &'a mut SurfaceGeometryInputState,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) air: &'a SourceAirHeatBalance,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) public: &'a OrderedCtfPublicInitialization,
    pub(crate) history: &'a mut CtfThermalHistoryState,
    pub(crate) radiation: &'a mut OrdinarySolarRadiation,
    pub(crate) physical: &'a [SourceSurfaceOutdoorState],
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) convection: &'a mut SourceInsideConvection,
    pub(crate) input: &'a SelectedInputProcessor,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) scan: &'a mut ReportScanState,
    pub(crate) carroll_method: bool,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
fn history_index(
    geometry: &SurfaceGeometryInputState,
    history: &CtfThermalHistoryState,
    number: i32,
) -> Result<usize, Error> {
    let id = geometry
        .final_surfaces
        .as_ref()
        .and_then(|rows| rows.get(index(number).ok()?))
        .and_then(|row| row.typed_id)
        .ok_or(Error::Unavailable("outside actual Source Surface identity"))?;
    history
        .surface_ids
        .iter()
        .position(|candidate| *candidate == id)
        .ok_or(Error::Unavailable("outside same normal history owner"))
}
fn outside_history(o: &SourceOutsideCtfOwners<'_, '_>, number: i32) -> Result<f64, Error> {
    let h = history_index(o.geometry, o.history, number)?;
    o.history.normal.outside_temperature[0]
        .get(h)
        .copied()
        .ok_or(Error::Unavailable("outside actual SurfOutsideTempHist(1)"))
}
fn write_outside_history(
    o: &mut SourceOutsideCtfOwners<'_, '_>,
    number: i32,
    temperature: f64,
) -> Result<(), Error> {
    let h = history_index(o.geometry, o.history, number)?;
    *o.history.normal.outside_temperature[0]
        .get_mut(h)
        .ok_or(Error::Unavailable(
            "outside actual SurfOutsideTempHist(1) output",
        ))? = temperature;
    Ok(())
}
fn outdoor<'o>(
    o: &'o SourceOutsideCtfOwners<'_, '_>,
    number: i32,
) -> Result<
    &'o crate::heat_balance::convection::surface_environment::SurfaceOutdoorEnvironmentRow,
    Error,
> {
    let id = o
        .geometry
        .final_surfaces
        .as_ref()
        .and_then(|rows| rows.get(index(number).ok()?))
        .and_then(|row| row.typed_id)
        .ok_or(Error::Unavailable(
            "outside actual physical Surface identity",
        ))?;
    o.physical
        .iter()
        .find(|row| row.surface_id == id)
        .map(|row| &row.outdoor_environment.row)
        .ok_or(Error::Unavailable(
            "outside same moved physical outdoor cells",
        ))
}

impl OrdinarySurfaceInitialization {
    /// The actual full all-Zone/no-resimulation call. No legacy surface endpoint,
    /// history recomputation or mutable radiation/temperature mirror is invoked.
    pub(crate) fn calculate_source_outside_surface_balance(
        &mut self,
        o: &mut SourceOutsideCtfOwners<'_, '_>,
    ) -> Result<(), Error> {
        local_environment::ground_temperature_average(o.geometry, o.globals)?;
        local_environment::surrounding_temperature_average(o.geometry, o.globals)?;
        if o.control.any_internal_heat_source_in_input {
            return Err(Error::Unavailable(
                "outside actual source/sink transfer SurfQsrcHist7022",
            ));
        }
        // Native7040-7049 and Main449 pass identical same-history term1, literal0
        // and sole NetLW arrays to this one accepted Source exchange kernel.
        self.exchange_source_main(
            o.radiation,
            o.geometry,
            o.control,
            o.history,
            o.globals,
            o.clock,
            o.carroll_method,
            o.backend,
        )?;
        if !o.geometry.surface_lists.all_outside_source.is_empty() {
            return Err(Error::Unavailable(
                "outside actual positive outsideHeatSourceTermSched getCurrentVal7055",
            ));
        }
        for zone_number in 1..=o.globals.num_of_zones {
            let spaces = o
                .zones
                .zones
                .as_ref()
                .and_then(|rows| rows.get(index(zone_number).ok()?))
                .ok_or(Error::Unavailable("outside actual Zone ordinal"))?
                .space_indexes
                .clone();
            for space_number in spaces {
                let space = o
                    .zones
                    .spaces
                    .as_ref()
                    .and_then(|rows| rows.get(index(space_number).ok()?))
                    .ok_or(Error::Unavailable("outside actual Space ordinal"))?;
                for number in space.ht_surface_first..=space.ht_surface_last {
                    let i = index(number)?;
                    let class = o
                        .geometry
                        .final_surfaces
                        .as_ref()
                        .and_then(|rows| rows.get(i))
                        .ok_or(Error::Unavailable("outside actual final Surface"))?
                        .row
                        .class;
                    if class == SurfaceClass::Window {
                        continue;
                    }
                    let construction = *o
                        .geometry
                        .window_arrays
                        .as_ref()
                        .and_then(|arrays| arrays.surf_active_construction.get(i))
                        .ok_or(Error::Unavailable("outside actual SurfActiveConstruction"))?;
                    let _actual_construction = o
                        .control
                        .construction_usage
                        .get(index(construction)?)
                        .ok_or(Error::Unavailable(
                            "outside actual active Construction before resets",
                        ))?;
                    // HMovInsul/HSky/HGround/HAir are native local zeros. The six
                    // actual field resets below precede every boundary branch.
                    set(&mut self.arrays.surf_h_conv_ext, i, 0.0)?;
                    set(&mut self.arrays.surf_h_air_ext, i, 0.0)?;
                    set(&mut self.arrays.surf_h_sky_ext, i, 0.0)?;
                    set(&mut self.arrays.surf_h_grd_ext, i, 0.0)?;
                    set(&mut self.arrays.surf_q_rad_lw_out_srd_surfs, i, 0.0)?;
                    set(&mut self.arrays.surf_h_srd_surf_ext, i, 0.0)?;
                    boundary::calculate(self, o, number, space_number, construction)?;
                    let oscm = o
                        .geometry
                        .final_surfaces
                        .as_ref()
                        .and_then(|rows| rows.get(i))
                        .ok_or(Error::Unavailable("outside actual report Surface"))?
                        .oscm_ptr;
                    if oscm > 0 {
                        return Err(Error::Unavailable(
                            "GetQdotConvOutPerArea actual positive OSCM7730",
                        ));
                    }
                    let report = if o.environment.current_weather.is_rain {
                        -value(&self.arrays.surf_h_conv_ext, i)?
                            * (outside_history(o, number)? - outdoor(o, number)?.wet_bulb_c)
                    } else {
                        -value(&self.arrays.surf_h_conv_ext, i)?
                            * (outside_history(o, number)? - outdoor(o, number)?.dry_bulb_c)
                    };
                    set(&mut self.arrays.surf_qdot_conv_out_per_area, i, report)?;
                }
            }
        }
        Ok(())
    }
}
