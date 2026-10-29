//! Generated exact finite Original input-statistics declarations, not counters.
//! Source official EnergyPlus26.1 schema; author ledger retains every operand.
use super::{SchemaAutomatic, SchemaDefault, SchemaDefinition, SchemaField};
pub(super) fn definition(name: &str) -> Option<SchemaDefinition> {
    Some(match name {
        "ZoneGroup" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_list_multiplier",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "SpaceList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: Some("spaces"),
            extensible: &[SchemaField {
                name: "space_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "Space" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ceiling_height",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "volume",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "floor_area",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "space_type",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
            ],
            extension: Some("tags"),
            extensible: &[SchemaField {
                name: "tag",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "ZoneList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: Some("zones"),
            extensible: &[SchemaField {
                name: "zone_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "ComfortViewFactorAngles" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: Some("angles"),
            extensible: &[
                SchemaField {
                    name: "surface_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "angle_factor",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "AirLoopHVAC:OutdoorAirSystem" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outdoor_air_equipment_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirLoopHVAC:OutdoorAirSystem:EquipmentList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_1_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_1_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_2_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_2_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_3_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_3_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_4_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_4_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_5_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_5_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_6_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_6_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_7_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_7_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_8_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_8_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_9_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_9_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "LifeCycleCost:Parameters" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "discounting_convention",
                    default: SchemaDefault::String("EndOfYear"),
                    automatic: &[],
                },
                SchemaField {
                    name: "inflation_approach",
                    default: SchemaDefault::String("ConstantDollar"),
                    automatic: &[],
                },
                SchemaField {
                    name: "real_discount_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "nominal_discount_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "inflation",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "base_date_month",
                    default: SchemaDefault::String("January"),
                    automatic: &[],
                },
                SchemaField {
                    name: "base_date_year",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "service_date_month",
                    default: SchemaDefault::String("January"),
                    automatic: &[],
                },
                SchemaField {
                    name: "service_date_year",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "length_of_study_period_in_years",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "tax_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "depreciation_method",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "LifeCycleCost:RecurringCosts" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "category",
                    default: SchemaDefault::String("Maintenance"),
                    automatic: &[],
                },
                SchemaField {
                    name: "cost",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "start_of_costs",
                    default: SchemaDefault::String("ServicePeriod"),
                    automatic: &[],
                },
                SchemaField {
                    name: "years_from_start",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "months_from_start",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "repeat_period_years",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "repeat_period_months",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "annual_escalation_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "LifeCycleCost:NonrecurringCost" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "category",
                    default: SchemaDefault::String("Construction"),
                    automatic: &[],
                },
                SchemaField {
                    name: "cost",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "start_of_costs",
                    default: SchemaDefault::String("ServicePeriod"),
                    automatic: &[],
                },
                SchemaField {
                    name: "years_from_start",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "months_from_start",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "LifeCycleCost:UsePriceEscalation" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "lcc_price_escalation_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "resource",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "escalation_start_year",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "escalation_start_month",
                    default: SchemaDefault::String("January"),
                    automatic: &[],
                },
            ],
            extension: Some("escalations"),
            extensible: &[SchemaField {
                name: "year_escalation",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "LifeCycleCost:UseAdjustment" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "resource",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("multipliers"),
            extensible: &[SchemaField {
                name: "year_multiplier",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "AvailabilityManager:OptimumStart" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "applicability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fan_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_type",
                    default: SchemaDefault::String("ControlZone"),
                    automatic: &[],
                },
                SchemaField {
                    name: "control_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_value_for_optimum_start_time",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_algorithm",
                    default: SchemaDefault::String("AdaptiveASHRAE"),
                    automatic: &[],
                },
                SchemaField {
                    name: "constant_temperature_gradient_during_cooling",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "constant_temperature_gradient_during_heating",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "initial_temperature_gradient_during_cooling",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "initial_temperature_gradient_during_heating",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "constant_start_time",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_previous_days",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:HybridVentilation" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "hvac_air_loop_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ventilation_control_mode_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "use_weather_file_rain_indicators",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_wind_speed",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_outdoor_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_outdoor_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_outdoor_enthalpy",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_outdoor_enthalpy",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_outdoor_dewpoint",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_outdoor_dewpoint",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_outdoor_ventilation_air_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "opening_factor_function_of_wind_speed_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "airflownetwork_control_type_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "simple_airflow_control_type_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zoneventilation_object_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_hvac_operation_time",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_ventilation_time",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:NightVentilation" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "applicability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fan_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ventilation_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ventilation_temperature_difference",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "ventilation_temperature_low_limit",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "night_venting_flow_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:LowTemperatureTurnOn" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sensor_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:LowTemperatureTurnOff" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sensor_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "applicability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:HighTemperatureTurnOn" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sensor_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:HighTemperatureTurnOff" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sensor_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:DifferentialThermostat" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "hot_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cold_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature_difference_on_limit",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature_difference_off_limit",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:NightCycle" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "applicability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fan_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_type",
                    default: SchemaDefault::String("StayOff"),
                    automatic: &[],
                },
                SchemaField {
                    name: "thermostat_tolerance",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "cycling_run_time_control_type",
                    default: SchemaDefault::String("FixedRunTime"),
                    automatic: &[],
                },
                SchemaField {
                    name: "cycling_run_time",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_zone_or_zone_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_control_zone_or_zone_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_control_zone_or_zone_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_zone_fans_only_zone_or_zone_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:ScheduledOff" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:ScheduledOn" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AvailabilityManager:Scheduled" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "WaterUse:Storage" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_quality_subcategory",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "initial_volume",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_in_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_out_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "overflow_destination",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "type_of_supply_controlled_by_float_valve",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "float_valve_on_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "float_valve_off_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "backup_mains_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "other_tank_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_thermal_mode",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ambient_temperature_indicator",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ambient_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "tank_surface_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "tank_u_value",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "tank_outside_surface_material_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "WaterUse:RainCollector" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "storage_tank_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "loss_factor_mode",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "collection_loss_factor",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "collection_loss_factor_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_collection_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("surfaces"),
            extensible: &[SchemaField {
                name: "collection_surface_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "WaterUse:Well" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "storage_tank_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "pump_depth",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "pump_rated_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "pump_rated_head",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "pump_rated_power_consumption",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "pump_efficiency",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "well_recovery_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "nominal_well_storage_volume",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_table_depth_mode",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_table_depth",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_table_depth_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Site:Precipitation" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "precipitation_model_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level_for_total_annual_precipitation",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "precipitation_rates_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "average_total_annual_precipitation",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "RoofIrrigation" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "irrigation_model_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "irrigation_rate_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "irrigation_maximum_saturation_threshold",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirLoopHVAC" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "availability_manager_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_supply_air_flow_rate",
                    default: SchemaDefault::NonString,
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "branch_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "connector_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "supply_side_inlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "demand_side_outlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "demand_side_inlet_node_names",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "supply_side_outlet_node_names",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_return_air_flow_fraction_of_supply_air_flow",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirLoopHVAC:ControllerList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_1_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_1_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_2_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_2_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_3_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_3_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_4_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_4_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_5_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_5_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_6_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_6_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_7_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_7_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_8_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "controller_8_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:SimulationControl" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "airflownetwork_control",
                    default: SchemaDefault::String("NoMultizoneOrDistribution"),
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_type",
                    default: SchemaDefault::String("SurfaceAverageCalculation"),
                    automatic: &[],
                },
                SchemaField {
                    name: "height_selection_for_local_wind_pressure_calculation",
                    default: SchemaDefault::String("OpeningHeight"),
                    automatic: &[],
                },
                SchemaField {
                    name: "building_type",
                    default: SchemaDefault::String("LowRise"),
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_number_of_iterations",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "initialization_type",
                    default: SchemaDefault::String("ZeroNodePressures"),
                    automatic: &[],
                },
                SchemaField {
                    name: "relative_airflow_convergence_tolerance",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "absolute_airflow_convergence_tolerance",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "convergence_acceleration_limit",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "azimuth_angle_of_long_axis_of_building",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "ratio_of_building_width_along_short_axis_to_width_along_long_axis",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "height_dependence_of_external_node_temperature",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "solver",
                    default: SchemaDefault::String("SkylineLU"),
                    automatic: &[],
                },
                SchemaField {
                    name: "allow_unsupported_zone_equipment",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "do_distribution_duct_sizing_calculation",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:MultiZone:Zone" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ventilation_control_mode",
                    default: SchemaDefault::String("NoVent"),
                    automatic: &[],
                },
                SchemaField {
                    name: "ventilation_control_zone_temperature_setpoint_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_venting_open_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "indoor_and_outdoor_temperature_difference_lower_limit_for_maximum_venting_open_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "indoor_and_outdoor_temperature_difference_upper_limit_for_minimum_venting_open_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "indoor_and_outdoor_enthalpy_difference_lower_limit_for_maximum_venting_open_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "indoor_and_outdoor_enthalpy_difference_upper_limit_for_minimum_venting_open_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "venting_availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "single_sided_wind_pressure_coefficient_algorithm",
                    default: SchemaDefault::String("Standard"),
                    automatic: &[],
                },
                SchemaField {
                    name: "facade_width",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "occupant_ventilation_control_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:MultiZone:Surface" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "surface_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "leakage_component_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "external_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "window_door_opening_factor_or_crack_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "ventilation_control_mode",
                    default: SchemaDefault::String("ZoneLevel"),
                    automatic: &[],
                },
                SchemaField {
                    name: "ventilation_control_zone_temperature_setpoint_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_venting_open_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "indoor_and_outdoor_temperature_difference_lower_limit_for_maximum_venting_open_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "indoor_and_outdoor_temperature_difference_upper_limit_for_minimum_venting_open_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "indoor_and_outdoor_enthalpy_difference_lower_limit_for_maximum_venting_open_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "indoor_and_outdoor_enthalpy_difference_upper_limit_for_minimum_venting_open_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "venting_availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "occupant_ventilation_control_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "equivalent_rectangle_method",
                    default: SchemaDefault::String("PolygonHeight"),
                    automatic: &[],
                },
                SchemaField {
                    name: "equivalent_rectangle_aspect_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:MultiZone:Component:DetailedOpening" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_mass_flow_coefficient_when_opening_is_closed",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_mass_flow_exponent_when_opening_is_closed",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "type_of_rectangular_large_vertical_opening_lvo_",
                    default: SchemaDefault::String("NonPivoted"),
                    automatic: &[],
                },
                SchemaField {
                    name: "extra_crack_length_or_height_of_pivoting_axis",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_sets_of_opening_factor_data",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "opening_factor_1",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "discharge_coefficient_for_opening_factor_1",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "width_factor_for_opening_factor_1",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "height_factor_for_opening_factor_1",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "start_height_factor_for_opening_factor_1",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "opening_factor_2",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "discharge_coefficient_for_opening_factor_2",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "width_factor_for_opening_factor_2",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "height_factor_for_opening_factor_2",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "start_height_factor_for_opening_factor_2",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "opening_factor_3",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "discharge_coefficient_for_opening_factor_3",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "width_factor_for_opening_factor_3",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "height_factor_for_opening_factor_3",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "start_height_factor_for_opening_factor_3",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "opening_factor_4",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "discharge_coefficient_for_opening_factor_4",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "width_factor_for_opening_factor_4",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "height_factor_for_opening_factor_4",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "start_height_factor_for_opening_factor_4",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:MultiZone:ExternalNode" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "external_node_height",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "symmetric_wind_pressure_coefficient_curve",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_angle_type",
                    default: SchemaDefault::String("Absolute"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:MultiZone:WindPressureCoefficientArray" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_1",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_2",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_3",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_4",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_5",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_6",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_7",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_8",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_9",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_10",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_11",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_12",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_13",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_14",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_15",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_16",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_17",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_18",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_19",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_20",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_21",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_22",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_23",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_24",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_25",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_26",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_27",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_28",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_29",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_30",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_31",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_32",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_33",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_34",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_35",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_36",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:MultiZone:WindPressureCoefficientValues" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "airflownetwork_multizone_windpressurecoefficientarray_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_1",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_2",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_3",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_4",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_5",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_6",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_7",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_8",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_9",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_10",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_11",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_12",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_13",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_14",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_15",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_16",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_17",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_18",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_19",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_20",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_21",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_22",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_23",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_24",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_25",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_26",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_27",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_28",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_29",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_30",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_31",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_32",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_33",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_34",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_35",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_value_36",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:Distribution:Node" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_name_or_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_object_type_or_node_type",
                    default: SchemaDefault::String("Other"),
                    automatic: &[],
                },
                SchemaField {
                    name: "node_height",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:Distribution:DuctViewFactors" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "linkage_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "duct_surface_exposure_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "duct_surface_emittance",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: Some("surfaces"),
            extensible: &[
                SchemaField {
                    name: "surface_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "surface_view_factor",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "AirflowNetwork:Distribution:Linkage" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "node_1_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "node_2_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:OccupantVentilationControl" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_opening_time",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_closing_time",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_comfort_low_temperature_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_comfort_temperature_boundary_point",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_comfort_high_temperature_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_threshold_for_persons_dissatisfied_ppd",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "occupancy_check",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "opening_probability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "closing_probability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:IntraZone:Node" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "roomair_node_airflownetwork_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "node_height",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:IntraZone:Linkage" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "node_1_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "node_2_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "airflownetwork_multizone_surface_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:ZoneControl:PressureController" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_object_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "pressure_control_availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "pressure_setpoint_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirflowNetwork:Distribution:DuctSizing" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "duct_sizing_method",
                    default: SchemaDefault::String("MaximumVelocity"),
                    automatic: &[],
                },
                SchemaField {
                    name: "duct_sizing_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_airflow_velocity",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "total_pressure_loss_across_supply_trunk",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "total_pressure_loss_across_supply_branch",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "total_pressure_loss_across_return_trunk",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "total_pressure_loss_across_return_branch",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "BranchList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: Some("branches"),
            extensible: &[SchemaField {
                name: "branch_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "Building" => SchemaDefinition {
            has_name: true,
            name_has_default: true,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::String("NONE"),
                    automatic: &[],
                },
                SchemaField {
                    name: "north_axis",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "terrain",
                    default: SchemaDefault::String("Suburbs"),
                    automatic: &[],
                },
                SchemaField {
                    name: "loads_convergence_tolerance_value",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature_convergence_tolerance_value",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "solar_distribution",
                    default: SchemaDefault::String("FullExterior"),
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_number_of_warmup_days",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_number_of_warmup_days",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "BuildingSurface:Detailed" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "surface_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "construction_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outside_boundary_condition",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outside_boundary_condition_object",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sun_exposure",
                    default: SchemaDefault::String("SunExposed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_exposure",
                    default: SchemaDefault::String("WindExposed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "view_factor_to_ground",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "number_of_vertices",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
            ],
            extension: Some("vertices"),
            extensible: &[
                SchemaField {
                    name: "vertex_x_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_y_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_z_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "Wall:Detailed" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "construction_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outside_boundary_condition",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outside_boundary_condition_object",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sun_exposure",
                    default: SchemaDefault::String("SunExposed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_exposure",
                    default: SchemaDefault::String("WindExposed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "view_factor_to_ground",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "number_of_vertices",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
            ],
            extension: Some("vertices"),
            extensible: &[
                SchemaField {
                    name: "vertex_x_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_y_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_z_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "Floor:Detailed" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "construction_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outside_boundary_condition",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outside_boundary_condition_object",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sun_exposure",
                    default: SchemaDefault::String("SunExposed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_exposure",
                    default: SchemaDefault::String("WindExposed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "view_factor_to_ground",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "number_of_vertices",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
            ],
            extension: Some("vertices"),
            extensible: &[
                SchemaField {
                    name: "vertex_x_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_y_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_z_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "RoofCeiling:Detailed" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "construction_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outside_boundary_condition",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outside_boundary_condition_object",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sun_exposure",
                    default: SchemaDefault::String("SunExposed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_exposure",
                    default: SchemaDefault::String("WindExposed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "view_factor_to_ground",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "number_of_vertices",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
            ],
            extension: Some("vertices"),
            extensible: &[
                SchemaField {
                    name: "vertex_x_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_y_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_z_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "FenestrationSurface:Detailed" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "surface_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "construction_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "building_surface_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outside_boundary_condition_object",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "view_factor_to_ground",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "frame_and_divider_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "multiplier",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_vertices",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "vertex_1_x_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_1_y_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_1_z_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_2_x_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_2_y_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_2_z_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_3_x_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_3_y_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_3_z_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_4_x_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_4_y_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_4_z_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Shading:Zone:Detailed" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "base_surface_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "transmittance_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_vertices",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
            ],
            extension: Some("vertices"),
            extensible: &[
                SchemaField {
                    name: "vertex_x_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_y_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_z_coordinate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "SurfaceProperty:SolarIncidentInside" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "surface_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "construction_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "inside_surface_incident_sun_solar_radiation_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Connector:Mixer" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outlet_branch_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("branches"),
            extensible: &[SchemaField {
                name: "inlet_branch_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "Connector:Splitter" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "inlet_branch_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("branches"),
            extensible: &[SchemaField {
                name: "outlet_branch_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "ConnectorList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "connector_1_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "connector_1_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "connector_2_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "connector_2_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Construction" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outside_layer",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "layer_2",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "layer_3",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "layer_4",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "layer_5",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "layer_6",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "layer_7",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "layer_8",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "layer_9",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "layer_10",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "DesignSpecification:AirTerminal:Sizing" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_design_cooling_load",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_design_supply_air_temperature_difference_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_design_heating_load",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_design_supply_air_temperature_difference_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_minimum_outdoor_air_flow",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "DesignSpecification:OutdoorAir" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outdoor_air_method",
                    default: SchemaDefault::String("Flow/Person"),
                    automatic: &[],
                },
                SchemaField {
                    name: "outdoor_air_flow_per_person",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "outdoor_air_flow_per_zone_floor_area",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "outdoor_air_flow_per_zone",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "outdoor_air_flow_air_changes_per_hour",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "outdoor_air_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "proportional_control_minimum_outdoor_air_flow_rate_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "DesignSpecification:OutdoorAir:SpaceList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: Some("space_specs"),
            extensible: &[
                SchemaField {
                    name: "space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_design_specification_outdoor_air_object_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "DesignSpecification:ZoneAirDistribution" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_air_distribution_effectiveness_in_cooling_mode",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_air_distribution_effectiveness_in_heating_mode",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_air_distribution_effectiveness_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_secondary_recirculation_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_zone_ventilation_efficiency",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "DesignSpecification:ZoneHVAC:Sizing" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_supply_air_flow_rate_method",
                    default: SchemaDefault::String("SupplyAirFlowRate"),
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_supply_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "cooling_supply_air_flow_rate_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_fraction_of_autosized_cooling_supply_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_supply_air_flow_rate_per_unit_cooling_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "no_load_supply_air_flow_rate_method",
                    default: SchemaDefault::String("SupplyAirFlowRate"),
                    automatic: &[],
                },
                SchemaField {
                    name: "no_load_supply_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "no_load_supply_air_flow_rate_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "no_load_fraction_of_cooling_supply_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "no_load_fraction_of_heating_supply_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_supply_air_flow_rate_method",
                    default: SchemaDefault::String("SupplyAirFlowRate"),
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_supply_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "heating_supply_air_flow_rate_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_fraction_of_heating_supply_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_supply_air_flow_rate_per_unit_heating_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_design_capacity_method",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_design_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "cooling_design_capacity_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_autosized_cooling_design_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_design_capacity_method",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_design_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "heating_design_capacity_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_autosized_heating_design_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ElectricEquipment" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_or_space_or_spacelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level_calculation_method",
                    default: SchemaDefault::String("EquipmentLevel"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "watts_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "watts_per_person",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_latent",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_radiant",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_lost",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ElectricEquipment:ITE:AirCooled" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_flow_calculation_method",
                    default: SchemaDefault::String("FlowFromSystem"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_power_input_calculation_method",
                    default: SchemaDefault::String("Watts/Unit"),
                    automatic: &[],
                },
                SchemaField {
                    name: "watts_per_unit",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_units",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "watts_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_power_input_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cpu_loading_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cpu_power_input_function_of_loading_and_air_temperature_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_fan_power_input_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_fan_air_flow_rate_per_power_input",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_flow_function_of_loading_and_air_temperature_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fan_power_input_function_of_flow_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_entering_air_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "environmental_class",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "air_inlet_connection_type",
                    default: SchemaDefault::String("AdjustedSupply"),
                    automatic: &[],
                },
                SchemaField {
                    name: "air_inlet_room_air_model_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_outlet_room_air_model_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "supply_air_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_recirculation_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "recirculation_function_of_loading_and_supply_temperature_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_electric_power_supply_efficiency",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "electric_power_supply_efficiency_function_of_part_load_ratio_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_electric_power_supply_losses_to_zone",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "cpu_end_use_subcategory",
                    default: SchemaDefault::String("ITE-CPU"),
                    automatic: &[],
                },
                SchemaField {
                    name: "fan_end_use_subcategory",
                    default: SchemaDefault::String("ITE-Fans"),
                    automatic: &[],
                },
                SchemaField {
                    name: "electric_power_supply_end_use_subcategory",
                    default: SchemaDefault::String("ITE-UPS"),
                    automatic: &[],
                },
                SchemaField {
                    name: "supply_temperature_difference",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "supply_temperature_difference_schedule",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "return_temperature_difference",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "return_temperature_difference_schedule",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Exterior:Lights" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_option",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ExternalInterface:FunctionalMockupUnitExport:To:Schedule" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_type_limits_names",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fmu_variable_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "initial_value",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ExternalInterface:FunctionalMockupUnitImport:To:Schedule" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_type_limits_names",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fmu_file_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fmu_instance_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fmu_variable_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "initial_value",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ExternalInterface:Schedule" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_type_limits_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "initial_value",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "GasEquipment" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_or_space_or_spacelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level_calculation_method",
                    default: SchemaDefault::String("EquipmentLevel"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "power_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "power_per_person",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_latent",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_radiant",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_lost",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "carbon_dioxide_generation_rate",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "GlobalGeometryRules" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "starting_vertex_position",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertex_entry_direction",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "coordinate_system",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "daylighting_reference_point_coordinate_system",
                    default: SchemaDefault::String("Relative"),
                    automatic: &[],
                },
                SchemaField {
                    name: "rectangular_surface_coordinate_system",
                    default: SchemaDefault::String("Relative"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "HeatBalanceAlgorithm" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "algorithm",
                    default: SchemaDefault::String("ConductionTransferFunction"),
                    automatic: &[],
                },
                SchemaField {
                    name: "surface_temperature_upper_limit",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_surface_convection_heat_transfer_coefficient_value",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_surface_convection_heat_transfer_coefficient_value",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "HotWaterEquipment" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_or_space_or_spacelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level_calculation_method",
                    default: SchemaDefault::String("EquipmentLevel"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "power_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "power_per_person",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_latent",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_radiant",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_lost",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Lights" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_or_space_or_spacelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level_calculation_method",
                    default: SchemaDefault::String("LightingLevel"),
                    automatic: &[],
                },
                SchemaField {
                    name: "lighting_level",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "watts_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "watts_per_person",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "return_air_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_radiant",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_visible",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_replaceable",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
                SchemaField {
                    name: "return_air_fraction_calculated_from_plenum_temperature",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "return_air_fraction_function_of_plenum_temperature_coefficient_1",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "return_air_fraction_function_of_plenum_temperature_coefficient_2",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "return_air_heat_gain_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "exhaust_air_heat_gain_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Material" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "roughness",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thickness",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "conductivity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "density",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "specific_heat",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_absorptance",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "solar_absorptance",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "visible_absorptance",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Material:InfraredTransparent" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: None,
            extensible: &[],
        },
        "Material:AirGap" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_resistance",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Material:NoMass" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "roughness",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_resistance",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_absorptance",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "solar_absorptance",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "visible_absorptance",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "OtherEquipment" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fuel_type",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_or_space_or_spacelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level_calculation_method",
                    default: SchemaDefault::String("EquipmentLevel"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "power_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "power_per_person",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_latent",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_radiant",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_lost",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "carbon_dioxide_generation_rate",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Output:Constructions" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "details_type_1",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "details_type_2",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Output:Diagnostics" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[],
            extension: Some("diagnostics"),
            extensible: &[SchemaField {
                name: "key",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "Output:Meter" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "key_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "reporting_frequency",
                    default: SchemaDefault::String("Hourly"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Output:Meter:Cumulative" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "key_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "reporting_frequency",
                    default: SchemaDefault::String("Hourly"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Output:Meter:Cumulative:MeterFileOnly" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "key_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "reporting_frequency",
                    default: SchemaDefault::String("Hourly"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Output:Meter:MeterFileOnly" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "key_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "reporting_frequency",
                    default: SchemaDefault::String("Hourly"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Output:Schedules" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[SchemaField {
                name: "key_field",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: None,
            extensible: &[],
        },
        "Output:Surfaces:List" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "report_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "report_specifications",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Output:Surfaces:Drawing" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "report_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "report_specifications_1",
                    default: SchemaDefault::String("Triangulate3DFace"),
                    automatic: &[],
                },
                SchemaField {
                    name: "report_specifications_2",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Output:Table:Monthly" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "digits_after_decimal",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: Some("variable_details"),
            extensible: &[
                SchemaField {
                    name: "variable_or_meter_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "aggregation_type_for_variable_or_meter",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "Output:Table:Annual" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "filter",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("variable_details"),
            extensible: &[
                SchemaField {
                    name: "variable_or_meter_or_ems_variable_or_field_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "aggregation_type_for_variable_or_meter",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "digits_after_decimal",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
        },
        "Output:Table:TimeBins" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "key_value",
                    default: SchemaDefault::String("*"),
                    automatic: &[],
                },
                SchemaField {
                    name: "variable_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "interval_start",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "interval_size",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "interval_count",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "variable_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Output:Table:SummaryReports" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[],
            extension: Some("reports"),
            extensible: &[SchemaField {
                name: "report_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "Output:Variable" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "key_value",
                    default: SchemaDefault::String("*"),
                    automatic: &[],
                },
                SchemaField {
                    name: "variable_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "reporting_frequency",
                    default: SchemaDefault::String("Hourly"),
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Output:VariableDictionary" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "key_field",
                    default: SchemaDefault::String("regular"),
                    automatic: &[],
                },
                SchemaField {
                    name: "sort_option",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "OutputControl:Sizing:Style" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[SchemaField {
                name: "column_separator",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: None,
            extensible: &[],
        },
        "OutputControl:Table:Style" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "column_separator",
                    default: SchemaDefault::String("Comma"),
                    automatic: &[],
                },
                SchemaField {
                    name: "unit_conversion",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "format_numeric_values",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "People" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_or_space_or_spacelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_people_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_people_calculation_method",
                    default: SchemaDefault::String("People"),
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_people",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "people_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "floor_area_per_person",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_radiant",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "sensible_heat_fraction",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "activity_level_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "carbon_dioxide_generation_rate",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "enable_ashrae_55_comfort_warnings",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "mean_radiant_temperature_calculation_type",
                    default: SchemaDefault::String("EnclosureAveraged"),
                    automatic: &[],
                },
                SchemaField {
                    name: "surface_name_angle_factor_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "work_efficiency_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "clothing_insulation_calculation_method",
                    default: SchemaDefault::String("ClothingInsulationSchedule"),
                    automatic: &[],
                },
                SchemaField {
                    name: "clothing_insulation_calculation_method_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "clothing_insulation_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_velocity_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_comfort_model_1_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_comfort_model_2_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_comfort_model_3_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_comfort_model_4_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_comfort_model_5_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_comfort_model_6_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_comfort_model_7_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ankle_level_air_velocity_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cold_stress_temperature_threshold",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "heat_stress_temperature_threshold",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:AirChiller" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "capacity_rating_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_unit_load_factor",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_relative_humidity",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_cooling_source_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_temperature_difference_dt1",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_temperature_difference_between_inlet_air_and_evaporating_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "coil_material_correction_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigerant_correction_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "capacity_correction_curve_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "capacity_correction_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "shr60_correction_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_total_heating_power",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_power_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fan_speed_control_type",
                    default: SchemaDefault::String("Fixed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_fan_power",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_air_flow",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_fan_air_flow_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_type",
                    default: SchemaDefault::String("Electric"),
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_control_type",
                    default: SchemaDefault::String("TimeSchedule"),
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_drip_down_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_power",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature_termination_defrost_fraction_to_ice",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "vertical_location",
                    default: SchemaDefault::String("Middle"),
                    automatic: &[],
                },
                SchemaField {
                    name: "average_refrigerant_charge_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:Case" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_ambient_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_ambient_relative_humidity",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_total_cooling_capacity_per_unit_length",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_latent_heat_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_runtime_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "case_length",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "case_operating_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "latent_case_credit_curve_type",
                    default: SchemaDefault::String("CaseTemperatureMethod"),
                    automatic: &[],
                },
                SchemaField {
                    name: "latent_case_credit_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "standard_case_fan_power_per_unit_length",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "operating_case_fan_power_per_unit_length",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "standard_case_lighting_power_per_unit_length",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "installed_case_lighting_power_per_unit_length",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "case_lighting_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_lighting_energy_to_case",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "case_anti_sweat_heater_power_per_unit_length",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_anti_sweat_heater_power_per_unit_length",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "anti_sweat_heater_control_type",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "humidity_at_zero_anti_sweat_heater_energy",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "case_height",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_anti_sweat_heater_energy_to_case",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "case_defrost_power_per_unit_length",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "case_defrost_type",
                    default: SchemaDefault::String("OffCycle"),
                    automatic: &[],
                },
                SchemaField {
                    name: "case_defrost_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "case_defrost_drip_down_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_energy_correction_curve_type",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_energy_correction_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "under_case_hvac_return_air_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigerated_case_restocking_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "case_credit_fraction_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_evaporator_temperature_or_brine_inlet_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "average_refrigerant_charge_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "under_case_hvac_return_air_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:CaseAndWalkInList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: Some("cases_and_walkins"),
            extensible: &[SchemaField {
                name: "case_or_walkin_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "Refrigeration:Compressor" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigeration_compressor_power_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigeration_compressor_capacity_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_superheat",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_return_gas_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_liquid_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_subcooling",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
                SchemaField {
                    name: "mode_of_operation",
                    default: SchemaDefault::String("Subcritical"),
                    automatic: &[],
                },
                SchemaField {
                    name: "transcritical_compressor_power_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "transcritical_compressor_capacity_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:CompressorList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: Some("compressors"),
            extensible: &[SchemaField {
                name: "refrigeration_compressor_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "Refrigeration:CompressorRack" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heat_rejection_location",
                    default: SchemaDefault::String("Outdoors"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_compressor_rack_cop",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "compressor_rack_cop_function_of_temperature_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_condenser_fan_power",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "condenser_fan_power_function_of_temperature_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "condenser_type",
                    default: SchemaDefault::String("AirCooled"),
                    automatic: &[],
                },
                SchemaField {
                    name: "water_cooled_condenser_inlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_cooled_condenser_outlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_cooled_loop_flow_type",
                    default: SchemaDefault::String("VariableFlow"),
                    automatic: &[],
                },
                SchemaField {
                    name: "water_cooled_condenser_outlet_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_cooled_condenser_design_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_cooled_condenser_maximum_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_cooled_condenser_maximum_water_outlet_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_cooled_condenser_minimum_water_inlet_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "evaporative_condenser_availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "evaporative_condenser_effectiveness",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "evaporative_condenser_air_flow_rate",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "basin_heater_capacity",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "basin_heater_setpoint_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_evaporative_condenser_water_pump_power",
                    default: SchemaDefault::NonString,
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "evaporative_water_supply_tank_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "condenser_air_inlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigeration_case_name_or_walkin_name_or_caseandwalkinlist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heat_rejection_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:Condenser:AirCooled" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_effective_total_heat_rejection_rate_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_subcooling_temperature_difference",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "condenser_fan_speed_control_type",
                    default: SchemaDefault::String("Fixed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_fan_power",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_fan_air_flow_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_inlet_node_name_or_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
                SchemaField {
                    name: "condenser_refrigerant_operating_charge_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "condensate_receiver_refrigerant_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "condensate_piping_refrigerant_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:Condenser:Cascade" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_condensing_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_approach_temperature_difference",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_effective_total_heat_rejection_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "condensing_temperature_control_type",
                    default: SchemaDefault::String("Fixed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "condenser_refrigerant_operating_charge_inventory",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "condensate_receiver_refrigerant_inventory",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "condensate_piping_refrigerant_inventory",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:Condenser:EvaporativeCooled" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_effective_total_heat_rejection_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_subcooling_temperature_difference",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fan_speed_control_type",
                    default: SchemaDefault::String("Fixed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_fan_power",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_fan_air_flow_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "approach_temperature_constant_term",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "approach_temperature_coefficient_2",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "approach_temperature_coefficient_3",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "approach_temperature_coefficient_4",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_capacity_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_capacity_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_inlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_air_flow_rate",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "basin_heater_capacity",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "basin_heater_setpoint_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_water_pump_power",
                    default: SchemaDefault::NonString,
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "evaporative_water_supply_tank_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "evaporative_condenser_availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
                SchemaField {
                    name: "condenser_refrigerant_operating_charge_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "condensate_receiver_refrigerant_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "condensate_piping_refrigerant_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:Condenser:WaterCooled" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_effective_total_heat_rejection_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_condensing_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_subcooling_temperature_difference",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_water_inlet_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_inlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_outlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_cooled_loop_flow_type",
                    default: SchemaDefault::String("VariableFlow"),
                    automatic: &[],
                },
                SchemaField {
                    name: "water_outlet_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_design_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_maximum_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_maximum_water_outlet_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_minimum_water_inlet_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
                SchemaField {
                    name: "condenser_refrigerant_operating_charge_inventory",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "condensate_receiver_refrigerant_inventory",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "condensate_piping_refrigerant_inventory",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:GasCooler:AirCooled" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_total_heat_rejection_rate_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "gas_cooler_fan_speed_control_type",
                    default: SchemaDefault::String("Fixed"),
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_fan_power",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_fan_air_flow_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "transition_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "transcritical_approach_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "subcritical_temperature_difference",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_condensing_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_inlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
                SchemaField {
                    name: "gas_cooler_refrigerant_operating_charge_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "gas_cooler_receiver_refrigerant_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "gas_cooler_outlet_piping_refrigerant_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:SecondarySystem" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigerated_case_or_walkin_or_caseandwalkinlist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "circulating_fluid_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "circulating_fluid_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "evaporator_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "evaporator_flow_rate_for_secondary_fluid",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "evaporator_evaporating_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "evaporator_approach_temperature_difference",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "evaporator_range_temperature_difference",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_pumps_in_loop",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "total_pump_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "total_pump_power",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "total_pump_head",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "phasechange_circulating_rate",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "pump_drive_type",
                    default: SchemaDefault::String("Constant"),
                    automatic: &[],
                },
                SchemaField {
                    name: "variable_speed_pump_cubic_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "pump_motor_heat_to_fluid",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "sum_ua_distribution_piping",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "distribution_piping_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sum_ua_receiver_separator_shell",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "receiver_separator_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "evaporator_refrigerant_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:Subcooler" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "subcooler_type",
                    default: SchemaDefault::String("LiquidSuction"),
                    automatic: &[],
                },
                SchemaField {
                    name: "liquid_suction_design_subcooling_temperature_difference",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_liquid_inlet_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_vapor_inlet_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "capacity_providing_system",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outlet_control_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:System" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigerated_case_or_walkin_or_caseandwalkinlist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigeration_transfer_load_or_transferload_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigeration_condenser_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "compressor_or_compressorlist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_condensing_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigeration_system_working_fluid_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "suction_temperature_control_type",
                    default: SchemaDefault::String("ConstantSuctionTemperature"),
                    automatic: &[],
                },
                SchemaField {
                    name: "mechanical_subcooler_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "liquid_suction_heat_exchanger_subcooler_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sum_ua_suction_piping",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "suction_piping_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_compressor_stages",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "intercooler_type",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "shell_and_coil_intercooler_effectiveness",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "high_stage_compressor_or_compressorlist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:TranscriticalSystem" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "system_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "medium_temperature_refrigerated_case_or_walkin_or_caseandwalkinlist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "low_temperature_refrigerated_case_or_walkin_or_caseandwalkinlist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigeration_gas_cooler_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "high_pressure_compressor_or_compressorlist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "low_pressure_compressor_or_compressorlist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "receiver_pressure",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "subcooler_effectiveness",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "refrigeration_system_working_fluid_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sum_ua_suction_piping_for_medium_temperature_loads",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "medium_temperature_suction_piping_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sum_ua_suction_piping_for_low_temperature_loads",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "low_temperature_suction_piping_zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Refrigeration:TransferLoadList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: Some("transfer_loads"),
            extensible: &[SchemaField {
                name: "cascade_condenser_name_or_secondary_system_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "Refrigeration:WalkIn" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_coil_cooling_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "operating_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_cooling_source_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_total_heating_power",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_power_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_cooling_coil_fan_power",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_circulation_fan_power",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_total_lighting_power",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "lighting_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_type",
                    default: SchemaDefault::String("Electric"),
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_control_type",
                    default: SchemaDefault::String("TimeSchedule"),
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_drip_down_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "defrost_power",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature_termination_defrost_fraction_to_ice",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "restocking_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "average_refrigerant_charge_inventory",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "insulated_floor_surface_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "insulated_floor_u_value",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: Some("zone_data"),
            extensible: &[
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "total_insulated_surface_area_facing_zone",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "insulated_surface_u_value_facing_zone",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "area_of_glass_reach_in_doors_facing_zone",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "height_of_glass_reach_in_doors_facing_zone",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "glass_reach_in_door_u_value_facing_zone",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "glass_reach_in_door_opening_schedule_name_facing_zone",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "area_of_stocking_doors_facing_zone",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "height_of_stocking_doors_facing_zone",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "stocking_door_u_value_facing_zone",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "stocking_door_opening_schedule_name_facing_zone",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "stocking_door_opening_protection_type_facing_zone",
                    default: SchemaDefault::String("AirCurtain"),
                    automatic: &[],
                },
            ],
        },
        "RunPeriod" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "begin_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "begin_day_of_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "begin_year",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_day_of_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_year",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "day_of_week_for_start_day",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "use_weather_file_holidays_and_special_days",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
                SchemaField {
                    name: "use_weather_file_daylight_saving_period",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
                SchemaField {
                    name: "apply_weekend_holiday_rule",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "use_weather_file_rain_indicators",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
                SchemaField {
                    name: "use_weather_file_snow_indicators",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
                SchemaField {
                    name: "treat_weather_as_actual",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "first_hour_interpolation_starting_values",
                    default: SchemaDefault::String("Hour24"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Schedule:Compact" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_type_limits_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("data"),
            extensible: &[SchemaField {
                name: "field",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "Schedule:Constant" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_type_limits_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "hourly_value",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Schedule:Day:Hourly" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_type_limits_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_1",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_2",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_3",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_4",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_5",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_6",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_7",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_8",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_9",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_10",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_11",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_12",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_13",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_14",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_15",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_16",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_17",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_18",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_19",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_20",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_21",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_22",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_23",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "hour_24",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Schedule:Day:Interval" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_type_limits_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "interpolate_to_timestep",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
            ],
            extension: Some("data"),
            extensible: &[
                SchemaField {
                    name: "time",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "value_until_time",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "Schedule:Day:List" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_type_limits_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "interpolate_to_timestep",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "minutes_per_item",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("extensions"),
            extensible: &[SchemaField {
                name: "value",
                default: SchemaDefault::NonString,
                automatic: &[],
            }],
        },
        "Schedule:File" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_type_limits_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "file_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "column_number",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rows_to_skip_at_top",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "number_of_hours_of_data",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "column_separator",
                    default: SchemaDefault::String("Comma"),
                    automatic: &[],
                },
                SchemaField {
                    name: "interpolate_to_timestep",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "minutes_per_item",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "adjust_schedule_for_daylight_savings",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Schedule:File:Shading" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[SchemaField {
                name: "file_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: None,
            extensible: &[],
        },
        "Schedule:Week:Compact" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: Some("data"),
            extensible: &[
                SchemaField {
                    name: "daytype_list",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "Schedule:Week:Daily" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sunday_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "monday_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "tuesday_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wednesday_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thursday_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "friday_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "saturday_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "holiday_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "summerdesignday_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "winterdesignday_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "customday1_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "customday2_schedule_day_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Schedule:Year" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_type_limits_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("schedule_weeks"),
            extensible: &[
                SchemaField {
                    name: "schedule_week_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "start_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "start_day",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_day",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "ScheduleTypeLimits" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "lower_limit_value",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "upper_limit_value",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "numeric_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "unit_type",
                    default: SchemaDefault::String("Dimensionless"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ShadowCalculation" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "shading_calculation_method",
                    default: SchemaDefault::String("PolygonClipping"),
                    automatic: &[],
                },
                SchemaField {
                    name: "shading_calculation_update_frequency_method",
                    default: SchemaDefault::String("Periodic"),
                    automatic: &[],
                },
                SchemaField {
                    name: "shading_calculation_update_frequency",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_figures_in_shadow_overlap_calculations",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "polygon_clipping_algorithm",
                    default: SchemaDefault::String("SutherlandHodgman"),
                    automatic: &[],
                },
                SchemaField {
                    name: "pixel_counting_resolution",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "sky_diffuse_modeling_algorithm",
                    default: SchemaDefault::String("SimpleSkyDiffuseModeling"),
                    automatic: &[],
                },
                SchemaField {
                    name: "output_external_shading_calculation_results",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "disable_self_shading_within_shading_zone_groups",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "disable_self_shading_from_shading_zone_groups_to_other_zones",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
            ],
            extension: Some("shading_zone_groups"),
            extensible: &[SchemaField {
                name: "shading_zone_group_zonelist_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "SimulationControl" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "do_zone_sizing_calculation",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "do_system_sizing_calculation",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "do_plant_sizing_calculation",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "run_simulation_for_sizing_periods",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
                SchemaField {
                    name: "run_simulation_for_weather_file_run_periods",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
                SchemaField {
                    name: "do_hvac_sizing_simulation_for_sizing_periods",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_number_of_hvac_sizing_simulation_passes",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Site:Location" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "latitude",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "longitude",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "time_zone",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "elevation",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "keep_site_location_information",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Site:GroundTemperature:BuildingSurface" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "january_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "february_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "march_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "april_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "may_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "june_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "july_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "august_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "september_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "october_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "november_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "december_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Site:GroundTemperature:FCfactorMethod" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "january_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "february_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "march_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "april_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "may_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "june_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "july_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "august_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "september_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "october_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "november_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "december_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Site:GroundTemperature:Shallow" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "january_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "february_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "march_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "april_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "may_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "june_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "july_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "august_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "september_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "october_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "november_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "december_surface_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Site:GroundTemperature:Deep" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "january_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "february_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "march_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "april_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "may_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "june_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "july_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "august_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "september_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "october_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "november_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "december_deep_ground_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Sizing:Parameters" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "heating_sizing_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_sizing_factor",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "timesteps_in_averaging_window",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Sizing:Plant" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "plant_or_condenser_loop_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "loop_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_loop_exit_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "loop_design_temperature_difference",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "sizing_option",
                    default: SchemaDefault::String("NonCoincident"),
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_timesteps_in_averaging_window",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "coincident_sizing_factor_mode",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Sizing:System" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "airloop_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "type_of_load_to_size_on",
                    default: SchemaDefault::String("Sensible"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_outdoor_air_flow_rate",
                    default: SchemaDefault::String("Autosize"),
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "central_heating_maximum_system_air_flow_ratio",
                    default: SchemaDefault::String("Autosize"),
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "preheat_design_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "preheat_design_humidity_ratio",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "precool_design_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "precool_design_humidity_ratio",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "central_cooling_design_supply_air_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "central_heating_design_supply_air_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "type_of_zone_sum_to_use",
                    default: SchemaDefault::String("NonCoincident"),
                    automatic: &[],
                },
                SchemaField {
                    name: "100_outdoor_air_in_cooling",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "100_outdoor_air_in_heating",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "central_cooling_design_supply_air_humidity_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "central_heating_design_supply_air_humidity_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_supply_air_flow_rate_method",
                    default: SchemaDefault::String("DesignDay"),
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_supply_air_flow_rate",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_supply_air_flow_rate_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_fraction_of_autosized_cooling_supply_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_supply_air_flow_rate_per_unit_cooling_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_supply_air_flow_rate_method",
                    default: SchemaDefault::String("DesignDay"),
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_supply_air_flow_rate",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_supply_air_flow_rate_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_fraction_of_autosized_heating_supply_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_fraction_of_autosized_cooling_supply_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_supply_air_flow_rate_per_unit_heating_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "system_outdoor_air_method",
                    default: SchemaDefault::String("ZoneSum"),
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_maximum_outdoor_air_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_design_capacity_method",
                    default: SchemaDefault::String("CoolingDesignCapacity"),
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_design_capacity",
                    default: SchemaDefault::String("Autosize"),
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "cooling_design_capacity_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_autosized_cooling_design_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_design_capacity_method",
                    default: SchemaDefault::String("HeatingDesignCapacity"),
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_design_capacity",
                    default: SchemaDefault::String("Autosize"),
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "heating_design_capacity_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_autosized_heating_design_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "central_cooling_capacity_control_method",
                    default: SchemaDefault::String("OnOff"),
                    automatic: &[],
                },
                SchemaField {
                    name: "occupant_diversity",
                    default: SchemaDefault::String("Autosize"),
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "heating_coil_sizing_method",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_heating_capacity_to_cooling_capacity_sizing_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "Sizing:Zone" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "zone_or_zonelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_cooling_design_supply_air_temperature_input_method",
                    default: SchemaDefault::String("SupplyAirTemperature"),
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_cooling_design_supply_air_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_cooling_design_supply_air_temperature_difference",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_heating_design_supply_air_temperature_input_method",
                    default: SchemaDefault::String("SupplyAirTemperature"),
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_heating_design_supply_air_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_heating_design_supply_air_temperature_difference",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_cooling_design_supply_air_humidity_ratio",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_heating_design_supply_air_humidity_ratio",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_specification_outdoor_air_object_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_heating_sizing_factor",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_cooling_sizing_factor",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_design_air_flow_method",
                    default: SchemaDefault::String("DesignDay"),
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_design_air_flow_rate",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_minimum_air_flow_per_zone_floor_area",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_minimum_air_flow",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_minimum_air_flow_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_design_air_flow_method",
                    default: SchemaDefault::String("DesignDay"),
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_design_air_flow_rate",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_maximum_air_flow_per_zone_floor_area",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_maximum_air_flow",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_maximum_air_flow_fraction",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_specification_zone_air_distribution_object_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "account_for_dedicated_outdoor_air_system",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "dedicated_outdoor_air_system_control_strategy",
                    default: SchemaDefault::String("NeutralSupplyAir"),
                    automatic: &[],
                },
                SchemaField {
                    name: "dedicated_outdoor_air_low_setpoint_temperature_for_design",
                    default: SchemaDefault::String("Autosize"),
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "dedicated_outdoor_air_high_setpoint_temperature_for_design",
                    default: SchemaDefault::String("Autosize"),
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "zone_load_sizing_method",
                    default: SchemaDefault::String("Sensible Load Only No Latent Load"),
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_latent_cooling_design_supply_air_humidity_ratio_input_method",
                    default: SchemaDefault::String("HumidityRatioDifference"),
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_dehumidification_design_supply_air_humidity_ratio",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_cooling_design_supply_air_humidity_ratio_difference",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_latent_heating_design_supply_air_humidity_ratio_input_method",
                    default: SchemaDefault::String("HumidityRatioDifference"),
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_humidification_design_supply_air_humidity_ratio",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_humidification_design_supply_air_humidity_ratio_difference",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_humidistat_dehumidification_set_point_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_humidistat_humidification_set_point_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "type_of_space_sum_to_use",
                    default: SchemaDefault::String("Coincident"),
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_coil_sizing_method",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_heating_capacity_to_cooling_load_sizing_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "SizingPeriod:DesignDay" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "day_of_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "day_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_dry_bulb_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "daily_dry_bulb_temperature_range",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "dry_bulb_temperature_range_modifier_type",
                    default: SchemaDefault::String("DefaultMultipliers"),
                    automatic: &[],
                },
                SchemaField {
                    name: "dry_bulb_temperature_range_modifier_day_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "humidity_condition_type",
                    default: SchemaDefault::String("WetBulb"),
                    automatic: &[],
                },
                SchemaField {
                    name: "wetbulb_or_dewpoint_at_maximum_dry_bulb",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "humidity_condition_day_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "humidity_ratio_at_maximum_dry_bulb",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "enthalpy_at_maximum_dry_bulb",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "daily_wet_bulb_temperature_range",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "barometric_pressure",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_speed",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rain_indicator",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "snow_indicator",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "daylight_saving_time_indicator",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "solar_model_indicator",
                    default: SchemaDefault::String("ASHRAEClearSky"),
                    automatic: &[],
                },
                SchemaField {
                    name: "beam_solar_day_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "diffuse_solar_day_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ashrae_clear_sky_optical_depth_for_beam_irradiance_taub_",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "ashrae_clear_sky_optical_depth_for_diffuse_irradiance_taud_",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "sky_clearness",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_number_warmup_days",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "begin_environment_reset_mode",
                    default: SchemaDefault::String("FullResetAtBeginEnvironment"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "SizingPeriod:WeatherFileConditionType" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "period_selection",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "day_of_week_for_start_day",
                    default: SchemaDefault::String("Monday"),
                    automatic: &[],
                },
                SchemaField {
                    name: "use_weather_file_daylight_saving_period",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
                SchemaField {
                    name: "use_weather_file_rain_and_snow_indicators",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "SizingPeriod:WeatherFileDays" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "begin_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "begin_day_of_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_day_of_month",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "day_of_week_for_start_day",
                    default: SchemaDefault::String("Monday"),
                    automatic: &[],
                },
                SchemaField {
                    name: "use_weather_file_daylight_saving_period",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
                SchemaField {
                    name: "use_weather_file_rain_and_snow_indicators",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "SteamEquipment" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_or_space_or_spacelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level_calculation_method",
                    default: SchemaDefault::String("EquipmentLevel"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_level",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "power_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "power_per_person",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_latent",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_radiant",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_lost",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "SurfaceProperty:ExteriorNaturalVentedCavity" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "boundary_conditions_model_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "area_fraction_of_openings",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "thermal_emissivity_of_exterior_baffle_material",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "solar_absorbtivity_of_exterior_baffle",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "height_scale_for_buoyancy_driven_ventilation",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "effective_thickness_of_cavity_behind_exterior_baffle",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ratio_of_actual_surface_area_to_projected_surface_area",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "roughness_of_exterior_surface",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "effectiveness_for_perforations_with_respect_to_wind",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "discharge_coefficient_for_openings_with_respect_to_buoyancy_driven_flow",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: Some("surface"),
            extensible: &[SchemaField {
                name: "surface_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "SurfaceConvectionAlgorithm:Inside" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[SchemaField {
                name: "algorithm",
                default: SchemaDefault::String("TARP"),
                automatic: &[],
            }],
            extension: None,
            extensible: &[],
        },
        "SurfaceConvectionAlgorithm:Outside" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[SchemaField {
                name: "algorithm",
                default: SchemaDefault::String("DOE-2"),
                automatic: &[],
            }],
            extension: None,
            extensible: &[],
        },
        "NodeList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[SchemaField {
                name: "name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
            extension: Some("nodes"),
            extensible: &[SchemaField {
                name: "node_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "OutdoorAir:Node" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "height_above_ground",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "drybulb_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wetbulb_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_speed_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_direction_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_pressure_coefficient_curve_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "symmetric_wind_pressure_coefficient_curve",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_angle_type",
                    default: SchemaDefault::String("Absolute"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "OutdoorAir:NodeList" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[],
            extension: Some("nodes"),
            extensible: &[SchemaField {
                name: "node_or_nodelist_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "ZoneCoolTower:Shower" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "water_supply_storage_tank_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "flow_control_type",
                    default: SchemaDefault::String("WindDrivenFlow"),
                    automatic: &[],
                },
                SchemaField {
                    name: "pump_flow_rate_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_water_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "effective_tower_height",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "airflow_outlet_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_indoor_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_water_loss",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_of_flow_schedule",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "rated_power_consumption",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneHVAC:EquipmentList" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "load_distribution_scheme",
                    default: SchemaDefault::String("SequentialLoad"),
                    automatic: &[],
                },
            ],
            extension: Some("equipment"),
            extensible: &[
                SchemaField {
                    name: "zone_equipment_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_equipment_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_equipment_cooling_sequence",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_equipment_heating_or_no_load_sequence",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_equipment_sequential_cooling_fraction_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_equipment_sequential_heating_fraction_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "ZoneHVAC:EquipmentConnections" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_conditioning_equipment_list_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_air_inlet_node_or_nodelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_air_exhaust_node_or_nodelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_air_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_return_air_node_or_nodelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_return_air_node_1_flow_rate_fraction_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_return_air_node_1_flow_rate_basis_node_or_nodelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "SpaceHVAC:EquipmentConnections" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_air_inlet_node_or_nodelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_air_exhaust_node_or_nodelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_air_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_return_air_node_or_nodelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_return_air_node_1_flow_rate_fraction_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "space_return_air_node_1_flow_rate_basis_node_or_nodelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "AirLoopHVAC:SupplyPath" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "supply_air_path_inlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("components"),
            extensible: &[
                SchemaField {
                    name: "component_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "AirLoopHVAC:ReturnPath" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "return_air_path_outlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("components"),
            extensible: &[
                SchemaField {
                    name: "component_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "component_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
        },
        "Timestep" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[SchemaField {
                name: "number_of_timesteps_per_hour",
                default: SchemaDefault::NonString,
                automatic: &[],
            }],
            extension: None,
            extensible: &[],
        },
        "Version" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[SchemaField {
                name: "version_identifier",
                default: SchemaDefault::NonString,
                automatic: &[],
            }],
            extension: None,
            extensible: &[],
        },
        "Zone" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "direction_of_relative_north",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "x_origin",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "y_origin",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "z_origin",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "type",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "multiplier",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "ceiling_height",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "volume",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "floor_area",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "zone_inside_convection_algorithm",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_outside_convection_algorithm",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "part_of_total_floor_area",
                    default: SchemaDefault::String("Yes"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneAirBalance:OutdoorAir" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_balance_method",
                    default: SchemaDefault::String("Quadrature"),
                    automatic: &[],
                },
                SchemaField {
                    name: "induced_outdoor_air_due_to_unbalanced_duct_leakage",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "induced_outdoor_air_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneInfiltration:EffectiveLeakageArea" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "effective_air_leakage_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "stack_coefficient",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_coefficient",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneInfiltration:FlowCoefficient" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "flow_coefficient",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "stack_coefficient",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "pressure_exponent",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "wind_coefficient",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "shelter_factor",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneInfiltration:DesignFlowRate" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_or_space_or_spacelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_flow_rate_calculation_method",
                    default: SchemaDefault::String("Flow/Zone"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "flow_rate_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "flow_rate_per_exterior_surface_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_changes_per_hour",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "constant_term_coefficient",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature_term_coefficient",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "velocity_term_coefficient",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "velocity_squared_term_coefficient",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "density_basis",
                    default: SchemaDefault::String("Outdoor"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneVentilation:DesignFlowRate" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_or_space_or_spacelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_flow_rate_calculation_method",
                    default: SchemaDefault::String("Flow/Zone"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "flow_rate_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "flow_rate_per_person",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_changes_per_hour",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "ventilation_type",
                    default: SchemaDefault::String("Natural"),
                    automatic: &[],
                },
                SchemaField {
                    name: "fan_pressure_rise",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "fan_total_efficiency",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "constant_term_coefficient",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature_term_coefficient",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "velocity_term_coefficient",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "velocity_squared_term_coefficient",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_indoor_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_indoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_indoor_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_indoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "delta_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "delta_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_outdoor_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_outdoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_outdoor_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_outdoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_wind_speed",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "density_basis",
                    default: SchemaDefault::String("Outdoor"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneVentilation:WindandStackOpenArea" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "opening_area",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "opening_area_fraction_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "opening_effectiveness",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "effective_angle",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "height_difference",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "discharge_coefficient_for_opening",
                    default: SchemaDefault::String("Autocalculate"),
                    automatic: &[SchemaAutomatic::Autocalculate],
                },
                SchemaField {
                    name: "minimum_indoor_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_indoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_indoor_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_indoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "delta_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "delta_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_outdoor_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_outdoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_outdoor_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_outdoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_wind_speed",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneMixing" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_flow_rate_calculation_method",
                    default: SchemaDefault::String("Flow/Zone"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "flow_rate_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "flow_rate_per_person",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_changes_per_hour",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "source_zone_or_space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "delta_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "delta_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_receiving_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_receiving_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_source_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_source_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_outdoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_outdoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneCrossMixing" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_flow_rate_calculation_method",
                    default: SchemaDefault::String("Flow/Zone"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "flow_rate_per_floor_area",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "flow_rate_per_person",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_changes_per_hour",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "source_zone_or_space_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "delta_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "delta_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_receiving_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_receiving_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_source_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_source_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_outdoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_outdoor_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneRefrigerationDoorMixing" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_space_name_1",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_space_name_2",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "door_height",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "door_area",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "door_protection_type",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneAirHeatBalanceAlgorithm" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "algorithm",
                    default: SchemaDefault::String("ThirdOrderBackwardDifference"),
                    automatic: &[],
                },
                SchemaField {
                    name: "do_space_heat_balance_for_sizing",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
                SchemaField {
                    name: "do_space_heat_balance_for_simulation",
                    default: SchemaDefault::String("No"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneBaseboard:OutdoorTemperatureControlled" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_or_space_or_spacelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "capacity_at_low_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "low_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "capacity_at_high_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "high_temperature",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "fraction_radiant",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_use_subcategory",
                    default: SchemaDefault::String("General"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneContaminantSourceAndSink:CarbonDioxide" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_generation_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneHVAC:RefrigerationChillerSet" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_inlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "air_outlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: Some("chillers"),
            extensible: &[SchemaField {
                name: "air_chiller_name",
                default: SchemaDefault::Absent,
                automatic: &[],
            }],
        },
        "ThermostatSetpoint:DualSetpoint" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_setpoint_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_setpoint_temperature_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneControl:Thermostat" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_or_zonelist_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_type_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_1_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_1_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_2_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_2_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_3_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_3_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_4_object_type",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "control_4_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "temperature_difference_between_cutout_and_setpoint",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "ZoneHVAC:IdealLoadsAirSystem" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_supply_air_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "zone_exhaust_air_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "system_inlet_air_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_heating_supply_air_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_cooling_supply_air_temperature",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_heating_supply_air_humidity_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "minimum_cooling_supply_air_humidity_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_limit",
                    default: SchemaDefault::String("NoLimit"),
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_heating_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "maximum_sensible_heating_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "cooling_limit",
                    default: SchemaDefault::String("NoLimit"),
                    automatic: &[],
                },
                SchemaField {
                    name: "maximum_cooling_air_flow_rate",
                    default: SchemaDefault::Absent,
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "maximum_total_cooling_capacity",
                    default: SchemaDefault::Absent,
                    automatic: &[SchemaAutomatic::Autosize],
                },
                SchemaField {
                    name: "heating_availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_availability_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "dehumidification_control_type",
                    default: SchemaDefault::String("ConstantSensibleHeatRatio"),
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_sensible_heat_ratio",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "humidification_control_type",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "design_specification_outdoor_air_object_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "outdoor_air_inlet_node_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "demand_controlled_ventilation_type",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "outdoor_air_economizer_type",
                    default: SchemaDefault::String("NoEconomizer"),
                    automatic: &[],
                },
                SchemaField {
                    name: "heat_recovery_type",
                    default: SchemaDefault::String("None"),
                    automatic: &[],
                },
                SchemaField {
                    name: "sensible_heat_recovery_effectiveness",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "latent_heat_recovery_effectiveness",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "design_specification_zonehvac_sizing_object_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_fuel_efficiency_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "heating_fuel_type",
                    default: SchemaDefault::String("DistrictHeatingWater"),
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_fuel_efficiency_schedule_name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "cooling_fuel_type",
                    default: SchemaDefault::String("DistrictCooling"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "RunPeriodControl:SpecialDays" => SchemaDefinition {
            has_name: true,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "name",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "start_date",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "duration",
                    default: SchemaDefault::NonString,
                    automatic: &[],
                },
                SchemaField {
                    name: "special_day_type",
                    default: SchemaDefault::String("Holiday"),
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        "RunPeriodControl:DaylightSavingTime" => SchemaDefinition {
            has_name: false,
            name_has_default: false,
            regular: &[
                SchemaField {
                    name: "start_date",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
                SchemaField {
                    name: "end_date",
                    default: SchemaDefault::Absent,
                    automatic: &[],
                },
            ],
            extension: None,
            extensible: &[],
        },
        _ => return None,
    })
}
