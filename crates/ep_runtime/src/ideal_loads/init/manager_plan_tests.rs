use ep_model::{
    AutoOrNumber, AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId,
    IdealLoadsFuelType, IdealLoadsLimit, InsideSurfaceConvectionAlgorithm, LoadDistributionScheme,
    NormalizedName, OutdoorAirEconomizerType, OutsideSurfaceConvectionAlgorithm, Point3,
    TypedModel, Zone, ZoneConvectionAlgorithm, ZoneEquipmentConnection, ZoneEquipmentConnectionId,
    ZoneEquipmentList, ZoneEquipmentListEntry, ZoneEquipmentListId, ZoneEquipmentObjectType,
    ZoneId,
};

use super::{
    PurchasedAirInitManagerPlan, PurchasedAirInitManagerPlanError, PurchasedAirInitManagerPlanRow,
};

#[test]
fn model_plan_preserves_non_sorted_system_declaration_order()
-> Result<(), PurchasedAirInitManagerPlanError> {
    let model = model_with_systems(&[7, 2, 9]);
    let plan = PurchasedAirInitManagerPlan::from_model(&model)?;
    assert_eq!(
        plan.system_order().collect::<Vec<_>>(),
        vec![
            IdealLoadsAirSystemId(7),
            IdealLoadsAirSystemId(2),
            IdealLoadsAirSystemId(9),
        ]
    );
    Ok(())
}

#[test]
fn model_plan_uses_first_matching_list_and_scans_past_earlier_entries()
-> Result<(), PurchasedAirInitManagerPlanError> {
    let mut model = model_with_systems(&[7]);
    model.zones = vec![zone(5), zone(2)];
    model.zone_equipment_lists = vec![
        equipment_list(0, &[IdealLoadsAirSystemId(7)]),
        equipment_list(1, &[IdealLoadsAirSystemId(7)]),
        equipment_list(8, &[IdealLoadsAirSystemId(99), IdealLoadsAirSystemId(7)]),
    ];
    model.zone_equipment_connections =
        vec![equipment_connection(0, 2, 1), equipment_connection(1, 5, 8)];
    let plan = PurchasedAirInitManagerPlan::from_model(&model)?;
    assert_eq!(
        plan.rows(),
        &[PurchasedAirInitManagerPlanRow {
            system: IdealLoadsAirSystemId(7),
            first_matching_equipment_list: Some(ZoneEquipmentListId(8)),
            return_plenum_active: false,
        }]
    );
    Ok(())
}

#[test]
fn model_plan_retains_missing_equipment_list_membership()
-> Result<(), PurchasedAirInitManagerPlanError> {
    let mut model = model_with_systems(&[4]);
    model.zones.push(zone(4));
    model.zone_equipment_lists = vec![equipment_list(3, &[IdealLoadsAirSystemId(4)])];
    let plan = PurchasedAirInitManagerPlan::from_model(&model)?;
    assert_eq!(plan.rows()[0].first_matching_equipment_list, None);
    Ok(())
}

#[test]
fn model_and_row_constructors_reject_duplicate_system_ids() {
    let model = model_with_systems(&[3, 3]);
    assert_eq!(
        PurchasedAirInitManagerPlan::from_model(&model),
        Err(PurchasedAirInitManagerPlanError::DuplicateSystemId {
            system: IdealLoadsAirSystemId(3),
        })
    );
    let duplicate_rows = vec![row(7, Some(2)), row(1, None), row(7, Some(4))];
    assert_eq!(
        PurchasedAirInitManagerPlan::try_from_rows(duplicate_rows),
        Err(PurchasedAirInitManagerPlanError::DuplicateSystemId {
            system: IdealLoadsAirSystemId(7),
        })
    );
}

#[test]
fn model_and_row_constructors_reject_any_active_return_plenum() {
    let mut model = model_with_systems(&[5, 2]);
    let inlet_name = model.ideal_loads_air_systems[1]
        .zone_supply_air_node_name
        .clone();
    model.ideal_loads_air_systems[1].system_inlet_air_node_name = Some(inlet_name);
    assert_eq!(
        PurchasedAirInitManagerPlan::from_model(&model),
        Err(PurchasedAirInitManagerPlanError::ReturnPlenumUnsupported {
            system: IdealLoadsAirSystemId(2),
        })
    );
    let active_rows = vec![
        row(5, None),
        PurchasedAirInitManagerPlanRow {
            system: IdealLoadsAirSystemId(2),
            first_matching_equipment_list: Some(ZoneEquipmentListId(6)),
            return_plenum_active: true,
        },
    ];
    assert_eq!(
        PurchasedAirInitManagerPlan::try_from_rows(active_rows),
        Err(PurchasedAirInitManagerPlanError::ReturnPlenumUnsupported {
            system: IdealLoadsAirSystemId(2),
        })
    );
}

