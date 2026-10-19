//! Genuine DataZoneEquipment Config environment/timestep physical Node writes.
use super::{EquipmentConfig, OrdinaryEquipment, SourceNodeData};
use crate::heat_balance::ordinary_heat_balance_initialization::SourceContaminantArrays;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState,
};
use crate::psychrometrics::energyplus_psy_h_fn_tdb_w;
use crate::weather::day::WeatherEnvironmentState;

fn node_index(ordinal: usize, nodes: &[SourceNodeData]) -> Result<usize, Error> {
    ordinal
        .checked_sub(1)
        .filter(|&at| at < nodes.len())
        .ok_or(Error::Unavailable(
            "actual Config physical Node positive ordinal",
        ))
}
fn begin_environment_node(
    nodes: &mut [SourceNodeData],
    ordinal: usize,
    environment: &WeatherEnvironmentState,
    control: &SelectedHeatBalanceInputState,
    contaminants: &SourceContaminantArrays,
) -> Result<(), Error> {
    let at = node_index(ordinal, nodes)?;
    let node = &mut nodes[at];
    node.temperature = 20.0;
    node.mass_flow_rate = 0.0;
    node.quality = 1.0;
    node.pressure = environment.current_weather.out_baro_press;
    node.humidity_ratio = environment.current_weather.out_hum_rat;
    node.enthalpy = energyplus_psy_h_fn_tdb_w(node.temperature, node.humidity_ratio);
    if control.carbon_dioxide_simulation {
        node.carbon_dioxide = contaminants.outdoor_co2;
    }
    if control.generic_contaminant_simulation {
        node.generic_contaminant = contaminants.outdoor_gc;
    }
    Ok(())
}
impl EquipmentConfig {
    fn begin_source_environment(
        &self,
        nodes: &mut [SourceNodeData],
        environment: &WeatherEnvironmentState,
        control: &SelectedHeatBalanceInputState,
        contaminants: &SourceContaminantArrays,
    ) -> Result<(), Error> {
        begin_environment_node(nodes, self.zone_node, environment, control, contaminants)?;
        if let Some(inlets) = &self.inlet_node_numbers {
            for &ordinal in inlets {
                begin_environment_node(nodes, ordinal, environment, control, contaminants)?;
            }
        }
        if let Some(exhausts) = &self.exhaust_node_numbers {
            for &ordinal in exhausts {
                begin_environment_node(nodes, ordinal, environment, control, contaminants)?;
            }
        }
        if self.return_nodes > 0 {
            if let Some(returns) = &self.return_node_numbers {
                for &ordinal in returns {
                    begin_environment_node(nodes, ordinal, environment, control, contaminants)?;
                }
            }
        }
        Ok(())
    }
    fn initialize_source_timestep(
        &mut self,
        nodes: &mut [SourceNodeData],
        first_hvac_iteration: bool,
        control: &SelectedHeatBalanceInputState,
    ) -> Result<(), Error> {
        // Native2305 binds the actual zone Node even when First=false or
        // the real exhaust allocation is empty; check it before ExcessZoneExh.
        let zone_at = node_index(self.zone_node, nodes)?;
        self.excess_zone_exhaust = 0.0;
        if first_hvac_iteration {
            if let Some(exhausts) = &self.exhaust_node_numbers {
                for &ordinal in exhausts {
                    let at = node_index(ordinal, nodes)?;
                    // Each RHS is the current same Node field immediately
                    // before its real write. Aliased ordinals are not cloned.
                    nodes[at].temperature = nodes[zone_at].temperature;
                    nodes[at].humidity_ratio = nodes[zone_at].humidity_ratio;
                    nodes[at].enthalpy = nodes[zone_at].enthalpy;
                    nodes[at].pressure = nodes[zone_at].pressure;
                    nodes[at].quality = nodes[zone_at].quality;
                    nodes[at].mass_flow_rate = 0.0;
                    nodes[at].mass_flow_rate_max_available = 0.0;
                    nodes[at].mass_flow_rate_min_available = 0.0;
                    if control.carbon_dioxide_simulation {
                        nodes[at].carbon_dioxide = nodes[zone_at].carbon_dioxide;
                    }
                    if control.generic_contaminant_simulation {
                        nodes[at].generic_contaminant = nodes[zone_at].generic_contaminant;
                    }
                }
            }
        }
        Ok(())
    }
}
impl OrdinaryEquipment {
    pub(crate) fn begin_source_equipment_environment(
        &mut self,
        space: bool,
        environment: &WeatherEnvironmentState,
        control: &SelectedHeatBalanceInputState,
        contaminants: &SourceContaminantArrays,
    ) -> Result<(), Error> {
        let configs = if space {
            &self.space_config
        } else {
            &self.zone_config
        };
        if let Some(configs) = configs {
            for config in configs {
                if !config.is_controlled {
                    continue;
                }
                let nodes = self
                    .nodes
                    .physical
                    .as_deref_mut()
                    .ok_or(Error::Unavailable(
                        "actual Config beginEnvirnInit physical allocation2236",
                    ))?;
                config.begin_source_environment(nodes, environment, control, contaminants)?;
            }
        }
        Ok(())
    }
    pub(crate) fn initialize_source_equipment_timestep(
        &mut self,
        space: bool,
        first_hvac_iteration: bool,
        control: &SelectedHeatBalanceInputState,
    ) -> Result<(), Error> {
        let configs = if space {
            &mut self.space_config
        } else {
            &mut self.zone_config
        };
        if let Some(configs) = configs {
            for config in configs {
                if !config.is_controlled {
                    continue;
                }
                let nodes = self
                    .nodes
                    .physical
                    .as_deref_mut()
                    .ok_or(Error::Unavailable(
                        "actual Config hvacTimeStepInit physical allocation2305",
                    ))?;
                config.initialize_source_timestep(nodes, first_hvac_iteration, control)?;
            }
        }
        Ok(())
    }
}
