//! Actual geometry GetZoneEquipment subset; no HVAC or dispatcher activation.
use super::inside_convection_dispatcher::ZoneEquipmentActiveElements;
use crate::node::NodeStateStore;
use ep_model::{SpaceId, ZoneId};

/// Unsupported input or an unavailable represented equipment owner.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OrdinaryEquipmentUnavailable {
    /// A positive family or present-empty SpaceHVAC branch is unsupported.
    Input(String),
    /// The actual zone/space/timestep association is outside this route.
    GeometryOrTimestep,
    /// Existing retained storage is inconsistent with admitted empty equipment.
    InconsistentOwner,
    /// The literal uniqueness/getter fatal boundary was reached.
    Fatal(String),
}
use OrdinaryEquipmentUnavailable as Unavailable;
impl std::fmt::Display for OrdinaryEquipmentUnavailable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for OrdinaryEquipmentUnavailable {}

/// Supplied family projection, built from every actual raw family/count.
/// Field and positive-family admission also runs in the real ep_run resolver.
/// This is not a complete InputProcessor or a no-Node readiness certificate.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OrdinaryEquipmentInput {
    supplied: [Option<usize>; 9],
    timesteps_per_hour: usize,
}
impl OrdinaryEquipmentInput {
    /// Five count-query families, NodeList, then three actual .find families.
    pub const FAMILIES: [&'static str; 9] = [
        "ZoneHVAC:EquipmentConnections",
        "SpaceHVAC:EquipmentConnections",
        "ZoneHVAC:EquipmentList",
        "AirLoopHVAC:SupplyPath",
        "AirLoopHVAC:ReturnPath",
        "NodeList",
        "SpaceHVAC:ZoneEquipmentSplitter",
        "SpaceHVAC:ZoneEquipmentMixer",
        "SpaceHVAC:ZoneReturnMixer",
    ];
    /// Preserve absent versus explicit-empty supplied maps, rejecting active
    /// queried families and the warning-bearing present-empty Space branches.
    pub fn from_family_inventory<'a>(
        inventory: impl IntoIterator<Item = (&'a str, usize)>,
        timesteps_per_hour: usize,
    ) -> Result<Self, OrdinaryEquipmentUnavailable> {
        if timesteps_per_hour == 0 || timesteps_per_hour > 60 || 60 % timesteps_per_hour != 0 {
            return Err(Unavailable::GeometryOrTimestep);
        }
        let mut supplied = [None; 9];
        for (family, count) in inventory {
            if let Some(index) = Self::FAMILIES.iter().position(|&name| name == family) {
                if supplied[index].is_some() || count != 0 || index >= 6 {
                    return Err(Unavailable::Input(family.into()));
                }
                supplied[index] = Some(count);
            }
        }
        Ok(Self {
            supplied,
            timesteps_per_hour,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FamilyShape {
    // Earlier unobserved definition lookups may already have inserted null.
    // No .find is permitted on this unresolved, zero-count projection.
    AbsentOrEarlierNull,
    Absent,
    Null,
    ObjectMap(usize),
}
#[derive(Clone, Debug, PartialEq, Eq)]
struct EquipmentInputOwner {
    families: [FamilyShape; 9],
}
impl EquipmentInputOwner {
    fn new(input: OrdinaryEquipmentInput) -> Self {
        Self {
            families: std::array::from_fn(|index| match input.supplied[index] {
                Some(count) => FamilyShape::ObjectMap(count),
                None if index < 6 => FamilyShape::AbsentOrEarlierNull,
                None => FamilyShape::Absent,
            }),
        }
    }
    fn count(&self, index: usize) -> usize {
        match self.families[index] {
            FamilyShape::ObjectMap(count) => count,
            FamilyShape::AbsentOrEarlierNull | FamilyShape::Absent | FamilyShape::Null => 0,
        }
    }
    fn definition_lookup(&mut self, index: usize) {
        // InputProcessor1430-1438, valid canonical schema family only.
        // Existing null/map stays intact. Scratch argument dimensions are not
        // represented by this retained-owner projection.
        match self.families[index] {
            FamilyShape::AbsentOrEarlierNull | FamilyShape::Absent => {
                self.families[index] = FamilyShape::Null
            }
            FamilyShape::Null | FamilyShape::ObjectMap(_) => (),
        }
    }
    fn find_size(&self, index: usize) -> Result<Option<usize>, Unavailable> {
        match self.families[index] {
            FamilyShape::AbsentOrEarlierNull => Err(Unavailable::InconsistentOwner),
            FamilyShape::Absent => Ok(None),
            FamilyShape::Null => Ok(Some(0)),
            FamilyShape::ObjectMap(count) => Ok(Some(count)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct EquipmentConfig {
    zone_name: String,
    equipment_list_name: String,
    equipment_list_index: usize,
    zone_node: usize,
    inlet_nodes: usize,
    exhaust_nodes: usize,
    return_nodes: usize,
    is_controlled: bool,
    in_floor_active_element: bool,
    in_wall_active_element: bool,
    in_ceiling_active_element: bool,
}

impl Default for EquipmentConfig {
    fn default() -> Self {
        Self {
            zone_name: "Uncontrolled Zone".into(),
            equipment_list_name: String::new(),
            equipment_list_index: 0,
            zone_node: 0,
            inlet_nodes: 0,
            exhaust_nodes: 0,
            return_nodes: 0,
            is_controlled: false,
            in_floor_active_element: false,
            in_wall_active_element: false,
            in_ceiling_active_element: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct EquipmentList {
    name: String,
    number_of_equipment_types: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct SpaceZoneGain {
    number_of_occupants: f64,
    visible_energy_from_lights: f64,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct SpaceGainDevices {
    number_of_devices: usize,
    max_number_of_devices: usize,
    /// No device has been registered at AllocateIntGains. A future real gain
    /// producer must extend this same storage, not create another gain owner.
    devices: Option<Vec<()>>,
}

#[derive(Clone, Debug, Default, PartialEq)]
struct InternalGainStorage {
    zone: Option<Vec<SpaceZoneGain>>,
    space: Option<Vec<SpaceZoneGain>>,
    devices: Option<Vec<SpaceGainDevices>>,
    daylight_power_reduction_factor: Option<Vec<f64>>,
}

impl InternalGainStorage {
    fn allocate_if_zone_unallocated(&mut self, zones: usize, spaces: usize) {
        if self.zone.is_none() {
            // DataHeatBalance1044-1047: four real retained allocations.
            self.zone = Some(vec![SpaceZoneGain::default(); zones]);
            self.space = Some(vec![SpaceZoneGain::default(); spaces]);
            self.devices = Some(vec![SpaceGainDevices::default(); spaces]);
            self.daylight_power_reduction_factor = Some(vec![1.0; spaces]);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct NodeOwner {
    /// Independent of counters and diagnostic typed-node projection length.
    physical: Option<NodeStateStore>,
    number_of_nodes: usize,
    number_of_unique_node_names: usize,
    number_of_node_lists: usize,
    get_input: bool,
    node_lists: Option<Vec<()>>,
    unique_names: Option<Vec<String>>,
    context_name: String,
    number_of_check_nodes: usize,
    max_check_nodes: usize,
}

impl Default for NodeOwner {
    fn default() -> Self {
        Self {
            physical: None,
            number_of_nodes: 0,
            number_of_unique_node_names: 0,
            number_of_node_lists: 0,
            get_input: true,
            node_lists: None,
            unique_names: None,
            context_name: String::new(),
            number_of_check_nodes: 0,
            max_check_nodes: 0,
        }
    }
}

impl NodeOwner {
    fn init_unique_check(
        &mut self,
        context: &str,
        input: &mut EquipmentInputOwner,
    ) -> Result<(), Unavailable> {
        if self.get_input {
            // GetNodeListsInput543-547 runs even at zero count; only its own
            // list storage is allocated, never the physical Node storage.
            input.definition_lookup(5);
            self.number_of_node_lists = input.count(5);
            self.node_lists = Some(Vec::new());
            self.get_input = false;
        }
        if !self.context_name.is_empty() {
            return Err(Unavailable::Fatal(format!(
                "Init Uniqueness called for \"{}, but checks for \"{}\" was already in progress.",
                context, self.context_name
            )));
        }
        if context.is_empty() {
            return Err(Unavailable::Fatal(
                "Init Uniqueness called with Blank Context Name".into(),
            ));
        }
        self.unique_names = None;
        self.number_of_check_nodes = 0;
        self.max_check_nodes = 100;
        self.unique_names = Some(vec![String::new(); self.max_check_nodes]);
        self.context_name = context.into();
        Ok(())
    }

    fn end_unique_check(&mut self, context: &str) -> Result<(), Unavailable> {
        if self.context_name != context {
            return Err(Unavailable::Fatal(format!(
                "End Uniqueness called for \"{}, but checks for \"{}\" was in progress.",
                context, self.context_name
            )));
        }
        if context.is_empty() {
            return Err(Unavailable::Fatal(
                "End Uniqueness called with Blank Context Name".into(),
            ));
        }
        self.context_name.clear();
        self.unique_names = None;
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct OrdinaryEquipment {
    zone_ids: Vec<ZoneId>,
    space_ids: Vec<SpaceId>,
    input: EquipmentInputOwner,
    nodes: NodeOwner,
    internal_gains: InternalGainStorage,
    get_equipment_input: bool,
    equipment_inputs_filled: bool,
    equipment_simulated_once: bool,
    equipment_data_errors_found: bool,
    equipment_data_found: usize,
    number_of_equipment_lists: usize,
    number_of_timesteps_in_day: usize,
    number_of_supply_paths: usize,
    number_of_return_paths: usize,
    supply_paths: Option<Vec<()>>,
    return_paths: Option<Vec<()>>,
    zone_config: Option<Vec<EquipmentConfig>>,
    equipment_lists: Option<Vec<EquipmentList>>,
    equipment_availability: Option<Vec<i32>>,
    priority_simulation_order: Option<Vec<()>>,
}

/// A borrow of actual Config storage, never a flags projection or snapshot.
pub(crate) struct ZoneEquipmentView<'a> {
    zone_ids: &'a [ZoneId],
    configs: &'a mut [EquipmentConfig],
}
impl ZoneEquipmentActiveElements for ZoneEquipmentView<'_> {
    fn zone_count(&self) -> usize {
        self.zone_ids.len()
    }
    fn zone_id(&self, index: usize) -> ZoneId {
        self.zone_ids[index]
    }
    fn active_elements(&self, index: usize) -> [bool; 3] {
        let row = &self.configs[index];
        [
            row.in_wall_active_element,
            row.in_ceiling_active_element,
            row.in_floor_active_element,
        ]
    }
    fn set_active_elements(&mut self, index: usize, flags: [bool; 3]) {
        let row = &mut self.configs[index];
        // The dispatcher contract is wall, roof/ceiling, floor in that order.
        row.in_wall_active_element = flags[0];
        row.in_ceiling_active_element = flags[1];
        row.in_floor_active_element = flags[2];
    }
}

impl OrdinaryEquipment {
    pub(crate) fn for_geometry(
        model: &ep_model::TypedModel,
        input: OrdinaryEquipmentInput,
    ) -> Result<Self, Unavailable> {
        if model.zones.len() != 1
            || model.spaces.len() != 1
            || model.spaces[0].origin != ep_model::SpaceOrigin::AutoZoneDefault
            || model.spaces[0].zone != model.zones[0].id
            || model.zones[0].spaces.as_slice() != [model.spaces[0].id]
            || model.timestep.number_of_timesteps_per_hour as usize != input.timesteps_per_hour
        {
            return Err(Unavailable::GeometryOrTimestep);
        }
        let zones = [model.zones[0].id];
        let spaces = [model.spaces[0].id];
        let mut owner = Self::new(input, &zones, &spaces)?;
        owner.get_zone_equipment(input.timesteps_per_hour)?;
        owner.validate_initialized(&zones, &spaces)?;
        Ok(owner)
    }

    pub(crate) fn new(
        input: OrdinaryEquipmentInput,
        zone_ids: &[ZoneId],
        space_ids: &[SpaceId],
    ) -> Result<Self, Unavailable> {
        if zone_ids.len() != 1 || space_ids.len() != 1 {
            return Err(Unavailable::GeometryOrTimestep);
        }
        Ok(Self {
            zone_ids: zone_ids.to_vec(),
            space_ids: space_ids.to_vec(),
            input: EquipmentInputOwner::new(input),
            nodes: NodeOwner::default(),
            internal_gains: InternalGainStorage::default(),
            get_equipment_input: true,
            equipment_inputs_filled: false,
            equipment_simulated_once: false,
            equipment_data_errors_found: false,
            equipment_data_found: 0,
            number_of_equipment_lists: 0,
            number_of_timesteps_in_day: 0,
            number_of_supply_paths: 0,
            number_of_return_paths: 0,
            supply_paths: None,
            return_paths: None,
            zone_config: None,
            equipment_lists: None,
            equipment_availability: None,
            priority_simulation_order: None,
        })
    }

    pub(crate) fn get_zone_equipment(
        &mut self,
        timesteps_per_hour: usize,
    ) -> Result<(), Unavailable> {
        if timesteps_per_hour == 0 || timesteps_per_hour > 60 || 60 % timesteps_per_hour != 0 {
            return Err(Unavailable::GeometryOrTimestep);
        }
        if !self.get_equipment_input {
            return Ok(());
        }
        self.get_zone_equipment_data()?;
        self.get_equipment_input = false;
        self.equipment_inputs_filled = true;
        self.number_of_timesteps_in_day = timesteps_per_hour * 24;
        let mut maximum_equipment_types = 0;
        let configs = self
            .zone_config
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?;
        let lists = self
            .equipment_lists
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?;
        for (config, list) in configs.iter().zip(lists) {
            if config.is_controlled {
                maximum_equipment_types =
                    maximum_equipment_types.max(list.number_of_equipment_types);
            }
        }
        if maximum_equipment_types != 0 {
            return Err(Unavailable::InconsistentOwner);
        }
        self.priority_simulation_order = Some(Vec::new());
        Ok(())
    }

    fn get_zone_equipment_data(&mut self) -> Result<(), Unavailable> {
        // Literal224-227: three count queries precede the six definitions.
        let controlled_zones = self.input.count(0);
        let controlled_spaces = self.input.count(1);
        self.number_of_equipment_lists = self.input.count(2);
        if controlled_zones != 0 || controlled_spaces != 0 || self.number_of_equipment_lists != 0 {
            return Err(Unavailable::InconsistentOwner);
        }
        for index in [5, 2, 0, 1, 3, 4] {
            self.input.definition_lookup(index);
        }
        if self.supply_paths.is_none() {
            self.number_of_supply_paths = self.input.count(3);
            self.supply_paths = Some(Vec::new());
        }
        if self.return_paths.is_none() {
            self.number_of_return_paths = self.input.count(4);
            self.return_paths = Some(Vec::new());
        }
        self.internal_gains
            .allocate_if_zone_unallocated(self.zone_ids.len(), self.space_ids.len());
        self.zone_config = Some(vec![EquipmentConfig::default(); self.zone_ids.len()]);
        self.equipment_lists = Some(vec![EquipmentList::default(); self.zone_ids.len()]);
        self.equipment_availability = Some(vec![0; self.zone_ids.len()]); // Avail::Status::NoAction
        self.nodes
            .init_unique_check("ZoneHVAC:EquipmentConnections", &mut self.input)?;
        let configs = self
            .zone_config
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?;
        for (index, list) in self
            .equipment_lists
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?
            .iter()
            .enumerate()
        {
            self.equipment_data_found = configs
                .iter()
                .position(|row| row.equipment_list_name == list.name)
                .map_or(0, |n| n + 1);
            if self.equipment_data_found > 0 {
                configs[self.equipment_data_found - 1].equipment_list_index = index + 1;
            }
        }
        self.nodes
            .end_unique_check("ZoneHVAC:EquipmentConnections")?;
        // Three real .find operations, not three fabricated count queries.
        for index in [6, 7, 8] {
            if self.input.find_size(index)?.is_some() {
                return Err(Unavailable::InconsistentOwner);
            }
        }
        if self.equipment_data_errors_found {
            return Err(Unavailable::Fatal(
                "GetZoneEquipmentData: Errors found in getting Zone Equipment input.".into(),
            ));
        }
        Ok(())
    }

    pub(crate) fn config_view(&mut self) -> Result<ZoneEquipmentView<'_>, Unavailable> {
        let configs = self
            .zone_config
            .as_mut()
            .ok_or(Unavailable::InconsistentOwner)?;
        if configs.len() != self.zone_ids.len() {
            return Err(Unavailable::InconsistentOwner);
        }
        Ok(ZoneEquipmentView {
            zone_ids: &self.zone_ids,
            configs,
        })
    }

    /// Consume represented storage in the actual admitted initializer. This is
    /// an association/scope guard, not a dispatcher readiness result.
    pub(crate) fn validate_initialized(
        &mut self,
        zones: &[ZoneId],
        spaces: &[SpaceId],
    ) -> Result<(), Unavailable> {
        if self.zone_ids != zones
            || self.space_ids != spaces
            || self.get_equipment_input
            || !self.equipment_inputs_filled
            || self.equipment_simulated_once
            || self.equipment_data_errors_found
            || self.equipment_data_found != 1
            || self.number_of_equipment_lists != 0
            || self.number_of_timesteps_in_day == 0
            || self.number_of_supply_paths != 0
            || self.number_of_return_paths != 0
            || self
                .supply_paths
                .as_ref()
                .is_none_or(|rows| !rows.is_empty())
            || self
                .return_paths
                .as_ref()
                .is_none_or(|rows| !rows.is_empty())
            || self
                .priority_simulation_order
                .as_ref()
                .is_none_or(|rows| !rows.is_empty())
            || self
                .equipment_availability
                .as_ref()
                .is_none_or(|rows| rows.as_slice() != [0])
            || self.equipment_lists.as_ref().is_none_or(|rows| {
                rows.len() != 1
                    || !rows[0].name.is_empty()
                    || rows[0].number_of_equipment_types != 0
            })
            || self.nodes.physical.is_some()
            || self.nodes.number_of_nodes != 0
            || self.nodes.number_of_unique_node_names != 0
            || self.nodes.number_of_node_lists != 0
            || self.nodes.get_input
            || self
                .nodes
                .node_lists
                .as_ref()
                .is_none_or(|rows| !rows.is_empty())
            || self.nodes.unique_names.is_some()
            || !self.nodes.context_name.is_empty()
            || self.nodes.number_of_check_nodes != 0
            || self.nodes.max_check_nodes != 100
        {
            return Err(Unavailable::InconsistentOwner);
        }
        for rows in [&self.internal_gains.zone, &self.internal_gains.space] {
            if rows
                .as_ref()
                .is_none_or(|rows| rows.len() != 1 || rows[0] != SpaceZoneGain::default())
            {
                return Err(Unavailable::InconsistentOwner);
            }
        }
        let devices = self
            .internal_gains
            .devices
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?;
        if devices.len() != 1
            || devices[0].number_of_devices != 0
            || devices[0].max_number_of_devices != 0
            || devices[0].devices.is_some()
            || self
                .internal_gains
                .daylight_power_reduction_factor
                .as_ref()
                .is_none_or(|rows| rows.as_slice() != [1.0])
        {
            return Err(Unavailable::InconsistentOwner);
        }
        let configs = self
            .zone_config
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?;
        if configs.iter().any(|row| {
            row.zone_name != "Uncontrolled Zone"
                || !row.equipment_list_name.is_empty()
                || row.equipment_list_index != 1
                || row.zone_node != 0
                || row.inlet_nodes != 0
                || row.exhaust_nodes != 0
                || row.return_nodes != 0
                || row.is_controlled
        }) {
            return Err(Unavailable::InconsistentOwner);
        }
        let view = self.config_view()?;
        if view.zone_count() != zones.len()
            || zones.iter().enumerate().any(|(index, &zone)| {
                view.zone_id(index) != zone || view.active_elements(index) != [false; 3]
            })
        {
            return Err(Unavailable::InconsistentOwner);
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "ordinary_equipment_tests.rs"]
mod tests;
