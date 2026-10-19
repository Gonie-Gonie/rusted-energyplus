//! Native IP621-687/747-887/950-1070 on the same retained raw IP/shortcut.
use super::schema::{DefaultValue, FIELDS, Field, MIN_FIELDS};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut, SourceInputStatisticsValue as V,
};
pub(super) const FAMILY: &str = "ZoneHVAC:IdealLoadsAirSystem";

fn uppercase(value: &str) -> Result<String, Error> {
    if !value.is_ascii() {
        return Err(Error::Unavailable(
            "actual PurchasedAir non-ASCII makeUPPER domain",
        ));
    }
    Ok(value.to_ascii_uppercase())
}
fn number(value: &str) -> Result<f64, Error> {
    if value.bytes().any(|byte| matches!(byte, b'.' | b'e' | b'E')) {
        value
            .parse::<f64>()
            .map_err(|_| Error::Unavailable("actual PurchasedAir JSON double scalar"))
    } else {
        value
            .parse::<i64>()
            .map(|v| v as f64)
            .map_err(|_| Error::Unavailable("actual PurchasedAir JSON signed integer scalar"))
    }
}
fn descriptor(name: &str) -> Result<&'static Field, Error> {
    FIELDS
        .iter()
        .find(|row| row.json == name)
        .ok_or(Error::Unavailable(
            "actual PurchasedAir schema field identity",
        ))
}
pub(super) fn alpha(
    input: &SelectedInputProcessor,
    object: &str,
    field: &str,
) -> Result<String, Error> {
    let DefaultValue::Alpha(default) = descriptor(field)?.default else {
        return Err(Error::Unavailable("actual PurchasedAir alpha schema type"));
    };
    let fields = input
        .source_raw_object_fields(FAMILY, object)
        .map_err(|_| Error::Unavailable("actual PurchasedAir retained raw object fields"))?;
    match fields.get(field) {
        None => uppercase(default),
        Some(V::String(value)) if value.is_empty() => uppercase(default),
        Some(V::String(value)) => uppercase(value),
        _ => Err(Error::Unavailable(
            "actual PurchasedAir getAlphaFieldValue string type",
        )),
    }
}
pub(super) fn real(
    input: &SelectedInputProcessor,
    object: &str,
    field: &str,
) -> Result<f64, Error> {
    let DefaultValue::Number(default) = descriptor(field)?.default else {
        return Err(Error::Unavailable("actual PurchasedAir real schema type"));
    };
    let fields = input
        .source_raw_object_fields(FAMILY, object)
        .map_err(|_| Error::Unavailable("actual PurchasedAir retained real object fields"))?;
    match fields.get(field) {
        None => Ok(default),
        Some(V::String(value)) if value.is_empty() => Ok(default),
        Some(V::String(_)) => Ok(-99999.0), // Actual Constant::AutoCalculate.
        Some(V::Number(token)) => number(token),
        _ => Err(Error::Unavailable(
            "actual PurchasedAir getRealFieldValue scalar type",
        )),
    }
}

