//! Narrow retained raw operands for SimulationManager::GetProjectData.
//! Object counts, schema maxima and unusedInputs stay on SelectedInputProcessor.

use super::{ProjectDataError, SelectedInputProcessor, SourceInputProcessorShortCut};
use std::collections::BTreeMap;

/// A genuine JSON scalar accepted by the four selected getter families.
#[derive(Clone, Debug, PartialEq)]
pub enum ProjectInputValue {
    /// Exact string, including an explicitly empty string.
    String(String),
    /// Actual finite JSON number, without applying defaults.
    Number(f64),
}

/// One actual epJSON object identity and its retained fields.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectInputObject {
    /// Original case-sensitive object-map identity.
    pub object_name: String,
    /// Original JSON field presence and scalar values.
    pub fields: BTreeMap<String, ProjectInputValue>,
    /// Actual retained IDF metadata; absent means Original findMaxFields returns zero.
    pub idf_max_fields: Option<usize>,
}

/// Direct JSON diagnostics are lexical, independent of IDF getObjectItem order.
#[derive(Clone, Debug, PartialEq)]
pub struct ProjectDiagnosticsObject {
    /// Actual case-sensitive map identity.
    pub object_name: String,
    /// Source extensible sequence; None means an actually missing key.
    pub keys: Vec<Option<String>>,
}

/// Selected values from the same fully admitted raw model as the live IP owner.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectedProjectDataInput {
    /// Actual input-loader format, never inferred from field contents.
    pub source_is_epjson: bool,
    /// Only Version, HeatBalanceAlgorithm, Timestep and SimulationControl.
    /// Each vector is the genuine getObjectItem order for that family.
    pub objects: BTreeMap<String, Vec<ProjectInputObject>>,
    /// Complete Output:Diagnostics object map in lexical order.
    pub diagnostics: Vec<ProjectDiagnosticsObject>,
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
mod schema;

pub(super) struct GetterValues {
    pub(super) alphas: [String; 10],
    pub(super) numbers: [f64; 4],
    pub(super) num_alphas: usize,
    pub(super) num_numbers: usize,
}

impl SelectedProjectDataInput {
    pub(super) fn get_object_item(
        &self,
        family: &str,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<GetterValues, ProjectDataError> {
        let (fields, min_fields) = schema::fields(family).ok_or(ProjectDataError::Unavailable(
            "unrepresented selected getter family",
        ))?;
        let objects = self
            .objects
            .get(family)
            .ok_or(ProjectDataError::Unavailable(
                "missing actual getter operands",
            ))?;
        if input.get_num_objects_found(family) != objects.len() {
            return Err(ProjectDataError::Unavailable(
                "raw getter/IP count association",
            ));
        }
        let object = objects.first().ok_or(ProjectDataError::Unavailable(
            "empty actual getter operands",
        ))?;
        if !input.has_object(family, &object.object_name) {
            return Err(ProjectDataError::Unavailable(
                "raw getter/IP object association",
            ));
        }
        let max_fields = if self.source_is_epjson {
            fields
                .iter()
                .enumerate()
                .fold(min_fields, |count, (index, field)| {
                    if object.fields.contains_key(field.json) {
                        count.max(index + 1)
                    } else {
                        count
                    }
                })
        } else {
            object.idf_max_fields.unwrap_or(0)
        };
        let alpha_count = fields
            .iter()
            .filter(|field| matches!(field.default, DefaultValue::Alpha(_)))
            .count();
        let numeric_count = fields.len() - alpha_count;
        if shortcut.alpha_field_blanks.len() < alpha_count
            || shortcut.alpha_field_names.len() < alpha_count
            || shortcut.numeric_field_blanks.len() < numeric_count
            || shortcut.numeric_field_names.len() < numeric_count
        {
            return Err(ProjectDataError::Unavailable(
                "actual shared shortcut allocation",
            ));
        }
        let mut values = GetterValues {
            alphas: std::array::from_fn(|_| String::new()),
            numbers: [0.0; 4],
            num_alphas: 0,
            num_numbers: 0,
        };
        // Exact getObjectItem caller arrays: all shared blanks/names are reset;
        // cAlphaArgs/rNumericArgs/current_module_object are not arguments here.
        shortcut.alpha_field_blanks.fill(true);
        shortcut.numeric_field_blanks.fill(true);
        shortcut.alpha_field_names.fill(String::new());
        shortcut.numeric_field_names.fill(String::new());
        input.mark_object_as_used(family, &object.object_name);
        let (mut alpha, mut numeric) = (0, 0);
        for (position, field) in fields.iter().enumerate() {
            let field_name = if self.source_is_epjson {
                field.json
            } else {
                field.legacy
            };
            match field.default {
                DefaultValue::Alpha(default) => {
                    let (value, blank) = match object.fields.get(field.json) {
                        None => (default, true),
                        Some(ProjectInputValue::String(value)) if value.is_empty() => {
                            (default, true)
                        }
                        Some(ProjectInputValue::String(value)) => (value.as_str(), false),
                        Some(ProjectInputValue::Number(_)) => {
                            return Err(ProjectDataError::Unavailable(
                                "selected alpha JSON number representation",
                            ));
                        }
                    };
                    values.alphas[alpha] = if field.retain_case {
                        value.into()
                    } else {
                        value.to_ascii_uppercase()
                    };
                    shortcut.alpha_field_blanks[alpha] = blank;
                    shortcut.alpha_field_names[alpha] = field_name.into();
                    alpha += 1;
                    if position < max_fields {
                        values.num_alphas = alpha;
                    }
                }
                DefaultValue::Numeric(default) => {
                    let (value, blank) = match object.fields.get(field.json) {
                        None => (default, true),
                        Some(ProjectInputValue::Number(value)) => (*value, false),
                        Some(ProjectInputValue::String(value)) if value.is_empty() => {
                            (default, true)
                        }
                        Some(ProjectInputValue::String(_)) => {
                            return Err(ProjectDataError::Unavailable(
                                "selected numeric autosize/autocalculate",
                            ));
                        }
                    };
                    values.numbers[numeric] = value;
                    shortcut.numeric_field_blanks[numeric] = blank;
                    shortcut.numeric_field_names[numeric] = field_name.into();
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
