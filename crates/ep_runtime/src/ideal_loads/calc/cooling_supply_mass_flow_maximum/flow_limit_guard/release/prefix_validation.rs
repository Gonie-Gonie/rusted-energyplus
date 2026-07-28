use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
    PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
};

pub(super) fn flow_limit_guard_links_to_ems_override_body(
    guard: PurchasedAirCalcCoolingSupplyMassFlowLimitGuardSnapshot,
    body: PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) -> bool {
    guard.system == body.system
        && guard.parent_call_ordinal == body.parent_call_ordinal
        && guard.controlled_zone == body.controlled_zone
        && guard.unit_body_entered == body.unit_body_entered
        && guard.predecessor_cooling_body_entered == body.cooling_body_entered
        && guard.predecessor_ems_supply_mass_flow_override_body_entered
            == body.predecessor_ems_supply_mass_flow_override_body_entered
        && guard.predecessor_ems_supply_mass_flow_override_body_skipped == body.body_skipped
        && guard.predecessor_ems_disabled_fallthrough == body.ems_disabled_fallthrough
        && guard.unit_off_skipped == body.unit_off_skipped
        && guard.non_cooling_skipped == body.non_cooling_skipped
        && guard.cooling_body_entered == body.cooling_body_entered
}