/// Genuine getObjectItem, including all argument/blank/name writes and the
/// consumed physical NumNums. A sole IDF object has exact JSON/IDF ordinal1.
pub(super) fn get(
    input: &mut SelectedInputProcessor,
    shortcut: &mut SourceInputProcessorShortCut,
    object: &str,
    number_in_json: usize,
    source_is_epjson: bool,
) -> Result<(i32, i32), Error> {
    let names = input
        .source_raw_object_names(FAMILY)
        .map_err(|_| Error::Unavailable("actual PurchasedAir getter retained object names"))?;
    if !source_is_epjson && names.len() != 1 {
        return Err(Error::Unavailable(
            "actual PurchasedAir multiple-IDF getJSONObjNum order not retained",
        ));
    }
    if names
        .get(number_in_json.checked_sub(1).ok_or(Error::Unavailable(
            "actual PurchasedAir getObjectItem ordinal",
        ))?)
        .map(String::as_str)
        != Some(object)
    {
        return Err(Error::Unavailable(
            "actual PurchasedAir getObjectItem original identity",
        ));
    }
    let fields = input
        .source_raw_object_fields(FAMILY, object)
        .map_err(|_| Error::Unavailable("actual PurchasedAir getObjectItem raw association"))?;
    let maximum = if source_is_epjson {
        let mut max = MIN_FIELDS;
        for (i, row) in FIELDS.iter().enumerate() {
            if fields.contains_key(row.json) {
                max = max.max(i + 1);
            }
        }
        max
    } else {
        match fields.get("idf_max_fields") {
            Some(V::Number(token)) => token.parse::<usize>().map_err(|_| {
                Error::Unavailable("actual PurchasedAir IDF physical max-fields integer")
            })?,
            _ => {
                return Err(Error::Unavailable(
                    "actual PurchasedAir consumed IDF max-fields metadata",
                ));
            }
        }
    };
    if shortcut.alpha_args.len() < 21
        || shortcut.alpha_field_names.len() < 21
        || shortcut.alpha_field_blanks.len() < 21
        || shortcut.numeric_args.len() < 11
        || shortcut.numeric_field_names.len() < 11
        || shortcut.numeric_field_blanks.len() < 11
    {
        return Err(Error::Unavailable(
            "actual PurchasedAir shared shortcut allocation",
        ));
    }
    shortcut.alpha_args.fill(String::new());
    shortcut.numeric_args.fill(0.0);
    shortcut.numeric_field_blanks.fill(true);
    shortcut.alpha_field_blanks.fill(true);
    shortcut.alpha_field_names.fill(String::new());
    shortcut.numeric_field_names.fill(String::new());
    input.mark_object_as_used(FAMILY, object);
    let mut a = 0usize;
    let mut n = 0usize;
    let mut num_alphas = 0i32;
    let mut num_numbers = 0i32;
    for (i, row) in FIELDS.iter().enumerate() {
        if row.json == "name" {
            shortcut.alpha_args[a] = uppercase(object)?;
            shortcut.alpha_field_blanks[a] = object.is_empty();
            shortcut.alpha_field_names[a] = if source_is_epjson {
                row.json
            } else {
                row.legacy
            }
            .into();
            a += 1;
            num_alphas = i32::try_from(a)
                .map_err(|_| Error::Unavailable("actual PurchasedAir NumAlphas integer"))?;
            continue;
        }
        let value = input
            .source_raw_object_fields(FAMILY, object)
            .map_err(|_| Error::Unavailable("actual PurchasedAir original argument field"))?
            .get(row.json);
        let blank = match value {
            None => true,
            Some(V::String(s)) => s.is_empty(),
            Some(V::Number(_)) => false,
            _ => {
                return Err(Error::Unavailable(
                    "actual PurchasedAir getObjectItem scalar type",
                ));
            }
        };
        match row.default {
            DefaultValue::Alpha(_) => {
                shortcut.alpha_args[a] = alpha(input, object, row.json)?;
                shortcut.alpha_field_blanks[a] = blank;
                shortcut.alpha_field_names[a] = if source_is_epjson {
                    row.json
                } else {
                    row.legacy
                }
                .into();
                a += 1;
                if i < maximum {
                    num_alphas = i32::try_from(a)
                        .map_err(|_| Error::Unavailable("actual PurchasedAir NumAlphas integer"))?;
                }
            }
            DefaultValue::Number(_) => {
                shortcut.numeric_args[n] = real(input, object, row.json)?;
                shortcut.numeric_field_blanks[n] = blank;
                shortcut.numeric_field_names[n] = if source_is_epjson {
                    row.json
                } else {
                    row.legacy
                }
                .into();
                n += 1;
                if i < maximum {
                    num_numbers = i32::try_from(n)
                        .map_err(|_| Error::Unavailable("actual PurchasedAir NumNums integer"))?;
                }
            }
        }
    }
    Ok((num_alphas, num_numbers))
}
