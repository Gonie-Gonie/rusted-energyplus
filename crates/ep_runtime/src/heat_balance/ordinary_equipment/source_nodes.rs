//! Genuine DataLoopNode records and AssignNodeNumber650-743 on the sole owner.
use super::{NodeOwner, Unavailable, source_marked_nodes::SourceMarkedNode};
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;

/// Native DataLoopNode.hh66-75, including its actual Blank constructor state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub(super) enum SourceNodeFluidType {
    Invalid = -1,
    Blank = 0,
    Air = 1,
    Water = 2,
    Steam = 3,
    Electric = 4,
    Num = 5,
}

/// Native DataLoopNode.hh439-500. These are the physical fields, not typed
/// model node IDs or the six-field diagnostic NodeStateStore projection.
#[derive(Clone, Debug, PartialEq)]
pub(super) struct SourceNodeData {
    pub(super) fluid_type: SourceNodeFluidType,
    pub(super) fluid_index: i32,
    pub(super) temperature: f64,
    pub(super) temperature_min: f64,
    pub(super) temperature_max: f64,
    pub(super) temperature_setpoint: f64,
    pub(super) temperature_last_timestep: f64,
    pub(super) mass_flow_rate_request: f64,
    pub(super) mass_flow_rate: f64,
    pub(super) mass_flow_rate_min: f64,
    pub(super) mass_flow_rate_max: f64,
    pub(super) mass_flow_rate_min_available: f64,
    pub(super) mass_flow_rate_max_available: f64,
    pub(super) mass_flow_rate_setpoint: f64,
    pub(super) quality: f64,
    pub(super) pressure: f64,
    pub(super) enthalpy: f64,
    pub(super) enthalpy_last_timestep: f64,
    pub(super) humidity_ratio: f64,
    pub(super) humidity_ratio_min: f64,
    pub(super) humidity_ratio_max: f64,
    pub(super) humidity_ratio_setpoint: f64,
    pub(super) temperature_setpoint_high: f64,
    pub(super) temperature_setpoint_low: f64,
    pub(super) height: f64,
    pub(super) is_local_node: bool,
    pub(super) outdoor_air_dry_bulb_schedule: Option<SourceScheduleIndex>,
    pub(super) outdoor_air_wet_bulb_schedule: Option<SourceScheduleIndex>,
    pub(super) outdoor_air_wind_speed_schedule: Option<SourceScheduleIndex>,
    pub(super) outdoor_air_wind_direction_schedule: Option<SourceScheduleIndex>,
    pub(super) outdoor_air_dry_bulb: f64,
    pub(super) ems_override_outdoor_air_dry_bulb: bool,
    pub(super) ems_value_outdoor_air_dry_bulb: f64,
    pub(super) outdoor_air_wet_bulb: f64,
    pub(super) ems_override_outdoor_air_wet_bulb: bool,
    pub(super) ems_value_outdoor_air_wet_bulb: f64,
    pub(super) outdoor_air_wind_speed: f64,
    pub(super) ems_override_outdoor_air_wind_speed: bool,
    pub(super) ems_value_outdoor_air_wind_speed: f64,
    pub(super) outdoor_air_wind_direction: f64,
    pub(super) ems_override_outdoor_air_wind_direction: bool,
    pub(super) ems_value_outdoor_air_wind_direction: f64,
    pub(super) carbon_dioxide: f64,
    pub(super) carbon_dioxide_setpoint: f64,
    pub(super) generic_contaminant: f64,
    pub(super) generic_contaminant_setpoint: f64,
    pub(super) setpoint_manager_wet_bulb_reporting_required: bool,
    pub(super) plant_node_error_message_issued: bool,
}
impl Default for SourceNodeData {
    fn default() -> Self {
        Self {
            fluid_type: SourceNodeFluidType::Blank,
            fluid_index: 0,
            temperature: 0.0,
            temperature_min: 0.0,
            temperature_max: 0.0,
            temperature_setpoint: -999.0,
            temperature_last_timestep: 0.0,
            mass_flow_rate_request: 0.0,
            mass_flow_rate: 0.0,
            mass_flow_rate_min: 0.0,
            mass_flow_rate_max: -999.0,
            mass_flow_rate_min_available: 0.0,
            mass_flow_rate_max_available: 0.0,
            mass_flow_rate_setpoint: 0.0,
            quality: 0.0,
            pressure: 101_325.0,
            enthalpy: 0.0,
            enthalpy_last_timestep: 0.0,
            humidity_ratio: 0.0,
            humidity_ratio_min: -999.0,
            humidity_ratio_max: -999.0,
            humidity_ratio_setpoint: -999.0,
            temperature_setpoint_high: -999.0,
            temperature_setpoint_low: -999.0,
            height: -1.0,
            is_local_node: false,
            outdoor_air_dry_bulb_schedule: None,
            outdoor_air_wet_bulb_schedule: None,
            outdoor_air_wind_speed_schedule: None,
            outdoor_air_wind_direction_schedule: None,
            outdoor_air_dry_bulb: 0.0,
            ems_override_outdoor_air_dry_bulb: false,
            ems_value_outdoor_air_dry_bulb: 0.0,
            outdoor_air_wet_bulb: 0.0,
            ems_override_outdoor_air_wet_bulb: false,
            ems_value_outdoor_air_wet_bulb: 0.0,
            outdoor_air_wind_speed: 0.0,
            ems_override_outdoor_air_wind_speed: false,
            ems_value_outdoor_air_wind_speed: 0.0,
            outdoor_air_wind_direction: 0.0,
            ems_override_outdoor_air_wind_direction: false,
            ems_value_outdoor_air_wind_direction: 0.0,
            carbon_dioxide: 0.0,
            carbon_dioxide_setpoint: 0.0,
            generic_contaminant: 0.0,
            generic_contaminant_setpoint: 0.0,
            setpoint_manager_wet_bulb_reporting_required: false,
            plant_node_error_message_issued: false,
        }
    }
}

