use super::{RunResultState, RuntimeClass, SupportStatus, assess_support};
use crate::{PartialRunPolicy, RunMode, RunOutputFormat, TraceLevel};
use ep_compiler::compile_raw_model;
use ep_raw_model::parse_epjson_str;

#[test]
fn simple_one_zone_model_is_supported() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Material:NoMass": {"R13": {"thermal_resistance": 2.29}},
                "Construction": {"Wall": {"outside_layer": "R13"}},
                "BuildingSurface:Detailed": {
                    "Wall One": {
                        "surface_type": "Wall",
                        "construction_name": "Wall",
                        "zone_name": "Zone One",
                        "outside_boundary_condition": "Outdoors",
                        "vertices": [
                            {"vertex_x_coordinate": 0, "vertex_y_coordinate": 0, "vertex_z_coordinate": 0},
                            {"vertex_x_coordinate": 1, "vertex_y_coordinate": 0, "vertex_z_coordinate": 0},
                            {"vertex_x_coordinate": 1, "vertex_y_coordinate": 0, "vertex_z_coordinate": 1},
                            {"vertex_x_coordinate": 0, "vertex_y_coordinate": 0, "vertex_z_coordinate": 1}
                        ]
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert_eq!(assessment.status, SupportStatus::SupportedCompatibility);
    assert_eq!(
        assessment.runtime_class,
        RuntimeClass::OneZoneHeatBalanceCompatibility
    );
    assert_eq!(
        assessment.run_result_state,
        RunResultState::SupportedCompatibilityRun
    );
    assert_eq!(
        assessment.matched_capability_ids,
        vec!["official_1zone_uncontrolled_declared_heat_balance"]
    );
    assert_eq!(assessment.matched_capabilities.len(), 1);
    assert_eq!(assessment.matched_capabilities[0].domain, "heat_balance");
    assert_eq!(
        assessment.matched_capabilities[0].evidence_cases,
        vec!["official_1zone_uncontrolled_dynamic_conformance_candidate_001"]
    );
    assert!(assessment.failed_capability_ids.is_empty());
    assert!(assessment.capability_registry_loaded);
    Ok(())
}

#[test]
fn missing_registry_capability_blocks_runtime_selection() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}}
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let (status, runtime_class, matched_capability_ids, missing_capability_ids) =
        super::runtime_boundaries::runtime_status_for_typed_model(
            result.model.as_ref(),
            &crate::support_registry::CapabilityRegistrySpec::default(),
        );

    assert_eq!(status, SupportStatus::Unsupported);
    assert_eq!(runtime_class, RuntimeClass::None);
    assert!(matched_capability_ids.is_empty());
    assert_eq!(
        missing_capability_ids,
        vec!["official_1zone_uncontrolled_declared_heat_balance"]
    );
    Ok(())
}
#[test]
fn output_objects_use_partial_rule_from_registry() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Output:Variable": {
                    "Zone Mean Air Temperature": {
                        "key_value": "*",
                        "variable_name": "Zone Mean Air Temperature",
                        "reporting_frequency": "Hourly"
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert!(
        assessment
            .ignored_raw_only_objects
            .iter()
            .any(|entry| entry.object_type == "Output:Variable"
                && entry.status == "ignored_reporting_objects")
    );
    Ok(())
}

#[test]
fn hvac_air_loop_uses_unsupported_rule_from_registry() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "AirLoopHVAC": {"Main Air Loop": {}}
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert_eq!(assessment.status, SupportStatus::Unsupported);
    assert!(
        assessment
            .unsupported_objects
            .iter()
            .any(|entry| entry.object_type == "AirLoopHVAC"
                && entry.note == "Broad HVAC air-loop semantics are not ported.")
    );
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedHVACObject"
            && diagnostic.object_type.as_deref() == Some("AirLoopHVAC")
    }));
    Ok(())
}