fn model_with_systems(system_ids: &[u32]) -> TypedModel {
    TypedModel {
        ideal_loads_air_systems: system_ids
            .iter()
            .map(|id| test_system(IdealLoadsAirSystemId(*id)))
            .collect(),
        ..TypedModel::default()
    }
}

fn equipment_list(id: u32, systems: &[IdealLoadsAirSystemId]) -> ZoneEquipmentList {
    ZoneEquipmentList {
        id: ZoneEquipmentListId(id),
        name: NormalizedName(String::new()),
        load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
        equipment: systems
            .iter()
            .map(|system| ZoneEquipmentListEntry {
                object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
                ideal_loads_air_system: *system,
                cooling_sequence: 1,
                heating_or_no_load_sequence: 1,
                sequential_cooling_fraction_schedule: None,
                sequential_heating_fraction_schedule: None,
            })
            .collect(),
    }
}

fn equipment_connection(id: u32, zone: u32, equipment_list: u32) -> ZoneEquipmentConnection {
    ZoneEquipmentConnection {
        id: ZoneEquipmentConnectionId(id),
        zone: ZoneId(zone),
        equipment_list: ZoneEquipmentListId(equipment_list),
        zone_air_inlet_node_or_nodelist_name: None,
        zone_air_exhaust_node_or_nodelist_name: None,
        zone_air_node_name: NormalizedName(String::new()),
        zone_return_air_node_or_nodelist_name: None,
        zone_return_air_node_1_flow_rate_fraction_schedule: None,
        zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
    }
}

fn zone(id: u32) -> Zone {
    Zone {
        id: ZoneId(id),
        name: NormalizedName(String::new()),
        direction_of_relative_north_deg: 0.0,
        origin: Point3 {
            x_m: 0.0,
            y_m: 0.0,
            z_m: 0.0,
        },
        zone_type: 1,
        multiplier: 1,
        list_multiplier: 1,
        list_group: None,
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::AutoCalculate,
        floor_area: AutoOrNumber::AutoCalculate,
        inside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            InsideSurfaceConvectionAlgorithm::Tarp,
        ),
        outside_convection_algorithm: ZoneConvectionAlgorithm::Inherited(
            OutsideSurfaceConvectionAlgorithm::Doe2,
        ),
        is_part_of_total_floor_area: true,
        is_nominal_controlled: true,
        linked_outdoor_air_node: None,
        spaces: Vec::new(),
    }
}

fn test_system(id: IdealLoadsAirSystemId) -> IdealLoadsAirSystem {
    IdealLoadsAirSystem {
        id,
        name: NormalizedName(String::new()),
        availability_schedule: None,
        zone_supply_air_node_name: NormalizedName(String::new()),
        zone_exhaust_air_node_name: None,
        system_inlet_air_node_name: None,
        maximum_heating_supply_air_temperature_c: 50.0,
        minimum_cooling_supply_air_temperature_c: 13.0,
        maximum_heating_supply_air_humidity_ratio: 0.0156,
        minimum_cooling_supply_air_humidity_ratio: 0.0077,
        heating_limit: IdealLoadsLimit::NoLimit,
        maximum_heating_air_flow_rate_m3_per_s: None::<AutosizeOrNumber>,
        maximum_sensible_heating_capacity_w: None::<AutosizeOrNumber>,
        cooling_limit: IdealLoadsLimit::NoLimit,
        maximum_cooling_air_flow_rate_m3_per_s: None::<AutosizeOrNumber>,
        maximum_total_cooling_capacity_w: None::<AutosizeOrNumber>,
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

fn row(system: u32, equipment_list: Option<u32>) -> PurchasedAirInitManagerPlanRow {
    PurchasedAirInitManagerPlanRow {
        system: IdealLoadsAirSystemId(system),
        first_matching_equipment_list: equipment_list.map(ZoneEquipmentListId),
        return_plenum_active: false,
    }
}
