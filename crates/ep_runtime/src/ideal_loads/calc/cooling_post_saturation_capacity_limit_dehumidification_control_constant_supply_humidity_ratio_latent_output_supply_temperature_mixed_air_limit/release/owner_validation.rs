//! Retained CP407 prefix and exact CP329 mixed-air operand-owner validation.

use ep_model::IdealLoadsAirSystem;

use super::super::{
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitRuntimeState as State,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureMixedAirLimitSnapshot as Snapshot,
    advance_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_state as advance,
};
use crate::ideal_loads::calc::{
    cooling_mixed_air_call_committed_latest_sensible_output_inputs,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_committed_latest_snapshot_is_consistent,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingMixedAirCallSnapshot as MixedAirOwner,
    PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlConstantSupplyHumidityRatioLatentOutputSupplyTemperatureAssignmentSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_mixed_air_call_snapshot_is_exact_direct_release,
    cooling_mixed_air_call_snapshots_match_bit_exact,
    cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_is_exact_direct_release,
};

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let retained = unit
        .calc_cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment
        .latest;
    let witness = runtime.cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_latest_witness(system.id);
    retained.is_some_and(|retained| {
        crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshots_match_bit_exact(retained, predecessor)
    })
        && witness.is_some_and(|witness| {
            crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshots_match_bit_exact(witness, predecessor)
        })
        && cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_snapshot_is_exact_direct_release(predecessor)
        && witness.is_some_and(|witness| {
            cooling_post_saturation_capacity_limit_dehumidification_control_constant_supply_humidity_ratio_latent_output_supply_temperature_assignment_committed_latest_snapshot_is_consistent(
                unit,
                system,
                predecessor,
                witness,
            )
        })
}

pub(super) fn active_owner_from_retained_runtime(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Option<MixedAirOwner> {
    let owner = unit.calc_cooling_mixed_air_call.latest?;
    let witness = runtime.cooling_mixed_air_call_latest_witness(system.id)?;
    let committed =
        cooling_mixed_air_call_committed_latest_sensible_output_inputs(unit, witness)?;
    if !same_identity(predecessor, owner)
        || !owner.mixed_air_temperature_assigned
        || !cooling_mixed_air_call_snapshot_is_exact_direct_release(owner)
        || !cooling_mixed_air_call_snapshots_match_bit_exact(owner, witness)
        || owner
            .mixed_air_temperature_c
            .is_none_or(|temperature| {
                temperature.to_bits() != committed.mixed_air_temperature_c.to_bits()
            })
    {
        return None;
    }
    Some(owner)
}

pub(super) fn mixed_air_limit_links_to_prefix(
    snapshot: Snapshot,
    predecessor: Predecessor,
    owner: Option<MixedAirOwner>,
) -> bool {
    let mut state = State::new(predecessor.system);
    advance(&mut state, predecessor, owner).is_some_and(|expected| {
        super::snapshot_validation::snapshots_match_bit_exact(expected, snapshot)
    })
}

pub(super) fn snapshot_operands_link_to_owner(
    snapshot: Snapshot,
    predecessor: Predecessor,
    owner: Option<MixedAirOwner>,
) -> bool {
    match owner {
        None => !snapshot.dehumidification_control_none_or_constant_supply_humidity_ratio_latent_output_supply_temperature_mixed_air_limit_executed,
        Some(owner) => {
            same_identity(predecessor, owner)
                && option_matches(
                    snapshot.supply_temperature_before_mixed_air_limit_c,
                    predecessor.resulting_supply_temperature_c,
                )
                && option_matches(snapshot.mixed_air_temperature_c, owner.mixed_air_temperature_c)
        }
    }
}

fn same_identity(predecessor: Predecessor, owner: MixedAirOwner) -> bool {
    predecessor.system == owner.system
        && predecessor.parent_call_ordinal == owner.parent_call_ordinal
        && predecessor.controlled_zone == owner.controlled_zone
}

fn option_matches(left: Option<f64>, right: Option<f64>) -> bool {
    match (left, right) {
        (Some(left), Some(right)) => left.to_bits() == right.to_bits(),
        (None, None) => true,
        _ => false,
    }
}
