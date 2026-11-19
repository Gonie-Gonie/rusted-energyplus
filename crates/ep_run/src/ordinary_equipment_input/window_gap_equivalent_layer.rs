//! Original-schema unused GapEQL prerequisite; physical GasMix state belongs to real GetMaterialData.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};
pub(crate) fn known_field(field: &str) -> bool {
    matches!(
        field,
        "gas_type"
            | "gap_vent_type"
            | "thickness"
            | "conductivity_coefficient_a"
            | "conductivity_coefficient_b"
            | "conductivity_coefficient_c"
            | "viscosity_coefficient_a"
            | "viscosity_coefficient_b"
            | "viscosity_coefficient_c"
            | "specific_heat_coefficient_a"
            | "specific_heat_coefficient_b"
            | "specific_heat_coefficient_c"
            | "molecular_weight"
            | "specific_heat_ratio"
    )
}
pub(super) fn admitted_field(field: &str, value: &RawValue) -> bool {
    if !matches!(
        field,
        "gas_type"
            | "gap_vent_type"
            | "thickness"
            | "conductivity_coefficient_a"
            | "conductivity_coefficient_b"
            | "conductivity_coefficient_c"
            | "viscosity_coefficient_a"
            | "viscosity_coefficient_b"
            | "viscosity_coefficient_c"
            | "specific_heat_coefficient_a"
            | "specific_heat_coefficient_b"
            | "specific_heat_coefficient_c"
            | "molecular_weight"
            | "specific_heat_ratio"
    ) {
        return true;
    }
    if field == "gas_type" {
        return matches!(value, RawValue::String(text) if matches!(text.as_str(), "AIR" | "ARGON" | "CUSTOM" | "KRYPTON" | "XENON"));
    }
    if field == "gap_vent_type" {
        return matches!(value, RawValue::String(text) if matches!(text.as_str(), "Sealed" | "VentedIndoor" | "VentedOutdoor"));
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|value| {
            value.is_finite()
                && match field {
                    "thickness" | "viscosity_coefficient_a" | "specific_heat_coefficient_a" => {
                        value > 0.0
                    }
                    "molecular_weight" => (20.0..=200.0).contains(&value),
                    "specific_heat_ratio" => value > 1.0,
                    _ => true,
                }
        })
}
pub(crate) fn validate(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw
        .objects
        .get(&ObjectType("WindowMaterial:Gap:EquivalentLayer".into()))
    else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    raw.validate_source_idf_object_custody("WindowMaterial:Gap:EquivalentLayer")?;
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0) {
            return Err("Source GapEQL requires original RE2 material name".into());
        }
        for required in ["gas_type", "thickness", "gap_vent_type"] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !admitted_field(required, value))
            {
                return Err(format!("Source GapEQL requires schema-valid {required}"));
            }
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !admitted_field(&field.0, value))
        {
            return Err(
                "Source GapEQL requires every supplied known field to remain original-schema-valid"
                    .into(),
            );
        }
    }
    if raw.objects.iter().filter(|(family, _)| family.0.starts_with("Construction")).any(|(_, constructions)| {
        constructions.values().any(|construction| construction.fields.iter().any(|(field, value)| {
            let layer = field.0 == "outside_layer" || (2..=10).any(|index| field.0 == format!("layer_{index}"));
            layer && matches!(value, RawValue::String(reference) if rows.keys().any(|key| key.0.eq_ignore_ascii_case(reference)))
        }))
    }) { return Err("Source unused GapEQL excludes every Construction layer reference".into()); }
    Ok(())
}
