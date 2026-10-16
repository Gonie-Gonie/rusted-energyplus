//! Genuine opaque ReportSurfaceHeatBalance and ReportSurfaceShading writes.
//! This consumes the same Surface/Solar/history owners after the Air/history calls.
use super::OrdinarySurfaceInitialization;
use super::inside_ctf::{set, value};
use super::prefix::index;
use crate::heat_balance::convection::surface_environment::source_rows::SourceSurfaceOutdoorState;
use crate::heat_balance::ctf_thermal_history::CtfThermalHistoryState;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::radiation::ordinary::OrdinarySolarRadiation;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error,
    surface::{SurfaceClass, SurfaceGeometryInputState},
    zone::SelectedZoneInputOwners,
};
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};

/// Borrows of actual persistent owners; this bundle creates no report flags/state.
pub(crate) struct SourceSurfaceReportOwners<'a> {
    pub(crate) geometry: &'a mut SurfaceGeometryInputState,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) radiation: &'a mut OrdinarySolarRadiation,
    pub(crate) history: &'a mut CtfThermalHistoryState,
    pub(crate) physical: &'a [SourceSurfaceOutdoorState],
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) predefined: &'a PredefinedRegistry,
    /// The real OutputReportTabular flag, including its actual earlier input writer.
    /// A caller may not substitute a constructor default or a typed summary guess.
    pub(crate) display_heat_emissions_summary: &'a bool,
}
fn area(g: &SurfaceGeometryInputState, number: i32) -> Result<f64, Error> {
    g.final_surfaces
        .as_ref()
        .and_then(|r| r.get(index(number).ok()?))
        .map(|r| r.row.area)
        .ok_or(Error::Unavailable("report actual opaque Surface Area"))
}
fn history_index(o: &SourceSurfaceReportOwners<'_>, number: i32) -> Result<usize, Error> {
    let id = o
        .geometry
        .final_surfaces
        .as_ref()
        .and_then(|r| r.get(index(number).ok()?))
        .and_then(|r| r.typed_id)
        .ok_or(Error::Unavailable("report Source Surface identity"))?;
    o.history
        .surface_ids
        .iter()
        .position(|x| *x == id)
        .ok_or(Error::Unavailable("report same history Surface identity"))
}
fn scalar(v: &[f64], i: usize) -> Result<f64, Error> {
    v.get(i)
        .copied()
        .ok_or(Error::Unavailable("report same allocated flux owner"))
}
fn write_scalar(v: &mut [f64], i: usize, x: f64) -> Result<(), Error> {
    *v.get_mut(i)
        .ok_or(Error::Unavailable("report same allocated flux owner"))? = x;
    Ok(())
}
fn spaces(
    o: &SourceSurfaceReportOwners<'_>,
) -> Result<Vec<(i32, i32, i32, i32, i32, i32, usize)>, Error> {
    let mut result = Vec::new();
    for zone_number in 1..=o.globals.num_of_zones {
        let zone = o
            .zones
            .zones
            .as_ref()
            .and_then(|r| r.get(index(zone_number).ok()?))
            .ok_or(Error::Unavailable("report actual Zone ordinal"))?;
        for &space_number in &zone.space_indexes {
            let space = o
                .zones
                .spaces
                .as_ref()
                .and_then(|r| r.get(index(space_number).ok()?))
                .ok_or(Error::Unavailable("report actual Space ordinal"))?;
            result.push((
                space.opaque_or_window_first,
                space.opaque_or_window_last,
                space.window_first,
                space.window_last,
                space.opaque_or_intmass_first,
                space.opaque_or_intmass_last,
                index(zone_number)?,
            ));
        }
    }
    Ok(result)
}

