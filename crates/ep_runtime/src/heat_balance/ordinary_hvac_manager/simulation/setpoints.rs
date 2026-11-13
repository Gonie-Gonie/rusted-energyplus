//! Native Scheduled Temperature GetInput/Init/Verify/Sim/Update on real Node owners.
use super::super::{Error, SourceHvacManager, SourceHvacOwners};
use super::state::SourceScheduledSetPoint;
use crate::heat_balance::error_backend::SourceDiagnosticFlags;
use crate::heat_balance::selected_input_processor::{
    SourceInputFamily, SourceInputStatisticsValue as Value,
};
const TYPES: [&str; 32] = [
    "SetpointManager:Scheduled",
    "SetpointManager:Scheduled:DualSetpoint",
    "SetpointManager:OutdoorAirReset",
    "SetpointManager:SingleZone:Reheat",
    "SetpointManager:SingleZone:Heating",
    "SetpointManager:SingleZone:Cooling",
    "SetpointManager:SingleZone:Humidity:Minimum",
    "SetpointManager:SingleZone:Humidity:Maximum",
    "SetpointManager:MixedAir",
    "SetpointManager:OutdoorAirPretreat",
    "SetpointManager:Warmest",
    "SetpointManager:Coldest",
    "SetpointManager:WarmestTemperatureFlow",
    "SetpointManager:ReturnAirBypassFlow",
    "SetpointManager:MultiZone:Cooling:Average",
    "SetpointManager:MultiZone:Heating:Average",
    "SetpointManager:MultiZone:MinimumHumidity:Average",
    "SetpointManager:MultiZone:MaximumHumidity:Average",
    "SetpointManager:MultiZone:Humidity:Minimum",
    "SetpointManager:MultiZone:Humidity:Maximum",
    "SetpointManager:FollowOutdoorAirTemperature",
    "SetpointManager:FollowSystemNodeTemperature",
    "SetpointManager:FollowGroundTemperature",
    "SetpointManager:CondenserEnteringReset",
    "SetpointManager:CondenserEnteringReset:Ideal",
    "SetpointManager:SingleZone:OneStageCooling",
    "SetpointManager:SingleZone:OneStageHeating",
    "SetpointManager:ReturnTemperature:ChilledWater",
    "SetpointManager:ReturnTemperature:HotWater",
    "SetpointManager:ScheduledTES",
    "SetpointManager:SystemNodeReset:Temperature",
    "SetpointManager:SystemNodeReset:Humidity",
];

const SCHEDULED: &str = "SetpointManager:Scheduled";
fn equipment_error(
    error: crate::heat_balance::ordinary_equipment::OrdinaryEquipmentUnavailable,
) -> Error {
    Error::SourceInput(Box::new(crate::RuntimeError::WarmupUnavailable {
        reason: format!("actual Scheduled physical Node operation: {error}"),
    }))
}
fn alpha(fields: &std::collections::BTreeMap<String, Value>, name: &str) -> Result<String, Error> {
    match fields.get(name) {
        Some(Value::String(value)) => Ok(value.to_ascii_uppercase()),
        None => Ok(String::new()), // None of the three Scheduled schema fields has a default.
        _ => Err(Error::Unavailable("actual Scheduled schema alpha type")),
    }
}
impl SourceHvacManager {
    pub(super) fn manage_source_setpoints(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.simulation.setpoints.get_input_flag {
            self.get_source_setpoint_input(o)?;
            self.simulation.setpoints.get_input_flag = false; // Native264 repeats getter310
        }
        self.simulation.setpoints.manager_on = true;
        if *o.equipment.source_equipment_inputs_filled() && self.simulation.air_loop.inputs_filled {
            if self.simulation.setpoints.init_one_time_flag {
                // Scheduled takes the Native1728-2245 default branch; real Verify still runs.
                self.verify_source_scheduled_setpoints(o)?;
                self.simulation.setpoints.init_one_time_flag = false;
            }
        }
        if (o.clock.begin_envrn_flag && self.simulation.setpoints.environment_flag)
            || self.simulation.setpoints.init_one_time_flag2
        {
            self.simulation.setpoints.manager_on = false;
            if let Some(rows) = &self.simulation.setpoints.managers {
                for row in rows {
                    for &node in &row.control_nodes {
                        // Native2272 reads current schedule inside the actual Node loop.
                        // This does not write the separate spm->setPt member.
                        let value =
                            o.schedules
                                .source_current_schedule_value(row.schedule.ok_or(
                                    Error::Unavailable("actual Scheduled schedule pointer2272"),
                                )?)?;
                        o.equipment
                            .set_source_scheduled_temperature_setpoint(node, value)
                            .map_err(equipment_error)?;
                    }
                }
            }
            self.simulation.setpoints.environment_flag = false;
            if !self.simulation.setpoints.init_one_time_flag {
                self.simulation.setpoints.init_one_time_flag2 = false;
            }
        }
        if !o.clock.begin_envrn_flag {
            self.simulation.setpoints.environment_flag = true;
        }
        if self.simulation.setpoints.manager_on {
            if let Some(rows) = &mut self.simulation.setpoints.managers {
                // Native2718 calculates all eligible rows BEFORE Update4179 writes nodes.
                for row in rows.iter_mut() {
                    row.setpoint = o
                        .schedules
                        .source_current_schedule_value(row.schedule.ok_or(
                            Error::Unavailable("actual Scheduled schedule pointer2733"),
                        )?)?;
                }
                for row in rows.iter() {
                    for &node in &row.control_nodes {
                        o.equipment
                            .set_source_scheduled_temperature_setpoint(node, row.setpoint)
                            .map_err(equipment_error)?;
                    }
                }
                // Subsequent MixedAir/OAPretreat filtered traversals have no selected rows.
                // These are not unported successful calls for any other manager type.
            }
        }
        Ok(())
    }

