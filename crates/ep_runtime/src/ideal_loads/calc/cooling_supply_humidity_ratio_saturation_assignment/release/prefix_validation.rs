//! CP377 immediate-predecessor and temperature-owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentSnapshot as Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioSaturationAssignmentTemperatureOwner as Owner,
};
use super::snapshot_validation::snapshot_links_to_predecessor;
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit::completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent;
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_pre_saturation_original_assignment::{
    completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshots_match_bit_exact as cp376_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureMixedAirLimitSnapshot as Cp344Snapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot as Cp334Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioPreSaturationOriginalAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn assignment_links_to_predecessor(
    assignment: Snapshot,
    predecessor: Predecessor,
) -> bool {
    snapshot_links_to_predecessor(assignment, predecessor)
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_supply_humidity_ratio_pre_saturation_original_assignment
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_supply_humidity_ratio_pre_saturation_original_assignment_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && cp376_snapshots_match_bit_exact(retained, predecessor)
        && cp376_snapshots_match_bit_exact(witness, predecessor)
        && cooling_supply_humidity_ratio_pre_saturation_original_assignment_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_supply_humidity_ratio_pre_saturation_original_assignment_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn direct_temperature_owner(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<(f64, Owner)> {
    let cp344 = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit
        .latest?;
    let cp344_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_latest_witness(
            system.id,
        )?;
    if !same_call(cp344.system, cp344.parent_call_ordinal, cp344.controlled_zone, predecessor)
        || !cp344_snapshots_match_bit_exact(cp344, cp344_witness)
        || !cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            cp344,
        )
        || !completed_direct_cooling_positive_supply_capacity_limit_sensible_output_supply_temperature_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            cp344,
            Some(cp344_witness),
        )
    {
        return None;
    }
    if cp344.capacity_limit_sensible_output_supply_temperature_mixed_air_limit_executed {
        let assigned = cp344.assigned_supply_temperature_c?;
        let resulting = cp344.resulting_supply_temperature_c?;
        return (assigned.to_bits() == resulting.to_bits())
            .then_some((resulting, Owner::Cp344CapacityMixedAirLimit));
    }
    if !cp344.capacity_limit_guard_false_fallthrough_skipped
        && !cp344.capacity_limit_sensible_output_guard_false_fallthrough
    {
        return None;
    }
    direct_cp334_temperature_owner(runtime, unit, system, predecessor)
        .map(|value| (value, Owner::Cp334MixedAirLimit))
}

fn direct_cp334_temperature_owner(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<f64> {
    let cp334 = unit
        .calc_cooling_positive_supply_temperature_mixed_air_limit
        .latest?;
    let cp334_witness =
        runtime.cooling_positive_supply_temperature_mixed_air_limit_latest_witness(system.id)?;
    if !same_call(
        cp334.system,
        cp334.parent_call_ordinal,
        cp334.controlled_zone,
        predecessor,
    ) || !cp334_snapshots_match_bit_exact(cp334, cp334_witness)
        || !cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            cp334,
        )
        || !completed_direct_cooling_positive_supply_temperature_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            cp334,
            Some(cp334_witness),
        )
    {
        return None;
    }
    cp334.assigned_supply_temperature_c
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

fn cp334_snapshots_match_bit_exact(mut left: Cp334Snapshot, mut right: Cp334Snapshot) -> bool {
    let values_match = [
        option_bits_match(
            left.supply_temperature_before_mixed_air_limit_c,
            right.supply_temperature_before_mixed_air_limit_c,
        ),
        option_bits_match(left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        option_bits_match(
            left.minimum_supply_temperature_c,
            right.minimum_supply_temperature_c,
        ),
        option_bits_match(
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_temperature_before_mixed_air_limit_c = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.minimum_supply_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
    }
    values_match && left == right
}

fn cp344_snapshots_match_bit_exact(mut left: Cp344Snapshot, mut right: Cp344Snapshot) -> bool {
    let values_match = [
        option_bits_match(
            left.preexisting_supply_temperature_c,
            right.preexisting_supply_temperature_c,
        ),
        option_bits_match(
            left.supply_temperature_before_mixed_air_limit_c,
            right.supply_temperature_before_mixed_air_limit_c,
        ),
        option_bits_match(left.mixed_air_temperature_c, right.mixed_air_temperature_c),
        option_bits_match(
            left.minimum_supply_temperature_c,
            right.minimum_supply_temperature_c,
        ),
        option_bits_match(
            left.assigned_supply_temperature_c,
            right.assigned_supply_temperature_c,
        ),
        option_bits_match(
            left.resulting_supply_temperature_c,
            right.resulting_supply_temperature_c,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_supply_temperature_c = None;
        snapshot.supply_temperature_before_mixed_air_limit_c = None;
        snapshot.mixed_air_temperature_c = None;
        snapshot.minimum_supply_temperature_c = None;
        snapshot.assigned_supply_temperature_c = None;
        snapshot.resulting_supply_temperature_c = None;
    }
    values_match && left == right
}

fn option_bits_match(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
