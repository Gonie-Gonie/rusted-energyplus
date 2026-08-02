//! CP395 canonical `PsyWFnTdbH` and binary64 preservation tests.

use super::*;
use crate::psychrometrics::energyplus_psy_w_fn_tdb_h;
use ep_model::DehumidificationControlType as D;

type State = PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatSupplyHumidityRatioAssignmentRuntimeState;

#[test]
fn active_humidistat_assignment_uses_canonical_inverse_for_ieee_edges() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0395);
    let pole_c = -2_500_940.0 / 1_858.95;
    for (temperature, enthalpy) in [
        (0.0, -1.0),
        (0.0, 1.0),
        (0.0, -0.0),
        (0.0, nan),
        (20.0, f64::NEG_INFINITY),
        (0.0, f64::INFINITY),
        (pole_c, 0.0),
    ] {
        let mut predecessor = active_predecessor();
        predecessor.predecessor_cp393_resulting_supply_temperature_c = Some(temperature);
        predecessor.resulting_supply_temperature_c = Some(temperature);
        predecessor.predecessor_cp393_resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
        predecessor.resulting_supply_enthalpy_j_per_kg = Some(enthalpy);
        assert!(
            crate::ideal_loads::calc::cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_case_entry_snapshot_is_exact(
                predecessor,
            )
        );

        let mut state = State::new(predecessor.system);
        let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state(
            &mut state,
            predecessor,
        )
        .expect("IEEE CP395 active route");
        let expected = energyplus_psy_w_fn_tdb_h(temperature, enthalpy);

        assert_eq!(
            snapshot.supply_temperature_c.map(f64::to_bits),
            Some(temperature.to_bits()),
        );
        assert_eq!(
            snapshot.supply_enthalpy_j_per_kg.map(f64::to_bits),
            Some(enthalpy.to_bits()),
        );
        for value in [
            snapshot.psychrometric_supply_humidity_ratio,
            snapshot.assigned_supply_humidity_ratio,
            snapshot.resulting_supply_humidity_ratio,
        ] {
            assert_eq!(value.map(f64::to_bits), Some(expected.to_bits()));
        }
        assert!(
            cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshot_is_exact(
                snapshot,
            )
        );
    }
}

#[test]
fn canonical_floor_signed_zero_nan_and_infinity_bits_remain_locked() {
    let nan = f64::from_bits(0x7ff8_0000_0000_0395);
    assert_eq!(
        energyplus_psy_w_fn_tdb_h(0.0, -1.0).to_bits(),
        1.0e-5f64.to_bits(),
    );
    let positive_subfloor = energyplus_psy_w_fn_tdb_h(0.0, 1.0);
    assert!(positive_subfloor > 0.0);
    assert!(positive_subfloor < 1.0e-5);
    assert_eq!(
        energyplus_psy_w_fn_tdb_h(0.0, -0.0).to_bits(),
        (-0.0f64).to_bits(),
    );
    assert_eq!(energyplus_psy_w_fn_tdb_h(0.0, nan).to_bits(), nan.to_bits(),);
    assert_eq!(
        energyplus_psy_w_fn_tdb_h(20.0, f64::NEG_INFINITY).to_bits(),
        1.0e-5f64.to_bits(),
    );
    assert_eq!(energyplus_psy_w_fn_tdb_h(0.0, f64::INFINITY), f64::INFINITY,);
    assert_eq!((1.0e-5f64).to_bits(), 0x3ee4_f8b5_88e3_68f1);
}

#[test]
fn binary64_snapshot_comparison_distinguishes_nan_payloads() {
    let mut predecessor = active_predecessor();
    let nan = f64::from_bits(0x7ff8_0000_0000_0395);
    predecessor.predecessor_cp393_resulting_supply_enthalpy_j_per_kg = Some(nan);
    predecessor.resulting_supply_enthalpy_j_per_kg = Some(nan);
    let mut state = State::new(predecessor.system);
    let snapshot = advance_cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_state(
        &mut state,
        predecessor,
    )
    .expect("NaN CP395");
    let mut forged = snapshot;
    forged.psychrometric_supply_humidity_ratio = Some(f64::from_bits(
        snapshot
            .psychrometric_supply_humidity_ratio
            .unwrap()
            .to_bits()
            ^ 1,
    ));
    assert!(
        !cooling_post_saturation_capacity_limit_dehumidification_control_humidistat_supply_humidity_ratio_assignment_snapshots_match_bit_exact(
            snapshot,
            forged,
        )
    );
}

fn active_predecessor(
) -> crate::ideal_loads::PurchasedAirCalcCoolingPostSaturationCapacityLimitDehumidificationControlHumidistatCaseEntrySnapshot
{
    fixtures::chain(3, 1, true, Some(D::Humidistat), 1, 0.7, 18.0, 1.0).cp394
}
