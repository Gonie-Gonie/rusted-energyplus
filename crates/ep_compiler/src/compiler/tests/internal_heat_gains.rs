use super::super::{Compiler, DiagnosticSeverity, compile_raw_model};
use ep_model::{ModelGraph, TypedModel};
use ep_raw_model::parse_epjson_str;

#[test]
fn direct_zone_people_and_other_equipment_use_existing_typed_arenas()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone":{"Direct Zone":{}},
            "People":{
                "Occupants":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone",
                    "number_of_people":3.0
                }
            },
            "OtherEquipment":{
                "Plug Load":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone",
                    "design_level":125.0
                }
            }
        }"#,
    )?;

    let result = compile_raw_model(&raw);

    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let model = result
        .model
        .ok_or_else(|| std::io::Error::other("expected typed model"))?;
    assert_eq!(model.people.len(), 1);
    assert_eq!(
        model.people_names.resolve("Occupants"),
        Some(model.people[0].id)
    );
    assert_eq!(model.people[0].zone, model.zones[0].id);
    assert_eq!(model.other_equipment.len(), 1);
    assert_eq!(
        model.other_equipment_names.resolve("Plug Load"),
        Some(model.other_equipment[0].id)
    );
    assert_eq!(model.other_equipment[0].zone, model.zones[0].id);
    assert_eq!(result.report.typed_object_count, model.object_count());
    assert_eq!(model.people[0].number_of_people_schedule, None);
    for (reference, expected) in [
        (
            "cOnStAnT-0.0",
            ep_model::PeopleScheduleReference::ConstantZero,
        ),
        (
            "CONSTANT-1.0",
            ep_model::PeopleScheduleReference::ConstantOne,
        ),
        (
            "User",
            ep_model::PeopleScheduleReference::User(ep_model::ScheduleId(0)),
        ),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"Schedule:Constant":{{"User":{{"hourly_value":0.25}}}},"People":{{"Occupants":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","number_of_people_schedule_name":"{reference}","number_of_people":2}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&text)?);
        assert!(
            !result.has_errors(),
            "{reference}: {:?}",
            result.report.diagnostics
        );
        let typed = result.model.ok_or("typed People schedule identity")?;
        assert_eq!(typed.people[0].number_of_people_schedule, Some(expected));
        assert_eq!(typed.schedules.len(), 1);
        assert_eq!(typed.schedules[0].id, ep_model::ScheduleId(0));
        assert_eq!(
            typed.schedule_names.resolve("User"),
            Some(ep_model::ScheduleId(0))
        );
        assert_eq!(typed.schedule_names.resolve("Constant-0.0"), None);
        assert_eq!(typed.schedule_names.resolve("Constant-1.0"), None);
    }
    // Distinct raw names do not collide merely because typed user-name
    // normalization trims them. Native preserves both name boundaries.
    for (authored, reference, expected) in [
        (
            " Constant-1.0",
            "Constant-1.0",
            ep_model::PeopleScheduleReference::ConstantOne,
        ),
        (
            "Constant-0.0 ",
            "Constant-0.0",
            ep_model::PeopleScheduleReference::ConstantZero,
        ),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"Schedule:Constant":{{"{authored}":{{"hourly_value":0.25}}}},"People":{{"Occupants":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","number_of_people_schedule_name":"{reference}","number_of_people":2}}}}}}"#
        );
        let raw = parse_epjson_str(&text)?;
        let result = compile_raw_model(&raw);
        assert!(
            !result.has_errors(),
            "{authored:?}: {:?}",
            result.report.diagnostics
        );
        let typed = result.model.ok_or("distinct raw schedule name")?;
        assert_eq!(typed.people[0].number_of_people_schedule, Some(expected));
        assert_eq!(typed.schedules.len(), 1);
        assert_eq!(typed.schedules[0].hourly_value, 0.25);
    }
    for (reference, schedules, expected_code) in [
        ("Missing", "", "MissingReference"),
        (" Constant-1.0", "", "MissingReference"),
        ("Constant-2.0", "", "MissingReference"),
        (
            "CONSTANT-1.0",
            r#", "Schedule:Constant":{"constant-1.0":{"hourly_value":0.25}}"#,
            "AmbiguousPeopleSchedule",
        ),
        (
            "Constant-0.0",
            r#", "Schedule:Constant":{"CONSTANT-0.0":{"hourly_value":0.25}}"#,
            "AmbiguousPeopleSchedule",
        ),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}}{schedules},"People":{{"Occupants":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","number_of_people_schedule_name":"{reference}","number_of_people":2}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&text)?);
        assert!(result.has_errors(), "{reference}");
        assert!(
            result
                .report
                .diagnostics
                .iter()
                .any(|d| d.code == expected_code
                    && d.object_type == "People"
                    && d.field.as_deref() == Some("number_of_people_schedule_name")),
            "{reference}: {:?}",
            result.report.diagnostics
        );
    }
    // Unit92: only explicit Source compilation resolves genuine built-ins.
    // No authored object or user ScheduleId is inserted for either constant.
    for (reference, expected) in [
        (
            "cOnStAnT-0.0",
            ep_model::InternalGainScheduleReference::ConstantZero,
        ),
        (
            "CONSTANT-1.0",
            ep_model::InternalGainScheduleReference::ConstantOne,
        ),
        (
            "User",
            ep_model::InternalGainScheduleReference::User(ep_model::ScheduleId(0)),
        ),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"Schedule:Constant":{{"User":{{"hourly_value":0.25}}}},"OtherEquipment":{{"Plug":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","schedule_name":"{reference}","design_level":10}}}}}}"#
        );
        let raw = parse_epjson_str(&text)?;
        let result = super::super::compile_raw_model_for_source(&raw);
        assert!(
            !result.has_errors(),
            "{reference}: {:?}",
            result.report.diagnostics
        );
        let typed = result
            .model
            .ok_or("Source OtherEquipment schedule identity")?;
        assert_eq!(typed.other_equipment[0].schedule, Some(expected));
        assert_eq!(typed.schedules.len(), 1);
        assert_eq!(typed.schedules[0].id, ep_model::ScheduleId(0));
        assert_eq!(
            typed.schedule_names.resolve("User"),
            Some(ep_model::ScheduleId(0))
        );
        assert_eq!(typed.schedule_names.resolve("Constant-0.0"), None);
        assert_eq!(typed.schedule_names.resolve("Constant-1.0"), None);
        assert_eq!(
            compile_raw_model(&raw).has_errors(),
            reference != "User",
            "legacy compilation retains authored-user-only OtherEquipment schedules"
        );
    }
    for (authored, reference, expected) in [
        (
            " Constant-1.0",
            "Constant-1.0",
            ep_model::InternalGainScheduleReference::ConstantOne,
        ),
        (
            "Constant-0.0 ",
            "Constant-0.0",
            ep_model::InternalGainScheduleReference::ConstantZero,
        ),
        (
            " Constant-1.0",
            " Constant-1.0",
            ep_model::InternalGainScheduleReference::User(ep_model::ScheduleId(0)),
        ),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"Schedule:Constant":{{"{authored}":{{"hourly_value":0.25}}}},"OtherEquipment":{{"Plug":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","schedule_name":"{reference}","design_level":10}}}}}}"#
        );
        let result = super::super::compile_raw_model_for_source(&parse_epjson_str(&text)?);
        assert!(
            !result.has_errors(),
            "{authored:?}/{reference:?}: {:?}",
            result.report.diagnostics
        );
        let typed = result.model.ok_or("Source raw schedule name boundary")?;
        assert_eq!(typed.other_equipment[0].schedule, Some(expected));
        assert_eq!(typed.schedules[0].hourly_value, 0.25);
    }
    for (reference, schedules, code) in [
        ("Missing", "", "MissingReference"),
        (" Constant-1.0", "", "MissingReference"),
        ("Constant-2.0", "", "MissingReference"),
        (
            "Constant-1.0",
            r#", "Schedule:Constant":{"CONSTANT-1.0":{"hourly_value":0.25}}"#,
            "AmbiguousOtherEquipmentSchedule",
        ),
        (
            "CONSTANT-0.0",
            r#", "Schedule:Constant":{"constant-0.0":{"hourly_value":0.25}}"#,
            "AmbiguousOtherEquipmentSchedule",
        ),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}}{schedules},"OtherEquipment":{{"Plug":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","schedule_name":"{reference}","design_level":10}}}}}}"#
        );
        let result = super::super::compile_raw_model_for_source(&parse_epjson_str(&text)?);
        assert!(result.has_errors(), "{reference}");
        assert!(
            result.report.diagnostics.iter().any(|d| d.code == code
                && d.object_type == "OtherEquipment"
                && d.field.as_deref() == Some("schedule_name")),
            "{reference}: {:?}",
            result.report.diagnostics
        );
    }
    // Unit93: direct epJSON keys retain their actual whitespace identity.
    // Source JSON order is natural; no IDF trim or fabricated IDs supply it.
    let exact_names = parse_epjson_str(
        r#"{
        "Zone":{"Direct Zone":{}},"Schedule:Constant":{"On":{"hourly_value":1}},
        "OtherEquipment":{
            "A":{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","schedule_name":"On","design_level":3},
            " A ":{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","schedule_name":"On","design_level":7},
            "A ":{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","schedule_name":"On","design_level":11}
        }}"#,
    )?;
    assert!(exact_names.is_epjson());
    let result = super::super::compile_raw_model_for_source(&exact_names);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let typed = result
        .model
        .ok_or("actual Source OtherEquipment exact names")?;
    assert_eq!(typed.other_equipment.len(), 3);
    for (row, (name, level)) in
        typed
            .other_equipment
            .iter()
            .zip([(" A ", 7.0), ("A", 3.0), ("A ", 11.0)])
    {
        assert_eq!(row.name.0, name);
        assert_eq!(row.design_level_w, level);
        assert_eq!(
            typed
                .other_equipment_names
                .resolve(&name.to_ascii_lowercase()),
            Some(row.id)
        );
        assert_eq!(
            typed.other_equipment_names.names()[row.id.0 as usize],
            row.name
        );
        assert_eq!(
            row.schedule,
            Some(ep_model::InternalGainScheduleReference::User(
                ep_model::ScheduleId(0)
            ))
        );
    }
    assert_eq!(typed.other_equipment_names.resolve(" A"), None);
    assert_eq!(typed.schedules.len(), 1);
    assert_eq!(ep_model::NormalizedName::new(" A ").0, "A");
    let legacy = compile_raw_model(&exact_names);
    assert!(
        legacy
            .report
            .diagnostics
            .iter()
            .any(|d| d.code == "DuplicateName" && d.object_type == "OtherEquipment"),
        "unscoped compilation keeps its established trimmed-name boundary"
    );
    for (json_name, actual_name) in [
        (" a ", " A "),
        (r"\tA\t", "\tA\t"),
        (r"\u000b", "\u{000b}"),
        (r"\u00a0", "\u{00a0}"),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"Schedule:Constant":{{"On":{{"hourly_value":1}}}},"OtherEquipment":{{"{json_name}":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","schedule_name":"On","design_level":7}}}}}}"#
        );
        let raw = parse_epjson_str(&text)?;
        let result = super::super::compile_raw_model_for_source(&raw);
        assert!(
            !result.has_errors(),
            "{json_name}: {:?}",
            result.report.diagnostics
        );
        let typed = result.model.ok_or("Source schema-valid exact gain name")?;
        assert_eq!(typed.other_equipment[0].name.0, actual_name);
        assert_eq!(
            typed.other_equipment_names.resolve(actual_name),
            Some(typed.other_equipment[0].id)
        );
    }
    let single = parse_epjson_str(
        r#"{"Zone":{"Direct Zone":{}},"OtherEquipment":{" a ":{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","design_level":7}}}"#,
    )?;
    let legacy = compile_raw_model(&single)
        .model
        .ok_or("unchanged legacy single gain name")?;
    assert_eq!(legacy.other_equipment[0].name.0, "A");
    assert_eq!(
        legacy.other_equipment_names.resolve(" A "),
        Some(legacy.other_equipment[0].id)
    );
    for json_name in ["", r" \t\r\f", r"A\nB"] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"OtherEquipment":{{"{json_name}":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","design_level":7}}}}}}"#
        );
        let result = super::super::compile_raw_model_for_source(&parse_epjson_str(&text)?);
        assert!(
            result
                .report
                .diagnostics
                .iter()
                .any(|d| d.code == "InvalidName" && d.object_type == "OtherEquipment"),
            "{json_name}: {:?}",
            result.report.diagnostics
        );
    }
    // Unit94: Source list instances use real members/implicit Spaces and dense
    // typed rows; raw statements retain their separate explicit range owner.
    let list_raw = parse_epjson_str(
        r#"{
        "Zone":{"Zulu":{"floor_area":12},"Alpha":{"floor_area":6}},
        "Schedule:Constant":{"On":{"hourly_value":1}},
        "ZoneList":{"Both":{"zones":[{"zone_name":"Zulu"},{"zone_name":"Alpha"}]},
            "Reverse":{"zones":[{"zone_name":"Alpha"},{"zone_name":"Zulu"}]},
            "Alpha":{"zones":[{"zone_name":"Zulu"}]}},
        "OtherEquipment":{
            "A level":{"zone_or_zonelist_or_space_or_spacelist_name":"Both","schedule_name":"On","design_level":23},
            "B area":{"zone_or_zonelist_or_space_or_spacelist_name":"Reverse","schedule_name":"On","design_level_calculation_method":"Watts/Area","design_level":-7,"power_per_floor_area":1.75,"power_per_person":-3},
            "C shadow":{"zone_or_zonelist_or_space_or_spacelist_name":"Alpha","schedule_name":"On","design_level":5}}}"#,
    )?;
    let result = super::super::compile_raw_model_for_source(&list_raw);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let list_model = result.model.ok_or("Source list expansion")?;
    assert_eq!(list_model.other_equipment.len(), 5);
    assert_eq!(list_model.source_other_equipment_inputs.len(), 3);
    assert_eq!(
        list_model
            .other_equipment
            .iter()
            .map(|r| r.name.0.as_str())
            .collect::<Vec<_>>(),
        vec![
            "ZULU A LEVEL",
            "ALPHA A LEVEL",
            "ALPHA B AREA",
            "ZULU B AREA",
            "C SHADOW"
        ]
    );
    assert!(
        list_model
            .other_equipment_names
            .resolve("A level")
            .is_none()
    );
    for (index, row) in list_model.other_equipment.iter().enumerate() {
        assert_eq!(usize::try_from(row.id.0)?, index);
        assert_eq!(
            list_model.other_equipment_names.resolve(&row.name.0),
            Some(row.id)
        );
        assert_eq!(
            row.schedule,
            Some(ep_model::InternalGainScheduleReference::User(
                ep_model::ScheduleId(0)
            ))
        );
    }
    for binding in &list_model.source_other_equipment_inputs {
        let rows = &list_model.other_equipment[binding.instances.clone()];
        match binding.target {
            ep_model::SourceOtherEquipmentInputTarget::ZoneList(id) => {
                let list = list_model
                    .zone_lists
                    .iter()
                    .find(|r| r.id == id)
                    .ok_or("real typed list")?;
                assert_eq!(rows.iter().map(|r| r.zone).collect::<Vec<_>>(), list.zones);
                for row in rows {
                    let space = list_model
                        .spaces
                        .iter()
                        .find(|s| s.zone == row.zone)
                        .ok_or("actual implicit Space")?;
                    assert_eq!(space.origin, ep_model::SpaceOrigin::AutoZoneDefault);
                    assert_eq!(
                        row.name.0,
                        format!(
                            "{} {}",
                            space.name.0,
                            binding.object_name.to_ascii_uppercase()
                        )
                    );
                }
            }
            ep_model::SourceOtherEquipmentInputTarget::Zone(id) => {
                assert_eq!(binding.object_name, "C shadow");
                assert_eq!(rows.len(), 1);
                assert_eq!(rows[0].zone, id);
                assert_eq!(list_model.zone_names.resolve("Alpha"), Some(id));
            }
        }
    }
    assert!(
        compile_raw_model(&list_raw).has_errors(),
        "legacy gains retain direct-Zone-only lookup"
    );
    // A one-member list still prefixes the real implicit Space.Name; preserve
    // actual raw spelling when legacy typed Zone/Space names themselves trim.
    let spaced = parse_epjson_str(
        r#"{"Zone":{" Z ":{}},"ZoneList":{" L ":{"zones":[{"zone_name":" Z "}]}},"OtherEquipment":{" p ":{"zone_or_zonelist_or_space_or_spacelist_name":" L ","schedule_name":"Constant-1.0","design_level":3}}}"#,
    )?;
    let result = super::super::compile_raw_model_for_source(&spaced);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let spaced_model = result.model.ok_or("actual source spelling prefix")?;
    assert_eq!(spaced_model.zone_lists[0].name.0, " L ");
    assert_eq!(
        spaced_model.other_equipment[0].name.0,
        format!("{} {}", " Z ", " P ")
    );
    assert_eq!(
        spaced_model.source_other_equipment_inputs[0].instances,
        0..1
    );
    // Existing direct People stays genuine when the actual Zone name has
    // whitespace; the exact-reference guard must not inspect a trimmed alias.
    let spaced_people = parse_epjson_str(
        r#"{"Zone":{" Z ":{}},"People":{"P":{"zone_or_zonelist_or_space_or_spacelist_name":" Z ","number_of_people_schedule_name":"Constant-1.0","number_of_people":1}}}"#,
    )?;
    let result = super::super::compile_raw_model_for_source(&spaced_people);
    assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
    let typed = result.model.ok_or("existing exact direct People Zone")?;
    assert_eq!(typed.people.len(), 1);
    assert_eq!(typed.people[0].zone, typed.zones[0].id);
    for text in [
        r#"{"Zone":{"Z":{}},"ZoneList":{" L ":{"zones":[{"zone_name":"Z"}]}},"OtherEquipment":{"P":{"zone_or_zonelist_or_space_or_spacelist_name":"L","design_level":3}}}"#,
        r#"{"Zone":{" Z ":{}},"ZoneList":{"L":{"zones":[{"zone_name":"Z"}]}}}"#,
        r#"{"Zone":{"Z":{}},"ZoneList":{"L":{"zones":[{"zone_name":"Z"}]}},"People":{"P":{"zone_or_zonelist_or_space_or_spacelist_name":"L","number_of_people":1}}}"#,
    ] {
        assert!(
            super::super::compile_raw_model_for_source(&parse_epjson_str(text)?).has_errors(),
            "Source exact references and direct People boundary must remain: {text}"
        );
    }
    for method in ["Watts/Person", "Power/Person"] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"Schedule:Constant":{{"on":{{"hourly_value":1}},"activity":{{"hourly_value":120}}}},"People":{{"Occupants":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","number_of_people_schedule_name":"on","activity_level_schedule_name":"activity","number_of_people":2}}}},"OtherEquipment":{{"Per person":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","schedule_name":"on","design_level_calculation_method":"{method}","design_level":-7,"power_per_floor_area":-9,"power_per_person":3.5}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&text)?);
        assert!(
            !result.has_errors(),
            "{method}: {:?}",
            result.report.diagnostics
        );
        let typed = result.model.ok_or("typed per-person equipment")?;
        let row = &typed.other_equipment[0];
        assert_eq!(
            row.design_level_calculation_method,
            ep_model::OtherEquipmentDesignLevelCalculationMethod::WattsPerPerson
        );
        assert_eq!(row.design_level_w, -7.0);
        assert_eq!(row.power_per_floor_area_w_per_m2, -9.0);
        assert_eq!(row.power_per_person_w, 3.5);
    }
    // Preserve Native's grouped subtraction at the representable clamp edge.
    // The next lost-fraction float crosses the threshold even though decimal
    // 0.401 itself is just inside after the actual rounded addition.
    for (lost, valid) in [
        (0.3995_f64, true),
        (0.4005, true),
        (f64::from_bits(0.401_f64.to_bits() - 1), true),
        (0.401, true),
        (f64::from_bits(0.401_f64.to_bits() + 1), false),
        (0.4011, false),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"OtherEquipment":{{"Clamp":{{
            "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","design_level":7,
            "fraction_latent":0,"fraction_radiant":0.6,"fraction_lost":{lost}}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&text)?);
        assert_eq!(
            !result.has_errors(),
            valid,
            "{lost:?}: {:?}",
            result.report.diagnostics
        );
        if valid {
            let typed = result.model.ok_or("typed clamp owner")?;
            assert_eq!(
                typed.other_equipment[0].fraction_lost.to_bits(),
                lost.to_bits()
            );
        } else {
            assert!(
                result
                    .report
                    .diagnostics
                    .iter()
                    .any(|d| d.code == "InvalidOtherEquipmentFractionSum")
            );
        }
    }
    // Native selects Numeric1 only. Omission defaults the typed projection to
    // zero; the genuine raw getter still owns its separate blank warning.
    for unused in [9.0_f64, -9.0] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"OtherEquipment":{{"Unused":{{
            "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone",
            "power_per_floor_area":{unused},"power_per_person":{unused}}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&text)?);
        assert!(!result.has_errors(), "{:?}", result.report.diagnostics);
        let typed = result.model.ok_or("typed unused fields")?;
        let row = &typed.other_equipment[0];
        assert_eq!(row.design_level_w.to_bits(), 0.0_f64.to_bits());
        assert_eq!(
            row.power_per_floor_area_w_per_m2.to_bits(),
            unused.to_bits()
        );
        assert_eq!(row.power_per_person_w.to_bits(), unused.to_bits());
    }
    let selected_negative = parse_epjson_str(
        r#"{"Zone":{"Direct Zone":{}},"OtherEquipment":{"Bad":{
        "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone",
        "design_level_calculation_method":"Power/Area","power_per_floor_area":-9}}}"#,
    )?;
    assert!(compile_raw_model(&selected_negative).has_errors());
    // Literal JSON preserves Native stored integer versus double kind for all
    // seven actual IHG numeric fields. No serde Value round trip precedes parse.
    for (token, expected) in [
        ("-0", 0.0_f64),
        ("-0.0", -0.0),
        ("-0e0", -0.0),
        ("1e-9999", 0.0),
        ("-1e-9999", -0.0),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"OtherEquipment":{{"Kinds":{{
            "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone",
            "design_level":{token},"power_per_floor_area":{token},"power_per_person":{token},
            "fraction_latent":{token},"fraction_radiant":{token},"fraction_lost":{token},
            "carbon_dioxide_generation_rate":{token}}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&text)?);
        assert!(
            !result.has_errors(),
            "{token}: {:?}",
            result.report.diagnostics
        );
        let typed = result.model.ok_or("typed JSON numeric kind")?;
        let row = &typed.other_equipment[0];
        for value in [
            row.design_level_w,
            row.power_per_floor_area_w_per_m2,
            row.power_per_person_w,
            row.fraction_latent,
            row.fraction_radiant,
            row.fraction_lost,
            row.carbon_dioxide_generation_rate_m3_per_s_w,
        ] {
            assert_eq!(value.to_bits(), expected.to_bits(), "{token}");
        }
    }
    for (token, expected) in [
        ("9223372036854775808", i64::MIN as f64),
        ("18446744073709551615", -1.0),
        ("18446744073709551615.0", 18446744073709551616.0),
        ("18446744073709551615e0", 18446744073709551616.0),
        ("18446744073709551616", 18446744073709551616.0),
        ("1E+00", 1.0),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"OtherEquipment":{{"Kinds":{{
            "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","fuel_type":"None",
            "design_level":{token},"power_per_floor_area":{token},"power_per_person":{token}}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&text)?);
        assert!(
            !result.has_errors(),
            "{token}: {:?}",
            result.report.diagnostics
        );
        let typed = result.model.ok_or("typed wide JSON numeric kind")?;
        let row = &typed.other_equipment[0];
        for value in [
            row.design_level_w,
            row.power_per_floor_area_w_per_m2,
            row.power_per_person_w,
        ] {
            assert_eq!(value.to_bits(), expected.to_bits(), "{token}");
        }
    }
    // Raw schema nonnegative checks precede Native JSON-kind conversion.
    // Successful unsigned?signed projection is retained, including unused fields.
    for (token, expected) in [
        ("-0", 0.0_f64),
        ("-0.0", -0.0),
        ("18446744073709551615", -1.0),
        ("18446744073709551615.0", 18446744073709551616.0),
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"People":{{"Kinds":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","number_of_people":{token},"people_per_floor_area":{token},"floor_area_per_person":{token}}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&text)?);
        assert!(
            !result.has_errors(),
            "{token}: {:?}",
            result.report.diagnostics
        );
        let typed = result.model.ok_or("typed People number origin")?;
        for value in [
            typed.people[0].number_of_people,
            typed.people[0].people_per_floor_area,
            typed.people[0].floor_area_per_person,
        ] {
            assert_eq!(value.to_bits(), expected.to_bits());
        }
    }
    for field in [
        "number_of_people",
        "people_per_floor_area",
        "floor_area_per_person",
    ] {
        let text = format!(
            r#"{{"Zone":{{"Direct Zone":{{}}}},"People":{{"Bad":{{"zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone","{field}":-1}}}}}}"#
        );
        let result = compile_raw_model(&parse_epjson_str(&text)?);
        assert!(result.has_errors());
        assert!(
            result
                .report
                .diagnostics
                .iter()
                .any(|d| d.code == "InvalidNumericRange")
        );
    }
    Ok(())
}

#[test]
fn projected_diagnostics_follow_people_then_other_equipment_source_order()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "OtherEquipment":{"Missing Other Zone":{}},
            "People":{"Missing People Zone":{}}
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();

    compiler.parse_bounded_internal_heat_gains_input(&mut model);

    let error_types = compiler
        .diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
        .map(|diagnostic| diagnostic.object_type.as_str())
        .collect::<Vec<_>>();
    assert_eq!(error_types, ["People", "OtherEquipment"]);
    assert!(model.people.is_empty());
    assert!(model.other_equipment.is_empty());
    Ok(())
}

#[test]
fn people_error_does_not_prevent_other_equipment_scan() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone":{"Direct Zone":{}},
            "People":{
                "Broken Occupants":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Missing Zone"
                }
            },
            "OtherEquipment":{
                "Valid Plug Load":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone"
                }
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_zones(&mut model);
    assert!(compiler.diagnostics.is_empty());

    compiler.parse_bounded_internal_heat_gains_input(&mut model);

    assert!(compiler.diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error
            && diagnostic.object_type == "People"
            && diagnostic.object_name.as_deref() == Some("Broken Occupants")
    }));
    assert!(model.people.is_empty());
    assert_eq!(model.other_equipment.len(), 1);
    assert_eq!(model.other_equipment[0].name.0, "VALID PLUG LOAD");
    Ok(())
}

