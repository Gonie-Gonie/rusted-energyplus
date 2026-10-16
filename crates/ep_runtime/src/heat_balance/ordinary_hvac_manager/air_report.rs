//! Genuine HVAC ReportAirHeatBalance2345-2882 on the existing live report cells.
//! Positive simple-air/fan/node records stop before their first unported operation.
use super::{Error, SourceHvacManager, SourceHvacOwners, index};
use crate::heat_balance::ordinary_air_heat_balance::{
    SourceZoneSpaceHeatBalanceData,
    predictor_corrector::{SourceAirReport, simple_air::SourceSimpleAir},
};
use crate::heat_balance::ordinary_equipment::SourceZoneEquipmentConfigRef;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::psychrometrics::{
    energyplus_psy_cp_air_fn_w, energyplus_psy_hg_air_fn_w_tdb, energyplus_psy_rho_air_fn_pb_tdb_w,
};
use crate::weather::day::WeatherEnvironmentState;
use std::convert::Infallible;

struct SourceAirReportContext<'a> {
    zones: &'a SelectedZoneInputOwners,
    environment: &'a WeatherEnvironmentState,
    hvac: &'a SelectedHvacGlobals,
    simple: &'a mut SourceSimpleAir,
    afn_control_type: &'a i32,
    fans: &'a Option<Vec<Infallible>>,
}

fn report_air_heat_bal1(
    report: &mut SourceAirReport,
    _equipment_config: &SourceZoneEquipmentConfigRef<'_>,
    zone_number: i32,
    context: &SourceAirReportContext<'_>,
) -> Result<(), Error> {
    let _cp_air = energyplus_psy_cp_air_fn_w(context.environment.current_weather.out_hum_rat);
    let _out_db = context
        .zones
        .zones
        .as_ref()
        .and_then(|v| v.get(index(zone_number).ok()?))
        .ok_or(Error::Unavailable("actual reportAirHeatBal1 Zone2412"))?
        .dry_bulb;
    let _ads_correction_factor = 1.0;
    if *context.afn_control_type == 2 {
        return Err(Error::Unavailable(
            "actual reportAirHeatBal1 ZoneEquipAvail2415 before AFN correction",
        ));
    }
    report.exh_total_loss = 0.0;
    report.exh_sensi_loss = 0.0;
    if context.fans.as_ref().is_some_and(|v| !v.is_empty()) {
        return Err(Error::Unavailable(
            "actual Fan pointer2426 before type read",
        ));
    }
    report.exh_latent_loss = report.exh_total_loss - report.exh_sensi_loss;
    Ok(())
}

