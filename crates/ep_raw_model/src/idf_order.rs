use std::collections::{BTreeMap, BTreeSet};
use std::fmt::{Display, Formatter};

use crate::{FieldName, RawModel, RawValue, SourceInputFormat};

/// One IDF object type whose declaration order must survive epJSON conversion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IdfOrderTarget {
    /// Canonical EnergyPlus object type.
    pub object_type: &'static str,
    /// Zero-based data-field index containing the required object name.
    /// Source-defined unnamed report/project families use IdfParser-generated names.
    pub name_field_index: usize,
}

/// Raw key, alpha/numeric kind and exact original IDD field label.
pub type SourceIdfField = (&'static str, bool, &'static str);
/// Schema minimum, fixed fields and optional repeated extension fields.
pub type SourceIdfObjectLayout = (usize, &'static [SourceIdfField], &'static [SourceIdfField]);

/// One Source original-IDF field-metadata owner; values/defaults remain in existing getters.
pub const SOURCE_IDF_OBJECT_FAMILIES: &[&str] = &[
    "Curve:Bicubic",
    "Curve:Biquadratic",
    "Curve:CubicLinear",
    "Curve:FanPressureRise",
    "Curve:QuadraticLinear",
    "Material:RoofVegetation",
    "MaterialProperty:GlazingSpectralData",
    "WindowGap:DeflectionState",
    "WindowGap:SupportPillar",
    "WindowMaterial:Blind",
    "WindowMaterial:Blind:EquivalentLayer",
    "WindowMaterial:Drape:EquivalentLayer",
    "WindowMaterial:Gap",
    "WindowMaterial:Gap:EquivalentLayer",
    "WindowMaterial:Gas",
    "WindowMaterial:GasMixture",
    "WindowMaterial:Glazing",
    "WindowMaterial:Glazing:EquivalentLayer",
    "WindowMaterial:Glazing:RefractionExtinctionMethod",
    "WindowMaterial:Screen",
    "WindowMaterial:Screen:EquivalentLayer",
    "WindowMaterial:Shade",
    "WindowMaterial:Shade:EquivalentLayer",
    "WindowMaterial:SimpleGlazingSystem",
    "OutdoorAir:Node",
    "OutdoorAir:NodeList",
    "DesignSpecification:OutdoorAir",
    "Material:AirGap",
    "Material:NoMass",
];

