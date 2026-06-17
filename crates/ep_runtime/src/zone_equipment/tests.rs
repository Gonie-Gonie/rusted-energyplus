use super::*;
use ep_model::{
    AutoOrNumber, AutosizeOrNumber, DehumidificationControlType, DemandControlledVentilationType,
    HeatRecoveryType, HumidificationControlType, IdealLoadsAirSystem, IdealLoadsAirSystemId,
    IdealLoadsFuelType, IdealLoadsLimit, LoadDistributionScheme, Node, NodeId, NodeList,
    NodeListId, NormalizedName, OutdoorAirEconomizerType, Point3, SimulationModel, TypedModel,
    Zone, ZoneEquipmentConnection, ZoneEquipmentConnectionId, ZoneEquipmentList,
    ZoneEquipmentListEntry, ZoneEquipmentListId, ZoneEquipmentObjectType, ZoneId,
};

#[test]
fn zone_equipment_dispatch_validation_accepts_single_equipment_path() {
    let model = single_ideal_loads_model("Zone Inlet", "Zone Inlet");

    let validation = validate_ideal_loads_zone_equipment_dispatch(&model, IdealLoadsAirSystemId(0));

    assert!(validation.is_dispatchable());
    assert!(validation.is_conformance_candidate());
    assert_eq!(validation.dispatch_status_label(), "pass");
    assert_eq!(validation.conformance_candidate_status_label(), "pass");
    assert_eq!(validation.scope_label(), "single-zone-single-equipment");
    assert_eq!(validation.zone, Some(ZoneId(0)));
    assert_eq!(validation.equipment_list, Some(ZoneEquipmentListId(0)));
    assert_eq!(validation.cooling_sequence, Some(1));
    assert_eq!(validation.heating_or_no_load_sequence, Some(1));
    assert_eq!(validation.supply_nodes, vec![NodeId(0)]);
    assert_eq!(validation.zone_inlet_nodes, vec![NodeId(0)]);
    assert!(validation.issue_codes().is_empty());
    assert!(validation.warning_codes().is_empty());
}

#[test]
fn zone_sys_energy_demand_preserves_source_field_sign_conventions() {
    let demand = ZoneSysEnergyDemand::sensible_only(ZoneId(0), 1200.0, -900.0);

    assert_eq!(
        ZONE_SYS_ENERGY_DEMAND_SOURCE_FILE,
        "src/EnergyPlus/DataZoneEnergyDemands.hh"
    );
    assert_eq!(ZONE_SYS_ENERGY_DEMAND_SOURCE_STRUCT, "ZoneSysEnergyDemand");
    assert_eq!(
        ZONE_SYS_ENERGY_DEMAND_HEATING_FIELD,
        "RemainingOutputReqToHeatSP"
    );
    assert_eq!(
        ZONE_SYS_ENERGY_DEMAND_COOLING_FIELD,
        "RemainingOutputReqToCoolSP"
    );
    assert!(ZONE_SYS_ENERGY_DEMAND_HEATING_SIGN_CONVENTION.contains("positive W"));
    assert!(ZONE_SYS_ENERGY_DEMAND_COOLING_SIGN_CONVENTION.contains("negative W"));
    assert_eq!(demand.remaining_output_req_to_heat_sp_w, 1200.0);
    assert_eq!(demand.remaining_output_req_to_cool_sp_w, -900.0);
    assert!(demand.has_inactive_moisture_demand());
}

#[test]
fn zone_equipment_dispatch_validation_rejects_supply_node_outside_zone_inlet_list() {
    let model = single_ideal_loads_model("Ideal Loads Supply", "Zone Inlet");

    let validation = validate_ideal_loads_zone_equipment_dispatch(&model, IdealLoadsAirSystemId(0));

    assert!(!validation.is_dispatchable());
    assert_eq!(
        validation.issues,
        vec![IdealLoadsZoneEquipmentDispatchIssue::SupplyNodeNotInZoneInletList]
    );
    assert_eq!(
        validation.issue_codes(),
        vec!["supply_node_not_in_zone_inlet_list"]
    );
}

#[test]
fn zone_equipment_dispatch_validation_marks_multiple_equipment_diagnostic_only() {
    let mut typed = single_ideal_loads_typed_model("Zone Inlet", "Zone Inlet");
    typed.nodes.push(Node {
        id: NodeId(3),
        name: NormalizedName::new("Second Zone Inlet"),
    });
    typed.node_names.insert("Second Zone Inlet", NodeId(3));
    typed.ideal_loads_air_systems.push(ideal_loads_system(
        IdealLoadsAirSystemId(1),
        "Second Ideal Loads",
        "Second Zone Inlet",
    ));
    typed.zone_equipment_lists[0]
        .equipment
        .push(ZoneEquipmentListEntry {
            object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
            ideal_loads_air_system: IdealLoadsAirSystemId(1),
            cooling_sequence: 2,
            heating_or_no_load_sequence: 2,
            sequential_cooling_fraction_schedule: None,
            sequential_heating_fraction_schedule: None,
        });

    let model = SimulationModel::from_typed(typed);
    let validation = validate_ideal_loads_zone_equipment_dispatch(&model, IdealLoadsAirSystemId(0));

    assert!(validation.is_dispatchable());
    assert!(!validation.is_conformance_candidate());
    assert_eq!(
        validation.warnings,
        vec![IdealLoadsZoneEquipmentDispatchWarning::MultipleZoneEquipmentDiagnosticOnly]
    );
    assert_eq!(
        validation.warning_codes(),
        vec!["multiple_zone_equipment_diagnostic_only"]
    );
    assert_eq!(
        validation.conformance_candidate_status_label(),
        "diagnostic-only"
    );
}

