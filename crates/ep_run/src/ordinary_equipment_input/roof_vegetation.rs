//! Original-schema EcoRoof input prerequisite; Construction custody is checked at Source admission.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
pub(crate) fn known_field(field: &str) -> bool {
    matches!(
        field,
        "height_of_plants"
            | "leaf_area_index"
            | "leaf_reflectivity"
            | "leaf_emissivity"
            | "minimum_stomatal_resistance"
            | "thickness"
            | "conductivity_of_dry_soil"
            | "density_of_dry_soil"
            | "specific_heat_of_dry_soil"
            | "thermal_absorptance"
            | "solar_absorptance"
            | "visible_absorptance"
            | "saturation_volumetric_moisture_content_of_the_soil_layer"
            | "residual_volumetric_moisture_content_of_the_soil_layer"
            | "initial_volumetric_moisture_content_of_the_soil_layer"
            | "soil_layer_name"
            | "roughness"
            | "moisture_diffusion_calculation_method"
    )
}
pub(super) fn admitted_field(field: &str, value: &RawValue) -> bool {
    if !known_field(field) {
        return true;
    }
    if field == "soil_layer_name" {
        return matches!(value, RawValue::String(_));
    }
    if field == "roughness" {
        return matches!(value, RawValue::String(text) if matches!(text.as_str(), "" | "VeryRough" | "Rough" | "MediumRough" | "MediumSmooth" | "Smooth" | "VerySmooth"));
    }
    if field == "moisture_diffusion_calculation_method" {
        return matches!(value, RawValue::String(text) if matches!(text.as_str(), "" | "Simple" | "Advanced"));
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|value| {
            value.is_finite()
                && match field {
                    "height_of_plants" => value > 0.005 && value <= 1.0,
                    "leaf_area_index" => value > 0.001 && value <= 5.0,
                    "leaf_reflectivity" => (0.05..=0.5).contains(&value),
                    "leaf_emissivity" => (0.8..=1.0).contains(&value),
                    "minimum_stomatal_resistance" => (50.0..=300.0).contains(&value),
                    "thickness" => value > 0.05 && value <= 0.7,
                    "conductivity_of_dry_soil" => (0.2..=1.5).contains(&value),
                    "density_of_dry_soil" => (300.0..=2000.0).contains(&value),
                    "specific_heat_of_dry_soil" => value > 500.0 && value <= 2000.0,
                    "thermal_absorptance" => value > 0.8 && value <= 1.0,
                    "solar_absorptance" => (0.4..=0.9).contains(&value),
                    "visible_absorptance" => value > 0.5 && value <= 1.0,
                    "saturation_volumetric_moisture_content_of_the_soil_layer" => {
                        value > 0.1 && value <= 0.5
                    }
                    "residual_volumetric_moisture_content_of_the_soil_layer" => {
                        (0.01..=0.1).contains(&value)
                    }
                    "initial_volumetric_moisture_content_of_the_soil_layer" => {
                        value > 0.05 && value <= 0.5
                    }
                    _ => false,
                }
        })
}
pub(crate) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw
        .objects
        .get(&ObjectType("Material:RoofVegetation".into()))
    else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    if !raw.is_epjson() {
        if !raw.has_idf_declaration_order("Material:RoofVegetation") {
            return Err(
                "Source RoofVegetation original IDF requires recovered declaration order".into(),
            );
        }
        if rows.values().any(|object| {
            !matches!(
                object.fields.get(&FieldName("idf_max_fields".into())),
                Some(RawValue::Number(token)) if token.parse::<usize>().is_ok()
            )
        }) {
            return Err(
                "Source RoofVegetation original IDF requires idf_max_fields for every object"
                    .into(),
            );
        }
    }
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0) {
            return Err("Source RoofVegetation requires original RE2 material name".into());
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !admitted_field(&field.0, value))
        {
            return Err("Source RoofVegetation requires every supplied known field to remain original-schema-valid".into());
        }
    }
    Ok(())
}
