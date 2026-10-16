//! Actual getObjectItem932-1137 with all six shared-shortcut output arrays supplied.
//! Reuses the same admitted ordered raw operands and literal field metadata.
use super::{
    DefaultValue, HeatBalanceInputError as Error, ProjectInputValue, SelectedHeatBalanceInput,
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
pub(crate) struct SourceSharedObjectItemCounts {
    pub(crate) num_alphas: usize,
    pub(crate) num_numbers: usize,
    pub(crate) io_status: i32,
}
impl SelectedHeatBalanceInput {
    pub(crate) fn get_shared_source_object_item(
        &self,
        family: &str,
        number: usize,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<SourceSharedObjectItemCounts, Error> {
        let (fields, min_fields, has_name) = super::schema::fields(family).ok_or(
            Error::Unavailable("actual shared getter literal field family"),
        )?;
        let objects = self.objects.get(family).ok_or(Error::Unavailable(
            "actual shared getter ordered raw objects",
        ))?;
        if objects.len() != input.get_num_objects_found(family) {
            return Err(Error::Unavailable(
                "actual shared getter raw/IP object count",
            ));
        }
        let object = objects.get(number).ok_or(Error::Unavailable(
            "actual shared getter Source object ordinal",
        ))?;
        if !input.has_object(family, &object.object_name) {
            return Err(Error::Unavailable(
                "actual shared getter permanent raw/IP identity",
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
                "actual shared getter staged IDF max_fields",
            ))?
        };
        let alpha_count = fields
            .iter()
            .filter(|f| matches!(f.default, DefaultValue::Alpha(_)))
            .count();
        let numeric_count = fields.len() - alpha_count;
        if shortcut.alpha_args.len() < alpha_count
            || shortcut.alpha_field_blanks.len() < alpha_count
            || shortcut.alpha_field_names.len() < alpha_count
            || shortcut.numeric_args.len() < numeric_count
            || shortcut.numeric_field_blanks.len() < numeric_count
            || shortcut.numeric_field_names.len() < numeric_count
        {
            return Err(Error::Unavailable(
                "actual shared getter earlier getMaxSchemaArgs allocation",
            ));
        }
        // Native1011-1023 exact argument/blank/name reset order.
        shortcut.alpha_args.fill(String::new());
        shortcut.numeric_args.fill(0.0);
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
                        shortcut.alpha_args[alpha] = if field.retain_case {
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
                        Some(ProjectInputValue::String(s)) if s.is_empty() => (default, true),
                        Some(ProjectInputValue::String(s)) => (s.as_str(), false),
                        Some(ProjectInputValue::Number(_)) => {
                            return Err(Error::Unavailable(
                                "actual shared getter numeric-to-alpha conversion",
                            ));
                        }
                    };
                    shortcut.alpha_args[alpha] = if field.retain_case {
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
                        Some(ProjectInputValue::String(s)) if s.is_empty() => (default, true),
                        Some(ProjectInputValue::Number(n)) => (*n, false),
                        Some(ProjectInputValue::String(_)) => {
                            return Err(Error::Unavailable(
                                "actual shared getter autosize/autocalculate conversion",
                            ));
                        }
                    };
                    shortcut.numeric_args[numeric] = value;
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