#[test]
fn pre_existing_error_suppresses_both_projected_families() -> Result<(), Box<dyn std::error::Error>>
{
    let raw = parse_epjson_str(
        r#"{
            "Timestep":{"Broken":{"number_of_timesteps_per_hour":"not a number"}},
            "Zone":{"Direct Zone":{}},
            "People":{
                "Occupants":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone"
                }
            },
            "OtherEquipment":{
                "Plug Load":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone"
                }
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_zones(&mut model);
    compiler.parse_timestep(&mut model);
    let diagnostics_before = compiler.diagnostics.clone();
    assert!(
        diagnostics_before
            .iter()
            .any(|diagnostic| diagnostic.severity == DiagnosticSeverity::Error)
    );

    compiler.parse_bounded_internal_heat_gains_input(&mut model);

    assert_eq!(compiler.diagnostics, diagnostics_before);
    assert!(model.people.is_empty());
    assert!(model.other_equipment.is_empty());
    Ok(())
}

#[test]
fn pre_existing_warning_does_not_suppress_the_bounded_pass()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone":{"Direct Zone":{}},
            "People":{
                "Occupants":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone"
                }
            },
            "OtherEquipment":{
                "Plug Load":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone"
                }
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_zones(&mut model);
    compiler.warning(
        "ExistingWarning",
        "Test",
        None,
        None,
        "warning before bounded pass".to_string(),
    );

    compiler.parse_bounded_internal_heat_gains_input(&mut model);

    assert_eq!(model.people.len(), 1);
    assert_eq!(model.other_equipment.len(), 1);
    assert_eq!(compiler.diagnostics.len(), 1);
    assert_eq!(
        compiler.diagnostics[0].severity,
        DiagnosticSeverity::Warning
    );
    Ok(())
}

