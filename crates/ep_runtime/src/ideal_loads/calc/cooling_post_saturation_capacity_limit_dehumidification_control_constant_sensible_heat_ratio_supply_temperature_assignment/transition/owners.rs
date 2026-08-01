//! Exact retained CP379 and active CP329/CP330/CP387 owner validation for CP389.

use super::routes::{RetainedRoute, predecessor_has_supply_temperature};
use super::{ActiveOwners, RetainedInput};
use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshots_match_bit_exact,
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact,
    cooling_supply_mass_flow_positive_guard::positive_guard_links_to_mixed_air_call,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioCpAirAssignmentSnapshot as CpAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSensibleHeatRatioSensibleOutputAssignmentSnapshot as Predecessor,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as TemperatureOwner,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
};

#[derive(Clone, Copy)]
pub(super) struct PreparedInput {
    pub preexisting_supply_temperature_c: Option<f64>,
    pub active: Option<PreparedActive>,
}

#[derive(Clone, Copy)]
pub(super) struct PreparedActive {
    pub mixed_air_temperature_c: f64,
    pub cooling_sensible_output_w: f64,
    pub cp_air_j_per_kg_k: f64,
    pub supply_mass_flow_rate_kg_per_s: f64,
}

pub(super) fn prepare_exact_input(
    predecessor: Predecessor,
    route: RetainedRoute,
    input: RetainedInput,
) -> Option<PreparedInput> {
    let temperature =
        exact_preexisting_temperature(predecessor, route, input.cp379_temperature_owner)?;
    let active = match (route.active, input.active_owners) {
        (false, None) => None,
        (true, Some(owners)) => Some(exact_active_values(predecessor, owners)?),
        _ => return None,
    };
    Some(PreparedInput {
        preexisting_supply_temperature_c: temperature,
        active,
    })
}

fn exact_preexisting_temperature(
    predecessor: Predecessor,
    route: RetainedRoute,
    owner: TemperatureOwner,
) -> Option<Option<f64>> {
    if !cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact(owner)
        || !same_identity(
            predecessor,
            owner.system,
            owner.parent_call_ordinal,
            owner.controlled_zone,
        )
        || predecessor_prefix_flags(predecessor) != temperature_prefix_flags(owner)
        || (predecessor.predecessor_dehumidification_control_type_read
            && predecessor.predecessor_dehumidification_control_type
                != owner.predecessor_dehumidification_control_type)
    {
        return None;
    }
    let has_temperature = predecessor_has_supply_temperature(route.predecessor_index);
    let transitive_owner_count =
        usize::from(owner.cp334_supply_temperature_mixed_air_limit_owned_read)
            + usize::from(owner.cp344_capacity_limit_supply_temperature_mixed_air_limit_owned_read);
    if owner.cp377_supply_temperature_owned_read != has_temperature
        || owner.purchased_air_supply_temperature_for_post_saturation_enthalpy_read
            != has_temperature
        || transitive_owner_count != usize::from(has_temperature)
        || owner.supply_temperature_c.is_some() != has_temperature
    {
        return None;
    }
    Some(owner.supply_temperature_c)
}

fn exact_active_values(predecessor: Predecessor, owners: ActiveOwners) -> Option<PreparedActive> {
    let mixed = owners.mixed_air_owner;
    let flow = owners.supply_mass_flow_owner;
    let cp_air = owners.cp_air_owner;
    if !cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed)
        || !cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(flow)
        || !cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshot_is_exact(cp_air)
        || !same_identity(predecessor, mixed.system, mixed.parent_call_ordinal, mixed.controlled_zone)
        || !same_identity(predecessor, flow.system, flow.parent_call_ordinal, flow.controlled_zone)
        || !positive_guard_links_to_mixed_air_call(flow, mixed)
        || !cp_air_matches_predecessor(predecessor, cp_air)
        || !mixed.mixed_air_temperature_assigned
        || !flow.positive_supply_mass_flow_body_entered
        || !flow.supply_mass_flow_rate_read
        || flow.supply_mass_flow_rate_strictly_positive != Some(true)
    {
        return None;
    }
    let mixed_air_temperature_c = mixed.mixed_air_temperature_c?;
    let supply_mass_flow_rate_kg_per_s = flow.supply_mass_flow_rate_kg_per_s?;
    if mixed.supply_mass_flow_rate_kg_per_s?.to_bits() != supply_mass_flow_rate_kg_per_s.to_bits()
        || mixed.child_supply_mass_flow_rate_kg_per_s?.to_bits()
            != supply_mass_flow_rate_kg_per_s.to_bits()
    {
        return None;
    }
    Some(PreparedActive {
        mixed_air_temperature_c,
        cooling_sensible_output_w: predecessor.cooling_sensible_output_w?,
        cp_air_j_per_kg_k: cp_air.cp_air_j_per_kg_k?,
        supply_mass_flow_rate_kg_per_s,
    })
}

fn cp_air_matches_predecessor(predecessor: Predecessor, owner: CpAirOwner) -> bool {
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_snapshots_match_bit_exact(
        owner,
        cp_air_from_predecessor(predecessor),
    )
}