/// Borrow original-IDF metadata, never an object-admission allowlist.
#[must_use]
pub fn source_idf_object_layout(family: &str) -> Option<SourceIdfObjectLayout> {
    match family {
        "Material:NoMass" => Some((
            3,
            &[
                ("name", true, "Name"),
                ("roughness", true, "Roughness"),
                ("thermal_resistance", false, "Thermal Resistance"),
                ("thermal_absorptance", false, "Thermal Absorptance"),
                ("solar_absorptance", false, "Solar Absorptance"),
                ("visible_absorptance", false, "Visible Absorptance"),
            ],
            &[],
        )),
        "Material:AirGap" => Some((
            2,
            &[
                ("name", true, "Name"),
                ("thermal_resistance", false, "Thermal Resistance"),
            ],
            &[],
        )),
        "DesignSpecification:OutdoorAir" => Some((
            1,
            &[
                ("name", true, "Name"),
                ("outdoor_air_method", true, "Outdoor Air Method"),
                (
                    "outdoor_air_flow_per_person",
                    false,
                    "Outdoor Air Flow per Person",
                ),
                (
                    "outdoor_air_flow_per_zone_floor_area",
                    false,
                    "Outdoor Air Flow per Zone Floor Area",
                ),
                (
                    "outdoor_air_flow_per_zone",
                    false,
                    "Outdoor Air Flow per Zone",
                ),
                (
                    "outdoor_air_flow_air_changes_per_hour",
                    false,
                    "Outdoor Air Flow Air Changes per Hour",
                ),
                (
                    "outdoor_air_schedule_name",
                    true,
                    "Outdoor Air Schedule Name",
                ),
                (
                    "proportional_control_minimum_outdoor_air_flow_rate_schedule_name",
                    true,
                    "Proportional Control Minimum Outdoor Air Flow Rate Schedule Name",
                ),
            ],
            &[],
        )),
        "OutdoorAir:Node" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("height_above_ground", false, "Height Above Ground"),
                (
                    "drybulb_temperature_schedule_name",
                    true,
                    "Drybulb Temperature Schedule Name",
                ),
                (
                    "wetbulb_temperature_schedule_name",
                    true,
                    "Wetbulb Temperature Schedule Name",
                ),
                ("wind_speed_schedule_name", true, "Wind Speed Schedule Name"),
                (
                    "wind_direction_schedule_name",
                    true,
                    "Wind Direction Schedule Name",
                ),
                (
                    "wind_pressure_coefficient_curve_name",
                    true,
                    "Wind Pressure Coefficient Curve Name",
                ),
                (
                    "symmetric_wind_pressure_coefficient_curve",
                    true,
                    "Symmetric Wind Pressure Coefficient Curve",
                ),
                ("wind_angle_type", true, "Wind Angle Type"),
            ],
            &[],
        )),
        "OutdoorAir:NodeList" => Some((
            0,
            &[],
            &[("node_or_nodelist_name", true, "Node or NodeList Name")],
        )),
        "Curve:Bicubic" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("coefficient1_constant", false, "Coefficient1 Constant"),
                ("coefficient2_x", false, "Coefficient2 x"),
                ("coefficient3_x_2", false, "Coefficient3 x**2"),
                ("coefficient4_y", false, "Coefficient4 y"),
                ("coefficient5_y_2", false, "Coefficient5 y**2"),
                ("coefficient6_x_y", false, "Coefficient6 x*y"),
                ("coefficient7_x_3", false, "Coefficient7 x**3"),
                ("coefficient8_y_3", false, "Coefficient8 y**3"),
                ("coefficient9_x_2_y", false, "Coefficient9 x**2*y"),
                ("coefficient10_x_y_2", false, "Coefficient10 x*y**2"),
                ("minimum_value_of_x", false, "Minimum Value of x"),
                ("maximum_value_of_x", false, "Maximum Value of x"),
                ("minimum_value_of_y", false, "Minimum Value of y"),
                ("maximum_value_of_y", false, "Maximum Value of y"),
                ("minimum_curve_output", false, "Minimum Curve Output"),
                ("maximum_curve_output", false, "Maximum Curve Output"),
                ("input_unit_type_for_x", true, "Input Unit Type for X"),
                ("input_unit_type_for_y", true, "Input Unit Type for Y"),
                ("output_unit_type", true, "Output Unit Type"),
            ],
            &[],
        )),
        "Curve:Biquadratic" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("coefficient1_constant", false, "Coefficient1 Constant"),
                ("coefficient2_x", false, "Coefficient2 x"),
                ("coefficient3_x_2", false, "Coefficient3 x**2"),
                ("coefficient4_y", false, "Coefficient4 y"),
                ("coefficient5_y_2", false, "Coefficient5 y**2"),
                ("coefficient6_x_y", false, "Coefficient6 x*y"),
                ("minimum_value_of_x", false, "Minimum Value of x"),
                ("maximum_value_of_x", false, "Maximum Value of x"),
                ("minimum_value_of_y", false, "Minimum Value of y"),
                ("maximum_value_of_y", false, "Maximum Value of y"),
                ("minimum_curve_output", false, "Minimum Curve Output"),
                ("maximum_curve_output", false, "Maximum Curve Output"),
                ("input_unit_type_for_x", true, "Input Unit Type for X"),
                ("input_unit_type_for_y", true, "Input Unit Type for Y"),
                ("output_unit_type", true, "Output Unit Type"),
            ],
            &[],
        )),
        "Curve:CubicLinear" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("coefficient1_constant", false, "Coefficient1 Constant"),
                ("coefficient2_x", false, "Coefficient2 x"),
                ("coefficient3_x_2", false, "Coefficient3 x**2"),
                ("coefficient4_x_3", false, "Coefficient4 x**3"),
                ("coefficient5_y", false, "Coefficient5 y"),
                ("coefficient6_x_y", false, "Coefficient6 x*y"),
                ("minimum_value_of_x", false, "Minimum Value of x"),
                ("maximum_value_of_x", false, "Maximum Value of x"),
                ("minimum_value_of_y", false, "Minimum Value of y"),
                ("maximum_value_of_y", false, "Maximum Value of y"),
                ("minimum_curve_output", false, "Minimum Curve Output"),
                ("maximum_curve_output", false, "Maximum Curve Output"),
                ("input_unit_type_for_x", true, "Input Unit Type for X"),
                ("input_unit_type_for_y", true, "Input Unit Type for Y"),
                ("output_unit_type", true, "Output Unit Type"),
            ],
            &[],
        )),
        "Curve:FanPressureRise" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("coefficient1_c1", false, "Coefficient1 C1"),
                ("coefficient2_c2", false, "Coefficient2 C2"),
                ("coefficient3_c3", false, "Coefficient3 C3"),
                ("coefficient4_c4", false, "Coefficient4 C4"),
                ("minimum_value_of_qfan", false, "Minimum Value of Qfan"),
                ("maximum_value_of_qfan", false, "Maximum Value of Qfan"),
                ("minimum_value_of_psm", false, "Minimum Value of Psm"),
                ("maximum_value_of_psm", false, "Maximum Value of Psm"),
                ("minimum_curve_output", false, "Minimum Curve Output"),
                ("maximum_curve_output", false, "Maximum Curve Output"),
            ],
            &[],
        )),
        "Curve:QuadraticLinear" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("coefficient1_constant", false, "Coefficient1 Constant"),
                ("coefficient2_x", false, "Coefficient2 x"),
                ("coefficient3_x_2", false, "Coefficient3 x**2"),
                ("coefficient4_y", false, "Coefficient4 y"),
                ("coefficient5_x_y", false, "Coefficient5 x*y"),
                ("coefficient6_x_2_y", false, "Coefficient6 x**2*y"),
                ("minimum_value_of_x", false, "Minimum Value of x"),
                ("maximum_value_of_x", false, "Maximum Value of x"),
                ("minimum_value_of_y", false, "Minimum Value of y"),
                ("maximum_value_of_y", false, "Maximum Value of y"),
                ("minimum_curve_output", false, "Minimum Curve Output"),
                ("maximum_curve_output", false, "Maximum Curve Output"),
                ("input_unit_type_for_x", true, "Input Unit Type for X"),
                ("input_unit_type_for_y", true, "Input Unit Type for Y"),
                ("output_unit_type", true, "Output Unit Type"),
            ],
            &[],
        )),
        "Material:RoofVegetation" => Some((
            19,
            &[
                ("name", true, "Name"),
                ("height_of_plants", false, "Height of Plants"),
                ("leaf_area_index", false, "Leaf Area Index"),
                ("leaf_reflectivity", false, "Leaf Reflectivity"),
                ("leaf_emissivity", false, "Leaf Emissivity"),
                (
                    "minimum_stomatal_resistance",
                    false,
                    "Minimum Stomatal Resistance",
                ),
                ("soil_layer_name", true, "Soil Layer Name"),
                ("roughness", true, "Roughness"),
                ("thickness", false, "Thickness"),
                (
                    "conductivity_of_dry_soil",
                    false,
                    "Conductivity of Dry Soil",
                ),
                ("density_of_dry_soil", false, "Density of Dry Soil"),
                (
                    "specific_heat_of_dry_soil",
                    false,
                    "Specific Heat of Dry Soil",
                ),
                ("thermal_absorptance", false, "Thermal Absorptance"),
                ("solar_absorptance", false, "Solar Absorptance"),
                ("visible_absorptance", false, "Visible Absorptance"),
                (
                    "saturation_volumetric_moisture_content_of_the_soil_layer",
                    false,
                    "Saturation Volumetric Moisture Content of the Soil Layer",
                ),
                (
                    "residual_volumetric_moisture_content_of_the_soil_layer",
                    false,
                    "Residual Volumetric Moisture Content of the Soil Layer",
                ),
                (
                    "initial_volumetric_moisture_content_of_the_soil_layer",
                    false,
                    "Initial Volumetric Moisture Content of the Soil Layer",
                ),
                (
                    "moisture_diffusion_calculation_method",
                    true,
                    "Moisture Diffusion Calculation Method",
                ),
            ],
            &[],
        )),
        "MaterialProperty:GlazingSpectralData" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("wavelength_1", false, "Wavelength 1"),
                ("transmittance_1", false, "Transmittance 1"),
                ("front_reflectance_1", false, "Front Reflectance 1"),
                ("back_reflectance_1", false, "Back Reflectance 1"),
                ("wavelength_2", false, "Wavelength 2"),
                ("transmittance_2", false, "Transmittance 2"),
                ("front_reflectance_2", false, "Front Reflectance 2"),
                ("back_reflectance_2", false, "Back Reflectance 2"),
                ("wavelength_3", false, "Wavelength 3"),
                ("transmittance_3", false, "Transmittance 3"),
                ("front_reflectance_3", false, "Front Reflectance 3"),
                ("back_reflectance_3", false, "Back Reflectance 3"),
                ("wavelength_4", false, "Wavelength 4"),
                ("transmittance_4", false, "Transmittance 4"),
                ("front_reflectance_4", false, "Front Reflectance 4"),
                ("back_reflectance_4", false, "Back Reflectance 4"),
            ],
            &[
                ("wavelength", false, "Wavelength"),
                ("transmittance", false, "Transmittance"),
                ("front_reflectance", false, "Front Reflectance"),
                ("back_reflectance", false, "Back Reflectance"),
            ],
        )),
        "WindowGap:DeflectionState" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("deflected_thickness", false, "Deflected Thickness"),
                ("initial_temperature", false, "Initial Temperature"),
                ("initial_pressure", false, "Initial Pressure"),
            ],
            &[],
        )),
        "WindowGap:SupportPillar" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("spacing", false, "Spacing"),
                ("radius", false, "Radius"),
            ],
            &[],
        )),
        "WindowMaterial:Blind" => Some((
            29,
            &[
                ("name", true, "Name"),
                ("slat_orientation", true, "Slat Orientation"),
                ("slat_width", false, "Slat Width"),
                ("slat_separation", false, "Slat Separation"),
                ("slat_thickness", false, "Slat Thickness"),
                ("slat_angle", false, "Slat Angle"),
                ("slat_conductivity", false, "Slat Conductivity"),
                (
                    "slat_beam_solar_transmittance",
                    false,
                    "Slat Beam Solar Transmittance",
                ),
                (
                    "front_side_slat_beam_solar_reflectance",
                    false,
                    "Front Side Slat Beam Solar Reflectance",
                ),
                (
                    "back_side_slat_beam_solar_reflectance",
                    false,
                    "Back Side Slat Beam Solar Reflectance",
                ),
                (
                    "slat_diffuse_solar_transmittance",
                    false,
                    "Slat Diffuse Solar Transmittance",
                ),
                (
                    "front_side_slat_diffuse_solar_reflectance",
                    false,
                    "Front Side Slat Diffuse Solar Reflectance",
                ),
                (
                    "back_side_slat_diffuse_solar_reflectance",
                    false,
                    "Back Side Slat Diffuse Solar Reflectance",
                ),
                (
                    "slat_beam_visible_transmittance",
                    false,
                    "Slat Beam Visible Transmittance",
                ),
                (
                    "front_side_slat_beam_visible_reflectance",
                    false,
                    "Front Side Slat Beam Visible Reflectance",
                ),
                (
                    "back_side_slat_beam_visible_reflectance",
                    false,
                    "Back Side Slat Beam Visible Reflectance",
                ),
                (
                    "slat_diffuse_visible_transmittance",
                    false,
                    "Slat Diffuse Visible Transmittance",
                ),
                (
                    "front_side_slat_diffuse_visible_reflectance",
                    false,
                    "Front Side Slat Diffuse Visible Reflectance",
                ),
                (
                    "back_side_slat_diffuse_visible_reflectance",
                    false,
                    "Back Side Slat Diffuse Visible Reflectance",
                ),
                (
                    "slat_infrared_hemispherical_transmittance",
                    false,
                    "Slat Infrared Hemispherical Transmittance",
                ),
                (
                    "front_side_slat_infrared_hemispherical_emissivity",
                    false,
                    "Front Side Slat Infrared Hemispherical Emissivity",
                ),
                (
                    "back_side_slat_infrared_hemispherical_emissivity",
                    false,
                    "Back Side Slat Infrared Hemispherical Emissivity",
                ),
                ("blind_to_glass_distance", false, "Blind to Glass Distance"),
                (
                    "blind_top_opening_multiplier",
                    false,
                    "Blind Top Opening Multiplier",
                ),
                (
                    "blind_bottom_opening_multiplier",
                    false,
                    "Blind Bottom Opening Multiplier",
                ),
                (
                    "blind_left_side_opening_multiplier",
                    false,
                    "Blind Left Side Opening Multiplier",
                ),
                (
                    "blind_right_side_opening_multiplier",
                    false,
                    "Blind Right Side Opening Multiplier",
                ),
                ("minimum_slat_angle", false, "Minimum Slat Angle"),
                ("maximum_slat_angle", false, "Maximum Slat Angle"),
            ],
            &[],
        )),
        "WindowMaterial:Blind:EquivalentLayer" => Some((
            10,
            &[
                ("name", true, "Name"),
                ("slat_orientation", true, "Slat Orientation"),
                ("slat_width", false, "Slat Width"),
                ("slat_separation", false, "Slat Separation"),
                ("slat_crown", false, "Slat Crown"),
                ("slat_angle", false, "Slat Angle"),
                (
                    "front_side_slat_beam_diffuse_solar_transmittance",
                    false,
                    "Front Side Slat Beam-Diffuse Solar Transmittance",
                ),
                (
                    "back_side_slat_beam_diffuse_solar_transmittance",
                    false,
                    "Back Side Slat Beam-Diffuse Solar Transmittance",
                ),
                (
                    "front_side_slat_beam_diffuse_solar_reflectance",
                    false,
                    "Front Side Slat Beam-Diffuse Solar Reflectance",
                ),
                (
                    "back_side_slat_beam_diffuse_solar_reflectance",
                    false,
                    "Back Side Slat Beam-Diffuse Solar Reflectance",
                ),
                (
                    "front_side_slat_beam_diffuse_visible_transmittance",
                    false,
                    "Front Side Slat Beam-Diffuse Visible Transmittance",
                ),
                (
                    "back_side_slat_beam_diffuse_visible_transmittance",
                    false,
                    "Back Side Slat Beam-Diffuse Visible Transmittance",
                ),
                (
                    "front_side_slat_beam_diffuse_visible_reflectance",
                    false,
                    "Front Side Slat Beam-Diffuse Visible Reflectance",
                ),
                (
                    "back_side_slat_beam_diffuse_visible_reflectance",
                    false,
                    "Back Side Slat Beam-Diffuse Visible Reflectance",
                ),
                (
                    "slat_diffuse_diffuse_solar_transmittance",
                    false,
                    "Slat Diffuse-Diffuse Solar Transmittance",
                ),
                (
                    "front_side_slat_diffuse_diffuse_solar_reflectance",
                    false,
                    "Front Side Slat Diffuse-Diffuse Solar Reflectance",
                ),
                (
                    "back_side_slat_diffuse_diffuse_solar_reflectance",
                    false,
                    "Back Side Slat Diffuse-Diffuse Solar Reflectance",
                ),
                (
                    "slat_diffuse_diffuse_visible_transmittance",
                    false,
                    "Slat Diffuse-Diffuse Visible Transmittance",
                ),
                (
                    "front_side_slat_diffuse_diffuse_visible_reflectance",
                    false,
                    "Front Side Slat Diffuse-Diffuse Visible Reflectance",
                ),
                (
                    "back_side_slat_diffuse_diffuse_visible_reflectance",
                    false,
                    "Back Side Slat Diffuse-Diffuse Visible Reflectance",
                ),
                (
                    "slat_infrared_transmittance",
                    false,
                    "Slat Infrared Transmittance",
                ),
                (
                    "front_side_slat_infrared_emissivity",
                    false,
                    "Front Side Slat Infrared Emissivity",
                ),
                (
                    "back_side_slat_infrared_emissivity",
                    false,
                    "Back Side Slat Infrared Emissivity",
                ),
                ("slat_angle_control", true, "Slat Angle Control"),
            ],
            &[],
        )),
        "WindowMaterial:Drape:EquivalentLayer" => Some((
            4,
            &[
                ("name", true, "Name"),
                (
                    "drape_beam_beam_solar_transmittance_at_normal_incidence",
                    false,
                    "Drape Beam-Beam Solar Transmittance at Normal Incidence",
                ),
                (
                    "front_side_drape_beam_diffuse_solar_transmittance",
                    false,
                    "Front Side Drape Beam-Diffuse Solar Transmittance",
                ),
                (
                    "back_side_drape_beam_diffuse_solar_transmittance",
                    false,
                    "Back Side Drape Beam-Diffuse Solar Transmittance",
                ),
                (
                    "front_side_drape_beam_diffuse_solar_reflectance",
                    false,
                    "Front Side Drape Beam-Diffuse Solar Reflectance",
                ),
                (
                    "back_side_drape_beam_diffuse_solar_reflectance",
                    false,
                    "Back Side Drape Beam-Diffuse Solar Reflectance",
                ),
                (
                    "drape_beam_beam_visible_transmittance",
                    false,
                    "Drape Beam-Beam Visible Transmittance",
                ),
                (
                    "drape_beam_diffuse_visible_transmittance",
                    false,
                    "Drape Beam-Diffuse Visible Transmittance",
                ),
                (
                    "drape_beam_diffuse_visible_reflectance",
                    false,
                    "Drape Beam-Diffuse Visible Reflectance",
                ),
                (
                    "drape_material_infrared_transmittance",
                    false,
                    "Drape Material Infrared Transmittance",
                ),
                (
                    "front_side_drape_material_infrared_emissivity",
                    false,
                    "Front Side Drape Material Infrared Emissivity",
                ),
                (
                    "back_side_drape_material_infrared_emissivity",
                    false,
                    "Back Side Drape Material Infrared Emissivity",
                ),
                ("width_of_pleated_fabric", false, "Width of Pleated Fabric"),
                (
                    "length_of_pleated_fabric",
                    false,
                    "Length of Pleated Fabric",
                ),
            ],
            &[],
        )),
        "WindowMaterial:Gap" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("thickness", false, "Thickness"),
                ("gas_or_gas_mixture_", true, "Gas (or Gas Mixture)"),
                ("pressure", false, "Pressure"),
                ("deflection_state", true, "Deflection State"),
                ("support_pillar", true, "Support Pillar"),
            ],
            &[],
        )),
        "WindowMaterial:Gap:EquivalentLayer" => Some((
            3,
            &[
                ("name", true, "Name"),
                ("gas_type", true, "Gas Type"),
                ("thickness", false, "Thickness"),
                ("gap_vent_type", true, "Gap Vent Type"),
                (
                    "conductivity_coefficient_a",
                    false,
                    "Conductivity Coefficient A",
                ),
                (
                    "conductivity_coefficient_b",
                    false,
                    "Conductivity Coefficient B",
                ),
                (
                    "conductivity_coefficient_c",
                    false,
                    "Conductivity Coefficient C",
                ),
                ("viscosity_coefficient_a", false, "Viscosity Coefficient A"),
                ("viscosity_coefficient_b", false, "Viscosity Coefficient B"),
                ("viscosity_coefficient_c", false, "Viscosity Coefficient C"),
                (
                    "specific_heat_coefficient_a",
                    false,
                    "Specific Heat Coefficient A",
                ),
                (
                    "specific_heat_coefficient_b",
                    false,
                    "Specific Heat Coefficient B",
                ),
                (
                    "specific_heat_coefficient_c",
                    false,
                    "Specific Heat Coefficient C",
                ),
                ("molecular_weight", false, "Molecular Weight"),
                ("specific_heat_ratio", false, "Specific Heat Ratio"),
            ],
            &[],
        )),
        "WindowMaterial:Gas" => Some((
            3,
            &[
                ("name", true, "Name"),
                ("gas_type", true, "Gas Type"),
                ("thickness", false, "Thickness"),
                (
                    "conductivity_coefficient_a",
                    false,
                    "Conductivity Coefficient A",
                ),
                (
                    "conductivity_coefficient_b",
                    false,
                    "Conductivity Coefficient B",
                ),
                (
                    "conductivity_coefficient_c",
                    false,
                    "Conductivity Coefficient C",
                ),
                ("viscosity_coefficient_a", false, "Viscosity Coefficient A"),
                ("viscosity_coefficient_b", false, "Viscosity Coefficient B"),
                ("viscosity_coefficient_c", false, "Viscosity Coefficient C"),
                (
                    "specific_heat_coefficient_a",
                    false,
                    "Specific Heat Coefficient A",
                ),
                (
                    "specific_heat_coefficient_b",
                    false,
                    "Specific Heat Coefficient B",
                ),
                (
                    "specific_heat_coefficient_c",
                    false,
                    "Specific Heat Coefficient C",
                ),
                ("molecular_weight", false, "Molecular Weight"),
                ("specific_heat_ratio", false, "Specific Heat Ratio"),
            ],
            &[],
        )),
        "WindowMaterial:GasMixture" => Some((
            7,
            &[
                ("name", true, "Name"),
                ("thickness", false, "Thickness"),
                (
                    "number_of_gases_in_mixture",
                    false,
                    "Number of Gases in Mixture",
                ),
                ("gas_1_type", true, "Gas 1 Type"),
                ("gas_1_fraction", false, "Gas 1 Fraction"),
                ("gas_2_type", true, "Gas 2 Type"),
                ("gas_2_fraction", false, "Gas 2 Fraction"),
                ("gas_3_type", true, "Gas 3 Type"),
                ("gas_3_fraction", false, "Gas 3 Fraction"),
                ("gas_4_type", true, "Gas 4 Type"),
                ("gas_4_fraction", false, "Gas 4 Fraction"),
            ],
            &[],
        )),
        "WindowMaterial:Glazing" => Some((
            14,
            &[
                ("name", true, "Name"),
                ("optical_data_type", true, "Optical Data Type"),
                (
                    "window_glass_spectral_data_set_name",
                    true,
                    "Window Glass Spectral Data Set Name",
                ),
                ("thickness", false, "Thickness"),
                (
                    "solar_transmittance_at_normal_incidence",
                    false,
                    "Solar Transmittance at Normal Incidence",
                ),
                (
                    "front_side_solar_reflectance_at_normal_incidence",
                    false,
                    "Front Side Solar Reflectance at Normal Incidence",
                ),
                (
                    "back_side_solar_reflectance_at_normal_incidence",
                    false,
                    "Back Side Solar Reflectance at Normal Incidence",
                ),
                (
                    "visible_transmittance_at_normal_incidence",
                    false,
                    "Visible Transmittance at Normal Incidence",
                ),
                (
                    "front_side_visible_reflectance_at_normal_incidence",
                    false,
                    "Front Side Visible Reflectance at Normal Incidence",
                ),
                (
                    "back_side_visible_reflectance_at_normal_incidence",
                    false,
                    "Back Side Visible Reflectance at Normal Incidence",
                ),
                (
                    "infrared_transmittance_at_normal_incidence",
                    false,
                    "Infrared Transmittance at Normal Incidence",
                ),
                (
                    "front_side_infrared_hemispherical_emissivity",
                    false,
                    "Front Side Infrared Hemispherical Emissivity",
                ),
                (
                    "back_side_infrared_hemispherical_emissivity",
                    false,
                    "Back Side Infrared Hemispherical Emissivity",
                ),
                ("conductivity", false, "Conductivity"),
                (
                    "dirt_correction_factor_for_solar_and_visible_transmittance",
                    false,
                    "Dirt Correction Factor for Solar and Visible Transmittance",
                ),
                ("solar_diffusing", true, "Solar Diffusing"),
                ("young_s_modulus", false, "Young's modulus"),
                ("poisson_s_ratio", false, "Poisson's ratio"),
                (
                    "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name",
                    true,
                    "Window Glass Spectral and Incident Angle Transmittance Data Set Table Name",
                ),
                (
                    "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name",
                    true,
                    "Window Glass Spectral and Incident Angle Front Reflectance Data Set Table Name",
                ),
                (
                    "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name",
                    true,
                    "Window Glass Spectral and Incident Angle Back Reflectance Data Set Table Name",
                ),
            ],
            &[],
        )),
        "WindowMaterial:Glazing:EquivalentLayer" => Some((
            11,
            &[
                ("name", true, "Name"),
                ("optical_data_type", true, "Optical Data Type"),
                (
                    "window_glass_spectral_data_set_name",
                    true,
                    "Window Glass Spectral Data Set Name",
                ),
                (
                    "front_side_beam_beam_solar_transmittance",
                    false,
                    "Front Side Beam-Beam Solar Transmittance",
                ),
                (
                    "back_side_beam_beam_solar_transmittance",
                    false,
                    "Back Side Beam-Beam Solar Transmittance",
                ),
                (
                    "front_side_beam_beam_solar_reflectance",
                    false,
                    "Front Side Beam-Beam Solar Reflectance",
                ),
                (
                    "back_side_beam_beam_solar_reflectance",
                    false,
                    "Back Side Beam-Beam Solar Reflectance",
                ),
                (
                    "front_side_beam_beam_visible_solar_transmittance",
                    false,
                    "Front Side Beam-Beam Visible Solar Transmittance",
                ),
                (
                    "back_side_beam_beam_visible_solar_transmittance",
                    false,
                    "Back Side Beam-Beam Visible Solar Transmittance",
                ),
                (
                    "front_side_beam_beam_visible_solar_reflectance",
                    false,
                    "Front Side Beam-Beam Visible Solar Reflectance",
                ),
                (
                    "back_side_beam_beam_visible_solar_reflectance",
                    false,
                    "Back Side Beam-Beam Visible Solar Reflectance",
                ),
                (
                    "front_side_beam_diffuse_solar_transmittance",
                    false,
                    "Front Side Beam-Diffuse Solar Transmittance",
                ),
                (
                    "back_side_beam_diffuse_solar_transmittance",
                    false,
                    "Back Side Beam-Diffuse Solar Transmittance",
                ),
                (
                    "front_side_beam_diffuse_solar_reflectance",
                    false,
                    "Front Side Beam-Diffuse Solar Reflectance",
                ),
                (
                    "back_side_beam_diffuse_solar_reflectance",
                    false,
                    "Back Side Beam-Diffuse Solar Reflectance",
                ),
                (
                    "front_side_beam_diffuse_visible_solar_transmittance",
                    false,
                    "Front Side Beam-Diffuse Visible Solar Transmittance",
                ),
                (
                    "back_side_beam_diffuse_visible_solar_transmittance",
                    false,
                    "Back Side Beam-Diffuse Visible Solar Transmittance",
                ),
                (
                    "front_side_beam_diffuse_visible_solar_reflectance",
                    false,
                    "Front Side Beam-Diffuse Visible Solar Reflectance",
                ),
                (
                    "back_side_beam_diffuse_visible_solar_reflectance",
                    false,
                    "Back Side Beam-Diffuse Visible Solar Reflectance",
                ),
                (
                    "diffuse_diffuse_solar_transmittance",
                    false,
                    "Diffuse-Diffuse Solar Transmittance",
                ),
                (
                    "front_side_diffuse_diffuse_solar_reflectance",
                    false,
                    "Front Side Diffuse-Diffuse Solar Reflectance",
                ),
                (
                    "back_side_diffuse_diffuse_solar_reflectance",
                    false,
                    "Back Side Diffuse-Diffuse Solar Reflectance",
                ),
                (
                    "diffuse_diffuse_visible_solar_transmittance",
                    false,
                    "Diffuse-Diffuse Visible Solar Transmittance",
                ),
                (
                    "front_side_diffuse_diffuse_visible_solar_reflectance",
                    false,
                    "Front Side Diffuse-Diffuse Visible Solar Reflectance",
                ),
                (
                    "back_side_diffuse_diffuse_visible_solar_reflectance",
                    false,
                    "Back Side Diffuse-Diffuse Visible Solar Reflectance",
                ),
                (
                    "infrared_transmittance_applies_to_front_and_back_",
                    false,
                    "Infrared Transmittance (applies to front and back)",
                ),
                (
                    "front_side_infrared_emissivity",
                    false,
                    "Front Side Infrared Emissivity",
                ),
                (
                    "back_side_infrared_emissivity",
                    false,
                    "Back Side Infrared Emissivity",
                ),
                ("thermal_resistance", false, "Thermal Resistance"),
            ],
            &[],
        )),
        "WindowMaterial:Glazing:RefractionExtinctionMethod" => Some((
            0,
            &[
                ("name", true, "Name"),
                ("thickness", false, "Thickness"),
                (
                    "solar_index_of_refraction",
                    false,
                    "Solar Index of Refraction",
                ),
                (
                    "solar_extinction_coefficient",
                    false,
                    "Solar Extinction Coefficient",
                ),
                (
                    "visible_index_of_refraction",
                    false,
                    "Visible Index of Refraction",
                ),
                (
                    "visible_extinction_coefficient",
                    false,
                    "Visible Extinction Coefficient",
                ),
                (
                    "infrared_transmittance_at_normal_incidence",
                    false,
                    "Infrared Transmittance at Normal Incidence",
                ),
                (
                    "infrared_hemispherical_emissivity",
                    false,
                    "Infrared Hemispherical Emissivity",
                ),
                ("conductivity", false, "Conductivity"),
                (
                    "dirt_correction_factor_for_solar_and_visible_transmittance",
                    false,
                    "Dirt Correction Factor for Solar and Visible Transmittance",
                ),
                ("solar_diffusing", true, "Solar Diffusing"),
            ],
            &[],
        )),
        "WindowMaterial:Screen" => Some((
            9,
            &[
                ("name", true, "Name"),
                (
                    "reflected_beam_transmittance_accounting_method",
                    true,
                    "Reflected Beam Transmittance Accounting Method",
                ),
                (
                    "diffuse_solar_reflectance",
                    false,
                    "Diffuse Solar Reflectance",
                ),
                (
                    "diffuse_visible_reflectance",
                    false,
                    "Diffuse Visible Reflectance",
                ),
                (
                    "thermal_hemispherical_emissivity",
                    false,
                    "Thermal Hemispherical Emissivity",
                ),
                ("conductivity", false, "Conductivity"),
                ("screen_material_spacing", false, "Screen Material Spacing"),
                (
                    "screen_material_diameter",
                    false,
                    "Screen Material Diameter",
                ),
                (
                    "screen_to_glass_distance",
                    false,
                    "Screen to Glass Distance",
                ),
                ("top_opening_multiplier", false, "Top Opening Multiplier"),
                (
                    "bottom_opening_multiplier",
                    false,
                    "Bottom Opening Multiplier",
                ),
                (
                    "left_side_opening_multiplier",
                    false,
                    "Left Side Opening Multiplier",
                ),
                (
                    "right_side_opening_multiplier",
                    false,
                    "Right Side Opening Multiplier",
                ),
                (
                    "angle_of_resolution_for_screen_transmittance_output_map",
                    false,
                    "Angle of Resolution for Screen Transmittance Output Map",
                ),
            ],
            &[],
        )),
        "WindowMaterial:Screen:EquivalentLayer" => Some((
            11,
            &[
                ("name", true, "Name"),
                (
                    "screen_beam_beam_solar_transmittance",
                    false,
                    "Screen Beam-Beam Solar Transmittance",
                ),
                (
                    "screen_beam_diffuse_solar_transmittance",
                    false,
                    "Screen Beam-Diffuse Solar Transmittance",
                ),
                (
                    "screen_beam_diffuse_solar_reflectance",
                    false,
                    "Screen Beam-Diffuse Solar Reflectance",
                ),
                (
                    "screen_beam_beam_visible_transmittance",
                    false,
                    "Screen Beam-Beam Visible Transmittance",
                ),
                (
                    "screen_beam_diffuse_visible_transmittance",
                    false,
                    "Screen Beam-Diffuse Visible Transmittance",
                ),
                (
                    "screen_beam_diffuse_visible_reflectance",
                    false,
                    "Screen Beam-Diffuse Visible Reflectance",
                ),
                (
                    "screen_infrared_transmittance",
                    false,
                    "Screen Infrared Transmittance",
                ),
                (
                    "screen_infrared_emissivity",
                    false,
                    "Screen Infrared Emissivity",
                ),
                ("screen_wire_spacing", false, "Screen Wire Spacing"),
                ("screen_wire_diameter", false, "Screen Wire Diameter"),
            ],
            &[],
        )),
        "WindowMaterial:Shade" => Some((
            15,
            &[
                ("name", true, "Name"),
                ("solar_transmittance", false, "Solar Transmittance"),
                ("solar_reflectance", false, "Solar Reflectance"),
                ("visible_transmittance", false, "Visible Transmittance"),
                ("visible_reflectance", false, "Visible Reflectance"),
                (
                    "infrared_hemispherical_emissivity",
                    false,
                    "Infrared Hemispherical Emissivity",
                ),
                ("infrared_transmittance", false, "Infrared Transmittance"),
                ("thickness", false, "Thickness"),
                ("conductivity", false, "Conductivity"),
                ("shade_to_glass_distance", false, "Shade to Glass Distance"),
                ("top_opening_multiplier", false, "Top Opening Multiplier"),
                (
                    "bottom_opening_multiplier",
                    false,
                    "Bottom Opening Multiplier",
                ),
                (
                    "left_side_opening_multiplier",
                    false,
                    "Left-Side Opening Multiplier",
                ),
                (
                    "right_side_opening_multiplier",
                    false,
                    "Right-Side Opening Multiplier",
                ),
                ("airflow_permeability", false, "Airflow Permeability"),
            ],
            &[],
        )),
        "WindowMaterial:Shade:EquivalentLayer" => Some((
            6,
            &[
                ("name", true, "Name"),
                (
                    "shade_beam_beam_solar_transmittance",
                    false,
                    "Shade Beam-Beam Solar Transmittance",
                ),
                (
                    "front_side_shade_beam_diffuse_solar_transmittance",
                    false,
                    "Front Side Shade Beam-Diffuse Solar Transmittance",
                ),
                (
                    "back_side_shade_beam_diffuse_solar_transmittance",
                    false,
                    "Back Side Shade Beam-Diffuse Solar Transmittance",
                ),
                (
                    "front_side_shade_beam_diffuse_solar_reflectance",
                    false,
                    "Front Side Shade Beam-Diffuse Solar Reflectance",
                ),
                (
                    "back_side_shade_beam_diffuse_solar_reflectance",
                    false,
                    "Back Side Shade Beam-Diffuse Solar Reflectance",
                ),
                (
                    "shade_beam_beam_visible_transmittance_at_normal_incidence",
                    false,
                    "Shade Beam-Beam Visible Transmittance at Normal Incidence",
                ),
                (
                    "shade_beam_diffuse_visible_transmittance_at_normal_incidence",
                    false,
                    "Shade Beam-Diffuse Visible Transmittance at Normal Incidence",
                ),
                (
                    "shade_beam_diffuse_visible_reflectance_at_normal_incidence",
                    false,
                    "Shade Beam-Diffuse Visible Reflectance at Normal Incidence",
                ),
                (
                    "shade_material_infrared_transmittance",
                    false,
                    "Shade Material Infrared Transmittance",
                ),
                (
                    "front_side_shade_material_infrared_emissivity",
                    false,
                    "Front Side Shade Material Infrared Emissivity",
                ),
                (
                    "back_side_shade_material_infrared_emissivity",
                    false,
                    "Back Side Shade Material Infrared Emissivity",
                ),
            ],
            &[],
        )),
        "WindowMaterial:SimpleGlazingSystem" => Some((
            3,
            &[
                ("name", true, "Name"),
                ("u_factor", false, "U-Factor"),
                (
                    "solar_heat_gain_coefficient",
                    false,
                    "Solar Heat Gain Coefficient",
                ),
                ("visible_transmittance", false, "Visible Transmittance"),
            ],
            &[],
        )),
        _ => None,
    }
}