#[test]
fn empty_input_is_a_no_op() -> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str("{}")?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    let model_before = model.clone();

    compiler.parse_bounded_internal_heat_gains_input(&mut model);

    assert_eq!(model, model_before);
    assert!(compiler.diagnostics.is_empty());
    assert!(compiler.defaults_applied.is_empty());
    Ok(())
}

#[test]
fn wrapper_adds_no_state_beyond_existing_people_and_other_equipment_parsers()
-> Result<(), Box<dyn std::error::Error>> {
    let raw = parse_epjson_str(
        r#"{
            "Zone":{"Direct Zone":{}},
            "People":{
                "Occupants":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone",
                    "number_of_people":2.0
                }
            },
            "OtherEquipment":{
                "Plug Load":{
                    "zone_or_zonelist_or_space_or_spacelist_name":"Direct Zone",
                    "design_level":80.0
                }
            }
        }"#,
    )?;
    let mut compiler = Compiler::new(&raw, None);
    let mut model = TypedModel::default();
    compiler.parse_zones(&mut model);
    let mut direct_compiler = Compiler::new(&raw, None);
    let mut direct_model = TypedModel::default();
    direct_compiler.parse_zones(&mut direct_model);

    compiler.parse_bounded_internal_heat_gains_input(&mut model);
    direct_compiler.parse_people(&mut direct_model);
    direct_compiler.parse_other_equipment(&mut direct_model);

    assert_eq!(model, direct_model);
    assert_eq!(model.object_count(), direct_model.object_count());
    assert_eq!(
        ModelGraph::from_typed(&model),
        ModelGraph::from_typed(&direct_model)
    );
    assert_eq!(compiler.diagnostics, direct_compiler.diagnostics);
    assert_eq!(compiler.defaults_applied, direct_compiler.defaults_applied);
    Ok(())
}
