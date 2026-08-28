//! Bounded `CalcPurchAirLoads` cooling-entry gate lifecycle.

use ep_model::{IdealLoadsAirSystemId, ZoneId};

use super::super::PurchasedAirRuntimeState;
use super::lifecycle::PurchasedAirCalcEntrySnapshot;
use super::minimum_oa_prefix::PurchasedAirCalcMinimumOaPrefixSnapshot;
use super::types::IdealLoadsSensibleMode;

mod release;

pub(in crate::ideal_loads) use release::PurchasedAirCalcCoolingEntryGateCommittedHeatingModeGuardNumericOperands;
pub use release::{
    PurchasedAirCalcCoolingEntryGateError, PurchasedAirCalcCoolingEntryGatePredicateInput,
    advance_direct_no_oa_calc_cooling_entry_gate,
};
pub(in crate::ideal_loads::calc) use release::{
    cooling_entry_gate_committed_latest_heating_mode_guard_numeric_operands,
    cooling_entry_gate_committed_latest_heating_mode_guard_temperature_control_type,
};

/// EnergyPlus source slice represented by this bounded transition.
pub const PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2046-2047";

/// Lexically first executable statement deliberately left for the next slice.
pub const PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE: &str =
    "EnergyPlus 26.1 PurchasedAirManager.cc:2056";

/// Exact source-order sites represented by the bounded gate.
pub const PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER: &[&str] = &[
    "read-minimum-outdoor-air-sensible-output",
    "read-cooling-setpoint-demand",
    "compare-inclusive-greater-equal",
    "read-zone-temperature-control-type-after-short-circuit",
    "exclude-exact-single-heating-control",
    "assign-cooling-operating-mode-if-admitted",
];

/// EnergyPlus `HVAC::SetptType` values visible to the line-2046 predicate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PurchasedAirTemperatureControlType {
    /// Invalid source sentinel.
    Invalid,
    /// No active thermostat control.
    Uncontrolled,
    /// Heating-only control; the cooling gate explicitly rejects this value.
    SingleHeat,
    /// Cooling-only control.
    SingleCool,
    /// One shared heating/cooling setpoint.
    SingleHeatCool,
    /// Dual setpoint with deadband.
    DualHeatCool,
}

/// Opaque proof of one successful direct CP312 invocation. This deliberately
/// stays out of the public snapshot and serializer schemas.
#[derive(Clone, Copy, Debug, PartialEq)]
struct PurchasedAirCalcCoolingEntryGateDirectReleaseInvocationWitness {
    calculation_entry: PurchasedAirCalcEntrySnapshot,
    minimum_oa_prefix: PurchasedAirCalcMinimumOaPrefixSnapshot,
    cooling_entry_gate: PurchasedAirCalcCoolingEntryGateSnapshot,
    prevalidated_temperature_control_type: PurchasedAirTemperatureControlType,
}

/// One CP311-to-CP312 cooling-entry transition result.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEntryGateSnapshot {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement not represented by this slice.
    pub first_excluded_source: &'static str,
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP311 parent call ordinal consumed by this transition.
    pub parent_call_ordinal: usize,
    /// Source-order sites represented by this bounded route.
    pub source_order: &'static [&'static str],
    /// Controlled Zone used by the temperature-control lookup.
    pub controlled_zone: ZoneId,
    /// Whether the enclosing CP310 `UnitOn` body was entered.
    pub unit_body_entered: bool,
    /// CP311 minimum-OA sensible output, absent when the body was skipped.
    pub minimum_outdoor_air_sensible_output_w: Option<f64>,
    /// CP310 cooling-setpoint demand, absent when the body was skipped.
    pub cooling_setpoint_demand_w: Option<f64>,
    /// Whether the first line-2046 comparison executed.
    pub sensible_comparison_evaluated: bool,
    /// Result of `MinOASensOutput >= QZnCoolSP`, absent when skipped.
    pub sensible_comparison_satisfied: Option<bool>,
    /// Whether short-circuit evaluation reached the thermostat read.
    pub temperature_control_type_read: bool,
    /// Read thermostat value, absent when the read site was skipped.
    pub temperature_control_type: Option<PurchasedAirTemperatureControlType>,
    /// Whether the read thermostat admitted cooling, absent when not read.
    pub temperature_control_type_permits_cooling: Option<bool>,
    /// Whether `SingleHeat` alone blocked an otherwise satisfied numeric gate.
    pub single_heat_blocked: bool,
    /// Whether execution entered the cooling body after line 2047.
    pub cooling_body_entered: bool,
    /// Local `OperatingMode` assignment made by this slice.
    pub assigned_operating_mode: Option<IdealLoadsSensibleMode>,
}

