//! SetPointManager1390-1416 calls the sole NodeInputManager owner.
//! These narrow accesses never allocate a parallel node or alter flow/temperature.
use super::{OrdinaryEquipment, Unavailable, source_nodes::SourceNodeFluidType};
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::startup_integrity::{
    SourceBranchNodeConnections, SourceCompFluidStream, SourceNodeConnectionType,
    SourceNodeObjectType,
};

impl OrdinaryEquipment {
    /// Caller retains Native GetSetPointManagerInputs' NodeNums scratch and
    /// local NodeListError. Name canonicalization and append-on-no-error stay
    /// with the actual manager getter; preserve NodeList member order here.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_source_scheduled_setpoint_node_nums(
        &mut self,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        diagnostic_flags: Option<SourceDiagnosticFlags<'_>>,
        branch: &mut SourceBranchNodeConnections,
        node_or_list_name: &str,
        object_name: &str,
        num_nodes: &mut usize,
        numbers: &mut [usize],
        errors_found: &mut bool,
    ) -> Result<(), Unavailable> {
        self.nodes.get_source_node_nums(
            input,
            backend,
            diagnostic_flags,
            branch,
            node_or_list_name,
            num_nodes,
            numbers,
            errors_found,
            SourceNodeFluidType::Blank,
            SourceNodeObjectType::SetpointManagerScheduled,
            object_name,
            SourceNodeConnectionType::SetPoint,
            SourceCompFluidStream::Primary,
            false,
            false,
            "setpoint_node_or_nodelist_name",
        )
    }

    /// Native Scheduled init2266-2306 and Update4183-4219 write precisely
    /// the selected existing Node cell. No clamp, unit inference or substitute.
    pub(crate) fn set_source_scheduled_setpoint(
        &mut self,
        number: usize,
        control_variable: i32,
        value: f64,
    ) -> Result<(), Unavailable> {
        let row = self
            .nodes
            .physical
            .as_mut()
            .and_then(|rows| number.checked_sub(1).and_then(|at| rows.get_mut(at)))
            .ok_or(Unavailable::InconsistentOwner)?;

        let destination = match control_variable {
            0 => &mut row.temperature_setpoint,
            1 => &mut row.temperature_setpoint_high,
            2 => &mut row.temperature_setpoint_low,
            3 => &mut row.humidity_ratio_setpoint,
            4 => &mut row.humidity_ratio_max,
            5 => &mut row.humidity_ratio_min,
            6 => &mut row.mass_flow_rate_setpoint,
            7 => &mut row.mass_flow_rate_max,
            8 => &mut row.mass_flow_rate_min,
            _ => return Err(Unavailable::InconsistentOwner),
        };
        *destination = value;
        Ok(())
    }
}
