//! Bounded `CalcPurchAirLoads` minimum-outdoor-air prefix lifecycle.

use ep_model::{IdealLoadsAirSystem, IdealLoadsAirSystemId, ZoneId};

use super::super::PurchasedAirRuntimeState;
use super::lifecycle::{PurchasedAirCalcEntryRuntimeState, PurchasedAirCalcEntrySnapshot};

/// Parent source slice represented by this bounded transition.
pub const PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2023-2040";

/// Child dependency reached at the one parent call site.
pub const PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE: &str = "EnergyPlus 26.1 PurchasedAirManager.cc:2762-2810; bounded no-OA route 2781,2783,2785,2806-2809";

/// Source-order sites retained by the bounded prefix.
pub const PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER: &[&str] = &[
    "resolve-zone-heat-balance-reference",
    "call-calc-purch-air-min-oa-mass-flow",
    "child-zero-no-outdoor-air-working-flow",
    "child-write-retained-minimum-outdoor-air-flow",
    "read-ems-outdoor-air-override-flag",
    "apply-ems-outdoor-air-flow-if-enabled",
    "read-outdoor-air-enabled",
    "calculate-outdoor-air-specific-heat-if-enabled",
    "calculate-or-zero-minimum-outdoor-air-sensible-output",
    "calculate-or-zero-minimum-outdoor-air-moisture-output",
];

/// One CP310-to-CP311 transition result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcMinimumOaPrefixSnapshot {
    /// Parent EnergyPlus source slice.
    pub source: &'static str,
    /// Bounded child dependency.
    pub minimum_oa_child_source: &'static str,
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP310 call ordinal consumed by this transition.
    pub parent_call_ordinal: usize,
    /// Source-order sites represented by this bounded route.
    pub source_order: &'static [&'static str],
    /// Controlled Zone used to bind the Zone heat-balance reference.
    pub controlled_zone: ZoneId,
    /// Whether CP310 entered the line-2022 body.
    pub unit_body_entered: bool,
    /// Whether the line-2023 Zone heat-balance reference was bound.
    pub zone_heat_balance_reference_bound: bool,
    /// Whether the line-2025 child call site was reached.
    pub minimum_oa_child_called: bool,
    /// Whether the bounded child took its no-outdoor-air route.
    pub minimum_oa_child_no_outdoor_air_route: bool,
    /// Pre-EMS minimum flow retained by the child, absent when UnitOn is false.
    pub retained_minimum_outdoor_air_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether the child performed its retained minimum-flow write.
    pub retained_minimum_outdoor_air_write_performed: bool,
    /// Whether the EMS override predicate was read.
    pub ems_override_flag_read: bool,
    /// Source EMS override flag value, absent when UnitOn is false.
    pub ems_override_enabled: Option<bool>,
    /// Whether EMS replaced the local working flow.
    pub ems_override_applied: bool,
    /// Post-EMS local working flow, absent when UnitOn is false.
    pub working_outdoor_air_mass_flow_rate_kg_per_s: Option<f64>,
    /// Whether the PurchasedAir outdoor-air predicate was read.
    pub outdoor_air_flag_read: bool,
    /// Source OutdoorAir value, absent when UnitOn is false.
    pub outdoor_air_enabled: Option<bool>,
    /// Whether the no-outdoor-air zero-output branch executed.
    pub no_outdoor_air_zero_branch_entered: bool,
    /// Psychrometric calls made by this bounded route.
    pub psychrometric_call_count: usize,
    /// Minimum-OA sensible effect in W, absent when UnitOn is false.
    pub minimum_outdoor_air_sensible_output_w: Option<f64>,
    /// Minimum-OA moisture effect in kg/s, absent when UnitOn is false.
    pub minimum_outdoor_air_moisture_output_kg_per_s: Option<f64>,
}

/// Bounded per-unit state retained across prefix transitions.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcMinimumOaPrefixRuntimeState {
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP310 snapshots consumed, including UnitOff skips.
    pub transition_count: usize,
    /// Transitions that executed the source body.
    pub source_execution_count: usize,
    /// Transitions skipped by the CP310 UnitOn gate.
    pub unit_off_skip_count: usize,
    /// Zone heat-balance reference bindings.
    pub zone_heat_balance_reference_count: usize,
    /// Minimum-OA child call-site visits.
    pub minimum_oa_child_call_count: usize,
    /// Bounded no-outdoor-air child-route executions.
    pub minimum_oa_child_no_outdoor_air_count: usize,
    /// Retained pre-EMS minimum-flow writes.
    pub retained_minimum_outdoor_air_write_count: usize,
    /// EMS override predicate reads.
    pub ems_override_flag_read_count: usize,
    /// EMS local-flow overrides applied.
    pub ems_override_apply_count: usize,
    /// PurchasedAir outdoor-air predicate reads.
    pub outdoor_air_flag_read_count: usize,
    /// Active outdoor-air effect branches.
    pub outdoor_air_effect_count: usize,
    /// No-outdoor-air zero-output branches.
    pub no_outdoor_air_zero_branch_count: usize,
    /// Psychrometric calls made by this bounded route.
    pub psychrometric_call_count: usize,
    /// Latest transition snapshot; no per-timestep log is retained.
    pub latest: Option<PurchasedAirCalcMinimumOaPrefixSnapshot>,
}

