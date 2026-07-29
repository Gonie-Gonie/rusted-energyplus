use super::super::{
    PurchasedAirCalcCoolingConstantShrSupplyHumidityRatioOverdryingLimitRuntimeState as State,
    advance_cooling_constant_shr_supply_humidity_ratio_overdrying_limit_state as advance,
    release::active_operands_from_retained_owners_for_test,
};
use super::{completed_cp353_case, private_active_cp353_predecessor};

#[test]
fn private_active_operands_resolve_cp345_humidity_and_cp353_values() {
    for capacity_limit in [false, true] {
        let completed = completed_cp353_case(-100_000.0, 1.0, capacity_limit);
        assert!(completed.is_some());
        let Some((runtime, system, direct_cp352, _)) = completed else {
            return;
        };
        let private =
            private_active_cp353_predecessor(direct_cp352, &runtime, &system)
                .expect("valid private CP353 counterfactual");
        let unit = runtime.units.get(&system.id).expect("selected unit");
        let operands =
            active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
                .expect("CP354 retained owners");
        let humidity_owner = unit
            .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .latest
            .and_then(|snapshot| snapshot.assigned_supply_humidity_ratio)
            .expect("CP345 humidity owner");
        assert_eq!(
            operands
                .supply_humidity_ratio_before_overdrying_limit
                .to_bits(),
            humidity_owner.to_bits()
        );
        assert_eq!(
            operands.supply_temperature_c.to_bits(),
            private
                .supply_temperature_c
                .expect("CP353 temperature")
                .to_bits()
        );
        assert_eq!(
            operands.supply_enthalpy_j_per_kg.to_bits(),
            private
                .resulting_supply_enthalpy_j_per_kg
                .expect("CP353 enthalpy")
                .to_bits()
        );
        let mut state = State::new(system.id);
        assert!(advance(&mut state, private, Some(operands)).is_some());
    }
}

#[test]
fn coordinated_cp345_owner_forge_is_rejected() {
    let completed = completed_cp353_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct_cp352, _)) = completed else {
        return;
    };
    let private =
        private_active_cp353_predecessor(direct_cp352, &runtime, &system)
            .expect("valid private CP353 counterfactual");
    let forged = {
        let unit = runtime.units.get_mut(&system.id).expect("selected unit");
        let latest = unit
            .calc_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment
            .latest
            .as_mut()
            .expect("CP345 latest");
        latest.mixed_air_humidity_ratio = latest
            .mixed_air_humidity_ratio
            .map(next_bits);
        latest.assigned_supply_humidity_ratio = latest
            .assigned_supply_humidity_ratio
            .map(next_bits);
        *latest
    };
    runtime
        .set_cooling_positive_supply_post_capacity_limit_humidity_ratio_mixed_air_assignment_latest_witness(
            system.id,
            forged,
        );
    let unit = runtime.units.get(&system.id).expect("selected unit");
    assert!(
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
            .is_none()
    );
}

#[test]
fn coordinated_direct_and_private_cp353_forgeries_are_rejected() {
    let completed = completed_cp353_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((mut runtime, system, direct_cp352, _)) = completed else {
        return;
    };
    let private =
        private_active_cp353_predecessor(direct_cp352, &runtime, &system)
            .expect("valid private CP353 counterfactual");
    let forged_direct = {
        let unit = runtime.units.get_mut(&system.id).expect("selected unit");
        let latest = unit
            .calc_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit
            .latest
            .as_mut()
            .expect("CP353 direct latest");
        latest.source_order = &[];
        *latest
    };
    runtime
        .set_cooling_positive_supply_post_capacity_limit_dehumidification_control_constant_sensible_heat_ratio_overdrying_limit_latest_witness(
            system.id,
            forged_direct,
        );
    let unit = runtime.units.get(&system.id).expect("selected unit");
    assert!(
        active_operands_from_retained_owners_for_test(&runtime, unit, &system, private)
            .is_none()
    );

    let completed = completed_cp353_case(-100_000.0, 1.0, false);
    assert!(completed.is_some());
    let Some((runtime, system, direct_cp352, _)) = completed else {
        return;
    };
    let mut forged_private =
        private_active_cp353_predecessor(direct_cp352, &runtime, &system)
            .expect("valid private CP353 counterfactual");
    for value in [
        &mut forged_private.maximum_supply_enthalpy_j_per_kg,
        &mut forged_private.assigned_supply_enthalpy_j_per_kg,
        &mut forged_private.resulting_supply_enthalpy_j_per_kg,
    ] {
        *value = value.map(next_bits);
    }
    let unit = runtime.units.get(&system.id).expect("selected unit");
    assert!(
        active_operands_from_retained_owners_for_test(
            &runtime,
            unit,
            &system,
            forged_private,
        )
        .is_none()
    );
}

fn next_bits(value: f64) -> f64 {
    f64::from_bits(value.to_bits().wrapping_add(1))
}