fn report_air_heat_bal2(
    report: &mut SourceAirReport,
    equipment_config: &SourceZoneEquipmentConfigRef<'_>,
    air: &SourceZoneSpaceHeatBalanceData,
    zone_number: i32,
    space_number: i32,
    context: &mut SourceAirReportContext<'_>,
) -> Result<(), Error> {
    let ads_correction_factor = 1.0;
    if *context.afn_control_type == 2 {
        return Err(Error::Unavailable(
            "actual reportAirHeatBal2 ZoneEquipAvail2456 before AFN correction",
        ));
    }
    let out_dry_bulb = context
        .zones
        .zones
        .as_ref()
        .and_then(|v| v.get(index(zone_number).ok()?))
        .ok_or(Error::Unavailable("actual reportAirHeatBal2 Zone2463"))?
        .dry_bulb;
    if air.mat > out_dry_bulb {
        report.infil_heat_loss = air.mcpi
            * (air.mat - out_dry_bulb)
            * context.hvac.time_step_sys_seconds
            * ads_correction_factor;
        report.infil_heat_gain = 0.0;
    } else {
        report.infil_heat_gain = air.mcpi
            * (out_dry_bulb - air.mat)
            * context.hvac.time_step_sys_seconds
            * ads_correction_factor;
        report.infil_heat_loss = 0.0;
    }
    let cp_air = energyplus_psy_cp_air_fn_w(context.environment.current_weather.out_hum_rat);
    let mut h2o_heat_of_vaporization = energyplus_psy_hg_air_fn_w_tdb(air.air_hum_rat, air.mat);
    if air.air_hum_rat > context.environment.current_weather.out_hum_rat {
        report.infil_latent_loss = air.mcpi / cp_air
            * (air.air_hum_rat - context.environment.current_weather.out_hum_rat)
            * h2o_heat_of_vaporization
            * context.hvac.time_step_sys_seconds
            * ads_correction_factor;
        report.infil_latent_gain = 0.0;
    } else {
        report.infil_latent_gain = air.mcpi / cp_air
            * (context.environment.current_weather.out_hum_rat - air.air_hum_rat)
            * h2o_heat_of_vaporization
            * context.hvac.time_step_sys_seconds
            * ads_correction_factor;
        report.infil_latent_loss = 0.0;
    }
    let mut total_load = report.infil_heat_gain + report.infil_latent_gain
        - report.infil_heat_loss
        - report.infil_latent_loss;
    if total_load > 0.0 {
        report.infil_total_gain = total_load * ads_correction_factor;
        report.infil_total_loss = 0.0;
    } else {
        report.infil_total_gain = 0.0;
        report.infil_total_loss = -total_load * ads_correction_factor;
    }
    report.infil_mdot = (air.mcpi / cp_air) * ads_correction_factor;
    report.infil_mass = report.infil_mdot * context.hvac.time_step_sys_seconds;
    report.ventil_mdot = (air.mcpv / cp_air) * ads_correction_factor;
    report.ventil_mass = report.ventil_mdot * context.hvac.time_step_sys_seconds;
    let air_density = energyplus_psy_rho_air_fn_pb_tdb_w(
        context.environment.current_weather.out_baro_press,
        air.mat,
        air.air_hum_rat_avg,
    );
    if air_density < 0.0 {
        return Err(Error::Unavailable(
            "actual reportAirHeatBal2 negative-density diagnostic2507",
        ));
    }
    report.infil_vdot_cur_density = report.infil_mdot / air_density;
    report.infil_volume_cur_density =
        report.infil_vdot_cur_density * context.hvac.time_step_sys_seconds;
    report.ventil_vdot_cur_density = report.ventil_mdot / air_density;
    report.ventil_volume_cur_density =
        report.ventil_vdot_cur_density * context.hvac.time_step_sys_seconds;
    report.infil_vdot_std_density = report.infil_mdot / context.environment.std_rho_air;
    report.infil_volume_std_density =
        report.infil_vdot_std_density * context.hvac.time_step_sys_seconds;
    report.ventil_vdot_std_density = report.ventil_mdot / context.environment.std_rho_air;
    report.ventil_volume_std_density =
        report.ventil_vdot_std_density * context.hvac.time_step_sys_seconds;
    let out_air_density = energyplus_psy_rho_air_fn_pb_tdb_w(
        context.environment.current_weather.out_baro_press,
        out_dry_bulb,
        context.environment.current_weather.out_hum_rat,
    );
    if out_air_density < 0.0 {
        return Err(Error::Unavailable(
            "actual reportAirHeatBal2 outdoor negative-density diagnostic2521",
        ));
    }
    report.infil_vdot_out_density = report.infil_mdot / out_air_density;
    report.ventil_vdot_out_density = report.ventil_mdot / out_air_density;
    let volume = if space_number == 0 {
        context
            .zones
            .zones
            .as_ref()
            .and_then(|v| v.get(index(zone_number).ok()?))
            .ok_or(Error::Unavailable(
                "actual reportAirHeatBal2 Zone volume2526",
            ))?
            .volume
    } else {
        context
            .zones
            .spaces
            .as_ref()
            .and_then(|v| v.get(index(space_number).ok()?))
            .ok_or(Error::Unavailable(
                "actual reportAirHeatBal2 Space volume2526",
            ))?
            .volume
    };
    if volume > 0.0 {
        let sec_in_hour_over_volume = 3600.0 / volume;
        report.infil_air_change_rate_cur_density =
            report.infil_vdot_cur_density * sec_in_hour_over_volume;
        report.infil_air_change_rate_std_density =
            report.infil_vdot_std_density * sec_in_hour_over_volume;
        report.infil_air_change_rate_out_density =
            report.infil_vdot_out_density * sec_in_hour_over_volume;
        report.ventil_air_change_rate_cur_density =
            report.ventil_vdot_cur_density * sec_in_hour_over_volume;
        report.ventil_air_change_rate_std_density =
            report.ventil_vdot_std_density * sec_in_hour_over_volume;
        report.ventil_air_change_rate_out_density =
            report.ventil_vdot_out_density * sec_in_hour_over_volume;
    }
    report.ventil_air_temp = 0.0;
    report.ventil_heat_loss = 0.0;
    report.ventil_heat_gain = 0.0;
    let vent_count = 0;
    let vent_zone_mass_flow = 0.0;
    let vent_zone_air_temp = 0.0;
    if context.simple.total_ventilation > 0 {
        return Err(Error::Unavailable(
            "actual Ventilation2554 before first record read",
        ));
    }
    if ads_correction_factor > 0.0 && vent_count > 1 && vent_zone_mass_flow > 0.0 {
        report.ventil_air_temp /= vent_zone_mass_flow;
    } else if ads_correction_factor > 0.0 && vent_count == 1 {
        report.ventil_air_temp = vent_zone_air_temp;
    } else {
        report.ventil_air_temp = out_dry_bulb;
    }
    report.mix_sen_load = 0.0;
    report.mix_lat_load = 0.0;
    report.mix_volume = 0.0;
    report.mix_vdot_cur_density = 0.0;
    report.mix_vdot_std_density = 0.0;
    report.mix_mass = 0.0;
    report.mix_mdot = 0.0;
    if context.simple.total_mixing > 0 {
        return Err(Error::Unavailable(
            "actual Mixing2619 before first record read",
        ));
    }
    if context.simple.total_cross_mixing > 0 {
        return Err(Error::Unavailable(
            "actual CrossMixing2647 before first record read",
        ));
    }
    if context.simple.total_refrigeration_door_mixing > 0 {
        return Err(Error::Unavailable(
            "actual RefDoorMixing2700 before first record read",
        ));
    }
    if report.mix_sen_load > 0.0 {
        report.mix_heat_loss =
            report.mix_sen_load * context.hvac.time_step_sys_seconds * ads_correction_factor;
        report.mix_heat_gain = 0.0;
    } else {
        report.mix_heat_loss = 0.0;
        report.mix_heat_gain =
            -report.mix_sen_load * context.hvac.time_step_sys_seconds * ads_correction_factor;
    }
    if report.mix_lat_load > 0.0 {
        report.mix_latent_loss =
            report.mix_lat_load * context.hvac.time_step_sys_seconds * ads_correction_factor;
        report.mix_latent_gain = 0.0;
    } else {
        report.mix_latent_loss = 0.0;
        report.mix_latent_gain =
            -report.mix_lat_load * context.hvac.time_step_sys_seconds * ads_correction_factor;
    }
    total_load = report.mix_heat_gain + report.mix_latent_gain
        - report.mix_heat_loss
        - report.mix_latent_loss;
    if total_load > 0.0 {
        report.mix_total_gain = total_load * ads_correction_factor;
        report.mix_total_loss = 0.0;
    } else {
        report.mix_total_gain = 0.0;
        report.mix_total_loss = -total_load * ads_correction_factor;
    }
    if context.simple.total_zone_air_balance > 0 {
        return Err(Error::Unavailable(
            "actual ZoneAirBalance2807 before BalanceMethod read",
        ));
    }
    h2o_heat_of_vaporization = energyplus_psy_hg_air_fn_w_tdb(
        context.environment.current_weather.out_hum_rat,
        out_dry_bulb,
    );
    report.sys_inlet_mass = 0.0;
    report.sys_outlet_mass = 0.0;
    if !*equipment_config.is_controlled() {
        if *equipment_config.number_inlet_nodes() > 0 {
            return Err(Error::Unavailable(
                "actual InletNode2857 before Node.MassFlowRate read",
            ));
        }
        if *equipment_config.number_exhaust_nodes() > 0 {
            return Err(Error::Unavailable(
                "actual ExhaustNode2861 before Node.MassFlowRate read",
            ));
        }
        if *equipment_config.number_return_nodes() > 0 {
            return Err(Error::Unavailable(
                "actual ReturnNode2865 before Node.MassFlowRate read",
            ));
        }
    }
    report.exfil_mass = report.infil_mass
        + report.ventil_mass
        + report.mix_mass
        + report.oa_balance_mass
        + report.sys_inlet_mass
        - report.sys_outlet_mass;
    report.exfil_sensi_loss =
        report.exfil_mass / context.hvac.time_step_sys_seconds * (air.mat - out_dry_bulb) * cp_air;
    report.exfil_latent_loss = report.exfil_mass / context.hvac.time_step_sys_seconds
        * (air.air_hum_rat - context.environment.current_weather.out_hum_rat)
        * h2o_heat_of_vaporization;
    report.exfil_total_loss = report.exfil_latent_loss + report.exfil_sensi_loss;
    if space_number == 0 {
        context.simple.zone_total_exfiltration_heat_loss +=
            report.exfil_total_loss * context.hvac.time_step_sys_seconds;
        context.simple.zone_total_exhaust_heat_loss +=
            report.exh_total_loss * context.hvac.time_step_sys_seconds;
    }
    Ok(())
}

