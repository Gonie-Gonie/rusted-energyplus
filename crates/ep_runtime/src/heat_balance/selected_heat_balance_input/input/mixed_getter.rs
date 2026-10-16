//! Actual getObjectItem with local alpha/numeric arrays and shared blank/name arrays.
//! Rust numeric availability is populated by the genuine getter reset at1012.
use super::shared_getter::SourceSharedObjectItemCounts;
use super::{
    DefaultValue, HeatBalanceInputError as Error, ProjectInputValue, SelectedHeatBalanceInput,
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
impl SelectedHeatBalanceInput {
    pub(crate) fn get_mixed_source_object_item<const A: usize, const N: usize>(
        &self,
        family: &str,
        number: usize,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        arguments: (&mut [String; A], &mut [Option<f64>; N]),
    ) -> Result<SourceSharedObjectItemCounts, Error> {
        let (alphas, numbers) = arguments;
        let (fields, min_fields, has_name) = super::schema::fields(family).ok_or(
            Error::Unavailable("actual mixed getObjectItem field family"),
        )?;
        let objects = self.objects.get(family).ok_or(Error::Unavailable(
            "actual mixed getObjectItem ordered raw operands",
        ))?;
        if objects.len() != input.get_num_objects_found(family) {
            return Err(Error::Unavailable("actual mixed getter raw/IP count"));
        }
        let object = objects.get(number).ok_or(Error::Unavailable(
            "actual mixed getter Source object ordinal",
        ))?;
        if !input.has_object(family, &object.object_name) {
            return Err(Error::Unavailable(
                "actual mixed getter permanent raw/IP identity",
            ));
        }
        let mut counts = SourceSharedObjectItemCounts {
            num_alphas: 0,
            num_numbers: 0,
            io_status: -1,
        };
        let max_fields = if self.source_is_epjson {
            fields
                .iter()
                .enumerate()
                .fold(min_fields, |max, (i, field)| {
                    if object.fields.contains_key(field.json) {
                        max.max(i + 1)
                    } else {
                        max
                    }
                })
        } else {
            object.idf_max_fields.ok_or(Error::Unavailable(
                "actual mixed getter staged IDF max_fields",
            ))?
        };
        let alpha_count = fields
            .iter()
            .filter(|field| matches!(field.default, DefaultValue::Alpha(_)))
            .count();
        let numeric_count = fields.len() - alpha_count;
        if A < alpha_count
            || N < numeric_count
            || shortcut.numeric_field_blanks.len() < numeric_count
            || shortcut.alpha_field_blanks.len() < alpha_count
            || shortcut.alpha_field_names.len() < alpha_count
            || shortcut.numeric_field_names.len() < numeric_count
        {
            return Err(Error::Unavailable(
                "actual mixed getter earlier local/shared array allocations",
            ));
        }
        alphas.fill(String::new());
        numbers.fill(Some(0.0));
        shortcut.numeric_field_blanks.fill(true);
        shortcut.alpha_field_blanks.fill(true);
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
                    if has_name && field.json == "name" {
                        alphas[alpha] = if field.retain_case {
                            object.object_name.clone()
                        } else {
                            object.object_name.to_ascii_uppercase()
                        };
                        shortcut.alpha_field_blanks[alpha] = object.object_name.is_empty();
                        shortcut.alpha_field_names[alpha] = label.into();
                        counts.num_alphas += 1;
                        alpha += 1;
                        continue;
                    }
                    let (value, blank) = match object.fields.get(field.json) {
                        None => (default, true),
                        Some(ProjectInputValue::String(value)) if value.is_empty() => {
                            (default, true)
                        }
                        Some(ProjectInputValue::String(value)) => (value.as_str(), false),
                        Some(ProjectInputValue::Number(_)) => {
                            return Err(Error::Unavailable(
                                "actual mixed getter numeric-to-alpha conversion",
                            ));
                        }
                    };
                    alphas[alpha] = if field.retain_case {
                        value.into()
                    } else {
                        value.to_ascii_uppercase()
                    };
                    shortcut.alpha_field_blanks[alpha] = blank;
                    if position < max_fields {
                        counts.num_alphas = alpha + 1;
                    }
                    shortcut.alpha_field_names[alpha] = label.into();
                    alpha += 1;
                }
                DefaultValue::Numeric(default) => {
                    let (value, blank) = match object.fields.get(field.json) {
                        None => (default, true),
                        Some(ProjectInputValue::String(value)) if value.is_empty() => {
                            (default, true)
                        }
                        Some(ProjectInputValue::Number(value)) => (*value, false),
                        Some(ProjectInputValue::String(_)) => {
                            return Err(Error::Unavailable(
                                "actual mixed getter autosize/autocalculate conversion",
                            ));
                        }
                    };
                    numbers[numeric] = Some(value);
                    shortcut.numeric_field_blanks[numeric] = blank;
                    if position < max_fields {
                        counts.num_numbers = numeric + 1;
                    }
                    shortcut.numeric_field_names[numeric] = label.into();
                    numeric += 1;
                }
            }
        }
        counts.io_status = 1;
        Ok(counts)
    }
}
