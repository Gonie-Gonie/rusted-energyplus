//! CP379 immediate-predecessor and transitive temperature-owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot as Snapshot,
};
use super::snapshot_validation::{snapshot_links_to_prefix, temperature_owner};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_assignment::{
    completed_direct_cooling_supply_humidity_ratio_saturation_assignment_is_consistent,
    cooling_supply_humidity_ratio_saturation_assignment_snapshots_match_bit_exact as cp377_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_saturation_limit_assignment::{
    completed_direct_cooling_supply_humidity_ratio_saturation_limit_assignment_is_consistent,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshots_match_bit_exact as cp378_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as TemperaturePrefix,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationLimitAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn assignment_links_to_prefix(
    assignment: Snapshot,
    predecessor: Predecessor,
    temperature_prefix: TemperaturePrefix,
) -> bool {
    snapshot_links_to_prefix(assignment, predecessor, temperature_prefix)
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_supply_humidity_ratio_saturation_limit_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_supply_humidity_ratio_saturation_limit_assignment_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && cp378_snapshots_match_bit_exact(retained, predecessor)
        && cp378_snapshots_match_bit_exact(witness, predecessor)
        && cooling_supply_humidity_ratio_saturation_limit_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_supply_humidity_ratio_saturation_limit_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn direct_temperature_prefix_and_input(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<(TemperaturePrefix, Option<ActiveInput>)> {
    let retained = unit
        .calc_cooling_supply_humidity_ratio_saturation_assignment
        .latest?;
    let witness = runtime
        .cooling_supply_humidity_ratio_saturation_assignment_latest_witness(system.id)?;
    if !same_call(
        retained.system,
        retained.parent_call_ordinal,
        retained.controlled_zone,
        predecessor,
    ) || !cp377_snapshots_match_bit_exact(retained, witness)
        || !cooling_supply_humidity_ratio_saturation_assignment_snapshot_is_exact_direct_release(
            retained,
        )
        || !completed_direct_cooling_supply_humidity_ratio_saturation_assignment_is_consistent(
            runtime,
            unit,
            system,
            retained,
            Some(witness),
        )
    {
        return None;
    }
    let active = predecessor
        .purchased_air_supply_humidity_ratio_saturation_limit_assignment_performed;
    if !active {
        return (!retained.purchased_air_supply_temperature_for_saturation_humidity_ratio_read
            && retained
                .supply_temperature_for_saturation_humidity_ratio_c
                .is_none()
            && temperature_owner(retained).is_none())
        .then_some((retained, None));
    }
    if !retained.purchased_air_supply_temperature_for_saturation_humidity_ratio_read {
        return None;
    }
    Some((
        retained,
        Some(ActiveInput {
            supply_temperature_c: retained
                .supply_temperature_for_saturation_humidity_ratio_c?,
            temperature_owner: temperature_owner(retained)?,
        }),
    ))
}

fn same_call(
    system: ep_model::IdealLoadsAirSystemId,
    ordinal: usize,
    zone: ep_model::ZoneId,
    predecessor: Predecessor,
) -> bool {
    system == predecessor.system
        && ordinal == predecessor.parent_call_ordinal
        && zone == predecessor.controlled_zone
}
