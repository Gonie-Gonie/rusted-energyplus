//! Fixture payloads for arbitrary-run integration tests.

pub(crate) const TWO_HOUR_EPW: &str = r#"LOCATION,Example
DESIGN CONDITIONS
TYPICAL/EXTREME PERIODS
GROUND TEMPERATURES
HOLIDAYS/DAYLIGHT SAVINGS
COMMENTS 1
COMMENTS 2
DATA PERIODS
1999,1,1,1,0,Source,-3.0,-4.0,50,82000,0,0,300,10,20,30,0,0,0,0,180,2.5
1999,1,1,2,0,Source,-2.0,-3.0,51,82100,0,0,301,11,21,31,0,0,0,0,190,2.6
"#;

pub(crate) const ONE_ZONE_EPJSON: &str = r#"{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Building": {"Defaulted Building": {"terrain": "Suburbs"}},
  "Timestep": {"Timestep 1": {}},
  "Site:Location": {"Denver Site": {"latitude": 39.74, "longitude": -105.18}},
  "Material:NoMass": {"R13": {"thermal_resistance": 2.29}},
  "Construction": {"Wall Construction": {"outside_layer": "R13"}},
  "ScheduleTypeLimits": {
    "Fraction": {
      "lower_limit_value": 0.0,
      "numeric_type": "Continuous",
      "upper_limit_value": 1.0
    }
  },
  "Schedule:Constant": {
    "Always On": {"schedule_type_limits_name": "Fraction"}
  },
  "Zone": {"Zone One": {"volume": 100}},
  "BuildingSurface:Detailed": {
    "Wall One": {
      "construction_name": "Wall Construction",
      "outside_boundary_condition": "Outdoors",
      "surface_type": "Wall",
      "vertices": [
        {"vertex_x_coordinate": 0.0, "vertex_y_coordinate": 0.0, "vertex_z_coordinate": 0.0},
        {"vertex_x_coordinate": 1.0, "vertex_y_coordinate": 0.0, "vertex_z_coordinate": 0.0},
        {"vertex_x_coordinate": 1.0, "vertex_y_coordinate": 1.0, "vertex_z_coordinate": 0.0},
        {"vertex_x_coordinate": 0.0, "vertex_y_coordinate": 1.0, "vertex_z_coordinate": 0.0}
      ],
      "zone_name": "Zone One"
    }
  }
}"#;

pub(crate) const MISSING_SURFACE_ZONE_EPJSON: &str = r#"{
  "Material:NoMass": {"R13": {}},
  "Construction": {"Wall Construction": {"outside_layer": "R13"}},
  "BuildingSurface:Detailed": {
    "Wall One": {
      "surface_type": "Wall",
      "construction_name": "Wall Construction",
      "zone_name": "Missing Zone",
      "outside_boundary_condition": "Outdoors",
      "vertices": [
        {"vertex_x_coordinate": 0, "vertex_y_coordinate": 0, "vertex_z_coordinate": 0}
      ]
    }
  }
}"#;

pub(crate) const AIR_LOOP_EPJSON: &str = r#"{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Zone": {"Zone One": {"volume": 100}},
  "AirLoopHVAC": {"Main Air Loop": {}}
}"#;

pub(crate) const EMS_EPJSON: &str = r#"{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Zone": {"Zone One": {"volume": 100}},
  "EnergyManagementSystem:Program": {
    "Override Program": {
      "lines": [{"program_line": "SET X = 1"}]
    }
  }
}"#;

