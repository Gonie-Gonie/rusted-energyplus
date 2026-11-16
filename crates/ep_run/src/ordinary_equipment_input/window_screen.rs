//! Original-schema unused Screen SourceB prerequisite; no typed optical seeding.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

pub(crate) fn known_field(field: &str) -> bool {
    matches!(
        field,
        "reflected_beam_transmittance_accounting_method"
            | "diffuse_solar_reflectance"
            | "diffuse_visible_reflectance"
            | "thermal_hemispherical_emissivity"
            | "conductivity"
            | "screen_material_spacing"
            | "screen_material_diameter"
            | "screen_to_glass_distance"
            | "top_opening_multiplier"
            | "bottom_opening_multiplier"
            | "left_side_opening_multiplier"
            | "right_side_opening_multiplier"
            | "angle_of_resolution_for_screen_transmittance_output_map"
    )
}

pub(super) fn admitted_field(field: &str, value: &RawValue) -> bool {
    if !known_field(field) {
        return true;
    }
    if field == "reflected_beam_transmittance_accounting_method" {
        return matches!(value, RawValue::String(value) if matches!(value.as_str(),
                "" | "DoNotModel" | "ModelAsDirectBeam" | "ModelAsDiffuse"));
    }
    if field == "angle_of_resolution_for_screen_transmittance_output_map" {
        return matches!(value, RawValue::String(value) if value.is_empty())
            || matches!(value, RawValue::Number(token)
                    if ep_model::classify_source_json_number(token).is_ok()
                    && token.parse::<f64>().ok().is_some_and(|value| matches!(value, 0.0 | 1.0 | 2.0 | 3.0 | 5.0)));
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|value| {
            value.is_finite()
                && match field {
                    "diffuse_solar_reflectance" | "diffuse_visible_reflectance" => {
                        (0.0..1.0).contains(&value)
                    }
                    "thermal_hemispherical_emissivity" => value > 0.0 && value < 1.0,
                    "conductivity" | "screen_material_spacing" | "screen_material_diameter" => {
                        value > 0.0
                    }
                    "screen_to_glass_distance" => (0.001..=1.0).contains(&value),
                    _ => (0.0..=1.0).contains(&value),
                }
        })
}

pub(crate) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw.objects.get(&ObjectType("WindowMaterial:Screen".into())) else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    if !raw.is_epjson() {
        return Err("Source unused Screen requires direct epJSON".into());
    }
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0) {
            return Err("Source Screen requires original RE2 material name".into());
        }
        for required in [
            "diffuse_solar_reflectance",
            "diffuse_visible_reflectance",
            "screen_material_spacing",
            "screen_material_diameter",
        ] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !admitted_field(required, value))
            {
                return Err(format!("Source Screen requires schema-valid {required}"));
            }
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !admitted_field(&field.0, value))
        {
            return Err(
                "Source Screen requires every supplied known field to remain original-schema-valid"
                    .into(),
            );
        }
    }
    if raw
        .objects
        .iter()
        .filter(|(family, _)| family.0.starts_with("Construction"))
        .any(|(_, constructions)| {
            constructions.values().any(|construction| {
                construction.fields.iter().any(|(field, value)| {
                    let layer = field.0 == "outside_layer"
                        || (2..=10).any(|index| field.0 == format!("layer_{index}"));
                    layer
                        && matches!(value, RawValue::String(reference)
                if rows.keys().any(|key| key.0.eq_ignore_ascii_case(reference)))
                })
            })
        })
    {
        return Err("Source unused Screen excludes every Construction layer reference".into());
    }
    Ok(())
}