/// Persistent bounded state for one system's cooling-entry transitions.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEntryGateRuntimeState {
    /// Selected IdealLoads system.
    pub system: IdealLoadsAirSystemId,
    /// CP311 snapshots consumed, including UnitOff skips.
    pub transition_count: usize,
    /// Transitions that executed the line-2046 comparison.
    pub source_execution_count: usize,
    /// Transitions skipped by the enclosing `UnitOn` gate.
    pub unit_off_skip_count: usize,
    /// Numeric sensible comparisons executed.
    pub sensible_comparison_count: usize,
    /// Numeric comparisons that satisfied `>=`.
    pub sensible_comparison_satisfied_count: usize,
    /// Temperature-control read sites reached after short-circuit evaluation.
    pub temperature_control_type_read_count: usize,
    /// Satisfied numeric gates blocked only by `SingleHeat`.
    pub single_heat_block_count: usize,
    /// Cooling-body entries.
    pub cooling_body_entry_count: usize,
    /// Local `OperatingMode::Cool` assignments.
    pub operating_mode_assignment_count: usize,
    /// Active-body transitions that did not enter cooling.
    pub active_fallthrough_count: usize,
    /// Latest transition snapshot; no timestep log is retained.
    pub latest: Option<PurchasedAirCalcCoolingEntryGateSnapshot>,
    direct_release_invocation_witness:
        Option<PurchasedAirCalcCoolingEntryGateDirectReleaseInvocationWitness>,
}

impl PurchasedAirCalcCoolingEntryGateRuntimeState {
    /// Creates bounded state for one selected system.
    #[must_use]
    pub const fn new(system: IdealLoadsAirSystemId) -> Self {
        Self {
            system,
            transition_count: 0,
            source_execution_count: 0,
            unit_off_skip_count: 0,
            sensible_comparison_count: 0,
            sensible_comparison_satisfied_count: 0,
            temperature_control_type_read_count: 0,
            single_heat_block_count: 0,
            cooling_body_entry_count: 0,
            operating_mode_assignment_count: 0,
            active_fallthrough_count: 0,
            latest: None,
            direct_release_invocation_witness: None,
        }
    }
}

/// Final selected-unit CP312 lifecycle summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PurchasedAirCalcCoolingEntryGateLifecycleSummary {
    /// EnergyPlus source slice.
    pub source: &'static str,
    /// First executable source statement deliberately excluded.
    pub first_excluded_source: &'static str,
    /// Final bounded per-unit state.
    pub state: PurchasedAirCalcCoolingEntryGateRuntimeState,
}

/// Returns the bounded selected-unit CP312 lifecycle summary.
pub fn purchased_air_calc_cooling_entry_gate_lifecycle_summary(
    runtime: &PurchasedAirRuntimeState,
    system: IdealLoadsAirSystemId,
) -> Result<PurchasedAirCalcCoolingEntryGateLifecycleSummary, PurchasedAirCalcCoolingEntryGateError>
{
    let unit = runtime
        .units
        .get(&system)
        .ok_or(PurchasedAirCalcCoolingEntryGateError::UnknownSystem { system })?;
    Ok(PurchasedAirCalcCoolingEntryGateLifecycleSummary {
        source: PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
        state: unit.calc_cooling_entry_gate.clone(),
    })
}

