//! Canonical private-active CP353 reconstruction.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem};

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRetainedRoute as Route,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitRuntimeState as State,
    PurchasedAirCalcCoolingPositiveSupplyPostCapacityLimitDehumidificationControlConstantSensibleHeatRatioOverdryingLimitSnapshot as Snapshot,
    advance_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_state as advance,
};
use super::prefix_validation::active_operands_from_retained_owners;
use super::snapshot_validation::{snapshot_route, snapshots_match_bit_exact};
use crate::ideal_loads::{PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState};
use crate::psychrometrics::energyplus_psy_cp_air_fn_w;

/// Rebuilds the exact private constant-SHR CP353 counterfactual from retained,
/// same-call canonical owners while keeping the retained direct release
/// authoritative.
pub(in crate::ideal_loads::calc) fn private_active_counterfactual_from_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
) -> Option<Snapshot> {
    let retained = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit
        .latest?;
    let witness = runtime
        .cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_latest_witness(
            system.id,
        )?;
    if system.id != direct.system
        || unit.system != system.id
        || snapshot_route(direct) != Some(Route::DehumidificationControlNoneCaseCompletedSkip)
        || !snapshots_match_bit_exact(retained, direct)
        || !snapshots_match_bit_exact(witness, direct)
        || !super::completed_direct_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_is_consistent(
            runtime,
            unit,
            system,
            direct,
            Some(witness),
        )
    {
        return None;
    }

    let mut private_cp352 = unit
        .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment
        .latest?;
    let flow = unit
        .calc_cooling_supply_mass_flow_positive_guard
        .latest?
        .supply_mass_flow_rate_kg_per_s?;
    let mixed_owner = unit.calc_cooling_mixed_air_call.latest?;
    let humidity = mixed_owner.mixed_air_humidity_ratio?;
    let mixed_temperature = mixed_owner.mixed_air_temperature_c?;
    let mixed_enthalpy = mixed_owner.mixed_air_enthalpy_projection_j_per_kg?;
    let provenance = unit
        .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
        .latest?;
    let supply_temperature = if provenance
        .capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed
    {
        unit.calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
            .latest?
            .resulting_supply_temperature_c?
    } else {
        unit.calc_cooling_positive_supply_temperature_mixed_air_limit
            .latest?
            .assigned_supply_temperature_c?
    };
    let cp_air = energyplus_psy_cp_air_fn_w(humidity);
    let sensible = (flow * cp_air) * (mixed_temperature - supply_temperature);
    let total = sensible / system.cooling_sensible_heat_ratio;
    let specific = total / flow;
    let supply_enthalpy = mixed_enthalpy - specific;

    private_cp352.predecessor_dehumidification_control_type =
        Some(DehumidificationControlType::ConstantSensibleHeatRatio);
    private_cp352.predecessor_dehumidification_control_none_case_completed_skip = false;
    private_cp352
        .predecessor_dehumidification_control_constant_sensible_heat_ratio_total_output_assignment_executed =
        true;
    private_cp352.dehumidification_control_none_case_completed_skip = false;
    private_cp352
        .dehumidification_control_constant_sensible_heat_ratio_supply_enthalpy_assignment_executed =
        true;
    private_cp352.mixed_air_enthalpy_read = true;
    private_cp352.mixed_air_enthalpy_j_per_kg = Some(mixed_enthalpy);
    private_cp352.cooling_total_output_read = true;
    private_cp352.cooling_total_output_w = Some(total);
    private_cp352.supply_mass_flow_rate_read = true;
    private_cp352.supply_mass_flow_rate_kg_per_s = Some(flow);
    private_cp352.specific_cooling_output_calculated = true;
    private_cp352.specific_cooling_output_j_per_kg = Some(specific);
    private_cp352.supply_enthalpy_calculated = true;
    private_cp352.calculated_supply_enthalpy_j_per_kg = Some(supply_enthalpy);
    private_cp352.supply_enthalpy_assigned = true;
    private_cp352.assigned_supply_enthalpy_j_per_kg = Some(supply_enthalpy);
    private_cp352.resulting_supply_enthalpy_j_per_kg = Some(supply_enthalpy);

    let operands = active_operands_from_retained_owners(runtime, unit, system, private_cp352)?;
    let mut state = State::new(system.id);
    let counterfactual = advance(&mut state, private_cp352, Some(operands))?;
    route_independent_identity_matches(direct, counterfactual).then_some(counterfactual)
}

/// Proves a supplied CP353 witness is the exact canonical private-active
/// counterfactual of the retained direct `None` release.
pub(in crate::ideal_loads::calc) fn private_active_counterfactual_links_to_direct_release(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    direct: Snapshot,
    counterfactual: Snapshot,
) -> bool {
    private_active_counterfactual_from_direct_release(runtime, unit, system, direct)
        .is_some_and(|expected| snapshots_match_bit_exact(expected, counterfactual))
}

fn route_independent_identity_matches(direct: Snapshot, counterfactual: Snapshot) -> bool {
    direct.system == counterfactual.system
        && direct.parent_call_ordinal == counterfactual.parent_call_ordinal
        && direct.controlled_zone == counterfactual.controlled_zone
        && direct.unit_body_entered == counterfactual.unit_body_entered
        && direct.predecessor_cooling_body_entered
            == counterfactual.predecessor_cooling_body_entered
        && direct.predecessor_no_outdoor_air_fallback_entered
            == counterfactual.predecessor_no_outdoor_air_fallback_entered
        && direct.predecessor_positive_supply_mass_flow_body_entered
            == counterfactual.predecessor_positive_supply_mass_flow_body_entered
        && direct.unit_off_skipped == counterfactual.unit_off_skipped
        && direct.non_cooling_skipped == counterfactual.non_cooling_skipped
        && direct.positive_guard_false_fallthrough_skipped
            == counterfactual.positive_guard_false_fallthrough_skipped
}
