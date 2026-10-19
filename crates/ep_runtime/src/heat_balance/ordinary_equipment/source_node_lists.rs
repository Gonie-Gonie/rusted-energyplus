//! GetNodeListsInput516-648 over the existing sole IP and NodeOwner.
use super::{NodeOwner, Unavailable, source_nodes::SourceNodeFluidType};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputStatisticsValue as Value,
};

#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct SourceNodeList {
    pub(super) name: String,
    pub(super) number_of_nodes: usize,
    pub(super) node_names: Option<Vec<String>>,
    pub(super) node_numbers: Option<Vec<usize>>,
}
fn unavailable(text: &'static str) -> Unavailable {
    Unavailable::Input(text.into())
}
impl NodeOwner {
    pub(super) fn get_source_node_lists_input(
        &mut self,
        input: &mut SelectedInputProcessor,
        _errors_found: &mut bool,
    ) -> Result<(), Unavailable> {
        let mut local_errors_found = false;
        let counts = input
            .get_object_def_max_args("NodeList")
            .map_err(|_| unavailable("actual GetNodeListsInput543 NodeList definition"))?;
        let mut alphas = vec![String::new(); counts.alphas];
        let mut numbers = vec![0.0_f64; counts.numerics];
        self.number_of_node_lists = input.get_num_objects_found("NodeList");
        self.node_lists = Some(vec![SourceNodeList::default(); self.number_of_node_lists]);
        for row in self
            .node_lists
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?
        {
            row.name.clear();
            row.number_of_nodes = 0;
        }
        // NodeList is not an IDF declaration-overlay target. A retained JSON
        // map order cannot establish its original multi-object IDF ordinal.
        if self.number_of_node_lists > 1 {
            return Err(unavailable(
                "actual multi-NodeList getter ordinal needs retained Native input order",
            ));
        }
        for index in 0..self.number_of_node_lists {
            // A single actual list has the same sole ordinal in either input
            // format. The zero-count SourceA path never needs this raw owner.
            let order = input
                .source_raw_object_names("NodeList")
                .map_err(|_| unavailable("actual NodeList getter retained object order"))?;
            if order.len() != self.number_of_node_lists {
                return Err(Unavailable::InconsistentOwner);
            }
            let object = order
                .get(index)
                .cloned()
                .ok_or(Unavailable::InconsistentOwner)?;
            alphas.fill(String::new());
            numbers.fill(0.0);
            input.mark_object_as_used("NodeList", &object);
            let fields = input
                .source_raw_object_fields("NodeList", &object)
                .map_err(|_| unavailable("actual NodeList getter retained raw fields"))?;
            let members = match fields.get("nodes") {
                Some(Value::Array(rows)) => rows.as_slice(),
                None | Some(Value::Null) => &[],
                _ => return Err(unavailable("actual NodeList extensible getter shape")),
            };
            let num_alphas = members
                .len()
                .checked_add(1)
                .ok_or(Unavailable::InconsistentOwner)?;
            if num_alphas > alphas.len() {
                return Err(Unavailable::InconsistentOwner);
            }
            alphas[0] = object.to_ascii_uppercase();
            for (member, value) in members.iter().enumerate() {
                let Value::Object(fields) = value else {
                    return Err(unavailable("actual NodeList member getter shape"));
                };
                alphas[member + 1] = match fields.get("node_name") {
                    Some(Value::String(value)) => value.to_ascii_uppercase(),
                    None | Some(Value::Null) => String::new(),
                    _ => return Err(unavailable("actual NodeList member alpha getter shape")),
                };
            }
            let row = self
                .node_lists
                .as_mut()
                .and_then(|rows| rows.get_mut(index))
                .ok_or(Unavailable::InconsistentOwner)?;
            row.name = alphas[0].clone();
            row.node_names = Some(vec![String::new(); num_alphas - 1]);
            row.node_numbers = Some(vec![0; num_alphas - 1]);
            row.number_of_nodes = num_alphas - 1;
            if num_alphas <= 1 {
                return Err(unavailable(
                    "actual GetNodeListsInput569 no-nodes severe diagnostic",
                ));
            }
            for member in 0..num_alphas - 1 {
                *self
                    .node_lists
                    .as_mut()
                    .and_then(|rows| rows.get_mut(index))
                    .and_then(|row| row.node_names.as_mut())
                    .and_then(|rows| rows.get_mut(member))
                    .ok_or(Unavailable::InconsistentOwner)? = alphas[member + 1].clone();
                if alphas[member + 1].is_empty() {
                    return Err(unavailable(
                        "actual GetNodeListsInput583 blank-node warning diagnostic",
                    ));
                }
                let number = self.assign_source_node_number(
                    &alphas[member + 1],
                    SourceNodeFluidType::Blank,
                    &mut local_errors_found,
                )?;
                let row = self
                    .node_lists
                    .as_mut()
                    .and_then(|rows| rows.get_mut(index))
                    .ok_or(Unavailable::InconsistentOwner)?;
                *row.node_numbers
                    .as_mut()
                    .and_then(|rows| rows.get_mut(member))
                    .ok_or(Unavailable::InconsistentOwner)? = number;
                if row
                    .node_names
                    .as_ref()
                    .and_then(|rows| rows.get(member))
                    .ok_or(Unavailable::InconsistentOwner)?
                    .eq_ignore_ascii_case(&row.name)
                {
                    return Err(unavailable(
                        "actual GetNodeListsInput596 node/list-name severe diagnostic",
                    ));
                }
            }
            let row = self
                .node_lists
                .as_ref()
                .and_then(|rows| rows.get(index))
                .ok_or(Unavailable::InconsistentOwner)?;
            let assigned = row
                .node_numbers
                .as_ref()
                .ok_or(Unavailable::InconsistentOwner)?;
            for first in 0..row.number_of_nodes {
                for other in first + 1..row.number_of_nodes {
                    if assigned[first] != assigned[other] {
                        continue;
                    }
                    return Err(unavailable(
                        "actual GetNodeListsInput611 duplicate-node severe diagnostic",
                    ));
                }
            }
        }
        let lists = self
            .node_lists
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?;
        for (index, row) in lists.iter().enumerate() {
            for member in 0..row.number_of_nodes {
                for (other, target) in lists.iter().enumerate() {
                    if index == other {
                        continue;
                    }
                    if !row
                        .node_names
                        .as_ref()
                        .and_then(|rows| rows.get(member))
                        .ok_or(Unavailable::InconsistentOwner)?
                        .eq_ignore_ascii_case(&target.name)
                    {
                        continue;
                    }
                    return Err(unavailable(
                        "actual GetNodeListsInput631 cross-list node-name severe diagnostic",
                    ));
                }
            }
        }
        drop(alphas);
        drop(numbers);
        if local_errors_found {
            return Err(unavailable("actual GetNodeListsInput645 fatal diagnostic"));
        }
        Ok(())
    }
}
