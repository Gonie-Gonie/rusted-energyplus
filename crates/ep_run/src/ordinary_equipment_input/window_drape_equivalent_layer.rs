//! Original-schema unused MaterialDrapeEQL prerequisite; runtime properties belong to its real getter.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

pub(crate) fn known_field(field: &str) -> bool {
    matches!(
        field,
        "drape_beam_beam_solar_transmittance_at_normal_incidence"
            | "front_side_drape_beam_diffuse_solar_transmittance"
            | "back_side_drape_beam_diffuse_solar_transmittance"
            | "front_side_drape_beam_diffuse_solar_reflectance"
            | "back_side_drape_beam_diffuse_solar_reflectance"
            | "drape_beam_beam_visible_transmittance"
            | "drape_beam_diffuse_visible_transmittance"
            | "drape_beam_diffuse_visible_reflectance"
            | "drape_material_infrared_transmittance"
            | "front_side_drape_material_infrared_emissivity"
            | "back_side_drape_material_infrared_emissivity"
            | "width_of_pleated_fabric"
            | "length_of_pleated_fabric"
    )
}

pub(super) fn admitted_field(field: &str, value: &RawValue) -> bool {
    if !known_field(field) {
        return true;
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    // These are original-schema checks, before Source signed-i64-effective conversion.
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|value| {
            value.is_finite()
                && match field {
                    "drape_beam_beam_solar_transmittance_at_normal_incidence" => {
                        (0.0..=0.2).contains(&value)
                    }
                    "front_side_drape_material_infrared_emissivity"
                    | "back_side_drape_material_infrared_emissivity" => value > 0.0 && value < 1.0,
                    "width_of_pleated_fabric" | "length_of_pleated_fabric" => value >= 0.0,
                    _ => (0.0..1.0).contains(&value),
                }
        })
}

pub(crate) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw
        .objects
        .get(&ObjectType("WindowMaterial:Drape:EquivalentLayer".into()))
    else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    raw.validate_source_idf_object_custody("WindowMaterial:Drape:EquivalentLayer")?;
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0) {
            return Err("Source DrapeEQL requires original RE2 material name".into());
        }
        for required in [
            "front_side_drape_beam_diffuse_solar_transmittance",
            "back_side_drape_beam_diffuse_solar_transmittance",
            "front_side_drape_beam_diffuse_solar_reflectance",
            "back_side_drape_beam_diffuse_solar_reflectance",
        ] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !admitted_field(required, value))
            {
                return Err(format!("Source DrapeEQL requires schema-valid {required}"));
            }
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !admitted_field(&field.0, value))
        {
            return Err("Source DrapeEQL requires every supplied known field to remain original-schema-valid".into());
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
        return Err("Source unused DrapeEQL excludes every Construction layer reference".into());
    }
    Ok(())
}
