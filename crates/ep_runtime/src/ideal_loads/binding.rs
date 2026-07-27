//! Model-bound schedule resolution for direct-Zone PurchasedAir coupling.

use crate::{
    heat_balance::state::ZoneHeatBalanceState,
    schedules::ScheduleSeriesCache,
    zone_equipment::{
        IdealLoadsZoneEquipmentDispatchValidation, validate_ideal_loads_zone_equipment_dispatch,
    },
};
use ep_model::{
    IdealLoadsAirSystem, IdealLoadsAirSystemId, LoadDistributionScheme, NodeId, NormalizedName,
    ScheduleId, SimulationModel, ThermostatSetpointId, ZoneEquipmentListId,
    ZoneEquipmentObjectType, ZoneId, ZoneThermostatId,
};

use super::{
    DirectZonePurchasedAirCouplingError, DirectZonePurchasedAirCouplingInput,
    DirectZonePurchasedAirCouplingOutput, IdealLoadsPurchasedAirBranch,
    IdealLoadsSensibleLimitContext, classify_no_oa_no_limit_sensible_subset,
    couple_direct_zone_predicted_demand_to_purchased_air, select_purchased_air_branch,
};

/// One-to-one relation required by the bounded direct-Zone binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectZonePurchasedAirBindingRelation {
    /// One typed Zone in the model.
    Zone,
    /// One Zone-to-thermostat graph edge.
    ZoneThermostatEdge,
    /// One typed Zone thermostat object.
    ZoneThermostat,
    /// One thermostat control entry.
    ThermostatControl,
    /// One thermostat-to-setpoint graph edge.
    ThermostatSetpointEdge,
    /// One typed dual-setpoint object.
    ThermostatDualSetpoint,
    /// One Zone-to-IdealLoads graph edge.
    ZoneIdealLoadsEdge,
    /// One typed IdealLoads system.
    IdealLoadsAirSystem,
    /// One typed Zone equipment connection.
    ZoneEquipmentConnection,
    /// One typed Zone equipment list.
    ZoneEquipmentList,
    /// One Zone equipment list entry.
    ZoneEquipmentListEntry,
    /// One IdealLoads supply-node graph edge.
    IdealLoadsSupplyNode,
    /// One resolved Zone inlet node.
    ZoneInletNode,
    /// One Zone-air-node graph edge.
    ZoneAirNode,
}

/// Static topology feature required by the bounded direct-Zone binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectZonePurchasedAirBindingFeature {
    /// Public typed payload and graph still describe the same model.
    CoherentTypedModelGraph,
    /// The Zone is a standard heat-transfer Zone.
    StandardZoneType,
    /// The Zone is named by `ZoneHVAC:EquipmentConnections`.
    NominallyControlledZone,
    /// The direct Zone multiplier is positive.
    PositiveZoneMultiplier,
    /// The ZoneList multiplier is positive.
    PositiveZoneListMultiplier,
    /// Thermostat cutout hysteresis is not active.
    ZeroCutoutDelta,
    /// The equipment list uses source SequentialLoad distribution.
    SequentialLoadDistribution,
    /// The sole equipment occupies heating and cooling sequence one.
    FirstEquipmentSequence,
    /// Sequential load-fraction schedules are absent.
    NoSequentialFractionSchedules,
    /// The existing Zone equipment dispatch validator has no issue or warning.
    ConformanceCandidateDispatch,
    /// The resolved IdealLoads supply node is the sole Zone inlet.
    DirectSupplyInletIdentity,
    /// The Zone air node is distinct from the supply node.
    DistinctZoneAirNode,
    /// The connection has no exhaust-node topology.
    NoZoneExhaustTopology,
    /// The connection has no return-node topology.
    NoZoneReturnTopology,
    /// The IdealLoads object has no exhaust node.
    NoIdealLoadsExhaustNode,
    /// The IdealLoads object has no system inlet or plenum node.
    NoIdealLoadsSystemInletNode,
    /// Mode-specific heating availability is not configured.
    NoHeatingAvailabilitySchedule,
    /// Mode-specific cooling availability is not configured.
    NoCoolingAvailabilitySchedule,
    /// The system is in the no-OA/no-limit sensible subset.
    NoOaNoLimitSensibleSubset,
    /// The model Zone timestep can produce a positive nominal step.
    PositiveNominalSystemTimestep,
}

