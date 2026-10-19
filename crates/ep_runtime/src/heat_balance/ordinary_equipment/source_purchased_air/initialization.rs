//! Native InitPurchasedAir/SizePurchasedAir on the same retained unit and real owners.
use super::{
    OrdinaryEquipment, at,
    owners::{SourcePurchasedAirEconomizer as Econ, SourcePurchasedAirLimit as Limit},
};
use crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays;
use crate::heat_balance::ordinary_hvac_manager::simulation::state::SourceAirLoopData;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::selected_sizing::SelectedSizingManager;
use crate::heat_balance::selected_sizing::{
    autosizing::{SourceAutoSizingType as Sizer, size_source_component},
    sizer_output::{SourceSizerReportingOwners, report_sizer_output},
};
use crate::heat_balance::source_output_registration::SourceOutputDictionaryHost;
use crate::weather::day::WeatherGlobalState;

/// Temporary disjoint borrows into actual HVAC caller owners; no copied readiness or values.
pub(crate) struct SourcePurchasedAirInitOwners<'a> {
    pub(crate) sizing: &'a mut SelectedSizingManager,
    pub(crate) fan: &'a SourceZoneFanHeatBalanceArrays,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) air_loop: &'a SourceAirLoopData,
    pub(crate) source_is_epjson: &'a bool,
}

fn bool_at(rows: &Option<Vec<bool>>, number: i32) -> Result<bool, Error> {
    rows.as_ref()
        .and_then(|rows| rows.get(at(number).ok()?))
        .copied()
        .ok_or(Error::Unavailable(
            "actual PurchasedAir initialization flag ordinal",
        ))
}
fn bool_at_mut(rows: &mut Option<Vec<bool>>, number: i32) -> Result<&mut bool, Error> {
    rows.as_mut()
        .and_then(|rows| rows.get_mut(at(number).ok()?))
        .ok_or(Error::Unavailable(
            "actual PurchasedAir mutable initialization flag ordinal",
        ))
}

// General::FindNumberInList981–1005: first exact match within the actual declared count.
fn find_number(number: i32, rows: &Option<Vec<usize>>, count: usize) -> Result<usize, Error> {
    for ordinal in 1..=count {
        let value =
            *rows
                .as_ref()
                .and_then(|rows| rows.get(ordinal - 1))
                .ok_or(Error::Unavailable(
                    "actual PurchasedAir FindNumberInList array element",
                ))?;
        if i32::try_from(value)
            .map_err(|_| Error::Unavailable("actual Source Node integer domain"))?
            == number
        {
            return Ok(ordinal);
        }
    }
    Ok(0)
}

impl OrdinaryEquipment {
    // DataZoneEquipment1553–1605. This call does not supply the optional CtrlZoneNum.
    fn purchased_air_is_on_equipment_list(
        &self,
        component_type: &str,
        component_name: &str,
        number_zones: i32,
    ) -> Result<bool, Error> {
        for number in 1..=number_zones {
            let row = self
                .equipment_lists
                .as_ref()
                .and_then(|rows| rows.get(at(number).ok()?))
                .ok_or(Error::Unavailable(
                    "actual CheckZoneEquipmentList ZoneEquipList1579",
                ))?;
            if row.name.is_empty() {
                continue;
            }
            for ordinal in 1..=row.number_of_equipment_types {
                let kind = row
                    .equipment_type_names
                    .as_ref()
                    .and_then(|rows| rows.get(ordinal - 1))
                    .ok_or(Error::Unavailable(
                        "actual CheckZoneEquipmentList EquipTypeName1584",
                    ))?;
                if !kind.eq_ignore_ascii_case(component_type) {
                    continue;
                }
                if component_name == "*" {
                    return Ok(true);
                }
                let name = row
                    .equipment_names
                    .as_ref()
                    .and_then(|rows| rows.get(ordinal - 1))
                    .ok_or(Error::Unavailable(
                        "actual CheckZoneEquipmentList EquipName1592",
                    ))?;
                if !name.eq_ignore_ascii_case(component_name) {
                    continue;
                }
                return Ok(true);
            }
        }
        Ok(false)
    }