/// IDF declaration-order targets backed by current source and oracle evidence.
///
/// This default is deliberately limited to source-order-sensitive families with
/// explicit identity and converter edge-case evidence. Other position-sensitive
/// families require the same evidence before joining the fail-closed overlay.
pub const IDF_ORDER_TARGETS: &[IdfOrderTarget] = &[
    // Same four actual HB getter targets from the frozen adjacent packet.
    IdfOrderTarget {
        object_type: "Building",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "SurfaceConvectionAlgorithm:Inside",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "SurfaceConvectionAlgorithm:Outside",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "ZoneAirHeatBalanceAlgorithm",
        name_field_index: 0,
    },
    // Material.cc regular direct JSON, NoMass and AirGap getters preserve IDF order.
    IdfOrderTarget {
        object_type: "Material",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Material:NoMass",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Material:AirGap",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Material:InfraredTransparent",
        name_field_index: 0,
    },
    // SimulationManager::GetProjectData uses real IDF max-field counts.
    IdfOrderTarget {
        object_type: "Version",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "HeatBalanceAlgorithm",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Timestep",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "SimulationControl",
        name_field_index: 0,
    },
    // Actual TAB style/summary getters require real staged field counts.
    IdfOrderTarget {
        object_type: "OutputControl:Table:Style",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:Table:SummaryReports",
        name_field_index: 0,
    },
    // OutputProcessor's actual getter requires the same unnamed-object order.
    IdfOrderTarget {
        object_type: "Output:Variable",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:Meter",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:Meter:MeterFileOnly",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:Meter:Cumulative",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:Meter:Cumulative:MeterFileOnly",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:Schedules",
        name_field_index: 0,
    },
    // IdfParser.cc277-301 generates these unnamed keys from the per-type count.
    IdfOrderTarget {
        object_type: "Output:Surfaces:List",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:Surfaces:Drawing",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:VariableDictionary",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:Constructions",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Output:EnergyManagementSystem",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "RunPeriodControl:DaylightSavingTime",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "RunPeriodControl:SpecialDays",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction:FfactorGroundFloor",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction:CfactorUndergroundWall",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction:ComplexFenestrationState",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction:WindowEquivalentLayer",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Construction:WindowDataFile",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Zone",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "ZoneList",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "ZoneGroup",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "ZoneProperty:LocalEnvironment",
        name_field_index: 0,
    },
    // Actual named RunPeriod getter uses staged declaration order/field bounds.
    IdfOrderTarget {
        object_type: "RunPeriod",
        name_field_index: 0,
    },
    // WeatherManager::GetDesignDayData calls getObjectItem in input order;
    // ReverseDD remaps the first two owners only after this effective ordering.
    IdfOrderTarget {
        object_type: "SizingPeriod:DesignDay",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "GlobalGeometryRules",
        name_field_index: 0,
    },
    // SurfaceGeometry::GetSurfaceData preserves IDF declaration order within
    // its Wall/Floor/Roof groups. Native epJSON keeps its own map order.
    IdfOrderTarget {
        object_type: "BuildingSurface:Detailed",
        name_field_index: 0,
    },
    // Four actual unnamed Site factories use the same physical IDF metadata.
    IdfOrderTarget {
        object_type: "Site:GroundTemperature:BuildingSurface",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Site:GroundTemperature:FCfactorMethod",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Site:GroundTemperature:Shallow",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Site:GroundTemperature:Deep",
        name_field_index: 0,
    },
    // Actual GetLocationInfo uses the named Site:Location's IDF metadata.
    IdfOrderTarget {
        object_type: "Site:Location",
        name_field_index: 0,
    },
    // Actual Exterior getter reads named lights in staged IDF declaration order.
    IdfOrderTarget {
        object_type: "Exterior:Lights",
        name_field_index: 0,
    },
    // Native Comfort list names may be blank or distinct only by case.
    IdfOrderTarget {
        object_type: "ComfortViewFactorAngles",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "People",
        name_field_index: 0,
    },
    // InternalHeatGains::GetInternalHeatGainsInput reads named OtherEquipment
    // in the same Source getObjectItem order and with real IDF field counts.
    IdfOrderTarget {
        object_type: "OtherEquipment",
        name_field_index: 0,
    },
    // GetZoneAirSetPoints consumes the actual thermostat numeric-field
    // count. Preserve its staged name/order/max-fields on the same raw owner.
    IdfOrderTarget {
        object_type: "ZoneControl:Thermostat",
        name_field_index: 0,
    },
    // PurchasedAir's real getter consumes IDF order and physical max-fields metadata.
    IdfOrderTarget {
        object_type: "ZoneHVAC:IdealLoadsAirSystem",
        name_field_index: 0,
    },
    // SetPointManager reads its original global IDF ordinal from the raw DOM.
    IdfOrderTarget {
        object_type: "SetpointManager:Scheduled",
        name_field_index: 0,
    },
    // NodeInputManager's legacy getter visits every list in actual input order.
    IdfOrderTarget {
        object_type: "NodeList",
        name_field_index: 0,
    },
    // ScheduleManager uses getObjectItem's preserved IDF order within these
    // families. Cross-family registration order remains the compiler's owner.
    IdfOrderTarget {
        object_type: "ScheduleTypeLimits",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:Constant",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:File",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:Compact",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:Day:Hourly",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:Day:Interval",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:Day:List",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:Week:Compact",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:Week:Daily",
        name_field_index: 0,
    },
    IdfOrderTarget {
        object_type: "Schedule:Year",
        name_field_index: 0,
    },
];

/// Error returned when staged IDF declaration order cannot be recovered safely.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IdfOrderError {
    message: String,
}

