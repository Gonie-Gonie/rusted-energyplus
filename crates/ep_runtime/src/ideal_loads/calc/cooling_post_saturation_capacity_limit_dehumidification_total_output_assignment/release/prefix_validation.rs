//! CP329/CP330/CP339/CP379/CP381 retained owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentActiveInput as ActiveInput,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentSnapshot as Snapshot,
};
use super::error::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationTotalOutputAssignmentInput as InvalidInput;
use super::snapshot_validation::snapshot_links_to_predecessor;
use crate::ideal_loads::calc::cooling_mixed_air_call::{
    cooling_mixed_air_call_committed_latest_mixed_air_enthalpy,
    cooling_mixed_air_call_committed_latest_sensible_output_inputs,
};
use crate::ideal_loads::calc::cooling_positive_supply_capacity_limit_sensible_output_assignment::cooling_positive_supply_capacity_limit_sensible_output_assignment_committed_latest_snapshot_is_consistent;
use crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_guard::{
    cooling_post_saturation_capacity_limit_dehumidification_guard_committed_latest_snapshot_is_consistent,
    cooling_post_saturation_capacity_limit_dehumidification_guard_snapshots_match_bit_exact as cp381_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_supply_enthalpy_post_saturation_assignment::{
    cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent,
    cooling_supply_enthalpy_post_saturation_assignment_snapshots_match_bit_exact as cp379_snapshots_match_bit_exact,
};
use crate::ideal_loads::calc::cooling_supply_mass_flow_positive_guard::{
    cooling_supply_mass_flow_positive_guard_committed_latest_snapshot_is_consistent,
    cooling_supply_mass_flow_positive_guard_snapshots_match_bit_exact as cp330_snapshots_match_bit_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
    cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release,
    cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release,
    cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release,
    cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release,
};

pub(super) enum ActiveInputValidationError {
    Lineage,
    Invalid(InvalidInput),
}

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
        .calc_cooling_post_saturation_capacity_limit_dehumidification_guard
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_post_saturation_capacity_limit_dehumidification_guard_latest_witness(system.id)
    else {
        return false;
    };
    system.id == predecessor.system
        && cp381_snapshots_match_bit_exact(retained, predecessor)
        && cp381_snapshots_match_bit_exact(witness, predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_guard_snapshot_is_exact_direct_release(predecessor)
        && cooling_post_saturation_capacity_limit_dehumidification_guard_committed_latest_snapshot_is_consistent(
            unit,
            system,
            witness,
        )
}

pub(super) fn retained_active_input(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Result<Option<ActiveInput>, ActiveInputValidationError> {
    if !predecessor.dehumidification_body_entered {
        return Ok(None);
    }

    let cp330 = unit
        .calc_cooling_supply_mass_flow_positive_guard
        .latest
        .ok_or(ActiveInputValidationError::Lineage)?;
    let cp330_witness = runtime
        .cooling_supply_mass_flow_positive_guard_latest_witness(system.id)
        .ok_or(ActiveInputValidationError::Lineage)?;
    let cp329 = unit
        .calc_cooling_mixed_air_call
        .latest
        .ok_or(ActiveInputValidationError::Lineage)?;
    let cp329_witness = runtime
        .cooling_mixed_air_call_latest_witness(system.id)
        .ok_or(ActiveInputValidationError::Lineage)?;
    let cp339 = unit
        .calc_cooling_positive_supply_capacity_limit_sensible_output_assignment
        .latest
        .ok_or(ActiveInputValidationError::Lineage)?;
    let cp339_witness = runtime
        .cooling_positive_supply_capacity_limit_sensible_output_assignment_latest_witness(system.id)
        .ok_or(ActiveInputValidationError::Lineage)?;
    let cp379 = unit
        .calc_cooling_supply_enthalpy_post_saturation_assignment
        .latest
        .ok_or(ActiveInputValidationError::Lineage)?;
    let cp379_witness = runtime
        .cooling_supply_enthalpy_post_saturation_assignment_latest_witness(system.id)
        .ok_or(ActiveInputValidationError::Lineage)?;
    let _cp329_inputs = cooling_mixed_air_call_committed_latest_sensible_output_inputs(
        unit,
        cp329_witness,
    )
    .ok_or(ActiveInputValidationError::Lineage)?;
    let committed_mixed_air_enthalpy =
        cooling_mixed_air_call_committed_latest_mixed_air_enthalpy(unit, cp329_witness)
            .ok_or(ActiveInputValidationError::Lineage)?;

    if !same_call(predecessor, cp330.system, cp330.parent_call_ordinal, cp330.controlled_zone)
        || !same_call(predecessor, cp329.system, cp329.parent_call_ordinal, cp329.controlled_zone)
        || !same_call(predecessor, cp339.system, cp339.parent_call_ordinal, cp339.controlled_zone)
        || !same_call(predecessor, cp379.system, cp379.parent_call_ordinal, cp379.controlled_zone)
        || !route_flags_match(predecessor, cp379)
        || !cp330_snapshots_match_bit_exact(cp330, cp330_witness)
        || !cooling_supply_mass_flow_positive_guard_snapshot_is_exact_direct_release(cp330)
        || !cooling_supply_mass_flow_positive_guard_committed_latest_snapshot_is_consistent(
            unit,
            system.id,
            cp330,
            cp330_witness,
        )
        || !cooling_mixed_air_call_snapshots_match_bit_exact(cp329, cp329_witness)
        || !cooling_mixed_air_call_snapshot_is_exact_direct_release(cp329)
        || cp329
            .mixed_air_enthalpy_projection_j_per_kg
            .is_none_or(|value| value.to_bits() != committed_mixed_air_enthalpy.to_bits())
        || !cp339.capacity_limit_sensible_output_assignment_executed
        || !cooling_positive_supply_capacity_limit_sensible_output_assignment_snapshot_is_exact_direct_release(cp339)
        || !cooling_positive_supply_capacity_limit_sensible_output_assignment_committed_latest_snapshot_is_consistent(
            unit,
            system.id,
            cp339,
            cp339_witness,
        )
        || !cp379_snapshots_match_bit_exact(cp379, cp379_witness)
        || !cooling_supply_enthalpy_post_saturation_assignment_snapshot_is_exact_direct_release(cp379)
        || !cooling_supply_enthalpy_post_saturation_assignment_committed_latest_snapshot_is_consistent(
            unit, cp379_witness,
        )
    {
        return Err(ActiveInputValidationError::Lineage);
    }

    let flow = cp330
        .supply_mass_flow_rate_kg_per_s
        .ok_or(ActiveInputValidationError::Lineage)?;
    let mixed = cp329
        .mixed_air_enthalpy_projection_j_per_kg
        .ok_or(ActiveInputValidationError::Lineage)?;
    let supply = cp379
        .resulting_supply_enthalpy_j_per_kg
        .ok_or(ActiveInputValidationError::Lineage)?;
    if flow <= 0.0 || flow.is_nan() {
        return Err(ActiveInputValidationError::Invalid(
            InvalidInput::SupplyMassFlowRate,
        ));
    }
    if !mixed.is_finite() {
        return Err(ActiveInputValidationError::Invalid(
            InvalidInput::MixedAirEnthalpy,
        ));
    }
    if !supply.is_finite() {
        return Err(ActiveInputValidationError::Invalid(
            InvalidInput::SupplyEnthalpy,
        ));
    }

    let cp329_flow_matches = [
        cp329.supply_mass_flow_rate_kg_per_s,
        cp329.child_supply_mass_flow_rate_kg_per_s,
        cp329.resulting_recirculation_mass_flow_rate_kg_per_s,
    ]
    .into_iter()
    .all(|value| option_bits_equal(value, Some(flow)));
    let cp339_flow_matches = option_bits_equal(cp339.supply_mass_flow_rate_kg_per_s, Some(flow));
    let cp329_enthalpy_matches = option_bits_equal(
        cp329.recirculation_enthalpy_projection_j_per_kg,
        Some(mixed),
    );
    let cp339_enthalpy_matches = option_bits_equal(cp339.mixed_air_enthalpy_j_per_kg, Some(mixed));
    let cp379_enthalpy_matches =
        option_bits_equal(cp379.assigned_supply_enthalpy_j_per_kg, Some(supply))
            && option_bits_equal(cp379.psychrometric_supply_enthalpy_j_per_kg, Some(supply));
    if !cp330.positive_supply_mass_flow_body_entered
        || !cp329_flow_matches
        || !cp339_flow_matches
        || !cp329_enthalpy_matches
        || !cp339_enthalpy_matches
        || !cp379.local_supply_enthalpy_after_saturation_limit_assignment_performed
        || !cp379_enthalpy_matches
    {
        return Err(ActiveInputValidationError::Lineage);
    }

    Ok(Some(ActiveInput {
        supply_mass_flow_rate_kg_per_s: flow,
        mixed_air_enthalpy_j_per_kg: mixed,
        supply_enthalpy_j_per_kg: supply,
        cp330_supply_mass_flow_rate_owned_read: true,
        cp329_same_call_supply_mass_flow_rate_bit_corroborated: true,
        cp339_same_call_supply_mass_flow_rate_bit_corroborated: true,
        cp329_mixed_air_enthalpy_owned_read: true,
        cp329_same_call_recirculation_enthalpy_bit_corroborated: true,
        cp339_same_call_mixed_air_enthalpy_bit_corroborated: true,
        cp379_post_saturation_supply_enthalpy_owned_read: true,
        cp379_same_call_supply_enthalpy_bits_corroborated: true,
    }))
}

fn same_call(
    predecessor: Predecessor,
    system: ep_model::IdealLoadsAirSystemId,
    ordinal: usize,
    zone: ep_model::ZoneId,
) -> bool {
    predecessor.system == system
        && predecessor.parent_call_ordinal == ordinal
        && predecessor.controlled_zone == zone
}

fn route_flags_match(
    predecessor: Predecessor,
    cp379: crate::ideal_loads::PurchasedAirCalcCoolingSupplyEnthalpyPostSaturationAssignmentSnapshot,
) -> bool {
    [
        predecessor.unit_off_skipped,
        predecessor.non_cooling_skipped,
        predecessor.positive_guard_false_fallthrough_skipped,
        predecessor.heating_availability_guard_false_fallthrough,
        predecessor.humidification_control_guard_false_fallthrough,
        predecessor.dehumidification_control_humidistat_maximum_assignment_executed,
        predecessor.dehumidification_control_none_maximum_assignment_executed,
        predecessor.dehumidification_control_guard_false_fallthrough,
    ] == [
        cp379.unit_off_skipped,
        cp379.non_cooling_skipped,
        cp379.positive_guard_false_fallthrough_skipped,
        cp379.heating_availability_guard_false_fallthrough,
        cp379.humidification_control_guard_false_fallthrough,
        cp379.dehumidification_control_humidistat_maximum_assignment_executed,
        cp379.dehumidification_control_none_maximum_assignment_executed,
        cp379.dehumidification_control_guard_false_fallthrough,
    ]
}

fn option_bits_equal(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
