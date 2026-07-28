//! CP334/CP335/CP336/CP342-to-CP343 retained-lineage validation.

use super::super::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedInput,
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release,
    cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release,
    cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release,
};

pub(super) fn supply_temperature_assignment_links_to_predecessors(
    assignment:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentSnapshot,
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    cp334: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot>,
    cp335: Option<PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot>,
) -> bool {
    let guard_false = predecessor.capacity_limit_sensible_output_guard_false_fallthrough;
    let assigned = predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    let inherited = assignment.system == predecessor.system
        && assignment.parent_call_ordinal == predecessor.parent_call_ordinal
        && assignment.controlled_zone == predecessor.controlled_zone
        && assignment.unit_body_entered == predecessor.unit_body_entered
        && assignment.predecessor_cooling_body_entered
            == predecessor.predecessor_cooling_body_entered
        && assignment.predecessor_no_outdoor_air_fallback_entered
            == predecessor.predecessor_no_outdoor_air_fallback_entered
        && assignment.predecessor_positive_supply_mass_flow_body_entered
            == predecessor.predecessor_positive_supply_mass_flow_body_entered
        && assignment.predecessor_active_guard_false_fallthrough
            == predecessor.predecessor_active_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_guard_evaluated
            == predecessor.predecessor_capacity_limit_guard_evaluated
        && assignment.predecessor_capacity_limit_body_entered
            == predecessor.predecessor_capacity_limit_body_entered
        && assignment.predecessor_active_capacity_limit_guard_false_fallthrough
            == predecessor.predecessor_active_capacity_limit_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_cp_air_assignment_executed
            == predecessor.predecessor_capacity_limit_cp_air_assignment_executed
        && assignment.predecessor_capacity_limit_sensible_output_assignment_executed
            == predecessor.predecessor_capacity_limit_sensible_output_assignment_executed
        && assignment.predecessor_capacity_limit_sensible_output_guard_evaluated
            == predecessor.predecessor_capacity_limit_sensible_output_guard_evaluated
        && assignment.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
            == predecessor.predecessor_capacity_limit_sensible_output_guard_false_fallthrough
        && assignment.predecessor_capacity_limit_sensible_output_adjustment_body_entered
            == predecessor.predecessor_capacity_limit_sensible_output_adjustment_body_entered
        && assignment
            .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
            == predecessor
                .predecessor_capacity_limit_sensible_output_maximum_capacity_assignment_executed
        && assignment
            .predecessor_capacity_limit_sensible_output_supply_enthalpy_assignment_executed
            == assigned
        && assignment.unit_off_skipped == predecessor.unit_off_skipped
        && assignment.non_cooling_skipped == predecessor.non_cooling_skipped
        && assignment.positive_guard_false_fallthrough_skipped
            == predecessor.positive_guard_false_fallthrough_skipped
        && assignment.capacity_limit_guard_false_fallthrough_skipped
            == predecessor.capacity_limit_guard_false_fallthrough_skipped
        && assignment.capacity_limit_sensible_output_guard_false_fallthrough == guard_false
        && assignment.capacity_limit_sensible_output_supply_temperature_assignment_executed
            == assigned;
    if !inherited {
        return false;
    }

    let active = guard_false || assigned;
    if !active {
        return cp334.is_none()
            && cp335.is_none()
            && assignment.preexisting_supply_temperature_c.is_none()
            && assignment.resulting_supply_temperature_c.is_none();
    }
    let (Some(cp334), Some(cp335)) = (cp334, cp335) else {
        return false;
    };
    let (Some(owner_temperature), Some(preexisting), Some(resulting)) = (
        cp334.assigned_supply_temperature_c,
        assignment.preexisting_supply_temperature_c,
        assignment.resulting_supply_temperature_c,
    ) else {
        return false;
    };
    if owner_temperature.to_bits() != preexisting.to_bits() {
        return false;
    }
    if guard_false {
        return preexisting.to_bits() == resulting.to_bits();
    }

    let (
        Some(predecessor_enthalpy),
        Some(owner_humidity),
        Some(enthalpy),
        Some(humidity),
        Some(psychrometric),
        Some(assigned_temperature),
    ) = (
        predecessor.resulting_supply_enthalpy_j_per_kg,
        cp335.assigned_supply_humidity_ratio,
        assignment.supply_enthalpy_j_per_kg,
        assignment.supply_humidity_ratio,
        assignment.psychrometric_supply_temperature_result_c,
        assignment.assigned_supply_temperature_c,
    )
    else {
        return false;
    };
    predecessor_enthalpy.to_bits() == enthalpy.to_bits()
        && owner_humidity.to_bits() == humidity.to_bits()
        && psychrometric.to_bits()
            == crate::psychrometrics::energyplus_psy_tdb_fn_h_w(enthalpy, humidity).to_bits()
        && assigned_temperature.to_bits() == psychrometric.to_bits()
        && resulting.to_bits() == assigned_temperature.to_bits()
}

