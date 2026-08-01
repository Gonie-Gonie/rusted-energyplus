use ep_model::{
    DehumidificationControlType, HumidificationControlType, IdealLoadsAirSystemId, ZoneId,
};

use super::super::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitActiveOperands as ActiveOperands,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationMaximumLimitRuntimeState as State,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_maximum_limit_state as advance,
};
use crate::ideal_loads::calc::{
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationControlHumidistatGuardRuntimeState as Cp370State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationDehumidificationControlHumidistatOrNoneGuardRuntimeState as Cp371State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentActiveInput as Cp372Input,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationMoistureDemandAssignmentRuntimeState as Cp372State,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentActiveOperands as Cp373Operands,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentRuntimeState as Cp373State,
    advance_cooling_supply_humidity_ratio_humidification_control_humidistat_guard_state as advance_cp370,
    advance_cooling_supply_humidity_ratio_humidification_dehumidification_control_humidistat_or_none_guard_state as advance_cp371,
    advance_cooling_supply_humidity_ratio_humidification_moisture_demand_assignment_state as advance_cp372,
    advance_cooling_supply_humidity_ratio_humidification_supply_humidity_ratio_for_humidification_assignment_state as advance_cp373,
};
use crate::ideal_loads::{
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
    PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationHeatingAvailabilityGuardSnapshot as Cp369Snapshot,
    PurchasedAirCalcCoolingSupplyHumidityRatioHumidificationSupplyHumidityRatioForHumidificationAssignmentSnapshot as Predecessor,
};

mod ieee;
mod overflow;
mod release;
mod routes;

#[test]
fn cp374_source_boundary_and_dependency_order_are_exact() {
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2250",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_FIRST_EXCLUDED_SOURCE,
        "EnergyPlus 26.1 PurchasedAirManager.cc:2251",
    );
    assert_eq!(
        PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_SUPPLY_HUMIDITY_RATIO_FOR_HUMIDIFICATION_MAXIMUM_LIMIT_SOURCE_ORDER,
        &[
            "read-local-supply-humidity-ratio-for-humidification-for-maximum-limit-minimum",
            "read-purchased-air-maximum-heating-supply-air-humidity-ratio-for-humidification-maximum-limit-minimum",
            "apply-source-shaped-two-argument-minimum-for-humidification-maximum-limit",
            "assign-local-supply-humidity-ratio-for-humidification-for-maximum-limit",
        ],
    );
}

fn active_cp373(
    selector: DehumidificationControlType,
    demand: f64,
    flow: f64,
    zone_humidity_ratio: f64,
) -> Predecessor {
    cp373(
        selector,
        HumidificationControlType::Humidistat,
        Some(demand),
        Some(Cp373Operands {
            supply_mass_flow_rate_kg_per_s: flow,
            zone_node_humidity_ratio: zone_humidity_ratio,
        }),
    )
}

fn skipped_cp373() -> Predecessor {
    cp373(
        DehumidificationControlType::None,
        HumidificationControlType::None,
        None,
        None,
    )
}

fn cp373(
    selector: DehumidificationControlType,
    humidification_control: HumidificationControlType,
    demand: Option<f64>,
    operands: Option<Cp373Operands>,
) -> Predecessor {
    cp373_from_cp369(
        active_cp369(selector),
        selector,
        humidification_control,
        demand,
        operands,
    )
}

fn cp373_from_cp369(
    cp369: Cp369Snapshot,
    selector: DehumidificationControlType,
    humidification_control: HumidificationControlType,
    demand: Option<f64>,
    operands: Option<Cp373Operands>,
) -> Predecessor {
    let mut cp370_state = Cp370State::new(cp369.system);
    let cp370 = advance_cp370(&mut cp370_state, cp369, humidification_control)
        .expect("valid CP370 fixture");
    let mut cp371_state = Cp371State::new(cp370.system);
    let cp371 = advance_cp371(&mut cp371_state, cp370, selector).expect("valid CP371 fixture");
    let mut cp372_state = Cp372State::new(cp371.system);
    let cp372 = advance_cp372(
        &mut cp372_state,
        cp371,
        demand.map(|zone_humidifying_setpoint_moisture_demand_kg_per_s| Cp372Input {
            zone_humidifying_setpoint_moisture_demand_kg_per_s,
        }),
    )
    .expect("valid CP372 fixture");
    let mut cp373_state = Cp373State::new(cp372.system);
    advance_cp373(&mut cp373_state, cp372, operands).expect("valid CP373 fixture")
}