impl SourceHvacManager {
    /// Native ReportInfiltrations2238 reads actual TimeStepSysSec even for an empty range.
    pub(crate) fn report_source_infiltrations(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        let _time_step_sys_seconds = o.hvac.time_step_sys_seconds;
        if o.controller
            .simple_air
            .infiltration
            .as_ref()
            .is_some_and(|v| !v.is_empty())
        {
            return Err(Error::Unavailable(
                "actual Infiltration.spaceIndex2259 before positive report body",
            ));
        }
        Ok(())
    }

    pub(crate) fn report_source_air_heat_balance(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        o.controller.simple_air.zone_total_exfiltration_heat_loss = 0.0;
        o.controller.simple_air.zone_total_exhaust_heat_loss = 0.0;
        if o.controller.subsystems.airflow_network.control_type != 0 {
            return Err(Error::Unavailable("actual AFN report2362"));
        }
        for number in 1..=o.globals.num_of_zones {
            let report = o
                .controller
                .reports
                .zones
                .as_mut()
                .and_then(|v| v.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("actual ZnAirRpt2367"))?;
            let equipment_config = o.equipment.source_zone_equipment_config(number)?;
            let context = SourceAirReportContext {
                zones: o.zones,
                environment: o.environment,
                hvac: o.hvac,
                simple: &mut o.controller.simple_air,
                afn_control_type: &o.controller.subsystems.airflow_network.control_type,
                fans: &self.ancillary.heat_emission.fans,
            };
            report_air_heat_bal1(report, &equipment_config, number, &context)?;
        }
        if o.space_controls.current {
            for number in 1..=o.globals.num_spaces {
                let report = o
                    .controller
                    .reports
                    .spaces
                    .as_mut()
                    .and_then(|v| v.get_mut(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual spaceAirRpt2373"))?;
                let equipment_config = o.equipment.source_space_equipment_config(number)?;
                let zone_number = o
                    .zones
                    .spaces
                    .as_ref()
                    .and_then(|v| v.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual Space.zoneNum2375"))?
                    .zone_num;
                let context = SourceAirReportContext {
                    zones: o.zones,
                    environment: o.environment,
                    hvac: o.hvac,
                    simple: &mut o.controller.simple_air,
                    afn_control_type: &o.controller.subsystems.airflow_network.control_type,
                    fans: &self.ancillary.heat_emission.fans,
                };
                report_air_heat_bal1(report, &equipment_config, zone_number, &context)?;
            }
        }
        if !(o.controller.subsystems.airflow_network.control_type == 0
            || o.controller.subsystems.airflow_network.control_type == 2)
        {
            return Ok(());
        }
        self.report_source_infiltrations(o)?;
        for number in 1..=o.globals.num_of_zones {
            let report = o
                .controller
                .reports
                .zones
                .as_mut()
                .and_then(|v| v.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("actual ZnAirRpt2389"))?;
            let equipment_config = o.equipment.source_zone_equipment_config(number)?;
            let air = &o
                .air
                .zones
                .as_ref()
                .and_then(|v| v.get(index(number).ok()?))
                .ok_or(Error::Unavailable("actual Zone AirHB2391"))?
                .air;
            let mut context = SourceAirReportContext {
                zones: o.zones,
                environment: o.environment,
                hvac: o.hvac,
                simple: &mut o.controller.simple_air,
                afn_control_type: &o.controller.subsystems.airflow_network.control_type,
                fans: &self.ancillary.heat_emission.fans,
            };
            report_air_heat_bal2(report, &equipment_config, air, number, 0, &mut context)?;
        }
        if o.space_controls.current {
            for number in 1..=o.globals.num_spaces {
                let report = o
                    .controller
                    .reports
                    .spaces
                    .as_mut()
                    .and_then(|v| v.get_mut(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual spaceAirRpt2396"))?;
                let equipment_config = o.equipment.source_space_equipment_config(number)?;
                let air = &o
                    .air
                    .spaces
                    .as_ref()
                    .and_then(|v| v.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual Space AirHB2398"))?
                    .air;
                let zone_number = o
                    .zones
                    .spaces
                    .as_ref()
                    .and_then(|v| v.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual Space.zoneNum2399"))?
                    .zone_num;
                let mut context = SourceAirReportContext {
                    zones: o.zones,
                    environment: o.environment,
                    hvac: o.hvac,
                    simple: &mut o.controller.simple_air,
                    afn_control_type: &o.controller.subsystems.airflow_network.control_type,
                    fans: &self.ancillary.heat_emission.fans,
                };
                report_air_heat_bal2(
                    report,
                    &equipment_config,
                    air,
                    zone_number,
                    number,
                    &mut context,
                )?;
            }
        }
        Ok(())
    }
}
