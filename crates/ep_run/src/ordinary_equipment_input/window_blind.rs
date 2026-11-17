//! Original-schema unused Blind SourceB prerequisite; actual getter owns joint rules.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

pub(crate) fn known_field(field: &str) -> bool {
    matches!(
        field,
        "slat_orientation"
            | "slat_width"
            | "slat_separation"
            | "slat_thickness"
            | "slat_angle"
            | "slat_conductivity"
            | "slat_beam_solar_transmittance"
            | "front_side_slat_beam_solar_reflectance"
            | "back_side_slat_beam_solar_reflectance"
            | "slat_diffuse_solar_transmittance"
            | "front_side_slat_diffuse_solar_reflectance"
            | "back_side_slat_diffuse_solar_reflectance"
            | "slat_beam_visible_transmittance"
            | "front_side_slat_beam_visible_reflectance"
            | "back_side_slat_beam_visible_reflectance"
            | "slat_diffuse_visible_transmittance"
            | "front_side_slat_diffuse_visible_reflectance"
            | "back_side_slat_diffuse_visible_reflectance"
            | "slat_infrared_hemispherical_transmittance"
            | "front_side_slat_infrared_hemispherical_emissivity"
            | "back_side_slat_infrared_hemispherical_emissivity"
            | "blind_to_glass_distance"
            | "blind_top_opening_multiplier"
            | "blind_bottom_opening_multiplier"
            | "blind_left_side_opening_multiplier"
            | "blind_right_side_opening_multiplier"
            | "minimum_slat_angle"
            | "maximum_slat_angle"
    )
}

pub(super) fn admitted_field(field: &str, value: &RawValue) -> bool {
    if !known_field(field) {
        return true;
    }
    if field == "slat_orientation" {
        return matches!(value, RawValue::String(value) if matches!(value.as_str(), "" | "Horizontal" | "Vertical"));
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|value| {
            value.is_finite()
                && match field {
                    "slat_width" => {
                        let bound_0 = value > 0.0;
                        let bound_1 = value <= 1.0;
                        bound_0 && bound_1
                    }
                    "slat_separation" => {
                        let bound_0 = value > 0.0;
                        let bound_1 = value <= 1.0;
                        bound_0 && bound_1
                    }
                    "slat_thickness" => {
                        let bound_0 = value > 0.0;
                        let bound_1 = value <= 0.1;
                        bound_0 && bound_1
                    }
                    "slat_angle" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value <= 180.0;
                        bound_0 && bound_1
                    }
                    "slat_conductivity" => value > 0.0,
                    "slat_beam_solar_transmittance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "front_side_slat_beam_solar_reflectance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "back_side_slat_beam_solar_reflectance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "slat_diffuse_solar_transmittance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "front_side_slat_diffuse_solar_reflectance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "back_side_slat_diffuse_solar_reflectance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "slat_beam_visible_transmittance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "front_side_slat_beam_visible_reflectance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "back_side_slat_beam_visible_reflectance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "slat_diffuse_visible_transmittance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "front_side_slat_diffuse_visible_reflectance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "back_side_slat_diffuse_visible_reflectance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "slat_infrared_hemispherical_transmittance" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "front_side_slat_infrared_hemispherical_emissivity" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "back_side_slat_infrared_hemispherical_emissivity" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value < 1.0;
                        bound_0 && bound_1
                    }
                    "blind_to_glass_distance" => {
                        let bound_0 = value >= 0.01;
                        let bound_1 = value <= 1.0;
                        bound_0 && bound_1
                    }
                    "blind_top_opening_multiplier" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value <= 1.0;
                        bound_0 && bound_1
                    }
                    "blind_bottom_opening_multiplier" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value <= 1.0;
                        bound_0 && bound_1
                    }
                    "blind_left_side_opening_multiplier" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value <= 1.0;
                        bound_0 && bound_1
                    }
                    "blind_right_side_opening_multiplier" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value <= 1.0;
                        bound_0 && bound_1
                    }
                    "minimum_slat_angle" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value <= 180.0;
                        bound_0 && bound_1
                    }
                    "maximum_slat_angle" => {
                        let bound_0 = value >= 0.0;
                        let bound_1 = value <= 180.0;
                        bound_0 && bound_1
                    }
                    _ => true,
                }
        })
}

pub(crate) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw.objects.get(&ObjectType("WindowMaterial:Blind".into())) else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    if !raw.is_epjson() {
        return Err("Source unused Blind requires direct epJSON".into());
    }
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0) {
            return Err("Source Blind requires original RE2 material name".into());
        }
        for required in [
            "slat_width",
            "slat_separation",
            "front_side_slat_beam_solar_reflectance",
            "back_side_slat_beam_solar_reflectance",
            "front_side_slat_diffuse_solar_reflectance",
            "back_side_slat_diffuse_solar_reflectance",
            "slat_beam_visible_transmittance",
        ] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !admitted_field(required, value))
            {
                return Err(format!("Source Blind requires schema-valid {required}"));
            }
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !admitted_field(&field.0, value))
        {
            return Err(
                "Source Blind requires every supplied known field to remain original-schema-valid"
                    .into(),
            );
        }
    }
    if raw.objects.iter().filter(|(family,_)| family.0.starts_with("Construction")).any(|(_,constructions)| {
        constructions.values().any(|construction| construction.fields.iter().any(|(field,value)| {
            let layer = field.0 == "outside_layer" || (2..=10).any(|index| field.0 == format!("layer_{index}"));
            layer && matches!(value, RawValue::String(reference) if rows.keys().any(|key| key.0.eq_ignore_ascii_case(reference)))
        }))
    }) { return Err("Source unused Blind excludes every Construction layer reference".into()); }
    Ok(())
}