#[test]
fn ideal_loads_no_oa_branch_matches_registry_capability() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Schedule:Constant": {
                    "Control Type": {"hourly_value": 4},
                    "Heating Setpoint": {"hourly_value": 21},
                    "Cooling Setpoint": {"hourly_value": 24}
                },
                "ThermostatSetpoint:DualSetpoint": {
                    "Dual Setpoints": {
                        "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
                        "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
                    }
                },
                "ZoneControl:Thermostat": {
                    "Zone Thermostat": {
                        "zone_or_zonelist_name": "Zone One",
                        "control_type_schedule_name": "Control Type",
                        "control_1_object_type": "ThermostatSetpoint:DualSetpoint",
                        "control_1_name": "Dual Setpoints"
                    }
                },
                "NodeList": {
                    "Zone Inlets": {
                        "nodes": [{"node_name": "Zone One Inlet"}]
                    }
                },
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads": {
                        "zone_supply_air_node_name": "Zone Inlets",
                        "dehumidification_control_type": "None",
                        "humidification_control_type": "None"
                    }
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone One": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_inlet_node_or_nodelist_name": "Zone Inlets",
                        "zone_air_node_name": "Zone One Air Node",
                        "zone_return_air_node_or_nodelist_name": "Zone One Return"
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Diagnostic,
        PartialRunPolicy::Allow,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert_eq!(assessment.status, SupportStatus::SupportedCompatibility);
    assert_eq!(
        assessment.run_result_state,
        RunResultState::SupportedCompatibilityRun
    );
    assert_eq!(
        assessment.runtime_class,
        RuntimeClass::IdealLoadsNoOaSensibleCompatibility
    );
    assert_eq!(
        assessment.matched_capability_ids,
        vec!["ideal_loads_no_oa_sensible"]
    );
    assert_eq!(assessment.matched_capabilities[0].domain, "ideal_loads");
    assert!(
        assessment.matched_capabilities[0]
            .evidence_cases
            .contains(&"ideal_loads_no_oa_sensible_conformance_001".to_string())
    );
    assert!(assessment.failed_capability_ids.is_empty());
    Ok(())
}

