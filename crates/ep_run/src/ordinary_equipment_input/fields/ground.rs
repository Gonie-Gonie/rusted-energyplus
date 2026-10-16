//! Exact four numeric-only ground-temperature families from IDD1633-1860.
use super::super::{blank, number};
use ep_raw_model::RawValue;
pub(super) fn admitted_field(family: &str, name: &str, value: &RawValue) -> Option<bool> {
    let suffix = match family {
        "Site:GroundTemperature:BuildingSurface" | "Site:GroundTemperature:FCfactorMethod" => {
            "_ground_temperature"
        }
        "Site:GroundTemperature:Shallow" => "_surface_ground_temperature",
        "Site:GroundTemperature:Deep" => "_deep_ground_temperature",
        _ => return None,
    };
    if matches!(
        name,
        "idf_order" | "idf_max_fields" | "idf_max_extensible_fields"
    ) {
        return Some(matches!(value,RawValue::Number(text) if text.parse::<usize>().is_ok()));
    }
    let month = name.strip_suffix(suffix);
    Some(
        month.is_some_and(|m| {
            [
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
            ]
            .contains(&m)
        }) && (blank(value) || number(value).is_some()),
    )
}
