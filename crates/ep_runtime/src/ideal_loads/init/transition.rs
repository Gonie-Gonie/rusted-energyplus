//! Source-ordered transitions for the bounded PurchasedAir initialization path.

use ep_model::{
    IdealLoadsAirSystem, IdealLoadsAirSystemId, IdealLoadsLimit, NodeId, ZoneEquipmentListId,
    ZoneId,
};

use super::super::{
    PurchasedAirHardSizeField, PurchasedAirHardSizeLegacyContext, PurchasedAirHardSizeLegacyError,
    PurchasedAirHardSizeLegacyOutcome, PurchasedAirSizedLimits,
    size_purchased_air_direct_hard_sized_legacy_route,
};
use super::topology_transition::advance_selected_unit_topology;
use super::{
    PURCHASED_AIR_INIT_LIFECYCLE_SOURCE, PurchasedAirInitDiagnostic,
    PurchasedAirInitDiagnosticKind, PurchasedAirInitLifecycleSummary, PurchasedAirInitManagerPlan,
    PurchasedAirInitTopologyError, PurchasedAirInitTopologyPlan, PurchasedAirRecirculationSource,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
};

/// Dynamic values visible to one `InitPurchasedAir` call.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirInitCallContext {
    /// Whether Zone equipment input is ready for the global membership pass.
    pub zone_equipment_inputs_filled: bool,
    /// Whether the simulation is currently inside the system sizing calculation.
    pub system_sizing_calculation: bool,
    /// Dynamic inputs to the bounded `SizePurchasedAir` legacy route.
    pub sizing: PurchasedAirHardSizeLegacyContext,
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
    /// Per-unit topology reached the normal one-time tail on this call.
    pub topology_completed: bool,
    /// Ordered topology diagnostics emitted on this call.
    pub topology_diagnostics_emitted: usize,
    /// OA/economizer flow-limit advisory emitted on this call.
    pub economizer_flow_limit_warning: bool,
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
    pub recirculation_node: Option<NodeId>,
    /// Source branch that selected or left recirculation unassigned.
    pub recirculation_source: Option<PurchasedAirRecirculationSource>,
    /// Configured exhaust rejected before return fallback.
    pub rejected_exhaust_node: Option<NodeId>,
    /// First return named by the multiple-return warning.
    pub reported_first_return_node: Option<NodeId>,
    /// Number of retained one-time topology diagnostics.
    pub topology_diagnostic_count: usize,
    /// Source-shaped flag state after the call.
    pub flags: super::IdealLoadsInitFlags,
    /// Transitions performed by this call.
    pub transition: PurchasedAirInitTransition,
    /// Cached maximum heating air mass flow.
    pub maximum_heating_air_mass_flow_rate_kg_per_s: f64,
    /// Cached maximum cooling air mass flow.
    pub maximum_cooling_air_mass_flow_rate_kg_per_s: f64,
    /// Runtime-owned four-field sizing overlay.
    pub sized_limits: PurchasedAirSizedLimits,
    /// Retained direct hard-size child outcome, once completed.
    pub sizing_outcome: Option<PurchasedAirHardSizeLegacyOutcome>,
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
    /// The bounded `SizePurchasedAir` child rejected its route or values.
    Sizing(PurchasedAirHardSizeLegacyError),
    /// Begin-environment initialization ran before a sizing child completed.
    SizingStateUnavailable {
        /// Selected system.
        system: IdealLoadsAirSystemId,
    },
    /// Begin-environment standard air density is not finite and positive.
    InvalidStandardAirDensity {
        /// Rejected density.
        value: f64,
    },
    /// Source semantic topology failure after the one-time latch committed.
    Topology(PurchasedAirInitTopologyError),
}