/// Fail-closed error while binding one typed model to the CP300 coupling.
#[derive(Clone, Debug, PartialEq)]
pub enum DirectZonePurchasedAirBindingError {
    /// A required relation did not have exactly one member.
    Cardinality {
        /// Relation being checked.
        relation: DirectZonePurchasedAirBindingRelation,
        /// Required cardinality.
        expected: usize,
        /// Actual cardinality.
        actual: usize,
    },
    /// The sole objects or edges did not reference one another.
    IdentityMismatch {
        /// Relation whose IDs did not agree.
        relation: DirectZonePurchasedAirBindingRelation,
    },
    /// A topology or feature lies outside the bounded family.
    UnsupportedFeature {
        /// Rejected feature boundary.
        feature: DirectZonePurchasedAirBindingFeature,
    },
    /// Existing Zone-equipment validation found a blocking issue or scope warning.
    DispatchNotConformanceCandidate {
        /// Complete existing validation evidence.
        validation: Box<IdealLoadsZoneEquipmentDispatchValidation>,
    },
    /// The typed system selected a PurchasedAir branch outside CP300.
    UnsupportedBranch {
        /// Selected branch.
        branch: IdealLoadsPurchasedAirBranch,
    },
}

/// Immutable production binding for one direct fully mixed Zone.
///
/// The binding owns no runtime state. It retains typed identities and the
/// already-resolved IdealLoads object so that per-sample execution performs no
/// string or topology lookup.
#[derive(Clone, Copy, Debug)]
pub struct DirectZonePurchasedAirModelBinding<'model> {
    /// Sole controlled Zone.
    pub zone: ZoneId,
    /// Sole Zone thermostat.
    pub thermostat: ZoneThermostatId,
    /// Sole dual-setpoint object.
    pub dual_setpoint: ThermostatSetpointId,
    /// Thermostat control-type schedule.
    pub control_type_schedule: ScheduleId,
    /// Heating setpoint schedule.
    pub heating_setpoint_schedule: ScheduleId,
    /// Cooling setpoint schedule.
    pub cooling_setpoint_schedule: ScheduleId,
    /// Optional overall IdealLoads availability schedule.
    pub overall_availability_schedule: Option<ScheduleId>,
    /// Sole equipment list.
    pub equipment_list: ZoneEquipmentListId,
    /// Sole IdealLoads system ID.
    pub ideal_loads_air_system: IdealLoadsAirSystemId,
    /// Sole supply/inlet node.
    pub supply_node: NodeId,
    /// Sole direct Zone air node.
    pub zone_air_node: NodeId,
    /// Positive Zone multiplier.
    pub zone_multiplier: u32,
    /// Positive ZoneList multiplier.
    pub zone_list_multiplier: u32,
    /// Fixed model Zone/system timestep for this first boundary.
    pub nominal_system_timestep_seconds: f64,
    /// Site-derived psychrometric context.
    pub limit_context: IdealLoadsSensibleLimitContext,
    /// Prebound typed IdealLoads system.
    pub system: &'model IdealLoadsAirSystem,
}

