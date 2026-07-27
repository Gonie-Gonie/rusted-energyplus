//! CP319 exact direct-release tests.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingSensibleFlowSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_dehumidification_flow,
    advance_direct_no_oa_calc_cooling_sensible_flow,
    purchased_air_calc_cooling_dehumidification_flow_lifecycle_summary,
};

mod corruption_tests;

#[test]
fn public_active_cooling_proves_none_control_and_skips_live_humidity_sites() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let snapshot =
        advance_direct_no_oa_calc_cooling_dehumidification_flow(&mut runtime, &system, predecessor)
            .expect("exact active CP319 release transition");

    assert!(super::cooling_dehumidification_flow_snapshot_is_exact_direct_release(snapshot));
    assert!(snapshot.cooling_body_entered);
    assert!(snapshot.supply_mass_flow_rate_for_dehumidification_reset_assigned);
    assert_eq!(snapshot.cooling_on, Some(true));
    assert!(snapshot.dehumidification_control_type_read);
    assert_eq!(
        snapshot.dehumidification_control_type,
        Some(ep_model::DehumidificationControlType::None)
    );
    assert_eq!(
        snapshot.dehumidification_control_type_humidistat,
        Some(false)
    );
    assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
    assert!(!snapshot.minimum_cooling_supply_air_humidity_ratio_read);
    assert!(!snapshot.zone_humidity_ratio_read);
    assert_eq!(
        snapshot
            .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .expect("positive-zero reset")
            .to_bits(),
        0.0_f64.to_bits()
    );

    let lifecycle =
        purchased_air_calc_cooling_dehumidification_flow_lifecycle_summary(&runtime, system.id)
            .expect("CP319 lifecycle");
    assert_eq!(lifecycle.state.transition_count, 1);
    assert_eq!(lifecycle.state.cooling_body_entry_count, 1);
    assert_eq!(
        lifecycle
            .state
            .dehumidification_control_type_fallthrough_count,
        1
    );
    assert_eq!(
        lifecycle
            .state
            .zone_dehumidifying_setpoint_moisture_demand_read_count,
        0
    );
    assert_eq!(lifecycle.state.latest, Some(snapshot));
}

#[test]
fn public_non_cooling_route_skips_the_entire_cp319_slice() {
    let (mut runtime, system, predecessor) = release_case(1.0);
    let snapshot =
        advance_direct_no_oa_calc_cooling_dehumidification_flow(&mut runtime, &system, predecessor)
            .expect("exact non-cooling CP319 release transition");

    assert!(super::cooling_dehumidification_flow_snapshot_is_exact_direct_release(snapshot));
    assert!(snapshot.non_cooling_skipped);
    assert!(!snapshot.cooling_body_entered);
    assert!(!snapshot.supply_mass_flow_rate_for_dehumidification_reset_assigned);
    assert!(!snapshot.cooling_on_read);
    assert!(!snapshot.dehumidification_control_type_read);
    assert!(!snapshot.zone_dehumidifying_setpoint_moisture_demand_read);
    assert!(
        snapshot
            .resulting_supply_mass_flow_rate_for_dehumidification_kg_per_s
            .is_none()
    );
}

pub(super) fn release_case(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    PurchasedAirCalcCoolingSensibleFlowSnapshot,
) {
    let (mut runtime, system, body, zone_state) =
        super::cooling_sensible_flow_release_tests::release_case(cooling_demand_w);
    let predecessor =
        advance_direct_no_oa_calc_cooling_sensible_flow(&mut runtime, &system, body, &zone_state)
            .expect("exact CP318 predecessor");
    (runtime, system, predecessor)
}