impl IdfOrderError {
    pub(crate) fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl Display for IdfOrderError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for IdfOrderError {}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ScannedObject {
    idf_order: u32,
    max_fields: usize,
    semicolon_terminated: bool,
    source_max_extensible_fields: Option<usize>,
    comfort_max_extensible_fields: Option<usize>,
    name: String,
    line: usize,
    column: usize,
}

/// Applies declaration order recovered from the staged IDF to a converted RawModel.
///
/// The operation is strict and transactional: every configured target instance must
/// reconcile one-to-one with the converted epJSON map before any overlay is installed.
pub fn apply_idf_declaration_order(
    model: &mut RawModel,
    idf: &str,
    targets: &[IdfOrderTarget],
) -> Result<(), IdfOrderError> {
    apply_idf_declaration_order_inner(model, idf, targets, false)
}

fn apply_idf_declaration_order_inner(
    model: &mut RawModel,
    idf: &str,
    targets: &[IdfOrderTarget],
    source_object_layouts: bool,
) -> Result<(), IdfOrderError> {
    validate_targets(targets)?;
    let scanned = scan_target_objects(idf, targets, source_object_layouts)?;
    let mut overlay = BTreeMap::new();
    let mut report_field_counts = Vec::new();
    let mut report_idf_orders = Vec::new();

    for target in targets {
        let exact_names = target.object_type == "ComfortViewFactorAngles";
        // Native preserves exact converted keys before its alpha getter uppercases
        // names. Only Source-layout SpectralData joins the existing NodeList rule;
        // Comfort-only metadata gates and all other family policies stay separate.
        let converted_exact_names = target.object_type.eq_ignore_ascii_case("NodeList")
            || (source_object_layouts
                && target.object_type == "MaterialProperty:GlazingSpectralData");
        let idf_objects = scanned.get(target.object_type).ok_or_else(|| {
            IdfOrderError::new(format!(
                "internal IDF order recovery error: missing scan bucket for {}",
                target.object_type
            ))
        })?;
        for (index, idf_object) in idf_objects.iter().enumerate() {
            if idf_objects[..index].iter().any(|earlier| {
                if exact_names || converted_exact_names {
                    earlier.name == idf_object.name
                } else {
                    earlier.name.eq_ignore_ascii_case(&idf_object.name)
                }
            }) {
                return Err(IdfOrderError::new(format!(
                    "duplicate staged IDF {} name '{}' at line {}, column {}",
                    target.object_type, idf_object.name, idf_object.line, idf_object.column
                )));
            }
        }
        let matching_types = model
            .objects
            .keys()
            .filter(|object_type| object_type.0.eq_ignore_ascii_case(target.object_type))
            .collect::<Vec<_>>();

        if matching_types.len() > 1 {
            return Err(IdfOrderError::new(format!(
                "IDF declaration-order recovery for {} is ambiguous: converted epJSON contains multiple case-insensitive object-type matches",
                target.object_type
            )));
        }

        let Some(actual_type) = matching_types.first().copied() else {
            if idf_objects.is_empty() {
                continue;
            }
            return Err(IdfOrderError::new(format!(
                "IDF declaration-order recovery mismatch for {}: staged IDF has {} object(s), converted epJSON has none",
                target.object_type,
                idf_objects.len()
            )));
        };
        let instances = model.objects.get(actual_type).ok_or_else(|| {
            IdfOrderError::new(format!(
                "internal IDF order recovery error: converted epJSON type {} disappeared",
                actual_type.0
            ))
        })?;

        let actual_names = instances.keys().collect::<Vec<_>>();
        for (index, actual_name) in actual_names.iter().enumerate() {
            if actual_names[..index].iter().any(|earlier| {
                if exact_names || converted_exact_names {
                    earlier.0 == actual_name.0
                } else {
                    earlier.0.eq_ignore_ascii_case(&actual_name.0)
                }
            }) {
                return Err(IdfOrderError::new(format!(
                    "converted epJSON contains duplicate case-insensitive {} name '{}'",
                    actual_type.0, actual_name.0
                )));
            }
        }

        if idf_objects.len() != instances.len() {
            return Err(IdfOrderError::new(format!(
                "IDF declaration-order recovery count mismatch for {}: staged IDF has {}, converted epJSON has {}",
                actual_type.0,
                idf_objects.len(),
                instances.len()
            )));
        }
        if instances.is_empty() {
            continue;
        }

        let mut seen_actual_names = BTreeSet::new();
        let mut ordered_names = Vec::with_capacity(idf_objects.len());
        for idf_object in idf_objects {
            // Case variants retain separate exact converted keys in these families.
            // Prefer that genuine identity; retain the established unique
            // case-insensitive fallback when a caller supplies only one spelling.
            let exact_converted_name = if converted_exact_names {
                instances.keys().find(|name| name.0 == idf_object.name)
            } else {
                None
            };
            let matching_names = if let Some(name) = exact_converted_name {
                vec![name]
            } else {
                instances
                    .keys()
                    .filter(|name| {
                        if exact_names {
                            name.0 == idf_object.name
                        } else {
                            name.0.eq_ignore_ascii_case(&idf_object.name)
                        }
                    })
                    .collect::<Vec<_>>()
            };
            if matching_names.is_empty() {
                return Err(IdfOrderError::new(format!(
                    "IDF declaration-order recovery name mismatch for {}: staged IDF name '{}' (idf_order {}) is absent from converted epJSON",
                    actual_type.0, idf_object.name, idf_object.idf_order
                )));
            }
            if matching_names.len() > 1 {
                return Err(IdfOrderError::new(format!(
                    "IDF declaration-order recovery for {}/{} is ambiguous: converted epJSON contains multiple case-insensitive name matches",
                    actual_type.0, idf_object.name
                )));
            }

            let actual_name = matching_names[0].clone();
            if !seen_actual_names.insert(actual_name.clone()) {
                return Err(IdfOrderError::new(format!(
                    "IDF declaration-order recovery mapped more than one staged object to {}/{}",
                    actual_type.0, actual_name.0
                )));
            }
            // Native END returns before attaching its temporary extensible
            // array. Reject a staged counterpart that invents those rows.
            if exact_names
                && !idf_object.semicolon_terminated
                && instances[&actual_name]
                    .fields
                    .contains_key(&FieldName("angles".into()))
            {
                return Err(IdfOrderError::new(
                    "staged ComfortViewFactorAngles EOF must retain absent angles",
                ));
            }
            if source_object_layouts
                && !idf_object.semicolon_terminated
                && source_idf_object_layout(target.object_type)
                    .is_some_and(|(_, _, extension)| !extension.is_empty())
                && instances[&actual_name].fields.contains_key(&FieldName(
                    match target.object_type {
                        "OutdoorAir:NodeList" => "nodes",
                        _ => "extensions",
                    }
                    .into(),
                ))
            {
                return Err(IdfOrderError::new(
                    "Source original-IDF EOF must retain absent extension array",
                ));
            }
            if (source_object_layouts && source_idf_object_layout(target.object_type).is_some())
                || unnamed_legacy_type(target.object_type)
                || matches!(
                    target.object_type,
                    "Material:NoMass"
                        | "Material:AirGap"
                        | "Material:InfraredTransparent"
                        | "Material:RoofVegetation"
                        | "Building"
                        | "Construction"
                        | "Zone"
                        | "ZoneList"
                        | "BuildingSurface:Detailed"
                        | "OtherEquipment"
                        | "People"
                        | "ComfortViewFactorAngles"
                        | "Exterior:Lights"
                        | "Site:Location"
                        | "RunPeriod"
                        | "RunPeriodControl:SpecialDays"
                        | "SizingPeriod:DesignDay"
                        | "ZoneControl:Thermostat"
                        | "ZoneHVAC:IdealLoadsAirSystem"
                        | "SetpointManager:Scheduled"
                        | "Schedule:Day:Hourly"
                        | "Schedule:Day:Interval"
                        | "Schedule:Day:List"
                        | "Schedule:Week:Compact"
                        | "Schedule:Week:Daily"
                        | "Schedule:Year"
                        | "Schedule:File"
                )
            {
                report_field_counts.push((
                    actual_type.clone(),
                    actual_name.clone(),
                    match target.object_type {
                        family
                            if (source_object_layouts || family == "Material:RoofVegetation")
                                && source_idf_object_layout(family).is_some() =>
                        {
                            let (minimum, _, _) =
                                source_idf_object_layout(family).ok_or_else(|| {
                                    IdfOrderError::new("Source fixed layout disappeared")
                                })?;
                            if idf_object.semicolon_terminated {
                                idf_object.max_fields.max(minimum)
                            } else {
                                idf_object.max_fields.saturating_sub(1)
                            }
                        }
                        "BuildingSurface:Detailed" => idf_object.max_fields.max(20),
                        "Material:AirGap" | "RunPeriodControl:DaylightSavingTime" => {
                            idf_object.max_fields.max(2)
                        }
                        "RunPeriodControl:SpecialDays" => idf_object.max_fields.max(4),
                        // IdfParser::parse_object pads IDF counts to schema min_fields.
                        "OtherEquipment" => idf_object.max_fields.max(11),
                        "ZoneList" => idf_object.max_fields.max(2),
                        "ComfortViewFactorAngles" if !idf_object.semicolon_terminated => {
                            idf_object.max_fields.saturating_sub(1)
                        }
                        "People" if idf_object.semicolon_terminated => {
                            idf_object.max_fields.max(10)
                        }
                        // Native END neither advances the last field nor min-pads.
                        "People" => idf_object.max_fields.saturating_sub(1),
                        "Site:Location" => idf_object.max_fields.max(5),
                        "RunPeriod" => idf_object.max_fields.max(7),
                        "Schedule:Day:Hourly" => idf_object.max_fields.max(26),
                        "Schedule:Day:Interval" | "Schedule:Day:List" => {
                            idf_object.max_fields.max(5)
                        }
                        "Schedule:Week:Compact" => idf_object.max_fields.max(3),
                        "Schedule:Week:Daily" => idf_object.max_fields.max(13),
                        "Schedule:Year" => idf_object.max_fields.max(7),
                        "Schedule:File" if idf_object.semicolon_terminated => {
                            idf_object.max_fields.max(10)
                        }
                        "Schedule:File" => idf_object.max_fields.saturating_sub(1),
                        // This nonextensible schema has no min_fields. Native
                        // END retains the final value without advancing F.
                        "SetpointManager:Scheduled" if !idf_object.semicolon_terminated => {
                            idf_object.max_fields.saturating_sub(1)
                        }
                        // Pinned IdealLoads IDD/schema min_fields is 27; only semicolon records reach this padding.
                        "ZoneHVAC:IdealLoadsAirSystem" => idf_object.max_fields.max(27),
                        "Site:GroundTemperature:BuildingSurface"
                        | "Site:GroundTemperature:FCfactorMethod"
                        | "Site:GroundTemperature:Shallow"
                        | "Site:GroundTemperature:Deep" => idf_object.max_fields.max(12),
                        _ => idf_object.max_fields,
                    },
                    match target.object_type {
                        "ComfortViewFactorAngles" => idf_object.comfort_max_extensible_fields,
                        "BuildingSurface:Detailed" => {
                            Some(idf_object.max_fields.saturating_sub(11))
                        }
                        "Output:Table:SummaryReports" => Some(idf_object.max_fields),
                        "ZoneList" => Some(idf_object.max_fields.saturating_sub(1)),
                        "Schedule:Year" => Some(idf_object.max_fields.saturating_sub(2)),
                        "Schedule:Day:Interval" => Some(idf_object.max_fields.saturating_sub(3)),
                        "Schedule:Day:List" => Some(idf_object.max_fields.saturating_sub(4)),
                        "Schedule:Week:Compact" => Some(idf_object.max_fields.saturating_sub(1)),
                        family
                            if (source_object_layouts || family == "Material:RoofVegetation")
                                && source_idf_object_layout(family).is_some() =>
                        {
                            let (_, _, extension) =
                                source_idf_object_layout(family).ok_or_else(|| {
                                    IdfOrderError::new("Source IDF layout disappeared")
                                })?;
                            if extension.is_empty() {
                                Some(0)
                            } else {
                                Some(idf_object.source_max_extensible_fields.ok_or_else(|| {
                                    IdfOrderError::new(
                                        "Source original-IDF extension token custody absent",
                                    )
                                })?)
                            }
                        }
                        "SetpointManager:Scheduled" => Some(0),
                        _ => None,
                    },
                ));
            }
            // Native retains the global parser ordinal on each original-IDF object.
            // The Source layouts already own this same recovered declaration.
            if target.object_type == "SetpointManager:Scheduled"
                || (source_object_layouts && source_idf_object_layout(target.object_type).is_some())
            {
                report_idf_orders.push((
                    actual_type.clone(),
                    actual_name.clone(),
                    idf_object.idf_order,
                ));
            }
            ordered_names.push(actual_name);
        }

        overlay.insert(actual_type.clone(), ordered_names);
    }

    // Apply only after every type/name/count has been reconciled successfully.
    for (object_type, name, max_fields, max_extensible_fields) in report_field_counts {
        let object = model
            .objects
            .get_mut(&object_type)
            .and_then(|objects| objects.get_mut(&name))
            .ok_or_else(|| IdfOrderError::new("reconciled report input disappeared"))?;
        object.fields.insert(
            FieldName("idf_max_fields".to_string()),
            RawValue::Number(max_fields.to_string()),
        );
        if let Some(count) = max_extensible_fields {
            object.fields.insert(
                FieldName("idf_max_extensible_fields".into()),
                RawValue::Number(count.to_string()),
            );
        }
    }
    for (object_type, name, idf_order) in report_idf_orders {
        let object = model
            .objects
            .get_mut(&object_type)
            .and_then(|objects| objects.get_mut(&name))
            .ok_or_else(|| IdfOrderError::new("reconciled Scheduled input disappeared"))?;
        object.fields.insert(
            FieldName("idf_order".into()),
            RawValue::Number(idf_order.to_string()),
        );
    }
    model.idf_declaration_order = overlay;
    model.source_input_format = SourceInputFormat::Idf;
    Ok(())
}

pub(crate) fn apply_idf_declaration_order_bytes(
    model: &mut RawModel,
    idf: &[u8],
    targets: &[IdfOrderTarget],
) -> Result<(), IdfOrderError> {
    // EnergyPlus token delimiters are ASCII and comments may contain legacy bytes.
    // Lossy decoding therefore keeps token boundaries while reconciliation still
    // fails closed if replacement bytes occur in a targeted object name.
    apply_idf_declaration_order(model, &String::from_utf8_lossy(idf), targets)
}

/// One transaction keeps common targets and adds the closed Source fixed-material layouts.
pub(crate) fn apply_source_idf_declaration_order_bytes(
    model: &mut RawModel,
    idf: &[u8],
) -> Result<(), IdfOrderError> {
    let mut targets = IDF_ORDER_TARGETS.to_vec();
    for &family in SOURCE_IDF_OBJECT_FAMILIES {
        // Source fixed layouts may share an existing common target (AirGap).
        // Keep that same target once; the legacy target set is unchanged.
        if !targets.iter().any(|target| target.object_type == family) {
            targets.push(IdfOrderTarget {
                object_type: family,
                name_field_index: 0,
            });
        }
    }
    apply_idf_declaration_order_inner(model, &String::from_utf8_lossy(idf), &targets, true)
}

fn validate_targets(targets: &[IdfOrderTarget]) -> Result<(), IdfOrderError> {
    for (index, target) in targets.iter().enumerate() {
        if target.object_type.trim().is_empty() {
            return Err(IdfOrderError::new(
                "IDF declaration-order target object type must not be blank",
            ));
        }
        if targets[..index]
            .iter()
            .any(|earlier| earlier.object_type.eq_ignore_ascii_case(target.object_type))
        {
            return Err(IdfOrderError::new(format!(
                "duplicate IDF declaration-order target {}",
                target.object_type
            )));
        }
    }
    Ok(())
}

fn scan_target_objects(
    idf: &str,
    targets: &[IdfOrderTarget],
    source_object_layouts: bool,
) -> Result<BTreeMap<&'static str, Vec<ScannedObject>>, IdfOrderError> {
    let idf = idf.trim_start_matches('\u{feff}');
    let mut scanned = targets
        .iter()
        .map(|target| (target.object_type, Vec::new()))
        .collect::<BTreeMap<_, _>>();
    let mut fields = Vec::<String>::new();
    let mut field = String::new();
    let mut parsed_field_segment = None::<String>;
    let mut object_start = None::<(usize, usize)>;
    let mut object_byte_start = 0;
    let mut in_comment = false;
    let mut idf_order = 0_u32;
    let mut line = 1_usize;
    let mut column = 1_usize;

    for (byte_index, character) in idf.char_indices() {
        let character_line = line;
        let character_column = column;

        if in_comment {
            if character == '\n' {
                in_comment = false;
            }
            advance_position(character, &mut line, &mut column);
            continue;
        }

        match character {
            '!' => {
                // IdfParser treats a comment as a value-token boundary. If text
                // resumes before the comma, its last nonblank segment wins.
                let parsed_segment = normalize_field(&field);
                if !parsed_segment.is_empty() {
                    parsed_field_segment = Some(parsed_segment);
                }
                field.clear();
                in_comment = true;
            }
            ',' => {
                if object_start.is_none() && fields.is_empty() && normalized_field_is_empty(&field)
                {
                    return Err(IdfOrderError::new(format!(
                        "malformed IDF at line {character_line}, column {character_column}: unexpected comma before an object type"
                    )));
                }
                fields.push(finish_field(&mut field, &mut parsed_field_segment));
            }
            ';' => {
                if object_start.is_some()
                    || !fields.is_empty()
                    || !normalized_field_is_empty(&field)
                {
                    if source_object_layouts && fields.is_empty() {
                        // Native parse_object advances past the header delimiter
                        // unconditionally, including a type-only semicolon. Keep
                        // the following token in this same actual object.
                        fields.push(finish_field(&mut field, &mut parsed_field_segment));
                    } else {
                        fields.push(finish_field(&mut field, &mut parsed_field_segment));
                        idf_order = idf_order.checked_add(1).ok_or_else(|| {
                            IdfOrderError::new("IDF declaration-order counter exceeded u32")
                        })?;
                        record_object(
                            &fields,
                            object_start.unwrap_or((character_line, character_column)),
                            idf_order,
                            (true, source_object_layouts),
                            &idf[object_byte_start..byte_index + 1],
                            targets,
                            &mut scanned,
                        )?;
                        fields.clear();
                        object_start = None;
                        parsed_field_segment = None;
                    }
                }
            }
            _ => {
                if object_start.is_none() && !is_idf_leading_whitespace(character) {
                    object_start = Some((character_line, character_column));
                    object_byte_start = byte_index;
                }
                field.push(character);
            }
        }

        advance_position(character, &mut line, &mut column);
    }

