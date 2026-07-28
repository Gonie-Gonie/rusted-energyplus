use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
};

pub(in crate::ideal_loads) fn cooling_supply_mass_flow_ems_override_guard_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
) -> bool {
    snapshot_is_exact_source_characterization(snapshot)
        && snapshot.ems_supply_mass_flow_override_enabled != Some(true)
}

pub(super) fn snapshot_is_exact_source_characterization(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_GUARD_SOURCE_ORDER;
    let unit_off =
        snapshot.unit_off_skipped && !snapshot.unit_body_entered && !snapshot.cooling_body_entered;
    let non_cooling = snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.cooling_body_entered;
    let cooling = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.cooling_body_entered;
    provenance
        && snapshot.predecessor_cooling_body_entered == snapshot.cooling_body_entered
        && usize::from(unit_off) + usize::from(non_cooling) + usize::from(cooling) == 1
        && if cooling {
            snapshot.ems_supply_mass_flow_override_flag_read
                && snapshot.ems_supply_mass_flow_override_guard_evaluated
                && snapshot
                    .ems_supply_mass_flow_override_enabled
                    .is_some_and(|enabled| {
                        snapshot.ems_supply_mass_flow_override_body_entered == enabled
                            && snapshot.ems_supply_mass_flow_override_guard_false_fallthrough
                                != enabled
                    })
        } else {
            !snapshot.ems_supply_mass_flow_override_flag_read
                && snapshot.ems_supply_mass_flow_override_enabled.is_none()
                && !snapshot.ems_supply_mass_flow_override_guard_evaluated
                && !snapshot.ems_supply_mass_flow_override_body_entered
                && !snapshot.ems_supply_mass_flow_override_guard_false_fallthrough
        }
}

pub(super) fn snapshot_route(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
) -> Option<Route> {
    if !snapshot_is_exact_source_characterization(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.ems_supply_mass_flow_override_body_entered {
        Some(Route::OverrideBodyEntered)
    } else {
        Some(Route::OverrideGuardFalseFallthrough)
    }
}