    fn get_source_setpoint_input(&mut self, o: &mut SourceHvacOwners<'_, '_>) -> Result<(), Error> {
        if !self.simulation.setpoints.get_input_flag {
            return Ok(());
        }
        let mut errors_found = false;
        let definition = o
            .input
            .get_object_def_max_args("NodeList")
            .map_err(|_| Error::Unavailable("actual SetPoint NodeList schema definition376"))?;
        let mut node_numbers = vec![
            0_usize;
            definition.total().map_err(|_| Error::Unavailable(
                "actual SetPoint NodeNums dimension377"
            ))?
        ];
        for family in TYPES {
            let Some(instances) = o.input.find_exact_family(family) else {
                continue;
            };
            if family != SCHEDULED {
                return Err(Error::Unavailable(
                    "actual unsupported positive SetpointManager schema390",
                ));
            }
            // Actual schema lookup does not insert a null family or getObjectItem defaults.
            let schema = o
                .input
                .source_object_definition(family)
                .map_err(|_| Error::Unavailable("actual Scheduled getObjectSchemaProps390"))?;
            if (
                schema.alpha_fields,
                schema.numeric_fields,
                schema.alpha_extensions,
                schema.numeric_extensions,
            ) != (4, 0, 0, 0)
                || schema.extension.is_some()
            {
                return Err(Error::Unavailable(
                    "actual Scheduled legacy schema association",
                ));
            }
            let names: Vec<String> = match instances {
                SourceInputFamily::Null => Vec::new(),
                SourceInputFamily::Objects(rows) => {
                    rows.iter().map(|r| r.object_name.clone()).collect()
                }
            };
            // initializeMaps retains actual JSON DOM map order. Native source epJSON
            // assigns increasing fake IDF numbers; IDF reads its original metadata.
            let mut idf_numbers = Vec::with_capacity(names.len());
            for (i, name) in names.iter().enumerate() {
                let number = if *o.source_is_epjson {
                    i32::try_from(i + 1)
                        .map_err(|_| Error::Unavailable("Scheduled fake IDF ordinal408"))?
                } else {
                    match o
                        .input
                        .source_raw_object_fields(family, name)
                        .map_err(|_| Error::Unavailable("Scheduled raw metadata410"))?
                        .get("idf_order")
                    {
                        Some(Value::Number(token)) => token
                            .parse::<i32>()
                            .map_err(|_| Error::Unavailable("Scheduled actual IDF int410"))?,
                        _ => {
                            return Err(Error::Unavailable(
                                "Scheduled missing actual IDF order410",
                            ));
                        }
                    }
                };
                idf_numbers.push(number);
            }
            let mut sorted = idf_numbers.clone();
            sorted.sort();
            for number in sorted {
                let index = idf_numbers
                    .iter()
                    .position(|&v| v == number)
                    .ok_or(Error::Unavailable("Scheduled source find IDF number422"))?;
                let key = &names[index];
                let fields = o
                    .input
                    .source_raw_object_fields(family, key)
                    .map_err(|_| Error::Unavailable("Scheduled source JSON fields425"))?
                    .clone();
                o.input.mark_object_as_used(family, key);
                let name = key.to_ascii_uppercase();
                if self.simulation.setpoints.manager_map.contains_key(&name) {
                    o.backend.show_source_severe_error(
                        &format!("GetSetPointManagerInputs: {family} = {name}, duplicate name."),
                        &SourceDiagnosticFlags {
                            kickoff: &o.globals.kickoff_simulation,
                            developer: &o.project.system.developer_flag,
                            warmup: &o.clock.warmup_flag,
                            doing_sizing: &o.globals.doing_sizing,
                        },
                    )?;
                    errors_found = true;
                }
                let rows = self
                    .simulation
                    .setpoints
                    .managers
                    .get_or_insert_with(Vec::new);
                rows.push(SourceScheduledSetPoint {
                    name: name.clone(),
                    control_variable: -1,
                    schedule: None,
                    setpoint: 0.0,
                    control_nodes: Vec::new(),
                });
                let index = rows.len() - 1;
                self.simulation.setpoints.manager_map.insert(
                    name.clone(),
                    i32::try_from(rows.len())
                        .map_err(|_| Error::Unavailable("Scheduled spmMap signed ordinal537"))?,
                );
                let control = alpha(&fields, "control_variable")?;
                if control != "TEMPERATURE" {
                    return Err(Error::Unavailable(
                        "Scheduled positive control other than Temperature",
                    ));
                }
                rows[index].control_variable = 0; // Native HVAC::CtrlVarType::Temp
                let schedule_name = alpha(&fields, "schedule_name")?;
                rows[index].schedule = o.schedules.get_source_schedule(&schedule_name)?;
                if rows[index].schedule.is_none() {
                    o.backend.show_source_severe_error(
                        &format!("GetSetPointManagerInputs: {family} = {name}"),
                        &SourceDiagnosticFlags {
                            kickoff: &o.globals.kickoff_simulation,
                            developer: &o.project.system.developer_flag,
                            warmup: &o.clock.warmup_flag,
                            doing_sizing: &o.globals.doing_sizing,
                        },
                    )?;
                    // Native schema has no retaincase, so getAlphaFieldValue already uppercases.
                    o.backend.show_continue_error(&format!(
                        "schedule_name = {schedule_name}, item not found."
                    ))?;
                    errors_found = true;
                }
                rows[index].setpoint = 0.0;
                let target = alpha(&fields, "setpoint_node_or_nodelist_name")?;
                let mut node_list_error = false;
                let mut num_nodes = 0;
                o.equipment
                    .get_source_scheduled_setpoint_node_nums(
                        o.input,
                        &mut self.ancillary.branch_node_connections,
                        &target,
                        &name,
                        &mut num_nodes,
                        &mut node_numbers,
                        &mut node_list_error,
                    )
                    .map_err(equipment_error)?;
                if !node_list_error {
                    rows[index].control_nodes.extend_from_slice(
                        node_numbers
                            .get(..num_nodes)
                            .ok_or(Error::Unavailable("Scheduled actual NodeNums range1410"))?,
                    );
                } else {
                    errors_found = true;
                }
            }
        }
        if errors_found {
            return match o.backend.show_source_fatal_error(
                "GetSetPointManagerInputs: Errors found in input.  Program terminates.",
                &SourceDiagnosticFlags {
                    kickoff: &o.globals.kickoff_simulation,
                    developer: &o.project.system.developer_flag,
                    warmup: &o.clock.warmup_flag,
                    doing_sizing: &o.globals.doing_sizing,
                },
            ) {
                Err(stop) => Err(stop),
                Ok(never) => match never {},
            };
        }
        self.simulation.setpoints.get_input_flag = false;
        Ok(())
    }

