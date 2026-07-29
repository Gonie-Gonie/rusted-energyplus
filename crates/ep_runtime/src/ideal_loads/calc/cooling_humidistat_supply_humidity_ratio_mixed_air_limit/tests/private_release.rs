use super::public_release::completed_cp362_case;
use super::super::{
    private_humidistat_counterfactual_from_direct_release,
    private_humidistat_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::calc::cooling_positive_supply_temperature_mixed_air_limit::source_shaped_two_argument_minimum;

const DEMAND: f64 = -0.001;
const ZONE_HUMIDITY: f64 = 0.008;

#[test]
fn private_h_bridge_reads_only_cp329_owner_and_cp361_local_bits() {
    let (runtime, system, direct) = completed_cp362_case().unwrap();
    let unit = runtime.units.get(&system.id).unwrap();
    let counterfactual = private_humidistat_counterfactual_from_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        DEMAND,
        ZONE_HUMIDITY,
    )
    .unwrap();
    let mixed = unit
        .calc_cooling_mixed_air_call
        .latest
        .unwrap()
        .mixed_air_humidity_ratio
        .unwrap();
    let local = counterfactual
        .supply_humidity_ratio_for_dehumidification_before_mixed_air_limit
        .unwrap();
    let expected = source_shaped_two_argument_minimum(mixed, local);
    assert_eq!(
        counterfactual.mixed_air_humidity_ratio.unwrap().to_bits(),
        mixed.to_bits()
    );
    assert_eq!(
        counterfactual.resulting_supply_humidity_ratio.unwrap().to_bits(),
        expected.to_bits()
    );
    assert!(private_humidistat_counterfactual_links_to_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        counterfactual,
        DEMAND,
        ZONE_HUMIDITY,
    ));
    assert!(!private_humidistat_counterfactual_links_to_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        counterfactual,
        DEMAND,
        ZONE_HUMIDITY + 0.01,
    ));
}

#[test]
fn cp329_owner_latest_and_witness_corruption_rejects_private_bridge() {
    let (runtime, system, direct) = completed_cp362_case().unwrap();

    let mut corrupt_latest = runtime.clone();
    corrupt_latest
        .units
        .get_mut(&system.id)
        .unwrap()
        .calc_cooling_mixed_air_call
        .latest
        .as_mut()
        .unwrap()
        .parent_call_ordinal += 1;
    assert!(
        private_humidistat_counterfactual_from_direct_release(
            &corrupt_latest,
            corrupt_latest.units.get(&system.id).unwrap(),
            &system,
            direct,
            DEMAND,
            ZONE_HUMIDITY,
        )
        .is_none()
    );

    let mut corrupt_witness = runtime;
    let mut forged = corrupt_witness
        .cooling_mixed_air_call_latest_witness(system.id)
        .unwrap();
    forged.controlled_zone = ep_model::ZoneId(999);
    corrupt_witness.set_cooling_mixed_air_call_latest_witness(system.id, forged);
    assert!(
        private_humidistat_counterfactual_from_direct_release(
            &corrupt_witness,
            corrupt_witness.units.get(&system.id).unwrap(),
            &system,
            direct,
            DEMAND,
            ZONE_HUMIDITY,
        )
        .is_none()
    );
}
