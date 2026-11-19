//! Pure unused single-Gas Source prerequisite; actual properties belong to the getter.
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
    // called only for the direct-epJSON, unused single-Gas Source family.
    if !known_field(name) {
        return true;
    }
    if name == "gas_type" {
        return matches!(value, RawValue::String(value)
            if matches!(value.as_str(), "Air" | "Argon" | "Custom" | "Krypton" | "Xenon"));
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
    raw.validate_source_idf_object_custody("WindowMaterial:Gas")?;
    for (name, object) in rows {
        if !valid_name(&name.0) {
            return Err("Source Gas requires an actual material name".into());
        }
        for required in ["gas_type", "thickness"] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !admitted_field(required, value))
            {
                return Err(format!("Source Gas requires schema-valid {required}"));
            }
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !admitted_field(&field.0, value))
        {
            return Err("Source Gas requires every supplied schema field to remain valid".into());
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
        return Err("Source unused Gas excludes every Construction reference".into());
    }
    Ok(())
}

pub(crate) fn mixture_known_field(name: &str) -> bool {
    matches!(
        name,
        "thickness"
            | "number_of_gases_in_mixture"
            | "gas_1_type"
            | "gas_1_fraction"
            | "gas_2_type"
            | "gas_2_fraction"
            | "gas_3_type"
            | "gas_3_fraction"
            | "gas_4_type"
            | "gas_4_fraction"
    )
}

pub(super) fn mixture_admitted_field(name: &str, value: &RawValue) -> bool {
    if !mixture_known_field(name) {
        return true;
    }
    if matches!(
        name,
        "gas_1_type" | "gas_2_type" | "gas_3_type" | "gas_4_type"
    ) {
        return matches!(value, RawValue::String(value)
            if matches!(value.as_str(), "Air" | "Argon" | "Krypton" | "Xenon"));
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    let Ok(effective) = ep_model::classify_source_json_number(token) else {
        return false;
    };
    if name == "number_of_gases_in_mixture" {
        // Strict Native JSON integer schema rejects integral Float tokens too.
        return matches!(effective, ep_model::SourceJsonNumber::Integer(value) if (1..=4).contains(&value));
    }
    token.parse::<f64>().ok().is_some_and(|value| {
        value.is_finite() && value > 0.0 && (name == "thickness" || value <= 1.0)
    })
}

pub(crate) fn validate_mixture(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw
        .objects
        .get(&ObjectType("WindowMaterial:GasMixture".into()))
    else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    raw.validate_source_idf_object_custody("WindowMaterial:GasMixture")?;
    for (name, object) in rows {
        if !valid_name(&name.0) {
            return Err("Source GasMixture requires an actual material name".into());
        }
        for required in [
            "thickness",
            "number_of_gases_in_mixture",
            "gas_1_type",
            "gas_1_fraction",
            "gas_2_type",
            "gas_2_fraction",
        ] {
            if object
                .fields
                .get(&FieldName(required.into()))
                .is_none_or(|value| !mixture_admitted_field(required, value))
            {
                return Err(format!(
                    "Source GasMixture requires schema-valid {required}"
                ));
            }
        }
        if object
            .fields
            .iter()
            .any(|(field, value)| !mixture_admitted_field(&field.0, value))
        {
            return Err(
                "Source GasMixture requires every supplied schema field to remain valid".into(),
            );
        }
        let count = match object
            .fields
            .get(&FieldName("number_of_gases_in_mixture".into()))
        {
            Some(RawValue::Number(token)) => ep_model::classify_source_json_number(token)
                .ok()
                .and_then(|value| value.as_i32()),
            _ => None,
        }
        .ok_or("Source GasMixture integer count custody")?;
        for index in 3..=count {
            if !object
                .fields
                .contains_key(&FieldName(format!("gas_{index}_type")))
            {
                // Native would subsequently index its fixed gas array by Invalid.
                return Err(
                    "Source GasMixture missing active type has undefined Native indexing".into(),
                );
            }
        }
    }
    if raw.objects.get(&ObjectType("Construction".into())).is_some_and(|constructions| {
        constructions.values().any(|construction| construction.fields.iter().any(|(field, value)| {
            let layer = field.0 == "outside_layer" || (2..=10).any(|index| field.0 == format!("layer_{index}"));
            layer && matches!(value, RawValue::String(name) if rows.keys().any(|key| key.0.eq_ignore_ascii_case(name)))
        }))
    }) {
        return Err("Source unused GasMixture excludes every Construction reference".into());
    }
    Ok(())
}
