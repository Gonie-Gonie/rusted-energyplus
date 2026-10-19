//! Genuine FillRemainingPredefinedEntries6242 prefix on the retained owners.
//! Real OA/SHGS/LEED summaries precede the mandatory thermostat report child.
use super::{Error, SourceTabularReports};
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::SourceZoneAirPredictorCorrector;
use crate::heat_balance::ordinary_exterior_energy_use::SourceExteriorEnergyUse;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::weather::day::WeatherEnvironmentState;

pub(crate) struct SourceRemainingPredefinedOwners<'a> {
    pub(crate) hvac: &'a mut SelectedHvacGlobals,
    pub(crate) hvac_manager: &'a crate::heat_balance::ordinary_hvac_manager::SourceHvacManager,
    pub(crate) gain_modules: &'a crate::heat_balance::selected_gain_modules::SourceGainModules,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) clock: &'a crate::weather::day::WeatherGlobalState,
    pub(crate) gains: &'a SelectedInternalHeatGains,
    pub(crate) exterior: &'a SourceExteriorEnergyUse,
    pub(crate) schedules: &'a SelectedScheduleManager,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) controller: &'a SourceZoneAirPredictorCorrector,
    pub(crate) predefined_summary:
        &'a crate::heat_balance::ordinary_thermal_comfort::SourceComfortPredefinedSummary,
    pub(crate) zone_plenum: &'a crate::heat_balance::startup_integrity::SourceZonePlenumCounts,
    pub(crate) weather_file_location_title: &'a str,
    pub(crate) registry: &'a PredefinedRegistry,
    pub(crate) table: &'a mut PredefinedStringTable,
}

fn at(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(Error::Unavailable(
            "actual FillRemaining positive array ordinal",
        ))
}
fn column(registry: &PredefinedRegistry, name: &'static str) -> Result<i32, Error> {
    registry
        .named_indices
        .get(name)
        .copied()
        .ok_or(Error::Unavailable(name))
}
fn real2(
    table: &mut PredefinedStringTable,
    registry: &PredefinedRegistry,
    name: &'static str,
    object: &str,
    value: f64,
) -> Result<(), Error> {
    table
        .fixed_real_entry_2(column(registry, name)?, object, value)
        .map_err(Error::from)
}

