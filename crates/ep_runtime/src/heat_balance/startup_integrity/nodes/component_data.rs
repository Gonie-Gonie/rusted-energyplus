//! Native BranchNodeConnections1476-1506/1600-1625/1686-1781.
//! The six arrays and counts are the actual per-call output arguments.
use super::{
    Error, SourceBranchNodeConnections, SourceCompFluidStream, SourceNodeConnection,
    SourceNodeConnectionType, SourceNodeObjectType, extent,
};

impl SourceBranchNodeConnections {
    fn component_connection(&self, ordinal: i32) -> Result<&SourceNodeConnection, Error> {
        let index = ordinal
            .checked_sub(1)
            .and_then(|n| usize::try_from(n).ok())
            .ok_or(Error::Unavailable(
                "actual component connection one-based ordinal",
            ))?;
        self.node_connections
            .as_ref()
            .and_then(|rows| rows.get(index))
            .ok_or(Error::Unavailable(
                "actual component connection retained NodeConnections row",
            ))
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn get_source_component_data(
        &self,
        component_type: SourceNodeObjectType,
        component_name: &str,
        is_parent: &mut bool,
        number_inlets: &mut i32,
        inlet_names: &mut Option<Vec<String>>,
        inlet_numbers: &mut Option<Vec<i32>>,
        inlet_streams: &mut Option<Vec<SourceCompFluidStream>>,
        number_outlets: &mut i32,
        outlet_names: &mut Option<Vec<String>>,
        outlet_numbers: &mut Option<Vec<i32>>,
        outlet_streams: &mut Option<Vec<SourceCompFluidStream>>,
    ) -> Result<(), Error> {
        if inlet_names.is_some() {
            *inlet_names = None;
        }
        if inlet_numbers.is_some() {
            *inlet_numbers = None;
        }
        if inlet_streams.is_some() {
            *inlet_streams = None;
        }
        if outlet_names.is_some() {
            *outlet_names = None;
        }
        if outlet_numbers.is_some() {
            *outlet_numbers = None;
        }
        if outlet_streams.is_some() {
            *outlet_streams = None;
        }
        *number_inlets = 0;
        *number_outlets = 0;
        *is_parent = false;
        for ordinal in 1..=self.number_node_connections {
            let row = self.component_connection(ordinal)?;
            if row.object_type != component_type || row.object_name != component_name {
                continue;
            }
            if row.object_is_parent {
                *is_parent = true;
            }
            if row.connection_type == SourceNodeConnectionType::Inlet {
                *number_inlets = number_inlets.checked_add(1).ok_or(Error::Unavailable(
                    "undefined GetComponentData1742 inlet count",
                ))?;
            } else if row.connection_type == SourceNodeConnectionType::Outlet {
                *number_outlets = number_outlets.checked_add(1).ok_or(Error::Unavailable(
                    "undefined GetComponentData1744 outlet count",
                ))?;
            }
        }
        *inlet_names = Some(vec![String::new(); extent(*number_inlets)?]);
        *inlet_numbers = Some(vec![0; extent(*number_inlets)?]);
        *inlet_streams = Some(vec![
            SourceCompFluidStream::Invalid;
            extent(*number_inlets)?
        ]);
        *outlet_names = Some(vec![String::new(); extent(*number_outlets)?]);
        *outlet_numbers = Some(vec![0; extent(*number_outlets)?]);
        *outlet_streams = Some(vec![
            SourceCompFluidStream::Invalid;
            extent(*number_outlets)?
        ]);
        inlet_names
            .as_mut()
            .ok_or(Error::Unavailable("actual GetComponentData1748 allocation"))?
            .fill(String::new());
        inlet_numbers
            .as_mut()
            .ok_or(Error::Unavailable("actual GetComponentData1749 allocation"))?
            .fill(0);
        inlet_streams
            .as_mut()
            .ok_or(Error::Unavailable("actual GetComponentData1750 allocation"))?
            .fill(SourceCompFluidStream::Invalid);
        outlet_names
            .as_mut()
            .ok_or(Error::Unavailable("actual GetComponentData1751 allocation"))?
            .fill(String::new());
        outlet_numbers
            .as_mut()
            .ok_or(Error::Unavailable("actual GetComponentData1752 allocation"))?
            .fill(0);
        outlet_streams
            .as_mut()
            .ok_or(Error::Unavailable("actual GetComponentData1753 allocation"))?
            .fill(SourceCompFluidStream::Invalid);
        *number_inlets = 0;
        *number_outlets = 0;
        for ordinal in 1..=self.number_node_connections {
            let row = self.component_connection(ordinal)?;
            if row.object_type != component_type || row.object_name != component_name {
                continue;
            }
            if row.connection_type == SourceNodeConnectionType::Inlet {
                *number_inlets = number_inlets.checked_add(1).ok_or(Error::Unavailable(
                    "undefined GetComponentData1770 inlet count",
                ))?;
                let index = extent(*number_inlets - 1)?;
                *inlet_names.as_mut().and_then(|v| v.get_mut(index)).ok_or(
                    Error::Unavailable("actual GetComponentData1771 InletNodeNames"),
                )? = row.node_name.clone();
                *inlet_numbers
                    .as_mut()
                    .and_then(|v| v.get_mut(index))
                    .ok_or(Error::Unavailable(
                        "actual GetComponentData1772 InletNodeNums",
                    ))? = i32::try_from(row.node_number)
                    .map_err(|_| Error::Unavailable("actual GetComponentData signed NodeNumber"))?;
                *inlet_streams
                    .as_mut()
                    .and_then(|v| v.get_mut(index))
                    .ok_or(Error::Unavailable(
                        "actual GetComponentData1773 InletFluidStreams",
                    ))? = row.fluid_stream;
            } else if row.connection_type == SourceNodeConnectionType::Outlet {
                *number_outlets = number_outlets.checked_add(1).ok_or(Error::Unavailable(
                    "undefined GetComponentData1775 outlet count",
                ))?;
                let index = extent(*number_outlets - 1)?;
                *outlet_names.as_mut().and_then(|v| v.get_mut(index)).ok_or(
                    Error::Unavailable("actual GetComponentData1776 OutletNodeNames"),
                )? = row.node_name.clone();
                *outlet_numbers
                    .as_mut()
                    .and_then(|v| v.get_mut(index))
                    .ok_or(Error::Unavailable(
                        "actual GetComponentData1777 OutletNodeNums",
                    ))? = i32::try_from(row.node_number)
                    .map_err(|_| Error::Unavailable("actual GetComponentData signed NodeNumber"))?;
                *outlet_streams
                    .as_mut()
                    .and_then(|v| v.get_mut(index))
                    .ok_or(Error::Unavailable(
                        "actual GetComponentData1778 OutletFluidStreams",
                    ))? = row.fluid_stream;
            }
        }
        Ok(())
    }

    pub(crate) fn is_source_parent_object(
        &self,
        component_type: SourceNodeObjectType,
        component_name: &str,
    ) -> Result<bool, Error> {
        let mut is_parent = false;
        for ordinal in 1..=self.number_node_connections {
            let row = self.component_connection(ordinal)?;
            if row.object_type == component_type && row.object_name == component_name {
                if row.object_is_parent {
                    is_parent = true;
                }
                break;
            }
        }
        if !is_parent {
            // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
            #[allow(clippy::never_loop)]
            for _ordinal in 1..=self.number_component_sets {
                return Err(Error::Unavailable(
                    "actual IsParentObjectCompSet1617 CompSets.ParentObjectType",
                ));
            }
        }
        Ok(is_parent)
    }
}
