//! CP318 pure-transition characterization tests.

use ep_model::IdealLoadsAirSystemId;

use super::cooling_economizer_body_release_tests::body_release_fixture_with_cooling_demand;
use super::{
    PurchasedAirCalcCoolingEconomizerBodySnapshot, PurchasedAirCalcCoolingSensibleFlowInput,
    PurchasedAirCalcCoolingSensibleFlowRuntimeState, PurchasedAirCalcCoolingSensibleFlowSnapshot,
    advance_cooling_sensible_flow_state,
};

mod gate_tests;
mod skip_tests;
mod source_order_tests;

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(5);

fn characterize(
    predecessor: PurchasedAirCalcCoolingEconomizerBodySnapshot,
    input: PurchasedAirCalcCoolingSensibleFlowInput,
) -> (
    PurchasedAirCalcCoolingSensibleFlowSnapshot,
    PurchasedAirCalcCoolingSensibleFlowRuntimeState,
) {
    let mut state = PurchasedAirCalcCoolingSensibleFlowRuntimeState::new(SYSTEM);
    let snapshot = advance_cooling_sensible_flow_state(&mut state, predecessor, input);
    (snapshot, state)
}

fn predecessor(cooling_demand_w: f64) -> PurchasedAirCalcCoolingEconomizerBodySnapshot {
    let (mut runtime, system, condition) =
        body_release_fixture_with_cooling_demand(cooling_demand_w);
    super::advance_direct_no_oa_calc_cooling_economizer_body(&mut runtime, &system, condition)
        .expect("exact CP317 test predecessor")
}

fn active_predecessor() -> PurchasedAirCalcCoolingEconomizerBodySnapshot {
    predecessor(-1_000.0)
}

fn non_cooling_predecessor() -> PurchasedAirCalcCoolingEconomizerBodySnapshot {
    predecessor(1.0)
}

fn unit_off_predecessor() -> PurchasedAirCalcCoolingEconomizerBodySnapshot {
    let mut predecessor = non_cooling_predecessor();
    predecessor.unit_body_entered = false;
    predecessor.unit_off_skipped = true;
    predecessor.non_cooling_skipped = false;
    predecessor
}

fn base_input() -> PurchasedAirCalcCoolingSensibleFlowInput {
    PurchasedAirCalcCoolingSensibleFlowInput {
        cooling_on: true,
        zone_humidity_ratio: 0.008,
        minimum_cooling_supply_air_temperature_c: 13.0,
        zone_temperature_c: 22.0,
        zone_cooling_setpoint_load_w: -1_000.0,
    }
}

fn poison_input() -> PurchasedAirCalcCoolingSensibleFlowInput {
    PurchasedAirCalcCoolingSensibleFlowInput {
        cooling_on: false,
        zone_humidity_ratio: f64::NAN,
        minimum_cooling_supply_air_temperature_c: f64::NAN,
        zone_temperature_c: f64::NAN,
        zone_cooling_setpoint_load_w: f64::NAN,
    }
}

fn assert_bits(actual: Option<f64>, expected: f64) {
    assert_eq!(
        actual.expect("source site value").to_bits(),
        expected.to_bits()
    );
}
