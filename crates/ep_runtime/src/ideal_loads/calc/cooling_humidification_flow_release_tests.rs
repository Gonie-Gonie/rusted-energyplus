//! CP320 direct-release tests nested below the pure-transition suite.

use ep_model::IdealLoadsAirSystem;

use crate::ideal_loads::{
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot, PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_dehumidification_flow,
    advance_direct_no_oa_calc_cooling_humidification_flow,
    purchased_air_calc_cooling_humidification_flow_lifecycle_summary,
};

#[test]
fn active_direct_route_uses_retained_heating_on_and_skips_live_services() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    let snapshot =
        advance_direct_no_oa_calc_cooling_humidification_flow(&mut runtime, &system, predecessor)
            .expect("CP320");
    assert!(super::super::cooling_humidification_flow_snapshot_is_exact_direct_release(snapshot));
    assert_eq!(snapshot.heating_on, Some(true));
    assert_eq!(
        snapshot.humidification_control_type,
        Some(ep_model::HumidificationControlType::None)
    );
    assert!(!snapshot.dehumidification_control_type_first_read);
    assert!(!snapshot.zone_humidifying_setpoint_moisture_demand_read);
    assert_eq!(
        snapshot
            .resulting_supply_mass_flow_rate_for_humidification_kg_per_s
            .expect("reset")
            .to_bits(),
        0.0_f64.to_bits()
    );
    let summary =
        purchased_air_calc_cooling_humidification_flow_lifecycle_summary(&runtime, system.id)
            .expect("summary");
    assert_eq!(summary.state.transition_count, 1);
    assert_eq!(summary.state.latest, Some(snapshot));
}

#[test]
fn non_cooling_release_skips_all_twenty_six_sites() {
    let (mut runtime, system, predecessor) = release_case(1.0);
    let snapshot =
        advance_direct_no_oa_calc_cooling_humidification_flow(&mut runtime, &system, predecessor)
            .expect("CP320");
    assert!(snapshot.non_cooling_skipped);
    assert!(!snapshot.supply_mass_flow_rate_for_humidification_reset_assigned);
    assert!(!snapshot.heating_on_read);
}

#[test]
fn failure_is_transactional() {
    let (mut runtime, mut system, predecessor) = release_case(-1_000.0);
    let before = runtime.clone();
    system.humidification_control_type = ep_model::HumidificationControlType::Humidistat;
    assert!(
        advance_direct_no_oa_calc_cooling_humidification_flow(&mut runtime, &system, predecessor,)
            .is_err()
    );
    assert_eq!(runtime, before);
}

#[test]
fn corrupted_pending_state_fails_without_partial_mutation() {
    let (mut runtime, system, predecessor) = release_case(-1_000.0);
    runtime
        .units
        .get_mut(&system.id)
        .expect("unit")
        .calc_cooling_humidification_flow
        .transition_count = 1;
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_humidification_flow(&mut runtime, &system, predecessor,)
            .is_err()
    );
    assert_eq!(runtime, before);
}

pub(super) fn release_case(
    cooling_demand_w: f64,
) -> (
    PurchasedAirRuntimeState,
    IdealLoadsAirSystem,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
) {
    let (mut runtime, system, sensible) =
        super::super::cooling_dehumidification_flow_release_tests::release_case(cooling_demand_w);
    let predecessor =
        advance_direct_no_oa_calc_cooling_dehumidification_flow(&mut runtime, &system, sensible)
            .expect("CP319");
    (runtime, system, predecessor)
}