#[test]
fn zone_equipment_dispatch_validation_rejects_sequence_ambiguity() {
    let mut typed = single_ideal_loads_typed_model("Zone Inlet", "Zone Inlet");
    typed.nodes.push(Node {
        id: NodeId(3),
        name: NormalizedName::new("Second Zone Inlet"),
    });
    typed.node_names.insert("Second Zone Inlet", NodeId(3));
    typed.ideal_loads_air_systems.push(ideal_loads_system(
        IdealLoadsAirSystemId(1),
        "Second Ideal Loads",
        "Second Zone Inlet",
    ));
    typed.zone_equipment_lists[0]
        .equipment
        .push(ZoneEquipmentListEntry {
            object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
            ideal_loads_air_system: IdealLoadsAirSystemId(1),
            cooling_sequence: 1,
            heating_or_no_load_sequence: 1,
            sequential_cooling_fraction_schedule: None,
            sequential_heating_fraction_schedule: None,
        });

    let model = SimulationModel::from_typed(typed);
    let validation = validate_ideal_loads_zone_equipment_dispatch(&model, IdealLoadsAirSystemId(0));

    assert!(!validation.is_dispatchable());
    assert!(
        validation
            .issues
            .contains(&IdealLoadsZoneEquipmentDispatchIssue::SequenceAmbiguity)
    );
}

fn single_ideal_loads_model(system_supply: &str, connection_inlet: &str) -> SimulationModel {
    SimulationModel::from_typed(single_ideal_loads_typed_model(
        system_supply,
        connection_inlet,
    ))
}

fn single_ideal_loads_typed_model(system_supply: &str, connection_inlet: &str) -> TypedModel {
    let mut typed = TypedModel::default();
    typed.zones.push(Zone {
        id: ZoneId(0),
        name: NormalizedName::new("Zone One"),
        direction_of_relative_north_deg: 0.0,
        origin: Point3 {
            x_m: 0.0,
            y_m: 0.0,
            z_m: 0.0,
        },
        zone_type: 1,
        multiplier: 1,
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::AutoCalculate,
    });
    for (id, name) in [
        (NodeId(0), "Zone Inlet"),
        (NodeId(1), "Ideal Loads Supply"),
        (NodeId(2), "Zone Air Node"),
    ] {
        typed.nodes.push(Node {
            id,
            name: NormalizedName::new(name),
        });
        typed.node_names.insert(name, id);
    }
    typed.node_lists.push(NodeList {
        id: NodeListId(0),
        name: NormalizedName::new("Zone Inlet List"),
        nodes: vec![NodeId(0)],
    });
    typed
        .node_list_names
        .insert("Zone Inlet List", NodeListId(0));
    typed.ideal_loads_air_systems.push(ideal_loads_system(
        IdealLoadsAirSystemId(0),
        "Zone Ideal Loads",
        system_supply,
    ));
    typed.zone_equipment_lists.push(ZoneEquipmentList {
        id: ZoneEquipmentListId(0),
        name: NormalizedName::new("Zone Equipment"),
        load_distribution_scheme: LoadDistributionScheme::SequentialLoad,
        equipment: vec![ZoneEquipmentListEntry {
            object_type: ZoneEquipmentObjectType::IdealLoadsAirSystem,
            ideal_loads_air_system: IdealLoadsAirSystemId(0),
            cooling_sequence: 1,
            heating_or_no_load_sequence: 1,
            sequential_cooling_fraction_schedule: None,
            sequential_heating_fraction_schedule: None,
        }],
    });
    typed
        .zone_equipment_connections
        .push(ZoneEquipmentConnection {
            id: ZoneEquipmentConnectionId(0),
            zone: ZoneId(0),
            equipment_list: ZoneEquipmentListId(0),
            zone_air_inlet_node_or_nodelist_name: Some(NormalizedName::new(connection_inlet)),
            zone_air_exhaust_node_or_nodelist_name: None,
            zone_air_node_name: NormalizedName::new("Zone Air Node"),
            zone_return_air_node_or_nodelist_name: None,
            zone_return_air_node_1_flow_rate_fraction_schedule: None,
            zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name: None,
        });
    typed
}

fn ideal_loads_system(
    id: IdealLoadsAirSystemId,
    name: &str,
    supply_node_name: &str,
) -> IdealLoadsAirSystem {
    IdealLoadsAirSystem {
        id,
        name: NormalizedName::new(name),
        availability_schedule: None,
        zone_supply_air_node_name: NormalizedName::new(supply_node_name),
        zone_exhaust_air_node_name: None,
        system_inlet_air_node_name: None,
        maximum_heating_supply_air_temperature_c: 50.0,
        minimum_cooling_supply_air_temperature_c: 13.0,
        maximum_heating_supply_air_humidity_ratio: 0.0156,
        minimum_cooling_supply_air_humidity_ratio: 0.0077,
        heating_limit: IdealLoadsLimit::NoLimit,
        maximum_heating_air_flow_rate_m3_per_s: Some(AutosizeOrNumber::Value(0.25)),
        maximum_sensible_heating_capacity_w: None,
        cooling_limit: IdealLoadsLimit::NoLimit,
        maximum_cooling_air_flow_rate_m3_per_s: None,
        maximum_total_cooling_capacity_w: None,
        heating_availability_schedule: None,
        cooling_availability_schedule: None,
        dehumidification_control_type: DehumidificationControlType::ConstantSensibleHeatRatio,
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
