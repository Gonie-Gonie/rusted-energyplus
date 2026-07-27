//! Persistent selected-unit state changes for the one-time topology block.

use ep_model::IdealLoadsAirSystem;

use super::{
    PurchasedAirInitError, PurchasedAirInitTopologyError, PurchasedAirInitTopologyPlan,
    PurchasedAirInitTransition, PurchasedAirUnitRuntimeState,
};

pub(super) fn advance_selected_unit_topology(
    unit: &mut PurchasedAirUnitRuntimeState,
    plan: &PurchasedAirInitTopologyPlan,
    system: &IdealLoadsAirSystem,
    transition: &mut PurchasedAirInitTransition,
) -> Result<(), PurchasedAirInitError> {
    if unit.one_time_latched && unit.topology_plan.as_ref() != Some(plan) {
        return Err(PurchasedAirInitError::LatchedTopologyChanged { system: system.id });
    }
    unit.init_call_count += 1;
    if unit.one_time_latched {
        // EnergyPlus Fatal paths do not return. Rust errors can be retried, so
        // retain a fail-closed poison result while still suppressing replay of
        // the source one-time validation and diagnostics.
        if let Some(failure) = unit.topology_failure {
            return Err(PurchasedAirInitError::Topology(failure));
        }
        return Ok(());
    }

    // EnergyPlus commits this latch before supply/exhaust/return validation.
    unit.one_time_latched = true;
    unit.one_time_initialization_count += 1;
    unit.topology_plan = Some(plan.clone());
    unit.controlled_zone = Some(plan.controlled_zone());
    unit.equipment_list = Some(plan.equipment_list());
    unit.supply_node = Some(plan.supply_node());
    transition.one_time_initialized = true;

    let evaluation = plan.evaluate(system);
    transition.topology_diagnostics_emitted = evaluation.diagnostics.len();
    transition.economizer_flow_limit_warning = evaluation.economizer_flow_limit_warning;
    unit.topology_diagnostics = evaluation.diagnostics;
    if evaluation.economizer_flow_limit_warning {
        unit.economizer_flow_limit_warning_count += 1;
    }

    match evaluation.outcome {
        Ok(outcome) => {
            unit.recirculation_node = outcome.recirculation_node;
            unit.recirculation_source = Some(outcome.recirculation_source);
            unit.rejected_exhaust_node = outcome.rejected_exhaust_node;
            unit.reported_first_return_node = outcome.reported_first_return_node;
            unit.topology_completed = true;
            unit.topology_completion_count += 1;
            transition.topology_completed = true;
            Ok(())
        }
        Err(failure) => {
            retain_failure_prefix(unit, failure);
            Err(PurchasedAirInitError::Topology(failure))
        }
    }
}

fn retain_failure_prefix(
    unit: &mut PurchasedAirUnitRuntimeState,
    failure: PurchasedAirInitTopologyError,
) {
    if let PurchasedAirInitTopologyError::NoRecirculationNode {
        rejected_exhaust_node,
        ..
    } = failure
    {
        unit.rejected_exhaust_node = rejected_exhaust_node;
    }
    unit.topology_failure = Some(failure);
}
