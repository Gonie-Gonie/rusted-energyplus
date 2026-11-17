//! Original-schema unused MaterialGlazingEQL prerequisite; runtime properties belong to its real getter.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

pub(crate) fn known_field(field: &str) -> bool {
    matches!(
        field,
        "optical_data_type"
            | "window_glass_spectral_data_set_name"
            | "front_side_beam_beam_solar_transmittance"
            | "back_side_beam_beam_solar_transmittance"
            | "front_side_beam_beam_solar_reflectance"
            | "back_side_beam_beam_solar_reflectance"
            | "front_side_beam_beam_visible_solar_transmittance"
            | "back_side_beam_beam_visible_solar_transmittance"
            | "front_side_beam_beam_visible_solar_reflectance"
            | "back_side_beam_beam_visible_solar_reflectance"
            | "front_side_beam_diffuse_solar_transmittance"
            | "back_side_beam_diffuse_solar_transmittance"
            | "front_side_beam_diffuse_solar_reflectance"
            | "back_side_beam_diffuse_solar_reflectance"
            | "front_side_beam_diffuse_visible_solar_transmittance"
            | "back_side_beam_diffuse_visible_solar_transmittance"
            | "front_side_beam_diffuse_visible_solar_reflectance"
            | "back_side_beam_diffuse_visible_solar_reflectance"
            | "diffuse_diffuse_solar_transmittance"
            | "front_side_diffuse_diffuse_solar_reflectance"
            | "back_side_diffuse_diffuse_solar_reflectance"
            | "diffuse_diffuse_visible_solar_transmittance"
            | "front_side_diffuse_diffuse_visible_solar_reflectance"
            | "back_side_diffuse_diffuse_visible_solar_reflectance"
            | "infrared_transmittance_applies_to_front_and_back_"
            | "front_side_infrared_emissivity"
            | "back_side_infrared_emissivity"
            | "thermal_resistance"
    )
}

pub(super) fn admitted_field(field: &str, value: &RawValue) -> bool {
    if !known_field(field) {
        return true;
    }
    if field == "optical_data_type" {
        return matches!(value, RawValue::String(text) if matches!(text.as_str(), "" | "SpectralAverage"));
    }
    if field == "window_glass_spectral_data_set_name" {
        return matches!(value, RawValue::String(_));
    }
    if matches!(
        field,
        "diffuse_diffuse_solar_transmittance"
            | "front_side_diffuse_diffuse_solar_reflectance"
            | "back_side_diffuse_diffuse_solar_reflectance"
            | "diffuse_diffuse_visible_solar_transmittance"
            | "front_side_diffuse_diffuse_visible_solar_reflectance"
            | "back_side_diffuse_diffuse_visible_solar_reflectance"
    ) && let RawValue::String(text) = value
    {
        return matches!(text.as_str(), "" | "Autocalculate");
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|value| {
            value.is_finite()
                && match field {
                    "thermal_resistance" => value > 0.0,
                    "front_side_infrared_emissivity" | "back_side_infrared_emissivity" => {
                        value > 0.0 && value < 1.0
                    }
                    _ => (0.0..=1.0).contains(&value),
                }
        })
}

pub(crate) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw
        .objects
        .get(&ObjectType("WindowMaterial:Glazing:EquivalentLayer".into()))
    else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    if !raw.is_epjson() {
        return Err("Source unused GlazingEQL requires direct epJSON".into());
    }
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0) {
            return Err("Source GlazingEQL requires original RE2 material name".into());
        }
        for required in [
            "front_side_beam_beam_solar_transmittance",
            "back_side_beam_beam_solar_transmittance",
            "front_side_beam_beam_solar_reflectance",
            "back_side_beam_beam_solar_reflectance",
        ] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !admitted_field(required, value))
            {
                return Err(format!(
                    "Source GlazingEQL requires schema-valid {required}"
                ));
            }
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !admitted_field(&field.0, value))
        {
            return Err("Source GlazingEQL requires every supplied known field to remain original-schema-valid".into());
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
        return Err("Source unused GlazingEQL excludes every Construction layer reference".into());
    }
    Ok(())
}
