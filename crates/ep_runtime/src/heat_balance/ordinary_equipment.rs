//! Actual geometry GetZoneEquipment subset; no HVAC or dispatcher activation.
mod heat_to_return_air;
pub(crate) mod management;
mod node_reporting;
mod source_input;
mod source_equipment_types;
mod source_equipment_getter;
mod source_controlled_equipment;
mod source_node_connections;
mod source_nodes;
mod source_node_lists;
mod source_endrun;
mod source_marked_nodes;
mod source_reset;
mod source_setup_reporting;

use super::inside_convection_dispatcher::ZoneEquipmentActiveElements;
use source_nodes::{SourceNodeData, SourceNodeSetpointCheck};
use source_node_lists::SourceNodeList;
use source_equipment_types::{SourceAirIn, SourceEquipmentData, SourceSimulationOrder};
use super::selected_schedule_manager::SourceScheduleIndex;
pub(crate) use source_input::SourceEquipmentInputOwners;
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
    /// GetZoneEquipment uses the effective DataGlobal clock after GetProjectData.
    /// This updates only that operand; the admitted family inventory is unchanged.
    pub(crate) fn apply_source_timestep(&mut self, steps: usize) -> Result<(), Unavailable> {
        if steps == 0 || steps > 60 || 60 % steps != 0 {
            return Err(Unavailable::GeometryOrTimestep);
        }
        self.timesteps_per_hour = steps;
        Ok(())
    }

    /// Carry actual positive presence/counts to the retained Source IP/getter.
    /// This does not initialize equipment or certify any positive branch.
    ///
    /// # Errors
    /// Rejects an invalid clock or duplicate supplied family identity.
    pub fn from_source_family_inventory<'a>(
        inventory: impl IntoIterator<Item = (&'a str, usize)>,
        timesteps_per_hour: usize,
    ) -> Result<Self, OrdinaryEquipmentUnavailable> {
        if timesteps_per_hour == 0 || timesteps_per_hour > 60 || 60 % timesteps_per_hour != 0 {
            return Err(Unavailable::GeometryOrTimestep);
        }
        let mut supplied = [None; 9];
        for (family, count) in inventory {
            if let Some(index) = Self::FAMILIES.iter().position(|&name| name == family) {
                if supplied[index].is_some() { return Err(Unavailable::Input(family.into())); }
                supplied[index] = Some(count);
            }
        }
        Ok(Self { supplied, timesteps_per_hour })
    }

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
    control_list_name: String,
    return_flow_fraction_schedule: Option<SourceScheduleIndex>,
    number_return_flow_basis_nodes: usize,
    flow_error: bool,
    inlet_node_numbers: Option<Vec<usize>>,
    inlet_node_air_loop_numbers: Option<Vec<i32>>,
    inlet_node_air_distribution_unit_numbers: Option<Vec<i32>>,
    exhaust_node_numbers: Option<Vec<usize>>,
    return_node_numbers: Option<Vec<usize>>,
    return_node_air_loop_numbers: Option<Vec<i32>>,
    return_node_return_path_numbers: Option<Vec<i32>>,
    return_node_return_path_component_numbers: Option<Vec<i32>>,
    return_node_inlet_numbers: Option<Vec<i32>>,
    fixed_return_flow: Option<Vec<bool>>,
    return_node_plenum_numbers: Option<Vec<i32>>,
    return_flow_basis_node_numbers: Option<Vec<usize>>,
    return_node_exhaust_node_numbers: Option<Vec<i32>>,
    shared_exhaust_nodes: Option<Vec<i32>>,
    return_node_space_mixer_indices: Option<Vec<i32>>,
    air_distribution_units_heat: Option<Vec<SourceAirIn>>,
    air_distribution_units_cool: Option<Vec<SourceAirIn>>,
    zone_exhaust: f64,
    zone_exhaust_balanced: f64,
    plenum_mass_flow: f64,
    excess_zone_exhaust: f64,
    total_available_air_loop_outdoor_air: f64,
    total_inlet_air_mass_flow_rate: f64,
    total_exhaust_air_mass_flow_rate: f64,
    air_loop_design_supply: f64,
    zone_has_air_loop_with_outdoor_air_system: bool,
    zone_air_distribution_index: i32,
    zone_design_specification_outdoor_air_index: i32,
    zone_name: String,
    equipment_list_name: String,
    equipment_list_index: usize,
    zone_node: usize,
    inlet_nodes: usize,
    exhaust_nodes: usize,
    return_nodes: usize,
    is_controlled: bool,
    /// Actual DataZoneEquipment EquipConfiguration constructorfalse; genuine HVAC writer.
    zonal_system_only: bool,
    in_floor_active_element: bool,
    in_wall_active_element: bool,
    in_ceiling_active_element: bool,
}

