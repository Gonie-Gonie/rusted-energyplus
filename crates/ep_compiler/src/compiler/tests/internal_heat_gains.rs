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
