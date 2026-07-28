use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodyRetainedRoute as Route,
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
};

pub(in crate::ideal_loads) fn cooling_supply_mass_flow_ems_override_body_snapshot_is_exact_direct_release(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) -> bool {
    snapshot_is_exact_source_characterization(snapshot) && snapshot.body_skipped
}

pub(super) fn snapshot_is_exact_source_characterization(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) -> bool {
    let provenance = snapshot.source
        == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE
        && snapshot.first_excluded_source
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_FIRST_EXCLUDED_SOURCE
        && snapshot.source_order
            == PURCHASED_AIR_CALC_COOLING_SUPPLY_MASS_FLOW_EMS_OVERRIDE_BODY_SOURCE_ORDER;
    let unit_off = snapshot.unit_off_skipped
        && !snapshot.unit_body_entered
        && !snapshot.cooling_body_entered
        && !snapshot.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough
        && !snapshot.ems_disabled_fallthrough;
    let non_cooling = snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && !snapshot.cooling_body_entered
        && !snapshot.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough
        && !snapshot.ems_disabled_fallthrough;
    let active_disabled = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.cooling_body_entered
        && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough
        && snapshot.ems_disabled_fallthrough;
    let active_override = !snapshot.unit_off_skipped
        && !snapshot.non_cooling_skipped
        && snapshot.unit_body_entered
        && snapshot.cooling_body_entered
        && snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && !snapshot.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough
        && !snapshot.ems_disabled_fallthrough;
    let route_count = usize::from(unit_off)
        + usize::from(non_cooling)
        + usize::from(active_disabled)
        + usize::from(active_override);

    provenance
        && snapshot.predecessor_cooling_body_entered == snapshot.cooling_body_entered
        && route_count == 1
        && snapshot.body_skipped != active_override
        && if active_override {
            entered_body_fields_are_exact(snapshot)
        } else {
            skipped_body_fields_are_exact(snapshot)
        }
}

pub(super) fn snapshot_route(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) -> Option<Route> {
    if !snapshot_is_exact_source_characterization(snapshot) {
        None
    } else if snapshot.unit_off_skipped {
        Some(Route::UnitOff)
    } else if snapshot.non_cooling_skipped {
        Some(Route::NonCooling)
    } else if snapshot.ems_disabled_fallthrough {
        Some(Route::EmsDisabledFallthrough)
    } else {
        Some(Route::OverrideApplied)
    }
}

fn entered_body_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) -> bool {
    let Some(ems_value) = snapshot.ems_supply_mass_flow_override_value_kg_per_s else {
        return false;
    };
    let Some(assigned_supply) = snapshot.assigned_supply_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(outdoor_air_before) = snapshot.outdoor_air_mass_flow_rate_before_override_kg_per_s
    else {
        return false;
    };
    let Some(supply_for_minimum) = snapshot.supply_mass_flow_rate_for_minimum_kg_per_s else {
        return false;
    };
    let Some(minimum) = snapshot.minimum_outdoor_air_mass_flow_rate_kg_per_s else {
        return false;
    };
    let Some(assigned_outdoor_air) = snapshot.assigned_outdoor_air_mass_flow_rate_kg_per_s else {
        return false;
    };

    snapshot.ems_supply_mass_flow_override_value_read
        && snapshot.supply_mass_flow_rate_override_assignment_performed
        && snapshot.outdoor_air_mass_flow_rate_for_minimum_read
        && snapshot.supply_mass_flow_rate_for_minimum_read
        && snapshot.source_shaped_two_argument_minimum_evaluated
        && snapshot.outdoor_air_mass_flow_rate_assignment_performed
        && assigned_supply.to_bits() == ems_value.to_bits()
        && supply_for_minimum.to_bits() == ems_value.to_bits()
        && minimum.to_bits() == source_min(outdoor_air_before, ems_value).to_bits()
        && assigned_outdoor_air.to_bits() == minimum.to_bits()
}

fn skipped_body_fields_are_exact(
    snapshot: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) -> bool {
    !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
        && !snapshot.ems_supply_mass_flow_override_value_read
        && snapshot
            .ems_supply_mass_flow_override_value_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_override_assignment_performed
        && snapshot.assigned_supply_mass_flow_rate_kg_per_s.is_none()
        && !snapshot.outdoor_air_mass_flow_rate_for_minimum_read
        && snapshot
            .outdoor_air_mass_flow_rate_before_override_kg_per_s
            .is_none()
        && !snapshot.supply_mass_flow_rate_for_minimum_read
        && snapshot
            .supply_mass_flow_rate_for_minimum_kg_per_s
            .is_none()
        && !snapshot.source_shaped_two_argument_minimum_evaluated
        && snapshot
            .minimum_outdoor_air_mass_flow_rate_kg_per_s
            .is_none()
        && !snapshot.outdoor_air_mass_flow_rate_assignment_performed
        && snapshot
            .assigned_outdoor_air_mass_flow_rate_kg_per_s
            .is_none()
}

#[inline]
fn source_min(left: f64, right: f64) -> f64 {
    if left < right { left } else { right }
}
