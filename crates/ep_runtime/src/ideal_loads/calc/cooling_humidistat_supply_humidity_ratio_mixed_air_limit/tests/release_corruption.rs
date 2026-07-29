use super::super::{
    advance_direct_no_oa_calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit as advance,
    private_humidistat_counterfactual_from_direct_release,
};
use super::public_release::{completed_cp361_case, completed_cp362_case};

const DEMAND: f64 = -0.001;
const ZONE_HUMIDITY: f64 = 0.008;

#[test]
fn coordinated_owner_predecessor_and_witness_forgeries_reject_transactionally() {
    let (mut owner_runtime, owner_system, direct) = completed_cp362_case().unwrap();
    let forged_owner = {
        let latest = owner_runtime
            .units
            .get_mut(&owner_system.id)
            .unwrap()
            .calc_cooling_mixed_air_call
            .latest
            .as_mut()
            .unwrap();
        latest.mixed_air_humidity_ratio = latest.mixed_air_humidity_ratio.map(next_bits);
        *latest
    };
    owner_runtime.set_cooling_mixed_air_call_latest_witness(owner_system.id, forged_owner);
    let before = owner_runtime.clone();
    assert!(
        private_humidistat_counterfactual_from_direct_release(
            &owner_runtime,
            owner_runtime.units.get(&owner_system.id).unwrap(),
            &owner_system,
            direct,
            DEMAND,
            ZONE_HUMIDITY,
        )
        .is_none()
    );
    assert_eq!(owner_runtime, before);

    let (mut predecessor_runtime, predecessor_system, _) = completed_cp361_case().unwrap();
    let forged_predecessor = {
        let latest = predecessor_runtime
            .units
            .get_mut(&predecessor_system.id)
            .unwrap()
            .calc_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit
            .latest
            .as_mut()
            .unwrap();
        latest.source_order = &[];
        *latest
    };
    predecessor_runtime
        .set_cooling_humidistat_supply_humidity_ratio_for_dehumidification_minimum_limit_latest_witness(
            predecessor_system.id,
            forged_predecessor,
        );
    let before = predecessor_runtime.clone();
    assert!(
        advance(
            &mut predecessor_runtime,
            &predecessor_system,
            forged_predecessor,
        )
        .is_err()
    );
    assert_eq!(predecessor_runtime, before);

    let (mut direct_runtime, direct_system, _) = completed_cp362_case().unwrap();
    let forged_direct = {
        let latest = direct_runtime
            .units
            .get_mut(&direct_system.id)
            .unwrap()
            .calc_cooling_humidistat_supply_humidity_ratio_mixed_air_limit
            .latest
            .as_mut()
            .unwrap();
        latest.source_order = &[];
        *latest
    };
    direct_runtime.set_cooling_humidistat_supply_humidity_ratio_mixed_air_limit_latest_witness(
        direct_system.id,
        forged_direct,
    );
    let before = direct_runtime.clone();
    assert!(
        private_humidistat_counterfactual_from_direct_release(
            &direct_runtime,
            direct_runtime.units.get(&direct_system.id).unwrap(),
            &direct_system,
            forged_direct,
            DEMAND,
            ZONE_HUMIDITY,
        )
        .is_none()
    );
    assert_eq!(direct_runtime, before);
}

fn next_bits(value: f64) -> f64 {
    f64::from_bits(value.to_bits().wrapping_add(1))
}