/// SolarShading11533–11619. Positive predefined real entries remain explicit.
pub(crate) fn report_source_surface_shading(
    o: &mut SourceSurfaceReportOwners<'_>,
) -> Result<(), Error> {
    for number in 1..=o.geometry.total_surfaces {
        let ordinal =
            usize::try_from(number).map_err(|_| Error::Unavailable("report Surface ordinal"))?;
        let hour = usize::try_from(o.clock.hour_of_day)
            .map_err(|_| Error::Unavailable("report HourOfDay"))?;
        let step = usize::try_from(o.clock.time_step)
            .map_err(|_| Error::Unavailable("report TimeStep"))?;
        let fraction = o
            .radiation
            .solar_timestep
            .heat_balance
            .surf_sunlit_frac
            .read([hour, step, ordinal])?;
        o.geometry
            .window_arrays
            .as_mut()
            .ok_or(Error::Unavailable("report same Surface solar arrays"))?
            .surf_sunlit_frac
            .set([ordinal], fraction)?;
        // Native reads the same 3D solar array after the fraction write, then Area.
        let fraction = o
            .radiation
            .solar_timestep
            .heat_balance
            .surf_sunlit_frac
            .read([hour, step, ordinal])?;
        let surface_area = area(o.geometry, number)?;
        o.geometry
            .window_arrays
            .as_mut()
            .ok_or(Error::Unavailable("report same Surface solar arrays"))?
            .surf_sunlit_area
            .set([ordinal], fraction * surface_area)?;
    }
    let mut column = 0;
    if o.environment.day_of_month == 21 && o.clock.time_step == 4 {
        let key = match (o.environment.month, o.clock.hour_of_day) {
            (3, 9) => Some("pdchSlfMar21_9"),
            (3, 12) => Some("pdchSlfMar21_12"),
            (3, 15) => Some("pdchSlfMar21_15"),
            (6, 9) => Some("pdchSlfJun21_9"),
            (6, 12) => Some("pdchSlfJun21_12"),
            (6, 15) => Some("pdchSlfJun21_15"),
            (12, 9) => Some("pdchSlfDec21_9"),
            (12, 12) => Some("pdchSlfDec21_12"),
            (12, 15) => Some("pdchSlfDec21_15"),
            _ => None,
        };
        if let Some(key) = key {
            column = *o
                .predefined
                .named_indices
                .get(key)
                .ok_or(Error::Unavailable("report actual predefined named column"))?;
        }
    }
    if column != 0 {
        for number in 1..=o.geometry.total_surfaces {
            let surface = o
                .geometry
                .final_surfaces
                .as_ref()
                .and_then(|r| r.get(index(number).ok()?))
                .ok_or(Error::Unavailable("report same final Surface"))?;
            if surface.row.class == SurfaceClass::Window {
                return Err(Error::Unavailable(
                    "ReportSurfaceShading genuine predefined real entry",
                ));
            }
        }
    }
    Ok(())
}

