//! Finite Source A ElectricEquipment fields; no sizing or fabricated Other row.
use super::{field, number, text_is};
use ep_raw_model::{RawObject, RawValue};

pub(super) fn admitted_field(name: &str, value: &RawValue) -> bool {
    match name {
        "idf_order" | "idf_max_fields" | "idf_max_extensible_fields" => {
            matches!(value,RawValue::Number(text)if text.parse::<usize>().is_ok())
        }
        "zone_or_zonelist_or_space_or_spacelist_name" | "schedule_name" | "end_use_subcategory" => {
            matches!(value, RawValue::String(_))
        }
        "design_level_calculation_method" => {
            text_is(value, &["", "EquipmentLevel", "Watts/Area", "Watts/Person"])
        }
        // Schema minima apply to every authored numeric operand, not only
        // the selected method. Missing fields retain genuine IP blank flags.
        "design_level" | "watts_per_floor_area" | "watts_per_person" => {
            number(value).is_some_and(|n| n >= 0.0)
        }
        "fraction_latent" | "fraction_radiant" | "fraction_lost" => {
            number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
        }
        // The official instance schema permits unknown properties of any
        // JSON kind. Retain them in the original IP, never as electric operands.
        // OtherEquipment aliases/fuel/CO2 therefore remain opaque metadata.
        _ => true,
    }
}

pub(super) fn admitted_object(object: &RawObject) -> bool {
    let [latent, radiant, lost] = ["fraction_latent", "fraction_radiant", "fraction_lost"]
        .map(|name| field(object, name).and_then(number).unwrap_or(0.0));
    let mut convected = 1.0 - (latent + radiant + lost);
    if convected.abs() <= 0.001 {
        convected = 0.0;
    }
    convected >= 0.0
}