#[test]
fn ideal_loads_constant_supply_humidity_branch_matches_registry_capability()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Schedule:Constant": {
                    "Control Type": {"hourly_value": 4},
                    "Heating Setpoint": {"hourly_value": 21},
                    "Cooling Setpoint": {"hourly_value": 24}
                },
                "ThermostatSetpoint:DualSetpoint": {
                    "Dual Setpoints": {
                        "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
                        "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
                    }
                },
                "ZoneControl:Thermostat": {
                    "Zone Thermostat": {
                        "zone_or_zonelist_name": "Zone One",
                        "control_type_schedule_name": "Control Type",
                        "control_1_object_type": "ThermostatSetpoint:DualSetpoint",
                        "control_1_name": "Dual Setpoints"
                    }
                },
                "NodeList": {
                    "Zone Inlets": {
                        "nodes": [{"node_name": "Zone One Inlet"}]
                    }
                },
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads": {
                        "zone_supply_air_node_name": "Zone Inlets",
                        "dehumidification_control_type": "ConstantSupplyHumidityRatio",
                        "humidification_control_type": "None",
                        "minimum_cooling_supply_air_humidity_ratio": 0.0077
                    }
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone One": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_inlet_node_or_nodelist_name": "Zone Inlets",
                        "zone_air_node_name": "Zone One Air Node",
                        "zone_return_air_node_or_nodelist_name": "Zone One Return"
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert_eq!(assessment.status, SupportStatus::SupportedCompatibility);
    assert_eq!(
        assessment.run_result_state,
        RunResultState::SupportedCompatibilityRun
    );
    assert_eq!(
        assessment.runtime_class,
        RuntimeClass::IdealLoadsHumiditySelectedBranchesCompatibility
    );
    assert_eq!(
        assessment.matched_capability_ids,
        vec!["ideal_loads_humidity_selected_branches"]
    );
    assert_eq!(assessment.matched_capabilities[0].domain, "ideal_loads");
    assert!(assessment.matched_capabilities[0].evidence_cases.contains(
        &"ideal_loads_constant_supply_humidity_cooling_conformance_candidate_001".to_string()
    ));
    assert!(assessment.failed_capability_ids.is_empty());
    Ok(())
}
#[test]
fn ideal_loads_dual_humidity_controls_are_not_selected_branch_compatibility()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
                "Version": {"Version 1": {"version_identifier": "26.1"}},
                "Zone": {"Zone One": {"volume": 100}},
                "Schedule:Constant": {
                    "Control Type": {"hourly_value": 4},
                    "Heating Setpoint": {"hourly_value": 21},
                    "Cooling Setpoint": {"hourly_value": 24}
                },
                "ThermostatSetpoint:DualSetpoint": {
                    "Dual Setpoints": {
                        "heating_setpoint_temperature_schedule_name": "Heating Setpoint",
                        "cooling_setpoint_temperature_schedule_name": "Cooling Setpoint"
                    }
                },
                "ZoneControl:Thermostat": {
                    "Zone Thermostat": {
                        "zone_or_zonelist_name": "Zone One",
                        "control_type_schedule_name": "Control Type",
                        "control_1_object_type": "ThermostatSetpoint:DualSetpoint",
                        "control_1_name": "Dual Setpoints"
                    }
                },
                "NodeList": {
                    "Zone Inlets": {
                        "nodes": [{"node_name": "Zone One Inlet"}]
                    }
                },
                "ZoneHVAC:IdealLoadsAirSystem": {
                    "Zone Ideal Loads": {
                        "zone_supply_air_node_name": "Zone Inlets",
                        "dehumidification_control_type": "ConstantSupplyHumidityRatio",
                        "humidification_control_type": "ConstantSupplyHumidityRatio"
                    }
                },
                "ZoneHVAC:EquipmentList": {
                    "Zone Equipment": {
                        "equipment": [
                            {
                                "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
                                "zone_equipment_name": "Zone Ideal Loads",
                                "zone_equipment_cooling_sequence": 1,
                                "zone_equipment_heating_or_no_load_sequence": 1
                            }
                        ]
                    }
                },
                "ZoneHVAC:EquipmentConnections": {
                    "Zone One": {
                        "zone_name": "Zone One",
                        "zone_conditioning_equipment_list_name": "Zone Equipment",
                        "zone_air_inlet_node_or_nodelist_name": "Zone Inlets",
                        "zone_air_node_name": "Zone One Air Node",
                        "zone_return_air_node_or_nodelist_name": "Zone One Return"
                    }
                }
            }"#,
    )?;
    let result = compile_raw_model(&raw);
    let assessment = assess_support(
        &raw,
        &result.report,
        result.model.as_ref(),
        RunMode::Compatibility,
        PartialRunPolicy::Deny,
        RunOutputFormat::RustNative,
        TraceLevel::Normal,
    );

    assert_eq!(assessment.status, SupportStatus::Unsupported);
    assert_eq!(assessment.run_result_state, RunResultState::RunBlocked);
    assert_eq!(assessment.runtime_class, RuntimeClass::None);
    assert!(assessment.diagnostics.diagnostics.iter().any(|diagnostic| {
        diagnostic.code == "UnsupportedAlgorithm"
            && diagnostic.message.contains("unsupported feature flags")
    }));
    Ok(())
}
#[test]
fn diagnostic_only_support_is_blocked_outside_diagnostic_mode() {
    assert_eq!(
        RunResultState::from_support_status(
            SupportStatus::SupportedDiagnosticOnly,
            RunMode::Compatibility,
            PartialRunPolicy::Deny
        ),
        RunResultState::RunBlocked
    );
    assert_eq!(
        RunResultState::from_support_status(
            SupportStatus::SupportedDiagnosticOnly,
            RunMode::Diagnostic,
            PartialRunPolicy::Allow
        ),
        RunResultState::PartialSupportedRun
    );
    assert_eq!(
        RunResultState::from_support_status(
            SupportStatus::SupportedDiagnosticOnly,
            RunMode::Diagnostic,
            PartialRunPolicy::Deny
        ),
        RunResultState::RunBlocked
    );
}