    fn verify_source_scheduled_setpoints(
        &self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        let rows = self
            .simulation
            .setpoints
            .managers
            .as_deref()
            .unwrap_or_default();
        for (i, row) in rows.iter().enumerate() {
            for (a, &node) in row.control_nodes.iter().enumerate() {
                for &other in row.control_nodes.iter().skip(a + 1) {
                    if node != other {
                        continue;
                    }
                    o.backend.show_source_warning_error(
                        &format!("{SCHEDULED} =\"{}\"", row.name),
                        &SourceDiagnosticFlags {
                            kickoff: &o.globals.kickoff_simulation,
                            developer: &o.project.system.developer_flag,
                            warmup: &o.clock.warmup_flag,
                            doing_sizing: &o.globals.doing_sizing,
                        },
                    )?;
                    o.backend.show_continue_error(&format!(
                        "...duplicate node specified = {}",
                        o.equipment.source_node_id(node)?
                    ))?;
                    o.backend
                        .show_continue_error("...control type variable    = TEMPERATURE")?;
                }
            }
            for other in rows.iter().skip(i + 1) {
                if row.control_variable != other.control_variable {
                    continue;
                }
                for &node in &row.control_nodes {
                    for &other_node in &other.control_nodes {
                        if node != other_node {
                            continue;
                        }
                        o.backend.show_source_warning_error(
                            &format!("{SCHEDULED}=\"{}\"", row.name),
                            &SourceDiagnosticFlags {
                                kickoff: &o.globals.kickoff_simulation,
                                developer: &o.project.system.developer_flag,
                                warmup: &o.clock.warmup_flag,
                                doing_sizing: &o.globals.doing_sizing,
                            },
                        )?;
                        o.backend.show_continue_error(
                            "...setpoint node conflicts with another setpoint manager.",
                        )?;
                        o.backend.show_continue_error(&format!(
                            "...conflicting setpoint manager = {SCHEDULED}:\"{}\"",
                            other.name
                        ))?;
                        o.backend.show_continue_error(&format!(
                            "...conflicting node name = {}",
                            o.equipment.source_node_id(node)?
                        ))?;
                        o.backend
                            .show_continue_error("...control type variable = Temperature")?;
                        // Native1660 does NOT set ErrorsFound: conflicts warn, last writer wins.
                    }
                }
            }
        }
        Ok(())
    }

    pub(crate) fn fill_source_predefined_setpoint_manager_tables(&self) -> Result<(), Error> {
        if let Some(rows) = &self.simulation.setpoints.managers {
            for _row in rows {
                // Native1477 Scheduled is the default/no-entry branch. This owned
                // subtype cannot represent an unsupported positive manager type.
            }
        }
        Ok(())
    }
}
