//! Raw material operands; opaque properties stay typed, GlassSimple has actual runtime rows.
use super::super::selected_project_data::{ProjectInputObject, ProjectInputValue};
use super::{MaterialDataOwners, unavailable};
use crate::error::RuntimeError;

/// Literal selected Material/Material:NoMass/Material:AirGap fields and input order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectedMaterialInput {
    /// Actual loader format; never inferred from one family's metadata.
    pub source_is_epjson: bool,
    /// Regular direct-JSON iteration order returned by getIDFOrderedKeys.
    pub regular: Vec<ProjectInputObject>,
    /// Actual getObjectItem order for Material:NoMass.
    pub no_mass: Vec<ProjectInputObject>,
    /// Actual getObjectItem order for Material:AirGap.
    pub air_gaps: Vec<ProjectInputObject>,
    /// Actual name-only getObjectItem order for Material:InfraredTransparent.
    pub infrared_transparent: Vec<ProjectInputObject>,
    /// Actual ordered Source epJSON SimpleGlazing getter operands.
    pub simple_glazing: Vec<ProjectInputObject>,
    /// Actual ordered direct-epJSON ordinary MaterialGlass operands.
    pub glazing: Vec<ProjectInputObject>,
    /// Actual ordered alternative MaterialGlass2A9N inputs.
    pub refraction: Vec<ProjectInputObject>,
    /// Actual ordered spectral-data fixed fields and all extensible rows.
    pub spectral_data: Vec<SourceGlazingSpectralInput>,
    /// Actual ordered Source epJSON named Gas getter operands.
    pub window_gas: Vec<ProjectInputObject>,
    /// Genuine ordered GasMixture getObjectItem operands, not typed properties.
    pub window_gas_mixture: Vec<ProjectInputObject>,
    /// Genuine ordered ordinary MaterialShade getObjectItem operands.
    pub window_shade: Vec<ProjectInputObject>,
    /// Genuine ordered MaterialShadeEQL1A11N operands; NumNums may be5..11.
    pub window_shade_equivalent_layer: Vec<ProjectInputObject>,
    /// Genuine ordered MaterialDrapeEQL1A13N operands; legal rows return5..13.
    pub window_drape_equivalent_layer: Vec<ProjectInputObject>,
    /// Genuine ordered ordinary MaterialScreen2A12N operands.
    pub window_screen: Vec<ProjectInputObject>,
    /// Genuine ordered ordinary MaterialBlind2A27N/min29 operands.
    pub window_blind: Vec<ProjectInputObject>,
}

/// Genuine scalar projection; empty extension objects retain their array position.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceGlazingSpectralInput {
    /// Actual object identity and declared fixed numeric fields.
    pub object: ProjectInputObject,
    /// Every real extension row;an empty map still occupies four numeric slots.
    pub extensions: Vec<std::collections::BTreeMap<String, ProjectInputValue>>,
}

pub(super) fn alpha<'a>(
    object: &'a ProjectInputObject,
    field: &str,
) -> Result<&'a str, RuntimeError> {
    match object.fields.get(field) {
        Some(ProjectInputValue::String(value)) if !value.is_empty() => Ok(value),
        _ => Err(unavailable("required admitted material alpha")),
    }
}

pub(super) fn numeric(
    object: &ProjectInputObject,
    field: &str,
    default: Option<f64>,
) -> Result<f64, RuntimeError> {
    match object.fields.get(field) {
        Some(ProjectInputValue::Number(value)) if value.is_finite() => Ok(*value),
        None | Some(ProjectInputValue::String(_))
            if object.fields.get(field).is_none_or(
                |value| matches!(value, ProjectInputValue::String(text) if text.is_empty()),
            ) =>
        {
            default.ok_or_else(|| unavailable("required admitted material numeric"))
        }
        _ => Err(unavailable("selected material JSON scalar")),
    }
}

