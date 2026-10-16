//! Actual no-optional-arguments getObjectItem for the four numeric-only Sites.
use super::*;
use crate::heat_balance::selected_project_data::ProjectInputValue;
const MONTHS: [&str; 12] = [
    "january",
    "february",
    "march",
    "april",
    "may",
    "june",
    "july",
    "august",
    "september",
    "october",
    "november",
    "december",
];
fn key(kind: SiteGroundModelKind, month: &str) -> String {
    match kind {
        SiteGroundModelKind::Shallow => format!("{month}_surface_ground_temperature"),
        SiteGroundModelKind::Deep => format!("{month}_deep_ground_temperature"),
        _ => format!("{month}_ground_temperature"),
    }
}
pub(super) fn get_object_item(
    raw: &SelectedHeatBalanceInput,
    kind: SiteGroundModelKind,
    input: &mut SelectedInputProcessor,
    shortcut: &mut SourceInputProcessorShortCut,
) -> Result<usize, Error> {
    let family = kind.family();
    let objects = raw
        .objects
        .get(family)
        .ok_or(Error::Unavailable("actual Site ground raw operands"))?;
    if input.get_num_objects_found(family) != objects.len() {
        return Err(Error::Unavailable("actual Site ground raw/IP count"));
    }
    let object = objects
        .first()
        .ok_or(Error::Unavailable("actual Site ground first object"))?;
    if !input.has_object(family, &object.object_name) {
        return Err(Error::Unavailable("actual Site ground IP identity"));
    }
    if shortcut.numeric_args.len() < 12 {
        return Err(Error::Unavailable(
            "actual shared Site ground numeric allocation",
        ));
    }
    let max_fields = if raw.source_is_epjson {
        12
    } else {
        object
            .idf_max_fields
            .ok_or(Error::Unavailable("actual Site ground IDF max_fields"))?
    };
    // Original1011/1012. Optional blanks/field names are absent and untouched.
    shortcut.alpha_args.fill(String::new());
    shortcut.numeric_args.fill(0.0);
    input.mark_object_as_used(family, &object.object_name);
    let mut numbers = 0;
    for (i, month) in MONTHS.iter().enumerate() {
        shortcut.numeric_args[i] = match object.fields.get(&key(kind, month)) {
            None => kind.default_temperature(),
            Some(ProjectInputValue::Number(value)) => *value,
            Some(ProjectInputValue::String(value)) if value.is_empty() => {
                kind.default_temperature()
            }
            // Original setObjectItemValue assigns AutoCalculate for nonempty strings.
            Some(ProjectInputValue::String(_)) => -99999.0,
        };
        if i < max_fields {
            numbers = i + 1;
        }
    }
    Ok(numbers)
}