    if object_start.is_some() || !fields.is_empty() || !normalized_field_is_empty(&field) {
        let (start_line, start_column) = object_start.unwrap_or((line, column));
        fields.push(finish_field(&mut field, &mut parsed_field_segment));
        idf_order = idf_order
            .checked_add(1)
            .ok_or_else(|| IdfOrderError::new("IDF declaration-order counter exceeded u32"))?;
        // Native EOF returns without advancing the final field or min-padding.
        // These metadata targets require a semicolon; all other EOF targets
        // retain their existing ordering behavior. DD ordering-only EOF input
        // is now explicitly outside this newly required metadata domain.
        if let Some(object_type) = fields.first()
            && (object_type.eq_ignore_ascii_case("RunPeriod")
                || object_type.eq_ignore_ascii_case("SizingPeriod:DesignDay")
                || object_type.eq_ignore_ascii_case("ZoneList")
                || object_type.eq_ignore_ascii_case("ZoneControl:Thermostat")
                || object_type.eq_ignore_ascii_case("ZoneHVAC:IdealLoadsAirSystem"))
            && targets
                .iter()
                .any(|target| target.object_type.eq_ignore_ascii_case(object_type))
        {
            return Err(IdfOrderError::new(format!(
                "staged {object_type} requires a terminating semicolon for exact IDF field metadata",
            )));
        }
        record_object(
            &fields,
            (start_line, start_column),
            idf_order,
            (false, source_object_layouts),
            &idf[object_byte_start..],
            targets,
            &mut scanned,
        )?;
        // IdfParser returns at END before advancing its final field or
        // min-padding. Preserve the actual count for this newly connected
        // unnamed report; Alphas(1) still contains the parsed key_field.
        if fields
            .first()
            .is_some_and(|family| family.eq_ignore_ascii_case("Output:Schedules"))
            && let Some(last) = scanned
                .get_mut("Output:Schedules")
                .and_then(|rows| rows.last_mut())
        {
            last.max_fields = last.max_fields.saturating_sub(1);
        }
    }

    Ok(scanned)
}

fn unnamed_legacy_type(object_type: &str) -> bool {
    matches!(
        object_type,
        "GlobalGeometryRules"
            | "Version"
            | "HeatBalanceAlgorithm"
            | "Timestep"
            | "SimulationControl"
            | "RunPeriodControl:DaylightSavingTime"
            | "SurfaceConvectionAlgorithm:Inside"
            | "SurfaceConvectionAlgorithm:Outside"
            | "ZoneAirHeatBalanceAlgorithm"
            | "Site:GroundTemperature:BuildingSurface"
            | "Site:GroundTemperature:FCfactorMethod"
            | "Site:GroundTemperature:Shallow"
            | "Site:GroundTemperature:Deep"
            | "Output:Surfaces:List"
            | "Output:Surfaces:Drawing"
            | "Output:VariableDictionary"
            | "Output:Constructions"
            | "Output:EnergyManagementSystem"
            | "Output:Schedules"
            | "Output:Variable"
            | "Output:Meter"
            | "Output:Meter:MeterFileOnly"
            | "Output:Meter:Cumulative"
            | "Output:Meter:Cumulative:MeterFileOnly"
            | "OutputControl:Table:Style"
            | "Output:Table:SummaryReports"
    )
}

