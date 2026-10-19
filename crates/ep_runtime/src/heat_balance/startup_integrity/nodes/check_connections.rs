//! Native BranchNodeConnections865-1474 on the sole retained connection registry.
//! Diagnostic bodies stop at their actual first request; successful traversals
//! preserve the caller latch and perform the real accumulated-count suffix.
use super::{Error, SourceBranchNodeConnections, SourceNodeConnection, SourceNodeObjectType, extent};
use super::SourceNodeConnectionType as Connection;

fn local_index(ordinal: i32) -> Result<usize, Error> {
    ordinal.checked_sub(1).and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual CheckNodeConnections one-based local index"))
}

fn local<T>(values: &[T], ordinal: i32) -> Result<&T, Error> {
    values.get(local_index(ordinal)?)
        .ok_or(Error::Unavailable("actual CheckNodeConnections local array element"))
}

fn local_mut<T>(values: &mut [T], ordinal: i32) -> Result<&mut T, Error> {
    values.get_mut(local_index(ordinal)?)
        .ok_or(Error::Unavailable("actual CheckNodeConnections local array element"))
}

fn is_loop_object(kind: SourceNodeObjectType) -> bool {
    // Exact DataLoopNode ConnectionObjectType ordinals. The current admitted
    // registration enum has no AirLoop/Condenser/Plant variants; this does not
    // expand that producer domain or substitute a wildcard object predicate.
    let kind = kind as i32;
    kind == 2 || kind == 109 || kind == 193
}

impl SourceBranchNodeConnections {
    fn checked_connection(&self, ordinal: i32) -> Result<&SourceNodeConnection, Error> {
        self.node_connections.as_ref().and_then(|rows| rows.get(local_index(ordinal).ok()?))
            .ok_or(Error::Unavailable("actual CheckNodeConnections retained NodeConnections row"))
    }

