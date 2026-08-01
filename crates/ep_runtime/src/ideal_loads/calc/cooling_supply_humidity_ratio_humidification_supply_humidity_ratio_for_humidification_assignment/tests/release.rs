//! CP373 public direct release and canonical private selected-None tests.

use super::super::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentError as Error,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment,
    cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release,
    private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_links_to_direct_release,
};
use ep_model::HumidificationControlType;
use crate::ideal_loads::calc::cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard::completed_cp370_case_for_cp372_test;
use crate::ideal_loads::{
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard,
    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment,
};

#[test]
fn cp373_direct_is_numeric_free_and_private_none_preserves_ieee_bits_transactionally() {
    let (mut runtime, system, cp370) =
        completed_cp370_case_for_cp372_test().expect("CP370 fixture");
    let cp371 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
        &mut runtime,
        &system,
        cp370,
    )
    .expect("CP371 direct");
    let cp372 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp371,
    )
    .expect("CP372 direct");
    let direct = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(
        &mut runtime,
        &system,
        cp372,
    )
    .expect("CP373 direct");

    assert!(
        cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_snapshot_is_exact_direct_release(
            direct,
        )
    );
    assert_eq!(direct.supply_mass_flow_rate_kg_per_s, None);
    assert_eq!(direct.zone_node_humidity_ratio, None);
    assert_eq!(
        direct.resulting_supply_humidity_ratio_for_humidification,
        None
    );
    let state = &runtime
        .units
        .get(&system.id)
        .expect("known unit")
        .calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment;
    assert_eq!(state.transition_count, 1);
    assert_eq!(state.humidification_control_guard_false_fallthrough_count, 1);
    assert_eq!(state.source_site_execution_count, 0);

    for (demand, zone_humidity) in [
        (-0.0, f64::from_bits(1)),
        (f64::from_bits(0x7ff8_0000_0000_0373), -0.0),
    ] {
        let before = runtime.clone();
        let unit = runtime.units.get(&system.id).expect("known unit");
        let private = private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_from_direct_release(
            &runtime,
            unit,
            &system,
            direct,
            demand,
            zone_humidity,
        )
        .expect("canonical private selected-None CP373");
        let flow = private.supply_mass_flow_rate_kg_per_s.expect("CP330 flow");
        let quotient = demand / flow;
        let expected = quotient + zone_humidity;
        assert_eq!(
            private
                .moisture_demand_derived_supply_humidity_ratio
                .expect("quotient")
                .to_bits(),
            quotient.to_bits()
        );
        assert_eq!(
            private
                .resulting_supply_humidity_ratio_for_humidification
                .expect("result")
                .to_bits(),
            expected.to_bits()
        );
        assert!(
            private_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_counterfactual_links_to_direct_release(
                &runtime,
                unit,
                &system,
                direct,
                private,
                demand,
                zone_humidity,
            )
        );
        assert_eq!(runtime, before);
    }
}

#[test]
fn cp373_public_release_rejects_forgery_wrong_control_and_replay_transactionally() {
    let (mut runtime, mut system, cp370) =
        completed_cp370_case_for_cp372_test().expect("CP370 fixture");
    let cp371 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard(
        &mut runtime,
        &system,
        cp370,
    )
    .expect("CP371 direct");
    let cp372 = advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment(
        &mut runtime,
        &system,
        cp371,
    )
    .expect("CP372 direct");

    let mut forged = cp372;
    forged.parent_call_ordinal = forged.parent_call_ordinal.saturating_add(1);
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(
            &mut runtime,
            &system,
            forged,
        ),
        Err(Error::CoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentSnapshotMismatch { .. })
    ));
    assert_eq!(runtime, before);

    system.humidification_control_type = HumidificationControlType::Humidistat;
    let before = runtime.clone();
    assert!(matches!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(
            &mut runtime,
            &system,
            cp372,
        ),
        Err(Error::SystemOutsideDirectSubset { .. })
    ));
    assert_eq!(runtime, before);
    system.humidification_control_type = HumidificationControlType::None;

    advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(
        &mut runtime,
        &system,
        cp372,
    )
    .expect("first CP373 direct release");
    let before = runtime.clone();
    assert!(
        advance_direct_no_oa_calc_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment(
            &mut runtime,
            &system,
            cp372,
        )
        .is_err()
    );
    assert_eq!(runtime, before);
}
