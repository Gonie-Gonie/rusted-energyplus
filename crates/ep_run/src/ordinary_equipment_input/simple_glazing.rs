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
    raw.validate_source_idf_object_custody("WindowMaterial:SimpleGlazingSystem")?;
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

// Ordinary SpectralAverage is the same Native MaterialGlass class, but has
// its own raw declaration and getter; it never inherits SGS Setup.
pub(crate) fn glazing_known_field(field: &str) -> bool {
    matches!(
        field,
        "optical_data_type"
            | "window_glass_spectral_data_set_name"
            | "thickness"
            | "solar_transmittance_at_normal_incidence"
            | "front_side_solar_reflectance_at_normal_incidence"
            | "back_side_solar_reflectance_at_normal_incidence"
            | "visible_transmittance_at_normal_incidence"
            | "front_side_visible_reflectance_at_normal_incidence"
            | "back_side_visible_reflectance_at_normal_incidence"
            | "infrared_transmittance_at_normal_incidence"
            | "front_side_infrared_hemispherical_emissivity"
            | "back_side_infrared_hemispherical_emissivity"
            | "conductivity"
            | "dirt_correction_factor_for_solar_and_visible_transmittance"
            | "solar_diffusing"
            | "young_s_modulus"
            | "poisson_s_ratio"
            | "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name"
            | "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name"
            | "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name"
    )
}

pub(super) fn glazing_admitted_field(field: &str, value: &RawValue) -> bool {
    if field == "optical_data_type" {
        return matches!(value, RawValue::String(value) if matches!(value.as_str(),"SpectralAverage"|"Spectral"|"SpectralAndAngle"));
    }
    if field == "solar_diffusing" {
        return matches!(value, RawValue::String(value) if matches!(value.as_str(), "" | "No" | "Yes"));
    }
    if matches!(
        field,
        "window_glass_spectral_data_set_name"
            | "window_glass_spectral_and_incident_angle_transmittance_data_set_table_name"
            | "window_glass_spectral_and_incident_angle_front_reflectance_data_set_table_name"
            | "window_glass_spectral_and_incident_angle_back_reflectance_data_set_table_name"
    ) {
        return matches!(value, RawValue::String(_));
    }
    if !matches!(
        field,
        "thickness"
            | "solar_transmittance_at_normal_incidence"
            | "front_side_solar_reflectance_at_normal_incidence"
            | "back_side_solar_reflectance_at_normal_incidence"
            | "visible_transmittance_at_normal_incidence"
            | "front_side_visible_reflectance_at_normal_incidence"
            | "back_side_visible_reflectance_at_normal_incidence"
            | "infrared_transmittance_at_normal_incidence"
            | "front_side_infrared_hemispherical_emissivity"
            | "back_side_infrared_hemispherical_emissivity"
            | "conductivity"
            | "dirt_correction_factor_for_solar_and_visible_transmittance"
            | "young_s_modulus"
            | "poisson_s_ratio"
    ) {
        return true; // Native ignores unknown direct-JSON members; Raw/IP keep them.
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|number| {
            number.is_finite()
                && match field {
                    "thickness" | "conductivity" | "young_s_modulus" => number > 0.0,
                    "front_side_infrared_hemispherical_emissivity"
                    | "back_side_infrared_hemispherical_emissivity"
                    | "poisson_s_ratio" => number > 0.0 && number < 1.0,
                    "dirt_correction_factor_for_solar_and_visible_transmittance" => {
                        number > 0.0 && number <= 1.0
                    }
                    _ => (0.0..=1.0).contains(&number),
                }
        })
}

pub(crate) fn validate_unused_spectral_average(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw
        .objects
        .get(&ObjectType("WindowMaterial:Glazing".into()))
    else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    raw.validate_source_idf_object_custody("WindowMaterial:Glazing")?;
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0) {
            return Err("Source Glazing requires the actual schema material name".into());
        }
        if ["optical_data_type", "thickness"].iter().any(|field| {
            object
                .fields
                .get(&FieldName((*field).into()))
                .is_none_or(|value| !glazing_admitted_field(field, value))
        }) || object
            .fields
            .iter()
            .any(|(field, value)| !glazing_admitted_field(&field.0, value))
        {
            return Err(
                "Source Glazing requires original schema-valid SpectralAverage, Spectral or SpectralAndAngle operands".into(),
            );
        }
    }
    if raw.objects.get(&ObjectType("Construction".into())).is_some_and(|constructions| {
        constructions.values().any(|construction| construction.fields.iter().any(|(field, value)| {
            let layer = field.0 == "outside_layer" || (2..=10).any(|index| field.0 == format!("layer_{index}"));
            layer && matches!(value, RawValue::String(reference) if rows.keys().any(|key| key.0.eq_ignore_ascii_case(reference)))
        }))
    }) {
        return Err("Source Glazing material-only route excludes every Construction reference".into());
    }
    Ok(())
}

