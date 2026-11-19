//! Purpose-specific Material raw custody, using the sole admitted IP and loader.
use ep_raw_model::{RawModel, RawValue};
use ep_runtime::heat_balance::selected_input_processor::SelectedInputProcessor;
use ep_runtime::heat_balance::selected_material_data::{
    SelectedMaterialInput, SourceGlazingSpectralInput,
};
use ep_runtime::heat_balance::selected_project_data::{ProjectInputObject, ProjectInputValue};
use std::collections::BTreeMap;

pub(crate) fn resolve(
    raw: &RawModel,
    input: &SelectedInputProcessor,
) -> Result<SelectedMaterialInput, String> {
    resolve_ordered(raw, input, false)
}

pub(crate) fn resolve_source(
    raw: &RawModel,
    input: &SelectedInputProcessor,
) -> Result<SelectedMaterialInput, String> {
    resolve_ordered(raw, input, true)
}

fn resolve_ordered(
    raw: &RawModel,
    input: &SelectedInputProcessor,
    source_order: bool,
) -> Result<SelectedMaterialInput, String> {
    let mut result = SelectedMaterialInput {
        source_is_epjson: raw.is_epjson(),
        ..SelectedMaterialInput::default()
    };
    if source_order {
        if !raw.is_epjson() {
            for &family in ep_raw_model::SOURCE_IDF_OBJECT_FAMILIES {
                raw.validate_source_idf_object_custody(family)?;
            }
        }
        let family = "MaterialProperty:GlazingSpectralData";
        let rows = raw.source_ordered_instances(family)?;
        if rows.len() != input.get_num_objects_found(family) {
            return Err("spectral raw/IP count".into());
        }
        let number = |value: &RawValue| -> Result<ProjectInputValue, String> {
            let RawValue::Number(token) = value else {
                return Err("spectral original schema numeric".into());
            };
            Ok(ProjectInputValue::Number(
                ep_model::classify_source_json_number(token)
                    .map_err(|_| "spectral stored JSON number")?
                    .as_f64(),
            ))
        };
        for (name, row) in rows {
            if !input.has_object(family, &name.0) {
                return Err("spectral raw/IP identity".into());
            }
            let mut fields = BTreeMap::new();
            let mut extensions = Vec::new();
            let mut idf_max_fields = None;
            for (field, value) in &row.fields {
                if !raw.is_epjson() && field.0 == "idf_max_fields" {
                    let RawValue::Number(token) = value else {
                        return Err("Source spectral original-IDF fixed-field custody".into());
                    };
                    idf_max_fields = Some(
                        token
                            .parse::<usize>()
                            .map_err(|_| "Source spectral original-IDF fixed-field custody")?,
                    );
                    continue;
                }
                if !crate::ordinary_equipment_input::source_b_spectral_data_known_field(&field.0) {
                    continue;
                }
                if field.0 == "extensions" {
                    let RawValue::Array(rows) = value else {
                        return Err("spectral extension array".into());
                    };
                    for row in rows {
                        let RawValue::Object(row) = row else {
                            return Err("spectral extension object".into());
                        };
                        let mut values = BTreeMap::new();
                        for (field, value) in row {
                            if matches!(
                                field.0.as_str(),
                                "wavelength"
                                    | "transmittance"
                                    | "front_reflectance"
                                    | "back_reflectance"
                            ) {
                                values.insert(field.0.clone(), number(value)?);
                            }
                        }
                        extensions.push(values);
                    }
                } else {
                    fields.insert(field.0.clone(), number(value)?);
                }
            }
            result.spectral_data.push(SourceGlazingSpectralInput {
                object: ProjectInputObject {
                    object_name: name.0.clone(),
                    fields,
                    idf_max_fields,
                },
                extensions,
            });
        }
    }
    for family in [
        "Material",
        "Material:NoMass",
        "Material:AirGap",
        "Material:InfraredTransparent",
        "WindowMaterial:Glazing:RefractionExtinctionMethod",
        "WindowMaterial:Glazing",
        "WindowMaterial:SimpleGlazingSystem",
        "WindowMaterial:Gas",
        "WindowMaterial:GasMixture",
        "WindowMaterial:Shade",
        "WindowMaterial:Shade:EquivalentLayer",
        "WindowMaterial:Drape:EquivalentLayer",
        "WindowMaterial:Screen:EquivalentLayer",
        "WindowMaterial:Blind:EquivalentLayer",
        "WindowMaterial:Glazing:EquivalentLayer",
        "WindowMaterial:Gap:EquivalentLayer",
        "Material:RoofVegetation",
        "WindowMaterial:Gap",
        "WindowMaterial:Screen",
        "WindowMaterial:Blind",
    ] {
        if matches!(
            family,
            "WindowMaterial:Glazing:RefractionExtinctionMethod"
                | "WindowMaterial:Glazing"
                | "WindowMaterial:SimpleGlazingSystem"
                | "WindowMaterial:Gas"
                | "WindowMaterial:GasMixture"
                | "WindowMaterial:Shade"
                | "WindowMaterial:Shade:EquivalentLayer"
                | "WindowMaterial:Drape:EquivalentLayer"
                | "WindowMaterial:Screen:EquivalentLayer"
                | "WindowMaterial:Blind:EquivalentLayer"
                | "WindowMaterial:Glazing:EquivalentLayer"
                | "WindowMaterial:Gap:EquivalentLayer"
                | "Material:RoofVegetation"
                | "WindowMaterial:Gap"
                | "WindowMaterial:Screen"
                | "WindowMaterial:Blind"
        ) && !source_order
        {
            // Source A/B admission is checked separately; Legacy projection stays held.
            continue;
        }
        let rows = if source_order {
            raw.source_ordered_instances(family)?
        } else {
            raw.ordered_instances(family)
                .map_err(|error| error.to_string())?
        };
        if rows.len() != input.get_num_objects_found(family) {
            return Err(format!("Material raw/IP count: {family}"));
        }
        if !raw.is_epjson() && !rows.is_empty() && !raw.has_idf_declaration_order(family) {
            return Err(format!("actual Material IDF order unavailable: {family}"));
        }
        let mut values = Vec::new();
        for (name, row) in rows {
            if !input.has_object(family, &name.0) {
                return Err(format!("Material raw/IP identity: {family}/{}", name.0));
            }
            let mut fields = BTreeMap::new();
            let mut idf_max_fields = None;
            for (field, value) in &row.fields {
                let source_json_material_fields = source_order
                    && raw.is_epjson()
                    && matches!(family, "Material:NoMass" | "Material:AirGap");
                let projected_key = if source_json_material_fields {
                    ep_model::source_json_c_string_prefix(&field.0)
                } else {
                    field.0.as_str()
                };
                // Native direct JSON reads only declared NoMass/AirGap operands.
                // This includes ignoring idf_* lookalikes and a nested name;
                // genuine Original-IDF custody still follows the existing path.
                if source_order
                    && raw.is_epjson()
                    && matches!(family, "Material:NoMass" | "Material:AirGap")
                    && ep_raw_model::source_idf_object_layout(family).is_some_and(
                        |(_, fixed, _)| {
                            fixed
                                .iter()
                                .skip(1)
                                .all(|(name, _, _)| *name != projected_key)
                        },
                    )
                {
                    continue;
                }
                if source_order
                    && !raw.is_epjson()
                    && ep_raw_model::source_idf_object_layout(family).is_some()
                    && field.0 == "idf_max_fields"
                {
                    let RawValue::Number(text) = value else {
                        return Err("Material idf_max_fields numeric custody".into());
                    };
                    idf_max_fields = Some(
                        text.parse::<usize>()
                            .map_err(|_| "Material idf_max_fields usize custody")?,
                    );
                    continue;
                }
                // Native's direct-JSON named Gas getter reads only declared
                // fields; Raw/IP retain every ignored member, including idf_*.
                if source_order
                    && family == "WindowMaterial:Glazing:RefractionExtinctionMethod"
                    && !crate::ordinary_equipment_input::source_b_refraction_known_field(&field.0)
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Glazing"
                    && !crate::ordinary_equipment_input::source_b_glazing_known_field(&field.0)
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Gas"
                    && !crate::ordinary_equipment_input::source_b_named_gas_known_field(&field.0)
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:GasMixture"
                    && !crate::ordinary_equipment_input::source_b_gas_mixture_known_field(&field.0)
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Shade:EquivalentLayer"
                    && !crate::ordinary_equipment_input::source_b_shade_equivalent_layer_known_field(
                        &field.0,
                    )
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Drape:EquivalentLayer"
                    && !crate::ordinary_equipment_input::source_b_drape_equivalent_layer_known_field(
                        &field.0,
                    )
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Screen:EquivalentLayer"
                    && !crate::ordinary_equipment_input::source_b_screen_equivalent_layer_known_field(
                        &field.0,
                    )
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Blind:EquivalentLayer"
                    && !crate::ordinary_equipment_input::source_b_blind_equivalent_layer_known_field(
                        &field.0,
                    )
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Glazing:EquivalentLayer"
                    && !crate::ordinary_equipment_input::source_b_glazing_equivalent_layer_known_field(
                        &field.0,
                    )
                {
                    continue;
                }

                if source_order
                    && family == "WindowMaterial:Gap:EquivalentLayer"
                    && !crate::ordinary_equipment_input::source_b_gap_equivalent_layer_known_field(
                        &field.0,
                    )
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Gap"
                    && !crate::ordinary_equipment_input::source_b_window_gap_known_field(
                        family, &field.0,
                    )
                {
                    continue;
                }
                if source_order
                    && family == "Material:RoofVegetation"
                    && !crate::ordinary_equipment_input::source_b_roof_vegetation_known_field(
                        &field.0,
                    )
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Shade"
                    && !crate::ordinary_equipment_input::source_b_shade_known_field(&field.0)
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Screen"
                    && !crate::ordinary_equipment_input::source_b_screen_known_field(&field.0)
                {
                    continue;
                }
                if source_order
                    && family == "WindowMaterial:Blind"
                    && !crate::ordinary_equipment_input::source_b_blind_known_field(&field.0)
                {
                    continue;
                }
                if field.0 == "idf_max_fields" {
                    let RawValue::Number(text) = value else {
                        return Err("Material idf_max_fields numeric custody".into());
                    };
                    idf_max_fields = Some(
                        text.parse::<usize>()
                            .map_err(|_| "Material idf_max_fields usize custody")?,
                    );
                    continue;
                }
                if matches!(field.0.as_str(), "idf_order" | "idf_max_extensible_fields") {
                    continue;
                }
                // Name-only IRT ignores additional JSON members; the real
                // raw/IP owners retain them rather than making them operands.
                if family == "Material:InfraredTransparent" {
                    continue;
                }
                let value = match value {
                    RawValue::String(value) => ProjectInputValue::String(value.clone()),
                    RawValue::Number(text) => {
                        let value = if source_order
                            && matches!(
                                family,
                                "WindowMaterial:Glazing:RefractionExtinctionMethod"
                                    | "WindowMaterial:Glazing"
                                    | "WindowMaterial:SimpleGlazingSystem"
                                    | "WindowMaterial:Gas"
                                    | "WindowMaterial:GasMixture"
                                    | "WindowMaterial:Shade"
                                    | "WindowMaterial:Shade:EquivalentLayer"
                                    | "WindowMaterial:Drape:EquivalentLayer"
                                    | "WindowMaterial:Screen:EquivalentLayer"
                                    | "WindowMaterial:Blind:EquivalentLayer"
                                    | "WindowMaterial:Glazing:EquivalentLayer"
                                    | "WindowMaterial:Gap:EquivalentLayer"
                                    | "WindowMaterial:Gap"
                                    | "Material:RoofVegetation"
                                    | "WindowMaterial:Screen"
                                    | "WindowMaterial:Blind"
                            ) {
                            ep_model::classify_source_json_number(text)
                                .map_err(|_| "SimpleGlazing stored JSON numeric kind")?
                                .as_f64()
                        } else {
                            text.parse::<f64>()
                                .map_err(|_| "Material raw binary64 parse")?
                        };
                        if !value.is_finite() {
                            return Err("Material nonfinite JSON number".into());
                        }
                        ProjectInputValue::Number(value)
                    }
                    _ => {
                        return Err(format!(
                            "Material scalar representation: {family}.{}",
                            field.0
                        ));
                    }
                };
                if source_json_material_fields {
                    if fields.insert(projected_key.into(), value).is_some() {
                        return Err(if family == "Material:NoMass" {
                            "uncoalesced Source NoMass field aliases"
                        } else {
                            "uncoalesced Source AirGap field aliases"
                        }
                        .into());
                    }
                } else {
                    fields.insert(field.0.clone(), value);
                }
            }
            if !raw.is_epjson()
                && (matches!(
                    family,
                    "Material:NoMass" | "Material:AirGap" | "Material:InfraredTransparent"
                ) || (source_order && ep_raw_model::source_idf_object_layout(family).is_some()))
                && idf_max_fields.is_none()
            {
                return Err(format!("actual {family} IDF physical field count absent"));
            }
            values.push(ProjectInputObject {
                object_name: name.0.clone(),
                fields,
                idf_max_fields,
            });
        }
        match family {
            "Material" => result.regular = values,
            "Material:NoMass" => result.no_mass = values,
            "Material:AirGap" => result.air_gaps = values,
            "Material:InfraredTransparent" => result.infrared_transparent = values,
            "WindowMaterial:Glazing:RefractionExtinctionMethod" => result.refraction = values,
            "WindowMaterial:Glazing" => result.glazing = values,
            "WindowMaterial:SimpleGlazingSystem" => result.simple_glazing = values,
            "WindowMaterial:Gas" => result.window_gas = values,
            "WindowMaterial:GasMixture" => result.window_gas_mixture = values,
            "WindowMaterial:Shade" => result.window_shade = values,
            "WindowMaterial:Shade:EquivalentLayer" => result.window_shade_equivalent_layer = values,
            "WindowMaterial:Drape:EquivalentLayer" => result.window_drape_equivalent_layer = values,
            "WindowMaterial:Screen:EquivalentLayer" => {
                result.window_screen_equivalent_layer = values
            }
            "WindowMaterial:Blind:EquivalentLayer" => result.window_blind_equivalent_layer = values,
            "WindowMaterial:Glazing:EquivalentLayer" => {
                result.window_glazing_equivalent_layer = values
            }
            "WindowMaterial:Gap:EquivalentLayer" => result.window_gap_equivalent_layer = values,
            "Material:RoofVegetation" => result.roof_vegetation = values,
            "WindowMaterial:Gap" => result.window_gap = values,
            "WindowMaterial:Screen" => result.window_screen = values,
            "WindowMaterial:Blind" => result.window_blind = values,
            _ => unreachable!("literal selected material family"),
        }
    }
    Ok(result)
}