impl Default for EquipmentConfig {
    fn default() -> Self {
        Self {
            control_list_name: String::new(),
            return_flow_fraction_schedule: None,
            number_return_flow_basis_nodes: 0,
            flow_error: false,
            inlet_node_numbers: None,
            inlet_node_air_loop_numbers: None,
            inlet_node_air_distribution_unit_numbers: None,
            exhaust_node_numbers: None,
            return_node_numbers: None,
            return_node_air_loop_numbers: None,
            return_node_return_path_numbers: None,
            return_node_return_path_component_numbers: None,
            return_node_inlet_numbers: None,
            fixed_return_flow: None,
            return_node_plenum_numbers: None,
            return_flow_basis_node_numbers: None,
            return_node_exhaust_node_numbers: None,
            shared_exhaust_nodes: None,
            return_node_space_mixer_indices: None,
            air_distribution_units_heat: None,
            air_distribution_units_cool: None,
            zone_exhaust: 0.0,
            zone_exhaust_balanced: 0.0,
            plenum_mass_flow: 0.0,
            excess_zone_exhaust: 0.0,
            total_available_air_loop_outdoor_air: 0.0,
            total_inlet_air_mass_flow_rate: 0.0,
            total_exhaust_air_mass_flow_rate: 0.0,
            air_loop_design_supply: 0.0,
            zone_has_air_loop_with_outdoor_air_system: false,
            zone_air_distribution_index: 0,
            zone_design_specification_outdoor_air_index: 0,
            zone_name: "Uncontrolled Zone".into(),
            equipment_list_name: String::new(),
            equipment_list_index: 0,
            zone_node: 0,
            inlet_nodes: 0,
            exhaust_nodes: 0,
            return_nodes: 0,
            is_controlled: false,
            zonal_system_only: false,
            in_floor_active_element: false,
            in_wall_active_element: false,
            in_ceiling_active_element: false,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct EquipmentList {
    load_distribution_scheme: i32,
    number_available_heating_equipment: i32,
    number_available_cooling_equipment: i32,
    equipment_type_names: Option<Vec<String>>,
    equipment_types: Option<Vec<i32>>,
    equipment_names: Option<Vec<String>>,
    equipment_indices: Option<Vec<i32>>,
    equipment_splitter_indices: Option<Vec<i32>>,
    component_pointers: Vec<Option<std::convert::Infallible>>,
    equipment_data: Option<Vec<SourceEquipmentData>>,
    cooling_priorities: Option<Vec<i32>>,
    heating_priorities: Option<Vec<i32>>,
    cooling_capacities: Option<Vec<i32>>,
    heating_capacities: Option<Vec<i32>>,
    sequential_cooling_fraction_schedules: Option<Vec<Option<SourceScheduleIndex>>>,
    sequential_heating_fraction_schedules: Option<Vec<Option<SourceScheduleIndex>>>,
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
    /// Genuine DataLoopNode.Node storage, independent of native counters.
    physical: Option<Vec<SourceNodeData>>,
    /// Native NodeID has lower bound0; only index0 is Undefined.
    node_ids: Option<Vec<String>>,
    default_node_values: SourceNodeData,
    node_setpoint_checks: Option<Vec<SourceNodeSetpointCheck>>,
    number_of_nodes: usize,
    number_of_unique_node_names: usize,
    number_of_node_lists: usize,
    get_input: bool,
    node_lists: Option<Vec<SourceNodeList>>,
    unique_names: Option<Vec<String>>,
    context_name: String,
    number_of_check_nodes: usize,
    max_check_nodes: usize,
    reporting: node_reporting::SourceNodeReporting,
    node_variables_setup: bool,
    more_node_info: Option<Vec<source_setup_reporting::SourceMoreNodeData>>,
    marked_nodes: Option<Vec<source_marked_nodes::SourceMarkedNode>>,
    node_references: Option<Vec<i32>>,
    only_single_node_first_time: bool,
    only_single_node_numbers: Option<Vec<usize>>,
}

impl Default for NodeOwner {
    fn default() -> Self {
        Self {
            physical: None,
            node_ids: None,
            default_node_values: SourceNodeData::default(),
            node_setpoint_checks: None,
            number_of_nodes: 0,
            number_of_unique_node_names: 0,
            number_of_node_lists: 0,
            get_input: true,
            node_lists: None,
            unique_names: None,
            context_name: String::new(),
            number_of_check_nodes: 0,
            max_check_nodes: 0,
            reporting: node_reporting::SourceNodeReporting::default(),
            node_variables_setup: false,
            more_node_info: None,
            marked_nodes: None,
            node_references: None,
            only_single_node_first_time: true,
            only_single_node_numbers: None,
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
        self.begin_unique_check_context(context)
    }

    fn begin_unique_check_context(&mut self, context: &str) -> Result<(), Unavailable> {
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
    input: Option<EquipmentInputOwner>,
    nodes: NodeOwner,
    internal_gains: InternalGainStorage,
    unique_equipment_list_names: std::collections::HashSet<String>,
    get_equipment_input: bool,
    equipment_inputs_filled: bool,
    equipment_simulated_once: bool,
    management_modules: management::SourceEquipmentManagementModules,
    zone_equipment_mixers: Vec<std::convert::Infallible>,
    zone_return_mixers: Vec<std::convert::Infallible>,
    init_equipment_one_time_flag: bool,
    init_equipment_environment_flag: bool,
    first_pass_zone_equipment_flag: bool,
    space_config: Option<Vec<EquipmentConfig>>,
    /// Same native DataDefineEquip.AirDistUnit; constructor-unallocated, positive rows unported.
    air_distribution_units: Option<Vec<std::convert::Infallible>>,
    /// Actual DataZoneEquipment.hh634–635 counts and arrays; native default0/None.
    number_exhaust_air_systems: i32,
    number_zone_exhaust_controls: i32,
    exhaust_air_systems: Option<Vec<std::convert::Infallible>>,
    zone_exhaust_controls: Option<Vec<std::convert::Infallible>>,
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
    priority_simulation_order: Option<Vec<SourceSimulationOrder>>,
}

/// Borrowed actual DataZoneEquip Config record, with no copied scalar flags.
pub(crate) struct SourceZoneEquipmentConfigRef<'a> {
    row: &'a EquipmentConfig,
}
impl SourceZoneEquipmentConfigRef<'_> {
    pub(crate) fn is_controlled(&self) -> &bool {
        &self.row.is_controlled
    }
    pub(crate) fn number_inlet_nodes(&self) -> &usize {
        &self.row.inlet_nodes
    }
    pub(crate) fn number_exhaust_nodes(&self) -> &usize {
        &self.row.exhaust_nodes
    }
    pub(crate) fn number_return_nodes(&self) -> &usize {
        &self.row.return_nodes
    }
}

/// Temporary disjoint references into the sole actual DataLoopNode.Node row.
/// Prediction rollback and setpoint handoff mutate this same physical owner.
pub(crate) struct SourceZoneAirNodeMut<'a> {
    pub(crate) temperature: &'a mut f64,
    pub(crate) humidity_ratio: &'a mut f64,
    pub(crate) enthalpy: &'a mut f64,
    pub(crate) temperature_setpoint: &'a mut f64,
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
    pub(crate) fn source_zone_air_node_mut(
        &mut self,
        ordinal: i32,
    ) -> Result<SourceZoneAirNodeMut<'_>, super::selected_heat_balance_input::HeatBalanceInputError> {
        use super::selected_heat_balance_input::HeatBalanceInputError as E;
        let at = ordinal.checked_sub(1).and_then(|n| usize::try_from(n).ok())
            .ok_or(E::Unavailable("actual prediction Node positive ordinal"))?;
        let row = self.nodes.physical.as_deref_mut().and_then(|rows| rows.get_mut(at))
            .ok_or(E::Unavailable("actual prediction same physical Node allocation"))?;
        Ok(SourceZoneAirNodeMut {
            temperature: &mut row.temperature,
            humidity_ratio: &mut row.humidity_ratio,
            enthalpy: &mut row.enthalpy,
            temperature_setpoint: &mut row.temperature_setpoint,
        })
    }
    /// Actual VerifyControlledZoneForThermostat5702 exact-name search.
    pub(crate) fn verify_source_controlled_zone_for_thermostat(&self, name: &str) -> bool {
        self.zone_config.as_ref().is_some_and(|rows| rows.iter().any(|row| row.zone_name == name))
    }
    /// Actual ReportLoopConnections2656 allocated(ZoneEquipConfig), not an input-count inference.
    pub(crate) fn source_zone_equipment_config_allocated(&self) -> bool {
        self.zone_config.is_some()
    }
    pub(crate) fn source_number_return_paths(&self) -> &usize {
        &self.number_of_return_paths
    }
    pub(crate) fn source_number_supply_paths(&self) -> &usize {
        &self.number_of_supply_paths
    }
    pub(crate) fn source_air_distribution_units(&self) -> Option<&[std::convert::Infallible]> {
        self.air_distribution_units.as_deref()
    }
    pub(crate) fn source_zone_equipment_config(
        &self,
        ordinal: i32,
    ) -> Result<
        SourceZoneEquipmentConfigRef<'_>,
        super::selected_heat_balance_input::HeatBalanceInputError,
    > {
        let at = ordinal
            .checked_sub(1)
            .and_then(|n| usize::try_from(n).ok())
            .ok_or(
                super::selected_heat_balance_input::HeatBalanceInputError::Unavailable(
                    "actual ZoneEquipConfig Source ordinal",
                ),
            )?;
        let row = self
            .zone_config
            .as_deref()
            .and_then(|rows| rows.get(at))
            .ok_or(
                super::selected_heat_balance_input::HeatBalanceInputError::Unavailable(
                    "actual same ZoneEquipConfig allocation",
                ),
            )?;
        Ok(SourceZoneEquipmentConfigRef { row })
    }
    /// Same genuine DataZoneEquipment.NumExhaustAirSystems; no Plant/Node inference.
    pub(crate) fn source_number_exhaust_air_systems(&self) -> &i32 {
        &self.number_exhaust_air_systems
    }

    /// Same actual DataZoneEquip spaceEquipConfig; no synthesized uncontrolled row.
    pub(crate) fn source_space_equipment_config(
        &self,
        ordinal: i32,
    ) -> Result<
        SourceZoneEquipmentConfigRef<'_>,
        super::selected_heat_balance_input::HeatBalanceInputError,
    > {
        let at = ordinal
            .checked_sub(1)
            .and_then(|n| usize::try_from(n).ok())
            .ok_or(
                super::selected_heat_balance_input::HeatBalanceInputError::Unavailable(
                    "actual spaceEquipConfig Source ordinal",
                ),
            )?;
        let row = self
            .space_config
            .as_deref()
            .and_then(|rows| rows.get(at))
            .ok_or(
                super::selected_heat_balance_input::HeatBalanceInputError::Unavailable(
                    "actual same spaceEquipConfig allocation",
                ),
            )?;
        Ok(SourceZoneEquipmentConfigRef { row })
    }
    /// Same independent DataLoopNode.NumOfNodes owner; no physical-array length inference.
    pub(crate) fn source_number_of_nodes(&self) -> &usize {
        &self.nodes.number_of_nodes
    }
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
        Self::new_owners(Some(EquipmentInputOwner::new(input)), zone_ids, space_ids)
    }