// Native ordinary alternative MaterialGlass declaration; unknown direct-JSON
// metadata (including optical_data_type) is not a Refraction getter operand.
pub(crate) fn refraction_known_field(field: &str) -> bool {
    matches!(
        field,
        "thickness"
            | "solar_index_of_refraction"
            | "solar_extinction_coefficient"
            | "visible_index_of_refraction"
            | "visible_extinction_coefficient"
            | "infrared_transmittance_at_normal_incidence"
            | "infrared_hemispherical_emissivity"
            | "conductivity"
            | "dirt_correction_factor_for_solar_and_visible_transmittance"
            | "solar_diffusing"
    )
}
pub(super) fn refraction_admitted_field(field: &str, value: &RawValue) -> bool {
    if field == "solar_diffusing" {
        return matches!(value, RawValue::String(value) if matches!(value.as_str(), "" | "No" | "Yes"));
    }
    if !matches!(
        field,
        "thickness"
            | "solar_index_of_refraction"
            | "solar_extinction_coefficient"
            | "visible_index_of_refraction"
            | "visible_extinction_coefficient"
            | "infrared_transmittance_at_normal_incidence"
            | "infrared_hemispherical_emissivity"
            | "conductivity"
            | "dirt_correction_factor_for_solar_and_visible_transmittance"
    ) {
        return true;
    }
    let RawValue::Number(token) = value else {
        return false;
    };
    ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(|number| {
            number.is_finite()
                && match field {
                    "solar_index_of_refraction" | "visible_index_of_refraction" => number > 1.0,
                    "infrared_transmittance_at_normal_incidence" => (0.0..1.0).contains(&number),
                    "infrared_hemispherical_emissivity" => number > 0.0 && number < 1.0,
                    "dirt_correction_factor_for_solar_and_visible_transmittance" => {
                        number > 0.0 && number <= 1.0
                    }
                    _ => number > 0.0,
                }
        })
}
pub(crate) fn validate_unused_refraction(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw.objects.get(&ObjectType(
        "WindowMaterial:Glazing:RefractionExtinctionMethod".into(),
    )) else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    raw.validate_source_idf_object_custody("WindowMaterial:Glazing:RefractionExtinctionMethod")?;
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0) {
            return Err("Source Refraction requires actual schema material name".into());
        }
        if [
            "thickness",
            "solar_index_of_refraction",
            "solar_extinction_coefficient",
            "visible_index_of_refraction",
            "visible_extinction_coefficient",
        ]
        .iter()
        .any(|field| {
            object
                .fields
                .get(&FieldName((*field).into()))
                .is_none_or(|value| !refraction_admitted_field(field, value))
        }) || object
            .fields
            .iter()
            .any(|(field, value)| !refraction_admitted_field(&field.0, value))
        {
            return Err("Source Refraction requires original schema-valid known fields".into());
        }
    }
    if raw.objects.get(&ObjectType("Construction".into())).is_some_and(|constructions| {
        constructions.values().any(|construction| construction.fields.iter().any(|(field,value)| {
            let layer = field.0 == "outside_layer" || (2..=10).any(|index| field.0 == format!("layer_{index}"));
            layer && matches!(value, RawValue::String(reference) if rows.keys().any(|key| key.0.eq_ignore_ascii_case(reference)))
        }))
    }) { return Err("Source Refraction material-only route excludes every Construction reference".into()); }
    Ok(())
}

pub(crate) fn spectral_data_known_field(field: &str) -> bool {
    field == "extensions"
        || (1..=4).any(|i| {
            [
                "wavelength",
                "transmittance",
                "front_reflectance",
                "back_reflectance",
            ]
            .iter()
            .any(|name| field == format!("{name}_{i}"))
        })
}
pub(super) fn spectral_data_admitted_field(field: &str, value: &RawValue) -> bool {
    let number = |value: &RawValue| {
        matches!(value,RawValue::Number(token)
        if ep_model::classify_source_json_number(token).is_ok()
        && token.parse::<f64>().ok().is_some_and(f64::is_finite))
    };
    if field == "extensions" {
        let RawValue::Array(rows) = value else {
            return false;
        };
        return rows.iter().all(|row| {
            let RawValue::Object(fields) = row else {
                return false;
            };
            fields.iter().all(|(field, value)| {
                !matches!(
                    field.0.as_str(),
                    "wavelength" | "transmittance" | "front_reflectance" | "back_reflectance"
                ) || number(value)
            })
        });
    }
    !spectral_data_known_field(field) || number(value)
}
pub(crate) fn validate_spectral_data(raw: &RawModel) -> Result<(), String> {
    let Some(rows) = raw
        .objects
        .get(&ObjectType("MaterialProperty:GlazingSpectralData".into()))
    else {
        return Ok(());
    };
    if rows.is_empty() {
        return Ok(());
    }
    raw.validate_source_idf_object_custody("MaterialProperty:GlazingSpectralData")?;
    for (name, object) in rows {
        if !super::window_gas::valid_name(&name.0)
            || object
                .fields
                .iter()
                .any(|(field, value)| !spectral_data_admitted_field(&field.0, value))
        {
            return Err("Source spectral data requires original schema names/numeric fields/extension objects".into());
        }
    }
    Ok(())
}