fn inherited_skip_cp369(route: usize) -> Cp369Snapshot {
    let mut snapshot = active_cp369(DehumidificationControlType::None);
    snapshot.predecessor_dehumidification_control_type = None;
    snapshot.predecessor_dehumidification_control_none_case_completed_skip = false;
    snapshot.predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip = false;
    snapshot.predecessor_dehumidification_control_humidistat_case_completed_skip = false;
    snapshot.predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip = false;
    snapshot.dehumidification_control_none_case_completed_skip = false;
    snapshot.dehumidification_control_constant_sensible_heat_ratio_case_completed_skip = false;
    snapshot.dehumidification_control_humidistat_case_completed_skip = false;
    snapshot.dehumidification_control_constant_supply_humidity_ratio_case_completed_skip = false;
    snapshot.heating_on_read = false;
    snapshot.heating_on = None;
    snapshot.cooling_supply_humidity_ratio_humidification_body_entered = false;
    snapshot.heating_on_guard_false_fallthrough = false;
    match route {
        0 => {
            snapshot.unit_off_skipped = true;
            snapshot.unit_body_entered = false;
            snapshot.predecessor_cooling_body_entered = false;
            snapshot.predecessor_no_outdoor_air_fallback_entered = false;
            snapshot.predecessor_positive_supply_mass_flow_body_entered = false;
        }
        1 => {
            snapshot.non_cooling_skipped = true;
            snapshot.predecessor_cooling_body_entered = false;
            snapshot.predecessor_no_outdoor_air_fallback_entered = false;
            snapshot.predecessor_positive_supply_mass_flow_body_entered = false;
        }
        2 => {
            snapshot.predecessor_positive_supply_mass_flow_body_entered = false;
            snapshot.positive_guard_false_fallthrough_skipped = true;
        }
        _ => unreachable!("only U/N/P inherited CP369 routes"),
    }
    snapshot
}

fn heating_guard_false_cp369() -> Cp369Snapshot {
    let mut snapshot = active_cp369(DehumidificationControlType::None);
    snapshot.heating_on = Some(false);
    snapshot.cooling_supply_humidity_ratio_humidification_body_entered = false;
    snapshot.heating_on_guard_false_fallthrough = true;
    snapshot
}

fn active_cp369(selector: DehumidificationControlType) -> Cp369Snapshot {
    Cp369Snapshot {
        source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE,
        first_excluded_source: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_FIRST_EXCLUDED_SOURCE,
        source_order: PURCHASED_AIR_CALC_COOLING_SUPPLY_HUMIDITY_RATIO_HUMIDIFICATION_HEATING_AVAILABILITY_GUARD_SOURCE_ORDER,
        system: IdealLoadsAirSystemId(0),
        parent_call_ordinal: 1,
        controlled_zone: ZoneId(0),
        unit_body_entered: true,
        predecessor_cooling_body_entered: true,
        predecessor_no_outdoor_air_fallback_entered: true,
        predecessor_positive_supply_mass_flow_body_entered: true,
        unit_off_skipped: false,
        non_cooling_skipped: false,
        positive_guard_false_fallthrough_skipped: false,
        predecessor_dehumidification_control_type: Some(selector),
        predecessor_dehumidification_control_none_case_completed_skip:
            selector == DehumidificationControlType::None,
        predecessor_dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            selector == DehumidificationControlType::ConstantSensibleHeatRatio,
        predecessor_dehumidification_control_humidistat_case_completed_skip:
            selector == DehumidificationControlType::Humidistat,
        predecessor_dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            selector == DehumidificationControlType::ConstantSupplyHumidityRatio,
        predecessor_dehumidification_control_default_supply_humidity_ratio_case_exited_via_break:
            false,
        dehumidification_control_none_case_completed_skip:
            selector == DehumidificationControlType::None,
        dehumidification_control_constant_sensible_heat_ratio_case_completed_skip:
            selector == DehumidificationControlType::ConstantSensibleHeatRatio,
        dehumidification_control_humidistat_case_completed_skip:
            selector == DehumidificationControlType::Humidistat,
        dehumidification_control_constant_supply_humidity_ratio_case_completed_skip:
            selector == DehumidificationControlType::ConstantSupplyHumidityRatio,
        heating_on_read: true,
        heating_on: Some(true),
        cooling_supply_humidity_ratio_humidification_body_entered: true,
        heating_on_guard_false_fallthrough: false,
    }
}