/// Resolves and validates the static model topology used by CP300.
///
/// This first binding intentionally admits one standard, nominally controlled
/// Zone; one zero-hysteresis DualSetpoint thermostat; and one sequence-one,
/// no-OA/no-limit sensible IdealLoads system with one direct inlet. The
/// function does not claim release-loop integration or non-mixing room-air
/// support.
pub fn bind_direct_zone_purchased_air_model(
    model: &SimulationModel,
) -> Result<DirectZonePurchasedAirModelBinding<'_>, DirectZonePurchasedAirBindingError> {
    let typed = &model.typed;
    let graph = &model.graph;
    if ep_model::ModelGraph::from_typed(typed) != *graph {
        return unsupported(DirectZonePurchasedAirBindingFeature::CoherentTypedModelGraph);
    }

    let zone = require_one(&typed.zones, DirectZonePurchasedAirBindingRelation::Zone)?;
    if zone.zone_type != 1 {
        return unsupported(DirectZonePurchasedAirBindingFeature::StandardZoneType);
    }
    if !zone.is_nominal_controlled {
        return unsupported(DirectZonePurchasedAirBindingFeature::NominallyControlledZone);
    }
    if zone.multiplier == 0 {
        return unsupported(DirectZonePurchasedAirBindingFeature::PositiveZoneMultiplier);
    }
    if zone.list_multiplier == 0 {
        return unsupported(DirectZonePurchasedAirBindingFeature::PositiveZoneListMultiplier);
    }

    let thermostat_edge = require_one(
        &graph.zone_thermostats,
        DirectZonePurchasedAirBindingRelation::ZoneThermostatEdge,
    )?;
    if thermostat_edge.zone != zone.id {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::ZoneThermostatEdge);
    }
    let thermostat = require_one(
        &typed.zone_thermostats,
        DirectZonePurchasedAirBindingRelation::ZoneThermostat,
    )?;
    if thermostat.id != thermostat_edge.thermostat || thermostat.zone != zone.id {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::ZoneThermostat);
    }
    let control = require_one(
        &thermostat.controls,
        DirectZonePurchasedAirBindingRelation::ThermostatControl,
    )?;
    if thermostat.temperature_difference_between_cutout_and_setpoint_delta_c != 0.0 {
        return unsupported(DirectZonePurchasedAirBindingFeature::ZeroCutoutDelta);
    }

    let setpoint_edge = require_one(
        &graph.thermostat_setpoints,
        DirectZonePurchasedAirBindingRelation::ThermostatSetpointEdge,
    )?;
    if setpoint_edge.thermostat != thermostat.id || setpoint_edge.setpoint != control.dual_setpoint
    {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::ThermostatSetpointEdge);
    }
    let setpoint = require_one(
        &typed.thermostat_dual_setpoints,
        DirectZonePurchasedAirBindingRelation::ThermostatDualSetpoint,
    )?;
    if setpoint.id != setpoint_edge.setpoint
        || control.object_type != ep_model::ThermostatControlObjectType::DualSetpoint
    {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::ThermostatDualSetpoint);
    }

    let ideal_loads_edge = require_one(
        &graph.zone_ideal_loads,
        DirectZonePurchasedAirBindingRelation::ZoneIdealLoadsEdge,
    )?;
    if ideal_loads_edge.zone != zone.id {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::ZoneIdealLoadsEdge);
    }
    let system = require_one(
        &typed.ideal_loads_air_systems,
        DirectZonePurchasedAirBindingRelation::IdealLoadsAirSystem,
    )?;
    if system.id != ideal_loads_edge.ideal_loads_air_system {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::IdealLoadsAirSystem);
    }

    let connection = require_one(
        &typed.zone_equipment_connections,
        DirectZonePurchasedAirBindingRelation::ZoneEquipmentConnection,
    )?;
    if connection.zone != zone.id || connection.equipment_list != ideal_loads_edge.equipment_list {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::ZoneEquipmentConnection);
    }
    if connection.zone_air_exhaust_node_or_nodelist_name.is_some() {
        return unsupported(DirectZonePurchasedAirBindingFeature::NoZoneExhaustTopology);
    }
    if connection.zone_return_air_node_or_nodelist_name.is_some()
        || connection
            .zone_return_air_node_1_flow_rate_fraction_schedule
            .is_some()
        || connection
            .zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name
            .is_some()
    {
        return unsupported(DirectZonePurchasedAirBindingFeature::NoZoneReturnTopology);
    }

    let equipment_list = require_one(
        &typed.zone_equipment_lists,
        DirectZonePurchasedAirBindingRelation::ZoneEquipmentList,
    )?;
    if equipment_list.id != ideal_loads_edge.equipment_list {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::ZoneEquipmentList);
    }
    if equipment_list.load_distribution_scheme != LoadDistributionScheme::SequentialLoad {
        return unsupported(DirectZonePurchasedAirBindingFeature::SequentialLoadDistribution);
    }
    let equipment = require_one(
        &equipment_list.equipment,
        DirectZonePurchasedAirBindingRelation::ZoneEquipmentListEntry,
    )?;
    if equipment.object_type != ZoneEquipmentObjectType::IdealLoadsAirSystem
        || equipment.ideal_loads_air_system != system.id
    {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::ZoneEquipmentListEntry);
    }
    if equipment.cooling_sequence != 1
        || equipment.heating_or_no_load_sequence != 1
        || ideal_loads_edge.cooling_sequence != 1
        || ideal_loads_edge.heating_or_no_load_sequence != 1
    {
        return unsupported(DirectZonePurchasedAirBindingFeature::FirstEquipmentSequence);
    }
    if equipment.sequential_cooling_fraction_schedule.is_some()
        || equipment.sequential_heating_fraction_schedule.is_some()
    {
        return unsupported(DirectZonePurchasedAirBindingFeature::NoSequentialFractionSchedules);
    }

    let dispatch_validation = validate_ideal_loads_zone_equipment_dispatch(model, system.id);
    if !dispatch_validation.is_conformance_candidate() {
        return Err(
            DirectZonePurchasedAirBindingError::DispatchNotConformanceCandidate {
                validation: Box::new(dispatch_validation),
            },
        );
    }

    let supply_edge = require_one(
        &graph.ideal_loads_supply_nodes,
        DirectZonePurchasedAirBindingRelation::IdealLoadsSupplyNode,
    )?;
    if supply_edge.ideal_loads_air_system != system.id {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::IdealLoadsSupplyNode);
    }
    let inlet_name = connection
        .zone_air_inlet_node_or_nodelist_name
        .as_ref()
        .ok_or(DirectZonePurchasedAirBindingError::Cardinality {
            relation: DirectZonePurchasedAirBindingRelation::ZoneInletNode,
            expected: 1,
            actual: 0,
        })?;
    let inlet_nodes = resolve_node_or_nodelist(typed, inlet_name);
    let inlet_node = require_one(
        &inlet_nodes,
        DirectZonePurchasedAirBindingRelation::ZoneInletNode,
    )?;
    if *inlet_node != supply_edge.node {
        return unsupported(DirectZonePurchasedAirBindingFeature::DirectSupplyInletIdentity);
    }

    let zone_air_edge = require_one(
        &graph.zone_air_nodes,
        DirectZonePurchasedAirBindingRelation::ZoneAirNode,
    )?;
    if zone_air_edge.zone != zone.id {
        return identity_mismatch(DirectZonePurchasedAirBindingRelation::ZoneAirNode);
    }
    if zone_air_edge.node == supply_edge.node {
        return unsupported(DirectZonePurchasedAirBindingFeature::DistinctZoneAirNode);
    }

    if system.zone_exhaust_air_node_name.is_some() {
        return unsupported(DirectZonePurchasedAirBindingFeature::NoIdealLoadsExhaustNode);
    }
    if system.system_inlet_air_node_name.is_some() {
        return unsupported(DirectZonePurchasedAirBindingFeature::NoIdealLoadsSystemInletNode);
    }
    if system.heating_availability_schedule.is_some() {
        return unsupported(DirectZonePurchasedAirBindingFeature::NoHeatingAvailabilitySchedule);
    }
    if system.cooling_availability_schedule.is_some() {
        return unsupported(DirectZonePurchasedAirBindingFeature::NoCoolingAvailabilitySchedule);
    }

    let branch = select_purchased_air_branch(system);
    if branch != IdealLoadsPurchasedAirBranch::NoOaNoLimitSensible {
        return Err(DirectZonePurchasedAirBindingError::UnsupportedBranch { branch });
    }
    if !classify_no_oa_no_limit_sensible_subset(system).is_supported() {
        return unsupported(DirectZonePurchasedAirBindingFeature::NoOaNoLimitSensibleSubset);
    }

    let timesteps_per_hour = typed.timestep.number_of_timesteps_per_hour;
    if timesteps_per_hour == 0 {
        return unsupported(DirectZonePurchasedAirBindingFeature::PositiveNominalSystemTimestep);
    }
    let nominal_system_timestep_seconds = 3_600.0 / f64::from(timesteps_per_hour);
    if !nominal_system_timestep_seconds.is_finite() || nominal_system_timestep_seconds <= 0.0 {
        return unsupported(DirectZonePurchasedAirBindingFeature::PositiveNominalSystemTimestep);
    }
    let limit_context = typed
        .site
        .as_ref()
        .and_then(|site| IdealLoadsSensibleLimitContext::from_site_elevation_m(site.elevation_m))
        .unwrap_or_default();

    Ok(DirectZonePurchasedAirModelBinding {
        zone: zone.id,
        thermostat: thermostat.id,
        dual_setpoint: setpoint.id,
        control_type_schedule: thermostat.control_type_schedule,
        heating_setpoint_schedule: setpoint.heating_setpoint_schedule,
        cooling_setpoint_schedule: setpoint.cooling_setpoint_schedule,
        overall_availability_schedule: system.availability_schedule,
        equipment_list: equipment_list.id,
        ideal_loads_air_system: system.id,
        supply_node: supply_edge.node,
        zone_air_node: zone_air_edge.node,
        zone_multiplier: zone.multiplier,
        zone_list_multiplier: zone.list_multiplier,
        nominal_system_timestep_seconds,
        limit_context,
        system,
    })
}

