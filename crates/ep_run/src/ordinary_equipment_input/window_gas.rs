//! Pure unused named-Gas Source prerequisite; no runtime property calculation.
use ep_raw_model::{FieldName, ObjectType, RawModel, RawValue};

pub(super) fn valid_name(name: &str) -> bool {
    // Official ^.*\S.*$ is evaluated by default RE2, not Unicode trim.
    // Dot excludes LF; Perl whitespace is HT/LF/FF/CR/space, not VT/NBSP.
    !name.contains('\n')
        && name
            .bytes()
            .any(|byte| !matches!(byte, b'\t' | b'\n' | 0x0c | b'\r' | b' '))
}

pub(crate) fn known_field(name: &str) -> bool {
    matches!(
        name,
        "gas_type"
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

pub(super) fn admitted_field(name: &str, value: &RawValue) -> bool {
    // Native's instance schema permits unknown members. This predicate is
    // called only for the direct-epJSON, unused named-Gas Source family.
    if !known_field(name) {
        return true;
    }
    if name == "gas_type" {
        return matches!(value, RawValue::String(value)
            if matches!(value.as_str(), "Air" | "Argon" | "Krypton" | "Xenon"));
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    if ep_model::classify_source_json_number(token).is_err() {
        return false;
    }
    // Schema bounds concern the original number, not IP's later signed-i64
    // conversion. Named records still validate every supplied custom field.
    token.parse::<f64>().ok().is_some_and(|value| {
        value.is_finite()
            && match name {
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
    let Some(rows) = raw.objects.get(&ObjectType("WindowMaterial:Gas".into())) else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    if !raw.is_epjson() {
        return Err("Source unused named WindowMaterial:Gas requires direct epJSON".into());
    }
    for (name, object) in rows {
        if !valid_name(&name.0) {
            return Err("Source named Gas requires an actual material name".into());
        }
        for required in ["gas_type", "thickness"] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !admitted_field(required, value))
            {
                return Err(format!("Source named Gas requires schema-valid {required}"));
            }
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !admitted_field(&field.0, value))
        {
            return Err(
                "Source named Gas requires every supplied schema field to remain valid".into(),
            );
        }
    }
    if raw
        .objects
        .get(&ObjectType("Construction".into()))
        .is_some_and(|constructions| {
            constructions.values().any(|construction| {
                construction.fields.iter().any(|(field, value)| {
                    let layer = field.0 == "outside_layer"
                        || (2..=10).any(|index| field.0 == format!("layer_{index}"));
                    layer
                        && matches!(value, RawValue::String(name)
                    if rows.keys().any(|key| key.0.eq_ignore_ascii_case(name)))
                })
            })
        })
    {
        return Err("Source unused named Gas excludes every Construction reference".into());
    }
    Ok(())
}