impl SelectedMaterialInput {
    pub(super) fn get_spectral_data_item(
        &self,
        row: &SourceGlazingSpectralInput,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        let family = "MaterialProperty:GlazingSpectralData";
        if !self.source_is_epjson || !owners.input.has_object(family, &row.object.object_name) {
            return Err(unavailable("spectral data actual raw/IP identity"));
        }
        const FIXED: [&str; 16] = [
            "wavelength_1",
            "transmittance_1",
            "front_reflectance_1",
            "back_reflectance_1",
            "wavelength_2",
            "transmittance_2",
            "front_reflectance_2",
            "back_reflectance_2",
            "wavelength_3",
            "transmittance_3",
            "front_reflectance_3",
            "back_reflectance_3",
            "wavelength_4",
            "transmittance_4",
            "front_reflectance_4",
            "back_reflectance_4",
        ];
        const EXT: [&str; 4] = [
            "wavelength",
            "transmittance",
            "front_reflectance",
            "back_reflectance",
        ];
        let allocated = 16_usize
            .checked_add(
                row.extensions
                    .len()
                    .checked_mul(4)
                    .ok_or_else(|| unavailable("spectral extension count"))?,
            )
            .ok_or_else(|| unavailable("spectral argument count"))?;
        let s = &mut owners.shortcut;
        if s.alpha_args.is_empty()
            || s.alpha_field_names.is_empty()
            || s.alpha_field_blanks.is_empty()
            || s.numeric_args.len() < allocated
            || s.numeric_field_names.len() < allocated
            || s.numeric_field_blanks.len() < allocated
        {
            return Err(unavailable("actual complete spectral1A16N+4E allocation"));
        }
        s.alpha_args.fill(String::new());
        s.alpha_field_names.fill(String::new());
        s.alpha_field_blanks.fill(true);
        s.numeric_args.fill(0.0);
        s.numeric_field_names.fill(String::new());
        s.numeric_field_blanks.fill(true);
        owners
            .input
            .mark_object_as_used(family, &row.object.object_name);
        s.alpha_args[0] = row.object.object_name.to_ascii_uppercase();
        s.alpha_field_names[0] = "name".into();
        s.alpha_field_blanks[0] = row.object.object_name.is_empty();
        let mut maximum = 0;
        for (index, field) in FIXED.iter().enumerate() {
            s.numeric_field_names[index] = (*field).into();
            if let Some(value) = row.object.fields.get(*field) {
                let ProjectInputValue::Number(value) = value else {
                    return Err(unavailable("admitted spectral numeric"));
                };
                s.numeric_args[index] = *value;
                s.numeric_field_blanks[index] = false;
                maximum = index + 1;
            }
        }
        if !row.extensions.is_empty() {
            maximum = allocated;
        }
        for (index, fields) in row.extensions.iter().enumerate() {
            for (part, field) in EXT.iter().enumerate() {
                let at = 16 + index * 4 + part;
                s.numeric_field_names[at] = (*field).into();
                if let Some(value) = fields.get(*field) {
                    let ProjectInputValue::Number(value) = value else {
                        return Err(unavailable("admitted spectral extension numeric"));
                    };
                    s.numeric_args[at] = *value;
                    s.numeric_field_blanks[at] = false;
                }
            }
        }
        Ok((1, maximum))
    }

