//! Original-schema unused Shade SourceB prerequisite; properties belong to the real getter.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

pub(crate) fn known_field(field: &str) -> bool {
    matches!(
        field,
        "solar_transmittance"
            | "solar_reflectance"
            | "visible_transmittance"
            | "visible_reflectance"
            | "infrared_hemispherical_emissivity"
            | "infrared_transmittance"
            | "thickness"
            | "conductivity"
            | "shade_to_glass_distance"
            | "top_opening_multiplier"
            | "bottom_opening_multiplier"
            | "left_side_opening_multiplier"
            | "right_side_opening_multiplier"
            | "airflow_permeability"
    )
}

pub(super) fn admitted_field(field: &str, value: &RawValue) -> bool {
    if !known_field(field) {
        return true;
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|value| {
            value.is_finite()
                && match field {
                    "thickness" | "conductivity" => value > 0.0,
                    "infrared_hemispherical_emissivity" => value > 0.0 && value < 1.0,
                    "shade_to_glass_distance" => (0.001..=1.0).contains(&value),
                    "top_opening_multiplier"
                    | "bottom_opening_multiplier"
                    | "left_side_opening_multiplier"
                    | "right_side_opening_multiplier" => (0.0..=1.0).contains(&value),
                    "airflow_permeability" => (0.0..=0.8).contains(&value),
                    _ => (0.0..1.0).contains(&value),
                }
        })
}

pub(crate) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw.objects.get(&ObjectType("WindowMaterial:Shade".into())) else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    if !raw.is_epjson() {
        return Err("Source unused Shade requires direct epJSON".into());
    }
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0) {
            return Err("Source Shade requires original RE2 material name".into());
        }
        for required in [
            "solar_transmittance",
            "solar_reflectance",
            "visible_transmittance",
            "visible_reflectance",
            "infrared_hemispherical_emissivity",
            "infrared_transmittance",
            "thickness",
            "conductivity",
        ] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !admitted_field(required, value))
            {
                return Err(format!("Source Shade requires schema-valid {required}"));
            }
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !admitted_field(&field.0, value))
        {
            return Err(
                "Source Shade requires every supplied known field to remain schema-valid".into(),
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
        return Err("Source unused Shade excludes every Construction layer reference".into());
    }
    Ok(())
}
