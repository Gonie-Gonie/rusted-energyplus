//! Genuine zero-setpoint-manager input and Init/Sim/Update child order.
use super::super::{Error, SourceHvacManager, SourceHvacOwners};
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
impl SourceHvacManager {
    pub(super) fn manage_source_setpoints(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.simulation.setpoints.get_input_flag {
            // GetSetPointManagerInputs298 has its own same flag test and successful clear310.
            if self.simulation.setpoints.get_input_flag {
                let node_list = o.input.get_object_def_max_args("NodeList").map_err(|_| {
                    Error::Unavailable("actual SetPoint NodeList schema definition376")
                })?;
                let _node_numbers = vec![
                    0_i32;
                    node_list.total().map_err(|_| Error::Unavailable(
                        "actual SetPoint NodeNums dimension total377"
                    ))?
                ];
                for family in TYPES {
                    if o.input.find_exact_family(family).is_none() {
                        continue;
                    }
                    return Err(Error::Unavailable(
                        "actual SetpointManager getObjectSchemaProps390",
                    ));
                }
                self.simulation.setpoints.get_input_flag = false;
            }
            self.simulation.setpoints.get_input_flag = false; // actual outer Manage264 repeated write
        }
        self.simulation.setpoints.manager_on = true;
        if *o.equipment.source_equipment_inputs_filled() && self.simulation.air_loop.inputs_filled {
            if self.simulation.setpoints.init_one_time_flag {
                if let Some(rows) = &self.simulation.setpoints.managers {
                    for row in rows {
                        match *row {}
                    }
                }
                // Actual VerifySetPointManagers nested1-based loops are empty at spms.size()==0.
                self.simulation.setpoints.init_one_time_flag = false;
            }
        }
        if (o.clock.begin_envrn_flag && self.simulation.setpoints.environment_flag)
            || self.simulation.setpoints.init_one_time_flag2
        {
            self.simulation.setpoints.manager_on = false;
            if let Some(rows) = &self.simulation.setpoints.managers {
                for row in rows {
                    match *row {}
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
            // Actual SimSetPointManagers, UpdateSetPointManagers, MixedAir calculate/update,
            // OutsideAirPretreat calculate/update each traverse this same real spms array.
            self.traverse_source_empty_setpoint_managers()?; // SimSetPointManagers2685
            self.traverse_source_empty_setpoint_managers()?; // UpdateSetPointManagers4133
            self.traverse_source_empty_setpoint_managers()?; // MixedAir calculate274
            self.traverse_source_empty_setpoint_managers()?; // UpdateMixedAirSetPoints
            self.traverse_source_empty_setpoint_managers()?; // OutsideAirPretreat calculate282
            self.traverse_source_empty_setpoint_managers()?; // UpdateOAPretreatSetPoints
        }
        Ok(())
    }
    fn traverse_source_empty_setpoint_managers(&self) -> Result<(), Error> {
        if let Some(rows) = &self.simulation.setpoints.managers {
            for row in rows {
                match *row {}
            }
        }
        Ok(())
    }
}

impl SourceHvacManager {
    pub(crate) fn fill_source_predefined_setpoint_manager_tables(&self) -> Result<(), Error> {
        if self
            .simulation
            .setpoints
            .managers
            .as_deref()
            .and_then(|rows| rows.first())
            .is_some()
        {
            return Err(Error::Unavailable(
                "actual FillPredefinedTablesForSetPointManagers first spm->type switch",
            ));
        }
        Ok(())
    }
}