// Reuse the existing event scanner: E advances for each parsed value or blank
// extension, while F advances only at delimiters. The sole layout owner supplies
// field kinds: strings include newlines; numerics stop at CR/LF/comments.
fn idf_extensible_fields(
    raw_object: &str,
    fixed_fields: usize,
    numeric_field: impl Fn(usize, usize) -> bool,
    source_object_layouts: bool,
) -> usize {
    let bytes = raw_object.as_bytes();
    let Some(comma) = bytes
        .iter()
        .position(|byte| *byte == b',' || (source_object_layouts && *byte == b';'))
    else {
        return 0;
    };
    let mut cursor = comma + 1;
    let mut legacy_index = 0;
    let mut extensible_index = 0;
    let mut was_value_parsed = false;
    loop {
        while bytes
            .get(cursor)
            .is_some_and(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
        {
            cursor += 1;
        }
        let Some(byte) = bytes.get(cursor) else {
            return extensible_index;
        };
        match byte {
            b',' | b';' => {
                if !was_value_parsed && legacy_index >= fixed_fields {
                    extensible_index += 1;
                }
                legacy_index += 1;
                was_value_parsed = false;
                cursor += 1;
                if *byte == b';' {
                    return extensible_index;
                }
            }
            b'!' => {
                while bytes.get(cursor).is_some_and(|byte| *byte != b'\n') {
                    cursor += 1;
                }
            }
            _ => {
                let numeric = numeric_field(legacy_index, extensible_index);
                while bytes.get(cursor).is_some_and(|byte| {
                    !(matches!(byte, b'!' | b',' | b';')
                        || numeric && matches!(byte, b'\r' | b'\n'))
                }) {
                    cursor += 1;
                }
                if legacy_index >= fixed_fields {
                    extensible_index += 1;
                }
                was_value_parsed = true;
            }
        }
    }
}

// Comfort keeps its original one-Name and alternating string/numeric semantics.
fn comfort_extensible_fields(raw_object: &str) -> usize {
    idf_extensible_fields(
        raw_object,
        1,
        |legacy_index, extensible_index| legacy_index >= 1 && extensible_index % 2 == 1,
        false,
    )
}

fn record_object(
    fields: &[String],
    (line, column): (usize, usize),
    idf_order: u32,
    (semicolon_terminated, source_object_layouts): (bool, bool),
    raw_object: &str,
    targets: &[IdfOrderTarget],
    scanned: &mut BTreeMap<&'static str, Vec<ScannedObject>>,
) -> Result<(), IdfOrderError> {
    let Some(object_type) = fields.first() else {
        return Err(IdfOrderError::new(format!(
            "malformed IDF object at line {line}, column {column}: missing object type"
        )));
    };
    if object_type.is_empty() {
        return Err(IdfOrderError::new(format!(
            "malformed IDF object at line {line}, column {column}: blank object type"
        )));
    }

    let Some(target) = targets
        .iter()
        .find(|target| object_type.eq_ignore_ascii_case(target.object_type))
    else {
        return Ok(());
    };
    if unnamed_legacy_type(target.object_type)
        || (source_object_layouts && target.object_type == "OutdoorAir:NodeList")
    {
        let bucket = scanned.get_mut(target.object_type).ok_or_else(|| {
            IdfOrderError::new(format!(
                "internal IDF order recovery error: missing target bucket for {}",
                target.object_type
            ))
        })?;
        bucket.push(ScannedObject {
            idf_order,
            max_fields: fields.len() - 1,
            semicolon_terminated,
            source_max_extensible_fields: (source_object_layouts
                && target.object_type == "OutdoorAir:NodeList")
                .then(|| idf_extensible_fields(raw_object, 0, |_, _| false, true)),
            comfort_max_extensible_fields: None,
            name: format!("{} {}", target.object_type, bucket.len() + 1),
            line,
            column,
        });
        return Ok(());
    }
    let field_index = target.name_field_index.checked_add(1).ok_or_else(|| {
        IdfOrderError::new(format!(
            "IDF declaration-order target {} has an invalid name-field index",
            target.object_type
        ))
    })?;
    let comfort = target.object_type == "ComfortViewFactorAngles";
    let name = match fields.get(field_index) {
        Some(name) => name.as_str(),
        None if comfort => "",
        None => {
            return Err(IdfOrderError::new(format!(
                "malformed staged IDF {} object at line {line}, column {column}: missing required name field {}",
                target.object_type, target.name_field_index
            )));
        }
    };
    if name.is_empty() && !comfort {
        return Err(IdfOrderError::new(format!(
            "malformed staged IDF {} object at line {line}, column {column}: required name is blank",
            target.object_type
        )));
    }

    let bucket = scanned.get_mut(target.object_type).ok_or_else(|| {
        IdfOrderError::new(format!(
            "internal IDF order recovery error: missing target bucket for {}",
            target.object_type
        ))
    })?;
    bucket.push(ScannedObject {
        idf_order,
        max_fields: fields.len() - 1,
        semicolon_terminated,
        source_max_extensible_fields: source_idf_object_layout(target.object_type).and_then(
            |(_, fixed, extension)| {
                (!extension.is_empty()).then(|| {
                    idf_extensible_fields(
                        raw_object,
                        fixed.len(),
                        |legacy_index, extensible_index| {
                            if legacy_index < fixed.len() {
                                !fixed[legacy_index].1
                            } else {
                                !extension[extensible_index % extension.len()].1
                            }
                        },
                        source_object_layouts,
                    )
                })
            },
        ),
        // Values can advance across comments/numeric newlines without a
        // delimiter. Keep that Native event count independent of legacy F.
        comfort_max_extensible_fields: comfort.then(|| comfort_extensible_fields(raw_object)),
        name: name.to_owned(),
        line,
        column,
    });
    Ok(())
}

fn normalize_field(field: &str) -> String {
    field
        .trim_start_matches(is_idf_leading_whitespace)
        .trim_end_matches(is_idf_trailing_whitespace)
        .to_string()
}

fn normalized_field_is_empty(field: &str) -> bool {
    normalize_field(field).is_empty()
}

fn finish_field(field: &mut String, parsed_segment: &mut Option<String>) -> String {
    let current_segment = normalize_field(field);
    field.clear();
    if current_segment.is_empty() {
        parsed_segment.take().unwrap_or(current_segment)
    } else {
        *parsed_segment = None;
        current_segment
    }
}

fn is_idf_leading_whitespace(character: char) -> bool {
    matches!(character, ' ' | '\r' | '\t' | '\n')
}

fn is_idf_trailing_whitespace(character: char) -> bool {
    matches!(character, ' ' | '\t' | '\0')
}

fn advance_position(character: char, line: &mut usize, column: &mut usize) {
    if character == '\n' {
        *line += 1;
        *column = 1;
    } else {
        *column += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        IDF_ORDER_TARGETS, IdfOrderError, apply_idf_declaration_order,
        apply_idf_declaration_order_bytes,
    };
    use crate::{FieldName, RawValue, parse_epjson_str, parse_epjson_str_with_idf_order};

    const TWO_SPECIAL_DAYS_EPJSON: &str = r#"{
        "RunPeriodControl:SpecialDays": {
            "Zulu Earlier Holiday": {
                "start_date": "6/15",
                "duration": 1,
                "special_day_type": "Holiday"
            },
            "Alpha Later Custom": {
                "start_date": "6/15",
                "duration": 1,
                "special_day_type": "CustomDay2"
            }
        }
    }"#;

    const REVERSE_ALPHABET_IDF: &str = r#"
        RunPeriodControl:SpecialDays,
          Zulu Earlier Holiday,
          6/15,
          1,
          Holiday;
        RunPeriodControl:SpecialDays,
          Alpha Later Custom,
          6/15,
          1,
          CustomDay2;
    "#;

    const SELECTED_SCHEDULE_EPJSON: &str = r#"{
        "SizingPeriod:DesignDay": {
            "A Design": {"month": 7},
            "Z Design": {"month": 12}
        },
        "ScheduleTypeLimits": {
            "A Type": {"lower_limit_value": 0, "upper_limit_value": 1},
            "Z Type": {"lower_limit_value": 0, "upper_limit_value": 1}
        },
        "Schedule:Constant": {
            "A Constant": {"schedule_type_limits_name": "A Type", "hourly_value": 0},
            "Z Constant": {"schedule_type_limits_name": "Z Type", "hourly_value": 1}
        },
        "Schedule:File": {
            "A File": {"file_name": "MixedCase.CsV", "column_number": 1, "rows_to_skip_at_top": 0},
            "Z File": {"file_name": "MixedCase.CsV", "column_number": 2, "rows_to_skip_at_top": 0}
        },
        "Schedule:Compact": {
            "A Compact": {"schedule_type_limits_name": "A Type"},
            "Z Compact": {"schedule_type_limits_name": "Z Type"}
        }
    }"#;

    const SELECTED_SCHEDULE_IDF: &str = r#"
        SizingPeriod:DesignDay,z design,12;
        SizingPeriod:DesignDay,a design,7;
        Schedule:File,z file,Z Type,MixedCase.CsV,2,0,8760,Comma,No,60,Yes;
        Schedule:File,a file,A Type,MixedCase.CsV,1,0,8760,Comma,No,60,Yes;
        Schedule:Constant,z constant,Z Type,1;
        ScheduleTypeLimits,z type,0,1,Continuous,Dimensionless;
        Schedule:Compact,z compact,Z Type,Through:12/31,For:AllDays,Until:24:00,1;
        Schedule:Compact,a compact,A Type,Through:12/31,For:AllDays,Until:24:00,0;
        ScheduleTypeLimits,a type,0,1,Continuous,Dimensionless;
        Schedule:Constant,a constant,A Type,0;
    "#;

    fn ordered_names(model: &crate::RawModel) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        Ok(model
            .ordered_instances("RunPeriodControl:SpecialDays")?
            .into_iter()
            .map(|(name, _object)| name.0.clone())
            .collect())
    }

    fn required_order_error(
        result: Result<(), IdfOrderError>,
        message: &str,
    ) -> Result<IdfOrderError, Box<dyn std::error::Error>> {
        match result {
            Ok(()) => Err(std::io::Error::other(message).into()),
            Err(error) => Ok(error),
        }
    }

    #[test]
    fn native_epjson_retains_canonical_name_order() -> Result<(), Box<dyn std::error::Error>> {
        let model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;

        assert!(!model.has_idf_declaration_order("RunPeriodControl:SpecialDays"));
        assert_eq!(
            ordered_names(&model)?,
            vec!["Alpha Later Custom", "Zulu Earlier Holiday"]
        );
        // The default/legacy BTree view stays unchanged. Explicit Source uses
        // the pinned signed-byte/numeric order observed by the Native IP.
        let direct = parse_epjson_str(
            r#"{"Schedule:File":{"AlwaysOn ":{},"A10":{},"\u00a0":{},"AlwaysOn":{},"A2":{}}}"#,
        )?;
        assert_eq!(
            direct
                .source_ordered_instances("Schedule:File")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["\u{a0}", "A2", "A10", "AlwaysOn", "AlwaysOn "]
        );
        assert_eq!(
            direct
                .ordered_instances("Schedule:File")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["A10", "A2", "AlwaysOn", "AlwaysOn ", "\u{a0}"]
        );
        let numeric = parse_epjson_str(
            r#"{"Schedule:File":{"x10":{},"x2":{},"x1":{},"x01":{},"0":{},"A":{},"a":{}}}"#,
        )?;
        assert_eq!(
            numeric
                .source_ordered_instances("Schedule:File")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["0", "A", "a", "x01", "x1", "x2", "x10"]
        );
        // Ordinary wrapped arithmetic is retained. Distinct aliases,
        // antipodes, and cycles in uncoalesced maps stay unavailable.
        // Singleton NUL names are covered by the existing lexical parser body.
        let wrapped = parse_epjson_str(r#"{"Schedule:File":{"4294967296":{},"1":{}}}"#)?;
        assert_eq!(
            wrapped
                .source_ordered_instances("Schedule:File")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["4294967296", "1"]
        );
        for text in [
            r#"{"Schedule:File":{"0000000000":{},"4294967296":{}}}"#,
            r#"{"Schedule:File":{"0":{},"2147483648":{}}}"#,
            r#"{"Schedule:File":{"0":{},"1431655765":{},"2863311530":{}}}"#,
        ] {
            let bounded = parse_epjson_str(text)?;
            assert!(
                bounded
                    .source_ordered_instances("Schedule:File")
                    .is_err_and(|error| error.starts_with("SourceObjectOrderUnavailable:"))
            );
            assert_eq!(
                bounded.ordered_instances("Schedule:File")?.len(),
                bounded.object_count()
            );
        }
        let staged =
            parse_epjson_str_with_idf_order(SELECTED_SCHEDULE_EPJSON, SELECTED_SCHEDULE_IDF)?;
        for family in [
            "ScheduleTypeLimits",
            "Schedule:Constant",
            "Schedule:Compact",
            "Schedule:File",
            "SizingPeriod:DesignDay",
        ] {
            assert_eq!(
                staged.source_ordered_instances(family)?,
                staged.ordered_instances(family)?
            );
        }
        // Direct DOM readers keep JSON map order for converted IDF too;
        // this must not silently become the getObjectItem declaration overlay.
        assert_eq!(
            staged
                .source_json_instances("Schedule:File")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["A File", "Z File"]
        );
        assert_eq!(
            staged
                .source_ordered_instances("Schedule:File")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["Z File", "A File"]
        );
        let mut invalid_overlay = staged.clone();
        invalid_overlay
            .objects
            .remove(&crate::ObjectType("Schedule:File".into()));
        let old_error = invalid_overlay
            .ordered_instances("Schedule:File")
            .err()
            .ok_or("existing invalid overlay must fail")?
            .to_string();
        assert_eq!(
            invalid_overlay.source_ordered_instances("Schedule:File"),
            Err(old_error)
        );
        assert!(direct.source_ordered_instances("Absent:Family")?.is_empty());
        Ok(())
    }

    #[test]
    fn selected_schedule_families_retain_idf_order_without_changing_direct_epjson()
    -> Result<(), Box<dyn std::error::Error>> {
        let staged =
            parse_epjson_str_with_idf_order(SELECTED_SCHEDULE_EPJSON, SELECTED_SCHEDULE_IDF)?;
        let direct = parse_epjson_str(SELECTED_SCHEDULE_EPJSON)?;

        for (object_type, earlier, later) in [
            ("SizingPeriod:DesignDay", "Z Design", "A Design"),
            ("ScheduleTypeLimits", "Z Type", "A Type"),
            ("Schedule:Constant", "Z Constant", "A Constant"),
            ("Schedule:Compact", "Z Compact", "A Compact"),
            ("Schedule:File", "Z File", "A File"),
        ] {
            let names = |model: &crate::RawModel| -> Result<Vec<String>, IdfOrderError> {
                Ok(model
                    .ordered_instances(object_type)?
                    .into_iter()
                    .map(|(name, _)| name.0.clone())
                    .collect())
            };
            assert_eq!(names(&staged)?, vec![earlier, later]);
            assert_eq!(names(&direct)?, vec![later, earlier]);
            assert!(staged.has_idf_declaration_order(object_type));
            assert!(!direct.has_idf_declaration_order(object_type));
        }
        // The two physical DD rows each contain Name and Month. Preserve the
        // complete direct object tree while expecting only those real bounds.
        let mut expected = direct.objects.clone();
        for object in expected
            .get_mut(&crate::ObjectType("SizingPeriod:DesignDay".into()))
            .ok_or("existing named DesignDay fixture objects")?
            .values_mut()
        {
            object.fields.insert(
                FieldName("idf_max_fields".into()),
                RawValue::Number("2".into()),
            );
        }
        for object in expected
            .get_mut(&crate::ObjectType("Schedule:File".into()))
            .ok_or("existing named File fixture objects")?
            .values_mut()
        {
            object.fields.insert(
                FieldName("idf_max_fields".into()),
                RawValue::Number("10".into()),
            );
        }
        assert_eq!(staged.objects, expected);
        // The File getter needs actual NumNumbers: semicolon records acquire
        // min_fields10, while Native EOF neither advances nor pads its last field.
        for (idf, count) in [
            ("Schedule:File,F,,data.csv,1,0;", "10"),
            ("Schedule:File,F,,data.csv,1,0,8760,Comma,No,20,No", "9"),
        ] {
            let file = parse_epjson_str_with_idf_order(r#"{"Schedule:File":{"F":{}}}"#, idf)?;
            assert_eq!(
                file.ordered_instances("Schedule:File")?[0]
                    .1
                    .fields
                    .get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number(count.into()))
            );
        }
        // Actual staged declaration order/counts for the regular chain, with
        // reverse lexical Day names. Parser min-fields and extensible counts
        // remain distinct; direct epJSON acquires no IDF metadata.
        let json = r#"{"Schedule:Day:Hourly":{"Z Day":{},"A Day":{}},"Schedule:Week:Daily":{"W":{}},"Schedule:Year":{"Y":{"schedule_weeks":[{"schedule_week_name":"W","start_month":1,"start_day":1,"end_month":12,"end_day":31}]}}}"#;
        let idf = "Schedule:Day:Hourly,Z Day;\nSchedule:Day:Hourly,A Day;\nSchedule:Week:Daily,W,Z Day,Z Day,Z Day,Z Day,Z Day,Z Day,Z Day,Z Day,Z Day,Z Day,Z Day,Z Day;\nSchedule:Year,Y,,W,1,1,12,31;";
        let ordered = parse_epjson_str_with_idf_order(json, idf)?;
        assert_eq!(
            ordered
                .ordered_instances("Schedule:Day:Hourly")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["Z Day", "A Day"]
        );
        let direct = parse_epjson_str(json)?;
        for (family, count, ext) in [
            ("Schedule:Day:Hourly", "26", None),
            ("Schedule:Week:Daily", "13", None),
            ("Schedule:Year", "7", Some("5")),
        ] {
            for (_, object) in ordered.ordered_instances(family)? {
                assert_eq!(
                    object.fields.get(&FieldName("idf_max_fields".into())),
                    Some(&RawValue::Number(count.into()))
                );
                if let Some(ext) = ext {
                    assert_eq!(
                        object
                            .fields
                            .get(&FieldName("idf_max_extensible_fields".into())),
                        Some(&RawValue::Number(ext.into()))
                    );
                }
            }
            assert!(!direct.has_idf_declaration_order(family));
        }
        // Semicolon padding changes only max_fields. Genuine physical
        // extension counts remain partial or zero, including blank fields.
        let json = r#"{"Schedule:Day:Interval":{"Z Interval":{"data":[{"time":"24:00"}]},"A Interval":{}},"Schedule:Day:List":{"Z List":{"minutes_per_item":60,"extensions":[{"value":0}]},"A List":{}},"Schedule:Week:Compact":{"Z Week":{"data":[{"daytype_list":"AllDays","schedule_day_name":"Z Interval"}]},"A Week":{}}}"#;
        let idf = "Schedule:Day:Interval,Z Interval,Fraction,No,24:00;\nSchedule:Day:Interval,A Interval;\nSchedule:Day:List,Z List,Fraction,No,60,0;\nSchedule:Day:List,A List;\nSchedule:Week:Compact,Z Week,AllDays,Z Interval;\nSchedule:Week:Compact,A Week;";
        let ordered = parse_epjson_str_with_idf_order(json, idf)?;
        let direct = parse_epjson_str(json)?;
        for (family, names, count, extension) in [
            (
                "Schedule:Day:Interval",
                ["Z Interval", "A Interval"],
                "5",
                "1",
            ),
            ("Schedule:Day:List", ["Z List", "A List"], "5", "1"),
            ("Schedule:Week:Compact", ["Z Week", "A Week"], "3", "2"),
        ] {
            let objects = ordered.ordered_instances(family)?;
            assert_eq!(
                objects
                    .iter()
                    .map(|(name, _)| name.0.as_str())
                    .collect::<Vec<_>>(),
                names
            );
            for (index, (_, object)) in objects.iter().enumerate() {
                assert_eq!(
                    object.fields.get(&FieldName("idf_max_fields".into())),
                    Some(&RawValue::Number(count.into()))
                );
                assert_eq!(
                    object
                        .fields
                        .get(&FieldName("idf_max_extensible_fields".into())),
                    Some(&RawValue::Number(
                        if index == 0 { extension } else { "0" }.into()
                    ))
                );
            }
            assert!(!direct.has_idf_declaration_order(family));
            for (_, object) in direct.ordered_instances(family)? {
                assert!(
                    !object
                        .fields
                        .contains_key(&FieldName("idf_max_fields".into()))
                );
                assert!(
                    !object
                        .fields
                        .contains_key(&FieldName("idf_max_extensible_fields".into()))
                );
            }
        }
        // Comfort lists preserve exact original identities, including blank
        // and case variants, before the shared Alpha Name becomes uppercase.
        let json = r#"{"ComfortViewFactorAngles":{"a":{},"":{},"A":{"angles":[{"surface_name":"Wall One","angle_factor":1}]}}}"#;
        let idf = "ComfortViewFactorAngles,a;\nComfortViewFactorAngles,;\nComfortViewFactorAngles,A,Wall One,1;";
        let ordered = parse_epjson_str_with_idf_order(json, idf)?;
        assert_eq!(
            ordered
                .ordered_instances("ComfortViewFactorAngles")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["a", "", "A"]
        );
        for (name, fields, extensible) in [("a", "1", "0"), ("", "1", "0"), ("A", "3", "2")] {
            let object = &ordered.objects[&crate::ObjectType("ComfortViewFactorAngles".into())]
                [&crate::ObjectName(name.into())];
            assert_eq!(
                object.fields.get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number(fields.into()))
            );
            assert_eq!(
                object
                    .fields
                    .get(&FieldName("idf_max_extensible_fields".into())),
                Some(&RawValue::Number(extensible.into()))
            );
        }
        let blank = parse_epjson_str_with_idf_order(
            r#"{"ComfortViewFactorAngles":{"":{}}}"#,
            "ComfortViewFactorAngles,   ;",
        )?;
        assert_eq!(
            blank.ordered_instances("ComfortViewFactorAngles")?[0].0.0,
            ""
        );
        assert!(
            parse_epjson_str_with_idf_order(
                json,
                "ComfortViewFactorAngles,a;ComfortViewFactorAngles,a;ComfortViewFactorAngles,A;"
            )
            .is_err()
        );
        for (idf, fields, extensible) in [
            ("ComfortViewFactorAngles,L,S,1", "2", "2"),
            ("ComfortViewFactorAngles,L,S,", "2", "1"),
            ("ComfortViewFactorAngles,L,S", "1", "1"),
        ] {
            // Genuine IdfParser END returns before attaching temporary angles.
            let raw =
                parse_epjson_str_with_idf_order(r#"{"ComfortViewFactorAngles":{"L":{}}}"#, idf)?;
            let object = raw.ordered_instances("ComfortViewFactorAngles")?[0].1;
            assert_eq!(
                object.fields.get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number(fields.into()))
            );
            assert_eq!(
                object
                    .fields
                    .get(&FieldName("idf_max_extensible_fields".into())),
                Some(&RawValue::Number(extensible.into()))
            );
            assert!(!object.fields.contains_key(&FieldName("angles".into())));
            assert!(parse_epjson_str_with_idf_order(r#"{"ComfortViewFactorAngles":{"L":{"angles":[{"surface_name":"S","angle_factor":1}]}}}"#, idf).is_err());
        }
        // A comment can end a string value without advancing legacy F;
        // a numeric CR/LF also ends a value and selects the next E operand.
        for (idf, angles, fields, extensible) in [
            (
                "ComfortViewFactorAngles,L,Wall One !comment\n 1;",
                r#"[{"surface_name":"Wall One","angle_factor":1}]"#,
                "2",
                "2",
            ),
            (
                "ComfortViewFactorAngles,L,Wall One,0.5\nWall Two,0.5;",
                r#"[{"surface_name":"Wall One","angle_factor":0.5},{"surface_name":"Wall Two","angle_factor":0.5}]"#,
                "4",
                "4",
            ),
            (
                "ComfortViewFactorAngles,L,Wall One !one\n0.5 !two\nWall Two !three\n0.5;",
                r#"[{"surface_name":"Wall One","angle_factor":0.5},{"surface_name":"Wall Two","angle_factor":0.5}]"#,
                "2",
                "4",
            ),
            (
                "ComfortViewFactorAngles,L,Wall\n One,1;",
                r#"[{"surface_name":"Wall\n One","angle_factor":1}]"#,
                "3",
                "2",
            ),
            (
                "ComfortViewFactorAngles,L,Wall One,!comment\n;",
                r#"[{"surface_name":"Wall One"}]"#,
                "3",
                "2",
            ),
        ] {
            let json = format!(r#"{{"ComfortViewFactorAngles":{{"L":{{"angles":{angles}}}}}}}"#);
            let raw = parse_epjson_str_with_idf_order(&json, idf)?;
            let object = raw.ordered_instances("ComfortViewFactorAngles")?[0].1;
            assert_eq!(
                object.fields.get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number(fields.into()))
            );
            assert_eq!(
                object
                    .fields
                    .get(&FieldName("idf_max_extensible_fields".into())),
                Some(&RawValue::Number(extensible.into()))
            );
        }
        for (idf, fields, extensible) in [
            ("ComfortViewFactorAngles,L,Wall One !comment\n 1", "1", "2"),
            (
                "ComfortViewFactorAngles,L,Wall One,0.5\nWall Two,0.5",
                "3",
                "4",
            ),
        ] {
            let raw =
                parse_epjson_str_with_idf_order(r#"{"ComfortViewFactorAngles":{"L":{}}}"#, idf)?;
            let object = raw.ordered_instances("ComfortViewFactorAngles")?[0].1;
            assert_eq!(
                object.fields.get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number(fields.into()))
            );
            assert_eq!(
                object
                    .fields
                    .get(&FieldName("idf_max_extensible_fields".into())),
                Some(&RawValue::Number(extensible.into()))
            );
            assert!(!object.fields.contains_key(&FieldName("angles".into())));
        }
        Ok(())
    }

    #[test]
    fn selected_schedule_overlay_rejects_duplicate_and_unmatched_names_transactionally()
    -> Result<(), Box<dyn std::error::Error>> {
        let original = parse_epjson_str(SELECTED_SCHEDULE_EPJSON)?;

        for later in ["a type", "a constant", "a compact"] {
            let earlier = later.replacen('a', "z", 1);
            for (replacement, expected_error) in [
                (earlier.to_uppercase(), "duplicate staged IDF"),
                (format!("Missing {later}"), "name mismatch"),
            ] {
                let mut model = original.clone();
                let idf = SELECTED_SCHEDULE_IDF.replace(later, &replacement);
                let error = required_order_error(
                    apply_idf_declaration_order(&mut model, &idf, IDF_ORDER_TARGETS),
                    "ambiguous or unmatched schedule names must fail",
                )?;
                assert!(error.to_string().contains(expected_error));
                assert_eq!(model, original);
            }
        }
        // NodeList's exact-key exception must not relax other families.
        let mut model = parse_epjson_str(
            r#"{"Schedule:Constant":{"Case":{"hourly_value":1},"case":{"hourly_value":2}}}"#,
        )?;
        let before = model.clone();
        let error = required_order_error(
            apply_idf_declaration_order(
                &mut model,
                "Schedule:Constant,case,,2;Schedule:Constant,Case,,1;",
                IDF_ORDER_TARGETS,
            ),
            "schedule case variants retain their existing duplicate guard",
        )?;
        assert!(error.to_string().contains("duplicate staged IDF"));
        assert_eq!(model, before);
        Ok(())
    }

    #[test]
    fn surface_declaration_order_survives_conversion_without_changing_native_epjson()
    -> Result<(), Box<dyn std::error::Error>> {
        let epjson = r#"{"BuildingSurface:Detailed": {
            "Zulu Wall": {"surface_type":"Wall"},
            "Alpha Floor": {"surface_type":"Floor"},
            "Beta Roof": {"surface_type":"Roof"}
        }}"#;
        let idf = "BuildingSurface:Detailed,ZULU WALL,Wall;\n\
                   BuildingSurface:Detailed,beta roof,Roof;\n\
                   BuildingSurface:Detailed,Alpha Floor,Floor;";
        let staged = parse_epjson_str_with_idf_order(epjson, idf)?;
        let native = parse_epjson_str(epjson)?;
        let names = |model: &crate::RawModel| -> Result<Vec<String>, crate::EpJsonError> {
            Ok(model
                .ordered_instances("BuildingSurface:Detailed")?
                .into_iter()
                .map(|(name, _)| name.0.clone())
                .collect())
        };
        assert_eq!(names(&staged)?, ["Zulu Wall", "Beta Roof", "Alpha Floor"]);
        assert_eq!(names(&native)?, ["Alpha Floor", "Beta Roof", "Zulu Wall"]);
        for (_, object) in staged.ordered_instances("BuildingSurface:Detailed")? {
            assert_eq!(
                object.fields.get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number("20".into()))
            );
            assert_eq!(
                object
                    .fields
                    .get(&FieldName("idf_max_extensible_fields".into())),
                Some(&RawValue::Number("0".into()))
            );
        }
        let mut without_metadata = staged.objects.clone();
        for objects in without_metadata.values_mut() {
            for object in objects.values_mut() {
                object.fields.remove(&FieldName("idf_max_fields".into()));
                object
                    .fields
                    .remove(&FieldName("idf_max_extensible_fields".into()));
            }
        }
        assert_eq!(without_metadata, native.objects);
        let rules = r#"{"GlobalGeometryRules":{"GlobalGeometryRules 1":{"starting_vertex_position":"UpperLeftCorner","vertex_entry_direction":"Counterclockwise","coordinate_system":"Relative"}}}"#;
        let mut rules = parse_epjson_str_with_idf_order(
            rules,
            "GlobalGeometryRules,UpperLeftCorner,Counterclockwise,Relative;",
        )?;
        let before = rules.clone();
        assert_eq!(
            rules.ordered_instances("GlobalGeometryRules")?[0]
                .1
                .fields
                .get(&FieldName("idf_max_fields".into())),
            Some(&RawValue::Number("3".into()))
        );
        assert!(apply_idf_declaration_order(&mut rules,"GlobalGeometryRules,UpperLeftCorner,Counterclockwise,Relative;GlobalGeometryRules,UpperLeftCorner,Counterclockwise,Relative;",IDF_ORDER_TARGETS).is_err());
        assert_eq!(rules, before);
        assert!(staged.has_idf_declaration_order("BuildingSurface:Detailed"));
        Ok(())
    }

    #[test]
    fn staged_idf_overlay_retains_reverse_alphabet_declaration_order()
    -> Result<(), Box<dyn std::error::Error>> {
        let model = parse_epjson_str_with_idf_order(TWO_SPECIAL_DAYS_EPJSON, REVERSE_ALPHABET_IDF)?;
        let native_model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;

        assert!(model.has_idf_declaration_order("RunPeriodControl:SpecialDays"));
        assert_eq!(
            ordered_names(&model)?,
            vec!["Zulu Earlier Holiday", "Alpha Later Custom"]
        );
        let mut without_metadata = model.objects.clone();
        for objects in without_metadata.values_mut() {
            for object in objects.values_mut() {
                assert_eq!(
                    object.fields.remove(&FieldName("idf_max_fields".into())),
                    Some(RawValue::Number("4".into()))
                );
            }
        }
        assert_eq!(without_metadata, native_model.objects);
        assert_ne!(model, native_model);
        assert_eq!(model, model.clone());
        Ok(())
    }

    #[test]
    fn scanner_handles_bom_comments_multiple_objects_and_literal_quotes()
    -> Result<(), Box<dyn std::error::Error>> {
        let epjson = r#"{
            "Version":{"Version 1":{"version_identifier":"26.1"}},
            "RunPeriodControl:SpecialDays": {
                "\"Zulu Definition\"": {"start_date": "6/15"},
                "Alpha Definition": {"start_date": "6/15"}
            }
        }"#;
        let idf = "\u{feff}!- header\nVersion,26.1; RunPeriodControl:SpecialDays,\"zulu definition\",6/15,1,Holiday; ! trailing\nrunperiodcontrol:specialdays,ALPHA DEFINITION,6/15,1,CustomDay1;";
        let model = parse_epjson_str_with_idf_order(epjson, idf)?;

        assert_eq!(
            ordered_names(&model)?,
            vec!["\"Zulu Definition\"", "Alpha Definition"]
        );
        Ok(())
    }

    #[test]
    fn byte_scanner_ignores_non_utf8_comment_bytes() -> Result<(), Box<dyn std::error::Error>> {
        let mut idf = REVERSE_ALPHABET_IDF.as_bytes().to_vec();
        idf.extend_from_slice(b"\n! Windows-1252 comment: ");
        idf.push(0xe9);
        let mut model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;

        apply_idf_declaration_order_bytes(&mut model, &idf, IDF_ORDER_TARGETS)?;

        assert_eq!(
            ordered_names(&model)?,
            vec!["Zulu Earlier Holiday", "Alpha Later Custom"]
        );
        Ok(())
    }

    #[test]
    fn scanner_matches_energyplus_asymmetric_field_whitespace()
    -> Result<(), Box<dyn std::error::Error>> {
        let epjson = r#"{
            "RunPeriodControl:SpecialDays": {
                "Zulu Definition\n": {"start_date": "6/15"},
                "Alpha Definition": {"start_date": "6/15"}
            }
        }"#;
        let idf = "RunPeriodControl:SpecialDays,\n  Zulu Definition\n  ,6/15,1,Holiday;\nRunPeriodControl:SpecialDays,\n  Alpha Definition,6/15,1,CustomDay1;";

        let model = parse_epjson_str_with_idf_order(epjson, idf)?;

        assert_eq!(
            ordered_names(&model)?,
            vec!["Zulu Definition\n", "Alpha Definition"]
        );
        Ok(())
    }

    #[test]
    fn scanner_matches_energyplus_comment_segment_overwrite()
    -> Result<(), Box<dyn std::error::Error>> {
        let epjson = r#"{
            "RunPeriodControl:SpecialDays": {
                "Alpha Tail": {"start_date": "6/15"},
                "Beta Definition": {"start_date": "6/16"}
            }
        }"#;
        let idf = "RunPeriodControl:SpecialDays, Zulu Head ! comment\n Alpha Tail, 6/15, 1, Holiday;\nRunPeriodControl:SpecialDays, Beta Definition, 6/16, 1, CustomDay1;";

        let model = parse_epjson_str_with_idf_order(epjson, idf)?;

        assert_eq!(
            ordered_names(&model)?,
            vec!["Alpha Tail", "Beta Definition"]
        );
        Ok(())
    }

    #[test]
    fn scanner_rejects_name_set_mismatch() -> Result<(), Box<dyn std::error::Error>> {
        let mut model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;
        let idf = REVERSE_ALPHABET_IDF.replace("Alpha Later Custom", "Missing Custom");

        let error = required_order_error(
            apply_idf_declaration_order(&mut model, &idf, IDF_ORDER_TARGETS),
            "mismatched target names must fail",
        )?;

        assert!(error.to_string().contains("name mismatch"));
        assert!(!model.has_idf_declaration_order("RunPeriodControl:SpecialDays"));

        let one_object_idf = r#"
            RunPeriodControl:SpecialDays,
              Zulu Earlier Holiday,
              6/15,
              1,
              Holiday;
        "#;
        let error = required_order_error(
            apply_idf_declaration_order(&mut model, one_object_idf, IDF_ORDER_TARGETS),
            "mismatched target counts must fail",
        )?;
        assert!(error.to_string().contains("count mismatch"));
        Ok(())
    }

    #[test]
    fn scanner_rejects_duplicate_target_names() -> Result<(), Box<dyn std::error::Error>> {
        let mut model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;
        let idf = REVERSE_ALPHABET_IDF.replace("Alpha Later Custom", "Zulu Earlier Holiday");

        let error = required_order_error(
            apply_idf_declaration_order(&mut model, &idf, IDF_ORDER_TARGETS),
            "duplicate target names must fail",
        )?;

        assert!(error.to_string().contains("duplicate staged IDF"));
        Ok(())
    }

    #[test]
    fn scanner_accepts_eof_terminated_object_and_rejects_malformed_idf()
    -> Result<(), Box<dyn std::error::Error>> {
        let mut model = parse_epjson_str(TWO_SPECIAL_DAYS_EPJSON)?;
        let eof_terminated = REVERSE_ALPHABET_IDF.trim_end().trim_end_matches(';');
        apply_idf_declaration_order(&mut model, eof_terminated, IDF_ORDER_TARGETS)?;
        assert_eq!(
            ordered_names(&model)?,
            vec!["Zulu Earlier Holiday", "Alpha Later Custom"]
        );

        let error = required_order_error(
            apply_idf_declaration_order(&mut model, ",", IDF_ORDER_TARGETS),
            "unexpected comma must fail",
        )?;
        assert!(error.to_string().contains("unexpected comma"));
        Ok(())
    }
}

