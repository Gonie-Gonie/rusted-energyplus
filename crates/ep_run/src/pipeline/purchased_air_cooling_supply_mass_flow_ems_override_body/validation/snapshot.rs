//! Exact direct-lane shape checks for one CP324 body snapshot.

use ep_runtime::PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot;

pub(in crate::pipeline) fn snapshot_shape(
    snapshot: &PurchasedAirCalcCoolingSupplyMassFlowEmsOverrideBodySnapshot,
) -> bool {
    let source_sites_skipped = !snapshot.ems_supply_mass_flow_override_value_read
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
            .is_none();
    if snapshot.cooling_body_entered {
        snapshot.unit_body_entered
            && snapshot.predecessor_cooling_body_entered
            && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
            && snapshot.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough
            && !snapshot.unit_off_skipped
            && !snapshot.non_cooling_skipped
            && snapshot.body_skipped
            && snapshot.ems_disabled_fallthrough
            && source_sites_skipped
    } else {
        !snapshot.predecessor_cooling_body_entered
            && !snapshot.predecessor_ems_supply_mass_flow_override_body_entered
            && !snapshot.predecessor_ems_supply_mass_flow_override_guard_false_fallthrough
            && usize::from(snapshot.unit_off_skipped) + usize::from(snapshot.non_cooling_skipped)
                == 1
            && snapshot.unit_body_entered == snapshot.non_cooling_skipped
            && snapshot.body_skipped
            && !snapshot.ems_disabled_fallthrough
            && source_sites_skipped
    }
}
