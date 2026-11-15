//! Pure unused-SGS Source prerequisite; no property calculation or runtime state.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

pub(super) fn admitted_field(name: &str, value: &RawValue) -> bool {
    if !matches!(
        name,
        "u_factor" | "solar_heat_gain_coefficient" | "visible_transmittance"
    ) {
        return false;
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|number| {
            number.is_finite() && number > 0.0 && (name == "u_factor" || number < 1.0)
        })
}

pub(crate) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw
        .objects
        .get(&ObjectType("WindowMaterial:SimpleGlazingSystem".into()))
    else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    if !raw.is_epjson() {
        return Err("Source unused SimpleGlazing requires direct epJSON".into());
    }
    for (name, object) in rows {
        if name.0.trim().is_empty() {
            return Err("Source SimpleGlazing requires an actual material name".into());
        }
        for required in ["u_factor", "solar_heat_gain_coefficient"] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !admitted_field(required, value))
            {
                return Err(format!(
                    "Source SimpleGlazing requires raw schema number {required}"
                ));
            }
        }
        if object
            .fields
            .get(&FieldName("visible_transmittance".into()))
            .is_some_and(|value| !admitted_field("visible_transmittance", value))
        {
            return Err("Source SimpleGlazing explicit VT requires a raw schema number".into());
        }
    }
    if raw.objects.get(&ObjectType("Construction".into())).is_some_and(|constructions| {
        constructions.values().any(|construction| {
            construction.fields.iter().any(|(field, value)| {
                let layer = field.0 == "outside_layer"
                    || (2..=10).any(|index| field.0 == format!("layer_{index}"));
                layer && matches!(value, RawValue::String(name) if rows.keys().any(|key| key.0.eq_ignore_ascii_case(name)))
            })
        })
    }) {
        return Err("Source SimpleGlazing material-only route excludes every Construction reference".into());
    }
    Ok(())
}