/// Actual DataLoopNode.hh525-530 and HVAC::CtrlVarType::Num nine entries.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceNodeSetpointCheck {
    pub(super) needs_setpoint_checking: bool,
    pub(super) check_setpoint: [bool; 9],
}
fn unavailable(text: &'static str) -> Unavailable { Unavailable::Input(text.into()) }
impl NodeOwner {
    pub(super) fn assign_source_node_number(
        &mut self, name: &str, fluid_type: SourceNodeFluidType,
        _errors_found: &mut bool,
    ) -> Result<usize, Unavailable> {
        if matches!(fluid_type, SourceNodeFluidType::Invalid | SourceNodeFluidType::Num) {
            return Err(unavailable("actual AssignNodeNumber675 invalid-fluid severe diagnostic"));
        }
        if self.number_of_unique_node_names > 0 {
            let names = self.node_ids.as_ref().ok_or(Unavailable::InconsistentOwner)?;
            let mut found = None;
            for number in 1..=self.number_of_unique_node_names {
                let entry = names.get(number).ok_or(Unavailable::InconsistentOwner)?;
                // Util::FindItemInList is explicitly case-sensitive; the
                // actual IP getter already uppercases non-retaincase alphas.
                if entry == name { found = Some(number); break; }
            }
            if let Some(number) = found {
                let reference = self.node_references.as_mut().and_then(|rows| rows.get_mut(number - 1))
                    .ok_or(Unavailable::InconsistentOwner)?;
                *reference = reference.checked_add(1).ok_or(Unavailable::InconsistentOwner)?;
                let row = self.physical.as_mut().and_then(|rows| rows.get_mut(number - 1))
                    .ok_or(Unavailable::InconsistentOwner)?;
                if fluid_type != SourceNodeFluidType::Blank
                    && row.fluid_type != fluid_type && row.fluid_type != SourceNodeFluidType::Blank {
                    return Err(unavailable("actual AssignNodeNumber694 existing-fluid severe diagnostic"));
                }
                if row.fluid_type == SourceNodeFluidType::Blank { row.fluid_type = fluid_type; }
                return Ok(number);
            }
            self.number_of_unique_node_names = self.number_of_unique_node_names.checked_add(1)
                .filter(|&count| i32::try_from(count).is_ok()).ok_or(Unavailable::InconsistentOwner)?;
            self.number_of_nodes = self.number_of_unique_node_names;
            // Native redimension order711-715, preserving every prior row.
            self.physical.as_mut().ok_or(Unavailable::InconsistentOwner)?
                .resize(self.number_of_nodes, SourceNodeData::default());
            self.node_ids.as_mut().ok_or(Unavailable::InconsistentOwner)?
                .resize(self.number_of_nodes + 1, String::new());
            self.node_references.as_mut().ok_or(Unavailable::InconsistentOwner)?
                .resize(self.number_of_nodes, 0);
            self.marked_nodes.as_mut().ok_or(Unavailable::InconsistentOwner)?
                .resize(self.number_of_nodes, SourceMarkedNode::default());
            self.node_setpoint_checks.as_mut().ok_or(Unavailable::InconsistentOwner)?
                .resize(self.number_of_nodes, SourceNodeSetpointCheck::default());
            self.physical.as_mut().and_then(|rows| rows.get_mut(self.number_of_nodes - 1))
                .ok_or(Unavailable::InconsistentOwner)?.fluid_type = fluid_type;
            *self.node_references.as_mut().and_then(|rows| rows.get_mut(self.number_of_nodes - 1))
                .ok_or(Unavailable::InconsistentOwner)? = 0;
            *self.node_ids.as_mut().and_then(|rows| rows.get_mut(self.number_of_unique_node_names))
                .ok_or(Unavailable::InconsistentOwner)? = name.into();
            return Ok(self.number_of_unique_node_names);
        }
        self.physical = Some(vec![SourceNodeData::default()]);
        self.physical.as_mut().and_then(|rows| rows.first_mut())
            .ok_or(Unavailable::InconsistentOwner)?.fluid_type = fluid_type;
        self.number_of_nodes = 1;
        self.node_ids = Some(vec![String::new(); 2]);
        self.node_references = Some(vec![0]);
        self.marked_nodes = Some(vec![SourceMarkedNode::default()]);
        self.node_setpoint_checks = Some(vec![SourceNodeSetpointCheck::default()]);
        self.number_of_unique_node_names = 1;
        let names = self.node_ids.as_mut().ok_or(Unavailable::InconsistentOwner)?;
        names[0] = "Undefined".into();
        names[1] = name.into();
        *self.node_references.as_mut().and_then(|rows| rows.first_mut())
            .ok_or(Unavailable::InconsistentOwner)? = 0;
        Ok(1)
    }
}
