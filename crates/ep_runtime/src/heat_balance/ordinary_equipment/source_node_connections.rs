//! Native NodeInputManager GetNodeNums/GetOnlySingleNode/CheckUnique children.
use super::{NodeOwner, Unavailable, source_nodes::SourceNodeFluidType};
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::startup_integrity::{
    SourceBranchNodeConnections, SourceCompFluidStream, SourceNodeConnectionType,
    SourceNodeObjectType,
};

fn unavailable(text: &'static str) -> Unavailable {
    Unavailable::Input(text.into())
}

impl NodeOwner {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn get_source_node_nums(
        &mut self,
        input: &mut SelectedInputProcessor,
        branch: &mut SourceBranchNodeConnections,
        name: &str,
        num_nodes: &mut usize,
        numbers: &mut [usize],
        errors_found: &mut bool,
        fluid_type: SourceNodeFluidType,
        object_type: SourceNodeObjectType,
        object_name: &str,
        connection_type: SourceNodeConnectionType,
        fluid_stream: SourceCompFluidStream,
        is_parent: bool,
        increment_stream: bool,
        input_field_name: &str,
    ) -> Result<(), Unavailable> {
        object_type
            .source_name()
            .map_err(|e| Unavailable::Input(e.to_string()))?;
        if self.get_input {
            self.get_source_node_lists_input(input, errors_found)?;
            self.get_input = false;
        }
        if matches!(
            fluid_type,
            SourceNodeFluidType::Invalid | SourceNodeFluidType::Num
        ) {
            return Err(unavailable(
                "actual GetNodeNums120 invalid-fluid severe diagnostic",
            ));
        }
        if !name.is_empty() {
            // Native Util::FindItemInList(Name, NodeLists) is case-sensitive.
            let found = self
                .node_lists
                .as_ref()
                .ok_or(Unavailable::InconsistentOwner)?
                .iter()
                .position(|row| row.name == name);
            if let Some(index) = found {
                let list = self
                    .node_lists
                    .as_ref()
                    .and_then(|rows| rows.get(index))
                    .ok_or(Unavailable::InconsistentOwner)?;
                *num_nodes = list.number_of_nodes;
                let assigned = list
                    .node_numbers
                    .as_ref()
                    .ok_or(Unavailable::InconsistentOwner)?;
                let destination = numbers
                    .get_mut(..*num_nodes)
                    .ok_or(Unavailable::InconsistentOwner)?;
                destination.copy_from_slice(
                    assigned
                        .get(..*num_nodes)
                        .ok_or(Unavailable::InconsistentOwner)?,
                );
                for number in numbers.iter().take(*num_nodes).copied() {
                    let node = self
                        .physical
                        .as_mut()
                        .and_then(|rows| number.checked_sub(1).and_then(|i| rows.get_mut(i)))
                        .ok_or(Unavailable::InconsistentOwner)?;
                    if fluid_type != SourceNodeFluidType::Blank
                        && node.fluid_type != SourceNodeFluidType::Blank
                        && node.fluid_type != fluid_type
                    {
                        return Err(unavailable(
                            "actual GetNodeNums134 existing-fluid severe diagnostic",
                        ));
                    }
                    if node.fluid_type == SourceNodeFluidType::Blank {
                        node.fluid_type = fluid_type;
                    }
                    let reference = self
                        .node_references
                        .as_mut()
                        .and_then(|rows| number.checked_sub(1).and_then(|i| rows.get_mut(i)))
                        .ok_or(Unavailable::InconsistentOwner)?;
                    *reference = reference
                        .checked_add(1)
                        .ok_or(Unavailable::InconsistentOwner)?;
                }
            } else {
                let number = self.assign_source_node_number(name, fluid_type, errors_found)?;
                *num_nodes = 1;
                *numbers.first_mut().ok_or(Unavailable::InconsistentOwner)? = number;
            }
        } else {
            *num_nodes = 0;
            *numbers.first_mut().ok_or(Unavailable::InconsistentOwner)? = 0;
        }
        let mut stream = fluid_stream;
        for ordinal in 0..*num_nodes {
            if increment_stream {
                stream = fluid_stream
                    .source_increment(ordinal)
                    .map_err(|e| Unavailable::Input(e.to_string()))?;
            }
            let number = *numbers.get(ordinal).ok_or(Unavailable::InconsistentOwner)?;
            let node_name = self
                .node_ids
                .as_ref()
                .and_then(|rows| rows.get(number))
                .ok_or(Unavailable::InconsistentOwner)?;
            branch
                .register_source_node_connection(
                    number,
                    node_name,
                    object_type,
                    object_name,
                    connection_type,
                    stream,
                    is_parent,
                    errors_found,
                    input_field_name,
                )
                .map_err(|e| Unavailable::Input(e.to_string()))?;
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn get_source_only_single_node(
        &mut self,
        input: &mut SelectedInputProcessor,
        branch: &mut SourceBranchNodeConnections,
        name: &str,
        errors_found: &mut bool,
        object_type: SourceNodeObjectType,
        object_name: &str,
        fluid_type: SourceNodeFluidType,
        connection_type: SourceNodeConnectionType,
        fluid_stream: SourceCompFluidStream,
        is_parent: bool,
        input_field_name: &str,
    ) -> Result<usize, Unavailable> {
        object_type
            .source_name()
            .map_err(|e| Unavailable::Input(e.to_string()))?;
        if self.only_single_node_first_time {
            let counts = input
                .get_object_def_max_args("NodeList")
                .map_err(|_| unavailable("actual GetOnlySingleNode769 NodeList definition"))?;
            self.only_single_node_numbers = Some(vec![
                0;
                counts.total().map_err(|_| {
                    Unavailable::InconsistentOwner
                })?
            ]);
            self.only_single_node_first_time = false;
        }
        // This is the same retained Native scratch array, moved temporarily
        // to express disjoint Rust borrows and restored even on a typed stop.
        let mut numbers = self
            .only_single_node_numbers
            .take()
            .ok_or(Unavailable::InconsistentOwner)?;
        let mut num_nodes = 0;
        let result = self.get_source_node_nums(
            input,
            branch,
            name,
            &mut num_nodes,
            &mut numbers,
            errors_found,
            fluid_type,
            object_type,
            object_name,
            connection_type,
            fluid_stream,
            is_parent,
            false,
            input_field_name,
        );
        self.only_single_node_numbers = Some(numbers);
        result?;
        if num_nodes > 1 {
            return Err(unavailable(
                "actual GetOnlySingleNode791 multi-node severe diagnostic",
            ));
        } else if num_nodes == 0 {
            *self
                .only_single_node_numbers
                .as_mut()
                .and_then(|rows| rows.first_mut())
                .ok_or(Unavailable::InconsistentOwner)? = 0;
        }
        self.only_single_node_numbers
            .as_ref()
            .and_then(|rows| rows.first())
            .copied()
            .ok_or(Unavailable::InconsistentOwner)
    }

    pub(super) fn check_source_unique_node_names(
        &mut self,
        _node_types: &str,
        _errors_found: &mut bool,
        check_name: &str,
        _object_name: &str,
    ) -> Result<(), Unavailable> {
        if check_name.is_empty() {
            return Ok(());
        }
        self.check_source_unique_node_value(
            check_name,
            "actual CheckUniqueNodeNames869 duplicate-node severe diagnostic",
        )
    }

    pub(super) fn check_source_unique_node_numbers(
        &mut self,
        _node_types: &str,
        _errors_found: &mut bool,
        check_number: usize,
        _object_name: &str,
    ) -> Result<(), Unavailable> {
        if check_number == 0 {
            return Ok(());
        }
        let name = self
            .node_ids
            .as_ref()
            .and_then(|rows| rows.get(check_number))
            .ok_or(Unavailable::InconsistentOwner)?
            .clone();
        self.check_source_unique_node_value(
            &name,
            "actual CheckUniqueNodeNumbers911 duplicate-node severe diagnostic",
        )
    }

    fn check_source_unique_node_value(
        &mut self,
        check_name: &str,
        duplicate_diagnostic: &'static str,
    ) -> Result<(), Unavailable> {
        if self.number_of_check_nodes > 0 {
            let rows = self
                .unique_names
                .as_ref()
                .ok_or(Unavailable::InconsistentOwner)?;
            if rows
                .get(..self.number_of_check_nodes)
                .ok_or(Unavailable::InconsistentOwner)?
                .iter()
                .any(|name| name == check_name)
            {
                return Err(unavailable(duplicate_diagnostic));
            }
        }
        self.number_of_check_nodes = self
            .number_of_check_nodes
            .checked_add(1)
            .filter(|&n| i32::try_from(n).is_ok())
            .ok_or(Unavailable::InconsistentOwner)?;
        if self.number_of_check_nodes > self.max_check_nodes {
            self.max_check_nodes = self
                .max_check_nodes
                .checked_add(100)
                .filter(|&n| i32::try_from(n).is_ok())
                .ok_or(Unavailable::InconsistentOwner)?;
            self.unique_names
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?
                .resize(self.max_check_nodes, String::new());
        }
        *self
            .unique_names
            .as_mut()
            .and_then(|rows| rows.get_mut(self.number_of_check_nodes - 1))
            .ok_or(Unavailable::InconsistentOwner)? = check_name.into();
        Ok(())
    }
}
