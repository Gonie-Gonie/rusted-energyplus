//! Actual getObjectItem with local arguments and all optional outputs absent.
use super::{
    DefaultValue, HeatBalanceInputError as Error, ProjectInputValue, SelectedHeatBalanceInput,
    SelectedInputProcessor,
};

pub(crate) struct SourceLocalObjectArguments<const A: usize, const N: usize> {
    pub(crate) alphas: [String; A],
    pub(crate) numbers: [Option<f64>; N],
    pub(crate) num_alphas: Option<usize>,
    pub(crate) num_numbers: Option<usize>,
    pub(crate) io_status: Option<i32>,
}
impl<const A: usize, const N: usize> SourceLocalObjectArguments<A, N> {
    pub(crate) fn allocated_unwritten() -> Self {
        Self {
            alphas: std::array::from_fn(|_| String::new()),
            numbers: [None; N],
            num_alphas: None,
            num_numbers: None,
            io_status: None,
        }
    }
}
impl SelectedHeatBalanceInput {
    pub(crate) fn get_no_optional_source_object_item<const A: usize, const N: usize>(
        &self,
        family: &str,
        number: usize,
        input: &mut SelectedInputProcessor,
        args: &mut SourceLocalObjectArguments<A, N>,
    ) -> Result<(), Error> {
        let objects = self
            .objects
            .get(family)
            .ok_or(Error::Unavailable("actual local raw object cache"))?;
        if objects.len() != input.get_num_objects_found(family) {
            return Err(Error::Unavailable("actual local raw/IP count association"));
        }
        // Native cache lookup succeeded970, before all argument reset writes.
        args.num_alphas = Some(0);
        args.num_numbers = Some(0);
        args.io_status = Some(-1);
        let object = objects
            .get(number)
            .ok_or(Error::Unavailable("actual local Source object ordinal978"))?;
        if !input.has_object(family, &object.object_name) {
            return Err(Error::Unavailable("actual local permanent raw/IP identity"));
        }
        let (fields, min_fields, has_name) = super::schema::fields(family)
            .ok_or(Error::Unavailable("actual local literal schema family"))?;
        let max_fields = if self.source_is_epjson {
            fields.iter().enumerate().fold(min_fields, |max, (i, f)| {
                if object.fields.contains_key(f.json) {
                    max.max(i + 1)
                } else {
                    max
                }
            })
        } else {
            object
                .idf_max_fields
                .ok_or(Error::Unavailable("actual local IDF max_fields"))?
        };
        let alpha_count = fields
            .iter()
            .filter(|f| matches!(f.default, DefaultValue::Alpha(_)))
            .count();
        if A < alpha_count || N < fields.len() - alpha_count {
            return Err(Error::Unavailable("actual local argument array bounds"));
        }
        args.alphas.fill(String::new());
        args.numbers.fill(Some(0.0));
        // Native optional blanks/names are absent: no shared shortcut array is touched.
        input.mark_object_as_used(family, &object.object_name);
        let (mut alpha, mut numeric) = (0, 0);
        for (position, field) in fields.iter().enumerate() {
            match field.default {
                DefaultValue::Alpha(default) => {
                    if has_name && field.json == "name" {
                        args.alphas[alpha] = if field.retain_case {
                            object.object_name.clone()
                        } else {
                            object.object_name.to_ascii_uppercase()
                        };
                        args.num_alphas = Some(
                            args.num_alphas
                                .ok_or(Error::Unavailable("actual local NumAlphas970"))?
                                + 1,
                        );
                        alpha += 1;
                        continue;
                    }
                    let value = match object.fields.get(field.json) {
                        None => default,
                        Some(ProjectInputValue::String(s)) if s.is_empty() => default,
                        Some(ProjectInputValue::String(s)) => s.as_str(),
                        Some(ProjectInputValue::Number(_)) => {
                            return Err(Error::Unavailable(
                                "actual local numeric-to-alpha conversion828",
                            ));
                        }
                    };
                    args.alphas[alpha] = if field.retain_case {
                        value.into()
                    } else {
                        value.to_ascii_uppercase()
                    };
                    if position < max_fields {
                        args.num_alphas = Some(alpha + 1);
                    }
                    alpha += 1;
                }
                DefaultValue::Numeric(default) => {
                    let value = match object.fields.get(field.json) {
                        None => default,
                        Some(ProjectInputValue::String(s)) if s.is_empty() => default,
                        Some(ProjectInputValue::Number(n)) => *n,
                        Some(ProjectInputValue::String(_)) => {
                            return Err(Error::Unavailable(
                                "actual local autosize/autocalculate conversion854",
                            ));
                        }
                    };
                    args.numbers[numeric] = Some(value);
                    if position < max_fields {
                        args.num_numbers = Some(numeric + 1);
                    }
                    numeric += 1;
                }
            }
        }
        args.io_status = Some(1);
        Ok(())
    }
}