impl PurchasedAirCalcMinimumOaPrefixRuntimeState {
    /// Creates bounded state for one selected system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            source_execution_count: 0,
            unit_off_skip_count: 0,
            zone_heat_balance_reference_count: 0,
            minimum_oa_child_call_count: 0,
            minimum_oa_child_no_outdoor_air_count: 0,
            retained_minimum_outdoor_air_write_count: 0,
            ems_override_flag_read_count: 0,
            ems_override_apply_count: 0,
            outdoor_air_flag_read_count: 0,
            outdoor_air_effect_count: 0,
            no_outdoor_air_zero_branch_count: 0,
            psychrometric_call_count: 0,
            latest: None,
        }
    }
}

/// Final selected-unit CP311 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcMinimumOaPrefixLifecycleSummary {
    /// Parent EnergyPlus source slice.
    pub source: &'static str,
    /// Bounded child dependency.
    pub minimum_oa_child_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcMinimumOaPrefixRuntimeState,
}

/// Fail-closed error before the bounded prefix mutates state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirCalcMinimumOaPrefixError {
    /// The selected unit is absent from the persistent arena.
    UnknownSystem {
        /// Missing typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The selected unit has not completed its bounded topology pass.
    InitializationNotReady {
        /// Unready typed system.
        system: IdealLoadsAirSystemId,
    },
    /// The model object and selected runtime unit disagree.
    SystemIdentityMismatch {
        /// Runtime-selected system.
        expected: IdealLoadsAirSystemId,
        /// Caller-supplied model system.
        actual: IdealLoadsAirSystemId,
    },
    /// The supplied CP310 snapshot is not the retained latest snapshot.
    CalculationEntrySnapshotMismatch {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
    /// CP310 and CP311 calls are not in one-for-one source order.
    CalculationEntryCallOrder {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
        /// Completed CP310 calls.
        calculation_entry_call_count: usize,
        /// Completed CP311 transitions.
        minimum_oa_prefix_transition_count: usize,
    },
    /// Outdoor air lies outside the direct no-OA release subset.
    OutdoorAirOutsideBoundedSubset {
        /// Selected typed system.
        system: IdealLoadsAirSystemId,
    },
}

/// Executes the no-OA/no-EMS CP311 route after one retained CP310 snapshot.
pub fn advance_direct_no_oa_calc_minimum_oa_prefix(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    calculation_entry: PurchasedAirCalcEntrySnapshot,
) -> Result<PurchasedAirCalcMinimumOaPrefixSnapshot, PurchasedAirCalcMinimumOaPrefixError> {
    let unit = runtime.units.get_mut(&calculation_entry.system).ok_or(
        PurchasedAirCalcMinimumOaPrefixError::UnknownSystem {
            system: calculation_entry.system,
        },
    )?;
    if !unit.topology_completed || unit.topology_failure.is_some() {
        return Err(
            PurchasedAirCalcMinimumOaPrefixError::InitializationNotReady {
                system: calculation_entry.system,
            },
        );
    }
    if system.id != calculation_entry.system {
        return Err(
            PurchasedAirCalcMinimumOaPrefixError::SystemIdentityMismatch {
                expected: calculation_entry.system,
                actual: system.id,
            },
        );
    }
    let entry_matches =
        unit.calc_entry.latest.is_some_and(|latest| {
            calculation_entry_snapshots_bitwise_equal(latest, calculation_entry)
        }) && Some(calculation_entry.controlled_zone) == unit.controlled_zone
            && calculation_entry.unit_body_entered == calculation_entry.unit_on;
    if !entry_matches {
        return Err(
            PurchasedAirCalcMinimumOaPrefixError::CalculationEntrySnapshotMismatch {
                system: calculation_entry.system,
            },
        );
    }
    if unit.calc_minimum_oa_prefix.transition_count.checked_add(1)
        != Some(unit.calc_entry.call_count)
        || calculation_entry.call_ordinal != unit.calc_entry.call_count
    {
        return Err(
            PurchasedAirCalcMinimumOaPrefixError::CalculationEntryCallOrder {
                system: calculation_entry.system,
                calculation_entry_call_count: unit.calc_entry.call_count,
                minimum_oa_prefix_transition_count: unit.calc_minimum_oa_prefix.transition_count,
            },
        );
    }
    let outdoor_air_absent = calculation_entry.outdoor_air_node.is_none()
        && system
            .design_specification_outdoor_air_object_name
            .is_none()
        && system.outdoor_air_inlet_node_name.is_none();
    if !outdoor_air_absent {
        return Err(
            PurchasedAirCalcMinimumOaPrefixError::OutdoorAirOutsideBoundedSubset {
                system: calculation_entry.system,
            },
        );
    }

    Ok(advance_minimum_oa_prefix_state(
        &mut unit.calc_entry,
        &mut unit.calc_minimum_oa_prefix,
        calculation_entry,
    ))
}

/// Returns the bounded selected-unit CP311 lifecycle summary.
pub fn purchased_air_calc_minimum_oa_prefix_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<PurchasedAirCalcMinimumOaPrefixLifecycleSummary, PurchasedAirCalcMinimumOaPrefixError> {
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcMinimumOaPrefixError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcMinimumOaPrefixLifecycleSummary {
        source: PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
        minimum_oa_child_source: PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE,
        state: unit.calc_minimum_oa_prefix.clone(),
    })
}

