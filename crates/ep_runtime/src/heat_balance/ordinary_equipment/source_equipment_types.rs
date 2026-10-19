//! Reached DataZoneEquipment/ZoneEquipmentManager constructor records.
use crate::heat_balance::source_output_registration::SourceMeterData;
use std::convert::Infallible;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceAirIn {
    pub(super) inlet_node: usize,
    pub(super) outlet_node: usize,
    pub(super) supply_air_path_exists: bool,
    pub(super) air_loop_number: i32,
    pub(super) main_branch_index: i32,
    pub(super) supply_branch_index: i32,
    pub(super) air_distribution_unit_index: i32,
    pub(super) terminal_unit_sizing_index: usize,
    pub(super) supply_air_path_index: i32,
    /// Native omits this primitive from AirIn's constructor; it remains
    /// unrepresented until a genuine later supply-path writer is ported.
    pub(super) supply_air_path_outlet_node_index: Option<i32>,
    pub(super) coils: Option<Vec<Infallible>>,
}
// Keep the explicit Native constructor/default literals and ordered member initialization.
#[allow(clippy::derivable_impls)]
impl Default for SourceAirIn {
    fn default() -> Self {
        Self {
            inlet_node: 0,
            outlet_node: 0,
            supply_air_path_exists: false,
            air_loop_number: 0,
            main_branch_index: 0,
            supply_branch_index: 0,
            air_distribution_unit_index: 0,
            terminal_unit_sizing_index: 0,
            supply_air_path_index: 0,
            supply_air_path_outlet_node_index: None,
            coils: None,
        }
    }
}

/// Full reached EquipmentData constructor; positive nested/meter rows have no
/// writer in this input routine and remain actually unallocated.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceEquipmentData {
    pub(super) parent: bool,
    pub(super) number_sub_equipment: i32,
    pub(super) type_of: String,
    pub(super) name: String,
    pub(super) on: bool,
    pub(super) number_inlets: i32,
    pub(super) number_outlets: i32,
    pub(super) inlet_node_numbers: Option<Vec<i32>>,
    pub(super) outlet_node_numbers: Option<Vec<i32>>,
    pub(super) number_metered_variables: i32,
    pub(super) metered_variables: Option<Vec<SourceMeterData>>,
    pub(super) sub_equipment: Option<Vec<Infallible>>,
    pub(super) energy_transfer_component: i32,
    pub(super) zone_equipment_to_plant_pointer: i32,
    pub(super) total_plant_supply_electricity: f64,
    pub(super) total_plant_supply_gas: f64,
    pub(super) total_plant_supply_purchased: f64,
    pub(super) operating_mode: i32,
}
impl Default for SourceEquipmentData {
    fn default() -> Self {
        Self {
            parent: false,
            number_sub_equipment: 0,
            type_of: String::new(),
            name: String::new(),
            on: true,
            number_inlets: 0,
            number_outlets: 0,
            inlet_node_numbers: None,
            outlet_node_numbers: None,
            number_metered_variables: 0,
            metered_variables: None,
            sub_equipment: None,
            energy_transfer_component: 0,
            zone_equipment_to_plant_pointer: 0,
            total_plant_supply_electricity: 0.0,
            total_plant_supply_gas: 0.0,
            total_plant_supply_purchased: 0.0,
            operating_mode: 0,
        }
    }
}

/// Native SimulationOrder72-80; allocation does not populate simulation order.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceSimulationOrder {
    pub(super) equipment_type_name: String,
    pub(super) equipment_type: i32,
    pub(super) equipment_name: String,
    pub(super) equipment_pointer: i32,
    pub(super) cooling_priority: i32,
    pub(super) heating_priority: i32,
}
impl Default for SourceSimulationOrder {
    fn default() -> Self {
        Self {
            equipment_type_name: String::new(),
            equipment_type: -1,
            equipment_name: String::new(),
            equipment_pointer: 0,
            cooling_priority: 0,
            heating_priority: 0,
        }
    }
}