    pub(super) fn get_refraction_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        if !self.source_is_epjson
            || !owners.input.has_object(
                "WindowMaterial:Glazing:RefractionExtinctionMethod",
                &object.object_name,
            )
        {
            return Err(unavailable(
                "Refraction actual direct-epJSON raw/IP identity",
            ));
        }
        const FIELDS: [(&str, bool); 11] = [
            ("name", true),
            ("thickness", false),
            ("solar_index_of_refraction", false),
            ("solar_extinction_coefficient", false),
            ("visible_index_of_refraction", false),
            ("visible_extinction_coefficient", false),
            ("infrared_transmittance_at_normal_incidence", false),
            ("infrared_hemispherical_emissivity", false),
            ("conductivity", false),
            (
                "dirt_correction_factor_for_solar_and_visible_transmittance",
                false,
            ),
            ("solar_diffusing", true),
        ];
        let s = &mut owners.shortcut;
        if s.alpha_args.len() < 2
            || s.alpha_field_names.len() < 2
            || s.alpha_field_blanks.len() < 2
            || s.numeric_args.len() < 9
            || s.numeric_field_names.len() < 9
            || s.numeric_field_blanks.len() < 9
        {
            return Err(unavailable("actual shared Refraction2A9N allocation"));
        }
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        owners.input.mark_object_as_used(
            "WindowMaterial:Glazing:RefractionExtinctionMethod",
            &object.object_name,
        );
        // No min_fields in Native schema: required operands give1A5N, not2A9N.
        let maximum = FIELDS
            .iter()
            .enumerate()
            .filter(|(_, (field, _))| object.fields.contains_key(*field))
            .map(|(index, _)| index + 1)
            .max()
            .unwrap_or(0);
        let (mut a, mut n, mut alphas, mut numerics) = (0, 0, 0, 0);
        for (position, (field, is_alpha)) in FIELDS.iter().enumerate() {
            if *is_alpha {
                s.alpha_field_names[a] = (*field).into();
                if position == 0 {
                    s.alpha_args[a] = object.object_name.to_ascii_uppercase();
                    s.alpha_field_blanks[a] = object.object_name.is_empty();
                } else {
                    match object.fields.get(*field) {
                        Some(ProjectInputValue::String(value)) if !value.is_empty() => {
                            s.alpha_args[a] = value.to_ascii_uppercase();
                            s.alpha_field_blanks[a] = false;
                        }
                        None | Some(ProjectInputValue::String(_)) => s.alpha_args[a] = "NO".into(),
                        _ => return Err(unavailable("admitted Refraction alpha scalar")),
                    }
                }
                a += 1;
                if position < maximum {
                    alphas = a;
                }
            } else {
                s.numeric_field_names[n] = (*field).into();
                s.numeric_args[n] = match *field {
                    "infrared_transmittance_at_normal_incidence" => 0.0,
                    "infrared_hemispherical_emissivity" => 0.84,
                    "conductivity" => 0.9,
                    "dirt_correction_factor_for_solar_and_visible_transmittance" => 1.0,
                    _ => 0.0,
                };
                match object.fields.get(*field) {
                    Some(ProjectInputValue::Number(value)) if value.is_finite() => {
                        s.numeric_args[n] = *value;
                        s.numeric_field_blanks[n] = false;
                    }
                    None => {}
                    _ => return Err(unavailable("admitted Refraction numeric scalar")),
                }
                n += 1;
                if position < maximum {
                    numerics = n;
                }
            }
        }
        Ok((alphas, numerics))
    }

    pub(super) fn get_glazing_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        if !self.source_is_epjson
            || !owners
                .input
                .has_object("WindowMaterial:Glazing", &object.object_name)
        {
            return Err(unavailable("Glazing owned direct-epJSON raw/IP identity"));
        }
        const FIELDS: [(&str, bool); 21] = [
            ("name", true),
            ("optical_data_type", true),
            ("window_glass_spectral_data_set_name", true),
            ("thickness", false),
            ("solar_transmittance_at_normal_incidence", false),
            ("front_side_solar_reflectance_at_normal_incidence", false),
            ("back_side_solar_reflectance_at_normal_incidence", false),
            ("visible_transmittance_at_normal_incidence", false),
            ("front_side_visible_reflectance_at_normal_incidence", false),
            ("back_side_visible_reflectance_at_normal_incidence", false),
            ("infrared_transmittance_at_normal_incidence", false),
            ("front_side_infrared_hemispherical_emissivity", false),
            ("back_side_infrared_hemispherical_emissivity", false),
            ("conductivity", false),
            (
                "dirt_correction_factor_for_solar_and_visible_transmittance",
                false,
            ),
            ("solar_diffusing", true),
            ("young_s_modulus", false),
            ("poisson_s_ratio", false),
            (
                "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name",
                true,
            ),
            (
                "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name",
                true,
            ),
            (
                "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name",
                true,
            ),
        ];
        let s = &mut owners.shortcut;
        if s.alpha_args.len() < 7
            || s.alpha_field_blanks.len() < 7
            || s.alpha_field_names.len() < 7
            || s.numeric_args.len() < 14
            || s.numeric_field_blanks.len() < 14
            || s.numeric_field_names.len() < 14
        {
            return Err(unavailable(
                "actual complete Glazing7alpha14numeric allocation",
            ));
        }
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("WindowMaterial:Glazing", &object.object_name);
        let maximum = FIELDS
            .iter()
            .enumerate()
            .filter(|(_, (field, _))| object.fields.contains_key(*field))
            .map(|(index, _)| index + 1)
            .max()
            .unwrap_or(0)
            .max(14);
        let (mut a, mut n, mut alphas, mut numerics) = (0, 0, 0, 0);
        for (position, (field, is_alpha)) in FIELDS.iter().enumerate() {
            if *is_alpha {
                s.alpha_field_names[a] = (*field).into();
                if position == 0 {
                    s.alpha_args[a] = object.object_name.to_ascii_uppercase();
                    s.alpha_field_blanks[a] = object.object_name.is_empty();
                } else {
                    let default = if *field == "solar_diffusing" {
                        "No"
                    } else {
                        ""
                    };
                    match object.fields.get(*field) {
                        Some(ProjectInputValue::String(value)) if !value.is_empty() => {
                            s.alpha_args[a] = value.to_ascii_uppercase();
                            s.alpha_field_blanks[a] = false;
                        }
                        None | Some(ProjectInputValue::String(_)) => {
                            s.alpha_args[a] = default.to_ascii_uppercase()
                        }
                        _ => return Err(unavailable("admitted Glazing alpha scalar")),
                    }
                }
                a += 1;
                if position < maximum {
                    alphas = a;
                }
            } else {
                s.numeric_field_names[n] = (*field).into();
                s.numeric_args[n] = match *field {
                    "infrared_transmittance_at_normal_incidence" => 0.0,
                    "front_side_infrared_hemispherical_emissivity" => 0.84,
                    "back_side_infrared_hemispherical_emissivity" => 0.84,
                    "conductivity" => 0.9,
                    "dirt_correction_factor_for_solar_and_visible_transmittance" => 1.0,
                    "young_s_modulus" => 72_000_000_000.0,
                    "poisson_s_ratio" => 0.22,
                    _ => 0.0,
                };
                match object.fields.get(*field) {
                    Some(ProjectInputValue::Number(value)) if value.is_finite() => {
                        s.numeric_args[n] = *value;
                        s.numeric_field_blanks[n] = false;
                    }
                    None => {}
                    _ => return Err(unavailable("admitted Glazing numeric scalar")),
                }
                n += 1;
                if position < maximum {
                    numerics = n;
                }
            }
        }
        Ok((alphas, numerics))
    }

    pub(super) fn get_window_blind_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        if !self.source_is_epjson
            || !owners
                .input
                .has_object("WindowMaterial:Blind", &object.object_name)
        {
            return Err(unavailable("Blind owned direct-epJSON raw/IP identity"));
        }
        const FIELDS: [(&str, f64); 27] = [
            ("slat_width", 0.0),
            ("slat_separation", 0.0),
            ("slat_thickness", 0.00025),
            ("slat_angle", 45.0),
            ("slat_conductivity", 221.0),
            ("slat_beam_solar_transmittance", 0.0),
            ("front_side_slat_beam_solar_reflectance", 0.0),
            ("back_side_slat_beam_solar_reflectance", 0.0),
            ("slat_diffuse_solar_transmittance", 0.0),
            ("front_side_slat_diffuse_solar_reflectance", 0.0),
            ("back_side_slat_diffuse_solar_reflectance", 0.0),
            ("slat_beam_visible_transmittance", 0.0),
            ("front_side_slat_beam_visible_reflectance", 0.0),
            ("back_side_slat_beam_visible_reflectance", 0.0),
            ("slat_diffuse_visible_transmittance", 0.0),
            ("front_side_slat_diffuse_visible_reflectance", 0.0),
            ("back_side_slat_diffuse_visible_reflectance", 0.0),
            ("slat_infrared_hemispherical_transmittance", 0.0),
            ("front_side_slat_infrared_hemispherical_emissivity", 0.9),
            ("back_side_slat_infrared_hemispherical_emissivity", 0.9),
            ("blind_to_glass_distance", 0.05),
            ("blind_top_opening_multiplier", 0.5),
            ("blind_bottom_opening_multiplier", 0.0),
            ("blind_left_side_opening_multiplier", 0.5),
            ("blind_right_side_opening_multiplier", 0.5),
            ("minimum_slat_angle", 0.0),
            ("maximum_slat_angle", 180.0),
        ];
        let s = &mut owners.shortcut;
        if s.alpha_args.len() < 2
            || s.alpha_field_blanks.len() < 2
            || s.alpha_field_names.len() < 2
            || s.numeric_args.len() < 27
            || s.numeric_field_blanks.len() < 27
            || s.numeric_field_names.len() < 27
        {
            return Err(unavailable(
                "actual complete Blind2alpha27numeric allocation",
            ));
        }
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("WindowMaterial:Blind", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = "name".into();
        s.alpha_field_names[1] = "slat_orientation".into();
        match object.fields.get("slat_orientation") {
            Some(ProjectInputValue::String(value)) if !value.is_empty() => {
                s.alpha_args[1] = value.to_ascii_uppercase();
                s.alpha_field_blanks[1] = false;
            }
            None | Some(ProjectInputValue::String(_)) => s.alpha_args[1] = "HORIZONTAL".into(),
            _ => return Err(unavailable("admitted Blind alpha scalar")),
        }
        for (index, (name, default)) in FIELDS.iter().enumerate() {
            s.numeric_field_names[index] = (*name).into();
            s.numeric_args[index] = *default;
            match object.fields.get(*name) {
                Some(ProjectInputValue::Number(value)) if value.is_finite() => {
                    s.numeric_args[index] = *value;
                    s.numeric_field_blanks[index] = false;
                }
                None => {}
                _ => return Err(unavailable("admitted Blind numeric scalar")),
            }
        }
        // Original min_fields29 makes all27 fixed numeric slots returned.
        Ok((2, 27))
    }

    pub(super) fn get_window_screen_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        if !self.source_is_epjson
            || !owners
                .input
                .has_object("WindowMaterial:Screen", &object.object_name)
        {
            return Err(unavailable("Screen owned direct-epJSON raw/IP identity"));
        }
        const FIELDS: [(&str, &str, f64); 12] = [
            (
                "diffuse_solar_reflectance",
                "Diffuse Solar Reflectance",
                0.0,
            ),
            (
                "diffuse_visible_reflectance",
                "Diffuse Visible Reflectance",
                0.0,
            ),
            (
                "thermal_hemispherical_emissivity",
                "Thermal Hemispherical Emissivity",
                0.9,
            ),
            ("conductivity", "Conductivity", 221.0),
            ("screen_material_spacing", "Screen Material Spacing", 0.0),
            ("screen_material_diameter", "Screen Material Diameter", 0.0),
            (
                "screen_to_glass_distance",
                "Screen to Glass Distance",
                0.025,
            ),
            ("top_opening_multiplier", "Top Opening Multiplier", 0.0),
            (
                "bottom_opening_multiplier",
                "Bottom Opening Multiplier",
                0.0,
            ),
            (
                "left_side_opening_multiplier",
                "Left Side Opening Multiplier",
                0.0,
            ),
            (
                "right_side_opening_multiplier",
                "Right Side Opening Multiplier",
                0.0,
            ),
            (
                "angle_of_resolution_for_screen_transmittance_output_map",
                "Angle of Resolution for Screen Transmittance Output Map",
                0.0,
            ),
        ];
        let s = &mut owners.shortcut;
        if s.alpha_args.len() < 2
            || s.alpha_field_blanks.len() < 2
            || s.alpha_field_names.len() < 2
            || s.numeric_args.len() < 12
            || s.numeric_field_blanks.len() < 12
            || s.numeric_field_names.len() < 12
        {
            return Err(unavailable(
                "actual complete Screen2alpha12numeric allocation",
            ));
        }
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("WindowMaterial:Screen", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = "name".into();
        s.alpha_field_names[1] = "reflected_beam_transmittance_accounting_method".into();
        match object
            .fields
            .get("reflected_beam_transmittance_accounting_method")
        {
            Some(ProjectInputValue::String(value)) if !value.is_empty() => {
                s.alpha_args[1] = value.to_ascii_uppercase();
                s.alpha_field_blanks[1] = false;
            }
            None | Some(ProjectInputValue::String(_)) => s.alpha_args[1] = "MODELASDIFFUSE".into(),
            _ => return Err(unavailable("admitted Screen alpha scalar")),
        }
        let maximum = FIELDS
            .iter()
            .enumerate()
            .filter(|(_, (name, _, _))| object.fields.contains_key(*name))
            .map(|(index, _)| index + 3)
            .max()
            .unwrap_or(0)
            .max(9);
        for (index, (name, _label, default)) in FIELDS.iter().enumerate() {
            s.numeric_field_names[index] = (*name).into();
            s.numeric_args[index] = *default;
            match object.fields.get(*name) {
                Some(ProjectInputValue::Number(value)) if value.is_finite() => {
                    s.numeric_args[index] = *value;
                    s.numeric_field_blanks[index] = false;
                }
                Some(ProjectInputValue::String(value))
                    if *name == "angle_of_resolution_for_screen_transmittance_output_map"
                        && value.is_empty() => {}
                None => {}
                _ => return Err(unavailable("admitted Screen numeric scalar")),
            }
        }
        Ok((2, maximum - 2))
    }

    pub(super) fn get_window_shade_equivalent_layer_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        if !self.source_is_epjson
            || !owners
                .input
                .has_object("WindowMaterial:Shade:EquivalentLayer", &object.object_name)
        {
            return Err(unavailable("ShadeEQL owned direct-epJSON raw/IP identity"));
        }
        const FIELDS: [(&str, f64); 11] = [
            ("shade_beam_beam_solar_transmittance", 0.0),
            ("front_side_shade_beam_diffuse_solar_transmittance", 0.0),
            ("back_side_shade_beam_diffuse_solar_transmittance", 0.0),
            ("front_side_shade_beam_diffuse_solar_reflectance", 0.0),
            ("back_side_shade_beam_diffuse_solar_reflectance", 0.0),
            (
                "shade_beam_beam_visible_transmittance_at_normal_incidence",
                0.0,
            ),
            (
                "shade_beam_diffuse_visible_transmittance_at_normal_incidence",
                0.0,
            ),
            (
                "shade_beam_diffuse_visible_reflectance_at_normal_incidence",
                0.0,
            ),
            ("shade_material_infrared_transmittance", 0.05),
            ("front_side_shade_material_infrared_emissivity", 0.91),
            ("back_side_shade_material_infrared_emissivity", 0.91),
        ];
        let s = &mut owners.shortcut;
        if s.alpha_args.is_empty()
            || s.alpha_field_blanks.is_empty()
            || s.alpha_field_names.is_empty()
            || s.numeric_args.len() < 11
            || s.numeric_field_blanks.len() < 11
            || s.numeric_field_names.len() < 11
        {
            return Err(unavailable(
                "actual complete ShadeEQL1alpha11numeric allocation",
            ));
        }
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("WindowMaterial:Shade:EquivalentLayer", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = "name".into();
        // Official min_fields6 includes Name. Fixed defaults are processed even
        // outside the returned count, exactly as setObjectItemValue does.
        let mut numbers = 5;
        for (index, (name, default)) in FIELDS.iter().enumerate() {
            s.numeric_field_names[index] = (*name).into();
            s.numeric_args[index] = *default;
            match object.fields.get(*name) {
                Some(ProjectInputValue::Number(value)) if value.is_finite() => {
                    s.numeric_args[index] = *value;
                    s.numeric_field_blanks[index] = false;
                    numbers = numbers.max(index + 1);
                }
                None => {}
                _ => return Err(unavailable("admitted ShadeEQL numeric scalar")),
            }
        }
        Ok((1, numbers))
    }

    pub(super) fn get_window_drape_equivalent_layer_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        if !self.source_is_epjson
            || !owners
                .input
                .has_object("WindowMaterial:Drape:EquivalentLayer", &object.object_name)
        {
            return Err(unavailable("DrapeEQL owned direct-epJSON raw/IP identity"));
        }
        const FIELDS: [(&str, f64); 13] = [
            (
                "drape_beam_beam_solar_transmittance_at_normal_incidence",
                0.0,
            ),
            ("front_side_drape_beam_diffuse_solar_transmittance", 0.0),
            ("back_side_drape_beam_diffuse_solar_transmittance", 0.0),
            ("front_side_drape_beam_diffuse_solar_reflectance", 0.0),
            ("back_side_drape_beam_diffuse_solar_reflectance", 0.0),
            ("drape_beam_beam_visible_transmittance", 0.0),
            ("drape_beam_diffuse_visible_transmittance", 0.0),
            ("drape_beam_diffuse_visible_reflectance", 0.0),
            ("drape_material_infrared_transmittance", 0.05),
            ("front_side_drape_material_infrared_emissivity", 0.87),
            ("back_side_drape_material_infrared_emissivity", 0.87),
            ("width_of_pleated_fabric", 0.0),
            ("length_of_pleated_fabric", 0.0),
        ];
        let s = &mut owners.shortcut;
        if s.alpha_args.is_empty()
            || s.alpha_field_blanks.is_empty()
            || s.alpha_field_names.is_empty()
            || s.numeric_args.len() < 13
            || s.numeric_field_blanks.len() < 13
            || s.numeric_field_names.len() < 13
        {
            return Err(unavailable(
                "actual complete DrapeEQL1alpha13numeric allocation",
            ));
        }
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("WindowMaterial:Drape:EquivalentLayer", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = "name".into();
        // Official min_fields4 includes Name. Fixed defaults are processed even
        // outside the returned count, exactly as setObjectItemValue does.
        // Original required N2..N5 make admitted whole-Source rows return5..13;
        // the underlying generic primitive still has Native numeric floor3.
        let mut numbers = 3;
        for (index, (name, default)) in FIELDS.iter().enumerate() {
            s.numeric_field_names[index] = (*name).into();
            s.numeric_args[index] = *default;
            match object.fields.get(*name) {
                Some(ProjectInputValue::Number(value)) if value.is_finite() => {
                    s.numeric_args[index] = *value;
                    s.numeric_field_blanks[index] = false;
                    numbers = numbers.max(index + 1);
                }
                None => {}
                _ => return Err(unavailable("admitted DrapeEQL numeric scalar")),
            }
        }
        Ok((1, numbers))
    }

    pub(super) fn get_window_shade_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        if !self.source_is_epjson
            || !owners
                .input
                .has_object("WindowMaterial:Shade", &object.object_name)
        {
            return Err(unavailable("Shade owned direct-epJSON raw/IP identity"));
        }
        const FIELDS: [(&str, &str, f64); 14] = [
            ("solar_transmittance", "Solar Transmittance", 0.0),
            ("solar_reflectance", "Solar Reflectance", 0.0),
            ("visible_transmittance", "Visible Transmittance", 0.0),
            ("visible_reflectance", "Visible Reflectance", 0.0),
            (
                "infrared_hemispherical_emissivity",
                "Infrared Hemispherical Emissivity",
                0.0,
            ),
            ("infrared_transmittance", "Infrared Transmittance", 0.0),
            ("thickness", "Thickness", 0.0),
            ("conductivity", "Conductivity", 0.0),
            ("shade_to_glass_distance", "Shade to Glass Distance", 0.05),
            ("top_opening_multiplier", "Top Opening Multiplier", 0.5),
            (
                "bottom_opening_multiplier",
                "Bottom Opening Multiplier",
                0.5,
            ),
            (
                "left_side_opening_multiplier",
                "Left-Side Opening Multiplier",
                0.5,
            ),
            (
                "right_side_opening_multiplier",
                "Right-Side Opening Multiplier",
                0.5,
            ),
            ("airflow_permeability", "Airflow Permeability", 0.0),
        ];
        let s = &mut owners.shortcut;
        if s.alpha_args.is_empty()
            || s.alpha_field_blanks.is_empty()
            || s.alpha_field_names.is_empty()
            || s.numeric_args.len() < 14
            || s.numeric_field_blanks.len() < 14
            || s.numeric_field_names.len() < 14
        {
            return Err(unavailable(
                "actual complete Shade1alpha14numeric allocation",
            ));
        }
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("WindowMaterial:Shade", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = "name".into();
        for (index, (name, _label, default)) in FIELDS.iter().enumerate() {
            s.numeric_field_names[index] = (*name).into();
            s.numeric_args[index] = *default;
            match object.fields.get(*name) {
                Some(ProjectInputValue::Number(value)) if value.is_finite() => {
                    s.numeric_args[index] = *value;
                    s.numeric_field_blanks[index] = false;
                }
                None => {}
                _ => return Err(unavailable("admitted Shade numeric scalar")),
            }
        }
        // Official min_fields15 includes Name: all14 numeric slots are returned.
        Ok((1, 14))
    }

    pub(super) fn get_window_gas_mixture_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        if !self.source_is_epjson
            || !owners
                .input
                .has_object("WindowMaterial:GasMixture", &object.object_name)
        {
            return Err(unavailable(
                "GasMixture owned direct-epJSON raw/IP identity",
            ));
        }
        // Exact legacy order, including required Gas2 even for NumGases1.
        const FIELDS: [(&str, bool); 11] = [
            ("name", true),
            ("thickness", false),
            ("number_of_gases_in_mixture", false),
            ("gas_1_type", true),
            ("gas_1_fraction", false),
            ("gas_2_type", true),
            ("gas_2_fraction", false),
            ("gas_3_type", true),
            ("gas_3_fraction", false),
            ("gas_4_type", true),
            ("gas_4_fraction", false),
        ];
        let s = &mut owners.shortcut;
        if s.alpha_args.len() < 5
            || s.alpha_field_blanks.len() < 5
            || s.alpha_field_names.len() < 5
            || s.numeric_args.len() < 6
            || s.numeric_field_blanks.len() < 6
            || s.numeric_field_names.len() < 6
        {
            return Err(unavailable(
                "actual complete shared GasMixture5alpha6numeric allocation",
            ));
        }
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("WindowMaterial:GasMixture", &object.object_name);
        let maximum = FIELDS
            .iter()
            .enumerate()
            .filter(|(_, (field, _))| object.fields.contains_key(*field))
            .map(|(index, _)| index + 1)
            .max()
            .unwrap_or(0)
            .max(7);
        let (mut a, mut n, mut alphas, mut numerics) = (0, 0, 0, 0);
        for (position, (field, is_alpha)) in FIELDS.iter().enumerate() {
            if *is_alpha {
                s.alpha_field_names[a] = (*field).into();
                if position == 0 {
                    s.alpha_args[a] = object.object_name.to_ascii_uppercase();
                    s.alpha_field_blanks[a] = object.object_name.is_empty();
                } else {
                    match object.fields.get(*field) {
                        Some(ProjectInputValue::String(value)) => {
                            s.alpha_args[a] = value.to_ascii_uppercase();
                            s.alpha_field_blanks[a] = value.is_empty();
                        }
                        None => {}
                        _ => return Err(unavailable("admitted GasMixture alpha scalar")),
                    }
                }
                a += 1;
                if position < maximum {
                    alphas = a;
                }
            } else {
                s.numeric_field_names[n] = (*field).into();
                match object.fields.get(*field) {
                    Some(ProjectInputValue::Number(value)) if value.is_finite() => {
                        s.numeric_args[n] = *value;
                        s.numeric_field_blanks[n] = false;
                    }
                    None => {}
                    _ => return Err(unavailable("admitted GasMixture numeric scalar")),
                }
                n += 1;
                if position < maximum {
                    numerics = n;
                }
            }
        }
        Ok((alphas, numerics))
    }

    pub(super) fn get_window_gas_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        if !self.source_is_epjson {
            return Err(unavailable("named Gas requires owned direct-epJSON getter"));
        }
        if !owners
            .input
            .has_object("WindowMaterial:Gas", &object.object_name)
        {
            return Err(unavailable("Gas raw/IP identity"));
        }
        const NUMERIC_FIELDS: [&str; 12] = [
            "thickness",
            "conductivity_coefficient_a",
            "conductivity_coefficient_b",
            "conductivity_coefficient_c",
            "viscosity_coefficient_a",
            "viscosity_coefficient_b",
            "viscosity_coefficient_c",
            "specific_heat_coefficient_a",
            "specific_heat_coefficient_b",
            "specific_heat_coefficient_c",
            "molecular_weight",
            "specific_heat_ratio",
        ];
        let s = &mut owners.shortcut;
        if s.alpha_args.len() < 2
            || s.alpha_field_blanks.len() < 2
            || s.alpha_field_names.len() < 2
            || s.numeric_args.len() < 12
            || s.numeric_field_blanks.len() < 12
            || s.numeric_field_names.len() < 12
        {
            return Err(unavailable(
                "actual complete shared Gas2alpha12numeric allocation",
            ));
        }
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("WindowMaterial:Gas", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = "name".into();
        s.alpha_args[1] = alpha(object, "gas_type")?.to_ascii_uppercase();
        s.alpha_field_blanks[1] = false;
        s.alpha_field_names[1] = "gas_type".into();
        let mut numeric_count = 1; // Native schema min_fields=3 includes N1.
        for (index, field) in NUMERIC_FIELDS.iter().enumerate() {
            s.numeric_field_names[index] = (*field).into();
            match object.fields.get(*field) {
                Some(ProjectInputValue::Number(value)) if value.is_finite() => {
                    s.numeric_args[index] = *value;
                    s.numeric_field_blanks[index] = false;
                    numeric_count = numeric_count.max(index + 1);
                }
                None if index > 0 => {}
                _ => {
                    return Err(unavailable(
                        "admitted Gas numeric scalar/required Thickness",
                    ));
                }
            }
        }
        Ok((2, numeric_count))
    }

    pub(super) fn get_simple_glazing_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        if !self.source_is_epjson {
            return Err(unavailable(
                "SimpleGlazing requires the owned Source epJSON getter",
            ));
        }
        if !owners
            .input
            .has_object("WindowMaterial:SimpleGlazingSystem", &object.object_name)
        {
            return Err(unavailable("SimpleGlazing raw/IP identity"));
        }
        let s = &mut owners.shortcut;
        if s.alpha_args.is_empty()
            || s.alpha_field_blanks.is_empty()
            || s.alpha_field_names.is_empty()
            || s.numeric_args.len() < 3
            || s.numeric_field_blanks.len() < 3
            || s.numeric_field_names.len() < 3
        {
            return Err(unavailable(
                "actual complete SimpleGlazing shared allocation",
            ));
        }
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("WindowMaterial:SimpleGlazingSystem", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = "name".into();
        for (index, field) in [
            "u_factor",
            "solar_heat_gain_coefficient",
            "visible_transmittance",
        ]
        .iter()
        .enumerate()
        {
            s.numeric_field_names[index] = (*field).into();
        }
        s.numeric_args[0] = numeric(object, "u_factor", None)?;
        s.numeric_field_blanks[0] = false;
        s.numeric_args[1] = numeric(object, "solar_heat_gain_coefficient", None)?;
        s.numeric_field_blanks[1] = false;
        let numerics = match object.fields.get("visible_transmittance") {
            None => 2,
            Some(ProjectInputValue::Number(value)) if value.is_finite() => {
                s.numeric_args[2] = *value;
                s.numeric_field_blanks[2] = false;
                3
            }
            Some(_) => return Err(unavailable("SimpleGlazing optional VT numeric scalar")),
        };
        Ok((1, numerics))
    }

    pub(super) fn get_infrared_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(), RuntimeError> {
        if !self.source_is_epjson && object.idf_max_fields.is_none() {
            return Err(unavailable("missing actual IRT IDF field count"));
        }
        if !owners
            .input
            .has_object("Material:InfraredTransparent", &object.object_name)
        {
            return Err(unavailable("IRT raw/IP identity"));
        }
        let s = &mut owners.shortcut;
        if s.alpha_args.is_empty()
            || s.alpha_field_blanks.is_empty()
            || s.alpha_field_names.is_empty()
        {
            return Err(unavailable("actual complete IRT shared alpha allocation"));
        }
        // All six passed arrays reset even though IRT has zero numeric fields.
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("Material:InfraredTransparent", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = if self.source_is_epjson {
            "name"
        } else {
            "Name"
        }
        .into();
        Ok(())
    }

    pub(super) fn get_air_gap_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        let max_fields = if self.source_is_epjson {
            2 // Name plus the schema-required thermal_resistance.
        } else {
            object
                .idf_max_fields
                .ok_or_else(|| unavailable("missing actual Material:AirGap IDF field count"))?
        };
        if !owners
            .input
            .has_object("Material:AirGap", &object.object_name)
        {
            return Err(unavailable("Material:AirGap raw/IP identity"));
        }
        let s = &mut owners.shortcut;
        if s.alpha_args.is_empty()
            || s.numeric_args.is_empty()
            || s.alpha_field_blanks.is_empty()
            || s.alpha_field_names.is_empty()
            || s.numeric_field_blanks.is_empty()
            || s.numeric_field_names.is_empty()
        {
            return Err(unavailable(
                "actual complete Material:AirGap shared allocation",
            ));
        }
        // This actual getObjectItem call receives, and resets, all six arrays.
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("Material:AirGap", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = if self.source_is_epjson {
            "name"
        } else {
            "Name"
        }
        .into();
        s.numeric_args[0] = numeric(object, "thermal_resistance", None)?;
        s.numeric_field_blanks[0] = false; // Required successful schema numeric; no default.
        s.numeric_field_names[0] = if self.source_is_epjson {
            "thermal_resistance"
        } else {
            "Thermal Resistance"
        }
        .into();
        Ok((1, usize::from(1 < max_fields)))
    }

    pub(super) fn get_no_mass_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        let fields = [
            "name",
            "roughness",
            "thermal_resistance",
            "thermal_absorptance",
            "solar_absorptance",
            "visible_absorptance",
        ];
        let legacy = [
            "Name",
            "Roughness",
            "Thermal Resistance",
            "Thermal Absorptance",
            "Solar Absorptance",
            "Visible Absorptance",
        ];
        let max_fields = if self.source_is_epjson {
            fields.iter().enumerate().fold(3, |count, (index, field)| {
                if object.fields.contains_key(*field) {
                    count.max(index + 1)
                } else {
                    count
                }
            })
        } else {
            object
                .idf_max_fields
                .ok_or_else(|| unavailable("missing actual Material:NoMass IDF field count"))?
        };
        if !owners
            .input
            .has_object("Material:NoMass", &object.object_name)
        {
            return Err(unavailable("Material:NoMass raw/IP identity"));
        }
        let s = &mut owners.shortcut;
        if s.alpha_args.len() < 2
            || s.numeric_args.len() < 4
            || s.alpha_field_blanks.len() < 2
            || s.alpha_field_names.len() < 2
            || s.numeric_field_blanks.len() < 4
            || s.numeric_field_names.len() < 4
        {
            return Err(unavailable(
                "actual complete Material:NoMass shared allocation",
            ));
        }
        // InputProcessor993-1013 resets ALL six passed arrays, not just used slots.
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("Material:NoMass", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = if self.source_is_epjson {
            fields[0]
        } else {
            legacy[0]
        }
        .into();
        s.alpha_args[1] = alpha(object, fields[1])?.to_ascii_uppercase();
        s.alpha_field_blanks[1] = false;
        s.alpha_field_names[1] = if self.source_is_epjson {
            fields[1]
        } else {
            legacy[1]
        }
        .into();
        let defaults = [None, Some(0.9), Some(0.7), Some(0.7)];
        let mut num_numbers = 0;
        for (index, default) in defaults.iter().enumerate() {
            let field = fields[index + 2];
            s.numeric_args[index] = numeric(object, field, *default)?;
            s.numeric_field_blanks[index] = object.fields.get(field).is_none_or(
                |value| matches!(value, ProjectInputValue::String(text) if text.is_empty()),
            );
            s.numeric_field_names[index] = if self.source_is_epjson {
                field
            } else {
                legacy[index + 2]
            }
            .into();
            if index + 2 < max_fields {
                num_numbers = index + 1;
            }
        }
        // Name always increments NumAlphas; roughness increments only in range.
        // Both are required by the admitted successful domain; no retained local
        // NumAlphas or IOStat owner is fabricated after the call returns.
        Ok((1 + usize::from(1 < max_fields), num_numbers))
    }
}