pub(super) fn retained_source_owner_lineage_is_exact(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    cp334: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot>,
    cp334_witness: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot>,
    cp335: Option<PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot>,
    cp335_witness: Option<
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    >,
    cp336: Option<PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot>,
    cp336_witness: Option<PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot>,
) -> bool {
    let active = predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    if !active {
        return cp334.is_none()
            && cp334_witness.is_none()
            && cp335.is_none()
            && cp335_witness.is_none()
            && cp336.is_none()
            && cp336_witness.is_none();
    }
    let (
        Some(cp334),
        Some(cp334_witness),
        Some(cp335),
        Some(cp335_witness),
        Some(cp336),
        Some(cp336_witness),
    ) = (
        cp334,
        cp334_witness,
        cp335,
        cp335_witness,
        cp336,
        cp336_witness,
    )
    else {
        return false;
    };
    let same_call = [
        (
            cp334.system,
            cp334.parent_call_ordinal,
            cp334.controlled_zone,
        ),
        (
            cp335.system,
            cp335.parent_call_ordinal,
            cp335.controlled_zone,
        ),
        (
            cp336.system,
            cp336.parent_call_ordinal,
            cp336.controlled_zone,
        ),
    ]
    .into_iter()
    .all(|identity| {
        identity
            == (
                predecessor.system,
                predecessor.parent_call_ordinal,
                predecessor.controlled_zone,
            )
    });
    let (
        Some(owner_temperature),
        Some(owner_humidity),
        Some(read_temperature),
        Some(read_humidity),
    ) = (
        cp334.assigned_supply_temperature_c,
        cp335.assigned_supply_humidity_ratio,
        cp336.supply_temperature_c,
        cp336.supply_humidity_ratio,
    )
    else {
        return false;
    };
    same_call
        && cp334.supply_temperature_mixed_air_limit_executed
        && cp334.supply_temperature_assignment_performed
        && cooling_positive_supply_temperature_mixed_air_limit_snapshot_is_exact_direct_release(
            cp334,
        )
        && temperature_snapshots_match_bit_exact(cp334, cp334_witness)
        && owner_temperature.is_finite()
        && cp335.supply_humidity_ratio_mixed_air_assignment_executed
        && cp335.supply_humidity_ratio_assignment_performed
        && cooling_positive_supply_humidity_ratio_mixed_air_assignment_snapshot_is_exact_direct_release(
            cp335,
        )
        && humidity_snapshots_match_bit_exact(cp335, cp335_witness)
        && owner_humidity.is_finite()
        && owner_humidity >= 0.0
        && cp336.supply_enthalpy_assignment_executed
        && cp336.supply_temperature_for_enthalpy_read
        && cp336.supply_humidity_ratio_for_enthalpy_read
        && cooling_positive_supply_enthalpy_assignment_snapshot_is_exact_direct_release(cp336)
        && enthalpy_snapshots_match_bit_exact(cp336, cp336_witness)
        && owner_temperature.to_bits() == read_temperature.to_bits()
        && owner_humidity.to_bits() == read_humidity.to_bits()
}

pub(super) fn retained_input_from_prefix(
    predecessor:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    cp334: Option<PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot>,
    cp335: Option<
        PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    >,
) -> Option<
    PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedInput,
