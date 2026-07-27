//! Source-ordered transitions for the bounded PurchasedAir initialization path.

use ep_model::{
    AutosizeOrNumber, IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit, NodeId,
    ZoneEquipmentListId, ZoneId,
};

use super::{
    PURCHASED_AIR_INIT_LIFECYCLE_SOURCE, PurchasedAirInitDiagnostic,
    PurchasedAirInitDiagnosticKind, PurchasedAirInitLifecycleSummary, PurchasedAirInitManagerPlan,
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
    /// Units visited by the global equipment-list sweep on this call.
    pub equipment_list_units_scanned: usize,
    /// Missing memberships diagnosed by the global sweep on this call.
    pub equipment_list_membership_missing: usize,
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
    /// The selected system is absent from the immutable manager plan.
    SelectedSystemMissingFromManagerPlan {
        /// Missing typed system.
        system: IdealLoadsAirSystemId,
    },
    /// A replay attempted to change the source declaration order.
    DeclaredSystemOrderChanged {
        /// Order retained by the allocated manager arena.
        expected: Vec<IdealLoadsAirSystemId>,
        /// Order supplied by the replay plan.
        actual: Vec<IdealLoadsAirSystemId>,
    },
    /// A replay attempted to change a retained first-match result.
    ManagerPlanMembershipChanged {
        /// System whose immutable lookup result changed.
        system: IdealLoadsAirSystemId,
        /// First match retained by the allocated arena.
        expected: Option<ZoneEquipmentListId>,
        /// First match supplied by the replay plan.
        actual: Option<ZoneEquipmentListId>,
    },
    /// The allocated manager arena is internally missing a declared unit.
    ManagerArenaMissingSystem {
        /// Missing typed system.
        system: IdealLoadsAirSystemId,
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
    manager_plan: &PurchasedAirInitManagerPlan,
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
    if !manager_plan
        .rows()
        .iter()
        .any(|row| row.system == system.id)
    {
        return Err(
            PurchasedAirInitError::SelectedSystemMissingFromManagerPlan { system: system.id },
        );
    }
    if state.module_initialized {
        validate_replayed_manager_plan(state, manager_plan)?;
    }
    if !state.module_initialized {
        allocate_unit_state(state, manager_plan);
        transition.module_initialized = true;
    }
    if !state.equipment_list_checked && context.zone_equipment_inputs_filled {
        validate_manager_arena_complete(state, manager_plan)?;
        state.equipment_list_checked = true;
        state.equipment_list_check_count += 1;
        transition.equipment_list_checked = true;
        for (index, row) in manager_plan.rows().iter().enumerate() {
            let scan_ordinal = index + 1;
            let membership_found = row.first_matching_equipment_list.is_some();
            state.equipment_list_scan_order.push(row.system);
            state.equipment_list_scanned_unit_count += 1;
            transition.equipment_list_units_scanned += 1;
            let unit = state
                .units
                .get_mut(&row.system)
                .ok_or(PurchasedAirInitError::ManagerArenaMissingSystem { system: row.system })?;
            unit.equipment_list_scan_ordinal = Some(scan_ordinal);
            unit.first_matching_equipment_list = row.first_matching_equipment_list;
            unit.equipment_list_membership_found = Some(membership_found);
            if !membership_found {
                state.equipment_list_missing_unit_count += 1;
                transition.equipment_list_membership_missing += 1;
                state
                    .equipment_list_diagnostics
                    .push(PurchasedAirInitDiagnostic {
                        system: row.system,
                        scan_ordinal,
                        kind: PurchasedAirInitDiagnosticKind::EquipmentListMembershipMissing,
                    });
            }
        }
    }

    let existing_unit = state
        .units
        .get(&system.id)
        .ok_or(PurchasedAirInitError::UnknownSystem { system: system.id })?;
    if existing_unit.one_time_initialized
        && (existing_unit.controlled_zone != Some(topology.controlled_zone)
            || existing_unit.equipment_list != Some(topology.equipment_list)
            || existing_unit.supply_node != Some(topology.supply_node)
            || existing_unit.recirculation_node != Some(topology.recirculation_node))
    {
        return Err(PurchasedAirInitError::LatchedTopologyChanged { system: system.id });
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
        declared_system_order: state.declared_system_order.clone(),
        equipment_list_scan_order: state.equipment_list_scan_order.clone(),
        equipment_list_scanned_unit_count: state.equipment_list_scanned_unit_count,
        equipment_list_missing_unit_count: state.equipment_list_missing_unit_count,
        equipment_list_diagnostics: state.equipment_list_diagnostics.clone(),
        equipment_list_scan_ordinal: unit.equipment_list_scan_ordinal,
        first_matching_equipment_list: unit.first_matching_equipment_list,
        equipment_list_membership_found: unit.equipment_list_membership_found,
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

fn allocate_unit_state(
    state: &mut PurchasedAirRuntimeState,
    manager_plan: &PurchasedAirInitManagerPlan,
) {
    state.declared_system_order = manager_plan.system_order().collect();
    for row in manager_plan.rows() {
        state.units.insert(
            row.system,
            PurchasedAirUnitRuntimeState::new(row.system, row.first_matching_equipment_list),
        );
    }
    state.module_initialized = true;
    state.module_initialization_count += 1;
}

fn validate_replayed_manager_plan(
    state: &PurchasedAirRuntimeState,
    manager_plan: &PurchasedAirInitManagerPlan,
) -> Result<(), PurchasedAirInitError> {
    let actual_order: Vec<_> = manager_plan.system_order().collect();
    if actual_order != state.declared_system_order {
        return Err(PurchasedAirInitError::DeclaredSystemOrderChanged {
            expected: state.declared_system_order.clone(),
            actual: actual_order,
        });
    }
    validate_manager_arena_complete(state, manager_plan)?;
    for row in manager_plan.rows() {
        let unit = state
            .units
            .get(&row.system)
            .ok_or(PurchasedAirInitError::ManagerArenaMissingSystem { system: row.system })?;
        if unit.planned_first_matching_equipment_list != row.first_matching_equipment_list {
            return Err(PurchasedAirInitError::ManagerPlanMembershipChanged {
                system: row.system,
                expected: unit.planned_first_matching_equipment_list,
                actual: row.first_matching_equipment_list,
            });
        }
    }
    Ok(())
}

fn validate_manager_arena_complete(
    state: &PurchasedAirRuntimeState,
    manager_plan: &PurchasedAirInitManagerPlan,
) -> Result<(), PurchasedAirInitError> {
    for row in manager_plan.rows() {
        if !state.units.contains_key(&row.system) {
            return Err(PurchasedAirInitError::ManagerArenaMissingSystem { system: row.system });
        }
    }
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
