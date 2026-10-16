//! Finite literal field grammar for the ordinary equipment input resolver.
use super::{blank, number, text_is};
use ep_raw_model::{FieldName, RawValue};
mod ground;

// These literal matches are the finite 26.1 field grammar, not a runtime schema
// engine. Existing compiler, geometry, schedules and thermal validation still
// run. The Source-reviewed branch restrictions below are deliberately narrower.
pub(super) fn admitted_field(family: &str, name: &str, value: &RawValue) -> bool {
    if let Some(admitted) = ground::admitted_field(family, name, value) {
        return admitted;
    }
    match (family, name) {
        // Actual parser metadata only, including the existing unnamed reports.
        (
            "GlobalGeometryRules"
            | "BuildingSurface:Detailed"
            | "Zone"
            | "Material"
            | "Material:NoMass"
            | "Version"
            | "HeatBalanceAlgorithm"
            | "Timestep"
            | "SimulationControl"
            | "Building"
            | "Construction"
            | "OtherEquipment"
            | "SurfaceConvectionAlgorithm:Inside"
            | "SurfaceConvectionAlgorithm:Outside"
            | "ZoneAirHeatBalanceAlgorithm"
            | "Output:VariableDictionary"
            | "Output:Surfaces:Drawing"
            | "Output:Constructions"
            | "Output:Variable",
            "idf_order" | "idf_max_fields" | "idf_max_extensible_fields",
        ) => {
            matches!(value, RawValue::Number(text) if text.parse::<usize>().is_ok())
        }
        ("Version", "version_identifier") => blank(value) || matches!(value, RawValue::String(_)),
        ("Timestep", "number_of_timesteps_per_hour") => {
            blank(value)
                || number(value).is_some_and(|n| n.fract() == 0.0 && (1.0..=60.0).contains(&n))
        }
        ("Building", "north_axis") => blank(value) || number(value).is_some(),
        ("Building", "terrain") => {
            blank(value) || text_is(value, &["", "City", "Country", "Ocean", "Suburbs", "Urban"])
        }
        ("Building", "loads_convergence_tolerance_value") => {
            blank(value) || number(value).is_some_and(|n| n <= 0.5 && n > 0.0)
        }
        ("Building", "temperature_convergence_tolerance_value") => {
            blank(value) || number(value).is_some_and(|n| n <= 0.5 && n > 0.0)
        }
        ("Building", "solar_distribution") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "FullExterior",
                        "FullExteriorWithReflections",
                        "FullInteriorAndExterior",
                        "FullInteriorAndExteriorWithReflections",
                        "MinimalShadowing",
                    ],
                )
        }
        ("Building", "maximum_number_of_warmup_days") => {
            blank(value) || number(value).is_some_and(|n| n.fract() == 0.0 && n > 0.0)
        }
        ("Building", "minimum_number_of_warmup_days") => {
            blank(value) || number(value).is_some_and(|n| n.fract() == 0.0 && n > 0.0)
        }
        ("HeatBalanceAlgorithm", "algorithm") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "CombinedHeatAndMoistureFiniteElement",
                        "ConductionFiniteDifference",
                        "ConductionTransferFunction",
                        "MoisturePenetrationDepthConductionTransferFunction",
                    ],
                )
        }
        ("HeatBalanceAlgorithm", "surface_temperature_upper_limit") => {
            blank(value) || number(value).is_some_and(|n| n >= 200.0)
        }
        ("HeatBalanceAlgorithm", "minimum_surface_convection_heat_transfer_coefficient_value") => {
            blank(value) || number(value).is_some_and(|n| n > 0.0)
        }
        ("HeatBalanceAlgorithm", "maximum_surface_convection_heat_transfer_coefficient_value") => {
            blank(value) || number(value).is_some_and(|n| n >= 1.0)
        }
        ("SurfaceConvectionAlgorithm:Inside", "algorithm") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "ASTMC1340",
                        "AdaptiveConvectionAlgorithm",
                        "CeilingDiffuser",
                        "Simple",
                        "TARP",
                    ],
                )
        }
        ("SurfaceConvectionAlgorithm:Outside", "algorithm") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "AdaptiveConvectionAlgorithm",
                        "DOE-2",
                        "MoWiTT",
                        "SimpleCombined",
                        "TARP",
                    ],
                )
        }
        ("SimulationControl", "do_zone_sizing_calculation") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("SimulationControl", "do_system_sizing_calculation") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("SimulationControl", "do_plant_sizing_calculation") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("SimulationControl", "run_simulation_for_sizing_periods") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("SimulationControl", "run_simulation_for_weather_file_run_periods") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("SimulationControl", "do_hvac_sizing_simulation_for_sizing_periods") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("SimulationControl", "maximum_number_of_hvac_sizing_simulation_passes") => {
            blank(value) || number(value).is_some_and(|n| n.fract() == 0.0 && n >= 1.0)
        }
        ("RunPeriod", "begin_month") => {
            blank(value)
                || number(value).is_some_and(|n| n.fract() == 0.0 && (1.0..=12.0).contains(&n))
        }
        ("RunPeriod", "begin_day_of_month") => {
            blank(value)
                || number(value).is_some_and(|n| n.fract() == 0.0 && (1.0..=31.0).contains(&n))
        }
        ("RunPeriod", "begin_year") => blank(value) || number(value).is_some(),
        ("RunPeriod", "end_month") => {
            blank(value)
                || number(value).is_some_and(|n| n.fract() == 0.0 && (1.0..=12.0).contains(&n))
        }
        ("RunPeriod", "end_day_of_month") => {
            blank(value)
                || number(value).is_some_and(|n| n.fract() == 0.0 && (1.0..=31.0).contains(&n))
        }
        ("RunPeriod", "end_year") => blank(value) || number(value).is_some(),
        ("RunPeriod", "day_of_week_for_start_day") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "Friday",
                        "Monday",
                        "Saturday",
                        "Sunday",
                        "Thursday",
                        "Tuesday",
                        "Wednesday",
                    ],
                )
        }
        ("RunPeriod", "use_weather_file_holidays_and_special_days") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("RunPeriod", "use_weather_file_daylight_saving_period") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("RunPeriod", "apply_weekend_holiday_rule") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("RunPeriod", "use_weather_file_rain_indicators") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("RunPeriod", "use_weather_file_snow_indicators") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("RunPeriod", "treat_weather_as_actual") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("RunPeriod", "first_hour_interpolation_starting_values") => {
            blank(value) || text_is(value, &["", "Hour1", "Hour24"])
        }
        ("Site:Location", "latitude") => {
            blank(value) || number(value).is_some_and(|n| (-90.0..=90.0).contains(&n))
        }
        ("Site:Location", "longitude") => {
            blank(value) || number(value).is_some_and(|n| (-180.0..=180.0).contains(&n))
        }
        ("Site:Location", "time_zone") => {
            blank(value) || number(value).is_some_and(|n| (-12.0..=14.0).contains(&n))
        }
        ("Site:Location", "elevation") => {
            blank(value) || number(value).is_some_and(|n| (-300.0..8900.0).contains(&n))
        }
        ("Site:Location", "keep_site_location_information") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("SizingPeriod:DesignDay", "month") => {
            blank(value)
                || number(value).is_some_and(|n| n.fract() == 0.0 && (1.0..=12.0).contains(&n))
        }
        ("SizingPeriod:DesignDay", "day_of_month") => {
            blank(value)
                || number(value).is_some_and(|n| n.fract() == 0.0 && (1.0..=31.0).contains(&n))
        }
        ("SizingPeriod:DesignDay", "day_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "CustomDay1",
                        "CustomDay2",
                        "Friday",
                        "Holiday",
                        "Monday",
                        "Saturday",
                        "SummerDesignDay",
                        "Sunday",
                        "Thursday",
                        "Tuesday",
                        "Wednesday",
                        "WinterDesignDay",
                    ],
                )
        }
        ("SizingPeriod:DesignDay", "maximum_dry_bulb_temperature") => {
            blank(value) || number(value).is_some_and(|n| (-90.0..=70.0).contains(&n))
        }
        ("SizingPeriod:DesignDay", "daily_dry_bulb_temperature_range") => {
            blank(value) || number(value).is_some_and(|n| n >= 0.0)
        }
        ("SizingPeriod:DesignDay", "dry_bulb_temperature_range_modifier_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "DefaultMultipliers",
                        "DifferenceSchedule",
                        "MultiplierSchedule",
                        "TemperatureProfileSchedule",
                    ],
                )
        }
        ("SizingPeriod:DesignDay", "dry_bulb_temperature_range_modifier_day_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("SizingPeriod:DesignDay", "humidity_condition_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "DewPoint",
                        "Enthalpy",
                        "HumidityRatio",
                        "RelativeHumiditySchedule",
                        "WetBulb",
                        "WetBulbProfileDefaultMultipliers",
                        "WetBulbProfileDifferenceSchedule",
                        "WetBulbProfileMultiplierSchedule",
                    ],
                )
        }
        ("SizingPeriod:DesignDay", "wetbulb_or_dewpoint_at_maximum_dry_bulb") => {
            blank(value) || number(value).is_some()
        }
        ("SizingPeriod:DesignDay", "humidity_condition_day_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("SizingPeriod:DesignDay", "humidity_ratio_at_maximum_dry_bulb") => {
            blank(value) || number(value).is_some()
        }
        ("SizingPeriod:DesignDay", "enthalpy_at_maximum_dry_bulb") => {
            blank(value) || number(value).is_some()
        }
        ("SizingPeriod:DesignDay", "daily_wet_bulb_temperature_range") => {
            blank(value) || number(value).is_some()
        }
        ("SizingPeriod:DesignDay", "barometric_pressure") => {
            blank(value) || number(value).is_some_and(|n| (31000.0..=120000.0).contains(&n))
        }
        ("SizingPeriod:DesignDay", "wind_speed") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=40.0).contains(&n))
        }
        ("SizingPeriod:DesignDay", "wind_direction") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=360.0).contains(&n))
        }
        ("SizingPeriod:DesignDay", "rain_indicator") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("SizingPeriod:DesignDay", "snow_indicator") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("SizingPeriod:DesignDay", "daylight_saving_time_indicator") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("SizingPeriod:DesignDay", "solar_model_indicator") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "ASHRAEClearSky",
                        "ASHRAETau",
                        "ASHRAETau2017",
                        "Schedule",
                        "ZhangHuang",
                    ],
                )
        }
        ("SizingPeriod:DesignDay", "beam_solar_day_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("SizingPeriod:DesignDay", "diffuse_solar_day_schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("SizingPeriod:DesignDay", "ashrae_clear_sky_optical_depth_for_beam_irradiance_taub_") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=1.2).contains(&n))
        }
        (
            "SizingPeriod:DesignDay",
            "ashrae_clear_sky_optical_depth_for_diffuse_irradiance_taud_",
        ) => blank(value) || number(value).is_some_and(|n| (0.0..=3.0).contains(&n)),
        ("SizingPeriod:DesignDay", "sky_clearness") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=1.2).contains(&n))
        }
        ("SizingPeriod:DesignDay", "maximum_number_warmup_days") => {
            blank(value) || number(value).is_some_and(|n| n.fract() == 0.0)
        }
        ("SizingPeriod:DesignDay", "begin_environment_reset_mode") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "FullResetAtBeginEnvironment",
                        "SuppressAllBeginEnvironmentResets",
                    ],
                )
        }
        ("Material:NoMass", "roughness") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "MediumRough",
                        "MediumSmooth",
                        "Rough",
                        "Smooth",
                        "VeryRough",
                        "VerySmooth",
                    ],
                )
        }
        ("Material:NoMass", "thermal_resistance") => {
            blank(value) || number(value).is_some_and(|n| n >= 0.001)
        }
        ("Material:NoMass", "thermal_absorptance") => {
            blank(value) || number(value).is_some_and(|n| n <= 0.99999 && n > 0.0)
        }
        ("Material:NoMass", "solar_absorptance") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
        }
        ("Material:NoMass", "visible_absorptance") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
        }
        ("Material", "roughness") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "MediumRough",
                        "MediumSmooth",
                        "Rough",
                        "Smooth",
                        "VeryRough",
                        "VerySmooth",
                    ],
                )
        }
        ("Material", "thickness") => blank(value) || number(value).is_some_and(|n| n > 0.0),
        ("Material", "conductivity") => blank(value) || number(value).is_some_and(|n| n > 0.0),
        ("Material", "density") => blank(value) || number(value).is_some_and(|n| n > 0.0),
        ("Material", "specific_heat") => blank(value) || number(value).is_some_and(|n| n >= 100.0),
        ("Material", "thermal_absorptance") => {
            blank(value) || number(value).is_some_and(|n| n <= 0.99999 && n > 0.0)
        }
        ("Material", "solar_absorptance") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
        }
        ("Material", "visible_absorptance") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
        }
        ("Construction", "outside_layer") => blank(value) || matches!(value, RawValue::String(_)),
        ("Construction", "layer_2") => blank(value) || matches!(value, RawValue::String(_)),
        ("Construction", "layer_3") => blank(value) || matches!(value, RawValue::String(_)),
        ("Construction", "layer_4") => blank(value) || matches!(value, RawValue::String(_)),
        ("Construction", "layer_5") => blank(value) || matches!(value, RawValue::String(_)),
        ("Construction", "layer_6") => blank(value) || matches!(value, RawValue::String(_)),
        ("Construction", "layer_7") => blank(value) || matches!(value, RawValue::String(_)),
        ("Construction", "layer_8") => blank(value) || matches!(value, RawValue::String(_)),
        ("Construction", "layer_9") => blank(value) || matches!(value, RawValue::String(_)),
        ("Construction", "layer_10") => blank(value) || matches!(value, RawValue::String(_)),
        ("Zone", "direction_of_relative_north") => blank(value) || number(value).is_some(),
        ("Zone", "x_origin") => blank(value) || number(value).is_some(),
        ("Zone", "y_origin") => blank(value) || number(value).is_some(),
        ("Zone", "z_origin") => blank(value) || number(value).is_some(),
        ("Zone", "type") => {
            blank(value)
                || number(value).is_some_and(|n| n.fract() == 0.0 && (1.0..=1.0).contains(&n))
        }
        ("Zone", "multiplier") => {
            blank(value) || number(value).is_some_and(|n| n.fract() == 0.0 && n >= 1.0)
        }
        ("Zone", "ceiling_height") => {
            blank(value) || (number(value).is_some() || text_is(value, &["", "Autocalculate"]))
        }
        ("Zone", "volume") => {
            blank(value) || (number(value).is_some() || text_is(value, &["", "Autocalculate"]))
        }
        ("Zone", "floor_area") => {
            blank(value) || (number(value).is_some() || text_is(value, &["", "Autocalculate"]))
        }
        ("Zone", "zone_inside_convection_algorithm") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "ASTMC1340",
                        "AdaptiveConvectionAlgorithm",
                        "CeilingDiffuser",
                        "Simple",
                        "TARP",
                        "TrombeWall",
                    ],
                )
        }
        ("Zone", "zone_outside_convection_algorithm") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "AdaptiveConvectionAlgorithm",
                        "DOE-2",
                        "MoWiTT",
                        "SimpleCombined",
                        "TARP",
                    ],
                )
        }
        ("Zone", "part_of_total_floor_area") => blank(value) || text_is(value, &["", "No", "Yes"]),
        ("ScheduleTypeLimits", "lower_limit_value") => blank(value) || number(value).is_some(),
        ("ScheduleTypeLimits", "upper_limit_value") => blank(value) || number(value).is_some(),
        ("ScheduleTypeLimits", "numeric_type") => {
            blank(value) || text_is(value, &["Continuous", "Discrete"])
        }
        ("ScheduleTypeLimits", "unit_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "ActivityLevel",
                        "Angle",
                        "Availability",
                        "Capacity",
                        "Control",
                        "ConvectionCoefficient",
                        "DeltaTemperature",
                        "Dimensionless",
                        "Mode",
                        "Percent",
                        "Power",
                        "PrecipitationRate",
                        "Temperature",
                        "Velocity",
                    ],
                )
        }
        ("GlobalGeometryRules", "starting_vertex_position") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "LowerLeftCorner",
                        "LowerRightCorner",
                        "UpperLeftCorner",
                        "UpperRightCorner",
                    ],
                )
        }
        ("GlobalGeometryRules", "vertex_entry_direction") => {
            blank(value) || text_is(value, &["Clockwise", "Counterclockwise"])
        }
        ("GlobalGeometryRules", "coordinate_system") => {
            blank(value) || text_is(value, &["Relative", "World"])
        }
        ("GlobalGeometryRules", "daylighting_reference_point_coordinate_system") => {
            blank(value) || text_is(value, &["", "Relative", "World"])
        }
        ("GlobalGeometryRules", "rectangular_surface_coordinate_system") => {
            blank(value) || text_is(value, &["", "Relative", "World"])
        }
        ("BuildingSurface:Detailed", "surface_type") => {
            blank(value) || text_is(value, &["Ceiling", "Floor", "Roof", "Wall"])
        }
        ("BuildingSurface:Detailed", "construction_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("BuildingSurface:Detailed", "zone_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("BuildingSurface:Detailed", "space_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("BuildingSurface:Detailed", "outside_boundary_condition") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "Adiabatic",
                        "Foundation",
                        "Ground",
                        "GroundBasementPreprocessorAverageFloor",
                        "GroundBasementPreprocessorAverageWall",
                        "GroundBasementPreprocessorLowerWall",
                        "GroundBasementPreprocessorUpperWall",
                        "GroundFCfactorMethod",
                        "GroundSlabPreprocessorAverage",
                        "GroundSlabPreprocessorCore",
                        "GroundSlabPreprocessorPerimeter",
                        "OtherSideCoefficients",
                        "OtherSideConditionsModel",
                        "Outdoors",
                        "Space",
                        "Surface",
                        "Zone",
                    ],
                )
        }
        ("BuildingSurface:Detailed", "outside_boundary_condition_object") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("BuildingSurface:Detailed", "sun_exposure") => {
            blank(value) || text_is(value, &["", "NoSun", "SunExposed"])
        }
        ("BuildingSurface:Detailed", "wind_exposure") => {
            blank(value) || text_is(value, &["", "NoWind", "WindExposed"])
        }
        ("BuildingSurface:Detailed", "view_factor_to_ground") => {
            blank(value)
                || (number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
                    || text_is(value, &["", "Autocalculate"]))
        }
        ("BuildingSurface:Detailed", "number_of_vertices") => {
            blank(value)
                || (number(value).is_some_and(|n| n >= 3.0)
                    || text_is(value, &["", "Autocalculate"]))
        }
        ("BuildingSurface:Detailed", "vertices") => {
            matches!(value, RawValue::Array(rows) if rows.iter().all(|entry| matches!(entry, RawValue::Object(row) if row.keys().all(|key| ["vertex_x_coordinate", "vertex_y_coordinate", "vertex_z_coordinate"].contains(&key.0.as_str())) && row.get(&FieldName("vertex_x_coordinate".into())).is_some_and(|value| number(value).is_some()) && row.get(&FieldName("vertex_y_coordinate".into())).is_some_and(|value| number(value).is_some()) && row.get(&FieldName("vertex_z_coordinate".into())).is_some_and(|value| number(value).is_some()))))
        }
        ("Output:Diagnostics", "diagnostics") => {
            matches!(value, RawValue::Array(rows) if rows.iter().all(|entry| matches!(entry, RawValue::Object(row) if row.keys().all(|key| ["key"].contains(&key.0.as_str())) && row.get(&FieldName("key".into())).is_some_and(|value| text_is(value, &["DisplayAdvancedReportVariables", "ReportDetailedWarmupConvergence"])))))
        }
        ("Output:VariableDictionary", "key_field") => {
            blank(value) || text_is(value, &["", "IDF", "regular"])
        }
        ("Output:VariableDictionary", "sort_option") => {
            blank(value) || text_is(value, &["Name", "Unsorted"])
        }
        ("Output:Surfaces:Drawing", "report_type") => {
            blank(value) || text_is(value, &["DXF", "DXF:WireFrame", "VRML"])
        }
        ("Output:Surfaces:Drawing", "report_specifications_1") => {
            blank(value)
                || text_is(
                    value,
                    &["", "RegularPolyline", "ThickPolyline", "Triangulate3DFace"],
                )
        }
        ("Output:Surfaces:Drawing", "report_specifications_2") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("Output:Constructions", "details_type_1") => {
            blank(value) || text_is(value, &["Constructions", "Materials"])
        }
        ("Output:Constructions", "details_type_2") => {
            blank(value) || text_is(value, &["Constructions", "Materials"])
        }
        ("Output:Meter:MeterFileOnly", "key_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("Output:Meter:MeterFileOnly", "reporting_frequency") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "Annual",
                        "Daily",
                        "Detailed",
                        "Environment",
                        "Hourly",
                        "Monthly",
                        "RunPeriod",
                        "Timestep",
                    ],
                )
        }
        ("OutputControl:Table:Style", "column_separator") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "All",
                        "Comma",
                        "CommaAndHTML",
                        "CommaAndXML",
                        "Fixed",
                        "HTML",
                        "Tab",
                        "TabAndHTML",
                        "XML",
                        "XMLandHTML",
                    ],
                )
        }
        ("OutputControl:Table:Style", "unit_conversion") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "InchPound",
                        "InchPoundExceptElectricity",
                        "JtoGJ",
                        "JtoKWH",
                        "JtoMJ",
                        "None",
                    ],
                )
        }
        ("OutputControl:Table:Style", "format_numeric_values") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("Output:Table:SummaryReports", "reports") => {
            matches!(value, RawValue::Array(rows) if rows.iter().all(|entry| matches!(entry, RawValue::Object(row) if row.keys().all(|key| ["report_name"].contains(&key.0.as_str())) && row.get(&FieldName("report_name".into())).is_some_and(|value| text_is(value, &["AllSummary"])))))
        }
        ("Exterior:Lights", "schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("Exterior:Lights", "design_level") => {
            blank(value) || number(value).is_some_and(|n| n >= 0.0)
        }
        ("Exterior:Lights", "control_option") => {
            blank(value) || text_is(value, &["AstronomicalClock", "ScheduleNameOnly"])
        }
        ("Exterior:Lights", "end_use_subcategory") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("OtherEquipment", "fuel_type") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "Coal",
                        "Diesel",
                        "DistrictCooling",
                        "DistrictHeatingSteam",
                        "DistrictHeatingWater",
                        "Electricity",
                        "FuelOilNo1",
                        "FuelOilNo2",
                        "Gasoline",
                        "NaturalGas",
                        "None",
                        "OtherFuel1",
                        "OtherFuel2",
                        "Propane",
                    ],
                )
        }
        ("OtherEquipment", "zone_or_zonelist_or_space_or_spacelist_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("OtherEquipment", "schedule_name") => blank(value) || matches!(value, RawValue::String(_)),
        ("OtherEquipment", "design_level_calculation_method") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "EquipmentLevel",
                        "Power/Area",
                        "Power/Person",
                        "Watts/Area",
                        "Watts/Person",
                    ],
                )
        }
        ("OtherEquipment", "design_level") => blank(value) || number(value).is_some(),
        ("OtherEquipment", "power_per_floor_area") => blank(value) || number(value).is_some(),
        ("OtherEquipment", "power_per_person") => blank(value) || number(value).is_some(),
        ("OtherEquipment", "fraction_latent") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
        }
        ("OtherEquipment", "fraction_radiant") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
        }
        ("OtherEquipment", "fraction_lost") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
        }
        ("OtherEquipment", "carbon_dioxide_generation_rate") => {
            blank(value) || number(value).is_some_and(|n| (0.0..=4e-07).contains(&n))
        }
        ("OtherEquipment", "end_use_subcategory") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("Schedule:Constant", "schedule_type_limits_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("Schedule:Constant", "hourly_value") => blank(value) || number(value).is_some(),
        ("ZoneAirHeatBalanceAlgorithm", "algorithm") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "AnalyticalSolution",
                        "EulerMethod",
                        "ThirdOrderBackwardDifference",
                    ],
                )
        }
        ("ZoneAirHeatBalanceAlgorithm", "do_space_heat_balance_for_sizing") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("ZoneAirHeatBalanceAlgorithm", "do_space_heat_balance_for_simulation") => {
            blank(value) || text_is(value, &["", "No", "Yes"])
        }
        ("Output:Variable", "key_value") => blank(value) || matches!(value, RawValue::String(_)),
        ("Output:Variable", "variable_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        ("Output:Variable", "reporting_frequency") => {
            blank(value)
                || text_is(
                    value,
                    &[
                        "",
                        "Annual",
                        "Daily",
                        "Detailed",
                        "Environment",
                        "Hourly",
                        "Monthly",
                        "RunPeriod",
                        "Timestep",
                    ],
                )
        }
        ("Output:Variable", "schedule_name") => {
            blank(value) || matches!(value, RawValue::String(_))
        }
        _ => false,
    }
}