pub(super) fn advance_minimum_oa_prefix_state(
    calculation_entry_state: &mut PurchasedAirCalcEntryRuntimeState,
    state: &mut PurchasedAirCalcMinimumOaPrefixRuntimeState,
    calculation_entry: PurchasedAirCalcEntrySnapshot,
) -> PurchasedAirCalcMinimumOaPrefixSnapshot {
    state.transition_count += 1;
    let body_entered = calculation_entry.unit_body_entered;
    if body_entered {
        state.source_execution_count += 1;
        state.zone_heat_balance_reference_count += 1;
        state.minimum_oa_child_call_count += 1;
        state.minimum_oa_child_no_outdoor_air_count += 1;
        state.retained_minimum_outdoor_air_write_count += 1;
        state.ems_override_flag_read_count += 1;
        state.outdoor_air_flag_read_count += 1;
        state.no_outdoor_air_zero_branch_count += 1;
        calculation_entry_state.minimum_outdoor_air_mass_flow_rate_kg_per_s = 0.0;
    } else {
        state.unit_off_skip_count += 1;
    }

    let snapshot = PurchasedAirCalcMinimumOaPrefixSnapshot {
        source: PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE,
        minimum_oa_child_source: PURCHASED_AIR_CALC_MINIMUM_OA_CHILD_SOURCE,
        system: state.system,
        parent_call_ordinal: calculation_entry.call_ordinal,
        source_order: PURCHASED_AIR_CALC_MINIMUM_OA_PREFIX_SOURCE_ORDER,
        controlled_zone: calculation_entry.controlled_zone,
        unit_body_entered: body_entered,
        zone_heat_balance_reference_bound: body_entered,
        minimum_oa_child_called: body_entered,
        minimum_oa_child_no_outdoor_air_route: body_entered,
        retained_minimum_outdoor_air_mass_flow_rate_kg_per_s: body_entered.then_some(0.0),
        retained_minimum_outdoor_air_write_performed: body_entered,
        ems_override_flag_read: body_entered,
        ems_override_enabled: body_entered.then_some(false),
        ems_override_applied: false,
        working_outdoor_air_mass_flow_rate_kg_per_s: body_entered.then_some(0.0),
        outdoor_air_flag_read: body_entered,
        outdoor_air_enabled: body_entered.then_some(false),
        no_outdoor_air_zero_branch_entered: body_entered,
        psychrometric_call_count: 0,
        minimum_outdoor_air_sensible_output_w: body_entered.then_some(0.0),
        minimum_outdoor_air_moisture_output_kg_per_s: body_entered.then_some(0.0),
    };
    state.latest = Some(snapshot);
    snapshot
}

fn calculation_entry_snapshots_bitwise_equal(
    retained: PurchasedAirCalcEntrySnapshot,
    supplied: PurchasedAirCalcEntrySnapshot,
) -> bool {
    let sampled_values_match = [
        (
            retained.demand.remaining_output_req_to_heat_sp_w,
            supplied.demand.remaining_output_req_to_heat_sp_w,
        ),
        (
            retained.demand.remaining_output_req_to_cool_sp_w,
            supplied.demand.remaining_output_req_to_cool_sp_w,
        ),
        (retained.overall_availability, supplied.overall_availability),
        (retained.heating_availability, supplied.heating_availability),
        (retained.cooling_availability, supplied.cooling_availability),
    ]
    .into_iter()
    .all(|(left, right)| left.to_bits() == right.to_bits());
    if !sampled_values_match {
        return false;
    }

    let mut retained_without_floats = retained;
    let mut supplied_without_floats = supplied;
    retained_without_floats
        .demand
        .remaining_output_req_to_heat_sp_w = 0.0;
    retained_without_floats
        .demand
        .remaining_output_req_to_cool_sp_w = 0.0;
    supplied_without_floats
        .demand
        .remaining_output_req_to_heat_sp_w = 0.0;
    supplied_without_floats
        .demand
        .remaining_output_req_to_cool_sp_w = 0.0;
    retained_without_floats.overall_availability = 0.0;
    retained_without_floats.heating_availability = 0.0;
    retained_without_floats.cooling_availability = 0.0;
    supplied_without_floats.overall_availability = 0.0;
    supplied_without_floats.heating_availability = 0.0;
    supplied_without_floats.cooling_availability = 0.0;
    retained_without_floats == supplied_without_floats
}