/// Advances the persistent source-order initialization lifecycle for one unit.
pub fn init_purchased_air_runtime(
    state: &mut PurchasedAirRuntimeState,
    manager_plan: &PurchasedAirInitManagerPlan,
    topology: &PurchasedAirInitTopologyPlan,
    system: &IdealLoadsAirSystem,
    context: PurchasedAirInitCallContext,
) -> Result<PurchasedAirInitSnapshot, PurchasedAirInitError> {
    let mut transition = PurchasedAirInitTransition::default();
    if topology.system() != system.id {
        return Err(PurchasedAirInitError::SystemIdentityMismatch {
            expected: system.id,
            actual: topology.system(),
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

    let unit = state
        .units
        .get_mut(&system.id)
        .ok_or(PurchasedAirInitError::UnknownSystem { system: system.id })?;
    advance_selected_unit_topology(unit, topology, system, &mut transition)?;
    if unit.sized_limits.is_none() {
        unit.sized_limits = Some(PurchasedAirSizedLimits::from_system(system));
    }

    if !context.system_sizing_calculation && unit.sizing_needed {
        unit.sizing_attempt_count += 1;
        let sized_limits = unit
            .sized_limits
            .as_mut()
            .ok_or(PurchasedAirInitError::SizingStateUnavailable { system: system.id })?;
        let sizing_outcome =
            size_purchased_air_direct_hard_sized_legacy_route(system, sized_limits, context.sizing)
                .map_err(PurchasedAirInitError::Sizing)?;
        unit.sizing_outcome = Some(sizing_outcome);
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
        controlled_zone: topology.controlled_zone(),
        supply_node: topology.supply_node(),
        recirculation_node: unit.recirculation_node,
        recirculation_source: unit.recirculation_source,
        rejected_exhaust_node: unit.rejected_exhaust_node,
        reported_first_return_node: unit.reported_first_return_node,
        topology_diagnostic_count: unit.topology_diagnostics.len(),
        flags: unit.flags(state.equipment_list_checked),
        transition,
        maximum_heating_air_mass_flow_rate_kg_per_s: unit
            .maximum_heating_air_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_kg_per_s: unit
            .maximum_cooling_air_mass_flow_rate_kg_per_s,
        sized_limits: unit
            .sized_limits
            .ok_or(PurchasedAirInitError::SizingStateUnavailable { system: system.id })?,
        sizing_outcome: unit.sizing_outcome,
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
        controlled_zone: unit.controlled_zone,
        equipment_list: unit.equipment_list,
        supply_node: unit.supply_node,
        recirculation_node: unit.recirculation_node,
        recirculation_source: unit.recirculation_source,
        rejected_exhaust_node: unit.rejected_exhaust_node,
        reported_first_return_node: unit.reported_first_return_node,
        topology_diagnostics: unit.topology_diagnostics.clone(),
        topology_failure: unit.topology_failure,
        init_call_count: unit.init_call_count,
        one_time_initialization_count: unit.one_time_initialization_count,
        topology_completion_count: unit.topology_completion_count,
        sizing_check_count: unit.sizing_check_count,
        sizing_attempt_count: unit.sizing_attempt_count,
        sized_limits: unit.sized_limits,
        sizing_outcome: unit.sizing_outcome,
        environment_initialization_count: unit.environment_initialization_count,
        environment_rearm_count: unit.environment_rearm_count,
        maximum_heating_air_mass_flow_rate_kg_per_s: unit
            .maximum_heating_air_mass_flow_rate_kg_per_s,
        maximum_cooling_air_mass_flow_rate_kg_per_s: unit
            .maximum_cooling_air_mass_flow_rate_kg_per_s,
        standard_air_density_kg_per_m3: unit.standard_air_density_kg_per_m3,
        cooling_supply_temperature_warning_count: unit.cooling_supply_temperature_warning_count,
        heating_supply_temperature_warning_count: unit.heating_supply_temperature_warning_count,
        economizer_flow_limit_warning_count: unit.economizer_flow_limit_warning_count,
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
    let sized_limits = unit
        .sized_limits
        .ok_or(PurchasedAirInitError::SizingStateUnavailable { system: system.id })?;
    let maximum_heating_air_mass_flow_rate_kg_per_s = initialized_mass_flow(
        system.id,
        system.heating_limit,
        sized_limits.maximum_heating_air_flow_rate_m3_per_s,
        standard_air_density_kg_per_m3,
        PurchasedAirHardSizeField::MaximumHeatingAirFlowRate,
    )?;
    let maximum_cooling_air_mass_flow_rate_kg_per_s = initialized_mass_flow(
        system.id,
        system.cooling_limit,
        sized_limits.maximum_cooling_air_flow_rate_m3_per_s,
        standard_air_density_kg_per_m3,
        PurchasedAirHardSizeField::MaximumCoolingAirFlowRate,
    )?;
    unit.maximum_heating_air_mass_flow_rate_kg_per_s = maximum_heating_air_mass_flow_rate_kg_per_s;
    unit.maximum_cooling_air_mass_flow_rate_kg_per_s = maximum_cooling_air_mass_flow_rate_kg_per_s;
    unit.standard_air_density_kg_per_m3 = Some(standard_air_density_kg_per_m3);
    Ok(())
}

fn initialized_mass_flow(
    system: IdealLoadsAirSystemId,
    limit: IdealLoadsLimit,
    volume_flow: Option<ep_model::AutosizeOrNumber>,
    density: f64,
    field: PurchasedAirHardSizeField,
) -> Result<f64, PurchasedAirInitError> {
    if !matches!(
        limit,
        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
    ) {
        return Ok(0.0);
    }
    match volume_flow {
        Some(ep_model::AutosizeOrNumber::Value(volume_flow))
            if volume_flow.is_finite() && volume_flow >= 0.0 =>
        {
            Ok(volume_flow * density)
        }
        Some(ep_model::AutosizeOrNumber::Autosize) => Err(PurchasedAirInitError::Sizing(
            PurchasedAirHardSizeLegacyError::AutosizingNotImplemented { system, field },
        )),
        Some(ep_model::AutosizeOrNumber::Value(_)) => Err(PurchasedAirInitError::Sizing(
            PurchasedAirHardSizeLegacyError::InvalidHardSize { system, field },
        )),
        None => Err(PurchasedAirInitError::Sizing(
            PurchasedAirHardSizeLegacyError::MissingRequiredHardSize { system, field },
        )),
    }
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
