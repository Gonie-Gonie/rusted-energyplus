//! CP319 pure-transition characterization tests.

use ep_model::{DehumidificationControlType, IdealLoadsAirSystemId};

use super::{
    PurchasedAirCalcCoolingDehumidificationFlowInput,
    PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingSensibleFlowSnapshot, advance_cooling_dehumidification_flow_state,
    advance_direct_no_oa_calc_cooling_sensible_flow,
};

mod gate_tests;
mod skip_tests;
mod source_order_tests;

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(5);

fn characterize(
    predecessor: PurchasedAirCalcCoolingSensibleFlowSnapshot,
    input: PurchasedAirCalcCoolingDehumidificationFlowInput,
) -> (
    PurchasedAirCalcCoolingDehumidificationFlowSnapshot,
    PurchasedAirCalcCoolingDehumidificationFlowRuntimeState,
) {
    let mut state = PurchasedAirCalcCoolingDehumidificationFlowRuntimeState::new(SYSTEM);
    let snapshot = advance_cooling_dehumidification_flow_state(&mut state, predecessor, input);
    (snapshot, state)
}

fn predecessor(cooling_demand_w: f64) -> PurchasedAirCalcCoolingSensibleFlowSnapshot {
    let (mut runtime, system, body, zone_state) =
        super::cooling_sensible_flow_release_tests::release_case(cooling_demand_w);
    advance_direct_no_oa_calc_cooling_sensible_flow(&mut runtime, &system, body, &zone_state)
        .expect("exact CP318 test predecessor")
}

fn active_predecessor() -> PurchasedAirCalcCoolingSensibleFlowSnapshot {
    predecessor(-1_000.0)
}

fn non_cooling_predecessor() -> PurchasedAirCalcCoolingSensibleFlowSnapshot {
    predecessor(1.0)
}

fn unit_off_predecessor() -> PurchasedAirCalcCoolingSensibleFlowSnapshot {
    let mut predecessor = non_cooling_predecessor();
    predecessor.unit_body_entered = false;
    predecessor.unit_off_skipped = true;
    predecessor.non_cooling_skipped = false;
    predecessor
}

fn base_input() -> PurchasedAirCalcCoolingDehumidificationFlowInput {
    PurchasedAirCalcCoolingDehumidificationFlowInput {
        cooling_on: true,
        dehumidification_control_type: DehumidificationControlType::Humidistat,
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s: -0.0002,
        minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air: 0.009,
        zone_humidity_ratio_kg_water_per_kg_dry_air: 0.012,
    }
}

fn poison_input(
    control: DehumidificationControlType,
) -> PurchasedAirCalcCoolingDehumidificationFlowInput {
    PurchasedAirCalcCoolingDehumidificationFlowInput {
        cooling_on: false,
        dehumidification_control_type: control,
        zone_dehumidifying_setpoint_moisture_demand_kg_per_s: f64::NAN,
        minimum_cooling_supply_air_humidity_ratio_kg_water_per_kg_dry_air: f64::NAN,
        zone_humidity_ratio_kg_water_per_kg_dry_air: f64::NAN,
    }
}

fn assert_bits(actual: Option<f64>, expected: f64) {
    assert_eq!(
        actual.expect("source site value").to_bits(),
        expected.to_bits()
    );
}
