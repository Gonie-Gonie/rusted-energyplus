//! First connected direct People input: original schema types, selected operands and raw references.
use super::{field, number, selected, text, text_is};
use ep_raw_model::{ObjectType, RawModel, RawObject, RawValue};
pub(super) fn known_field(name: &str) -> bool {
    matches!(
        name,
        "zone_or_zonelist_or_space_or_spacelist_name"
            | "number_of_people_schedule_name"
            | "number_of_people_calculation_method"
            | "number_of_people"
            | "people_per_floor_area"
            | "floor_area_per_person"
            | "fraction_radiant"
            | "sensible_heat_fraction"
            | "activity_level_schedule_name"
            | "carbon_dioxide_generation_rate"
            | "enable_ashrae_55_comfort_warnings"
            | "mean_radiant_temperature_calculation_type"
            | "surface_name_angle_factor_list_name"
            | "work_efficiency_schedule_name"
            | "clothing_insulation_calculation_method"
            | "clothing_insulation_calculation_method_schedule_name"
            | "clothing_insulation_schedule_name"
            | "air_velocity_schedule_name"
            | "thermal_comfort_model_1_type"
            | "thermal_comfort_model_2_type"
            | "thermal_comfort_model_3_type"
            | "thermal_comfort_model_4_type"
            | "thermal_comfort_model_5_type"
            | "thermal_comfort_model_6_type"
            | "thermal_comfort_model_7_type"
            | "ankle_level_air_velocity_schedule_name"
            | "cold_stress_temperature_threshold"
            | "heat_stress_temperature_threshold"
    )
}
pub(super) fn admitted_field(name: &str, value: &RawValue) -> bool {
    match name {
        "idf_order" | "idf_max_fields" | "idf_max_extensible_fields" => {
            matches!(value, RawValue::Number(s) if s.parse::<usize>().is_ok())
        }
        "zone_or_zonelist_or_space_or_spacelist_name" => matches!(value, RawValue::String(_)),
        "number_of_people_schedule_name" => matches!(value, RawValue::String(_)),
        "number_of_people_calculation_method" => {
            text_is(value, &["", "Area/Person", "People", "People/Area"])
        }
        "number_of_people" => number(value).is_some_and(|n| n.is_finite() && n >= 0.0),
        "people_per_floor_area" => number(value).is_some_and(|n| n.is_finite() && n >= 0.0),
        "floor_area_per_person" => number(value).is_some_and(|n| n.is_finite() && n >= 0.0),
        "fraction_radiant" => {
            number(value).is_some_and(|n| n.is_finite() && (0.0..=1.0).contains(&n))
        }
        "sensible_heat_fraction" => {
            number(value).is_some_and(|n| (0.0..=1.0).contains(&n))
                || text_is(value, &["", "Autocalculate"])
        }
        "activity_level_schedule_name" => matches!(value, RawValue::String(_)),
        "carbon_dioxide_generation_rate" => {
            number(value).is_some_and(|n| n.is_finite() && (0.0..=3.82e-07).contains(&n))
        }
        "enable_ashrae_55_comfort_warnings" => text_is(value, &["", "No", "Yes"]),
        "mean_radiant_temperature_calculation_type" => text_is(
            value,
            &["", "AngleFactor", "EnclosureAveraged", "SurfaceWeighted"],
        ),
        "surface_name_angle_factor_list_name" => matches!(value, RawValue::String(_)),
        "work_efficiency_schedule_name" => matches!(value, RawValue::String(_)),
        "clothing_insulation_calculation_method" => text_is(
            value,
            &[
                "",
                "CalculationMethodSchedule",
                "ClothingInsulationSchedule",
                "DynamicClothingModelASHRAE55",
            ],
        ),
        "clothing_insulation_calculation_method_schedule_name" => {
            matches!(value, RawValue::String(_))
        }
        "clothing_insulation_schedule_name" => matches!(value, RawValue::String(_)),
        "air_velocity_schedule_name" => matches!(value, RawValue::String(_)),
        "thermal_comfort_model_1_type" => text_is(
            value,
            &[
                "AdaptiveASH55",
                "AdaptiveCEN15251",
                "AnkleDraftASH55",
                "CoolingEffectASH55",
                "Fanger",
                "KSU",
                "Pierce",
            ],
        ),
        "thermal_comfort_model_2_type" => text_is(
            value,
            &[
                "AdaptiveASH55",
                "AdaptiveCEN15251",
                "AnkleDraftASH55",
                "CoolingEffectASH55",
                "Fanger",
                "KSU",
                "Pierce",
            ],
        ),
        "thermal_comfort_model_3_type" => text_is(
            value,
            &[
                "AdaptiveASH55",
                "AdaptiveCEN15251",
                "AnkleDraftASH55",
                "CoolingEffectASH55",
                "Fanger",
                "KSU",
                "Pierce",
            ],
        ),
        "thermal_comfort_model_4_type" => text_is(
            value,
            &[
                "AdaptiveASH55",
                "AdaptiveCEN15251",
                "AnkleDraftASH55",
                "CoolingEffectASH55",
                "Fanger",
                "KSU",
                "Pierce",
            ],
        ),
        "thermal_comfort_model_5_type" => text_is(
            value,
            &[
                "AdaptiveASH55",
                "AdaptiveCEN15251",
                "AnkleDraftASH55",
                "CoolingEffectASH55",
                "Fanger",
                "KSU",
                "Pierce",
            ],
        ),
        "thermal_comfort_model_6_type" => text_is(
            value,
            &[
                "AdaptiveASH55",
                "AdaptiveCEN15251",
                "AnkleDraftASH55",
                "CoolingEffectASH55",
                "Fanger",
                "KSU",
                "Pierce",
            ],
        ),
        "thermal_comfort_model_7_type" => text_is(
            value,
            &[
                "AdaptiveASH55",
                "AdaptiveCEN15251",
                "AnkleDraftASH55",
                "CoolingEffectASH55",
                "Fanger",
                "KSU",
                "Pierce",
            ],
        ),
        "ankle_level_air_velocity_schedule_name" => matches!(value, RawValue::String(_)),
        "cold_stress_temperature_threshold" => number(value).is_some_and(|n| n.is_finite()),
        "heat_stress_temperature_threshold" => number(value).is_some_and(|n| n.is_finite()),
        _ => false,
    }
}
pub(super) fn admitted_object(raw: &RawModel, object: &RawObject) -> bool {
    if !selected(
        object,
        "number_of_people_calculation_method",
        &["People", "People/Area", "Area/Person"],
    ) {
        return false;
    }
    // Known field types/enums were validated first. No-model auxiliaries stay
    // unconsumed; this positive branch connects only genuine scheduled Fanger.
    let mut fanger = false;
    for name in [
        "thermal_comfort_model_1_type",
        "thermal_comfort_model_2_type",
        "thermal_comfort_model_3_type",
        "thermal_comfort_model_4_type",
        "thermal_comfort_model_5_type",
        "thermal_comfort_model_6_type",
        "thermal_comfort_model_7_type",
    ] {
        if let Some(value) = field(object, name) {
            if text_is(value, &["Fanger"]) {
                fanger = true;
            } else if !matches!(value, RawValue::String(s) if s.is_empty()) {
                return false;
            }
        }
    }
    if fanger
        && (!selected(
            object,
            "mean_radiant_temperature_calculation_type",
            &["EnclosureAveraged"],
        ) || !selected(
            object,
            "clothing_insulation_calculation_method",
            &["ClothingInsulationSchedule"],
        ))
    {
        return false;
    }
    let Some(zones) = raw.objects.get(&ObjectType("Zone".into())) else {
        return false;
    };
    if zones.len() != 1
        || !text(object, "zone_or_zonelist_or_space_or_spacelist_name")
            .is_some_and(|target| zones.keys().any(|n| n.0.eq_ignore_ascii_case(target)))
    {
        return false;
    }
    let source_schedule = |schedule: &str| {
        ["Constant-0.0", "Constant-1.0"]
            .iter()
            .any(|n| schedule.eq_ignore_ascii_case(n))
            || raw
                .objects
                .get(&ObjectType("Schedule:Constant".into()))
                .is_some_and(|objects| objects.keys().any(|n| n.0.eq_ignore_ascii_case(schedule)))
            || super::source_b::has_source_detailed_reference(raw, schedule)
    };
    let mut required_schedules = vec![
        "number_of_people_schedule_name",
        "activity_level_schedule_name",
    ];
    if fanger {
        required_schedules.extend([
            "work_efficiency_schedule_name",
            "clothing_insulation_schedule_name",
            "air_velocity_schedule_name",
        ]);
        // Native validates any nonempty A21 even when AnkleDraft is false.
        if text(object, "ankle_level_air_velocity_schedule_name").is_some_and(|s| !s.is_empty()) {
            required_schedules.push("ankle_level_air_velocity_schedule_name");
        }
    }
    for name in required_schedules {
        let Some(schedule) = text(object, name) else {
            return false;
        };
        if !source_schedule(schedule) {
            return false;
        }
    }
    true
}