impl OrdinarySurfaceInitialization {
    pub(crate) fn report_source_surface_heat_balance(
        &mut self,
        o: &mut SourceSurfaceReportOwners<'_>,
    ) -> Result<(), Error> {
        report_source_surface_shading(o)?;
        if o.geometry.use_representative_surface_calculations {
            return Err(Error::Unavailable(
                "ReportNonRepresentativeSurfaceResults6618",
            ));
        }
        let loops = spaces(o)?;
        for &(first, last, _, _, _, _, _) in &loops {
            for number in first..=last {
                let i = index(number)?;
                let surface_area = area(o.geometry, number)?;
                let a = &mut self.arrays;
                macro_rules! v {
                    ($field:ident) => {
                        value(&a.$field, i)?
                    };
                }
                macro_rules! put {
                    ($field:ident,$expression:expr) => {{
                        let x = $expression;
                        set(&mut a.$field, i, x)?;
                    }};
                }
                put!(
                    surf_qdot_conv_in_rep,
                    surface_area * v!(surf_qdot_conv_in_per_area)
                );
                put!(
                    surf_q_conv_in_rep,
                    v!(surf_qdot_conv_in_rep) * o.clock.time_step_zone_sec
                );
                put!(
                    surf_qdot_rad_net_surf_in_rep,
                    v!(surf_qdot_rad_net_lw_in_per_area) * surface_area
                );
                put!(
                    surf_q_rad_net_surf_in_rep,
                    v!(surf_qdot_rad_net_surf_in_rep) * o.clock.time_step_zone_sec
                );
                put!(
                    surf_qdot_rad_int_gains_in_rep,
                    v!(surf_qdot_rad_int_gains_in_per_area) * surface_area
                );
                put!(
                    surf_q_rad_int_gains_in_rep,
                    v!(surf_qdot_rad_int_gains_in_rep) * o.clock.time_step_zone_sec
                );
                put!(
                    surf_qdot_rad_hvac_in_rep,
                    v!(surf_qdot_rad_hvac_in_per_area) * surface_area
                );
                put!(
                    surf_q_rad_hvac_in_rep,
                    v!(surf_qdot_rad_hvac_in_rep) * o.clock.time_step_zone_sec
                );
                put!(
                    surf_qdot_conv_out_rep,
                    v!(surf_qdot_conv_out_per_area) * surface_area
                );
                put!(
                    surf_q_conv_out_report,
                    v!(surf_qdot_conv_out_rep) * o.clock.time_step_zone_sec
                );
                put!(
                    surf_qdot_rad_out_rep,
                    v!(surf_qdot_rad_out_rep_per_area) * surface_area
                );
                put!(
                    surf_q_rad_out_report,
                    v!(surf_qdot_rad_out_rep) * o.clock.time_step_zone_sec
                );
                let id = o
                    .geometry
                    .final_surfaces
                    .as_ref()
                    .and_then(|r| r.get(i))
                    .and_then(|r| r.typed_id)
                    .ok_or(Error::Unavailable("report physical Source binding"))?;
                let dry_bulb = o
                    .physical
                    .iter()
                    .find(|r| r.surface_id == id)
                    .ok_or(Error::Unavailable("report same physical outdoor owner"))?
                    .outdoor_environment
                    .row
                    .dry_bulb_c;
                put!(
                    surf_q_air_ext_report,
                    surface_area * v!(surf_h_air_ext) * (v!(surf_temp_out) - dry_bulb)
                );
                put!(
                    surf_q_heat_emi_report,
                    v!(surf_q_air_ext_report) - v!(surf_qdot_conv_out_rep)
                );
            }
        }
        if *o.display_heat_emissions_summary {
            self.sum_surface_heat_emission = 0.0;
            for number in 1..=o.geometry.total_surfaces {
                let i = index(number)?;
                let row = o
                    .geometry
                    .final_surfaces
                    .as_ref()
                    .and_then(|r| r.get(i))
                    .ok_or(Error::Unavailable("report emission Surface"))?;
                if row.row.exterior_boundary == 0 {
                    self.sum_surface_heat_emission +=
                        value(&self.arrays.surf_q_heat_emi_report, i)? * o.clock.time_step_zone_sec;
                }
            }
        }
        for &(_, _, first, last, _, _, _) in &loops {
            if first <= last {
                return Err(Error::Unavailable(
                    "ReportSurfaceHeatBalance positive Window initial diffuse write6687",
                ));
            }
        }
        if o.geometry.any_movable_insulation {
            return Err(Error::Unavailable("ReportIntMovInsInsideSurfTemp6741"));
        }
        for &(_, _, _, _, first, last, _) in &loops {
            for number in first..=last {
                let i = index(number)?;
                let h = history_index(o, number)?;
                let surface_area = area(o.geometry, number)?;
                let a = &mut self.arrays;
                macro_rules! v {
                    ($field:ident) => {
                        value(&a.$field, i)?
                    };
                }
                macro_rules! put {
                    ($field:ident,$expression:expr) => {{
                        let x = $expression;
                        set(&mut a.$field, i, x)?;
                    }};
                }
                put!(
                    surf_opaq_sw_out_abs_total_report,
                    v!(surf_opaq_q_rad_sw_out_abs) * surface_area
                );
                put!(
                    surf_opaq_sw_out_abs_energy_report,
                    v!(surf_opaq_sw_out_abs_total_report) * o.clock.time_step_zone_sec
                );
                put!(
                    surf_qdot_rad_solar_in_rep,
                    v!(surf_qdot_rad_solar_in_rep_per_area) * surface_area
                );
                put!(
                    surf_q_rad_solar_in_rep,
                    v!(surf_qdot_rad_solar_in_rep) * o.clock.time_step_zone_sec
                );
                put!(
                    surf_qdot_rad_lights_in_rep,
                    v!(surf_qdot_rad_lights_in_per_area) * surface_area
                );
                put!(
                    surf_q_rad_lights_in_rep,
                    v!(surf_qdot_rad_lights_in_rep) * o.clock.time_step_zone_sec
                );
                o.radiation
                    .solar_timestep
                    .heat_balance
                    .surf_initial_dif_sol_in_abs_report
                    .set([i + 1], v!(surf_opaq_initial_dif_sol_in_abs) * surface_area)?;
                o.radiation
                    .solar_timestep
                    .heat_balance
                    .surf_sw_in_abs_total_report
                    .set([i + 1], v!(surf_opaq_q_rad_sw_in_abs) * surface_area)?;
                let f = &mut o.history.flux_reports;
                let x = scalar(&f.inside_flux, h)? * surface_area;
                write_scalar(&mut f.inside_conduction, h, x)?;
                put!(
                    surf_opaq_ins_face_cond_energy,
                    scalar(&f.inside_conduction, h)? * o.clock.time_step_zone_sec
                );
                put!(surf_opaq_ins_face_cond_gain_rep, 0.0);
                put!(surf_opaq_ins_face_cond_loss_rep, 0.0);
                if scalar(&f.inside_conduction, h)? >= 0.0 {
                    put!(
                        surf_opaq_ins_face_cond_gain_rep,
                        scalar(&f.inside_conduction, h)?
                    );
                } else {
                    put!(
                        surf_opaq_ins_face_cond_loss_rep,
                        -scalar(&f.inside_conduction, h)?
                    );
                }
                let x = surface_area * scalar(&f.outside_flux, h)?;
                write_scalar(&mut f.outside_conduction, h, x)?;
                put!(
                    surf_opaq_out_face_cond_energy,
                    scalar(&f.outside_conduction, h)? * o.clock.time_step_zone_sec
                );
                put!(surf_opaq_ext_face_cond_gain_rep, 0.0);
                put!(surf_opaq_ext_face_cond_loss_rep, 0.0);
                if scalar(&f.outside_conduction, h)? >= 0.0 {
                    put!(
                        surf_opaq_ext_face_cond_gain_rep,
                        scalar(&f.outside_conduction, h)?
                    );
                } else {
                    put!(
                        surf_opaq_ext_face_cond_loss_rep,
                        -scalar(&f.outside_conduction, h)?
                    );
                }
            }
            for number in first..=last {
                let i = index(number)?;
                let h = history_index(o, number)?;
                let a = &mut self.arrays;
                let f = &o.history.flux_reports;
                macro_rules! v {
                    ($field:ident) => {
                        value(&a.$field, i)?
                    };
                }
                macro_rules! put {
                    ($field:ident,$expression:expr) => {{
                        let x = $expression;
                        set(&mut a.$field, i, x)?;
                    }};
                }
                put!(
                    surf_opaq_avg_face_cond,
                    (scalar(&f.inside_conduction, h)? - scalar(&f.outside_conduction, h)?) / 2.0
                );
                put!(
                    surf_opaq_avg_face_cond_flux,
                    (scalar(&f.inside_flux, h)? - scalar(&f.outside_flux, h)?) / 2.0
                );
                put!(
                    surf_opaq_avg_face_cond_energy,
                    v!(surf_opaq_avg_face_cond) * o.clock.time_step_zone_sec
                );
                put!(surf_opaq_avg_face_cond_gain_rep, 0.0);
                put!(surf_opaq_avg_face_cond_loss_rep, 0.0);
                if v!(surf_opaq_avg_face_cond) >= 0.0 {
                    put!(
                        surf_opaq_avg_face_cond_gain_rep,
                        v!(surf_opaq_avg_face_cond)
                    );
                } else {
                    put!(
                        surf_opaq_avg_face_cond_loss_rep,
                        -v!(surf_opaq_avg_face_cond)
                    );
                }
                put!(
                    surf_opaq_storage_cond_flux,
                    -(scalar(&f.inside_flux, h)? + scalar(&f.outside_flux, h)?)
                );
                put!(
                    surf_opaq_storage_cond,
                    -(scalar(&f.inside_conduction, h)? + scalar(&f.outside_conduction, h)?)
                );
                put!(
                    surf_opaq_storage_cond_energy,
                    v!(surf_opaq_storage_cond) * o.clock.time_step_zone_sec
                );
                put!(surf_opaq_storage_cond_gain_rep, 0.0);
                put!(surf_opaq_storage_cond_loss_rep, 0.0);
                if v!(surf_opaq_storage_cond) >= 0.0 {
                    put!(surf_opaq_storage_cond_gain_rep, v!(surf_opaq_storage_cond));
                } else {
                    put!(surf_opaq_storage_cond_loss_rep, -v!(surf_opaq_storage_cond));
                }
            }
        }
        if o.globals.zone_sizing_calculation
            && o.globals.component_load_report_requested
            && !o.clock.warmup_flag
        {
            return Err(Error::Unavailable(
                "ReportSurfaceHeatBalance actual component load day/time row6832",
            ));
        }
        if o.globals.display_advanced_report_variables {
            for &(_, _, _, _, first, last, zone) in &loops {
                let sol = &mut o.radiation.solar_timestep.heat_balance;
                for number in first..=last {
                    let h = history_index_from_ids(o.geometry, &o.history.surface_ids, number)?;
                    sol.zone_opaq_surf_ins_face_cond.set(
                        [zone + 1],
                        sol.zone_opaq_surf_ins_face_cond.read([zone + 1])?
                            + scalar(&o.history.flux_reports.inside_conduction, h)?,
                    )?;
                    sol.zone_opaq_surf_ext_face_cond.set(
                        [zone + 1],
                        sol.zone_opaq_surf_ext_face_cond.read([zone + 1])?
                            + scalar(&o.history.flux_reports.outside_conduction, h)?,
                    )?;
                }
                if sol.zone_opaq_surf_ins_face_cond.read([zone + 1])? >= 0.0 {
                    sol.zone_opaq_surf_ins_face_cond_gain_rep.set(
                        [zone + 1],
                        sol.zone_opaq_surf_ins_face_cond.read([zone + 1])?,
                    )?;
                    sol.zn_opq_surf_ins_face_cond_gn_rep_enrg.set(
                        [zone + 1],
                        sol.zone_opaq_surf_ins_face_cond_gain_rep.read([zone + 1])?
                            * o.clock.time_step_zone_sec,
                    )?;
                } else {
                    sol.zone_opaq_surf_ins_face_cond_loss_rep.set(
                        [zone + 1],
                        -sol.zone_opaq_surf_ins_face_cond.read([zone + 1])?,
                    )?;
                    sol.zn_opq_surf_ins_face_cond_ls_rep_enrg.set(
                        [zone + 1],
                        sol.zone_opaq_surf_ins_face_cond_loss_rep.read([zone + 1])?
                            * o.clock.time_step_zone_sec,
                    )?;
                }
                if sol.zone_opaq_surf_ext_face_cond.read([zone + 1])? >= 0.0 {
                    sol.zone_opaq_surf_ext_face_cond_gain_rep.set(
                        [zone + 1],
                        sol.zone_opaq_surf_ext_face_cond.read([zone + 1])?,
                    )?;
                    sol.zn_opq_surf_ext_face_cond_gn_rep_enrg.set(
                        [zone + 1],
                        sol.zone_opaq_surf_ext_face_cond_gain_rep.read([zone + 1])?
                            * o.clock.time_step_zone_sec,
                    )?;
                } else {
                    sol.zone_opaq_surf_ext_face_cond_loss_rep.set(
                        [zone + 1],
                        -sol.zone_opaq_surf_ext_face_cond.read([zone + 1])?,
                    )?;
                    sol.zn_opq_surf_ext_face_cond_ls_rep_enrg.set(
                        [zone + 1],
                        sol.zone_opaq_surf_ext_face_cond_loss_rep.read([zone + 1])?
                            * o.clock.time_step_zone_sec,
                    )?;
                }
            }
        }
        Ok(())
    }
}
fn history_index_from_ids(
    g: &SurfaceGeometryInputState,
    ids: &[ep_model::SurfaceId],
    number: i32,
) -> Result<usize, Error> {
    let id = g
        .final_surfaces
        .as_ref()
        .and_then(|r| r.get(index(number).ok()?))
        .and_then(|r| r.typed_id)
        .ok_or(Error::Unavailable("advanced report Source identity"))?;
    ids.iter()
        .position(|x| *x == id)
        .ok_or(Error::Unavailable("advanced report same history identity"))
}
