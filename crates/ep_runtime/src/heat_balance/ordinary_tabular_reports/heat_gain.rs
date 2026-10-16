//! Genuine GatherHeatGainReport4464-5188 on the sole Source owners.
//! Positive ADU/radiant device first-record operations remain explicit frontiers.
use super::*;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SourceGainReport;
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::{
    SourceAirReport, SourceZoneAirPredictorCorrector,
};
use crate::heat_balance::ordinary_equipment::OrdinaryEquipment;
use crate::heat_balance::radiation::ordinary::solar_timestep::array::SourceSolarArray;
use crate::heat_balance::selected_heat_balance_input::zone::ZonePredefined;
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual HeatGain one-based ordinal"))
}
fn allocate(a: &mut SourceSolarArray<f64, 1>, number: i32) -> Result<(), Error> {
    a.dimension(
        [usize::try_from(number)
            .map_err(|_| Error::Unavailable("actual HeatGain allocation bounds"))?],
        None,
    )
}
fn predefined(z: &SelectedZoneInputOwners, number: i32) -> Result<&ZonePredefined, Error> {
    z.zone_predefined
        .as_ref()
        .and_then(|v| v.get(index(number).ok()?))
        .ok_or(Error::Unavailable("same actual HeatGain ZonePreDefRep"))
}
fn predefined_mut(
    z: &mut SelectedZoneInputOwners,
    number: i32,
) -> Result<&mut ZonePredefined, Error> {
    z.zone_predefined
        .as_mut()
        .and_then(|v| v.get_mut(index(number).ok()?))
        .ok_or(Error::Unavailable("same actual HeatGain ZonePreDefRep"))
}
fn zone_multiplier(z: &SelectedZoneInputOwners, number: i32) -> Result<f64, Error> {
    let row = z
        .zones
        .as_ref()
        .and_then(|v| v.get(index(number).ok()?))
        .ok_or(Error::Unavailable("same actual HeatGain Zone"))?;
    Ok(f64::from(
        row.multiplier
            .checked_mul(row.list_multiplier)
            .ok_or(Error::Unavailable(
                "actual HeatGain native multiplier int product",
            ))?,
    ))
}
fn gain_report(g: &SelectedInternalHeatGains, number: i32) -> Result<&SourceGainReport, Error> {
    g.allocations
        .zone_report
        .as_ref()
        .and_then(|v| v.get(index(number).ok()?))
        .ok_or(Error::Unavailable("same actual HeatGain ZoneRpt"))
}
fn air_report(c: &SourceZoneAirPredictorCorrector, number: i32) -> Result<&SourceAirReport, Error> {
    c.reports
        .zones
        .as_ref()
        .and_then(|v| v.get(index(number).ok()?))
        .ok_or(Error::Unavailable("same actual HeatGain ZnAirRpt"))
}
fn air_distribution_unit_count(e: &OrdinaryEquipment) -> Result<i32, Error> {
    i32::try_from(
        e.source_air_distribution_units()
            .map_or(0, <[std::convert::Infallible]>::len),
    )
    .map_err(|_| Error::Unavailable("actual HeatGain ADU size int domain"))
}
impl SourceTabularReports {
    pub(super) fn gather_source_heat_gain_report(
        &mut self,
        time_step: TimeStepKind,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        let time_step_sys_sec = o.hvac.time_step_sys_seconds;
        if !o.clock.do_weath_sim {
            return Ok(());
        }
        let report = *o
            .registry
            .named_indices
            .get("pdrSensibleGain")
            .ok_or(Error::Unavailable("same actual pdrSensibleGain4538"))?;
        let show = o
            .registry
            .reports
            .as_ref()
            .and_then(|v| v.get(index(report).ok()?))
            .ok_or(Error::Unavailable(
                "same actual sensible gain reportName4538",
            ))?
            .show;
        if !show {
            return Ok(());
        }
        if time_step == TimeStepKind::Zone {
            return Ok(());
        }
        let s = &mut self.input_state;
        if s.gather_heat_gain_report_first_time {
            allocate(&mut s.radiant_heat, o.globals.num_of_zones)?;
            allocate(&mut s.radiant_cool, o.globals.num_of_zones)?;
            allocate(&mut s.atu_heat, o.globals.num_of_zones)?;
            allocate(&mut s.atu_cool, o.globals.num_of_zones)?;
            let value_5 = false;
            s.gather_heat_gain_report_first_time = value_5;
        }
        s.radiant_heat.fill(0.0)?;
        s.radiant_cool.fill(0.0)?;
        s.atu_heat.fill(0.0)?;
        s.atu_cool.fill(0.0)?;
        s.iunit_ghgr = 1;
        if s.iunit_ghgr <= air_distribution_unit_count(o.equipment)? {
            return Err(Error::Unavailable(
                "actual GatherHeatGain AirDistUnit.ZoneNum4569",
            ));
        }
        let value_10 = o.hvac.time_step_sys / o.clock.time_step_zone;
        s.time_step_ratio = value_10;
        s.i_zone_ghgr = 1;
        while s.i_zone_ghgr <= o.globals.num_of_zones {
            let mult = zone_multiplier(o.zones, s.i_zone_ghgr)?;
            let value_12 =
                gain_report(o.gains, s.i_zone_ghgr)?.people_sen_gain * mult * s.time_step_ratio;
            predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_peopl_add += value_12;
            let value_13 =
                gain_report(o.gains, s.i_zone_ghgr)?.lts_tot_gain * mult * s.time_step_ratio;
            predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_lite_add += value_13;
            let zone_equipment_heat_or_cool = air_report(o.controller, s.i_zone_ghgr)?
                .sum_m_cp_d_tsystem
                + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult
                - s.atu_heat.read([index(s.i_zone_ghgr)? + 1])?
                - s.atu_cool.read([index(s.i_zone_ghgr)? + 1])?;
            if zone_equipment_heat_or_cool > 0.0 {
                let value_15 = zone_equipment_heat_or_cool * time_step_sys_sec;
                predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_zone_eq_ht += value_15;
            } else {
                let value_16 = zone_equipment_heat_or_cool * time_step_sys_sec;
                predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_zone_eq_cl += value_16;
            }
            if air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones > 0.0 {
                let value_17 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones
                    * mult
                    * time_step_sys_sec;
                predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_iza_add += value_17;
            } else {
                let value_18 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones
                    * mult
                    * time_step_sys_sec;
                predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_iza_rem += value_18;
            }
            let value_19 = o
                .radiation
                .solar_timestep
                .heat_balance
                .zone_win_heat_gain_rep_energy
                .read([index(s.i_zone_ghgr)? + 1])?
                * mult
                * s.time_step_ratio;
            predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_wind_add += value_19;
            let value_20 = o
                .radiation
                .solar_timestep
                .heat_balance
                .zone_win_heat_loss_rep_energy
                .read([index(s.i_zone_ghgr)? + 1])?
                * mult
                * s.time_step_ratio;
            predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_wind_rem -= value_20;
            if air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil > 0.0 {
                let value_21 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil
                    * mult
                    * time_step_sys_sec;
                predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_infil_add += value_21;
            } else {
                let value_22 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil
                    * mult
                    * time_step_sys_sec;
                predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_infil_rem += value_22;
            }
            let value_23 = (gain_report(o.gains, s.i_zone_ghgr)?.elec_rad_gain
                + gain_report(o.gains, s.i_zone_ghgr)?.gas_rad_gain
                + gain_report(o.gains, s.i_zone_ghgr)?.hw_rad_gain
                + gain_report(o.gains, s.i_zone_ghgr)?.steam_rad_gain
                + gain_report(o.gains, s.i_zone_ghgr)?.other[0]
                + gain_report(o.gains, s.i_zone_ghgr)?.elec_con_gain
                + gain_report(o.gains, s.i_zone_ghgr)?.gas_con_gain
                + gain_report(o.gains, s.i_zone_ghgr)?.hw_con_gain
                + gain_report(o.gains, s.i_zone_ghgr)?.steam_con_gain
                + gain_report(o.gains, s.i_zone_ghgr)?.other[2])
                * s.time_step_ratio;
            s.eqp_sens_ghgr = value_23;
            if s.eqp_sens_ghgr > 0.0 {
                let value_24 = s.eqp_sens_ghgr * mult;
                predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_equip_add += value_24;
            } else {
                let value_25 = s.eqp_sens_ghgr * mult;
                predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_equip_rem += value_25;
            }
            s.i_zone_ghgr = s
                .i_zone_ghgr
                .checked_add(1)
                .ok_or(Error::Unavailable("actual GatherHeatGain loop int domain"))?;
        }
        let value_26 = 0;
        s.cur_zone_ghgr = value_26;
        s.i_radiant_ghgr = 1;
        if s.i_radiant_ghgr
            <= o.surface_averages
                .ventilated_slab
                .number_of_ventilated_slabs
        {
            return Err(Error::Unavailable(
                "actual GatherHeatGain VentSlab.ZonePtr4651",
            ));
        }
        s.i_radiant_ghgr = 1;
        if s.i_radiant_ghgr
            <= o.surface_averages
                .low_temperature
                .number_of_hydronic_systems
        {
            return Err(Error::Unavailable(
                "actual GatherHeatGain HydrRadSys.ZonePtr4666",
            ));
        }
        s.i_radiant_ghgr = 1;
        if s.i_radiant_ghgr
            <= o.surface_averages
                .low_temperature
                .number_of_constant_flow_systems
        {
            return Err(Error::Unavailable(
                "actual GatherHeatGain CFloRadSys.ZonePtr4681",
            ));
        }
        s.i_radiant_ghgr = 1;
        if s.i_radiant_ghgr
            <= o.surface_averages
                .low_temperature
                .number_of_electric_systems
        {
            return Err(Error::Unavailable(
                "actual GatherHeatGain ElecRadSys.ZonePtr4696",
            ));
        }
        s.i_zone_ghgr = 1;
        while s.i_zone_ghgr <= o.globals.num_of_zones {
            let value_27 = predefined(o.zones, s.i_zone_ghgr)?.shgs_an_peopl_add
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_lite_add
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_zone_eq_ht
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_zone_eq_cl
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_hvac_atu_ht
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_hvac_atu_cl
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_iza_add
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_iza_rem
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_wind_add
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_wind_rem
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_infil_add
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_infil_rem
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_equip_add
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_equip_rem
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_surf_ht
                + predefined(o.zones, s.i_zone_ghgr)?.shgs_an_surf_cl;
            s.total_ghgr = value_27;
            let value_28 = -s.total_ghgr;
            s.total_ghgr = value_28;
            if s.total_ghgr > 0.0 {
                let value_29 = s.total_ghgr;
                predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_other_add = value_29;
            } else {
                let value_30 = s.total_ghgr;
                predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_an_other_rem = value_30;
            }
            s.i_zone_ghgr = s
                .i_zone_ghgr
                .checked_add(1)
                .ok_or(Error::Unavailable("actual GatherHeatGain loop int domain"))?;
        }
        s.i_zone_ghgr = 1;
        while s.i_zone_ghgr <= o.globals.num_of_zones {
            let mult = zone_multiplier(o.zones, s.i_zone_ghgr)?;
            if (air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                + s.radiant_heat.read([index(s.i_zone_ghgr)? + 1])?
                + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult)
                > 0.0
            {
                if (air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                    + s.radiant_heat.read([index(s.i_zone_ghgr)? + 1])?
                    + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult)
                    > predefined(o.zones, s.i_zone_ghgr)?.ht_peak
                {
                    let value_32 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                        + s.radiant_heat.read([index(s.i_zone_ghgr)? + 1])?
                        + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.ht_peak = value_32;
                    s.timestep_time_stamp_ghgr = super::source_demand::peak_timestamp(o)?;
                    let value_35 = s.timestep_time_stamp_ghgr;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.ht_pt_time_stamp = value_35;
                    let value_36 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                        + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_hvac_ht = value_36;
                    let value_37 = 0.0;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_hvac_cl = value_37;
                    let value_38 = s.radiant_heat.read([index(s.i_zone_ghgr)? + 1])?;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_surf_ht = value_38;
                    let value_39 = s.radiant_cool.read([index(s.i_zone_ghgr)? + 1])?;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_surf_cl = value_39;
                    let value_40 = s.atu_heat.read([index(s.i_zone_ghgr)? + 1])?;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_hvac_atu_ht = value_40;
                    let value_41 = s.atu_cool.read([index(s.i_zone_ghgr)? + 1])?;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_hvac_atu_cl = value_41;
                    let value_42 = gain_report(o.gains, s.i_zone_ghgr)?.people_sen_gain_rate * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_peopl_add = value_42;
                    let value_43 = gain_report(o.gains, s.i_zone_ghgr)?.lts_tot_gain_rate * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_lite_add = value_43;
                    let value_44 = gain_report(o.gains, s.i_zone_ghgr)?.elec_rad_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.gas_rad_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.hw_rad_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.steam_rad_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.other[1]
                        + gain_report(o.gains, s.i_zone_ghgr)?.elec_con_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.gas_con_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.hw_con_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.steam_con_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.other[3];
                    s.eqp_sens_ghgr = value_44;
                    if s.eqp_sens_ghgr > 0.0 {
                        let value_45 = s.eqp_sens_ghgr * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_equip_add = value_45;
                        let value_46 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_equip_rem = value_46;
                    } else {
                        let value_47 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_equip_add = value_47;
                        let value_48 = s.eqp_sens_ghgr * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_equip_rem = value_48;
                    }
                    let value_49 = o
                        .radiation
                        .solar_timestep
                        .heat_balance
                        .zone_win_heat_gain_rep
                        .read([index(s.i_zone_ghgr)? + 1])?
                        * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_wind_add = value_49;
                    let value_50 = -o
                        .radiation
                        .solar_timestep
                        .heat_balance
                        .zone_win_heat_loss_rep
                        .read([index(s.i_zone_ghgr)? + 1])?
                        * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_wind_rem = value_50;
                    if air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones > 0.0 {
                        let value_51 =
                            air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_iza_add = value_51;
                        let value_52 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_iza_rem = value_52;
                    } else {
                        let value_53 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_iza_add = value_53;
                        let value_54 =
                            air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_iza_rem = value_54;
                    }
                    if air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil > 0.0 {
                        let value_55 =
                            air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_infil_add = value_55;
                        let value_56 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_infil_rem = value_56;
                    } else {
                        let value_57 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_infil_add = value_57;
                        let value_58 =
                            air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_infil_rem = value_58;
                    }
                    let value_59 = predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_peopl_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_lite_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_hvac_ht
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_hvac_cl
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_iza_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_iza_rem
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_wind_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_wind_rem
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_infil_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_infil_rem
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_equip_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_equip_rem
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_surf_ht
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_ht_surf_cl;
                    s.total_ghgr = value_59;
                    let value_60 = -s.total_ghgr;
                    s.total_ghgr = value_60;
                    if s.total_ghgr > 0.0 {
                        let value_61 = s.total_ghgr;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_other_add = value_61;
                        let value_62 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_other_rem = value_62;
                    } else {
                        let value_63 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_other_add = value_63;
                        let value_64 = s.total_ghgr;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_ht_other_rem = value_64;
                    }
                }
            } else {
                if (air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                    + s.radiant_cool.read([index(s.i_zone_ghgr)? + 1])?
                    + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult)
                    < predefined(o.zones, s.i_zone_ghgr)?.cl_peak
                {
                    let value_65 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                        + s.radiant_cool.read([index(s.i_zone_ghgr)? + 1])?
                        + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.cl_peak = value_65;
                    s.timestep_time_stamp_ghgr = super::source_demand::peak_timestamp(o)?;
                    let value_68 = s.timestep_time_stamp_ghgr;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.cl_pt_time_stamp = value_68;
                    let value_69 = 0.0;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_hvac_ht = value_69;
                    let value_70 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                        + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_hvac_cl = value_70;
                    let value_71 = s.radiant_heat.read([index(s.i_zone_ghgr)? + 1])?;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_surf_ht = value_71;
                    let value_72 = s.radiant_cool.read([index(s.i_zone_ghgr)? + 1])?;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_surf_cl = value_72;
                    let value_73 = s.atu_heat.read([index(s.i_zone_ghgr)? + 1])?;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_hvac_atu_ht = value_73;
                    let value_74 = s.atu_cool.read([index(s.i_zone_ghgr)? + 1])?;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_hvac_atu_cl = value_74;
                    let value_75 = gain_report(o.gains, s.i_zone_ghgr)?.people_sen_gain_rate * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_peopl_add = value_75;
                    let value_76 = gain_report(o.gains, s.i_zone_ghgr)?.lts_tot_gain_rate * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_lite_add = value_76;
                    let value_77 = gain_report(o.gains, s.i_zone_ghgr)?.elec_rad_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.gas_rad_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.hw_rad_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.steam_rad_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.other[1]
                        + gain_report(o.gains, s.i_zone_ghgr)?.elec_con_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.gas_con_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.hw_con_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.steam_con_gain_rate
                        + gain_report(o.gains, s.i_zone_ghgr)?.other[3];
                    s.eqp_sens_ghgr = value_77;
                    if s.eqp_sens_ghgr > 0.0 {
                        let value_78 = s.eqp_sens_ghgr * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_equip_add = value_78;
                        let value_79 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_equip_rem = value_79;
                    } else {
                        let value_80 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_equip_add = value_80;
                        let value_81 = s.eqp_sens_ghgr * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_equip_rem = value_81;
                    }
                    let value_82 = o
                        .radiation
                        .solar_timestep
                        .heat_balance
                        .zone_win_heat_gain_rep
                        .read([index(s.i_zone_ghgr)? + 1])?
                        * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_wind_add = value_82;
                    let value_83 = -o
                        .radiation
                        .solar_timestep
                        .heat_balance
                        .zone_win_heat_loss_rep
                        .read([index(s.i_zone_ghgr)? + 1])?
                        * mult;
                    predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_wind_rem = value_83;
                    if air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones > 0.0 {
                        let value_84 =
                            air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_iza_add = value_84;
                        let value_85 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_iza_rem = value_85;
                    } else {
                        let value_86 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_iza_add = value_86;
                        let value_87 =
                            air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_iza_rem = value_87;
                    }
                    if air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil > 0.0 {
                        let value_88 =
                            air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_infil_add = value_88;
                        let value_89 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_infil_rem = value_89;
                    } else {
                        let value_90 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_infil_add = value_90;
                        let value_91 =
                            air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil * mult;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_infil_rem = value_91;
                    }
                    let value_92 = predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_peopl_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_lite_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_hvac_ht
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_hvac_cl
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_iza_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_iza_rem
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_wind_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_wind_rem
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_infil_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_infil_rem
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_equip_add
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_equip_rem
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_surf_ht
                        + predefined(o.zones, s.i_zone_ghgr)?.shgs_cl_surf_cl;
                    s.total_ghgr = value_92;
                    let value_93 = -s.total_ghgr;
                    s.total_ghgr = value_93;
                    if s.total_ghgr > 0.0 {
                        let value_94 = s.total_ghgr;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_other_add = value_94;
                        let value_95 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_other_rem = value_95;
                    } else {
                        let value_96 = 0.0;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_other_add = value_96;
                        let value_97 = s.total_ghgr;
                        predefined_mut(o.zones, s.i_zone_ghgr)?.shgs_cl_other_rem = value_97;
                    }
                }
            }
            s.i_zone_ghgr = s
                .i_zone_ghgr
                .checked_add(1)
                .ok_or(Error::Unavailable("actual GatherHeatGain loop int domain"))?;
        }
        let value_98 = 0.0;
        s.bldg_ht_pk = value_98;
        let value_99 = 0.0;
        s.bldg_cl_pk = value_99;
        s.i_zone_ghgr = 1;
        while s.i_zone_ghgr <= o.globals.num_of_zones {
            let mult = zone_multiplier(o.zones, s.i_zone_ghgr)?;
            if (air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                + s.radiant_heat.read([index(s.i_zone_ghgr)? + 1])?
                + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult)
                > 0.0
            {
                let value_101 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                    + s.radiant_heat.read([index(s.i_zone_ghgr)? + 1])?
                    + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult;
                s.bldg_ht_pk += value_101;
            } else {
                let value_102 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                    + s.radiant_cool.read([index(s.i_zone_ghgr)? + 1])?
                    + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult;
                s.bldg_cl_pk += value_102;
            }
            s.i_zone_ghgr = s
                .i_zone_ghgr
                .checked_add(1)
                .ok_or(Error::Unavailable("actual GatherHeatGain loop int domain"))?;
        }
        if s.bldg_ht_pk > o.zones.building_predefined.ht_peak {
            let value_103 = s.bldg_ht_pk;
            o.zones.building_predefined.ht_peak = value_103;
            s.timestep_time_stamp_ghgr = super::source_demand::peak_timestamp(o)?;
            let value_106 = s.timestep_time_stamp_ghgr;
            o.zones.building_predefined.ht_pt_time_stamp = value_106;
            let value_107 = 0.0;
            o.zones.building_predefined.shgs_ht_hvac_ht = value_107;
            let value_108 = 0.0;
            o.zones.building_predefined.shgs_ht_hvac_cl = value_108;
            let value_109 = 0.0;
            o.zones.building_predefined.shgs_ht_hvac_atu_ht = value_109;
            let value_110 = 0.0;
            o.zones.building_predefined.shgs_ht_hvac_atu_cl = value_110;
            let value_111 = 0.0;
            o.zones.building_predefined.shgs_ht_surf_ht = value_111;
            let value_112 = 0.0;
            o.zones.building_predefined.shgs_ht_surf_cl = value_112;
            let value_113 = 0.0;
            o.zones.building_predefined.shgs_ht_peopl_add = value_113;
            let value_114 = 0.0;
            o.zones.building_predefined.shgs_ht_lite_add = value_114;
            let value_115 = 0.0;
            o.zones.building_predefined.shgs_ht_equip_add = value_115;
            let value_116 = 0.0;
            o.zones.building_predefined.shgs_ht_wind_add = value_116;
            let value_117 = 0.0;
            o.zones.building_predefined.shgs_ht_iza_add = value_117;
            let value_118 = 0.0;
            o.zones.building_predefined.shgs_ht_infil_add = value_118;
            let value_119 = 0.0;
            o.zones.building_predefined.shgs_ht_other_add = value_119;
            let value_120 = 0.0;
            o.zones.building_predefined.shgs_ht_equip_rem = value_120;
            let value_121 = 0.0;
            o.zones.building_predefined.shgs_ht_wind_rem = value_121;
            let value_122 = 0.0;
            o.zones.building_predefined.shgs_ht_iza_rem = value_122;
            let value_123 = 0.0;
            o.zones.building_predefined.shgs_ht_infil_rem = value_123;
            let value_124 = 0.0;
            o.zones.building_predefined.shgs_ht_other_rem = value_124;
            s.i_zone_ghgr = 1;
            while s.i_zone_ghgr <= o.globals.num_of_zones {
                let mult = zone_multiplier(o.zones, s.i_zone_ghgr)?;
                let value_126 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                    + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult;
                o.zones.building_predefined.shgs_ht_hvac_ht += value_126;
                let value_127 = s.radiant_heat.read([index(s.i_zone_ghgr)? + 1])?;
                o.zones.building_predefined.shgs_ht_surf_ht += value_127;
                let value_128 = s.radiant_cool.read([index(s.i_zone_ghgr)? + 1])?;
                o.zones.building_predefined.shgs_ht_surf_cl += value_128;
                let value_129 = s.atu_heat.read([index(s.i_zone_ghgr)? + 1])?;
                o.zones.building_predefined.shgs_ht_hvac_atu_ht += value_129;
                let value_130 = s.atu_cool.read([index(s.i_zone_ghgr)? + 1])?;
                o.zones.building_predefined.shgs_ht_hvac_atu_cl += value_130;
                let value_131 = gain_report(o.gains, s.i_zone_ghgr)?.people_sen_gain_rate * mult;
                o.zones.building_predefined.shgs_ht_peopl_add += value_131;
                let value_132 = gain_report(o.gains, s.i_zone_ghgr)?.lts_tot_gain_rate * mult;
                o.zones.building_predefined.shgs_ht_lite_add += value_132;
                let value_133 = gain_report(o.gains, s.i_zone_ghgr)?.elec_rad_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.gas_rad_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.hw_rad_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.steam_rad_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.other[1]
                    + gain_report(o.gains, s.i_zone_ghgr)?.elec_con_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.gas_con_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.hw_con_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.steam_con_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.other[3];
                s.eqp_sens_ghgr = value_133;
                if s.eqp_sens_ghgr > 0.0 {
                    let value_134 = s.eqp_sens_ghgr * mult;
                    o.zones.building_predefined.shgs_ht_equip_add += value_134;
                } else {
                    let value_135 = s.eqp_sens_ghgr * mult;
                    o.zones.building_predefined.shgs_ht_equip_rem += value_135;
                }
                let value_136 = o
                    .radiation
                    .solar_timestep
                    .heat_balance
                    .zone_win_heat_gain_rep
                    .read([index(s.i_zone_ghgr)? + 1])?
                    * mult;
                o.zones.building_predefined.shgs_ht_wind_add += value_136;
                let value_137 = o
                    .radiation
                    .solar_timestep
                    .heat_balance
                    .zone_win_heat_loss_rep
                    .read([index(s.i_zone_ghgr)? + 1])?
                    * mult;
                o.zones.building_predefined.shgs_ht_wind_rem -= value_137;
                if air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones > 0.0 {
                    let value_138 =
                        air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones * mult;
                    o.zones.building_predefined.shgs_ht_iza_add += value_138;
                } else {
                    let value_139 =
                        air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones * mult;
                    o.zones.building_predefined.shgs_ht_iza_rem += value_139;
                }
                if air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil > 0.0 {
                    let value_140 =
                        air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil * mult;
                    o.zones.building_predefined.shgs_ht_infil_add += value_140;
                } else {
                    let value_141 =
                        air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil * mult;
                    o.zones.building_predefined.shgs_ht_infil_rem += value_141;
                }
                s.i_zone_ghgr = s
                    .i_zone_ghgr
                    .checked_add(1)
                    .ok_or(Error::Unavailable("actual GatherHeatGain loop int domain"))?;
            }
            let value_142 = o.zones.building_predefined.shgs_ht_peopl_add
                + o.zones.building_predefined.shgs_ht_lite_add
                + o.zones.building_predefined.shgs_ht_hvac_ht
                + o.zones.building_predefined.shgs_ht_hvac_cl
                + o.zones.building_predefined.shgs_ht_iza_add
                + o.zones.building_predefined.shgs_ht_iza_rem
                + o.zones.building_predefined.shgs_ht_wind_add
                + o.zones.building_predefined.shgs_ht_wind_rem
                + o.zones.building_predefined.shgs_ht_infil_add
                + o.zones.building_predefined.shgs_ht_infil_rem
                + o.zones.building_predefined.shgs_ht_equip_add
                + o.zones.building_predefined.shgs_ht_equip_rem
                + o.zones.building_predefined.shgs_ht_surf_ht
                + o.zones.building_predefined.shgs_ht_surf_cl;
            s.total_ghgr = value_142;
            let value_143 = -s.total_ghgr;
            s.total_ghgr = value_143;
            if s.total_ghgr > 0.0 {
                let value_144 = s.total_ghgr;
                o.zones.building_predefined.shgs_ht_other_add += value_144;
            } else {
                let value_145 = s.total_ghgr;
                o.zones.building_predefined.shgs_ht_other_rem += value_145;
            }
        }
        if s.bldg_cl_pk < o.zones.building_predefined.cl_peak {
            let value_146 = s.bldg_cl_pk;
            o.zones.building_predefined.cl_peak = value_146;
            s.timestep_time_stamp_ghgr = super::source_demand::peak_timestamp(o)?;
            let value_149 = s.timestep_time_stamp_ghgr;
            o.zones.building_predefined.cl_pt_time_stamp = value_149;
            let value_150 = 0.0;
            o.zones.building_predefined.shgs_cl_hvac_ht = value_150;
            let value_151 = 0.0;
            o.zones.building_predefined.shgs_cl_hvac_cl = value_151;
            let value_152 = 0.0;
            o.zones.building_predefined.shgs_cl_surf_ht = value_152;
            let value_153 = 0.0;
            o.zones.building_predefined.shgs_cl_surf_cl = value_153;
            let value_154 = 0.0;
            o.zones.building_predefined.shgs_cl_hvac_atu_ht = value_154;
            let value_155 = 0.0;
            o.zones.building_predefined.shgs_cl_hvac_atu_cl = value_155;
            let value_156 = 0.0;
            o.zones.building_predefined.shgs_cl_peopl_add = value_156;
            let value_157 = 0.0;
            o.zones.building_predefined.shgs_cl_lite_add = value_157;
            let value_158 = 0.0;
            o.zones.building_predefined.shgs_cl_equip_add = value_158;
            let value_159 = 0.0;
            o.zones.building_predefined.shgs_cl_wind_add = value_159;
            let value_160 = 0.0;
            o.zones.building_predefined.shgs_cl_iza_add = value_160;
            let value_161 = 0.0;
            o.zones.building_predefined.shgs_cl_infil_add = value_161;
            let value_162 = 0.0;
            o.zones.building_predefined.shgs_cl_other_add = value_162;
            let value_163 = 0.0;
            o.zones.building_predefined.shgs_cl_equip_rem = value_163;
            let value_164 = 0.0;
            o.zones.building_predefined.shgs_cl_wind_rem = value_164;
            let value_165 = 0.0;
            o.zones.building_predefined.shgs_cl_iza_rem = value_165;
            let value_166 = 0.0;
            o.zones.building_predefined.shgs_cl_infil_rem = value_166;
            let value_167 = 0.0;
            o.zones.building_predefined.shgs_cl_other_rem = value_167;
            s.i_zone_ghgr = 1;
            while s.i_zone_ghgr <= o.globals.num_of_zones {
                let mult = zone_multiplier(o.zones, s.i_zone_ghgr)?;
                let value_169 = air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tsystem
                    + air_report(o.controller, s.i_zone_ghgr)?.sum_non_air_system * mult;
                o.zones.building_predefined.shgs_cl_hvac_cl += value_169;
                let value_170 = s.radiant_heat.read([index(s.i_zone_ghgr)? + 1])?;
                o.zones.building_predefined.shgs_cl_surf_ht += value_170;
                let value_171 = s.radiant_cool.read([index(s.i_zone_ghgr)? + 1])?;
                o.zones.building_predefined.shgs_cl_surf_cl += value_171;
                let value_172 = s.atu_heat.read([index(s.i_zone_ghgr)? + 1])?;
                o.zones.building_predefined.shgs_cl_hvac_atu_ht += value_172;
                let value_173 = s.atu_cool.read([index(s.i_zone_ghgr)? + 1])?;
                o.zones.building_predefined.shgs_cl_hvac_atu_cl += value_173;
                let value_174 = gain_report(o.gains, s.i_zone_ghgr)?.people_sen_gain_rate * mult;
                o.zones.building_predefined.shgs_cl_peopl_add += value_174;
                let value_175 = gain_report(o.gains, s.i_zone_ghgr)?.lts_tot_gain_rate * mult;
                o.zones.building_predefined.shgs_cl_lite_add += value_175;
                let value_176 = gain_report(o.gains, s.i_zone_ghgr)?.elec_rad_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.gas_rad_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.hw_rad_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.steam_rad_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.other[1]
                    + gain_report(o.gains, s.i_zone_ghgr)?.elec_con_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.gas_con_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.hw_con_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.steam_con_gain_rate
                    + gain_report(o.gains, s.i_zone_ghgr)?.other[3];
                s.eqp_sens_ghgr = value_176;
                if s.eqp_sens_ghgr > 0.0 {
                    let value_177 = s.eqp_sens_ghgr * mult;
                    o.zones.building_predefined.shgs_cl_equip_add += value_177;
                } else {
                    let value_178 = s.eqp_sens_ghgr * mult;
                    o.zones.building_predefined.shgs_cl_equip_rem += value_178;
                }
                let value_179 = o
                    .radiation
                    .solar_timestep
                    .heat_balance
                    .zone_win_heat_gain_rep
                    .read([index(s.i_zone_ghgr)? + 1])?
                    * mult;
                o.zones.building_predefined.shgs_cl_wind_add += value_179;
                let value_180 = o
                    .radiation
                    .solar_timestep
                    .heat_balance
                    .zone_win_heat_loss_rep
                    .read([index(s.i_zone_ghgr)? + 1])?
                    * mult;
                o.zones.building_predefined.shgs_cl_wind_rem -= value_180;
                if air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones > 0.0 {
                    let value_181 =
                        air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones * mult;
                    o.zones.building_predefined.shgs_cl_iza_add += value_181;
                } else {
                    let value_182 =
                        air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_d_tzones * mult;
                    o.zones.building_predefined.shgs_cl_iza_rem += value_182;
                }
                if air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil > 0.0 {
                    let value_183 =
                        air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil * mult;
                    o.zones.building_predefined.shgs_cl_infil_add += value_183;
                } else {
                    let value_184 =
                        air_report(o.controller, s.i_zone_ghgr)?.sum_m_cp_dt_infil * mult;
                    o.zones.building_predefined.shgs_cl_infil_rem += value_184;
                }
                s.i_zone_ghgr = s
                    .i_zone_ghgr
                    .checked_add(1)
                    .ok_or(Error::Unavailable("actual GatherHeatGain loop int domain"))?;
            }
            let value_185 = o.zones.building_predefined.shgs_cl_peopl_add
                + o.zones.building_predefined.shgs_cl_lite_add
                + o.zones.building_predefined.shgs_cl_hvac_ht
                + o.zones.building_predefined.shgs_cl_hvac_cl
                + o.zones.building_predefined.shgs_cl_iza_add
                + o.zones.building_predefined.shgs_cl_iza_rem
                + o.zones.building_predefined.shgs_cl_wind_add
                + o.zones.building_predefined.shgs_cl_wind_rem
                + o.zones.building_predefined.shgs_cl_infil_add
                + o.zones.building_predefined.shgs_cl_infil_rem
                + o.zones.building_predefined.shgs_cl_equip_add
                + o.zones.building_predefined.shgs_cl_equip_rem
                + o.zones.building_predefined.shgs_cl_surf_ht
                + o.zones.building_predefined.shgs_cl_surf_cl;
            s.total_ghgr = value_185;
            let value_186 = -s.total_ghgr;
            s.total_ghgr = value_186;
            if s.total_ghgr > 0.0 {
                let value_187 = s.total_ghgr;
                o.zones.building_predefined.shgs_cl_other_add += value_187;
            } else {
                let value_188 = s.total_ghgr;
                o.zones.building_predefined.shgs_cl_other_rem += value_188;
            }
        }
        Ok(())
    }
}
