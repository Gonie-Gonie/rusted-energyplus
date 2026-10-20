//! Literal six-family official field order/defaults/names, no count ownership.
use super::{DefaultValue, Field};
const F0: &[Field] = &[
    Field {
        json: "name",
        legacy: "Name",
        default: DefaultValue::Alpha("NONE"),
        retain_case: true,
    },
    Field {
        json: "north_axis",
        legacy: "North Axis",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "terrain",
        legacy: "Terrain",
        default: DefaultValue::Alpha("Suburbs"),
        retain_case: false,
    },
    Field {
        json: "loads_convergence_tolerance_value",
        legacy: "Loads Convergence Tolerance Value",
        default: DefaultValue::Numeric(0.04),
        retain_case: false,
    },
    Field {
        json: "temperature_convergence_tolerance_value",
        legacy: "Temperature Convergence Tolerance Value",
        default: DefaultValue::Numeric(0.4),
        retain_case: false,
    },
    Field {
        json: "solar_distribution",
        legacy: "Solar Distribution",
        default: DefaultValue::Alpha("FullExterior"),
        retain_case: false,
    },
    Field {
        json: "maximum_number_of_warmup_days",
        legacy: "Maximum Number of Warmup Days",
        default: DefaultValue::Numeric(25.0),
        retain_case: false,
    },
    Field {
        json: "minimum_number_of_warmup_days",
        legacy: "Minimum Number of Warmup Days",
        default: DefaultValue::Numeric(1.0),
        retain_case: false,
    },
];
const F1: &[Field] = &[Field {
    json: "algorithm",
    legacy: "Algorithm",
    default: DefaultValue::Alpha("TARP"),
    retain_case: false,
}];
const F2: &[Field] = &[Field {
    json: "algorithm",
    legacy: "Algorithm",
    default: DefaultValue::Alpha("DOE-2"),
    retain_case: false,
}];
const F3: &[Field] = &[
    Field {
        json: "algorithm",
        legacy: "Algorithm",
        default: DefaultValue::Alpha("ConductionTransferFunction"),
        retain_case: false,
    },
    Field {
        json: "surface_temperature_upper_limit",
        legacy: "Surface Temperature Upper Limit",
        default: DefaultValue::Numeric(200.0),
        retain_case: false,
    },
    Field {
        json: "minimum_surface_convection_heat_transfer_coefficient_value",
        legacy: "Minimum Surface Convection Heat Transfer Coefficient Value",
        default: DefaultValue::Numeric(0.1),
        retain_case: false,
    },
    Field {
        json: "maximum_surface_convection_heat_transfer_coefficient_value",
        legacy: "Maximum Surface Convection Heat Transfer Coefficient Value",
        default: DefaultValue::Numeric(1000.0),
        retain_case: false,
    },
];
const F4: &[Field] = &[
    Field {
        json: "algorithm",
        legacy: "Algorithm",
        default: DefaultValue::Alpha("ThirdOrderBackwardDifference"),
        retain_case: false,
    },
    Field {
        json: "do_space_heat_balance_for_sizing",
        legacy: "Do Space Heat Balance for Sizing",
        default: DefaultValue::Alpha("No"),
        retain_case: false,
    },
    Field {
        json: "do_space_heat_balance_for_simulation",
        legacy: "Do Space Heat Balance for Simulation",
        default: DefaultValue::Alpha("No"),
        retain_case: false,
    },
];
const F5: &[Field] = &[
    Field {
        json: "name",
        legacy: "Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "outside_layer",
        legacy: "Outside Layer",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_2",
        legacy: "Layer 2",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_3",
        legacy: "Layer 3",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_4",
        legacy: "Layer 4",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_5",
        legacy: "Layer 5",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_6",
        legacy: "Layer 6",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_7",
        legacy: "Layer 7",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_8",
        legacy: "Layer 8",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_9",
        legacy: "Layer 9",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "layer_10",
        legacy: "Layer 10",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
];
// OtherEquipment IDD22738-22829: same local getter, not a new count owner.
const OTHER: &[Field] = &[
    Field {
        json: "name",
        legacy: "Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "fuel_type",
        legacy: "Fuel Type",
        default: DefaultValue::Alpha("None"),
        retain_case: false,
    },
    Field {
        json: "zone_or_zonelist_or_space_or_spacelist_name",
        legacy: "Zone or ZoneList or Space or SpaceList Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "schedule_name",
        legacy: "Schedule Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "design_level_calculation_method",
        legacy: "Design Level Calculation Method",
        default: DefaultValue::Alpha("EquipmentLevel"),
        retain_case: false,
    },
    Field {
        json: "design_level",
        legacy: "Design Level",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "power_per_floor_area",
        legacy: "Power per Floor Area",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "power_per_person",
        legacy: "Power per Person",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "fraction_latent",
        legacy: "Fraction Latent",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "fraction_radiant",
        legacy: "Fraction Radiant",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "fraction_lost",
        legacy: "Fraction Lost",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "carbon_dioxide_generation_rate",
        legacy: "Carbon Dioxide Generation Rate",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "end_use_subcategory",
        legacy: "End-Use Subcategory",
        default: DefaultValue::Alpha("General"),
        retain_case: true,
    },
];
// Genuine Exterior:Lights IDD27346-27372: actual 4alpha/1number order.
const EXTERIOR_LIGHTS: &[Field] = &[
    Field {
        json: "name",
        legacy: "Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "schedule_name",
        legacy: "Schedule Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "design_level",
        legacy: "Design Level",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "control_option",
        legacy: "Control Option",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "end_use_subcategory",
        legacy: "End-Use Subcategory",
        default: DefaultValue::Alpha("General"),
        retain_case: true,
    },
];
// Literal Site:Location IDD995?1035, min_fields5, 2alpha4number.
const SITE_LOCATION: &[Field] = &[
    Field {
        json: "name",
        legacy: "Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "latitude",
        legacy: "Latitude",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "longitude",
        legacy: "Longitude",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "time_zone",
        legacy: "Time Zone",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "elevation",
        legacy: "Elevation",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "keep_site_location_information",
        legacy: "Keep Site Location Information",
        default: DefaultValue::Alpha("No"),
        retain_case: false,
    },
];
// Actual unnamed Output:Meter four-family IDD108387-108495; default Hourly.
const OUTPUT_METER: &[Field] = &[
    Field {
        json: "key_name",
        legacy: "Key Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "reporting_frequency",
        legacy: "Reporting Frequency",
        default: DefaultValue::Alpha("Hourly"),
        retain_case: false,
    },
];
// Actual RunPeriod IDD1398–1487,9alpha6numeric,min_fields7; no new query owner.
const RUN_PERIOD: &[Field] = &[
    Field {
        json: "name",
        legacy: "Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "begin_month",
        legacy: "Begin Month",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "begin_day_of_month",
        legacy: "Begin Day of Month",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "begin_year",
        legacy: "Begin Year",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "end_month",
        legacy: "End Month",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "end_day_of_month",
        legacy: "End Day of Month",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "end_year",
        legacy: "End Year",
        default: DefaultValue::Numeric(0.0),
        retain_case: false,
    },
    Field {
        json: "day_of_week_for_start_day",
        legacy: "Day of Week for Start Day",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "use_weather_file_holidays_and_special_days",
        legacy: "Use Weather File Holidays and Special Days",
        default: DefaultValue::Alpha("Yes"),
        retain_case: false,
    },
    Field {
        json: "use_weather_file_daylight_saving_period",
        legacy: "Use Weather File Daylight Saving Period",
        default: DefaultValue::Alpha("Yes"),
        retain_case: false,
    },
    Field {
        json: "apply_weekend_holiday_rule",
        legacy: "Apply Weekend Holiday Rule",
        default: DefaultValue::Alpha("No"),
        retain_case: false,
    },
    Field {
        json: "use_weather_file_rain_indicators",
        legacy: "Use Weather File Rain Indicators",
        default: DefaultValue::Alpha("Yes"),
        retain_case: false,
    },
    Field {
        json: "use_weather_file_snow_indicators",
        legacy: "Use Weather File Snow Indicators",
        default: DefaultValue::Alpha("Yes"),
        retain_case: false,
    },
    Field {
        json: "treat_weather_as_actual",
        legacy: "Treat Weather as Actual",
        default: DefaultValue::Alpha("No"),
        retain_case: false,
    },
    Field {
        json: "first_hour_interpolation_starting_values",
        legacy: "First Hour Interpolation Starting Values",
        default: DefaultValue::Alpha("Hour24"),
        retain_case: false,
    },
];
const SHADOW_CALCULATION: &[Field] = &[
    Field {
        json: "shading_calculation_method",
        legacy: "Shading Calculation Method",
        default: DefaultValue::Alpha("PolygonClipping"),
        retain_case: false,
    },
    Field {
        json: "shading_calculation_update_frequency_method",
        legacy: "Shading Calculation Update Frequency Method",
        default: DefaultValue::Alpha("Periodic"),
        retain_case: false,
    },
    Field {
        json: "shading_calculation_update_frequency",
        legacy: "Shading Calculation Update Frequency",
        default: DefaultValue::Numeric(20.0),
        retain_case: false,
    },
    Field {
        json: "maximum_figures_in_shadow_overlap_calculations",
        legacy: "Maximum Figures in Shadow Overlap Calculations",
        default: DefaultValue::Numeric(15000.0),
        retain_case: false,
    },
    Field {
        json: "polygon_clipping_algorithm",
        legacy: "Polygon Clipping Algorithm",
        default: DefaultValue::Alpha("SutherlandHodgman"),
        retain_case: false,
    },
    Field {
        json: "pixel_counting_resolution",
        legacy: "Pixel Counting Resolution",
        default: DefaultValue::Numeric(512.0),
        retain_case: false,
    },
    Field {
        json: "sky_diffuse_modeling_algorithm",
        legacy: "Sky Diffuse Modeling Algorithm",
        default: DefaultValue::Alpha("SimpleSkyDiffuseModeling"),
        retain_case: false,
    },
    Field {
        json: "output_external_shading_calculation_results",
        legacy: "Output External Shading Calculation Results",
        default: DefaultValue::Alpha("No"),
        retain_case: false,
    },
    Field {
        json: "disable_self_shading_within_shading_zone_groups",
        legacy: "Disable Self-Shading Within Shading Zone Groups",
        default: DefaultValue::Alpha("No"),
        retain_case: false,
    },
    Field {
        json: "disable_self_shading_from_shading_zone_groups_to_other_zones",
        legacy: "Disable Self-Shading From Shading Zone Groups to Other Zones",
        default: DefaultValue::Alpha("No"),
        retain_case: false,
    },
];
const SPECIAL_DAYS: &[Field] = &[
    Field {
        json: "name",
        legacy: "Name",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "start_date",
        legacy: "Start Date",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "duration",
        legacy: "Duration",
        default: DefaultValue::Numeric(1.0),
        retain_case: false,
    },
    Field {
        json: "special_day_type",
        legacy: "Special Day Type",
        default: DefaultValue::Alpha("Holiday"),
        retain_case: false,
    },
];
const DAYLIGHT_SAVING: &[Field] = &[
    Field {
        json: "start_date",
        legacy: "Start Date",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
    Field {
        json: "end_date",
        legacy: "End Date",
        default: DefaultValue::Alpha(""),
        retain_case: false,
    },
];
pub(super) fn fields(family: &str) -> Option<(&'static [Field], usize, bool)> {
    match family {
        "Building" => Some((F0, 8, true)),
        "ShadowCalculation" => Some((SHADOW_CALCULATION, 0, false)),
        "SurfaceConvectionAlgorithm:Inside" => Some((F1, 0, false)),
        "SurfaceConvectionAlgorithm:Outside" => Some((F2, 0, false)),
        "HeatBalanceAlgorithm" => Some((F3, 0, false)),
        "ZoneAirHeatBalanceAlgorithm" => Some((F4, 2, false)),
        "Construction" => Some((F5, 0, true)),
        "OtherEquipment" => Some((OTHER, 11, true)),
        "Exterior:Lights" => Some((EXTERIOR_LIGHTS, 0, true)),
        "Site:Location" => Some((SITE_LOCATION, 5, true)),
        "RunPeriod" => Some((RUN_PERIOD, 7, true)),
        "RunPeriodControl:SpecialDays" => Some((SPECIAL_DAYS, 4, true)),
        "RunPeriodControl:DaylightSavingTime" => Some((DAYLIGHT_SAVING, 2, false)),
        "SizingPeriod:DesignDay" => Some((super::design_day_schema::DESIGN_DAY, 0, true)),
        "Output:Meter"
        | "Output:Meter:MeterFileOnly"
        | "Output:Meter:Cumulative"
        | "Output:Meter:Cumulative:MeterFileOnly" => Some((OUTPUT_METER, 0, false)),
        _ => None,
    }
}