    pub(crate) fn check_source_node_connections(&mut self, _errors_found: &mut bool) -> Result<(), Error> {
        let error_counter = 0;
        // Check1a: Sensor excludes only Actuator/Sensor, not OutsideAir.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.connection_type != Connection::Sensor { continue; }
            let mut is_valid = false;
            for second in 1..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(first)?.node_number != self.checked_connection(second)?.node_number { continue; }
                if matches!(self.checked_connection(second)?.connection_type, Connection::Actuator | Connection::Sensor) { continue; }
                is_valid = true;
            }
            if !is_valid { return Err(Error::Unavailable("actual CheckNodeConnections939 Sensor severe diagnostic")); }
        }
        // Check1b: the actual Actuator branch additionally excludes OutsideAir.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.connection_type != Connection::Actuator { continue; }
            let mut is_valid = false;
            for second in 1..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(first)?.node_number != self.checked_connection(second)?.node_number { continue; }
                if matches!(self.checked_connection(second)?.connection_type,
                    Connection::Actuator | Connection::Sensor | Connection::OutsideAir) { continue; }
                is_valid = true;
            }
            if !is_valid { return Err(Error::Unavailable("actual CheckNodeConnections978 Actuator severe diagnostic")); }
        }
        // Check2: preserve both independently ordered validity predicates.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.connection_type != Connection::SetPoint { continue; }
            let mut is_valid = false;
            let mut is_inlet = false;
            let mut is_outlet = false;
            for second in 1..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(first)?.node_number != self.checked_connection(second)?.node_number { continue; }
                if matches!(self.checked_connection(second)?.connection_type, Connection::SetPoint | Connection::OutsideAir) { continue; }
                if self.checked_connection(second)?.connection_type == Connection::Inlet { is_inlet = true; }
                else if self.checked_connection(second)?.connection_type == Connection::Outlet { is_outlet = true; }
                is_valid = true;
            }
            if !is_valid { return Err(Error::Unavailable("actual CheckNodeConnections1024 Setpoint matching-type severe diagnostic")); }
            if !is_inlet && !is_outlet { return Err(Error::Unavailable("actual CheckNodeConnections1039 Setpoint inlet/outlet severe diagnostic")); }
        }
        // Check3: every actual ZoneInlet must have another Outlet row.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.connection_type != Connection::ZoneInlet { continue; }
            let mut is_valid = false;
            for second in 1..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(first)?.node_number != self.checked_connection(second)?.node_number { continue; }
                if self.checked_connection(second)?.connection_type != Connection::Outlet { continue; }
                is_valid = true;
            }
            if !is_valid { return Err(Error::Unavailable("actual CheckNodeConnections1076 ZoneInlet severe diagnostic")); }
        }
        // Check4: every actual ZoneExhaust must have another Inlet row.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.connection_type != Connection::ZoneExhaust { continue; }
            let mut is_valid = false;
            for second in 1..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(first)?.node_number != self.checked_connection(second)?.node_number { continue; }
                if self.checked_connection(second)?.connection_type != Connection::Inlet { continue; }
                is_valid = true;
            }
            if !is_valid { return Err(Error::Unavailable("actual CheckNodeConnections1109 ZoneExhaust severe diagnostic")); }
        }
        // Check5: return-plenum InducedAir must have another Inlet row.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.connection_type != Connection::InducedAir { continue; }
            let mut is_valid = false;
            for second in 1..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(first)?.node_number != self.checked_connection(second)?.node_number { continue; }
                if self.checked_connection(second)?.connection_type != Connection::Inlet { continue; }
                is_valid = true;
            }
            if !is_valid { return Err(Error::Unavailable("actual CheckNodeConnections1142 InducedAir severe diagnostic")); }
        }
        // Check6: IsValid is never set true in the Native body; the actual
        // MatchedAtLeastOne latch admits appropriate same-node connections.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.connection_type != Connection::Inlet { continue; }
            if is_loop_object(self.checked_connection(first)?.object_type) { continue; }
            let is_valid = false;
            let mut matched_at_least_one = false;
            for second in 1..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(first)?.node_number != self.checked_connection(second)?.node_number { continue; }
                if matches!(self.checked_connection(second)?.connection_type,
                    Connection::Outlet | Connection::ZoneReturn | Connection::ZoneExhaust |
                    Connection::InducedAir | Connection::ReliefAir | Connection::OutsideAir) {
                    matched_at_least_one = true;
                    continue;
                }
                if self.checked_connection(second)?.connection_type == Connection::Inlet
                    && is_loop_object(self.checked_connection(second)?.object_type) {
                    matched_at_least_one = true;
                    continue;
                }
            }
            if !is_valid && !matched_at_least_one { return Err(Error::Unavailable("actual CheckNodeConnections1201 Inlet severe diagnostic")); }
        }
        // Check7: only later non-parent Inlet rows are duplicate candidates.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.object_is_parent { continue; }
            if self.checked_connection(first)?.connection_type != Connection::Inlet { continue; }
            for second in first..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(second)?.object_is_parent { continue; }
                if self.checked_connection(second)?.connection_type != Connection::Inlet { continue; }
                if self.checked_connection(second)?.node_number == self.checked_connection(first)?.node_number {
                    return Err(Error::Unavailable("actual CheckNodeConnections1238 duplicate Inlet severe diagnostic"));
                }
            }
        }
        // Check8: the corresponding ordered non-parent Outlet check.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.object_is_parent { continue; }
            if self.checked_connection(first)?.connection_type != Connection::Outlet { continue; }
            let _is_valid = true;
            for second in first..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(second)?.object_is_parent { continue; }
                if self.checked_connection(second)?.connection_type != Connection::Outlet { continue; }
                if self.checked_connection(second)?.node_number == self.checked_connection(first)?.node_number {
                    return Err(Error::Unavailable("actual CheckNodeConnections1283 duplicate Outlet severe diagnostic"));
                }
            }
        }
        // Check9: first matching actual OutsideAir row ends the inner scan.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.connection_type != Connection::OutsideAirReference { continue; }
            let mut is_valid = false;
            for second in 1..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(first)?.node_number != self.checked_connection(second)?.node_number { continue; }
                if self.checked_connection(second)?.connection_type != Connection::OutsideAir { continue; }
                is_valid = true;
                break;
            }
            if !is_valid { return Err(Error::Unavailable("actual CheckNodeConnections1327 OutsideAirReference severe diagnostic")); }
        }
        // Check10: maxval visits the WHOLE allocated EPVector, including its
        // genuine default unused rows, whereas grouping uses the active count.
        if self.number_node_connections > 0 {
            let max_fluid_stream = self.node_connections.as_ref()
                .ok_or(Error::Unavailable("actual CheckNodeConnections1347 NodeConnections maxval allocation"))?
                .iter().map(|row| row.fluid_stream as i32).max()
                .ok_or(Error::Unavailable("actual CheckNodeConnections1347 empty positive NodeConnections maxval"))?;
            let mut inlet_count = vec![0_i32; extent(max_fluid_stream)?];
            let mut outlet_count = vec![0_i32; extent(max_fluid_stream)?];
            let mut stream_counts = vec![false; extent(max_fluid_stream)?];
            let count_plus_one = self.number_node_connections.checked_add(1)
                .ok_or(Error::Unavailable("undefined CheckNodeConnections1351 count-plus-one"))?;
            let mut node_objects = vec![0_i32; extent(count_plus_one)?];
            inlet_count.fill(0);
            outlet_count.fill(0);
            node_objects.fill(0);
            stream_counts.fill(false);
            let mut object = 1;
            let mut end_connect = 0;
            let mut number_objects = 2;
            *local_mut(&mut node_objects, 1)? = 1;
            while object < self.number_node_connections {
                if self.checked_connection(object)?.object_type != self.checked_connection(object + 1)?.object_type
                    || self.checked_connection(object)?.object_name != self.checked_connection(object + 1)?.object_name {
                    end_connect = object + 1;
                    *local_mut(&mut node_objects, number_objects)? = end_connect;
                    number_objects += 1;
                }
                object += 1;
            }
            *local_mut(&mut node_objects, number_objects)? = count_plus_one;
            for object in 1..=number_objects - 1 {
                let mut is_valid = true;
                inlet_count.fill(0);
                outlet_count.fill(0);
                stream_counts.fill(false);
                let first = *local(&node_objects, object)?;
                if self.number_node_connections < 2 { continue; }
                if self.checked_connection(first)?.object_is_parent { continue; }
                let first_row = self.checked_connection(first)?;
                if first_row.connection_type == Connection::Inlet {
                    *local_mut(&mut inlet_count, first_row.fluid_stream as i32)? += 1;
                } else if first_row.connection_type == Connection::Outlet {
                    *local_mut(&mut outlet_count, first_row.fluid_stream as i32)? += 1;
                }
                for second in first + 1..=*local(&node_objects, object + 1)? - 1 {
                    if self.checked_connection(second)?.object_is_parent { continue; }
                    let row = self.checked_connection(second)?;
                    if row.connection_type == Connection::Inlet {
                        *local_mut(&mut inlet_count, row.fluid_stream as i32)? += 1;
                    } else if row.connection_type == Connection::Outlet {
                        *local_mut(&mut outlet_count, row.fluid_stream as i32)? += 1;
                    }
                }
                for stream in 1..=max_fluid_stream {
                    if *local(&inlet_count, stream)? > 1 && *local(&outlet_count, stream)? > 1 {
                        is_valid = false;
                        *local_mut(&mut stream_counts, stream)? = true;
                    }
                }
                if !is_valid { return Err(Error::Unavailable("actual CheckNodeConnections1410 multiple-stream severe diagnostic")); }
            }
            drop(inlet_count);
            drop(outlet_count);
            drop(stream_counts);
            drop(node_objects);
        }
        // Check11 deliberately compares names and only later records.
        for first in 1..=self.number_node_connections {
            if self.checked_connection(first)?.connection_type != Connection::ZoneNode { continue; }
            let _is_valid = true;
            for second in first..=self.number_node_connections {
                if first == second { continue; }
                if self.checked_connection(first)?.node_name == self.checked_connection(second)?.node_name {
                    if matches!(self.checked_connection(second)?.connection_type,
                        Connection::Actuator | Connection::Sensor | Connection::SetPoint) { continue; }
                    return Err(Error::Unavailable("actual CheckNodeConnections1451 duplicate ZoneNode severe diagnostic"));
                }
            }
        }
        // No error branch reaches this suffix until its genuine diagnostics are
        // implemented; all successful checks derived the actual local zero.
        self.number_node_connection_errors += error_counter;
        Ok(())
    }
}
