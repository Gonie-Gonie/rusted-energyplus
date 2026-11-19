//! Original-schema unused WindowMaterial:Gap prerequisite; actual copies and diagnostics belong to GetMaterialData.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

pub(crate) fn known_field(family: &str, field: &str) -> bool {
    match family {
        "WindowMaterial:Gap" => matches!(
            field,
            "thickness"
                | "gas_or_gas_mixture_"
                | "pressure"
                | "deflection_state"
                | "support_pillar"
        ),
        "WindowGap:DeflectionState" => matches!(
            field,
            "deflected_thickness" | "initial_temperature" | "initial_pressure"
        ),
        "WindowGap:SupportPillar" => matches!(field, "spacing" | "radius"),
        _ => false,
    }
}
pub(super) fn admitted_field(family: &str, field: &str, value: &RawValue) -> bool {
    if !known_field(family, field) {
        return true;
    }
    if family == "WindowMaterial:Gap"
        && matches!(
            field,
            "gas_or_gas_mixture_" | "deflection_state" | "support_pillar"
        )
    {
        return matches!(value, RawValue::String(_));
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|value| {
            value.is_finite()
                && match (family, field) {
                    ("WindowMaterial:Gap", "thickness") | ("WindowGap:SupportPillar", _) => {
                        value > 0.0
                    }
                    ("WindowGap:DeflectionState", _) => value >= 0.0,
                    _ => true, // Pressure has no original positive bound.
                }
        })
}
pub(crate) fn validate(raw: &RawModel) -> Result<(), String> {
    for family in [
        "WindowMaterial:Gap",
        "WindowGap:DeflectionState",
        "WindowGap:SupportPillar",
    ] {
        let Some(rows) = raw.objects.get(&ObjectType(family.into())) else {
            continue;
        };
        if rows.is_empty() {
            continue;
        }
        raw.validate_source_idf_object_custody(family)?;
        raw.source_json_instances(family)?;
        for (name, object) in rows {
            if !super::window_gas::valid_name(&name.0) {
                return Err("Source WindowGap requires original RE2 object names".into());
            }
            if family == "WindowMaterial:Gap" {
                for field in ["thickness", "gas_or_gas_mixture_"] {
                    if object
                        .fields
                        .get(&FieldName(field.into()))
                        .is_none_or(|value| !admitted_field(family, field, value))
                    {
                        return Err(format!("Source WindowGap requires schema-valid {field}"));
                    }
                }
            }
            if object
                .fields
                .iter()
                .any(|(field, value)| !admitted_field(family, &field.0, value))
            {
                return Err(
                    "Source WindowGap requires original-schema known field kinds and bounds".into(),
                );
            }
        }
    }
    let Some(gaps) = raw.objects.get(&ObjectType("WindowMaterial:Gap".into())) else {
        return Ok(());
    };
    for object in gaps.values() {
        if let Some(RawValue::String(reference)) =
            object.fields.get(&FieldName("gas_or_gas_mixture_".into()))
            && !reference.is_empty()
            && raw.objects.iter().any(|(family, rows)| {
                matches!(
                    family.0.as_str(),
                    "Material"
                        | "Material:NoMass"
                        | "Material:InfraredTransparent"
                        | "Material:AirGap"
                        | "WindowMaterial:Glazing"
                        | "WindowMaterial:Glazing:RefractionExtinctionMethod"
                        | "WindowMaterial:Glazing:EquivalentLayer"
                        | "WindowMaterial:Shade"
                        | "WindowMaterial:Shade:EquivalentLayer"
                        | "WindowMaterial:Drape:EquivalentLayer"
                        | "WindowMaterial:Screen"
                        | "WindowMaterial:Screen:EquivalentLayer"
                        | "WindowMaterial:Blind"
                        | "WindowMaterial:Blind:EquivalentLayer"
                        | "Material:RoofVegetation"
                        | "WindowMaterial:GlazingGroup:Thermochromic"
                        | "WindowMaterial:SimpleGlazingSystem"
                ) && rows.keys().any(|key| key.0.eq_ignore_ascii_case(reference))
            })
        {
            return Err("Source WindowGap excludes undefined Native non-GasMix reference".into());
        }
    }
    if raw.objects.iter().filter(|(family, _)| family.0.starts_with("Construction")).any(|(_, rows)| {
        rows.values().any(|object| object.fields.iter().any(|(field, value)| {
            let layer = field.0 == "outside_layer" || (2..=11).any(|n| field.0 == format!("layer_{n}"))
                || field.0 == "outside_layer_name" || (2..=5).any(|n| field.0 == format!("layer_{n}_name"))
                || (1..=4).any(|n| field.0 == format!("gap_{n}_name"));
            layer && matches!(value, RawValue::String(reference) if gaps.keys().any(|key| key.0.eq_ignore_ascii_case(reference)))
        }))
    }) { return Err("Source unused WindowGap excludes every Construction layer reference".into()); }
    Ok(())
}
