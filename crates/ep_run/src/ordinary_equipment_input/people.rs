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
    // unconsumed; either supported model consumes the real auxiliary inputs.
    let mut fanger = false;
    let mut ankle = false;
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
            } else if text_is(value, &["AnkleDraftASH55"]) {
                ankle = true;
            } else if !matches!(value, RawValue::String(s) if s.is_empty()) {
                return false;
            }
        }
    }
    let additional_inputs = fanger || ankle;
    if additional_inputs
        && (!selected(
            object,
            "mean_radiant_temperature_calculation_type",
            &["EnclosureAveraged", "SurfaceWeighted", "AngleFactor"],
        ) || !selected(
            object,
            "clothing_insulation_calculation_method",
            &[
                "ClothingInsulationSchedule",
                "DynamicClothingModelASHRAE55",
                "CalculationMethodSchedule",
            ],
        ))
    {
        return false;
    }
    if additional_inputs
        && field(object, "mean_radiant_temperature_calculation_type")
            .is_some_and(|value| text_is(value, &["SurfaceWeighted"]))
        && !text(object, "surface_name_angle_factor_list_name").is_some_and(|name| {
            raw.objects
                .get(&ObjectType("BuildingSurface:Detailed".into()))
                .is_some_and(|surfaces| surfaces.keys().any(|n| n.0.eq_ignore_ascii_case(name)))
        })
    {
        return false;
    }
    if additional_inputs
        && field(object, "mean_radiant_temperature_calculation_type")
            .is_some_and(|value| text_is(value, &["AngleFactor"]))
    {
        // A missing A8 defaults to empty. Empty/whitespace can name a genuine
        // list; only the shared getter applies Native uppercase, without trim.
        let name = match field(object, "surface_name_angle_factor_list_name") {
            None => "",
            Some(RawValue::String(name)) => name,
            _ => return false,
        };
        if !raw
            .objects
            .get(&ObjectType("ComfortViewFactorAngles".into()))
            .is_some_and(|lists| lists.keys().any(|key| key.0.eq_ignore_ascii_case(name)))
        {
            return false;
        }
    }
    let Some(zones) = raw.objects.get(&ObjectType("Zone".into())) else {
        return false;
    };
    if !text(object, "zone_or_zonelist_or_space_or_spacelist_name")
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
    if additional_inputs {
        required_schedules.extend([
            "work_efficiency_schedule_name",
            "air_velocity_schedule_name",
        ]);
        if selected(
            object,
            "clothing_insulation_calculation_method",
            &["ClothingInsulationSchedule"],
        ) {
            required_schedules.push("clothing_insulation_schedule_name");
        } else if field(object, "clothing_insulation_calculation_method")
            .is_some_and(|value| text_is(value, &["CalculationMethodSchedule"]))
        {
            required_schedules.push("clothing_insulation_calculation_method_schedule_name");
            // A12 is consumed only if the real method schedule hasVal(1).
            // Raw admission cannot substitute current values or min/max here.
        }
        // Dynamic clothing consumes neither the method nor clothing schedule.
        // Native validates any nonempty A21 even when AnkleDraft is false.
        if ankle
            || matches!(field(object, "ankle_level_air_velocity_schedule_name"), Some(RawValue::String(s)) if !s.is_empty())
        {
            required_schedules.push("ankle_level_air_velocity_schedule_name");
        }
    }
    for name in required_schedules {
        let Some(RawValue::String(schedule)) = field(object, name)
            .filter(|value| matches!(value, RawValue::String(text) if !text.is_empty()))
        else {
            return false;
        };
        if !source_schedule(schedule) {
            return false;
        }
    }
    true
}

// Exact optional extensible schema; counts/defaults belong to the real getter.
pub(super) fn admitted_angle_field(name: &str, value: &RawValue) -> bool {
    match name {
        "idf_order" | "idf_max_fields" | "idf_max_extensible_fields" => {
            matches!(value, RawValue::Number(s) if s.parse::<usize>().is_ok())
        }
        "angles" => matches!(value, RawValue::Array(rows) if rows.iter().all(|row| {
            let RawValue::Object(fields) = row else { return false; };
            fields.get(&ep_raw_model::FieldName("surface_name".into()))
                .is_none_or(|value| matches!(value, RawValue::String(_)))
                && fields.get(&ep_raw_model::FieldName("angle_factor".into()))
                    .is_none_or(|value| number(value).is_some_and(|n| (0.0..=1.0).contains(&n)))
        })),
        // Unknown instance/row members are retained raw metadata. Reserved
        // top-level IDF names above never reach this fallback.
        _ => true,
    }
}