/// Schedule role resolved for one model-bound coupling call.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectZonePurchasedAirScheduleRole {
    /// `ZoneControl:Thermostat` control type.
    ThermostatControlType,
    /// Active heating setpoint.
    HeatingSetpoint,
    /// Active cooling setpoint.
    CoolingSetpoint,
    /// Overall IdealLoads availability.
    OverallAvailability,
}

/// Dynamic fixed-timestep invariant required by the first scheduled caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DirectZonePurchasedAirRuntimeInvariant {
    /// Runtime state belongs to the prebound Zone.
    BoundZoneIdentity,
    /// System-timestep history is active at the fixed predictor boundary.
    SystemTimestepHistory,
    /// Adaptive shortening is inactive.
    UnshortenedSystemTimestep,
    /// The previous Zone timestep used one system step.
    SinglePreviousSystemTimestep,
    /// The requested system timestep equals the model Zone timestep.
    NominalSystemTimestep,
    /// `PriorTimeStep` equals the model Zone timestep.
    NominalPriorTimestep,
}

/// Fail-closed error for one model-bound schedule sample and CP300 call.
#[derive(Clone, Debug, PartialEq)]
pub enum DirectZonePurchasedAirScheduledCouplingError {
    /// The requested logical sample lies outside the supplied cache.
    SampleIndexOutOfRange {
        /// Requested sample.
        sample_index: usize,
        /// Cache sample count.
        sample_count: usize,
    },
    /// The cache has no series for a required typed schedule.
    MissingSchedule {
        /// Schedule role.
        role: DirectZonePurchasedAirScheduleRole,
        /// Missing typed schedule.
        schedule: ScheduleId,
    },
    /// A required current schedule value is NaN or infinite.
    NonFiniteScheduleValue {
        /// Schedule role.
        role: DirectZonePurchasedAirScheduleRole,
        /// Typed schedule.
        schedule: ScheduleId,
    },
    /// The thermostat control schedule is not exact DualHeatCool value 4.
    UnsupportedControlType {
        /// Current finite schedule value.
        value: f64,
    },
    /// Current heating setpoint exceeds current cooling setpoint.
    HeatingSetpointAboveCoolingSetpoint {
        /// Current heating setpoint in C.
        heating_setpoint_c: f64,
        /// Current cooling setpoint in C.
        cooling_setpoint_c: f64,
    },
    /// Dynamic state lies outside the fixed-timestep direct-Zone boundary.
    RuntimeInvariant {
        /// Rejected invariant.
        invariant: DirectZonePurchasedAirRuntimeInvariant,
    },
    /// CP300 rejected predictor, PurchasedAir, or feedback state.
    Coupling(DirectZonePurchasedAirCouplingError),
}

