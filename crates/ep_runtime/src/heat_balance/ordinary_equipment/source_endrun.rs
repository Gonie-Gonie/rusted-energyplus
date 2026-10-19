//! Actual EMSManager1574-1612 deferred setpoint check on the sole retained nodes.
use super::OrdinaryEquipment;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_simulation_startup::SelectedRuntimeLanguage;

// Literal EMSManager114-122 positional table. Do not derive names from the
// CtrlVarType enum's Max/Min spellings: Native uses this exact array order.
const CONTROL_TYPE_NAMES: [&str; 9] = [
    "Temperature Setpoint",
    "Temperature Minimum Setpoint",
    "Temperature Maximum Setpoint",
    "Humidity Ratio Setpoint",
    "Humidity Ratio Minimum Setpoint",
    "Humidity Ratio Maximum Setpoint",
    "Mass Flow Rate Setpoint",
    "Mass Flow Rate Minimum Available Setpoint",
    "Mass Flow Rate Maximum Available Setpoint",
];

fn check_source_node_setpoint_managed_by_handle(
    node_ids: &Option<Vec<String>>,
    node_number: usize,
    control_type: usize,
    runtime_language: &SelectedRuntimeLanguage,
) -> Result<bool, Error> {
    let _found_control = false;
    let _node_name = node_ids
        .as_ref()
        .and_then(|rows| rows.get(node_number))
        .ok_or(Error::Unavailable(
            "actual CheckIfNodeSetPointManaged1495 NodeID",
        ))?
        .clone();
    let _component_type_name = "System Node Setpoint";
    let _control_type_name = *CONTROL_TYPE_NAMES
        .get(control_type)
        .ok_or(Error::Unavailable(
            "actual CheckIfNodeSetPointManaged1497 controlTypeNames ordinal",
        ))?;
    if runtime_language.number_ems_actuators_available > 0 {
        return Err(Error::Unavailable(
            "actual CheckIfNodeSetPointManaged1501 EMSActuatorAvailable.handleCount first record",
        ));
    }
    // Native byHandle=true emits this warning even when the real available
    // count is zero. Do not return its local false while dropping that effect.
    Err(Error::Unavailable(
        "actual CheckIfNodeSetPointManaged1510 missing-handle ShowWarningError",
    ))
}

impl OrdinaryEquipment {
    pub(crate) fn check_source_setpoint_nodes_at_end(
        &mut self,
        runtime_language: &SelectedRuntimeLanguage,
    ) -> Result<(), Error> {
        let mut fatal_error_flag = false;
        for node_number in 1..=self.nodes.number_of_nodes {
            let check = self
                .nodes
                .node_setpoint_checks
                .as_mut()
                .and_then(|rows| rows.get_mut(node_number - 1))
                .ok_or(Error::Unavailable(
                    "actual checkSetpointNodesAtEnd1588 NodeSetpointCheck",
                ))?;
            if check.needs_setpoint_checking {
                check.needs_setpoint_checking = false;
                for control_type in 0..CONTROL_TYPE_NAMES.len() {
                    if check.check_setpoint[control_type] {
                        // Compound OR evaluates the helper for every selected
                        // control, including after a preceding failed match.
                        check.needs_setpoint_checking |=
                            !check_source_node_setpoint_managed_by_handle(
                                &self.nodes.node_ids,
                                node_number,
                                control_type,
                                runtime_language,
                            )?;
                    }
                }
                if check.needs_setpoint_checking {
                    fatal_error_flag = true;
                }
            }
        }
        if fatal_error_flag {
            return Err(Error::Unavailable(
                "actual checkSetpointNodesAtEnd1608 ShowFatalError",
            ));
        }
        Ok(())
    }
}