fn real0(
    table: &mut PredefinedStringTable,
    registry: &PredefinedRegistry,
    name: &'static str,
    object: &str,
    value: f64,
) -> Result<(), Error> {
    table
        .fixed_real_entry_0(column(registry, name)?, object, value)
        .map_err(Error::from)
}
fn real3(
    table: &mut PredefinedStringTable,
    registry: &PredefinedRegistry,
    name: &'static str,
    object: &str,
    value: f64,
) -> Result<(), Error> {
    table
        .fixed_real_entry_3(column(registry, name)?, object, value)
        .map_err(Error::from)
}
fn real4(
    table: &mut PredefinedStringTable,
    registry: &PredefinedRegistry,
    name: &'static str,
    object: &str,
    value: f64,
) -> Result<(), Error> {
    table
        .fixed_real_entry_4(column(registry, name)?, object, value)
        .map_err(Error::from)
}
fn integer(
    table: &mut PredefinedStringTable,
    registry: &PredefinedRegistry,
    name: &'static str,
    object: &str,
    value: i32,
) -> Result<(), Error> {
    table
        .integer_entry(column(registry, name)?, object, value)
        .map_err(Error::from)
}
fn text(
    table: &mut PredefinedStringTable,
    registry: &PredefinedRegistry,
    name: &'static str,
    object: &str,
    value: &str,
) -> Result<(), Error> {
    table
        .string_entry(column(registry, name)?, object, value)
        .map_err(Error::from)
}
pub(super) fn source_date_to_string(coded_date: i32) -> Result<String, Error> {
    const MONTHS: [&str; 12] = [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    if coded_date == 0 {
        return Ok("-".into());
    }
    let mut item = coded_date;
    let month = item / 1000000;
    item -= month * 1000000;
    let day = item / 10000;
    item -= day * 10000;
    let mut hour = item / 100;
    let mut minute = item - (item / 100) * 100;
    if month < 1 || month > 12 {
        return Ok("-".into());
    }
    hour -= 1;
    if minute == 60 {
        hour += 1;
        minute = 0;
    }
    Ok(format!(
        "{day:02}-{:3}-{hour:02}:{minute:02}",
        MONTHS[(month - 1) as usize]
    ))
}
impl SourceTabularReports {
    pub(crate) fn fill_source_remaining_predefined_entries(
        &mut self,
        o: &mut SourceRemainingPredefinedOwners<'_>,
    ) -> Result<(), Error> {
        let number_primary_air_systems = o.hvac.number_of_primary_air_systems;
        let number_autocalculated_fields = o.prescan.number_autocalculated_fields;
        let number_autosized_fields = o.prescan.number_autosized_fields;
        let number_defaulted_fields = o.prescan.number_defaulted_fields;
        let number_input_records = o.prescan.number_input_records;
        let total_autocalculatable_fields = o.prescan.total_autocalculatable_fields;
        let total_autosizable_fields = o.prescan.total_autosizable_fields;
        let total_fields_with_defaults = o.prescan.total_fields_with_defaults;

        let mut start_of_week = o.environment.run_period_start_day_of_week;
        if start_of_week == 0 {
            start_of_week = 2;
        }

        let mut consumption_total = 0.0;
        if o.gains.totals[1] > 0 {
            return Err(Error::Unavailable(
                "actual FillRemaining Lights(iLight)6276 before ZonePtr6277",
            ));
        }
        real2(
            o.table,
            o.registry,
            "pdchInLtConsump",
            "Interior Lighting Total",
            consumption_total,
        )?;

        consumption_total = 0.0;
        for number in 1..=*o.exterior.source_number_lights() {
            let light = o.exterior.source_light_report(number)?;
            if light.is_schedule_only() {
                let hours = o.schedules.source_average_weekly_hours_full_load(
                    light.schedule().ok_or(Error::Unavailable(
                        "actual FillRemaining Exterior Schedule nullptr6315",
                    ))?,
                    start_of_week,
                    o.environment.current_year_is_leap_year,
                    o.clock.time_steps_in_hour,

                )?;
                real2(
                    o.table,
                    o.registry,
                    "pdchExLtAvgHrSchd",
                    light.name(),
                    hours,
                )?;
            }
            if self.input_state.gather_elapsed_time_beps > 0.0 {
                self.input_state.hrs_per_week = 24.0 * 7.0 * light.sum_time_not_zero_consumption()
                    / self.input_state.gather_elapsed_time_beps;
                real2(
                    o.table,
                    o.registry,
                    "pdchExLtAvgHrOper",
                    light.name(),
                    self.input_state.hrs_per_week,
                )?;
            }
            if (light.design_level() * self.input_state.gather_elapsed_time_beps) > 0.0 {
                self.input_state.hrs_per_week = 24.0 * 7.0 * light.sum_consumption()
                    / (light.design_level() * self.input_state.gather_elapsed_time_beps * 3600.0);
                real2(
                    o.table,
                    o.registry,
                    "pdchExLtFullLoadHrs",
                    light.name(),
                    self.input_state.hrs_per_week,
                )?;
            }
            real2(
                o.table,
                o.registry,
                "pdchExLtConsump",
                light.name(),
                light.sum_consumption() / 1000000000.0,
            )?;
            consumption_total += light.sum_consumption() / 1000000000.0;
        }
        real2(
            o.table,
            o.registry,
            "pdchExLtConsump",
            "Exterior Lighting Total",
            consumption_total,
        )?;

        let total_occupants = 0.0;
        let total_average_occupants = 0.0;
        let total_area = 0.0;
        let total_voz_min = 0.0;
        let total_mech_vent_vol = 0.0;
        let total_nat_vent_vol = 0.0;
        let total_infil_vol = 0.0;
        let total_voz_dyn = 0.0;
        let total_mech_vent_rate_occ = 0.0;
        let total_nat_vent_rate_occ = 0.0;
        let total_infil_rate_occ = 0.0;
        let total_voz_dyn_occ = 0.0;
        for number in 1..=o.globals.num_of_zones {
            let zone = o
                .zones
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at(number).ok()?))
                .ok_or(Error::Unavailable("actual FillRemaining Zone(iZone)6350"))?;
            let _zone_multiplier =
                zone.multiplier
                    .checked_mul(zone.list_multiplier)
                    .ok_or(Error::Unavailable(
                        "actual FillRemaining defined native zoneMult int product6352",
                    ))?;
            if zone.system_zone_node_number >= 0 {
                let predefined = o
                    .zones
                    .zone_predefined
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at(number).ok()?))
                    .ok_or(Error::Unavailable("actual FillRemaining ZonePreDefRep6355"))?;
                if !o
                    .controller
                    .subsystems
                    .airflow_network
                    .multizone_always_simulated
                {
                    predefined.afn_infil_vol_total_std_den = 0.0;
                    predefined.afn_vent_vol_total_std_den = 0.0;
                    predefined.afn_infil_vol_total_occ_std_den = 0.0;
                    predefined.afn_vent_vol_total_occ_std_den = 0.0;
                    predefined.afn_infil_vol_min = 0.0;
                    predefined.afn_infil_vol_total_occ = 0.0;
                }
                if zone.is_controlled {
                    let _air_loop_name = String::new();
                    return Err(Error::Unavailable(
                        "actual FillRemaining controlled ZoneEquipConfig.NumInletNodes6370",
                    ));
                }
            }
        }
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual FillRemaining PrimaryAirSystems(iSys)6540 before SysPreDefRep6541",
            ));
        }
        real4(
            o.table,
            o.registry,
            "pdchOaMvDesZnOa",
            "Total Facility",
            o.predefined_summary.total_voz_max,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaMvZoneVol",
            "Total Facility",
            self.input_state.total_volume,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaMvZoneArea",
            "Total Facility",
            total_area,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaMvNomNumOcc",
            "Total Facility",
            total_occupants,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaMvAvgNumOcc",
            "Total Facility",
            total_average_occupants,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchOaMvMinDynTrgVent",
            "Total Facility",
            total_voz_min,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaTaBzMechVent",
            "Total Facility",
            total_mech_vent_vol,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaTaBzNatVent",
            "Total Facility",
            total_nat_vent_vol,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaTaBzTotVent",
            "Total Facility",
            total_mech_vent_vol + total_nat_vent_vol,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaTaBzInfil",
            "Total Facility",
            total_infil_vol,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaTaBzTotVentInfil",
            "Total Facility",
            total_mech_vent_vol + total_nat_vent_vol + total_infil_vol,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaTaBzDynTrgVent",
            "Total Facility",
            total_voz_dyn,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaTaBzTmBelow",
            "Total Facility",
            o.predefined_summary.total_any_zone_below_voz_dyn_for_oa,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaTaBzTmAt",
            "Total Facility",
            o.predefined_summary.total_all_zones_at_voz_dyn_for_oa,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaTaBzTmAbove",
            "Total Facility",
            o.predefined_summary.total_any_zone_above_voz_dyn_for_oa,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaTaBzTmAboveUnocc",
            "Total Facility",
            o.predefined_summary.total_any_zone_vent_unocc_for_oa,
        )?;
        real4(
            o.table,
            o.registry,
            "pdchOaOccBzMechVent",
            "Total Facility",
            total_mech_vent_rate_occ,
        )?;
        real4(
            o.table,
            o.registry,
            "pdchOaOccBzNatVent",
            "Total Facility",
            total_nat_vent_rate_occ,
        )?;
        real4(
            o.table,
            o.registry,
            "pdchOaOccBzTotVent",
            "Total Facility",
            total_mech_vent_rate_occ + total_nat_vent_rate_occ,
        )?;
        real4(
            o.table,
            o.registry,
            "pdchOaOccBzInfil",
            "Total Facility",
            total_infil_rate_occ,
        )?;
        real4(
            o.table,
            o.registry,
            "pdchOaOccBzTotVentInfil",
            "Total Facility",
            total_mech_vent_rate_occ + total_nat_vent_rate_occ + total_infil_rate_occ,
        )?;
        real4(
            o.table,
            o.registry,
            "pdchOaOccBzDynTrgVent",
            "Total Facility",
            total_voz_dyn_occ,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaOccBzTmBelow",
            "Total Facility",
            o.predefined_summary.total_any_zone_below_voz_dyn_occ_for_oa,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaOccBzTmAt",
            "Total Facility",
            o.predefined_summary.total_all_zones_at_voz_dyn_occ_for_oa,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchOaOccBzTmAbove",
            "Total Facility",
            o.predefined_summary.total_any_zone_above_voz_dyn_occ_for_oa,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchHVACcntVal",
            "HVAC Air Loops",
            number_primary_air_systems,
        )?;
        for number in 1..=o.globals.num_of_zones {
            let zone = o
                .zones
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at(number).ok()?))
                .ok_or(Error::Unavailable(
                    "actual FillRemaining conditioned-zone count Zone row",
                ))?;
            if zone.system_zone_node_number > 0 {
                self.input_state.num_cond_zones = self
                    .input_state
                    .num_cond_zones
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "actual FillRemaining numCondZones native int increment",
                    ))?;
            } else {
                self.input_state.num_uncond_zones = self
                    .input_state
                    .num_uncond_zones
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "actual FillRemaining numUncondZones native int increment",
                    ))?;
            }
        }
        integer(
            o.table,
            o.registry,
            "pdchHVACcntVal",
            "Conditioned Zones",
            self.input_state.num_cond_zones,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchHVACcntVal",
            "Unconditioned Zones",
            self.input_state.num_uncond_zones,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchHVACcntVal",
            "Supply Plenums",
            o.zone_plenum.number_supply_plenums,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchHVACcntVal",
            "Return Plenums",
            o.zone_plenum.number_return_plenums,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchFieldCntVal",
            "IDF Objects",
            number_input_records,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchFieldCntVal",
            "Defaulted Fields",
            number_defaulted_fields,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchFieldCntVal",
            "Fields with Defaults",
            total_fields_with_defaults,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchFieldCntVal",
            "Autosized Fields",
            number_autosized_fields,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchFieldCntVal",
            "Autosizable Fields",
            total_autosizable_fields,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchFieldCntVal",
            "Autocalculated Fields",
            number_autocalculated_fields,
        )?;
        integer(
            o.table,
            o.registry,
            "pdchFieldCntVal",
            "Autocalculatable Fields",
            total_autocalculatable_fields,
        )?;
        for number in 1..=o.globals.num_of_zones {
            let zone = o
                .zones
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at(number).ok()?))
                .ok_or(Error::Unavailable("actual FillRemaining annual Zone row"))?;
            let predefined = o
                .zones
                .zone_predefined
                .as_ref()
                .and_then(|rows| rows.get(at(number).ok()?))
                .ok_or(Error::Unavailable(
                    "actual FillRemaining annual ZonePreDefRep row",
                ))?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnZoneEqHt",
                &zone.name,
                predefined.shgs_an_zone_eq_ht * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnZoneEqCl",
                &zone.name,
                predefined.shgs_an_zone_eq_cl * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnHvacATUHt",
                &zone.name,
                predefined.shgs_an_hvac_atu_ht * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnHvacATUCl",
                &zone.name,
                predefined.shgs_an_hvac_atu_cl * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnSurfHt",
                &zone.name,
                predefined.shgs_an_surf_ht * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnSurfCl",
                &zone.name,
                predefined.shgs_an_surf_cl * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnPeoplAdd",
                &zone.name,
                predefined.shgs_an_peopl_add * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnLiteAdd",
                &zone.name,
                predefined.shgs_an_lite_add * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnEquipAdd",
                &zone.name,
                predefined.shgs_an_equip_add * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnWindAdd",
                &zone.name,
                predefined.shgs_an_wind_add * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnIzaAdd",
                &zone.name,
                predefined.shgs_an_iza_add * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnInfilAdd",
                &zone.name,
                predefined.shgs_an_infil_add * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnOtherAdd",
                &zone.name,
                predefined.shgs_an_other_add * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnEquipRem",
                &zone.name,
                predefined.shgs_an_equip_rem * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnWindRem",
                &zone.name,
                predefined.shgs_an_wind_rem * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnIzaRem",
                &zone.name,
                predefined.shgs_an_iza_rem * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnInfilRem",
                &zone.name,
                predefined.shgs_an_infil_rem * 1.0e-9,
            )?;
            real3(
                o.table,
                o.registry,
                "pdchSHGSAnOtherRem",
                &zone.name,
                predefined.shgs_an_other_rem * 1.0e-9,
            )?;
            text(
                o.table,
                o.registry,
                "pdchSHGSClTimePeak",
                &zone.name,
                &source_date_to_string(predefined.cl_pt_time_stamp)?,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClHvacHt",
                &zone.name,
                (predefined.shgs_cl_hvac_ht - predefined.shgs_cl_hvac_atu_ht),
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClHvacCl",
                &zone.name,
                (predefined.shgs_cl_hvac_cl - predefined.shgs_cl_hvac_atu_cl),
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClHvacATUHt",
                &zone.name,
                predefined.shgs_cl_hvac_atu_ht,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClHvacATUCl",
                &zone.name,
                predefined.shgs_cl_hvac_atu_cl,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClSurfHt",
                &zone.name,
                predefined.shgs_cl_surf_ht,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClSurfCl",
                &zone.name,
                predefined.shgs_cl_surf_cl,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClPeoplAdd",
                &zone.name,
                predefined.shgs_cl_peopl_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClLiteAdd",
                &zone.name,
                predefined.shgs_cl_lite_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClEquipAdd",
                &zone.name,
                predefined.shgs_cl_equip_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClWindAdd",
                &zone.name,
                predefined.shgs_cl_wind_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClIzaAdd",
                &zone.name,
                predefined.shgs_cl_iza_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClInfilAdd",
                &zone.name,
                predefined.shgs_cl_infil_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClOtherAdd",
                &zone.name,
                predefined.shgs_cl_other_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClEquipRem",
                &zone.name,
                predefined.shgs_cl_equip_rem,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClWindRem",
                &zone.name,
                predefined.shgs_cl_wind_rem,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClIzaRem",
                &zone.name,
                predefined.shgs_cl_iza_rem,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClInfilRem",
                &zone.name,
                predefined.shgs_cl_infil_rem,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSClOtherRem",
                &zone.name,
                predefined.shgs_cl_other_rem,
            )?;
            text(
                o.table,
                o.registry,
                "pdchSHGSHtTimePeak",
                &zone.name,
                &source_date_to_string(predefined.ht_pt_time_stamp)?,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtHvacHt",
                &zone.name,
                (predefined.shgs_ht_hvac_ht - predefined.shgs_ht_hvac_atu_ht),
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtHvacCl",
                &zone.name,
                (predefined.shgs_ht_hvac_cl - predefined.shgs_ht_hvac_atu_cl),
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtHvacATUHt",
                &zone.name,
                predefined.shgs_ht_hvac_atu_ht,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtHvacATUCl",
                &zone.name,
                predefined.shgs_ht_hvac_atu_cl,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtSurfHt",
                &zone.name,
                predefined.shgs_ht_surf_ht,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtSurfCl",
                &zone.name,
                predefined.shgs_ht_surf_cl,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtPeoplAdd",
                &zone.name,
                predefined.shgs_ht_peopl_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtLiteAdd",
                &zone.name,
                predefined.shgs_ht_lite_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtEquipAdd",
                &zone.name,
                predefined.shgs_ht_equip_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtWindAdd",
                &zone.name,
                predefined.shgs_ht_wind_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtIzaAdd",
                &zone.name,
                predefined.shgs_ht_iza_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtInfilAdd",
                &zone.name,
                predefined.shgs_ht_infil_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtOtherAdd",
                &zone.name,
                predefined.shgs_ht_other_add,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtEquipRem",
                &zone.name,
                predefined.shgs_ht_equip_rem,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtWindRem",
                &zone.name,
                predefined.shgs_ht_wind_rem,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtIzaRem",
                &zone.name,
                predefined.shgs_ht_iza_rem,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtInfilRem",
                &zone.name,
                predefined.shgs_ht_infil_rem,
            )?;
            real2(
                o.table,
                o.registry,
                "pdchSHGSHtOtherRem",
                &zone.name,
                predefined.shgs_ht_other_rem,
            )?;
        }
        for number in 1..=o.globals.num_of_zones {
            let predefined = o
                .zones
                .zone_predefined
                .as_ref()
                .and_then(|rows| rows.get(at(number).ok()?))
                .ok_or(Error::Unavailable(
                    "actual FillRemaining annual facility ZonePreDefRep row",
                ))?;
            self.input_state.total_zone_eq_ht += predefined.shgs_an_zone_eq_ht;
            self.input_state.total_zone_eq_cl += predefined.shgs_an_zone_eq_cl;
            self.input_state.total_hvac_atu_ht += predefined.shgs_an_hvac_atu_ht;
            self.input_state.total_hvac_atu_cl += predefined.shgs_an_hvac_atu_cl;
            self.input_state.total_surf_ht += predefined.shgs_an_surf_ht;
            self.input_state.total_surf_cl += predefined.shgs_an_surf_cl;
            self.input_state.total_peopl_add += predefined.shgs_an_peopl_add;
            self.input_state.total_lite_add += predefined.shgs_an_lite_add;
            self.input_state.total_equip_add += predefined.shgs_an_equip_add;
            self.input_state.total_wind_add += predefined.shgs_an_wind_add;
            self.input_state.total_iza_add += predefined.shgs_an_iza_add;
            self.input_state.total_infil_add += predefined.shgs_an_infil_add;
            self.input_state.total_other_add += predefined.shgs_an_other_add;
            self.input_state.total_equip_rem += predefined.shgs_an_equip_rem;
            self.input_state.total_wind_rem += predefined.shgs_an_wind_rem;
            self.input_state.total_iza_rem += predefined.shgs_an_iza_rem;
            self.input_state.total_infil_rem += predefined.shgs_an_infil_rem;
            self.input_state.total_other_rem += predefined.shgs_an_other_rem;
        }
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnZoneEqHt",
            "Total Facility",
            self.input_state.total_zone_eq_ht * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnZoneEqCl",
            "Total Facility",
            self.input_state.total_zone_eq_cl * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnHvacATUHt",
            "Total Facility",
            self.input_state.total_hvac_atu_ht * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnHvacATUCl",
            "Total Facility",
            self.input_state.total_hvac_atu_cl * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnSurfHt",
            "Total Facility",
            self.input_state.total_surf_ht * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnSurfCl",
            "Total Facility",
            self.input_state.total_surf_cl * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnPeoplAdd",
            "Total Facility",
            self.input_state.total_peopl_add * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnLiteAdd",
            "Total Facility",
            self.input_state.total_lite_add * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnEquipAdd",
            "Total Facility",
            self.input_state.total_equip_add * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnWindAdd",
            "Total Facility",
            self.input_state.total_wind_add * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnIzaAdd",
            "Total Facility",
            self.input_state.total_iza_add * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnInfilAdd",
            "Total Facility",
            self.input_state.total_infil_add * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnOtherAdd",
            "Total Facility",
            self.input_state.total_other_add * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnEquipRem",
            "Total Facility",
            self.input_state.total_equip_rem * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnWindRem",
            "Total Facility",
            self.input_state.total_wind_rem * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnIzaRem",
            "Total Facility",
            self.input_state.total_iza_rem * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnInfilRem",
            "Total Facility",
            self.input_state.total_infil_rem * 1.0e-9,
        )?;
        real3(
            o.table,
            o.registry,
            "pdchSHGSAnOtherRem",
            "Total Facility",
            self.input_state.total_other_rem * 1.0e-9,
        )?;
        text(
            o.table,
            o.registry,
            "pdchSHGSClTimePeak",
            "Total Facility",
            &source_date_to_string(o.zones.building_predefined.cl_pt_time_stamp)?,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClHvacHt",
            "Total Facility",
            (o.zones.building_predefined.shgs_cl_hvac_ht
                - o.zones.building_predefined.shgs_cl_hvac_atu_ht),
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClHvacCl",
            "Total Facility",
            (o.zones.building_predefined.shgs_cl_hvac_cl
                - o.zones.building_predefined.shgs_cl_hvac_atu_cl),
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClHvacATUHt",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_hvac_atu_ht,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClHvacATUCl",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_hvac_atu_cl,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClSurfHt",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_surf_ht,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClSurfCl",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_surf_cl,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClPeoplAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_peopl_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClLiteAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_lite_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClEquipAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_equip_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClWindAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_wind_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClIzaAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_iza_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClInfilAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_infil_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClOtherAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_other_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClEquipRem",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_equip_rem,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClWindRem",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_wind_rem,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClIzaRem",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_iza_rem,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClInfilRem",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_infil_rem,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSClOtherRem",
            "Total Facility",
            o.zones.building_predefined.shgs_cl_other_rem,
        )?;
        text(
            o.table,
            o.registry,
            "pdchSHGSHtTimePeak",
            "Total Facility",
            &source_date_to_string(o.zones.building_predefined.ht_pt_time_stamp)?,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtHvacHt",
            "Total Facility",
            (o.zones.building_predefined.shgs_ht_hvac_ht
                - o.zones.building_predefined.shgs_ht_hvac_atu_ht),
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtHvacCl",
            "Total Facility",
            (o.zones.building_predefined.shgs_ht_hvac_cl
                - o.zones.building_predefined.shgs_ht_hvac_atu_cl),
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtHvacATUHt",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_hvac_atu_ht,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtHvacATUCl",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_hvac_atu_cl,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtSurfHt",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_surf_ht,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtSurfCl",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_surf_cl,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtPeoplAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_peopl_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtLiteAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_lite_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtEquipAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_equip_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtWindAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_wind_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtIzaAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_iza_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtInfilAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_infil_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtOtherAdd",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_other_add,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtEquipRem",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_equip_rem,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtWindRem",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_wind_rem,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtIzaRem",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_iza_rem,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtInfilRem",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_infil_rem,
        )?;
        real2(
            o.table,
            o.registry,
            "pdchSHGSHtOtherRem",
            "Total Facility",
            o.zones.building_predefined.shgs_ht_other_rem,
        )?;
        if o.environment.environment_name == o.weather_file_location_title {
            text(
                o.table,
                o.registry,
                "pdchLeedGenData",
                "Weather File",
                &o.environment.environment_name,
            )?;
        } else {
            text(
                o.table,
                o.registry,
                "pdchLeedGenData",
                "Weather File",
                &format!(
                    "{} ** {}",
                    o.environment.environment_name, o.weather_file_location_title
                ),
            )?;
        }
        if self.input_state.units_style_tabular == 4 || self.input_state.units_style_tabular == 5 {
            text(
                o.table,
                o.registry,
                "pdchLeedGenData",
                "Total gross floor area [ft2]",
                "-",
            )?;
        } else {
            text(
                o.table,
                o.registry,
                "pdchLeedGenData",
                "Total gross floor area [m2]",
                "-",
            )?;
        }
        for schedule in &o.schedules.schedules {
            let Some(type_number) = schedule.schedule_type_number else {
                continue;
            };
            let schedule_type =
                o.schedules
                    .schedule_types
                    .get(type_number)
                    .ok_or(Error::Unavailable(
                        "actual LEED scheduleTypes[schedTypeNum]",
                    ))?;
            if !schedule_type.name.eq_ignore_ascii_case("FRACTION") {
                continue;
            }
            let annual = o.schedules.source_annual_hours_full_load(
                schedule.number,
                start_of_week,
                o.environment.current_year_is_leap_year,
                    o.clock.time_steps_in_hour,

            )?;
            real0(
                o.table,
                o.registry,
                "pdchLeedEflhEflh",
                &schedule.name,
                annual,
            )?;
            let annual_nonzero = o.schedules.source_annual_hours_greater_than_1_percent(
                schedule.number,
                start_of_week,
                o.environment.current_year_is_leap_year,
                    o.clock.time_steps_in_hour,
                    o.clock.time_step_zone,

            )?;
            real0(
                o.table,
                o.registry,
                "pdchLeedEflhNonZerHrs",
                &schedule.name,
                annual_nonzero,
            )?;
        }
        o.controller.fill_source_predefined_table_on_thermostat_setpoints()?;
        o.controller.fill_source_predefined_table_on_thermostat_schedules()?;
        const _FLOW_FRACTIONS: [f64; 10] = [0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let save_night_vent_on = o.hvac.night_vent_on;
        let save_turn_fans_on = o.hvac.turn_fans_on;
        o.hvac.night_vent_on = false;
        o.hvac.turn_fans_on = true;
        if o.hvac_manager.ancillary.heat_emission.fans.as_deref().and_then(|rows| rows.first()).is_some() {
            return Err(Error::Unavailable("actual Fan Operating Points first fan Name/type PreDefTableEntry before simulation"));
        }
        o.hvac.night_vent_on = save_night_vent_on;
        o.hvac.turn_fans_on = save_turn_fans_on;
        o.gain_modules.water_use.fill_source_predefined_water_equipment_tables()?;
        o.hvac_manager.fill_source_predefined_setpoint_manager_tables()?;
        o.hvac_manager.ancillary.availability.fill_source_predefined_availability_manager_tables(o.hvac)?;
        o.hvac_manager.fill_source_plant_equipment_operation_load()?;
        Ok(())
    }
}