/// Values sampled before one model-bound CP300 call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectZonePurchasedAirScheduleSnapshot {
    /// Logical schedule sample index.
    pub sample_index: usize,
    /// Exact thermostat control-type value.
    pub control_type: f64,
    /// Active heating setpoint in C.
    pub heating_setpoint_c: f64,
    /// Active cooling setpoint in C.
    pub cooling_setpoint_c: f64,
    /// Overall availability value, or one when no schedule is configured.
    pub overall_availability: f64,
    /// Overall availability projected to the CP300 on/off input.
    pub unit_available: bool,
}

/// Input for one prebound schedule sample and CP300 call.
pub struct DirectZonePurchasedAirScheduledCouplingInput<'a, 'model> {
    /// Immutable topology/model binding.
    pub binding: &'a DirectZonePurchasedAirModelBinding<'model>,
    /// Schedule cache built from the bound model for the active time axis.
    ///
    /// `ScheduleSeriesCache` does not retain model identity, so same-model
    /// provenance is a caller precondition rather than a property this API can
    /// verify.
    pub schedule_cache: &'a ScheduleSeriesCache,
    /// Active logical schedule sample.
    pub schedule_sample_index: usize,
    /// Live Zone heat-balance state.
    pub zone_state: &'a mut ZoneHeatBalanceState,
    /// Active system timestep in seconds.
    pub system_timestep_seconds: f64,
}

