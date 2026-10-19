//! Actual HVACManager::ResetNodeData2151-2197 on the sole retained node owner.
use super::OrdinaryEquipment;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;

impl OrdinaryEquipment {
    pub(crate) fn reset_source_node_data(&mut self) -> Result<(), Error> {
        // Native2161 returns before touching Node, defaults or MoreNodeInfo.
        if self.nodes.number_of_nodes == 0 {
            return Ok(());
        }
        let defaults = &self.nodes.default_node_values;
        let nodes = self.nodes.physical.as_mut().ok_or(Error::Unavailable(
            "actual positive ResetNodeData Node allocation2165",
        ))?;
        // Native traverses the whole allocation, independently of NumOfNodes.
        for node in nodes {
            node.temperature = defaults.temperature;
            node.temperature_min = defaults.temperature_min;
            node.temperature_max = defaults.temperature_max;
            node.temperature_setpoint = defaults.temperature_setpoint;
            node.mass_flow_rate = defaults.mass_flow_rate;
            node.mass_flow_rate_min = defaults.mass_flow_rate_min;
            node.mass_flow_rate_max = defaults.mass_flow_rate_max;
            node.mass_flow_rate_min_available = defaults.mass_flow_rate_min_available;
            node.mass_flow_rate_max_available = defaults.mass_flow_rate_max_available;
            node.mass_flow_rate_setpoint = defaults.mass_flow_rate_setpoint;
            node.quality = defaults.quality;
            node.pressure = defaults.pressure;
            node.enthalpy = defaults.enthalpy;
            node.humidity_ratio = defaults.humidity_ratio;
            node.humidity_ratio_min = defaults.humidity_ratio_min;
            node.humidity_ratio_max = defaults.humidity_ratio_max;
            node.humidity_ratio_setpoint = defaults.humidity_ratio_setpoint;
            node.temperature_setpoint_high = defaults.temperature_setpoint_high;
            node.temperature_setpoint_low = defaults.temperature_setpoint_low;
        }
        if let Some(rows) = self.nodes.more_node_info.as_mut() {
            for row in rows {
                row.reset_source_node_data(defaults.temperature, defaults.enthalpy);
            }
        }
        Ok(())
    }
}
