//! Release-bound CP362 Humidistat mixed-air humidity-ratio limit.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitSnapshot as Snapshot,
    advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_state,
};
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit::completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent;
use crate::ideal_loads::calc::cooling_humidistat_supply_humidity_ratio_mixed_air_limit::transition::predecessor_snapshots_match_bit_exact;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioForDehumidificationMinimumLimitSnapshot as Predecessor,
    PurchasedAirRuntimeState, classify_no_oa_sensible_subset,
    cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release,
};

mod private_counterfactual;
mod runtime_validation;
mod snapshot_validation;

pub(in crate::ideal_loads::calc) use private_counterfactual::{
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
use runtime_validation::{
    calc_state_identities_match, call_order_is_pending, completed_state_is_consistent,
    next_transition_fits, pending_state_is_consistent,
};
pub(in crate::ideal_loads) use runtime_validation::cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_metadata_is_consistent;
pub(in crate::ideal_loads) use snapshot_validation::{
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor,
    cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshots_match_bit_exact,
};
#[cfg(test)]
pub(super) use snapshot_validation::snapshot_route;

/// Fail-closed CP362 public release error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError {
    UnknownSystem {
        system: IdealLoadsAirSystemId,
    },
    InitializationNotReady {
        system: IdealLoadsAirSystemId,
    },
    SystemIdentityMismatch {
        expected: IdealLoadsAirSystemId,
        actual: IdealLoadsAirSystemId,
    },
    SystemOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    DehumidificationControlTypeOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
        actual: DehumidificationControlType,
    },
    PredecessorSnapshotMismatch {
        system: IdealLoadsAirSystemId,
    },
    PredecessorCallOrder {
        system: IdealLoadsAirSystemId,
    },
    PredecessorOutsideDirectSubset {
        system: IdealLoadsAirSystemId,
    },
    RuntimeStateInvariantViolation {
        system: IdealLoadsAirSystemId,
    },
}

impl std::fmt::Display for PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "CP362 Humidistat mixed-air-limit release failed: {self:?}"
        )
    }
}

impl std::error::Error for PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError {}

#[allow(dead_code)]
pub(in crate::ideal_loads::calc) fn completed_direct_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_is_consistent(
    runtime: &PurchasedAirRuntimeState,
    unit: &crate::ideal_loads::PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    snapshot: Snapshot,
    witness: Option<Snapshot>,
) -> bool {
    let Some(predecessor) = unit
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
        .latest
    else {
        return false;
    };
    let Some(predecessor_witness) = runtime
        .cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witness(
            system.id,
        )
    else {
        return false;
    };
    system.dehumidification_control_type == DehumidificationControlType::None
        && predecessor_snapshots_match_bit_exact(predecessor_witness, predecessor)
        && cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(predecessor_witness),
        )
        && cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_links_to_predecessor(
            snapshot,
            predecessor,
        )
        && completed_state_is_consistent(
            unit,
            snapshot,
            witness,
            system.dehumidification_control_type,
        )
}

/// Executes CP362 for the exact direct no-OA release route.
pub fn advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit(
    runtime: &mut PurchasedAirRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> Result<
    Snapshot,
    PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError,
> {
    use PurchasedAirCalcCoolingHumidistatSupplyHumidityRatioMixedAirLimitError as Error;

    let selected = predecessor.system;
    let unit = runtime
        .units
        .get(&selected)
        .ok_or(Error::UnknownSystem { system: selected })?;
    let retained_predecessor = unit
        .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
        .latest;
    let predecessor_witness = runtime
        .cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witness(
            selected,
        );
    let current_witness = runtime
        .cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(selected);

    if system.id != selected {
        return Err(Error::SystemIdentityMismatch {
            expected: selected,
            actual: system.id,
        });
    }
    if !classify_no_oa_sensible_subset(system).is_supported() {
        return Err(Error::SystemOutsideDirectSubset { system: selected });
    }
    if system.dehumidification_control_type != DehumidificationControlType::None {
        return Err(Error::DehumidificationControlTypeOutsideDirectSubset {
            system: selected,
            actual: system.dehumidification_control_type,
        });
    }
    if !unit.topology_completed
        || unit.topology_failure.is_some()
        || unit.init_call_count == 0
        || unit.calc_entry.latest.is_none()
    {
        return Err(Error::InitializationNotReady { system: selected });
    }
    let controlled_zone = unit
        .controlled_zone
        .ok_or(Error::InitializationNotReady { system: selected })?;
    let Some(retained_predecessor) = retained_predecessor else {
        return Err(Error::PredecessorSnapshotMismatch { system: selected });
    };
    if predecessor.controlled_zone != controlled_zone
        || !predecessor_snapshots_match_bit_exact(retained_predecessor, predecessor)
        || !predecessor_witness.is_some_and(|witness| {
            predecessor_snapshots_match_bit_exact(witness, predecessor)
        })
    {
        return Err(Error::PredecessorSnapshotMismatch { system: selected });
    }
    if !cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_snapshot_is_exact_direct_release(
        predecessor,
    ) {
        return Err(Error::PredecessorOutsideDirectSubset { system: selected });
    }
    if !calc_state_identities_match(unit, selected)
        || !pending_state_is_consistent(
            unit,
            retained_predecessor,
            current_witness,
            system.dehumidification_control_type,
        )
        || !completed_direct_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_is_consistent(
            runtime,
            unit,
            system,
            retained_predecessor,
            predecessor_witness,
        )
    {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }
    if !call_order_is_pending(unit, retained_predecessor)
        || retained_predecessor.parent_call_ordinal != unit.init_call_count
    {
        return Err(Error::PredecessorCallOrder { system: selected });
    }
    if !next_transition_fits(
        &unit.calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
        retained_predecessor,
    ) {
        return Err(Error::RuntimeStateInvariantViolation { system: selected });
    }

    let snapshot = {
        let unit = runtime
            .units
            .get_mut(&selected)
            .ok_or(Error::UnknownSystem { system: selected })?;
        advance_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_state(
            &mut unit.calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit,
            retained_predecessor,
            None,
        )
    }
    .ok_or(Error::RuntimeStateInvariantViolation { system: selected })?;
    runtime.set_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(
        selected, snapshot,
    );
    debug_assert!(
        cooling_humidistat_supply_humidity_ratio_mixed_air_limit_snapshot_is_exact_direct_release(
            snapshot,
        )
    );
    debug_assert!(runtime.units.get(&selected).is_some_and(|unit| {
        completed_direct_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_is_consistent(
            runtime,
            unit,
            system,
            snapshot,
            runtime
                .cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(selected),
        )
    }));
    Ok(snapshot)
}