    fn new_owners(
        input: Option<EquipmentInputOwner>,
        zone_ids: &[ZoneId],
        space_ids: &[SpaceId],
    ) -> Result<Self, Unavailable> {
        if zone_ids.len() != 1 || space_ids.len() != 1 {
            return Err(Unavailable::GeometryOrTimestep);
        }
        Ok(Self {
            zone_ids: zone_ids.to_vec(),
            space_ids: space_ids.to_vec(),
            input,
            nodes: NodeOwner::default(),
            internal_gains: InternalGainStorage::default(),
            unique_equipment_list_names: std::collections::HashSet::new(),
            get_equipment_input: true,
            equipment_inputs_filled: false,
            equipment_simulated_once: false,
            management_modules: Default::default(),
            zone_equipment_mixers: Vec::new(),
            zone_return_mixers: Vec::new(),
            init_equipment_one_time_flag: true,
            init_equipment_environment_flag: true,
            first_pass_zone_equipment_flag: true,
            space_config: None,
            air_distribution_units: None,
            number_exhaust_air_systems: 0,
            number_zone_exhaust_controls: 0,
            exhaust_air_systems: None,
            zone_exhaust_controls: None,
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
        self.finish_get_zone_equipment(timesteps_per_hour)
    }

    fn finish_get_zone_equipment(&mut self, timesteps_per_hour: usize) -> Result<(), Unavailable> {
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
        self.priority_simulation_order = Some(vec![SourceSimulationOrder::default(); maximum_equipment_types]);
        Ok(())
    }

    fn get_zone_equipment_data(&mut self) -> Result<(), Unavailable> {
        // Literal224-227: three count queries precede the six definitions.
        let controlled_zones = self
            .input
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?
            .count(0);
        let controlled_spaces = self
            .input
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?
            .count(1);
        self.number_of_equipment_lists = self
            .input
            .as_ref()
            .ok_or(Unavailable::InconsistentOwner)?
            .count(2);
        if controlled_zones != 0 || controlled_spaces != 0 || self.number_of_equipment_lists != 0 {
            return Err(Unavailable::InconsistentOwner);
        }
        for index in [5, 2, 0, 1, 3, 4] {
            self.input
                .as_mut()
                .ok_or(Unavailable::InconsistentOwner)?
                .definition_lookup(index);
        }
        if self.supply_paths.is_none() {
            self.number_of_supply_paths = self
                .input
                .as_ref()
                .ok_or(Unavailable::InconsistentOwner)?
                .count(3);
            self.supply_paths = Some(Vec::new());
        }
        if self.return_paths.is_none() {
            self.number_of_return_paths = self
                .input
                .as_ref()
                .ok_or(Unavailable::InconsistentOwner)?
                .count(4);
            self.return_paths = Some(Vec::new());
        }
        self.internal_gains
            .allocate_if_zone_unallocated(self.zone_ids.len(), self.space_ids.len());
        self.zone_config = Some(vec![EquipmentConfig::default(); self.zone_ids.len()]);
        self.equipment_lists = Some(vec![EquipmentList::default(); self.zone_ids.len()]);
        self.equipment_availability = Some(vec![0; self.zone_ids.len()]); // Avail::Status::NoAction
        self.nodes.init_unique_check(
            "ZoneHVAC:EquipmentConnections",
            self.input.as_mut().ok_or(Unavailable::InconsistentOwner)?,
        )?;
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
            if self
                .input
                .as_ref()
                .ok_or(Unavailable::InconsistentOwner)?
                .find_size(index)?
                .is_some()
            {
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
        self.validate_initialized_storage(zones, spaces, None)
    }

    /// Direct read of the sole Source GetZoneEquipment169-197 scalar owner.
    pub(crate) fn source_equipment_inputs_filled(&self) -> &bool {
        &self.equipment_inputs_filled
    }

    fn validate_initialized_storage(
        &mut self,
        zones: &[ZoneId],
        spaces: &[SpaceId],
        source_gains:Option<&crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains>,
    ) -> Result<(), Unavailable> {
        if let Some(gains) = source_gains {
            if self.zone_config.as_ref().is_some_and(|rows| rows.iter().any(|row| row.is_controlled)) {
                return self.validate_source_controlled_equipment_storage(zones, spaces, gains);
            }
        }
        if self.zone_ids != zones
            || self.space_ids != spaces
            || self.get_equipment_input
            || !self.equipment_inputs_filled
            || (source_gains.is_none() && self.equipment_simulated_once)
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
        if let Some(gains) = source_gains {
            // Actual Source route never allocates a second legacy gain collection.
            if self.internal_gains.zone.is_some()
                || self.internal_gains.space.is_some()
                || self.internal_gains.devices.is_some()
                || self
                    .internal_gains
                    .daylight_power_reduction_factor
                    .is_some()
            {
                return Err(Unavailable::InconsistentOwner);
            }
            gains
                .validate_source_initial_gain_allocations(zones.len(), spaces.len())
                .map_err(|_| Unavailable::InconsistentOwner)?;
        } else {
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

/// A temporary borrow of the actual optional ZoneEquipConfig allocation.
/// InitIntConvCoeff does not require this allocation before ActiveSurfaceCheck.
pub(crate) struct SourceConvectionEquipment<'a> {
    zone_ids: &'a [ZoneId],
    configs: &'a mut [EquipmentConfig],
}
impl ZoneEquipmentActiveElements for SourceConvectionEquipment<'_> {
    fn zone_count(&self) -> usize {
        self.configs.len()
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
        row.in_wall_active_element = flags[0];
        row.in_ceiling_active_element = flags[1];
        row.in_floor_active_element = flags[2];
    }
}
impl OrdinaryEquipment {
    /// Read genuine module and Node allocation flags; no getter/cadence is run.
    pub(crate) fn source_convection_readiness(
        &self,
        begin_environment: bool,
        system_sizing: bool,
        zone_sizing: bool,
    ) -> super::inside_convection_dispatcher::InsideConvectionReadiness {
        super::inside_convection_dispatcher::InsideConvectionReadiness {
            begin_environment,
            system_sizing,
            zone_sizing,
            equipment_inputs_filled: self.equipment_inputs_filled,
            nodes_allocated: self.nodes.physical.is_some(),
            equipment_simulated_once: self.equipment_simulated_once,
        }
    }
    /// No allocation or numeric projection: retain the real optional Config.
    pub(crate) fn source_convection_equipment(&mut self) -> SourceConvectionEquipment<'_> {
        SourceConvectionEquipment {
            zone_ids: &self.zone_ids,
            configs: match self.zone_config.as_deref_mut() {
                Some(rows) => rows,
                // The actual allocation remains None. The empty view cannot
                // pass the reached Config cardinality guard for a real Zone.
                None => &mut [],
            },
        }
    }
}