    pub(super) fn initialize_source_purchased_air<D: SourceOutputDictionaryHost>(
        &mut self,
        unit_number: i32,
        controlled_zone: i32,
        schedules: &SelectedScheduleManager,
        o: &mut SourcePurchasedAirInitOwners<'_>,
        report: &mut SourceSizerReportingOwners<'_, D>,
        backend: &mut crate::heat_balance::error_backend::SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.purchased_air.initialize_once {
            let count = usize::try_from(self.purchased_air.number_purchased_air)
                .map_err(|_| Error::Unavailable("actual InitPurchasedAir allocation count1075"))?;
            // The real three allocations precede the three unconditional assignments.
            // Rust primitive storage values are not certified as Native allocation values.
            self.purchased_air.environment_flags = Some(vec![false; count]);
            self.purchased_air.sizing_flags = Some(vec![false; count]);
            self.purchased_air.unit_initializations_done = Some(vec![false; count]);
            self.purchased_air
                .environment_flags
                .as_mut()
                .ok_or(Error::Unavailable("actual environment flags1078"))?
                .fill(true);
            self.purchased_air
                .sizing_flags
                .as_mut()
                .ok_or(Error::Unavailable("actual sizing flags1079"))?
                .fill(true);
            self.purchased_air
                .unit_initializations_done
                .as_mut()
                .ok_or(Error::Unavailable("actual unit flags1080"))?
                .fill(false);
            self.purchased_air.initialize_once = false;
        }
        if !self.purchased_air.zone_equipment_list_checked && self.equipment_inputs_filled {
            self.purchased_air.zone_equipment_list_checked = true;
            for number in 1..=self.purchased_air.number_purchased_air {
                let row = self.purchased_unit(number)?;
                if row.plenum_exhaust_air_node_num > 0 {
                    return Err(Error::Unavailable(
                        "actual InitPurchasedAir GetReturnPlenumIndex1092",
                    ));
                }
                if self.purchased_air_is_on_equipment_list(
                    &row.c_object_name,
                    &row.name,
                    o.globals.num_of_zones,
                )? {
                    continue;
                }
                return Err(Error::Unavailable(
                    "actual InitPurchasedAir missing equipment-list severe1107",
                ));
            }
        }
        // Native obtains the unit reference before reading this retained flag.
        let _unit = self.purchased_unit(unit_number)?;
        if !bool_at(&self.purchased_air.unit_initializations_done, unit_number)? {
            *bool_at_mut(
                &mut self.purchased_air.unit_initializations_done,
                unit_number,
            )? = true;
            let supply = self.purchased_unit(unit_number)?.zone_supply_air_node_num;
            if supply > 0 {
                let config = self
                    .zone_config
                    .as_ref()
                    .and_then(|rows| rows.get(at(controlled_zone).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual InitPurchasedAir controlled Config1124",
                    ))?;
                if find_number(supply, &config.inlet_node_numbers, config.inlet_nodes)? == 0 {
                    let row = self.purchased_unit(unit_number)?;
                    backend.show_severe_error(&format!(
                        "InitPurchasedAir: In {} = {}",
                        row.c_object_name, row.name
                    ))?;
                    backend.show_continue_error(&format!(
                        "Zone Supply Air Node Name={} is not a zone inlet node.",
                        self.source_node_id(usize::try_from(supply).map_err(|_| {
                            Error::Unavailable("actual positive supply NodeID ordinal")
                        })?)?
                    ))?;
                    backend.show_continue_error(&format!(
                        "Check ZoneHVAC:EquipmentConnections for zone={}",
                        config.zone_name
                    ))?;
                    return Err(super::purchased_air_fatal(
                        backend,
                        "Preceding condition causes termination.",
                    ));
                }
            }
            let mut use_return_node = false;
            if self.purchased_unit(unit_number)?.zone_exhaust_air_node_num > 0 {
                let exhaust = self.purchased_unit(unit_number)?.zone_exhaust_air_node_num;
                let config = self
                    .zone_config
                    .as_ref()
                    .and_then(|rows| rows.get(at(controlled_zone).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual InitPurchasedAir controlled Config1144",
                    ))?;
                if find_number(exhaust, &config.exhaust_node_numbers, config.exhaust_nodes)? == 0 {
                    let row = self.purchased_unit(unit_number)?;
                    backend.show_severe_error(&format!(
                        "InitPurchasedAir: In {} = {}",
                        row.c_object_name, row.name
                    ))?;
                    backend.show_continue_error(&format!(
                        "Zone Exhaust Air Node Name={} is not a zone exhaust node.",
                        self.source_node_id(usize::try_from(exhaust).map_err(|_| {
                            Error::Unavailable("actual positive exhaust NodeID ordinal")
                        })?)?
                    ))?;
                    backend.show_continue_error(&format!(
                        "Check ZoneHVAC:EquipmentConnections for zone={}",
                        config.zone_name
                    ))?;
                    backend.show_continue_error(
                        "Zone return air node will be used for ideal loads recirculation air.",
                    )?;
                    use_return_node = true;
                } else {
                    self.purchased_unit_mut(unit_number)?
                        .zone_recirc_air_node_num =
                        self.purchased_unit(unit_number)?.zone_exhaust_air_node_num;
                }
            } else {
                use_return_node = true;
            }
            if use_return_node {
                let config = self
                    .zone_config
                    .as_ref()
                    .and_then(|rows| rows.get(at(controlled_zone).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual InitPurchasedAir controlled Config1163",
                    ))?;
                if config.return_nodes == 1 {
                    let node = *config
                        .return_node_numbers
                        .as_ref()
                        .and_then(|rows| rows.first())
                        .ok_or(Error::Unavailable(
                            "actual InitPurchasedAir ReturnNode1 1164",
                        ))?;
                    self.purchased_unit_mut(unit_number)?
                        .zone_recirc_air_node_num = i32::try_from(node).map_err(|_| {
                        Error::Unavailable("actual InitPurchasedAir return Node integer domain")
                    })?;
                } else if config.return_nodes > 1 {
                    // Native only emits warnings here: it does NOT assign ReturnNode(1).
                    return Err(Error::Unavailable(
                        "actual InitPurchasedAir multiple-return warning1166",
                    ));
                } else {
                    return Err(Error::Unavailable(
                        "actual InitPurchasedAir missing recirculation fatal1174",
                    ));
                }
            }
            let row = self.purchased_unit(unit_number)?;
            if row.outdoor_air && row.economizer_type != Econ::NoEconomizer {
                if row.cooling_limit == Limit::None || row.cooling_limit == Limit::Capacity {
                    return Err(Error::Unavailable(
                        "actual InitPurchasedAir OA economizer flow-limit severe1184",
                    ));
                }
            }
        }
        if !o.globals.system_sizing_calculation
            && bool_at(&self.purchased_air.sizing_flags, unit_number)?
        {
            self.size_source_purchased_air(unit_number, o, report)?;
            *bool_at_mut(&mut self.purchased_air.sizing_flags, unit_number)? = false;
        }
        if o.clock.begin_envrn_flag && bool_at(&self.purchased_air.environment_flags, unit_number)?
        {
            let row = self.purchased_unit_mut(unit_number)?;
            if row.heating_limit == Limit::FlowRate
                || row.heating_limit == Limit::FlowRateAndCapacity
            {
                row.max_heat_mass_flow_rate =
                    report.environment.std_rho_air * row.max_heat_vol_flow_rate;
            } else {
                row.max_heat_mass_flow_rate = 0.0;
            }
            if row.cooling_limit == Limit::FlowRate
                || row.cooling_limit == Limit::FlowRateAndCapacity
            {
                row.max_cool_mass_flow_rate =
                    report.environment.std_rho_air * row.max_cool_vol_flow_rate;
            } else {
                row.max_cool_mass_flow_rate = 0.0;
            }
            *bool_at_mut(&mut self.purchased_air.environment_flags, unit_number)? = false;
        }
        if !o.clock.begin_envrn_flag {
            *bool_at_mut(&mut self.purchased_air.environment_flags, unit_number)? = true;
        }
        let setpoint = o
            .fan
            .thermostat
            .zone_tstat_setpoints
            .as_ref()
            .and_then(|rows| rows.get(at(controlled_zone).ok()?))
            .ok_or(Error::Unavailable(
                "actual InitPurchasedAir zoneTstatSetpts1221",
            ))?;
        let row = self.purchased_unit(unit_number)?;
        if row.min_cool_supp_air_temp > setpoint.setpt_hi
            && setpoint.setpt_hi != 0.0
            && row.cooling_limit == Limit::None
        {
            let mut unit_on = true;
            if schedules.source_current_schedule_value(
                row.avail_sched
                    .ok_or(Error::Unavailable("actual PurchAir availSched1229"))?,
            )? <= 0.0
            {
                unit_on = false;
            }
            let mut cool_on = true;
            if schedules.source_current_schedule_value(
                row.cool_avail_sched
                    .ok_or(Error::Unavailable("actual PurchAir coolAvailSched1236"))?,
            )? <= 0.0
            {
                cool_on = false;
            }
            if unit_on && cool_on {
                if row.cool_err_index == 0 {
                    return Err(Error::Unavailable(
                        "actual InitPurchasedAir cooling setpoint severe1242",
                    ));
                }
                return Err(Error::Unavailable(
                    "actual InitPurchasedAir recurring cooling setpoint1260",
                ));
            }
        }
        if row.max_heat_supp_air_temp < setpoint.setpt_lo
            && setpoint.setpt_lo != 0.0
            && row.heating_limit == Limit::None
        {
            let mut unit_on = true;
            if schedules.source_current_schedule_value(
                row.avail_sched
                    .ok_or(Error::Unavailable("actual PurchAir availSched1277"))?,
            )? <= 0.0
            {
                unit_on = false;
            }
            let mut heat_on = true;
            if schedules.source_current_schedule_value(
                row.heat_avail_sched
                    .ok_or(Error::Unavailable("actual PurchAir heatAvailSched1285"))?,
            )? <= 0.0
            {
                heat_on = false;
            }
            if unit_on && heat_on {
                if row.heat_err_index == 0 {
                    return Err(Error::Unavailable(
                        "actual InitPurchasedAir heating setpoint severe1291",
                    ));
                }
                return Err(Error::Unavailable(
                    "actual InitPurchasedAir recurring heating setpoint1309",
                ));
            }
        }
        Ok(())
    }