pub(super) fn advance_cooling_entry_gate_state(
    state: &mut PurchasedAirCalcCoolingEntryGateRuntimeState,
    calculation_entry: PurchasedAirCalcEntrySnapshot,
    minimum_oa_prefix: PurchasedAirCalcMinimumOaPrefixSnapshot,
    temperature_control_type: PurchasedAirTemperatureControlType,
) -> PurchasedAirCalcCoolingEntryGateSnapshot {
    state.transition_count += 1;
    let body_entered = calculation_entry.unit_body_entered;
    let (minimum_oa_sensible_output_w, cooling_setpoint_demand_w, sensible_comparison_satisfied) =
        if body_entered {
            let minimum_oa_sensible_output_w =
                minimum_oa_prefix.minimum_outdoor_air_sensible_output_w;
            let cooling_setpoint_demand_w =
                calculation_entry.demand.remaining_output_req_to_cool_sp_w;
            let sensible_comparison_satisfied =
                minimum_oa_sensible_output_w.is_some_and(|minimum_oa_sensible_output_w| {
                    minimum_oa_sensible_output_w >= cooling_setpoint_demand_w
                });
            (
                minimum_oa_sensible_output_w,
                Some(cooling_setpoint_demand_w),
                Some(sensible_comparison_satisfied),
            )
        } else {
            (None, None, None)
        };
    let temperature_control_type_read = sensible_comparison_satisfied == Some(true);
    let temperature_control_type_value =
        temperature_control_type_read.then_some(temperature_control_type);
    let temperature_control_type_permits_cooling = temperature_control_type_value
        .map(|value| value != PurchasedAirTemperatureControlType::SingleHeat);
    let single_heat_blocked = temperature_control_type_permits_cooling == Some(false);
    let cooling_body_entered = temperature_control_type_permits_cooling == Some(true);

    if body_entered {
        state.source_execution_count += 1;
        state.sensible_comparison_count += 1;
        if sensible_comparison_satisfied == Some(true) {
            state.sensible_comparison_satisfied_count += 1;
            state.temperature_control_type_read_count += 1;
        }
        if single_heat_blocked {
            state.single_heat_block_count += 1;
        }
        if cooling_body_entered {
            state.cooling_body_entry_count += 1;
            state.operating_mode_assignment_count += 1;
        } else {
            state.active_fallthrough_count += 1;
        }
    } else {
        state.unit_off_skip_count += 1;
    }

    let snapshot = PurchasedAirCalcCoolingEntryGateSnapshot {
        source: PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_FIRST_EXCLUDED_SOURCE,
        system: state.system,
        parent_call_ordinal: minimum_oa_prefix.parent_call_ordinal,
        source_order: PURCHASED_AIR_CALC_COOLING_ENTRY_GATE_SOURCE_ORDER,
        controlled_zone: calculation_entry.controlled_zone,
        unit_body_entered: body_entered,
        minimum_outdoor_air_sensible_output_w: minimum_oa_sensible_output_w,
        cooling_setpoint_demand_w,
        sensible_comparison_evaluated: body_entered,
        sensible_comparison_satisfied,
        temperature_control_type_read,
        temperature_control_type: temperature_control_type_value,
        temperature_control_type_permits_cooling,
        single_heat_blocked,
        cooling_body_entered,
        assigned_operating_mode: cooling_body_entered.then_some(IdealLoadsSensibleMode::Cooling),
    };
    state.latest = Some(snapshot);
    snapshot
}

pub(super) fn cooling_entry_gate_snapshots_bitwise_equal(
    retained: PurchasedAirCalcCoolingEntryGateSnapshot,
    supplied: PurchasedAirCalcCoolingEntryGateSnapshot,
) -> bool {
    let floats_match = [
        (
            retained.minimum_outdoor_air_sensible_output_w,
            supplied.minimum_outdoor_air_sensible_output_w,
        ),
        (
            retained.cooling_setpoint_demand_w,
            supplied.cooling_setpoint_demand_w,
        ),
    ]
    .into_iter()
    .all(|(left, right)| option_f64_bits_equal(left, right));
    if !floats_match {
        return false;
    }
    let mut retained_without_floats = retained;
    let mut supplied_without_floats = supplied;
    retained_without_floats.minimum_outdoor_air_sensible_output_w = None;
    retained_without_floats.cooling_setpoint_demand_w = None;
    supplied_without_floats.minimum_outdoor_air_sensible_output_w = None;
    supplied_without_floats.cooling_setpoint_demand_w = None;
    retained_without_floats == supplied_without_floats
}

fn option_f64_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
