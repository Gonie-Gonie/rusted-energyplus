//! Exact direct-lane shape checks for one CP323 guard snapshot.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot;

pub(in crate::pipeline) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideGuardSnapshot,
) -> bool {
    if snapshot.cooling_body_entered {
        snapshot.unit_body_entered
            && snapshot.predecessor_cooling_body_entered
            && !snapshot.unit_off_skipped
            && !snapshot.non_cooling_skipped
            && snapshot.ems_supply_mass_flow_override_flag_read
            && snapshot.ems_supply_mass_flow_override_enabled == Some(false)
            && snapshot.ems_supply_mass_flow_override_guard_evaluated
            && !snapshot.ems_supply_mass_flow_override_body_entered
            && snapshot.ems_supply_mass_flow_override_guard_false_fallthrough
    } else {
        snapshot.predecessor_cooling_body_entered == snapshot.cooling_body_entered
            && usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
                == 1
            && snapshot.unit_body_entered == snapshot.non_cooling_skipped
            && !snapshot.ems_supply_mass_flow_override_flag_read
            && snapshot.ems_supply_mass_flow_override_enabled.is_none()
            && !snapshot.ems_supply_mass_flow_override_guard_evaluated
            && !snapshot.ems_supply_mass_flow_override_body_entered
            && !snapshot.ems_supply_mass_flow_override_guard_false_fallthrough
    }
}
