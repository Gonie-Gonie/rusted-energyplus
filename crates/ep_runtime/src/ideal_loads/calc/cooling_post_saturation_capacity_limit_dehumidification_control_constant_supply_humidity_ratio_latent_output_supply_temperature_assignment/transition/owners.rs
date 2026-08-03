//! Exact CP406/CP378/CP385 operand-owner validation for CP407.

use super::ActiveOwners;
use super::routes::{
    RetainedRoute, predecessor_has_supply_enthalpy, predecessor_has_supply_humidity_ratio,
    predecessor_has_supply_temperature,
};
use crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputCapacityGuardElseBranchEntrySnapshot as Predecessor;
use crate::ideal_loads::calc::{
    cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_route,
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
    pub supply_enthalpy_j_per_kg: f64,
    pub supply_humidity_ratio: f64,
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
    if predecessor_supply_humidity_ratio.is_some() != predecessor_has_supply_humidity_ratio(route)
        || predecessor_supply_enthalpy_j_per_kg.is_some() != predecessor_has_supply_enthalpy(index)
        || predecessor_supply_temperature_c.is_some() != predecessor_has_supply_temperature(index)
    {
        return None;
    }
    let active = match (route.assignment_executed, active_owners) {
        (false, None) => None,
        (true, Some(owners)) => Some(exact_active_values(predecessor, owners)?),
        _ => return None,
    };
    Some(PreparedInput {
        predecessor_supply_humidity_ratio,
        predecessor_supply_enthalpy_j_per_kg,
        predecessor_supply_temperature_c,
        active,
    })
}

fn exact_active_values(predecessor: Predecessor, owners: ActiveOwners) -> Option<PreparedActive> {
    let humidity = owners.supply_humidity_ratio_owner;
    let enthalpy = owners.supply_enthalpy_owner;
    if cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_route(humidity).is_none()
        || !cooling_post_saturation_capacity_limit_dehumidification_total_output_supply_enthalpy_assignment_snapshot_is_exact(enthalpy)
        || !same_identity(
            predecessor,
            humidity.system,
            humidity.parent_call_ordinal,
            humidity.controlled_zone,
        )
        || !same_identity(
            predecessor,
            enthalpy.system,
            enthalpy.parent_call_ordinal,
            enthalpy.controlled_zone,
        )
        || !humidity.purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed
        || !enthalpy.supply_enthalpy_assignment_executed
    {
        return None;
    }
    let supply_humidity_ratio = humidity.resulting_supply_humidity_ratio?;
    let supply_enthalpy_j_per_kg = enthalpy.resulting_supply_enthalpy_j_per_kg?;
    let predecessor_enthalpy = predecessor.resulting_supply_enthalpy_j_per_kg?;
    predecessor.resulting_supply_temperature_c?;
    if predecessor.resulting_supply_humidity_ratio.is_some()
        || predecessor_enthalpy.to_bits() != supply_enthalpy_j_per_kg.to_bits()
    {
        return None;
    }
    Some(PreparedActive {
        supply_enthalpy_j_per_kg,
        supply_humidity_ratio,
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
