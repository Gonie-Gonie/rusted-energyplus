use super::*;

use ep_model::{
    AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId,
    IdealLoadsFuelType, IdealLoadsLimit, NormalizedName, OutdoorAirEconomizerType,
};

const SYSTEM: IdealLoadsAirSystemId = IdealLoadsAirSystemId(0);

#[test]
fn positive_no_limit_fields_call_all_children_in_source_order() {
    let mut system = test_system();
    system.heating_limit = IdealLoadsLimit::NoLimit;
    system.cooling_limit = IdealLoadsLimit::NoLimit;
    let mut sized_limits = PurchasedAirSizedLimits::from_system(&system);
    let original = sized_limits;

    let outcome = size_purchased_air_direct_hard_sized_legacy_route(
        &system,
        &mut sized_limits,
        direct_context(),
    )
    .expect("clean direct hard-size route");

    assert_eq!(
        outcome.route,
        PurchasedAirHardSizeLegacyRoute::DirectHardSizedNoSizingRun
    );
    assert!(outcome.entry_fan_flags_cleared);
    assert_eq!(outcome.child_sizer_call_count(), 4);
    assert_eq!(outcome.characterized_report_record_count(), 6);
    assert_eq!(sized_limits, original);
    assert_eq!(outcome.sized_limits, original);

    let fields: Vec<_> = outcome.fields.into_iter().flatten().collect();
    assert_eq!(
        fields.iter().map(|field| field.field).collect::<Vec<_>>(),
        vec![
            PurchasedAirHardSizeField::MaximumHeatingAirFlowRate,
            PurchasedAirHardSizeField::MaximumSensibleHeatingCapacity,
            PurchasedAirHardSizeField::MaximumCoolingAirFlowRate,
            PurchasedAirHardSizeField::MaximumTotalCoolingCapacity,
        ]
    );
    assert_eq!(
        fields
            .iter()
            .map(|field| field.object_writeback)
            .collect::<Vec<_>>(),
        vec![true, false, true, true]
    );
    assert_eq!(fields[1].local_design_value, 5_000.0);
    assert_eq!(fields[1].outer_report_records, 2);
    assert_eq!(fields[1].child_sizing_label_unit, "m3/s");
    assert_eq!(fields[3].local_design_value, 0.0);
    assert_eq!(fields[3].outer_report_records, 0);
    assert_eq!(fields[3].child_sizing_label_unit, "m3/s");
}

#[test]
fn zero_and_blank_fields_skip_children_and_small_heat_capacity_outer_report() {
    let mut system = test_system();
    system.heating_limit = IdealLoadsLimit::NoLimit;
    system.cooling_limit = IdealLoadsLimit::NoLimit;
    system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(0.0));
    system.maximum_sensible_heating_capacity_w = Some(AutosizeOrNumber::Value(0.5));
    system.maximum_cooling_air_flow_rate_m3_per_s = None;
    system.maximum_total_cooling_capacity_w = Some(AutosizeOrNumber::Value(0.0));
    let mut sized_limits = PurchasedAirSizedLimits::from_system(&system);

    let outcome = size_purchased_air_direct_hard_sized_legacy_route(
        &system,
        &mut sized_limits,
        direct_context(),
    )
    .expect("zero and blank hard-size visits");
    let fields: Vec<_> = outcome.fields.into_iter().flatten().collect();

    assert_eq!(outcome.child_sizer_call_count(), 1);
    assert_eq!(outcome.characterized_report_record_count(), 1);
    assert!(!fields[0].child_sizer_called);
    assert!(fields[1].child_sizer_called);
    assert_eq!(fields[1].child_result, Some(0.5));
    assert_eq!(fields[1].local_design_value, 0.0);
    assert_eq!(fields[1].outer_report_records, 0);
    assert!(!fields[2].child_sizer_called);
    assert!(!fields[3].child_sizer_called);
}

#[test]
fn missing_current_zone_equipment_suppresses_the_entire_field_body() {
    let mut system = test_system();
    system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Autosize);
    system.design_specification_zonehvac_sizing_object_name =
        Some(NormalizedName::new("CUSTOM SIZING"));
    let mut sized_limits = PurchasedAirSizedLimits::from_system(&system);
    let original = sized_limits;

    let outcome = size_purchased_air_direct_hard_sized_legacy_route(
        &system,
        &mut sized_limits,
        PurchasedAirHardSizeLegacyContext {
            current_zone_equipment_index: 0,
            zone_sizing_run_done: true,
        },
    )
    .expect("source outer condition returns normally");

    assert_eq!(
        outcome.route,
        PurchasedAirHardSizeLegacyRoute::NoCurrentZoneEquipment
    );
    assert_eq!(outcome.fields, [None; 4]);
    assert_eq!(outcome.child_sizer_call_count(), 0);
    assert_eq!(sized_limits, original);
}

