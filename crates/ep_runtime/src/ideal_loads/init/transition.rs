//! Source-ordered transitions for the bounded PurchasedAir initialization path.

use ep_model::{
    AutosizeOrNumber, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit, NodeId,
    ZoneEquipmentListId, ZoneId,
};

use super::{
    PURCHASED_AIR_INIT_LIFECYCLE_SOURCE, PurchasedAirInitLifecycleSummary,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};

/// Prevalidated topology consumed by the persistent initialization state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PurchasedAirInitBoundTopology {
    /// IdealLoads system being initialized.
    pub system: IdealLoadsAirSystemId,
    /// Controlled Zone selected by the caller.
    pub controlled_zone: ZoneId,
    /// Equipment list proven to contain the system.
    pub equipment_list: ZoneEquipmentListId,
    /// Supply node proven to be a controlled-Zone inlet.
    pub supply_node: NodeId,
    /// Exhaust-or-return node selected for recirculation.
    pub recirculation_node: NodeId,
    /// Whether the system belongs to the selected equipment list.
    pub equipment_list_membership_verified: bool,
    /// Whether an unsupported return-plenum/system-inlet route is active.
    pub return_plenum_active: bool,
}

/// Dynamic values visible to one `InitPurchasedAir` call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirInitCallContext {
    /// Whether Zone equipment input is ready for the global membership pass.
    pub zone_equipment_inputs_filled: bool,
    /// Whether the simulation is currently inside the system sizing calculation.
    pub system_sizing_calculation: bool,
    /// Current begin-environment flag.
    pub begin_environment: bool,
    /// Standard air density used by begin-environment mass-flow conversion.
    pub standard_air_density_kg_per_m3: f64,
    /// Active heating thermostat setpoint.
    pub heating_setpoint_c: f64,
    /// Active cooling thermostat setpoint.
    pub cooling_setpoint_c: f64,
    /// Current overall availability value.
    pub overall_availability: f64,
    /// Current heating availability value.
    pub heating_availability: f64,
    /// Current cooling availability value.
    pub cooling_availability: f64,
}

/// Transitions performed by one initialization call.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PurchasedAirInitTransition {
    /// Per-unit arena was allocated on this call.
    pub module_initialized: bool,
    /// Global equipment-list check latched on this call.
    pub equipment_list_checked: bool,
    /// Per-unit topology latched on this call.
    pub one_time_initialized: bool,
    /// Hard-size/sizing gate completed on this call.
    pub sizing_checked: bool,
    /// Begin-environment values were written on this call.
    pub environment_initialized: bool,
    /// Environment latch was rearmed on this call.
    pub environment_rearmed: bool,
    /// Cooling recurring diagnostic was active on this call.
    pub cooling_supply_temperature_warning: bool,
    /// Heating recurring diagnostic was active on this call.
    pub heating_supply_temperature_warning: bool,
}

/// Snapshot returned after one source-ordered initialization call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirInitSnapshot {
    /// System identity.
    pub system: IdealLoadsAirSystemId,
    /// Controlled Zone identity.
    pub controlled_zone: ZoneId,
    /// Supply-node identity.
    pub supply_node: NodeId,
    /// Recirculation-node identity.
    pub recirculation_node: NodeId,
    /// Source-shaped flag state after the call.
    pub flags: super::IdealLoadsInitFlags,
    /// Transitions performed by this call.
    pub transition: PurchasedAirInitTransition,
    /// Cached maximum heating air mass flow.
    pub maximum_heating_air_mass_flow_rate_kg_per_s: f64,
    /// Cached maximum cooling air mass flow.
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    /// Standard density owning the cached values, once environment init ran.
    pub standard_air_density_kg_per_m3: Option<f64>,
}

