//! CP394 control-only binary64 carrier tests.

use super::*;
use ep_model::DehumidificationControlType as D;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntryRuntimeState;

#[test]
fn compressed_snapshot_preserves_arbitrary_carrier_bits_without_numeric_gates() {
    let chain = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    );
    let mut state = State::new(chain.cp393.system);
    let mut snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state(
        &mut state,
        chain.cp393,
    )
    .expect("carrier-complete CP394");

    for (predecessor, resulting, bits) in [
        (
            &mut snapshot.predecessor_cp393_resulting_supply_humidity_ratio,
            &mut snapshot.resulting_supply_humidity_ratio,
            0x7ff8_0000_0000_0394,
        ),
        (
            &mut snapshot.predecessor_cp393_resulting_supply_enthalpy_j_per_kg,
            &mut snapshot.resulting_supply_enthalpy_j_per_kg,
            f64::NEG_INFINITY.to_bits(),
        ),
        (
            &mut snapshot.predecessor_cp393_resulting_supply_temperature_c,
            &mut snapshot.resulting_supply_temperature_c,
            (-0.0f64).to_bits(),
        ),
    ] {
        *predecessor = Some(f64::from_bits(bits));
        *resulting = Some(f64::from_bits(bits));
    }
    assert!(cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_snapshot_is_exact(snapshot));
}

#[test]
fn binary64_snapshot_comparison_distinguishes_nan_payloads() {
    let chain = fixtures::chain(
        3,
        1,
        true,
        Some(D::ConstantSensibleHeatRatio),
        1,
        0.7,
        18.0,
        1.0,
    );
    let mut state = State::new(chain.cp393.system);
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_state(
        &mut state,
        chain.cp393,
    )
    .expect("CP394");
    let mut forged = snapshot;
    forged.resulting_supply_humidity_ratio = Some(f64::from_bits(0x7ff8_0000_0000_0394));
    assert!(!cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_snapshots_match_bit_exact(snapshot, forged));
}