pub(super) fn required_fields(family: &str) -> &'static [&'static str] {
    match family {
        "RunPeriod" => &[
            "begin_month",
            "begin_day_of_month",
            "end_month",
            "end_day_of_month",
        ],
        "SizingPeriod:DesignDay" => &[
            "month",
            "day_of_month",
            "day_type",
            "wind_speed",
            "wind_direction",
        ],
        "Material:NoMass" => &["roughness", "thermal_resistance"],
        "Material" => &[
            "roughness",
            "thickness",
            "conductivity",
            "density",
            "specific_heat",
        ],
        "Construction" => &["outside_layer"],
        "GlobalGeometryRules" => &[
            "starting_vertex_position",
            "vertex_entry_direction",
            "coordinate_system",
        ],
        "BuildingSurface:Detailed" => &[
            "surface_type",
            "construction_name",
            "zone_name",
            "outside_boundary_condition",
        ],
        "Output:Surfaces:Drawing" => &["report_type"],
        "Output:Meter:MeterFileOnly" => &["key_name"],
        "Exterior:Lights" => &["schedule_name", "design_level"],
        "OtherEquipment" => &[
            "zone_or_zonelist_or_space_or_spacelist_name",
            "schedule_name",
        ],
        "Output:Variable" => &["variable_name"],
        _ => &[],
    }
}
