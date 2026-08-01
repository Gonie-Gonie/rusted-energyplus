//! CP369-to-CP370 retained lineage and HumidCtrlType provenance validation.

use ep_model::{HumidificationControlType, IdealLoadsAirSystem};

use super::super::PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardSnapshot as Snapshot;
use super::snapshot_validation::{predecessor_snapshot, snapshot_route};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_control_humidistat_guard::transition::predecessor_route;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_heating_availability_guard::{
    completed_direct_cooling_supply_humidity_ratio_humidification_heating_availability_guard_is_consistent,
    cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact,
};
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState, PurchasedAirUnitRuntimeState,
    cooling_humidification_flow_snapshot_is_exact_direct_release,
    cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release,
};

pub(super) fn guard_links_to_predecessor(
    guard: Snapshot,
    predecessor: Predecessor,
    control: HumidificationControlType,
) -> bool {
    let Some(route) = predecessor_route(predecessor) else {
        return false;
    };
    let evaluate = matches!(
        route,
        crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_control_humidistat_guard::transition::PredecessorRoute::Active {
            heating_on: true,
            ..
        }
    );
    let humidistat = evaluate.then_some(control == HumidificationControlType::Humidistat);
    snapshot_route(guard).is_some()
        && cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact(
            predecessor_snapshot(guard),
            predecessor,
        )
        && guard.humidification_control_type_read == evaluate
        && guard.humidification_control_type == evaluate.then_some(control)
        && guard.humidification_control_type_humidistat == humidistat
        && guard.humidification_control_body_entered == (humidistat == Some(true))
        && guard.humidification_control_guard_false_fallthrough
            == (humidistat == Some(false))
}

pub(super) fn direct_predecessor_is_retained_and_complete(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let Some(retained) = unit
        .calc_cooling_supply_humidity_ratio_humidification_heating_availability_guard
        .latest
    else {
        return false;
    };
    let Some(witness) = runtime
        .cooling_supply_humidity_ratio_humidification_heating_availability_guard_latest_witness(
            system.id,
        )
    else {
        return false;
    };
    system.id == predecessor.system
        && cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact(
            retained,
            predecessor,
        )
        && cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshots_match_exact(
            witness,
            predecessor,
        )
        && cooling_supply_humidity_ratio_humidification_heating_availability_guard_snapshot_is_exact_direct_release(
            predecessor,
        )
        && completed_direct_cooling_supply_humidity_ratio_humidification_heating_availability_guard_is_consistent(
            runtime,
            unit,
            system,
            predecessor,
            Some(witness),
        )
}

pub(super) fn humidification_control_type_provenance_is_exact(
    runtime: &PurchasedAirRuntimeState,
    unit: &PurchasedAirUnitRuntimeState,
    system: &IdealLoadsAirSystem,
    predecessor: Predecessor,
) -> bool {
    let owner = system.humidification_control_type;
    let Some(cp320) = unit.calc_cooling_humidification_flow.latest else {
        return false;
    };
    let Some(cp320_witness) = runtime.cooling_humidification_flow_latest_witness(system.id) else {
        return false;
    };
    let owner_is_humidistat = owner == HumidificationControlType::Humidistat;
    let control_operand_matches =
        !predecessor.cooling_supply_humidity_ratio_humidification_body_entered
            || (predecessor.heating_on == Some(true)
                && cp320.humidification_control_type_read
                && cp320.humidification_control_type == Some(owner)
                && cp320.humidification_control_type_humidistat == Some(owner_is_humidistat)
                && cp320.humidification_control_body_entered == owner_is_humidistat);
    system.id == predecessor.system
        && cp320.system == predecessor.system
        && cp320.parent_call_ordinal == predecessor.parent_call_ordinal
        && cp320.controlled_zone == predecessor.controlled_zone
        && cp320 == cp320_witness
        && cooling_humidification_flow_snapshot_is_exact_direct_release(cp320)
        && cooling_humidification_flow_snapshot_is_exact_direct_release(cp320_witness)
        && control_operand_matches
}