    // Native ordinary SizePurchasedAir branch, with the real local Sizer lifecycle.
    #[allow(unused_variables, unused_assignments)]
    fn size_source_purchased_air<D: SourceOutputDictionaryHost>(
        &mut self,
        unit_number: i32,
        o: &mut SourcePurchasedAirInitOwners<'_>,
        report: &mut SourceSizerReportingOwners<'_, D>,
    ) -> Result<(), Error> {
        let cooling_air_vol_flow_des = 0.0;
        let heating_air_vol_flow_des = 0.0;
        let mut max_heat_vol_flow_rate_des = 0.0;
        let max_heat_vol_flow_rate_user = 0.0;
        let mut max_cool_vol_flow_rate_des = 0.0;
        let max_cool_vol_flow_rate_user = 0.0;
        let mut max_heat_sens_cap_des = 0.0;
        let mut max_heat_sens_cap_user = 0.0;
        let mut max_cool_tot_cap_des = 0.0;
        let mut max_cool_tot_cap_user = 0.0;
        o.sizing.zone_heating_only_fan = false;
        o.sizing.zone_cooling_only_fan = false;
        let comp_type = self.purchased_unit(unit_number)?.c_object_name.clone();
        let comp_name = self.purchased_unit(unit_number)?.name.clone();
        if o.sizing.current_zone_equipment_number > 0 {
            let _zone_sizing = o
                .sizing
                .zone_equipment_sizing
                .as_mut()
                .and_then(|r| r.get_mut(at(o.sizing.current_zone_equipment_number).ok()?))
                .ok_or(Error::Unavailable(
                    "actual SizePurchasedAir ZoneEqSizing1390",
                ))?;
            let mut sizing_string: String;
            let mut field_number: usize;
            let mut print_flag: bool;
            let mut errors_found = false;
            if self.purchased_unit(unit_number)?.hvac_sizing_index > 0 {
                o.sizing.autosizing.DataZoneNumber = self.purchased_unit(unit_number)?.zone_ptr;
                let zone_hvac_index = self.purchased_unit(unit_number)?.hvac_sizing_index;
                field_number = 5;
                print_flag = true;
                sizing_string = self
                    .purchased_numeric_field_name(unit_number, field_number)?
                    .to_owned()
                    + " [m3/s]";
                return Err(Error::Unavailable(
                    "actual SizePurchasedAir ZoneHVACSizing HeatingSAFMethod1407",
                ));
            } else {
                field_number = 5;
                sizing_string = self
                    .purchased_numeric_field_name(unit_number, field_number)?
                    .to_owned()
                    + " [m3/s]";
                let mut is_auto_size = false;
                print_flag = true;
                {
                    let row = self.purchased_unit(unit_number)?;
                    if row.max_heat_vol_flow_rate == -99999.0
                        && (row.heating_limit == Limit::FlowRate
                            || row.heating_limit == Limit::FlowRateAndCapacity)
                    {
                        is_auto_size = true;
                    }
                }
                if !is_auto_size && !o.sizing.zone_sizing_run_done {
                    if self.purchased_unit(unit_number)?.max_heat_vol_flow_rate > 0.0 {
                        let value = self.purchased_unit(unit_number)?.max_heat_vol_flow_rate;
                        let result = size_source_component(
                            Sizer::HeatingAirFlow,
                            &sizing_string,
                            value,
                            print_flag,
                            &comp_type,
                            &comp_name,
                            o.sizing,
                            o.globals,
                            o.hvac,
                            o.air_loop,
                            o.source_is_epjson,
                            report,
                            &mut errors_found,
                        )?;
                        self.purchased_unit_mut(unit_number)?.max_heat_vol_flow_rate = result;
                    }
                    max_heat_vol_flow_rate_des = 0.0;
                } else {
                    o.sizing.zone_heating_only_fan = true;
                    let temp_size = self.purchased_unit(unit_number)?.max_heat_vol_flow_rate;
                    max_heat_vol_flow_rate_des = size_source_component(
                        Sizer::HeatingAirFlow,
                        &sizing_string,
                        self.purchased_unit(unit_number)?.max_heat_vol_flow_rate,
                        print_flag,
                        &comp_type,
                        &comp_name,
                        o.sizing,
                        o.globals,
                        o.hvac,
                        o.air_loop,
                        o.source_is_epjson,
                        report,
                        &mut errors_found,
                    )?;
                    self.purchased_unit_mut(unit_number)?.max_heat_vol_flow_rate =
                        max_heat_vol_flow_rate_des;
                    o.sizing.zone_heating_only_fan = false;
                }
                is_auto_size = false;
                field_number = 6;
                sizing_string = self
                    .purchased_numeric_field_name(unit_number, field_number)?
                    .to_owned()
                    + " [m3/s]";
                {
                    let row = self.purchased_unit(unit_number)?;
                    if row.max_heat_sens_cap == -99999.0
                        && (row.heating_limit == Limit::Capacity
                            || row.heating_limit == Limit::FlowRateAndCapacity)
                    {
                        is_auto_size = true;
                    }
                }
                if !is_auto_size && !o.sizing.zone_sizing_run_done {
                    if self.purchased_unit(unit_number)?.max_heat_sens_cap > 0.0 {
                        max_heat_sens_cap_des = size_source_component(
                            Sizer::HeatingCapacity,
                            &sizing_string,
                            self.purchased_unit(unit_number)?.max_heat_sens_cap,
                            print_flag,
                            &comp_type,
                            &comp_name,
                            o.sizing,
                            o.globals,
                            o.hvac,
                            o.air_loop,
                            o.source_is_epjson,
                            report,
                            &mut errors_found,
                        )?;
                    }
                } else {
                    let temp_size = self.purchased_unit(unit_number)?.max_heat_sens_cap;
                    return Err(Error::Unavailable(
                        "actual SizePurchasedAir FinalZoneSizing.MinOA1746",
                    ));
                }
                if max_heat_sens_cap_des < 1.0 {
                    max_heat_sens_cap_des = 0.0;
                }
                if is_auto_size {
                    self.purchased_unit_mut(unit_number)?.max_heat_sens_cap = max_heat_sens_cap_des;
                    report_sizer_output(
                        report,
                        &comp_type,
                        &comp_name,
                        "Design Size Maximum Sensible Heating Capacity [W]",
                        max_heat_sens_cap_des,
                        None,
                        None,
                    )?;
                    if self.purchased_unit(unit_number)?.outdoor_air {
                        return Err(Error::Unavailable(
                            "actual SizePurchasedAir FinalZoneSizing.MinOA1763",
                        ));
                    }
                } else if self.purchased_unit(unit_number)?.max_heat_sens_cap > 0.0
                    && max_heat_sens_cap_des > 0.0
                {
                    max_heat_sens_cap_user = self.purchased_unit(unit_number)?.max_heat_sens_cap;
                    report_sizer_output(
                        report,
                        &comp_type,
                        &comp_name,
                        "Design Size Maximum Sensible Heating Capacity [W]",
                        max_heat_sens_cap_des,
                        Some("User-Specified Maximum Sensible Heating Capacity [W]"),
                        Some(max_heat_sens_cap_user),
                    )?;
                    if o.globals.display_extra_warnings {
                        if ((max_heat_sens_cap_des - max_heat_sens_cap_user).abs()
                            / max_heat_sens_cap_user)
                            > o.sizing.autosizing.AutoVsHardSizingThreshold
                        {
                            return Err(Error::Unavailable(
                                "actual SizePurchasedAir heating sizing difference ShowMessage1785",
                            ));
                        }
                    }
                }
                print_flag = true;
                is_auto_size = false;
                {
                    let row = self.purchased_unit(unit_number)?;
                    if row.max_cool_vol_flow_rate == -99999.0
                        && (row.cooling_limit == Limit::FlowRate
                            || row.cooling_limit == Limit::FlowRateAndCapacity
                            || (row.outdoor_air && row.economizer_type != Econ::NoEconomizer))
                    {
                        is_auto_size = true;
                    }
                }
                if !is_auto_size && !o.sizing.zone_sizing_run_done {
                    if self.purchased_unit(unit_number)?.max_cool_vol_flow_rate > 0.0 {
                        let string_override = if *o.source_is_epjson {
                            "maximum_cooling_air_flow_rate [m3/s]"
                        } else {
                            "Maximum Cooling Air Flow Rate [m3/s]"
                        };
                        let result = size_source_component(
                            Sizer::CoolingAirFlow,
                            string_override,
                            self.purchased_unit(unit_number)?.max_cool_vol_flow_rate,
                            print_flag,
                            &comp_type,
                            &comp_name,
                            o.sizing,
                            o.globals,
                            o.hvac,
                            o.air_loop,
                            o.source_is_epjson,
                            report,
                            &mut errors_found,
                        )?;
                        self.purchased_unit_mut(unit_number)?.max_cool_vol_flow_rate = result;
                    }
                } else {
                    o.sizing.zone_cooling_only_fan = true;
                    let temp_size = self.purchased_unit(unit_number)?.max_cool_vol_flow_rate;
                    let string_override = if *o.source_is_epjson {
                        "maximum_cooling_air_flow_rate [m3/s]"
                    } else {
                        "Maximum Cooling Air Flow Rate [m3/s]"
                    };
                    max_cool_vol_flow_rate_des = size_source_component(
                        Sizer::CoolingAirFlow,
                        string_override,
                        temp_size,
                        print_flag,
                        &comp_type,
                        &comp_name,
                        o.sizing,
                        o.globals,
                        o.hvac,
                        o.air_loop,
                        o.source_is_epjson,
                        report,
                        &mut errors_found,
                    )?;
                    self.purchased_unit_mut(unit_number)?.max_cool_vol_flow_rate =
                        max_cool_vol_flow_rate_des;
                    o.sizing.zone_cooling_only_fan = false;
                }
                is_auto_size = false;
                field_number = 8;
                sizing_string = self
                    .purchased_numeric_field_name(unit_number, field_number)?
                    .to_owned()
                    + " [m3/s]";
                {
                    let row = self.purchased_unit(unit_number)?;
                    if row.max_cool_tot_cap == -99999.0
                        && (row.cooling_limit == Limit::Capacity
                            || row.cooling_limit == Limit::FlowRateAndCapacity)
                    {
                        is_auto_size = true;
                    }
                }
                if !is_auto_size && !o.sizing.zone_sizing_run_done {
                    if self.purchased_unit(unit_number)?.max_cool_tot_cap > 0.0 {
                        let result = size_source_component(
                            Sizer::CoolingCapacity,
                            &sizing_string,
                            self.purchased_unit(unit_number)?.max_cool_tot_cap,
                            print_flag,
                            &comp_type,
                            &comp_name,
                            o.sizing,
                            o.globals,
                            o.hvac,
                            o.air_loop,
                            o.source_is_epjson,
                            report,
                            &mut errors_found,
                        )?;
                        self.purchased_unit_mut(unit_number)?.max_cool_tot_cap = result;
                    }
                } else {
                    o.sizing.zone_cooling_only_fan = true;
                    return Err(Error::Unavailable(
                        "actual SizePurchasedAir FinalZoneSizing.MinOA1851",
                    ));
                }
                if max_cool_tot_cap_des < 1.0 {
                    max_cool_tot_cap_des = 0.0;
                }
                if is_auto_size {
                    self.purchased_unit_mut(unit_number)?.max_cool_tot_cap = max_cool_tot_cap_des;
                    report_sizer_output(
                        report,
                        &comp_type,
                        &comp_name,
                        "Design Size Maximum Total Cooling Capacity [W]",
                        max_cool_tot_cap_des,
                        None,
                        None,
                    )?;
                    if self.purchased_unit(unit_number)?.outdoor_air {
                        return Err(Error::Unavailable(
                            "actual SizePurchasedAir FinalZoneSizing.MinOA1868",
                        ));
                    }
                } else if self.purchased_unit(unit_number)?.max_cool_tot_cap > 0.0
                    && max_cool_tot_cap_des > 0.0
                {
                    max_cool_tot_cap_user = self.purchased_unit(unit_number)?.max_cool_tot_cap;
                    report_sizer_output(
                        report,
                        &comp_type,
                        &comp_name,
                        "Design Size Maximum Total Cooling Capacity [W]",
                        max_cool_tot_cap_des,
                        Some("User-Specified Maximum Total Cooling Capacity [W]"),
                        Some(max_cool_tot_cap_user),
                    )?;
                    if o.globals.display_extra_warnings {
                        if ((max_cool_tot_cap_des - max_cool_tot_cap_user).abs()
                            / max_cool_tot_cap_user)
                            > o.sizing.autosizing.AutoVsHardSizingThreshold
                        {
                            return Err(Error::Unavailable(
                                "actual SizePurchasedAir cooling sizing difference ShowMessage1890",
                            ));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn purchased_numeric_field_name(
        &self,
        unit_number: i32,
        field_number: usize,
    ) -> Result<&str, Error> {
        self.purchased_air
            .numeric_fields
            .as_ref()
            .and_then(|rows| rows.get(at(unit_number).ok()?))
            .and_then(|row| row.field_names.as_ref())
            .and_then(|names| names.get(field_number.checked_sub(1)?))
            .map(String::as_str)
            .ok_or(Error::Unavailable(
                "actual PurchasedAir sizing FieldNames ordinal",
            ))
    }
}

#[cfg(all(test, feature = "native-controlled-air-cohort"))]
impl OrdinaryEquipment {
    // These typed0 identities are unused Source bookkeeping only; Native's sizing
    // unit has no Zone/Space allocation, and this helper does not construct one.
    pub(crate) fn source_sizer_cohort_cold_owner() -> Result<Self, Error> {
        Self::new_owners(None, &[ep_model::ZoneId(0)], &[ep_model::SpaceId(0)])
            .map_err(|_| Error::Unavailable("test-only Source cold sizing owner"))
    }
    pub(crate) fn source_sizer_cohort_full_size<D: SourceOutputDictionaryHost>(
        &mut self,
        o: &mut SourcePurchasedAirInitOwners<'_>,
        report: &mut SourceSizerReportingOwners<'_, D>,
    ) -> Result<(), Error> {
        self.size_source_purchased_air(1, o, report)
    }
}
