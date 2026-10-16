//! Source-backed local arguments; IP remains the sole count/identity/unused owner.
use super::super::selected_project_data::{ProjectInputObject, ProjectInputValue};
use super::{HeatBalanceInputError, SelectedInputProcessor, SourceInputProcessorShortCut};
use std::collections::BTreeMap;

/// Actual raw scalar operands for the selected HB getters, including Zone.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectedHeatBalanceInput {
    /// Actual loader custody; IDF max fields are never inferred from JSON keys.
    pub source_is_epjson: bool,
    /// Raw pre-transformation Surface operands from the same admitted model.
    pub surface: super::surface::SelectedSurfaceInput,
    /// Genuine per-family getObjectItem order from the same admitted raw model.
    pub objects: BTreeMap<String, Vec<ProjectInputObject>>,
}
#[derive(Clone, Copy)]
pub(super) enum DefaultValue {
    Alpha(&'static str),
    Numeric(f64),
}
#[derive(Clone, Copy)]
pub(super) struct Field {
    pub(super) json: &'static str,
    pub(super) legacy: &'static str,
    pub(super) default: DefaultValue,
    pub(super) retain_case: bool,
}
mod design_day_schema;
pub(crate) mod local_getter;
mod mixed_getter;
mod schema;
mod shared_getter;
pub(crate) struct GetterValues<const A: usize, const N: usize> {
    pub(crate) alphas: [String; A],
    pub(crate) numbers: [f64; N],
    pub(crate) num_alphas: usize,
    pub(crate) num_numbers: usize,
}
impl SelectedHeatBalanceInput {
    pub(crate) fn get_object_item<const A: usize, const N: usize>(
        &self,
        family: &str,
        index: usize,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<GetterValues<A, N>, HeatBalanceInputError> {
        let (fields, min_fields, has_name) = schema::fields(family).ok_or(
            HeatBalanceInputError::Unavailable("unrepresented HB getter family"),
        )?;
        let objects = self
            .objects
            .get(family)
            .ok_or(HeatBalanceInputError::Unavailable(
                "missing genuine HB raw operands",
            ))?;
        if input.get_num_objects_found(family) != objects.len() {
            return Err(HeatBalanceInputError::Unavailable(
                "HB raw/IP count association",
            ));
        }
        let object = objects
            .get(index)
            .ok_or(HeatBalanceInputError::Unavailable("HB raw object index"))?;
        if !input.has_object(family, &object.object_name) {
            return Err(HeatBalanceInputError::Unavailable(
                "HB raw/IP permanent identity",
            ));
        }
        let max_fields = if self.source_is_epjson {
            fields.iter().enumerate().fold(min_fields, |n, (i, f)| {
                if object.fields.contains_key(f.json) {
                    n.max(i + 1)
                } else {
                    n
                }
            })
        } else {
            object.idf_max_fields.unwrap_or(0)
        };
        let alpha_count = fields
            .iter()
            .filter(|f| matches!(f.default, DefaultValue::Alpha(_)))
            .count();
        let numeric_count = fields.len() - alpha_count;
        if A < alpha_count
            || N < numeric_count
            || shortcut.alpha_field_names.len() < alpha_count
            || shortcut.alpha_field_blanks.len() < alpha_count
            || shortcut.numeric_field_names.len() < numeric_count
            || shortcut.numeric_field_blanks.len() < numeric_count
        {
            return Err(HeatBalanceInputError::Unavailable(
                "HB actual local/shared allocation",
            ));
        }
        let mut values = GetterValues {
            alphas: std::array::from_fn(|_| String::new()),
            numbers: [0.0; N],
            num_alphas: 0,
            num_numbers: 0,
        };
        shortcut.alpha_field_blanks.fill(true);
        shortcut.numeric_field_blanks.fill(true);
        shortcut.alpha_field_names.fill(String::new());
        shortcut.numeric_field_names.fill(String::new());
        input.mark_object_as_used(family, &object.object_name);
        let (mut alpha, mut numeric) = (0, 0);
        for (position, field) in fields.iter().enumerate() {
            let label = if self.source_is_epjson {
                field.json
            } else {
                field.legacy
            };
            match field.default {
                DefaultValue::Alpha(default) => {
                    let (value, blank) = if has_name && field.json == "name" {
                        (object.object_name.as_str(), object.object_name.is_empty())
                    } else {
                        match object.fields.get(field.json) {
                            None => (default, true),
                            Some(ProjectInputValue::String(v)) if v.is_empty() => (default, true),
                            Some(ProjectInputValue::String(v)) => (v.as_str(), false),
                            Some(ProjectInputValue::Number(_)) => {
                                return Err(HeatBalanceInputError::Unavailable(
                                    "HB alpha numeric JSON representation",
                                ));
                            }
                        }
                    };
                    values.alphas[alpha] = if field.retain_case {
                        value.into()
                    } else {
                        value.to_ascii_uppercase()
                    };
                    shortcut.alpha_field_names[alpha] = label.into();
                    shortcut.alpha_field_blanks[alpha] = blank;
                    alpha += 1;
                    if has_name && field.json == "name" {
                        values.num_alphas += 1;
                    } else if position < max_fields {
                        values.num_alphas = alpha;
                    }
                }
                DefaultValue::Numeric(default) => {
                    let (value, blank) = match object.fields.get(field.json) {
                        None => (default, true),
                        Some(ProjectInputValue::Number(v)) => (*v, false),
                        Some(ProjectInputValue::String(v)) if v.is_empty() => (default, true),
                        Some(ProjectInputValue::String(_)) => {
                            return Err(HeatBalanceInputError::Unavailable(
                                "HB numeric autosize/autocalculate",
                            ));
                        }
                    };
                    values.numbers[numeric] = value;
                    shortcut.numeric_field_names[numeric] = label.into();
                    shortcut.numeric_field_blanks[numeric] = blank;
                    numeric += 1;
                    if position < max_fields {
                        values.num_numbers = numeric;
                    }
                }
            }
        }
        Ok(values)
    }
}