pub(crate) const PLANT_LOOP_EPJSON: &str = r#"{
  "Version": {"Version 1": {"version_identifier": "26.1"}},
  "Pump:ConstantSpeed": {
    "HW Pump": {
      "inlet_node_name": "HW Supply Inlet",
      "outlet_node_name": "HW Pump Outlet",
      "design_flow_rate": 0.001,
      "design_pump_head": 179352,
      "pump_control_type": "Intermittent"
    }
  },
  "Boiler:HotWater": {
    "HW Boiler": {
      "fuel_type": "NaturalGas",
      "nominal_capacity": 10000,
      "design_water_flow_rate": 0.001,
      "boiler_water_inlet_node_name": "HW Pump Outlet",
      "boiler_water_outlet_node_name": "HW Supply Outlet"
    }
  },
  "Branch": {
    "HW Supply Inlet Branch": {
      "components": [
        {
          "component_object_type": "Pump:ConstantSpeed",
          "component_name": "HW Pump",
          "component_inlet_node_name": "HW Supply Inlet",
          "component_outlet_node_name": "HW Pump Outlet"
        }
      ]
    },
    "HW Boiler Branch": {
      "components": [
        {
          "component_object_type": "Boiler:HotWater",
          "component_name": "HW Boiler",
          "component_inlet_node_name": "HW Pump Outlet",
          "component_outlet_node_name": "HW Supply Outlet"
        }
      ]
    },
    "HW Demand Branch": {
      "components": [
        {
          "component_object_type": "Pipe:Adiabatic",
          "component_name": "HW Demand Pipe",
          "component_inlet_node_name": "HW Demand Inlet",
          "component_outlet_node_name": "HW Demand Outlet"
        }
      ]
    }
  },
  "BranchList": {
    "HW Supply Branches": {
      "branches": [
        {"branch_name": "HW Supply Inlet Branch"},
        {"branch_name": "HW Boiler Branch"}
      ]
    },
    "HW Demand Branches": {
      "branches": [
        {"branch_name": "HW Demand Branch"}
      ]
    }
  },
  "Connector:Splitter": {
    "HW Supply Splitter": {
      "inlet_branch_name": "HW Supply Inlet Branch",
      "branches": [
        {"outlet_branch_name": "HW Boiler Branch"}
      ]
    }
  },
  "Connector:Mixer": {
    "HW Supply Mixer": {
      "outlet_branch_name": "HW Boiler Branch",
      "branches": [
        {"inlet_branch_name": "HW Supply Inlet Branch"}
      ]
    }
  },
  "ConnectorList": {
    "HW Supply Connectors": {
      "connector_1_object_type": "Connector:Splitter",
      "connector_1_name": "HW Supply Splitter",
      "connector_2_object_type": "Connector:Mixer",
      "connector_2_name": "HW Supply Mixer"
    }
  },
  "PlantLoop": {
    "Hot Water Loop": {
      "fluid_type": "Water",
      "plant_side_inlet_node_name": "HW Supply Inlet",
      "plant_side_outlet_node_name": "HW Supply Outlet",
      "plant_side_branch_list_name": "HW Supply Branches",
      "plant_side_connector_list_name": "HW Supply Connectors",
      "demand_side_inlet_node_name": "HW Demand Inlet",
      "demand_side_outlet_node_name": "HW Demand Outlet",
      "demand_side_branch_list_name": "HW Demand Branches",
      "load_distribution_scheme": "SequentialLoad"
    }
  }
}"#;

pub(crate) const IDEAL_LOADS_EPJSON: &str = r#"{
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
}"#;

pub(crate) const IDEAL_LOADS_MIXED_BRANCH_EPJSON: &str = r#"{
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
      "nodes": [
        {"node_name": "Zone One Inlet"},
        {"node_name": "Zone One Limited Inlet"}
      ]
    },
    "Zone Limited Inlets": {
      "nodes": [{"node_name": "Zone One Limited Inlet"}]
    }
  },
  "ZoneHVAC:IdealLoadsAirSystem": {
    "Zone Ideal Loads": {
      "zone_supply_air_node_name": "Zone Inlets",
      "dehumidification_control_type": "None",
      "humidification_control_type": "None"
    },
    "Zone Ideal Loads Limited": {
      "zone_supply_air_node_name": "Zone Limited Inlets",
      "heating_limit": "LimitCapacity",
      "maximum_sensible_heating_capacity": 500.0,
      "cooling_limit": "LimitCapacity",
      "maximum_total_cooling_capacity": 500.0,
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
        },
        {
          "zone_equipment_object_type": "ZoneHVAC:IdealLoadsAirSystem",
          "zone_equipment_name": "Zone Ideal Loads Limited",
          "zone_equipment_cooling_sequence": 2,
          "zone_equipment_heating_or_no_load_sequence": 2
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
}"#;
