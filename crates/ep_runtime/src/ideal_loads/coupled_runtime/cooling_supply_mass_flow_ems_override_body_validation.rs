//! Release validation for the bounded cooling supply mass-flow EMS-override body.

use crate::ideal_loads::{
    DirectZonePurchasedAirModelBinding, DirectZonePurchasedAirScheduledCouplingOutput,
    IdealLoadsSensibleMode,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
};

use super::super::calc::{
    cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release,
};
use super::DirectZonePurchasedAirCoupledRuntimeError as Error;

pub(super) fn snapshot_matches_release(
    output: &DirectZonePurchasedAirScheduledCouplingOutput,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> bool {
    let predecessor = output.calculation_cooling_supply_mass_flow_ems_override_guard;
    let body = output.calculation_cooling_supply_mass_flow_ems_override_body;
    let numerical_cooling =
        output.coupling.purchased_air.calculation.mode == IdealLoadsSensibleMode::Cooling;

    predecessor.system == binding.ideal_loads_air_system
        && predecessor.parent_call_ordinal == call_ordinal
        && predecessor.controlled_zone == binding.zone
        && output.initialization.system == predecessor.system
        && output.initialization.controlled_zone == predecessor.controlled_zone
        && predecessor.cooling_body_entered == numerical_cooling
        && cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release(predecessor)
        && cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release(body)
        && body == expected_snapshot(predecessor, call_ordinal, binding)
}

fn expected_snapshot(
    predecessor: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
    call_ordinal: usize,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
    let cooling = predecessor.cooling_body_entered;
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
        first_excluded_source:
            PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
        system: binding.ideal_loads_air_system,
        parent_call_ordinal: call_ordinal,
        controlled_zone: binding.zone,
        unit_body_entered: predecessor.unit_body_entered,
        predecessor_cooling_body_entered: predecessor.cooling_body_entered,
        predecessor_ems_supply_mass_flow_override_body_entered: predecessor
            .ems_supply_mass_flow_override_body_entered,
        predecessor_ems_supply_mass_flow_override_guard_false_fallthrough: predecessor
            .ems_supply_mass_flow_override_guard_false_fallthrough,
        unit_off_skipped: predecessor.unit_off_skipped,
        non_cooling_skipped: predecessor.non_cooling_skipped,
        cooling_body_entered: cooling,
        body_skipped: true,
        ems_disabled_fallthrough: cooling,
        ems_supply_mass_flow_override_value_read: false,
        ems_supply_mass_flow_override_value_kg_per_s: None,
        supply_mass_flow_rate_override_assignment_performed: false,
        assigned_supply_mass_flow_rate_kg_per_s: None,
        outdoor_air_mass_flow_rate_for_minimum_read: false,
        outdoor_air_mass_flow_rate_before_override_kg_per_s: None,
        supply_mass_flow_rate_for_minimum_read: false,
        supply_mass_flow_rate_for_minimum_kg_per_s: None,
        source_shaped_two_argument_minimum_evaluated: false,
        minimum_outdoor_air_mass_flow_rate_kg_per_s: None,
        outdoor_air_mass_flow_rate_assignment_performed: false,
        assigned_outdoor_air_mass_flow_rate_kg_per_s: None,
    }
}

pub(super) fn validate_lifecycle(
    lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleSummary,
    predecessor_lifecycle: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardLifecycleSummary,
    timestep_count: usize,
    numerical_cooling_count: usize,
    latest_output: &DirectZonePurchasedAirScheduledCouplingOutput,
    binding: &DirectZonePurchasedAirModelBinding<'_>,
) -> Result<(), Error> {
    let state = &lifecycle.state;
    let predecessor = &predecessor_lifecycle.state;
    let skipped = checked_add(
        state.unit_off_skip_count,
        state.non_cooling_skip_count,
        "skip_partition_overflow",
        timestep_count,
    )?;
    let transition_partition = checked_add(
        skipped,
        state.cooling_body_entry_count,
        "transition_partition_overflow",
        timestep_count,
    )?;

    for (field, expected, actual) in [
        ("transition_count", timestep_count, state.transition_count),
        (
            "predecessor_transition_count",
            predecessor.transition_count,
            state.transition_count,
        ),
        (
            "transition_partition",
            state.transition_count,
            transition_partition,
        ),
        (
            "unit_off_skip_count",
            predecessor.unit_off_skip_count,
            state.unit_off_skip_count,
        ),
        (
            "non_cooling_skip_count",
            predecessor.non_cooling_skip_count,
            state.non_cooling_skip_count,
        ),
        (
            "cooling_body_entry_count",
            predecessor.cooling_body_entry_count,
            state.cooling_body_entry_count,
        ),
        (
            "numerical_cooling_count",
            numerical_cooling_count,
            state.cooling_body_entry_count,
        ),
        ("body_entry_count", 0, state.body_entry_count),
        (
            "body_skip_count",
            state.transition_count,
            state.body_skip_count,
        ),
        (
            "ems_disabled_fallthrough_count",
            state.cooling_body_entry_count,
            state.ems_disabled_fallthrough_count,
        ),
        (
            "ems_supply_mass_flow_override_value_read_count",
            0,
            state.ems_supply_mass_flow_override_value_read_count,
        ),
        (
            "supply_mass_flow_rate_override_assignment_count",
            0,
            state.supply_mass_flow_rate_override_assignment_count,
        ),
        (
            "outdoor_air_mass_flow_rate_for_minimum_read_count",
            0,
            state.outdoor_air_mass_flow_rate_for_minimum_read_count,
        ),
        (
            "supply_mass_flow_rate_for_minimum_read_count",
            0,
            state.supply_mass_flow_rate_for_minimum_read_count,
        ),
        (
            "source_shaped_two_argument_minimum_evaluation_count",
            0,
            state.source_shaped_two_argument_minimum_evaluation_count,
        ),
        (
            "outdoor_air_mass_flow_rate_assignment_count",
            0,
            state.outdoor_air_mass_flow_rate_assignment_count,
        ),
    ] {
        ensure_count(actual, expected, field)?;
    }

    let latest = state
        .latest
        .as_ref()
        .ok_or_else(|| violation("latest_release_snapshot_ready", 1, 0))?;
    if lifecycle.source != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE
        || lifecycle.first_excluded_source
            != PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE
        || state.system != binding.ideal_loads_air_system
        || latest != &latest_output.calculation_cooling_supply_mass_flow_ems_override_body
        || !snapshot_matches_release(latest_output, timestep_count, binding)
    {
        return Err(violation("latest_release_snapshot_ready", 1, 0));
    }
    Ok(())
}

pub(super) fn checked_add(
    left: usize,
    right: usize,
    field: &'static str,
    expected: usize,
) -> Result<usize, Error> {
    left.checked_add(right)
        .ok_or_else(|| violation(field, expected, usize::MAX))
}

fn ensure_count(actual: usize, expected: usize, field: &'static str) -> Result<(), Error> {
    if actual == expected {
        Ok(())
    } else {
        Err(violation(field, expected, actual))
    }
}

fn violation(field: &'static str, expected: usize, actual: usize) -> Error {
    Error::CalcCoolingSupplyMassFlowEmsOverrideBodyLifecycleInvariant {
        field,
        expected,
        actual,
    }
}
