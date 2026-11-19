//! Original-schema unused MaterialBlindEQL prerequisite; runtime properties belong to its real getter.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

pub(crate) fn known_field(field: &str) -> bool {
    matches!(
        field,
        "slat_orientation"
            | "slat_width"
            | "slat_separation"
            | "slat_crown"
            | "slat_angle"
            | "front_side_slat_beam_diffuse_solar_transmittance"
            | "back_side_slat_beam_diffuse_solar_transmittance"
            | "front_side_slat_beam_diffuse_solar_reflectance"
            | "back_side_slat_beam_diffuse_solar_reflectance"
            | "front_side_slat_beam_diffuse_visible_transmittance"
            | "back_side_slat_beam_diffuse_visible_transmittance"
            | "front_side_slat_beam_diffuse_visible_reflectance"
            | "back_side_slat_beam_diffuse_visible_reflectance"
            | "slat_diffuse_diffuse_solar_transmittance"
            | "front_side_slat_diffuse_diffuse_solar_reflectance"
            | "back_side_slat_diffuse_diffuse_solar_reflectance"
            | "slat_diffuse_diffuse_visible_transmittance"
            | "front_side_slat_diffuse_diffuse_visible_reflectance"
            | "back_side_slat_diffuse_diffuse_visible_reflectance"
            | "slat_infrared_transmittance"
            | "front_side_slat_infrared_emissivity"
            | "back_side_slat_infrared_emissivity"
            | "slat_angle_control"
    )
}

pub(super) fn admitted_field(field: &str, value: &RawValue) -> bool {
    if !known_field(field) {
        return true;
    }
    if matches!(field, "slat_orientation" | "slat_angle_control") {
        let RawValue::String(text) = value else {
            return false;
        };
        return if field == "slat_orientation" {
            matches!(text.as_str(), "" | "Horizontal" | "Vertical")
        } else {
            matches!(
                text.as_str(),
                "" | "FixedSlatAngle" | "MaximizeSolar" | "BlockBeamSolar"
            )
        };
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|value| {
            value.is_finite()
                && match field {
                    "slat_width" | "slat_separation" => value > 0.0 && value <= 0.025,
                    "slat_crown" => (0.0..=0.00156).contains(&value),
                    "slat_angle" => (-90.0..=90.0).contains(&value),
                    _ => (0.0..1.0).contains(&value),
                }
        })
}

pub(crate) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw
        .objects
        .get(&ObjectType("WindowMaterial:Blind:EquivalentLayer".into()))
    else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    raw.validate_source_idf_object_custody("WindowMaterial:Blind:EquivalentLayer")?;
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0) {
            return Err("Source BlindEQL requires original RE2 material name".into());
        }
        for required in [
            "slat_width",
            "slat_separation",
            "front_side_slat_beam_diffuse_solar_reflectance",
            "back_side_slat_beam_diffuse_solar_reflectance",
            "front_side_slat_diffuse_diffuse_solar_reflectance",
            "back_side_slat_diffuse_diffuse_solar_reflectance",
        ] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !admitted_field(required, value))
            {
                return Err(format!("Source BlindEQL requires schema-valid {required}"));
            }
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !admitted_field(&field.0, value))
        {
            return Err("Source BlindEQL requires every supplied known field to remain original-schema-valid".into());
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
        return Err("Source unused BlindEQL excludes every Construction layer reference".into());
    }
    Ok(())
}
