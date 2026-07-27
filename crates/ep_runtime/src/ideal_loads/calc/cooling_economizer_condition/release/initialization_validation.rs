//! Exact retained initialization prerequisites for the CP316 release.

use ep_model::{AutosizeOrNumber, IdealLoadsAirSystem, IdealLoadsLimit};

use crate::ideal_loads::{
    PurchasedAirHardSizeLegacyRoute, PurchasedAirInitTopologyOutcome,
    PurchasedAirRecirculationSource, PurchasedAirRuntimeState, PurchasedAirSizedLimits,
    PurchasedAirUnitRuntimeState,
};

pub(super) fn initialization_state_is_exact_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
) -> bool {
    let Some(topology_plan) = unit.topology_plan.as_ref() else {
        return false;
    };
    let Some(controlled_zone) = unit.controlled_zone else {
        return false;
    };
    let Some(equipment_list) = unit.equipment_list else {
        return false;
    };
    let Some(supply_node) = unit.supply_node else {
        return false;
    };
    let Some(recirculation_node) = unit.recirculation_node else {
        return false;
    };
    let topology_outcome = topology_plan.resolve();
    let topology_ready = topology_plan.system() == system.id
        && topology_plan.controlled_zone() == controlled_zone
        && topology_plan.equipment_list() == equipment_list
        && topology_plan.supply_node() == supply_node
        && !topology_plan.outdoor_air_resolved()
        && topology_outcome
            == Ok(PurchasedAirInitTopologyOutcome {
                recirculation_node: Some(recirculation_node),
                recirculation_source: PurchasedAirRecirculationSource::SingleZoneReturn,
                rejected_exhaust_node: None,
                reported_first_return_node: None,
            })
        && unit.recirculation_source == Some(PurchasedAirRecirculationSource::SingleZoneReturn)
        && unit.rejected_exhaust_node.is_none()
        && unit.reported_first_return_node.is_none()
        && unit.topology_diagnostics.is_empty()
        && unit.topology_failure.is_none()
        && unit.planned_first_matching_equipment_list == Some(equipment_list)
        && unit.first_matching_equipment_list == Some(equipment_list)
        && unit.equipment_list_scan_ordinal == Some(1)
        && unit.equipment_list_membership_found == Some(true);
    let flags = unit.flags(runtime.equipment_list_checked);
    let flags_ready = flags.state_machine_used
        && flags.one_time_checked
        && flags.topology_ready
        && flags.environment_initialized
        && flags.sizing_checked
        && flags.equipment_list_checked
        && flags.return_plenum_inactive
        && flags.environment_initialization_needed == (unit.init_call_count > 1);
    let counts_ready = runtime.module_initialized
        && runtime.equipment_list_checked
        && unit.one_time_initialization_count == 1
        && unit.topology_completion_count == 1
        && unit.sizing_attempt_count == 1
        && unit.sizing_check_count == 1
        && unit.environment_initialization_count == 1
        && unit.environment_rearm_count == usize::from(unit.init_call_count > 1);
    let expected_sized_limits = PurchasedAirSizedLimits::from_system(system);
    let sizing_ready = !unit.sizing_needed
        && unit.sized_limits == Some(expected_sized_limits)
        && unit.sizing_outcome.is_some_and(|outcome| {
            outcome.route == PurchasedAirHardSizeLegacyRoute::DirectHardSizedNoSizingRun
                && outcome.sized_limits == expected_sized_limits
                && outcome.entry_fan_flags_cleared
                && outcome.fields.iter().all(Option::is_some)
        });
    let caches_ready = unit.standard_air_density_kg_per_m3.is_some_and(|density| {
        density.is_finite()
            && density > 0.0
            && initialized_mass_flow_has_expected_bits(
                system.heating_limit,
                expected_sized_limits.maximum_heating_air_flow_rate_m3_per_s,
                density,
                unit.maximum_heating_air_mass_flow_rate_kg_per_s,
            )
            && initialized_mass_flow_has_expected_bits(
                system.cooling_limit,
                expected_sized_limits.maximum_cooling_air_flow_rate_m3_per_s,
                density,
                unit.maximum_cooling_air_mass_flow_rate_kg_per_s,
            )
    });

    topology_ready && flags_ready && counts_ready && sizing_ready && caches_ready
}

fn initialized_mass_flow_has_expected_bits(
    limit: IdealLoadsLimit,
    volume_flow: Option<AutosizeOrNumber>,
    density: f64,
    actual_mass_flow: f64,
) -> bool {
    let expected_mass_flow = if matches!(
        limit,
        IdealLoadsLimit::LimitFlowRate | IdealLoadsLimit::LimitFlowRateAndCapacity
    ) {
        let Some(AutosizeOrNumber::Value(volume_flow)) = volume_flow else {
            return false;
        };
        if !volume_flow.is_finite() || volume_flow < 0.0 {
            return false;
        }
        volume_flow * density
    } else {
        0.0
    };
    expected_mass_flow.is_finite() && actual_mass_flow.to_bits() == expected_mass_flow.to_bits()
}