/// Output from one successful model-bound schedule sample and CP300 call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DirectZonePurchasedAirScheduledCouplingOutput {
    /// Fully resolved current schedule values.
    pub schedules: DirectZonePurchasedAirScheduleSnapshot,
    /// Predictor, PurchasedAir, and feedback result from CP300.
    pub coupling: DirectZonePurchasedAirCouplingOutput,
}

/// Samples the bound schedules and calls CP300 transactionally.
///
/// Schedule and runtime-state validation completes before the mutable Zone
/// state is passed to CP300. CP300 itself buffers all feedback before its
/// two-field commit, so every error leaves the complete Zone state unchanged.
pub fn couple_model_bound_direct_zone_purchased_air(
    input: DirectZonePurchasedAirScheduledCouplingInput<'_, '_>,
) -> Result<
    DirectZonePurchasedAirScheduledCouplingOutput,
    DirectZonePurchasedAirScheduledCouplingError,
> {
    let binding = input.binding;
    let sample_index = input.schedule_sample_index;
    let sample_count = input.schedule_cache.sample_count();
    if sample_index >= sample_count {
        return Err(
            DirectZonePurchasedAirScheduledCouplingError::SampleIndexOutOfRange {
                sample_index,
                sample_count,
            },
        );
    }

    validate_runtime_state(binding, input.zone_state, input.system_timestep_seconds)?;

    let control_type = schedule_value(
        input.schedule_cache,
        binding.control_type_schedule,
        sample_index,
        DirectZonePurchasedAirScheduleRole::ThermostatControlType,
    )?;
    if control_type != 4.0 {
        return Err(
            DirectZonePurchasedAirScheduledCouplingError::UnsupportedControlType {
                value: control_type,
            },
        );
    }
    let cooling_setpoint_c = schedule_value(
        input.schedule_cache,
        binding.cooling_setpoint_schedule,
        sample_index,
        DirectZonePurchasedAirScheduleRole::CoolingSetpoint,
    )?;
    let heating_setpoint_c = schedule_value(
        input.schedule_cache,
        binding.heating_setpoint_schedule,
        sample_index,
        DirectZonePurchasedAirScheduleRole::HeatingSetpoint,
    )?;
    if heating_setpoint_c > cooling_setpoint_c {
        return Err(
            DirectZonePurchasedAirScheduledCouplingError::HeatingSetpointAboveCoolingSetpoint {
                heating_setpoint_c,
                cooling_setpoint_c,
            },
        );
    }
    let overall_availability = if let Some(schedule) = binding.overall_availability_schedule {
        schedule_value(
            input.schedule_cache,
            schedule,
            sample_index,
            DirectZonePurchasedAirScheduleRole::OverallAvailability,
        )?
    } else {
        1.0
    };
    let unit_available = overall_availability > 0.0;
    let schedules = DirectZonePurchasedAirScheduleSnapshot {
        sample_index,
        control_type,
        heating_setpoint_c,
        cooling_setpoint_c,
        overall_availability,
        unit_available,
    };
    let zone_node_temperature_c = input.zone_state.mean_air_temperature_c;

    let coupling =
        couple_direct_zone_predicted_demand_to_purchased_air(DirectZonePurchasedAirCouplingInput {
            zone_state: input.zone_state,
            heating_setpoint_c,
            cooling_setpoint_c,
            zone_node_temperature_c,
            load_correction_factor: 1.0,
            zone_multiplier: binding.zone_multiplier,
            zone_list_multiplier: binding.zone_list_multiplier,
            system_timestep_seconds: input.system_timestep_seconds,
            system: binding.system,
            supply_node: binding.supply_node,
            unit_available,
            limit_context: binding.limit_context,
        })
        .map_err(DirectZonePurchasedAirScheduledCouplingError::Coupling)?;

    Ok(DirectZonePurchasedAirScheduledCouplingOutput {
        schedules,
        coupling,
    })
}