/// Fail-closed error for the bounded persistent initialization lifecycle.
#[derive(Clone, Debug, PartialEq)]
pub enum PurchasedAirInitError {
    /// The declared system arena contains a repeated typed ID.
    DuplicateSystemId {
        /// Repeated typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The bounded lifecycle was asked to allocate more or fewer than one unit.
    UnsupportedDeclaredSystemCount {
        /// Rejected declared-system count.
        actual: usize,
    },
    /// The sole declared system does not match the initialized typed object.
    DeclaredSystemIdentityMismatch {
        /// System being initialized.
        expected: IdealLoadsAirSystemId,
        /// Sole declared system.
        actual: IdealLoadsAirSystemId,
    },
    /// The selected system is absent from the allocated arena.
    UnknownSystem {
        /// Missing typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The selected topology belongs to another system.
    SystemIdentityMismatch {
        /// Typed system object identity.
        expected: IdealLoadsAirSystemId,
        /// Bound topology identity.
        actual: IdealLoadsAirSystemId,
    },
    /// Equipment-list membership was not proven.
    EquipmentListMembershipNotVerified {
        /// Unverified typed system.
        system: IdealLoadsAirSystemId,
    },
    /// Return-plenum/system-inlet topology is not typed by this runtime.
    ReturnPlenumUnsupported {
        /// System selecting a plenum path.
        system: IdealLoadsAirSystemId,
    },
    /// A replay attempted to change a latched topology identity.
    LatchedTopologyChanged {
        /// System whose topology changed.
        system: IdealLoadsAirSystemId,
    },
    /// Autosizing reached the still-unported `SizePurchasedAir` boundary.
    AutosizingNotImplemented {
        /// System requiring autosizing.
        system: IdealLoadsAirSystemId,
        /// Autosized source field.
        field: &'static str,
    },
    /// A required hard-sized value is missing, negative, NaN, or infinite.
    InvalidHardSize {
        /// System with an invalid hard size.
        system: IdealLoadsAirSystemId,
        /// Invalid source field.
        field: &'static str,
    },
    /// Begin-environment standard air density is not finite and positive.
    InvalidStandardAirDensity {
        /// Rejected density.
        value: f64,
    },
}

/// Advances the persistent source-order initialization lifecycle for one unit.
pub fn init_purchased_air_runtime(
    state: &mut PurchasedAirRuntimeState,
    declared_systems: &[IdealLoadsAirSystemId],
    topology: PurchasedAirInitBoundTopology,
    system: &IdealLoadsAirSystem,
    context: PurchasedAirInitCallContext,
) -> Result<PurchasedAirInitSnapshot, PurchasedAirInitError> {
    let mut transition = PurchasedAirInitTransition::default();
    if topology.system != system.id {
        return Err(PurchasedAirInitError::SystemIdentityMismatch {
            expected: system.id,
            actual: topology.system,
        });
    }
    validate_single_declared_system(declared_systems, system.id)?;
    if !state.module_initialized {
        allocate_unit_state(state, declared_systems)?;
        transition.module_initialized = true;
    }
    if !state.equipment_list_checked && context.zone_equipment_inputs_filled {
        state.equipment_list_checked = true;
        state.equipment_list_check_count += 1;
        transition.equipment_list_checked = true;
    }
    if state.equipment_list_checked && !topology.equipment_list_membership_verified {
        return Err(PurchasedAirInitError::EquipmentListMembershipNotVerified {
            system: system.id,
        });
    }
    if topology.return_plenum_active {
        return Err(PurchasedAirInitError::ReturnPlenumUnsupported { system: system.id });
    }

    let unit = state
        .units
        .get_mut(&system.id)
        .ok_or(PurchasedAirInitError::UnknownSystem { system: system.id })?;
    unit.init_call_count += 1;
    if !unit.one_time_initialized {
        unit.controlled_zone = Some(topology.controlled_zone);
        unit.equipment_list = Some(topology.equipment_list);
        unit.supply_node = Some(topology.supply_node);
        unit.recirculation_node = Some(topology.recirculation_node);
        unit.one_time_initialized = true;
        unit.one_time_initialization_count += 1;
        transition.one_time_initialized = true;
    } else if unit.controlled_zone != Some(topology.controlled_zone)
        || unit.equipment_list != Some(topology.equipment_list)
        || unit.supply_node != Some(topology.supply_node)
        || unit.recirculation_node != Some(topology.recirculation_node)
    {
        return Err(PurchasedAirInitError::LatchedTopologyChanged { system: system.id });
    }

    if !context.system_sizing_calculation && unit.sizing_needed {
        validate_hard_sizes(system)?;
        unit.sizing_needed = false;
        unit.sizing_check_count += 1;
        transition.sizing_checked = true;
    }
    if context.begin_environment && unit.environment_initialization_needed {
        initialize_environment(unit, system, context.standard_air_density_kg_per_m3)?;
        unit.environment_initialization_needed = false;
        unit.environment_initialization_count += 1;
        transition.environment_initialized = true;
    } else if !context.begin_environment && !unit.environment_initialization_needed {
        unit.environment_initialization_needed = true;
        unit.environment_rearm_count += 1;
        transition.environment_rearmed = true;
    }

    transition.cooling_supply_temperature_warning =
        cooling_supply_temperature_warning_active(system, context);
    if transition.cooling_supply_temperature_warning {
        unit.cooling_supply_temperature_warning_count += 1;
    }
    transition.heating_supply_temperature_warning =
        heating_supply_temperature_warning_active(system, context);
    if transition.heating_supply_temperature_warning {
        unit.heating_supply_temperature_warning_count += 1;
    }

    Ok(PurchasedAirInitSnapshot {
        system: system.id,
        controlled_zone: topology.controlled_zone,
        supply_node: topology.supply_node,
        recirculation_node: topology.recirculation_node,
        flags: unit.flags(state.equipment_list_checked),
        transition,
        maximum_heating_air_mass_flow_rate_kg_per_s: unit
            .maximum_heating_air_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_kg_per_s: unit
            .maximum_cooling_air_mass_flow_rate_kg_per_s,
        standard_air_density_kg_per_m3: unit.standard_air_density_kg_per_m3,
    })
}

/// Builds the final lifecycle report for one declared system.
pub fn purchased_air_init_lifecycle_summary(
    state: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<PurchasedAirInitLifecycleSummary, PurchasedAirInitError> {
    let unit = state
        .units
        .get(&system)
        .ok_or(PurchasedAirInitError::UnknownSystem { system })?;
    Ok(PurchasedAirInitLifecycleSummary {
        source: PURCHASED_AIR_INIT_LIFECYCLE_SOURCE,
        flags: unit.flags(state.equipment_list_checked),
        module_initialization_count: state.module_initialization_count,
        equipment_list_check_count: state.equipment_list_check_count,
        init_call_count: unit.init_call_count,
        one_time_initialization_count: unit.one_time_initialization_count,
        sizing_check_count: unit.sizing_check_count,
        environment_initialization_count: unit.environment_initialization_count,
        environment_rearm_count: unit.environment_rearm_count,
        maximum_heating_air_mass_flow_rate_kg_per_s: unit
            .maximum_heating_air_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_kg_per_s: unit
            .maximum_cooling_air_mass_flow_rate_kg_per_s,
        standard_air_density_kg_per_m3: unit.standard_air_density_kg_per_m3,
        cooling_supply_temperature_warning_count: unit.cooling_supply_temperature_warning_count,
        heating_supply_temperature_warning_count: unit.heating_supply_temperature_warning_count,
    })
}

fn validate_single_declared_system(
    declared_systems: &[IdealLoadsAirSystemId],
    expected: IdealLoadsAirSystemId,
) -> Result<(), PurchasedAirInitError> {
    if declared_systems.len() != 1 {
        return Err(PurchasedAirInitError::UnsupportedDeclaredSystemCount {
            actual: declared_systems.len(),
        });
    }
    if declared_systems[0] != expected {
        return Err(PurchasedAirInitError::DeclaredSystemIdentityMismatch {
            expected,
            actual: declared_systems[0],
        });
    }
    Ok(())
}

fn allocate_unit_state(
    state: &mut PurchasedAirRuntimeState,
    declared_systems: &[IdealLoadsAirSystemId],
) -> Result<(), PurchasedAirInitError> {
    for system in declared_systems {
        if state
            .units
            .insert(*system, PurchasedAirUnitRuntimeState::new(*system))
            .is_some()
        {
            state.units.clear();
            return Err(PurchasedAirInitError::DuplicateSystemId { system: *system });
        }
    }
    state.module_initialized = true;
    state.module_initialization_count += 1;
    Ok(())
}

fn validate_hard_sizes(system: &IdealLoadsAirSystem) -> Result<(), PurchasedAirInitError> {
    validate_limit_value(
        system.id,
        system.heating_limit,
        system.maximum_heating_air_flow_rate_m3_per_s,
        system.maximum_sensible_heating_capacity_w,
        "maximum_heating_air_flow_rate_m3_per_s",
        "maximum_sensible_heating_capacity_w",
    )?;
    validate_limit_value(
        system.id,
        system.cooling_limit,
        system.maximum_cooling_air_flow_rate_m3_per_s,
        system.maximum_total_cooling_capacity_w,
        "maximum_cooling_air_flow_rate_m3_per_s",
        "maximum_total_cooling_capacity_w",
    )
}

fn validate_limit_value(
    system: IdealLoadsAirSystemId,
    limit: IdealLoadsLimit,
    flow: Option<AutosizeOrNumber>,
    capacity: Option<AutosizeOrNumber>,
    flow_field: &'static str,
    capacity_field: &'static str,
) -> Result<(), PurchasedAirInitError> {
    if matches!(
        limit,
        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
    ) {
        require_hard_size(system, flow, flow_field)?;
    }
    if matches!(
        limit,
        IdealLoadsLimit::LimitCapacity | IdealLoadsLimit::LimitFlowRateAndCapacity
    ) {
        require_hard_size(system, capacity, capacity_field)?;
    }
    Ok(())
}

fn require_hard_size(
    system: IdealLoadsAirSystemId,
    value: Option<AutosizeOrNumber>,
    field: &'static str,
) -> Result<f64, PurchasedAirInitError> {
    match value {
        Some(AutosizeOrNumber::Value(value)) if value.is_finite() && value >= 0.0 => Ok(value),
        Some(AutosizeOrNumber::Autosize) => {
            Err(PurchasedAirInitError::AutosizingNotImplemented { system, field })
        }
        Some(AutosizeOrNumber::Value(_)) | None => {
            Err(PurchasedAirInitError::InvalidHardSize { system, field })
        }
    }
}

fn initialize_environment(
    unit: &mut PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    standard_air_density_kg_per_m3: f64,
) -> Result<(), PurchasedAirInitError> {
    if !standard_air_density_kg_per_m3.is_finite() || standard_air_density_kg_per_m3 <= 0.0 {
        return Err(PurchasedAirInitError::InvalidStandardAirDensity {
            value: standard_air_density_kg_per_m3,
        });
    }
    unit.maximum_heating_air_mass_flow_rate_kg_per_s = initialized_mass_flow(
        system.id,
        system.heating_limit,
        system.maximum_heating_air_flow_rate_m3_per_s,
        standard_air_density_kg_per_m3,
        "maximum_heating_air_flow_rate_m3_per_s",
    )?;
    unit.maximum_cooling_air_mass_flow_rate_kg_per_s = initialized_mass_flow(
        system.id,
        system.cooling_limit,
        system.maximum_cooling_air_flow_rate_m3_per_s,
        standard_air_density_kg_per_m3,
        "maximum_cooling_air_flow_rate_m3_per_s",
    )?;
    unit.standard_air_density_kg_per_m3 = Some(standard_air_density_kg_per_m3);
    Ok(())
}

fn initialized_mass_flow(
    system: IdealLoadsAirSystemId,
    limit: IdealLoadsLimit,
    volume_flow: Option<AutosizeOrNumber>,
    density: f64,
    field: &'static str,
) -> Result<f64, PurchasedAirInitError> {
    if !matches!(
        limit,
        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
    ) {
        return Ok(0.0);
    }
    Ok(require_hard_size(system, volume_flow, field)? * density)
}

fn cooling_supply_temperature_warning_active(
    system: &IdealLoadsAirSystem,
    context: PurchasedAirInitCallContext,
) -> bool {
    system.minimum_cooling_supply_air_temperature_c > context.cooling_setpoint_c
        && context.cooling_setpoint_c != 0.0
        && system.cooling_limit == IdealLoadsLimit::NoLimit
        && nominally_on(context.overall_availability)
        && nominally_on(context.cooling_availability)
}

fn heating_supply_temperature_warning_active(
    system: &IdealLoadsAirSystem,
    context: PurchasedAirInitCallContext,
) -> bool {
    system.maximum_heating_supply_air_temperature_c < context.heating_setpoint_c
        && context.heating_setpoint_c != 0.0
        && system.heating_limit == IdealLoadsLimit::NoLimit
        && nominally_on(context.overall_availability)
        && nominally_on(context.heating_availability)
}

fn nominally_on(value: f64) -> bool {
    value > 0.0 || value.is_nan()
}
