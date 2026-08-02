//! Exact CP399/CP329/CP330 operand-owner validation for CP400.

use super::ActiveOwners;
use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature,
};
use crate::ideal_loads::calc::cooling_supply_mass_flow_positive_guard::positive_guard_links_to_mixed_air_call;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioCpAirAssignmentSnapshot as Predecessor,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
};

#[derive(Clone, Copy)]
pub(super) struct PreparedInput {
    pub predecessor_supply_humidity_ratio: Option<f64>,
    pub predecessor_supply_enthalpy_j_per_kg: Option<f64>,
    pub predecessor_supply_temperature_c: Option<f64>,
    pub active: Option<PreparedActive>,
}

#[derive(Clone, Copy)]
pub(super) struct PreparedActive {
    pub supply_mass_flow_rate_kg_per_s: f64,
    pub cp_air_j_per_kg_k: f64,
    pub mixed_air_temperature_c: f64,
    pub supply_temperature_c: f64,
}

pub(super) fn prepare_exact_input(
    predecessor: Predecessor,
    route: RetainedRoute,
    active_owners: Option<ActiveOwners>,
) -> Option<PreparedInput> {
    let index = route.predecessor_index;
    let predecessor_supply_humidity_ratio = predecessor.resulting_supply_humidity_ratio;
    let predecessor_supply_enthalpy_j_per_kg = predecessor.resulting_supply_enthalpy_j_per_kg;
    let predecessor_supply_temperature_c = predecessor.resulting_supply_temperature_c;
    if predecessor_supply_humidity_ratio.is_some() != predecessor_has_supply_humidity_ratio(index)
        || predecessor_supply_enthalpy_j_per_kg.is_some() != predecessor_has_supply_enthalpy(index)
        || predecessor_supply_temperature_c.is_some() != predecessor_has_supply_temperature(index)
    {
        return None;
    }
    let active = match (route.active, active_owners) {
        (false, None) => None,
        (true, Some(owners)) => Some(exact_active_values(
            predecessor,
            predecessor_supply_temperature_c?,
            owners,
        )?),
        _ => return None,
    };
    Some(PreparedInput {
        predecessor_supply_humidity_ratio,
        predecessor_supply_enthalpy_j_per_kg,
        predecessor_supply_temperature_c,
        active,
    })
}

fn exact_active_values(
    predecessor: Predecessor,
    supply_temperature_c: f64,
    owners: ActiveOwners,
) -> Option<PreparedActive> {
    let mixed = owners.mixed_air_owner;
    let flow = owners.supply_mass_flow_owner;
    if !cooling_mixed_air_call_snapshot_is_exact_direct_release(mixed)
        || !cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(flow)
        || !same_identity(
            predecessor,
            mixed.system,
            mixed.parent_call_ordinal,
            mixed.controlled_zone,
        )
        || !same_identity(
            predecessor,
            flow.system,
            flow.parent_call_ordinal,
            flow.controlled_zone,
        )
        || !positive_guard_links_to_mixed_air_call(flow, mixed)
        || !mixed.cooling_call_executed
        || !mixed.no_outdoor_air_fallback_entered
        || !mixed.mixed_air_temperature_assigned
        || !flow.positive_supply_mass_flow_body_entered
        || !flow.supply_mass_flow_rate_read
        || flow.supply_mass_flow_rate_strictly_positive != Some(true)
        || !predecessor
            .dehumidification_control_none_or_constant_supply_humidity_ratio_cp_air_assignment_executed
        || !predecessor.cp_air_assigned
    {
        return None;
    }
    let supply_mass_flow_rate_kg_per_s = flow.supply_mass_flow_rate_kg_per_s?;
    let mixed_air_temperature_c = mixed.mixed_air_temperature_c?;
    let cp_air_j_per_kg_k = predecessor.cp_air_j_per_kg_k?;
    if mixed.supply_mass_flow_rate_kg_per_s?.to_bits() != supply_mass_flow_rate_kg_per_s.to_bits()
        || mixed.child_supply_mass_flow_rate_kg_per_s?.to_bits()
            != supply_mass_flow_rate_kg_per_s.to_bits()
    {
        return None;
    }
    Some(PreparedActive {
        supply_mass_flow_rate_kg_per_s,
        cp_air_j_per_kg_k,
        mixed_air_temperature_c,
        supply_temperature_c,
    })
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
