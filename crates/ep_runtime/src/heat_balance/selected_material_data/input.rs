//! Raw selected material operands; properties remain the existing typed owners.
use super::super::selected_project_data::{ProjectInputObject, ProjectInputValue};
use super::{MaterialDataOwners, unavailable};
use crate::error::RuntimeError;

/// Literal selected Material/Material:NoMass raw fields and effective input order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectedMaterialInput {
    /// Actual loader format; never inferred from one family's metadata.
    pub source_is_epjson: bool,
    /// Regular direct-JSON iteration order returned by getIDFOrderedKeys.
    pub regular: Vec<ProjectInputObject>,
    /// Actual getObjectItem order for Material:NoMass.
    pub no_mass: Vec<ProjectInputObject>,
}

pub(super) fn alpha<'a>(
    object: &'a ProjectInputObject,
    field: &str,
) -> Result<&'a str, RuntimeError> {
    match object.fields.get(field) {
        Some(ProjectInputValue::String(value)) if !value.is_empty() => Ok(value),
        _ => Err(unavailable("required admitted material alpha")),
    }
}

pub(super) fn numeric(
    object: &ProjectInputObject,
    field: &str,
    default: Option<f64>,
) -> Result<f64, RuntimeError> {
    match object.fields.get(field) {
        Some(ProjectInputValue::Number(value)) if value.is_finite() => Ok(*value),
        None | Some(ProjectInputValue::String(_))
            if object.fields.get(field).is_none_or(
                |value| matches!(value, ProjectInputValue::String(text) if text.is_empty()),
            ) =>
        {
            default.ok_or_else(|| unavailable("required admitted material numeric"))
        }
        _ => Err(unavailable("selected material JSON scalar")),
    }
}

impl SelectedMaterialInput {
    pub(super) fn get_no_mass_item(
        &self,
        object: &ProjectInputObject,
        owners: &mut MaterialDataOwners<'_, '_>,
    ) -> Result<(usize, usize), RuntimeError> {
        let fields = [
            "name",
            "roughness",
            "thermal_resistance",
            "thermal_absorptance",
            "solar_absorptance",
            "visible_absorptance",
        ];
        let legacy = [
            "Name",
            "Roughness",
            "Thermal Resistance",
            "Thermal Absorptance",
            "Solar Absorptance",
            "Visible Absorptance",
        ];
        let max_fields = if self.source_is_epjson {
            fields.iter().enumerate().fold(3, |count, (index, field)| {
                if object.fields.contains_key(*field) {
                    count.max(index + 1)
                } else {
                    count
                }
            })
        } else {
            object
                .idf_max_fields
                .ok_or_else(|| unavailable("missing actual Material:NoMass IDF field count"))?
        };
        if !owners
            .input
            .has_object("Material:NoMass", &object.object_name)
        {
            return Err(unavailable("Material:NoMass raw/IP identity"));
        }
        let s = &mut owners.shortcut;
        if s.alpha_args.len() < 2
            || s.numeric_args.len() < 4
            || s.alpha_field_blanks.len() < 2
            || s.alpha_field_names.len() < 2
            || s.numeric_field_blanks.len() < 4
            || s.numeric_field_names.len() < 4
        {
            return Err(unavailable(
                "actual complete Material:NoMass shared allocation",
            ));
        }
        // InputProcessor993-1013 resets ALL six passed arrays, not just used slots.
        s.alpha_args.fill(String::new());
        s.numeric_args.fill(0.0);
        s.alpha_field_blanks.fill(true);
        s.numeric_field_blanks.fill(true);
        s.alpha_field_names.fill(String::new());
        s.numeric_field_names.fill(String::new());
        owners
            .input
            .mark_object_as_used("Material:NoMass", &object.object_name);
        s.alpha_args[0] = object.object_name.to_ascii_uppercase();
        s.alpha_field_blanks[0] = object.object_name.is_empty();
        s.alpha_field_names[0] = if self.source_is_epjson {
            fields[0]
        } else {
            legacy[0]
        }
        .into();
        s.alpha_args[1] = alpha(object, fields[1])?.to_ascii_uppercase();
        s.alpha_field_blanks[1] = false;
        s.alpha_field_names[1] = if self.source_is_epjson {
            fields[1]
        } else {
            legacy[1]
        }
        .into();
        let defaults = [None, Some(0.9), Some(0.7), Some(0.7)];
        let mut num_numbers = 0;
        for (index, default) in defaults.iter().enumerate() {
            let field = fields[index + 2];
            s.numeric_args[index] = numeric(object, field, *default)?;
            s.numeric_field_blanks[index] = object.fields.get(field).is_none_or(
                |value| matches!(value, ProjectInputValue::String(text) if text.is_empty()),
            );
            s.numeric_field_names[index] = if self.source_is_epjson {
                field
            } else {
                legacy[index + 2]
            }
            .into();
            if index + 2 < max_fields {
                num_numbers = index + 1;
            }
        }
        // Name always increments NumAlphas; roughness increments only in range.
        // Both are required by the admitted successful domain; no retained local
        // NumAlphas or IOStat owner is fabricated after the call returns.
        Ok((1 + usize::from(1 < max_fields), num_numbers))
    }
}