fn validate_runtime_state(
    binding: &DirectZonePurchasedAirModelBinding<'_>,
    zone_state: &ZoneHeatBalanceState,
    system_timestep_seconds: f64,
) -> Result<(), DirectZonePurchasedAirScheduledCouplingError> {
    if zone_state.zone_id != binding.zone {
        return runtime_invariant(DirectZonePurchasedAirRuntimeInvariant::BoundZoneIdentity);
    }
    if zone_state.use_zone_timestep_history {
        return runtime_invariant(DirectZonePurchasedAirRuntimeInvariant::SystemTimestepHistory);
    }
    if zone_state.shorten_timestep_sys {
        return runtime_invariant(
            DirectZonePurchasedAirRuntimeInvariant::UnshortenedSystemTimestep,
        );
    }
    if zone_state.previous_system_timestep_count != 1 {
        return runtime_invariant(
            DirectZonePurchasedAirRuntimeInvariant::SinglePreviousSystemTimestep,
        );
    }
    if system_timestep_seconds != binding.nominal_system_timestep_seconds {
        return runtime_invariant(DirectZonePurchasedAirRuntimeInvariant::NominalSystemTimestep);
    }
    if zone_state.prior_timestep_seconds != binding.nominal_system_timestep_seconds {
        return runtime_invariant(DirectZonePurchasedAirRuntimeInvariant::NominalPriorTimestep);
    }
    Ok(())
}

fn schedule_value(
    cache: &ScheduleSeriesCache,
    schedule: ScheduleId,
    sample_index: usize,
    role: DirectZonePurchasedAirScheduleRole,
) -> Result<f64, DirectZonePurchasedAirScheduledCouplingError> {
    let value = cache
        .value(schedule, sample_index)
        .ok_or(DirectZonePurchasedAirScheduledCouplingError::MissingSchedule { role, schedule })?;
    if !value.is_finite() {
        return Err(
            DirectZonePurchasedAirScheduledCouplingError::NonFiniteScheduleValue { role, schedule },
        );
    }
    Ok(value)
}

fn resolve_node_or_nodelist(model: &ep_model::TypedModel, name: &NormalizedName) -> Vec<NodeId> {
    if let Some(node) = model.node_names.resolve(&name.0) {
        return vec![node];
    }
    if let Some(node_list) = model.node_list_names.resolve(&name.0)
        && let Some(list) = model.node_lists.iter().find(|list| list.id == node_list)
    {
        return list.nodes.clone();
    }
    Vec::new()
}

fn require_one<T>(
    values: &[T],
    relation: DirectZonePurchasedAirBindingRelation,
) -> Result<&T, DirectZonePurchasedAirBindingError> {
    if values.len() != 1 {
        return Err(DirectZonePurchasedAirBindingError::Cardinality {
            relation,
            expected: 1,
            actual: values.len(),
        });
    }
    Ok(&values[0])
}

fn unsupported<T>(
    feature: DirectZonePurchasedAirBindingFeature,
) -> Result<T, DirectZonePurchasedAirBindingError> {
    Err(DirectZonePurchasedAirBindingError::UnsupportedFeature { feature })
}

fn identity_mismatch<T>(
    relation: DirectZonePurchasedAirBindingRelation,
) -> Result<T, DirectZonePurchasedAirBindingError> {
    Err(DirectZonePurchasedAirBindingError::IdentityMismatch { relation })
}

fn runtime_invariant<T>(
    invariant: DirectZonePurchasedAirRuntimeInvariant,
) -> Result<T, DirectZonePurchasedAirScheduledCouplingError> {
    Err(DirectZonePurchasedAirScheduledCouplingError::RuntimeInvariant { invariant })
}

#[cfg(test)]
#[path = "binding_tests.rs"]
mod tests;