#[test]
fn unported_routes_and_unresolved_values_fail_closed() {
    let mut system = test_system();
    system.design_specification_zonehvac_sizing_object_name =
        Some(NormalizedName::new("CUSTOM SIZING"));
    assert_eq!(
        run_sizing(&system, direct_context()),
        Err(PurchasedAirHardSizeLegacyError::CustomZoneHvacSizingNotImplemented { system: SYSTEM })
    );

    system.design_specification_zonehvac_sizing_object_name = None;
    assert_eq!(
        run_sizing(
            &system,
            PurchasedAirHardSizeLegacyContext {
                current_zone_equipment_index: 1,
                zone_sizing_run_done: true,
            },
        ),
        Err(PurchasedAirHardSizeLegacyError::ZoneSizingRunNotImplemented { system: SYSTEM })
    );

    system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Autosize);
    assert_eq!(
        run_sizing(&system, direct_context()),
        Err(PurchasedAirHardSizeLegacyError::AutosizingNotImplemented {
            system: SYSTEM,
            field: PurchasedAirHardSizeField::MaximumHeatingAirFlowRate,
        })
    );

    system.maximum_heating_air_flow_rate_m3_per_s = Some(AutosizeOrNumber::Value(-1.0));
    assert_eq!(
        run_sizing(&system, direct_context()),
        Err(PurchasedAirHardSizeLegacyError::InvalidHardSize {
            system: SYSTEM,
            field: PurchasedAirHardSizeField::MaximumHeatingAirFlowRate,
        })
    );

    system.maximum_heating_air_flow_rate_m3_per_s = None;
    assert_eq!(
        run_sizing(&system, direct_context()),
        Err(PurchasedAirHardSizeLegacyError::MissingRequiredHardSize {
            system: SYSTEM,
            field: PurchasedAirHardSizeField::MaximumHeatingAirFlowRate,
        })
    );
}

fn run_sizing(
    system: &IdealLoadsAirSystem,
    context: PurchasedAirHardSizeLegacyContext,
) -> Result<PurchasedAirHardSizeLegacyOutcome, PurchasedAirHardSizeLegacyError> {
    let mut sized_limits = PurchasedAirSizedLimits::from_system(system);
    size_purchased_air_direct_hard_sized_legacy_route(system, &mut sized_limits, context)
}

const fn direct_context() -> PurchasedAirHardSizeLegacyContext {
    PurchasedAirHardSizeLegacyContext {
        current_zone_equipment_index: 1,
        zone_sizing_run_done: false,
    }
}

fn test_system() -> IdealLoadsAirSystem {
    IdealLoadsAirSystem {
        id: SYSTEM,
        name: NormalizedName::new("ZONE ONE IDEAL LOADS"),
        availability_schedule: None,
        zone_supply_air_node_name: NormalizedName::new("ZONE ONE INLET"),
        zone_exhaust_air_node_name: None,
        system_inlet_air_node_name: None,
        maximum_heating_supply_air_temperature_c: 50.0,
        minimum_cooling_supply_air_temperature_c: 13.0,
        maximum_heating_supply_air_humidity_ratio: 0.0156,
        minimum_cooling_supply_air_humidity_ratio: 0.0077,
        heating_limit: IdealLoadsLimit::LimitFlowRateAndCapacity,
        maximum_heating_air_flow_rate_m3_per_s: Some(AutosizeOrNumber::Value(0.5)),
        maximum_sensible_heating_capacity_w: Some(AutosizeOrNumber::Value(5_000.0)),
        cooling_limit: IdealLoadsLimit::LimitFlowRateAndCapacity,
        maximum_cooling_air_flow_rate_m3_per_s: Some(AutosizeOrNumber::Value(0.4)),
        maximum_total_cooling_capacity_w: Some(AutosizeOrNumber::Value(4_000.0)),
        heating_availability_schedule: None,
        cooling_availability_schedule: None,
        dehumidification_control_type: DehumidificationControlType::None,
        cooling_sensible_heat_ratio: 0.7,
        humidification_control_type: HumidificationControlType::None,
        design_specification_outdoor_air_object_name: None,
        outdoor_air_inlet_node_name: None,
        demand_controlled_ventilation_type: DemandControlledVentilationType::None,
        outdoor_air_economizer_type: OutdoorAirEconomizerType::NoEconomizer,
        heat_recovery_type: HeatRecoveryType::None,
        sensible_heat_recovery_effectiveness: 0.7,
        latent_heat_recovery_effectiveness: 0.65,
        design_specification_zonehvac_sizing_object_name: None,
        heating_fuel_efficiency_schedule: None,
        heating_fuel_type: IdealLoadsFuelType::DistrictHeatingWater,
        cooling_fuel_efficiency_schedule: None,
        cooling_fuel_type: IdealLoadsFuelType::DistrictCooling,
    }
}