>{
    let active = predecessor.capacity_limit_sensible_output_guard_false_fallthrough
        || predecessor.capacity_limit_sensible_output_supply_enthalpy_assignment_executed;
    if !active {
        return None;
    }
    let preexisting_supply_temperature_c = cp334?.assigned_supply_temperature_c?;
    let active_operands = if predecessor
        .capacity_limit_sensible_output_supply_enthalpy_assignment_executed
    {
        Some(
            PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentActiveOperands {
                supply_enthalpy_j_per_kg: predecessor.resulting_supply_enthalpy_j_per_kg?,
                supply_humidity_ratio: cp335?.assigned_supply_humidity_ratio?,
            },
        )
    } else {
        None
    };
    Some(
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyTemperatureAssignmentRetainedInput {
            preexisting_supply_temperature_c,
            active_operands,
        },
    )
}

pub(super) fn supply_enthalpy_assignment_snapshots_match_bit_exact(
    mut left:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
    mut right:
        PurchasedAirCalcCoolingPositiveSupplyCapacityLimitSensibleOutputSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(
            left.preexisting_supply_enthalpy_j_per_kg,
            right.preexisting_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.mixed_air_enthalpy_j_per_kg,
            right.mixed_air_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.cooling_sensible_output_w,
            right.cooling_sensible_output_w,
        ),
        option_bits_match(
            left.supply_mass_flow_rate_kg_per_s,
            right.supply_mass_flow_rate_kg_per_s,
        ),
        option_bits_match(
            left.specific_cooling_output_j_per_kg,
            right.specific_cooling_output_j_per_kg,
        ),
        option_bits_match(
            left.calculated_supply_enthalpy_j_per_kg,
            right.calculated_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.assigned_supply_enthalpy_j_per_kg,
            right.assigned_supply_enthalpy_j_per_kg,
        ),
        option_bits_match(
            left.resulting_supply_enthalpy_j_per_kg,
            right.resulting_supply_enthalpy_j_per_kg,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.preexisting_supply_enthalpy_j_per_kg = None;
        snapshot.mixed_air_enthalpy_j_per_kg = None;
        snapshot.cooling_sensible_output_w = None;
        snapshot.supply_mass_flow_rate_kg_per_s = None;
        snapshot.specific_cooling_output_j_per_kg = None;
        snapshot.calculated_supply_enthalpy_j_per_kg = None;
        snapshot.assigned_supply_enthalpy_j_per_kg = None;
        snapshot.resulting_supply_enthalpy_j_per_kg = None;
    }
    values_match && left == right
}

fn temperature_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyTemperatureMixedAirLimitSnapshot,
) -> bool {
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

fn humidity_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyHumidityRatioMixedAirAssignmentSnapshot,
) -> bool {
    let values_match = option_bits_match(
        left.mixed_air_humidity_ratio,
        right.mixed_air_humidity_ratio,
    ) && option_bits_match(
        left.assigned_supply_humidity_ratio,
        right.assigned_supply_humidity_ratio,
    );
    for snapshot in [&mut left, &mut right] {
        snapshot.mixed_air_humidity_ratio = None;
        snapshot.assigned_supply_humidity_ratio = None;
    }
    values_match && left == right
}

fn enthalpy_snapshots_match_bit_exact(
    mut left: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
    mut right: PurchasedAirCalcCoolingPositiveSupplyEnthalpyAssignmentSnapshot,
) -> bool {
    let values_match = [
        option_bits_match(left.supply_temperature_c, right.supply_temperature_c),
        option_bits_match(left.supply_humidity_ratio, right.supply_humidity_ratio),
        option_bits_match(
            left.psychrometric_supply_enthalpy_result_j_per_kg,
            right.psychrometric_supply_enthalpy_result_j_per_kg,
        ),
        option_bits_match(
            left.supply_enthalpy_j_per_kg,
            right.supply_enthalpy_j_per_kg,
        ),
    ]
    .into_iter()
    .all(|matches| matches);
    for snapshot in [&mut left, &mut right] {
        snapshot.supply_temperature_c = None;
        snapshot.supply_humidity_ratio = None;
        snapshot.psychrometric_supply_enthalpy_result_j_per_kg = None;
        snapshot.supply_enthalpy_j_per_kg = None;
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