#[cfg(test)]
mod unnamed_report_order_tests {
    use crate::{
        FieldName, IDF_ORDER_TARGETS, RawModel, RawValue, apply_idf_declaration_order,
        parse_epjson_str, parse_epjson_str_with_idf_order,
    };

    #[test]
    fn twelve_unnamed_reports_keep_actual_idf_count_order_and_field_counts()
    -> Result<(), Box<dyn std::error::Error>> {
        let objects = (1..=12)
            .map(|index| {
                format!("\"Output:Constructions {index}\":{{\"details_type_1\":\"Materials\"}}")
            })
            .collect::<Vec<_>>()
            .join(",");
        let epjson = format!("{{\"Output:Constructions\":{{{objects}}}}}");
        let idf = "Output:Constructions, Materials;".repeat(12);
        assert!(RawModel::default().is_epjson());
        assert!(RawModel::new(None, Default::default()).is_epjson());
        let mut plain = parse_epjson_str(&epjson)?;
        assert!(plain.is_epjson());
        let before_failure = plain.clone();
        assert!(apply_idf_declaration_order(&mut plain, "", IDF_ORDER_TARGETS).is_err());
        assert_eq!(plain, before_failure);
        let mut raw = parse_epjson_str_with_idf_order(&epjson, &idf)?;
        assert!(!raw.is_epjson());
        let before_failure = raw.clone();
        assert!(apply_idf_declaration_order(&mut raw, "", IDF_ORDER_TARGETS).is_err());
        assert_eq!(raw, before_failure);
        let rows = raw.ordered_instances("Output:Constructions")?;
        assert_eq!(rows.len(), 12);
        for (index, (name, object)) in rows.iter().enumerate() {
            assert_eq!(name.0, format!("Output:Constructions {}", index + 1));
            assert_eq!(
                object.fields.get(&FieldName("idf_max_fields".to_string())),
                Some(&RawValue::Number("1".to_string()))
            );
        }
        // No authored Name exists: the converter creates per-family keys.
        // Twelve records prove physical order instead of 1,10,11,12,2 lexical order.
        let objects = (1..=12)
            .map(|index| {
                format!(
                    "\"Output:Schedules {index}\":{{\"key_field\":\"{}\"}}",
                    if index % 2 == 0 { "Timestep" } else { "Hourly" }
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        let json = format!("{{\"Output:Schedules\":{{{objects}}}}}");
        let idf = (1..=12)
            .map(|index| {
                format!(
                    "Output:Schedules,{};",
                    if index % 2 == 0 { "Timestep" } else { "Hourly" }
                )
            })
            .collect::<String>();
        let ordered = parse_epjson_str_with_idf_order(&json, &idf)?;
        let direct = parse_epjson_str(&json)?;
        let rows = ordered.ordered_instances("Output:Schedules")?;
        assert_eq!(rows.len(), 12);
        for (index, (name, object)) in rows.iter().enumerate() {
            assert_eq!(name.0, format!("Output:Schedules {}", index + 1));
            assert_eq!(
                object.fields.get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number("1".into()))
            );
            assert_eq!(
                object.fields.get(&FieldName("key_field".into())),
                Some(&RawValue::String(
                    if index % 2 == 0 { "Hourly" } else { "Timestep" }.into()
                ))
            );
        }
        assert!(!direct.has_idf_declaration_order("Output:Schedules"));
        for (idf, count) in [
            ("Output:Schedules,Hourly", "0"),
            ("Output:Schedules,Hourly;", "1"),
            ("Output:Schedules,Hourly,", "1"),
        ] {
            let raw = parse_epjson_str_with_idf_order(
                r#"{"Output:Schedules":{"Output:Schedules 1":{"key_field":"Hourly"}}}"#,
                idf,
            )?;
            let rows = raw.ordered_instances("Output:Schedules")?;
            assert_eq!(rows[0].0.0, "Output:Schedules 1");
            assert_eq!(
                rows[0].1.fields.get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number(count.into()))
            );
            assert_eq!(
                rows[0].1.fields.get(&FieldName("key_field".into())),
                Some(&RawValue::String("Hourly".into()))
            );
        }
        // The same unnamed Source parser counts blank positional fields too.
        // Named materials use the same strict source overlay, not lexical order.
        for (family, idf, expected) in [
            (
                "Material",
                "Material,Z,Rough,0.1,1,100,1000;Material,A,Smooth,0.2,1,100,1000;",
                None,
            ),
            (
                "Material:NoMass",
                "Material:NoMass,Z,Rough,1;Material:NoMass,A,Smooth,2,,,;",
                Some([3, 6]),
            ),
            (
                "Material:AirGap",
                "Material:AirGap,Z,0.15;Material:AirGap,A;",
                Some([2, 2]),
            ),
            (
                "Material:InfraredTransparent",
                "Material:InfraredTransparent,Z;Material:InfraredTransparent,A;",
                Some([1, 1]),
            ),
            (
                "ZoneList",
                "ZoneList,Z,Zone;ZoneList,A,Zone,Other;",
                Some([2, 3]),
            ),
            (
                "Zone",
                "Zone,Z;Zone,A,0,0,0,0,1,1,autocalculate,autocalculate,autocalculate,,,Yes;",
                Some([1, 13]),
            ),
            (
                "Building",
                "Building,Z,0,Suburbs,0.04,0.4,MinimalShadowing,25,6;Building,A,0,Suburbs,0.04,0.4,MinimalShadowing,25,6;",
                Some([8, 8]),
            ),
            (
                "Construction",
                "Construction,Z,Layer;Construction,A,Layer1,Layer2;",
                Some([2, 3]),
            ),
            (
                "OtherEquipment",
                "OtherEquipment,Z,None,Zone,Schedule,EquipmentLevel,-3;OtherEquipment,A,None,Zone,Schedule,EquipmentLevel,7,,,,,,,MixedCase;",
                Some([11, 13]),
            ),
            (
                "RunPeriod",
                "RunPeriod,Z,1,1,,1,1;RunPeriod,A,1,1,2013,1,2,2013,Tuesday,,,No,Yes,Yes,,Hour24;",
                Some([7, 15]),
            ),
            (
                "SizingPeriod:DesignDay",
                "SizingPeriod:DesignDay,Z,12;SizingPeriod:DesignDay,A,7,,SummerDesignDay;",
                Some([2, 4]),
            ),
            (
                "ZoneHVAC:IdealLoadsAirSystem",
                "ZoneHVAC:IdealLoadsAirSystem,Z;ZoneHVAC:IdealLoadsAirSystem,A,Availability,Supply,,,50,13,0.0156,0.0077,NoLimit,,,NoLimit,,,,,None,0.7,None,,,None,NoEconomizer,None,0.7,0.65,,,DistrictHeatingWater,,DistrictCooling;",
                Some([27, 32]),
            ),
        ] {
            let epjson = format!("{{\"{family}\":{{\"A\":{{}},\"Z\":{{}}}}}}");
            let raw = parse_epjson_str_with_idf_order(&epjson, idf)?;
            let rows = raw.ordered_instances(family)?;
            assert_eq!(
                rows.iter()
                    .map(|(name, _)| name.0.as_str())
                    .collect::<Vec<_>>(),
                ["Z", "A"]
            );
            for (index, (_, object)) in rows.iter().enumerate() {
                let count = object.fields.get(&FieldName("idf_max_fields".into()));
                if let Some(expected) = expected {
                    assert_eq!(count, Some(&RawValue::Number(expected[index].to_string())));
                    if family == "ZoneList" {
                        assert_eq!(
                            object
                                .fields
                                .get(&FieldName("idf_max_extensible_fields".into())),
                            Some(&RawValue::Number((expected[index] - 1).to_string()))
                        );
                    }
                } else {
                    assert!(count.is_none());
                }
            }
        }
        // Native END returns before attaching the ZoneList local zones array;
        // no admitted positive list can use an EOF metadata approximation.
        assert!(
            parse_epjson_str_with_idf_order(r#"{"ZoneList":{"L":{}}}"#, "ZoneList,L,Zone").is_err()
        );
        for (family, fields, expected) in [
            ("Version", "26.1", 1),
            (
                "HeatBalanceAlgorithm",
                "ConductionTransferFunction,200,0.1,1000",
                4,
            ),
            ("Timestep", "4", 1),
            ("SimulationControl", "No,No,No,No,Yes,,1", 7),
            ("SurfaceConvectionAlgorithm:Inside", "TARP", 1),
            ("SurfaceConvectionAlgorithm:Outside", "DOE-2", 1),
            (
                "ZoneAirHeatBalanceAlgorithm",
                "ThirdOrderBackwardDifference,No,No",
                3,
            ),
        ] {
            let epjson = format!("{{\"{family}\":{{\"{family} 1\":{{}}}}}}");
            let raw = parse_epjson_str_with_idf_order(&epjson, &format!("{family},{fields};"))?;
            assert!(!raw.is_epjson());
            let rows = raw.ordered_instances(family)?;
            assert_eq!(rows.len(), 1);
            assert_eq!(rows[0].0.0, format!("{family} 1"));
            assert_eq!(
                rows[0].1.fields.get(&FieldName("idf_max_fields".into())),
                Some(&RawValue::Number(expected.to_string()))
            );
        }
        // Building is named: preserve its actual source name and eight fields.
        let raw = parse_epjson_str_with_idf_order(
            r#"{"Building":{"Actual Building":{"north_axis":0}}}"#,
            "Building,Actual Building,0,Suburbs,0.04,0.4,MinimalShadowing,25,6;",
        )?;
        let rows = raw.ordered_instances("Building")?;
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].0.0, "Actual Building");
        assert_eq!(
            rows[0].1.fields.get(&FieldName("idf_max_fields".into())),
            Some(&RawValue::Number("8".into()))
        );
        // EOF-only Native counters do not have the semicolon min-field pad.
        // The bounded metadata route must fail before publishing any overlay.
        let mut eof_run_period = parse_epjson_str(r#"{"RunPeriod":{"A":{}}}"#)?;
        for idf in ["RunPeriod,A,1,1,,1,1", "RunPeriod,A,1,1,,1,1,"] {
            let before_eof = eof_run_period.clone();
            let error = apply_idf_declaration_order(&mut eof_run_period, idf, IDF_ORDER_TARGETS)
                .err()
                .ok_or("unterminated RunPeriod must remain unavailable")?;
            assert!(error.to_string().contains("terminating semicolon"));
            assert_eq!(eof_run_period, before_eof);
        }
        let mut eof_design_day = parse_epjson_str(r#"{"SizingPeriod:DesignDay":{"A":{}}}"#)?;
        for idf in [
            "sizingperiod:designday,a,7,21",
            "SizingPeriod:DesignDay,A,7,21,",
        ] {
            let before_eof = eof_design_day.clone();
            let error = apply_idf_declaration_order(&mut eof_design_day, idf, IDF_ORDER_TARGETS)
                .err()
                .ok_or("unterminated DesignDay must remain unavailable")?;
            assert!(error.to_string().contains("terminating semicolon"));
            assert_eq!(eof_design_day, before_eof);
        }
        let mut eof_ideal_loads = parse_epjson_str(r#"{"ZoneHVAC:IdealLoadsAirSystem":{"A":{}}}"#)?;
        for idf in [
            "zonehvac:idealloadsairsystem,a",
            "ZoneHVAC:IdealLoadsAirSystem,A,",
        ] {
            let before_eof = eof_ideal_loads.clone();
            let error = apply_idf_declaration_order(&mut eof_ideal_loads, idf, IDF_ORDER_TARGETS)
                .err()
                .ok_or("unterminated IdealLoads must remain unavailable")?;
            assert!(error.to_string().contains("terminating semicolon"));
            assert_eq!(eof_ideal_loads, before_eof);
        }
        // People is named, with its genuine legacy min_fields10 and independent
        // supplied count beyond that minimum; declaration order is not key sort.
        let people_json = r#"{"People":{"A":{"zone_or_zonelist_or_space_or_spacelist_name":"zone","number_of_people_schedule_name":"on","number_of_people":1,"fraction_radiant":0.3,"sensible_heat_fraction":0.6,"activity_level_schedule_name":"activity"},"Z":{"zone_or_zonelist_or_space_or_spacelist_name":"zone","number_of_people_schedule_name":"on","number_of_people":2,"fraction_radiant":0.3,"sensible_heat_fraction":0.6,"activity_level_schedule_name":"activity","enable_ashrae_55_comfort_warnings":"Yes"}}}"#;
        let ordered = parse_epjson_str_with_idf_order(
            people_json,
            "People,Z,zone,on,People,2,,,0.3,0.6,activity,,Yes;People,A,zone,on,People,1,,,0.3,0.6,activity;",
        )?;
        let rows = ordered.ordered_instances("People")?;
        assert_eq!(
            rows.iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["Z", "A"]
        );
        for ((_, object), count) in rows.into_iter().zip([12, 10]) {
            assert_eq!(
                object.fields[&FieldName("idf_max_fields".into())],
                RawValue::Number(count.to_string())
            );
        }
        // Actual EOF retains the parsed Activity alpha while its final field
        // is outside NumAlphas; only a semicolon performs min_fields padding.
        let people_eof = r#"{"People":{"A":{"zone_or_zonelist_or_space_or_spacelist_name":"zone","number_of_people_schedule_name":"on","number_of_people":1,"fraction_radiant":0.3,"sensible_heat_fraction":0.6,"activity_level_schedule_name":"activity"}}}"#;
        for (ending, count) in [("", 9), (",", 10), (";", 10)] {
            let idf = format!("People,A,zone,on,People,1,,,0.3,0.6,activity{ending}");
            let actual = parse_epjson_str_with_idf_order(people_eof, &idf)?;
            let rows = actual.ordered_instances("People")?;
            assert_eq!(
                rows[0].1.fields[&FieldName("idf_max_fields".into())],
                RawValue::Number(count.to_string())
            );
            assert_eq!(
                rows[0].1.fields[&FieldName("activity_level_schedule_name".into())],
                RawValue::String("activity".into())
            );
        }
        // SetPointManager's direct-JSON getter sorts original global IDF
        // numbers. Unrelated declarations count; comments/empty records do not.
        // Native conversion removes these metadata fields, so the actual
        // staged IDF supplies them after strict one-to-one reconciliation.
        let scheduled_json = r#"{"SetpointManager:Scheduled":{
            "Alpha Manager":{"control_variable":"Temperature","schedule_name":"On","setpoint_node_or_nodelist_name":"Supply"},
            "Zulu Manager":{"control_variable":"Temperature","schedule_name":"On","setpoint_node_or_nodelist_name":"Supply"}
        },"NodeList":{
            "Unrelated First":{"nodes":[{"node_name":"Zone"}]},
            "Unrelated Middle":{"nodes":[{"node_name":"Other"}]}
        }}"#;
        let scheduled_idf = "NodeList,Unrelated First,Zone;\n\
            ; ! Empty records do not consume a Native object ordinal.\n\
            SETPOINTMANAGER:SCHEDULED, ZULU MANAGER ,Temperature,On,Supply;\n\
            NodeList,Unrelated Middle,Other;\n\
            SetpointManager:Scheduled,alpha manager,Temperature,On,Supply;";
        let direct = parse_epjson_str(scheduled_json)?;
        let staged = parse_epjson_str_with_idf_order(scheduled_json, scheduled_idf)?;
        assert!(direct.is_epjson());
        assert!(!staged.is_epjson());
        assert!(!direct.has_idf_declaration_order("SetpointManager:Scheduled"));
        let rows = staged.ordered_instances("SetpointManager:Scheduled")?;
        assert_eq!(
            rows.iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["Zulu Manager", "Alpha Manager"]
        );
        for ((_, object), ordinal) in rows.into_iter().zip([2, 4]) {
            for (field, value) in [
                ("idf_order", ordinal),
                ("idf_max_fields", 4),
                ("idf_max_extensible_fields", 0),
            ] {
                assert_eq!(
                    object.fields.get(&FieldName(field.into())),
                    Some(&RawValue::Number(value.to_string()))
                );
            }
        }
        let mut without_metadata = staged.objects.clone();
        for objects in without_metadata.values_mut() {
            for object in objects.values_mut() {
                for field in ["idf_order", "idf_max_fields", "idf_max_extensible_fields"] {
                    object.fields.remove(&FieldName(field.into()));
                }
            }
        }
        assert_eq!(without_metadata, direct.objects);
        assert_eq!(
            direct
                .ordered_instances("SetpointManager:Scheduled")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["Alpha Manager", "Zulu Manager"]
        );
        for replacement in ["ZULU MANAGER", "Missing Manager", ""] {
            let mut bad = direct.clone();
            let before = bad.clone();
            let idf = scheduled_idf.replace("alpha manager", replacement);
            assert!(apply_idf_declaration_order(&mut bad, &idf, IDF_ORDER_TARGETS).is_err());
            assert_eq!(bad, before);
        }
        // Scheduled has no schema min_fields. Native END leaves its final
        // parsed Node value present without advancing F; a trailing comma
        // advances F before END, while a semicolon ends the regular record.
        let single = r#"{"SetpointManager:Scheduled":{"SP":{"control_variable":"Temperature","schedule_name":"On","setpoint_node_or_nodelist_name":"Supply"}}}"#;
        for (ending, count) in [("", 3), (",", 4), (";", 4)] {
            let idf = format!("SetpointManager:Scheduled,SP,Temperature,On,Supply{ending}");
            let actual = parse_epjson_str_with_idf_order(single, &idf)?;
            let rows = actual.ordered_instances("SetpointManager:Scheduled")?;
            for (field, value) in [
                ("idf_order", 1),
                ("idf_max_fields", count),
                ("idf_max_extensible_fields", 0),
            ] {
                assert_eq!(
                    rows[0].1.fields.get(&FieldName(field.into())),
                    Some(&RawValue::Number(value.to_string()))
                );
            }
            assert_eq!(
                rows[0]
                    .1
                    .fields
                    .get(&FieldName("setpoint_node_or_nodelist_name".into())),
                Some(&RawValue::String("Supply".into()))
            );
        }
        // NodeList uses legacy GetObjectItem order, rather than the global
        // idf_order scalar consumed by Scheduled's direct-JSON getter. Keep
        // its actual member arrays intact; no reconstructed field counts.
        let lists_json = r#"{"NodeList":{
            "Alpha List":{"nodes":[{"node_name":"First"}]},
            "Zulu List":{"nodes":[{"node_name":"Second"},{"node_name":"First"}]}
        }}"#;
        let lists_idf = "NodeList,ZULU LIST,Second,First;\n\
            ; ! Comments and empty records do not change list order.\n\
            NodeList,alpha list,First;";
        let direct = parse_epjson_str(lists_json)?;
        let staged = parse_epjson_str_with_idf_order(lists_json, lists_idf)?;
        assert!(direct.is_epjson());
        assert!(!direct.has_idf_declaration_order("NodeList"));
        assert!(!staged.is_epjson());
        assert!(staged.has_idf_declaration_order("NodeList"));
        assert_eq!(staged.objects, direct.objects);
        assert_eq!(
            staged
                .ordered_instances("NodeList")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["Zulu List", "Alpha List"]
        );
        assert_eq!(
            direct
                .ordered_instances("NodeList")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["Alpha List", "Zulu List"]
        );
        for replacement in ["ZULU LIST", "Missing List", ""] {
            let mut bad = direct.clone();
            let before = bad.clone();
            let idf = lists_idf.replace("alpha list", replacement);
            assert!(apply_idf_declaration_order(&mut bad, &idf, IDF_ORDER_TARGETS).is_err());
            assert_eq!(bad, before);
        }
        let mut bad = direct.clone();
        let before = bad.clone();
        assert!(
            apply_idf_declaration_order(
                &mut bad,
                "NodeList,Zulu List,Second,First;",
                IDF_ORDER_TARGETS
            )
            .is_err()
        );
        assert_eq!(bad, before);
        // Native decode preserves both exact Name keys. Their actual IDF
        // declaration order may be the reverse of the JSON object key order.
        let collision_json = r#"{"NodeList":{
            "Case":{"nodes":[{"node_name":"FIRST"}]},
            "case":{"nodes":[{"node_name":"SECOND"}]}
        }}"#;
        let collision_idf = "NodeList,case,SECOND;NodeList,Case,FIRST;";
        let direct_collision = parse_epjson_str(collision_json)?;
        let ordered = parse_epjson_str_with_idf_order(collision_json, collision_idf)?;
        assert_eq!(ordered.objects, direct_collision.objects);
        assert_eq!(
            ordered
                .ordered_instances("NodeList")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["case", "Case"]
        );
        assert_eq!(
            ordered
                .source_ordered_instances("NodeList")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["case", "Case"]
        );
        assert_eq!(
            ordered
                .source_json_instances("NodeList")?
                .iter()
                .map(|(name, _)| name.0.as_str())
                .collect::<Vec<_>>(),
            ["Case", "case"]
        );
        assert!(ordered.has_idf_declaration_order("NodeList") && !ordered.is_epjson());
        for (idf, expected) in [
            (
                "NodeList,case,SECOND;NodeList,case,SECOND;",
                "duplicate staged IDF",
            ),
            ("NodeList,CASE,FIRST;NodeList,case,SECOND;", "ambiguous"),
            (
                "NodeList,Missing,FIRST;NodeList,case,SECOND;",
                "name mismatch",
            ),
            ("NodeList,case,SECOND;", "count mismatch"),
        ] {
            let mut actual = direct_collision.clone();
            let before = actual.clone();
            let error = apply_idf_declaration_order(&mut actual, idf, IDF_ORDER_TARGETS)
                .err()
                .ok_or_else(|| {
                    std::io::Error::other(
                        "invalid NodeList name associations must remain transactional",
                    )
                })?;
            assert!(error.to_string().contains(expected), "{error}");
            assert_eq!(actual, before);
        }
        // A later target failure must not install the earlier valid list overlay.
        let mut later_failure = parse_epjson_str(
            r#"{
            "NodeList":{"Case":{"nodes":[{"node_name":"FIRST"}]},"case":{"nodes":[{"node_name":"SECOND"}]}},
            "Schedule:Constant":{"Only":{"hourly_value":1}}
        }"#,
        )?;
        let before = later_failure.clone();
        let idf = format!("{collision_idf}Schedule:Constant,Missing,,1;");
        assert!(apply_idf_declaration_order(&mut later_failure, &idf, IDF_ORDER_TARGETS).is_err());
        assert_eq!(later_failure, before);
        Ok(())
    }
}