fn cp_air_from_predecessor(snapshot: Predecessor) -> CpAirOwner {
    CpAirOwner {
        source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_POST_SATURATION_CAPACITY_LIMIT_DEHUMIDIFICATION_CONTROL_CONSTANT_SENSIBLE_HEAT_RATIO_CP_AIR_ASSIGNMENT_SOURCE_ORDER,
        system: snapshot.system,
        parent_call_ordinal: snapshot.parent_call_ordinal,
        controlled_zone: snapshot.controlled_zone,
        unit_off_skipped: snapshot.unit_off_skipped,
        non_cooling_skipped: snapshot.non_cooling_skipped,
        positive_guard_false_fallthrough_skipped: snapshot.positive_guard_false_fallthrough_skipped,
        heating_availability_guard_false_fallthrough: snapshot.heating_availability_guard_false_fallthrough,
        humidification_control_guard_false_fallthrough: snapshot.humidification_control_guard_false_fallthrough,
        dehumidification_control_humidistat_maximum_assignment_executed: snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        dehumidification_control_none_maximum_assignment_executed: snapshot.dehumidification_control_none_maximum_assignment_executed,
        dehumidification_control_guard_false_fallthrough: snapshot.dehumidification_control_guard_false_fallthrough,
        predecessor_capacity_limit_guard_evaluated: snapshot.predecessor_capacity_limit_guard_evaluated,
        predecessor_capacity_limit_body_entered: snapshot.predecessor_capacity_limit_body_entered,
        predecessor_active_capacity_limit_guard_false_fallthrough: snapshot.predecessor_active_capacity_limit_guard_false_fallthrough,
        predecessor_dehumidification_guard_evaluated: snapshot.predecessor_dehumidification_guard_evaluated,
        predecessor_dehumidification_body_entered: snapshot.predecessor_dehumidification_body_entered,
        predecessor_dehumidification_guard_false_fallthrough: snapshot.predecessor_dehumidification_guard_false_fallthrough,
        predecessor_dehumidification_total_output_assignment_executed: snapshot.predecessor_dehumidification_total_output_assignment_executed,
        predecessor_dehumidification_total_output_capacity_guard_evaluated: snapshot.predecessor_dehumidification_total_output_capacity_guard_evaluated,
        predecessor_dehumidification_total_output_capacity_adjustment_body_entered: snapshot.predecessor_dehumidification_total_output_capacity_adjustment_body_entered,
        predecessor_dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.predecessor_dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_capacity_guard_false_fallthrough: snapshot.dehumidification_total_output_capacity_guard_false_fallthrough,
        dehumidification_total_output_maximum_capacity_assignment_executed: snapshot.dehumidification_total_output_maximum_capacity_assignment_executed,
        predecessor_supply_enthalpy_assignment_executed: snapshot.predecessor_supply_enthalpy_assignment_executed,
        predecessor_dehumidification_control_type_read: snapshot.predecessor_dehumidification_control_type_read,
        predecessor_dehumidification_control_type: snapshot.predecessor_dehumidification_control_type,
        predecessor_dehumidification_control_switch_dispatched: snapshot.predecessor_dehumidification_control_switch_dispatched,
        predecessor_resulting_supply_enthalpy_j_per_kg: snapshot.resulting_supply_enthalpy_j_per_kg,
        dehumidification_control_constant_sensible_heat_ratio_case_entered: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_entered,
        dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed: snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_cp_air_assignment_executed,
        mixed_air_humidity_ratio_read: snapshot.predecessor_mixed_air_humidity_ratio_read,
        mixed_air_humidity_ratio: snapshot.predecessor_mixed_air_humidity_ratio,
        psychrometric_cp_air_evaluated: snapshot.predecessor_psychrometric_cp_air_evaluated,
        psychrometric_cp_air_result_j_per_kg_k: snapshot.predecessor_psychrometric_cp_air_result_j_per_kg_k,
        cp_air_assigned: snapshot.predecessor_cp_air_assigned,
        cp_air_j_per_kg_k: snapshot.predecessor_cp_air_j_per_kg_k,
        resulting_supply_enthalpy_j_per_kg: snapshot.resulting_supply_enthalpy_j_per_kg,
    }
}

fn same_identity(
    predecessor: Predecessor,
    system: ep_model::IdealLoadsAirSystemId,
    ordinal: usize,
    zone: ep_model::ZoneId,
) -> bool {
    predecessor.system == system
        && predecessor.parent_call_ordinal == ordinal
        && predecessor.controlled_zone == zone
}

fn predecessor_prefix_flags(snapshot: Predecessor) -> [bool; 8] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ]
}

fn temperature_prefix_flags(snapshot: TemperatureOwner) -> [bool; 8] {
    [
        snapshot.unit_off_skipped,
        snapshot.non_cooling_skipped,
        snapshot.positive_guard_false_fallthrough_skipped,
        snapshot.heating_availability_guard_false_fallthrough,
        snapshot.humidification_control_guard_false_fallthrough,
        snapshot.dehumidification_control_humidistat_maximum_assignment_executed,
        snapshot.dehumidification_control_none_maximum_assignment_executed,
        snapshot.dehumidification_control_guard_false_fallthrough,
    ]
}
