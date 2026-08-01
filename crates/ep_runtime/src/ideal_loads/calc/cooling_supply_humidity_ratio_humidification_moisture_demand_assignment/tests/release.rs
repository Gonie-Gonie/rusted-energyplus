//! CP372 public release and pre-sampled-scalar bridge tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentError as Error,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment,
    cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_links_to_direct_release,
};
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::completed_cp370_case_for_cp372_test;
use crate::ideal_loads::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshot as Predecessor,
    PurchasedAirRuntimeState,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
};
#[test]
fn direct_release_is_numeric_free_and_private_bridge_copies_pre_sampled_scalar_bit_exact() {
    let (mut runtime, system, predecessor) = completed_cp371_case().expect("CP371 fixture");
    let direct = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP372 direct release");

    assert!(
        cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_snapshot_is_exact_direct_release(
            direct,
        )
    );
    assert!(!direct.humidification_moisture_demand_assignment_executed);
    assert!(!direct.zone_humidifying_setpoint_moisture_demand_read);
    assert_eq!(
        direct.resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s,
        None
    );
    let state = &runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.humidification_control_guard_false_fallthrough_count, 1);
    assert_eq!(state.source_site_execution_count, 0);

    for value in [-0.0, f64::from_bits(0x7ff8_0000_0000_0372)] {
        let unit = runtime.units.get(&system.id).expect("known unit");
        let private = private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_from_direct_release(
            &runtime,
            unit,
            &system,
            direct,
            value,
        )
        .expect("canonical private CP372");
        assert!(private.predecessor_dehumidification_control_body_entered);
        assert!(private.humidification_moisture_demand_assignment_executed);
        assert!(private.zone_humidifying_setpoint_moisture_demand_read);
        assert!(private.zone_humidifying_setpoint_moisture_demand_assigned);
        assert_eq!(
            private
                .resulting_zone_humidifying_setpoint_moisture_demand_kg_per_s
                .expect("private result")
                .to_bits(),
            value.to_bits()
        );
        assert!(
            private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_links_to_direct_release(
                &runtime,
                unit,
                &system,
                direct,
                private,
                value,
            )
        );
    }
}

#[test]
fn supplied_private_scalar_is_bit_specific_without_mutating_direct_state() {
    let (mut runtime, system, predecessor) = completed_cp371_case().expect("CP371 fixture");
    let direct = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(
        &mut runtime,
        &system,
        predecessor,
    )
    .expect("CP372 direct release");
    let sampled = 0.001_f64;
    let different = f64::from_bits(sampled.to_bits() + 1);
    let unit = runtime.units.get(&system.id).expect("known unit");
    let private = private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_from_direct_release(
        &runtime,
        unit,
        &system,
        direct,
        sampled,
    )
    .expect("canonical private CP372");
    let before = runtime.clone();
    assert!(
        !private_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_counterfactual_links_to_direct_release(
            &runtime,
            unit,
            &system,
            direct,
            private,
            different,
        )
    );
    assert_eq!(runtime, before);
}

#[test]
fn supplied_cp371_drift_is_rejected_transactionally() {
    let (mut runtime, system, mut predecessor) = completed_cp371_case().expect("CP371 fixture");
    predecessor.dehumidification_control_body_entered = true;
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(
            &mut runtime,
            &system,
            predecessor,
        ),
        Err(Error::CoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardSnapshotMismatch { .. })
    ));
    assert_eq!(runtime, before);
}

fn completed_cp371_case() -> Option<(
    PurchasedAirRuntimeState,
    ep_model::IdealLoadsAirSystem,
    Predecessor,
)> {
    let (mut runtime, system, cp370) = completed_cp370_case_for_cp372_test()?;
    let cp371 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
        &mut runtime,
        &system,
        cp370,
    )
    .ok()?;
    Some((runtime, system, cp371))
}
