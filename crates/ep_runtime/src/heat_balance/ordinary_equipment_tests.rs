//! One conventional table for the real getter pair and Source-only transitions.
use super::*;
use serde_json::Value;

#[test]
fn actual_native_get_equipment_pair_and_input_owner_transitions()
-> Result<(), Box<dyn std::error::Error>> {
    let fixture: Value = serde_json::from_str(include_str!(
        "fixtures/ordinary_equipment_native_selected.json"
    ))?;
    let cases = fixture["cases"].as_array().ok_or("missing genuine cases")?;
    assert_eq!(cases.len(), 8);
    let pair = &cases[7];
    assert_eq!(
        pair["operation"],
        "function:GetZoneEquipment:get_zone_equipment_return"
    );
    assert_eq!(fixture["selection"][7]["source_case_index"], 13);
    let before = &pair["before"];
    let after = &pair["after"];
    assert_eq!(before["invocation_ordinal"], after["invocation_ordinal"]);
    assert_eq!(before["clock"], after["clock"]);
    let bits = u64::from_str_radix(
        before["clock"]["TimeStepZone"]["value_bits"]
            .as_str()
            .ok_or("missing real timestep")?,
        16,
    )?;
    let per_hour = 1.0 / f64::from_bits(bits);
    assert!(per_hour.is_finite() && per_hour.fract() == 0.0 && (1.0..=60.0).contains(&per_hour));
    let timesteps = per_hour as usize;
    let inventory = fixture["supplied_family_counts"]
        .as_object()
        .ok_or("missing supplied source counts")?;
    let inventory = inventory
        .iter()
        .map(|(family, count)| {
            Ok((
                family.as_str(),
                count.as_u64().ok_or("noninteger actual supplied count")? as usize,
            ))
        })
        .collect::<Result<Vec<_>, &str>>()?;
    let input = OrdinaryEquipmentInput::from_family_inventory(inventory, timesteps)?;
    let zones = [ZoneId(0)];
    let spaces = [SpaceId(0)];
    let mut owner = OrdinaryEquipment::new(input, &zones, &spaces)?;
    let integer = |value: &Value| value.as_u64().expect("genuine nonnegative integer") as usize;
    let boolean = |value: &Value| value.as_bool().expect("genuine bool");
    let assert_shape = |allocated: bool, len: usize, native: &Value| {
        assert_eq!(allocated, boolean(&native["allocated"]));
        assert_eq!(len, integer(&native["size"]));
        if allocated {
            assert_eq!(native["lower"], 1);
            assert_eq!(integer(&native["upper"]), len);
        } else {
            assert!(native["lower"].is_null() && native["upper"].is_null());
        }
    };
    for (side, run_getter) in [(before, false), (after, true)] {
        if run_getter {
            owner.get_zone_equipment(timesteps)?;
        }
        let r = &side["owners"]["readiness"];
        let n = &side["owners"]["nodes"];
        assert_eq!(owner.zone_ids.len(), integer(&r["NumOfZones"]));
        assert_eq!(owner.space_ids.len(), integer(&r["numSpaces"]));
        assert_eq!(
            owner.get_equipment_input,
            boolean(&r["flags"]["GetZoneEquipmentInputFlag"])
        );
        assert_eq!(
            owner.equipment_inputs_filled,
            boolean(&r["flags"]["ZoneEquipInputsFilled"])
        );
        assert_eq!(
            owner.equipment_simulated_once,
            boolean(&r["flags"]["ZoneEquipSimulatedOnce"])
        );
        assert_eq!(
            owner.equipment_data_errors_found,
            boolean(&r["flags"]["GetZoneEquipmentDataErrorsFound"])
        );
        assert_eq!(
            owner.number_of_timesteps_in_day,
            integer(&r["NumOfTimeStepInDay"])
        );
        assert_eq!(
            owner.number_of_equipment_lists,
            integer(&r["NumOfZoneEquipLists"])
        );
        assert_eq!(
            owner.number_of_supply_paths,
            integer(&r["NumSupplyAirPaths"])
        );
        assert_eq!(
            owner.number_of_return_paths,
            integer(&r["NumReturnAirPaths"])
        );
        assert_shape(
            owner.supply_paths.is_some(),
            owner.supply_paths.as_ref().map_or(0, Vec::len),
            &r["SupplyAirPath"],
        );
        assert_shape(
            owner.return_paths.is_some(),
            owner.return_paths.as_ref().map_or(0, Vec::len),
            &r["ReturnAirPath"],
        );
        assert_shape(
            owner.zone_config.is_some(),
            owner.zone_config.as_ref().map_or(0, Vec::len),
            &r["ZoneEquipConfig"],
        );
        assert_shape(
            owner.equipment_lists.is_some(),
            owner.equipment_lists.as_ref().map_or(0, Vec::len),
            &r["ZoneEquipList"],
        );
        assert_shape(
            owner.equipment_availability.is_some(),
            owner.equipment_availability.as_ref().map_or(0, Vec::len),
            &r["ZoneEquipAvail"],
        );
        assert_shape(
            owner.priority_simulation_order.is_some(),
            owner.priority_simulation_order.as_ref().map_or(0, Vec::len),
            &r["PrioritySimOrder"],
        );
        assert_shape(
            owner.nodes.physical.is_some(),
            owner.nodes.physical.as_ref().map_or(0, Vec::len),
            &n["Node"],
        );
        assert_eq!(owner.nodes.get_input, boolean(&n["GetNodeInputFlag"]));
        assert_eq!(owner.nodes.number_of_nodes, integer(&n["NumOfNodes"]));
        assert_eq!(
            owner.nodes.number_of_unique_node_names,
            integer(&n["NumOfUniqueNodeNames"])
        );
        assert_eq!(
            owner.nodes.number_of_node_lists,
            integer(&n["NumOfNodeLists"])
        );
        if let Some(rows) = &owner.zone_config {
            let actual = &r["ZoneEquipConfig"]["values"][0];
            let row = &rows[0];
            assert_eq!(
                row.zone_name,
                actual["ZoneName"]
                    .as_str()
                    .ok_or("missing actual Config name")?
            );
            for (value, field) in [
                (row.equipment_list_index, "EquipListIndex"),
                (row.zone_node, "ZoneNode"),
                (row.inlet_nodes, "NumInletNodes"),
                (row.exhaust_nodes, "NumExhaustNodes"),
                (row.return_nodes, "NumReturnNodes"),
            ] {
                assert_eq!(value, integer(&actual[field]));
            }
            for (value, field) in [
                (row.is_controlled, "IsControlled"),
                (row.in_floor_active_element, "InFloorActiveElement"),
                (row.in_wall_active_element, "InWallActiveElement"),
                (row.in_ceiling_active_element, "InCeilingActiveElement"),
            ] {
                assert_eq!(value, boolean(&actual[field]));
            }
            assert_eq!(
                owner.equipment_lists.as_ref().ok_or("missing list")?[0].number_of_equipment_types,
                integer(&r["zone_equipment_list_counts"][0]["NumOfEquipTypes"])
            );
            assert_eq!(
                owner
                    .equipment_availability
                    .as_ref()
                    .ok_or("missing availability")?[0],
                r["zone_equipment_availability"][0]["value"]
                    .as_i64()
                    .ok_or("missing actual NoAction")? as i32
            );
        }
    }
    // Actual query results are the getter's inputs, not reconstructed owners.
    assert_eq!(cases[0]["event"]["locals"]["numControlledZones"], 0);
    assert_eq!(cases[0]["event"]["locals"]["numControlledSpaces"], 0);
    assert_eq!(cases[0]["event"]["locals"]["NumOfZoneEquipLists"], 0);
    assert_eq!(
        cases[1]["event"]["owners"]["readiness"]["NumSupplyAirPaths"],
        0
    );
    assert_eq!(
        cases[2]["event"]["owners"]["readiness"]["NumReturnAirPaths"],
        0
    );
    assert_eq!(cases[3]["event"]["owners"]["nodes"]["NumOfNodeLists"], 0);
    for case in &cases[4..7] {
        assert_eq!(case["event"]["locals"]["object_found"], 0);
    }
    owner.validate_initialized(&zones, &spaces)?;

    // Source-only branch table. These states were not observed by this Native
    // collector: no claim of InputProcessor prepresence/scratch/gain parity.
    for (name, inventory, accepted) in [
        ("supplied absent", vec![], true),
        ("supplied explicit empty", vec![("NodeList", 0)], true),
        ("positive NodeList", vec![("NodeList", 1)], false),
        (
            "positive connections",
            vec![("ZoneHVAC:EquipmentConnections", 1)],
            false,
        ),
        (
            "present-empty actual find",
            vec![("SpaceHVAC:ZoneEquipmentMixer", 0)],
            false,
        ),
    ] {
        let result = OrdinaryEquipmentInput::from_family_inventory(inventory, timesteps);
        assert_eq!(result.is_ok(), accepted, "{name}");
        if let Ok(input) = result {
            let mut branch = OrdinaryEquipment::new(input, &zones, &spaces)?;
            let supplied = branch
                .input
                .as_ref()
                .ok_or("actual retained legacy equipment projection")?
                .families[5];
            branch.get_zone_equipment(timesteps)?;
            assert_eq!(
                branch
                    .input
                    .as_ref()
                    .ok_or("actual retained legacy equipment projection")?
                    .families[5],
                match supplied {
                    FamilyShape::ObjectMap(count) => FamilyShape::ObjectMap(count),
                    _ => FamilyShape::Null,
                }
            );
            assert!(
                branch
                    .input
                    .as_ref()
                    .ok_or("actual retained legacy equipment projection")?
                    .families[6..]
                    .iter()
                    .all(|shape| *shape == FamilyShape::Absent)
            );
            branch.validate_initialized(&zones, &spaces)?;
        }
    }
    // The sole Config borrow mutates retained storage. The guarded getter's
    // subsequent return retains it; no copied flags owner or readiness bool.
    for flags in [
        [true, false, false],
        [false, true, false],
        [false, false, true],
    ] {
        owner.config_view()?.set_active_elements(0, flags);
        let row = &owner.zone_config.as_ref().ok_or("missing sole Config")?[0];
        assert_eq!(
            row.in_wall_active_element, flags[0],
            "wall is dispatcher index0"
        );
        assert_eq!(
            row.in_ceiling_active_element, flags[1],
            "roof/ceiling is dispatcher index1"
        );
        assert_eq!(
            row.in_floor_active_element, flags[2],
            "floor is dispatcher index2"
        );
        assert_eq!(owner.config_view()?.active_elements(0), flags);
    }
    let retained = owner.clone();
    owner.get_zone_equipment(timesteps)?;
    assert_eq!(owner, retained);
    assert_eq!(
        owner.config_view()?.active_elements(0),
        [false, false, true]
    );
    owner.config_view()?.set_active_elements(0, [false; 3]);
    owner.nodes.physical = Some(vec![SourceNodeData::default()]);
    assert_eq!(owner.nodes.physical.as_ref().map_or(0, Vec::len), 1);
    assert_eq!(owner.nodes.number_of_nodes, 0);
    assert_eq!(
        owner.validate_initialized(&zones, &spaces),
        Err(Unavailable::InconsistentOwner)
    );
    owner.nodes.physical = None;
    owner.validate_initialized(&zones, &spaces)?;

    // Authored Rust-only Source owner table, separate from the retained Native
    // singleton fixture above. Natural dense typed IDs and genuine GetZoneData
    // plus implicit GetSpaceData create the associations; no IDs are rewritten.
    use crate::heat_balance::error_backend::{
        SourceErrorBackend, SourceErrorState, SourceErrorTracking,
    };
    use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
    use crate::heat_balance::selected_heat_balance_input::zone::{
        SelectedZoneInputOwners, ZoneGetterOwners,
    };
    use crate::heat_balance::selected_heat_balance_input::{
        SelectedHeatBalanceInput, SelectedHeatBalanceInputState,
    };
    use crate::heat_balance::selected_input_processor::{
        SelectedInputProcessor, SourceInputFamily, SourceInputProcessorShortCut,
        SourceObjectDefinition, SourceObjectShape,
    };
    use crate::heat_balance::selected_project_data::ProjectInputObject;
    use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
    use crate::heat_balance::source_output_registration::{
        SourceOutputFileHost, SourceOutputRegistration,
    };
    use std::collections::BTreeMap;
    struct MemoryMtd(Option<Vec<u8>>);
    impl SourceOutputFileHost for MemoryMtd {
        fn ensure_mtd_open(&mut self) -> std::io::Result<()> {
            self.0.get_or_insert_with(Vec::new);
            Ok(())
        }
    }
    for total in [1_usize, 2, 3] {
        let control = SelectedHeatBalanceInputState::default();
        let mut model = ep_model::TypedModel::default();
        model.timestep.number_of_timesteps_per_hour = u32::try_from(timesteps)?;
        let mut raw_zones = Vec::new();
        for index in 0..total {
            let zone_id = ZoneId(u32::try_from(index)?);
            let space_id = SpaceId(u32::try_from(index)?);
            let name = format!("ZONE {}", index + 1);
            model.zones.push(ep_model::Zone {
                id: zone_id,
                name: ep_model::NormalizedName::new(&name),
                direction_of_relative_north_deg: 0.0,
                origin: ep_model::Point3 {
                    x_m: 0.0,
                    y_m: 0.0,
                    z_m: 0.0,
                },
                zone_type: 1,
                multiplier: 1,
                list_multiplier: 1,
                list_group: None,
                ceiling_height: ep_model::AutoOrNumber::AutoCalculate,
                volume: ep_model::AutoOrNumber::AutoCalculate,
                floor_area: ep_model::AutoOrNumber::AutoCalculate,
                inside_convection_algorithm: ep_model::ZoneConvectionAlgorithm::Inherited(
                    control.inside_convection_algorithm,
                ),
                outside_convection_algorithm: ep_model::ZoneConvectionAlgorithm::Inherited(
                    control.outside_convection_algorithm,
                ),
                is_part_of_total_floor_area: true,
                is_nominal_controlled: false,
                linked_outdoor_air_node: None,
                spaces: vec![space_id],
            });
            model.spaces.push(ep_model::Space {
                id: space_id,
                name: ep_model::NormalizedName::new(&name),
                zone: zone_id,
                ceiling_height: ep_model::AutoOrNumber::AutoCalculate,
                volume: ep_model::AutoOrNumber::AutoCalculate,
                floor_area: ep_model::AutoOrNumber::AutoCalculate,
                space_type: ep_model::NormalizedName::new("GENERAL"),
                space_type_id: ep_model::SpaceTypeId(0),
                tags: Vec::new(),
                origin: ep_model::SpaceOrigin::AutoZoneDefault,
            });
            raw_zones.push(ProjectInputObject {
                object_name: name,
                fields: BTreeMap::new(),
                idf_max_fields: None,
            });
        }
        let raw = SelectedHeatBalanceInput {
            source_is_epjson: true,
            objects: BTreeMap::from([("Zone".into(), raw_zones)]),
            ..Default::default()
        };
        // The same selected legacy_idd counts used by the production IP.
        let definitions = [
            ("Zone", 4, 9, 0, 0, None),
            ("Output:Variable", 4, 0, 0, 0, None),
            ("NodeList", 1, 0, 1, 0, Some("nodes")),
            ("ZoneHVAC:EquipmentList", 2, 0, 4, 2, Some("equipment")),
            ("ZoneHVAC:EquipmentConnections", 8, 0, 0, 0, None),
            ("SpaceHVAC:EquipmentConnections", 7, 0, 0, 0, None),
            ("AirLoopHVAC:SupplyPath", 2, 0, 2, 0, Some("components")),
            ("AirLoopHVAC:ReturnPath", 2, 0, 2, 0, Some("components")),
        ]
        .into_iter()
        .map(
            |(
                family,
                alpha_fields,
                numeric_fields,
                alpha_extensions,
                numeric_extensions,
                extension,
            )| SourceObjectDefinition {
                object_type: family.into(),
                alpha_fields,
                numeric_fields,
                alpha_extensions,
                numeric_extensions,
                extension: extension.map(str::to_owned),
            },
        )
        .collect();
        let families = BTreeMap::from([(
            "Zone".into(),
            SourceInputFamily::Objects(
                raw.objects["Zone"]
                    .iter()
                    .map(|row| SourceObjectShape {
                        object_name: row.object_name.clone(),
                        field_sizes: BTreeMap::new(),
                    })
                    .collect(),
            ),
        )]);
        let mut input = SelectedInputProcessor::from_admitted_input(definitions, families)?;
        let mut shortcut = SourceInputProcessorShortCut::default();
        shortcut.allocate_from_input(&input)?;
        let mut globals = SelectedHeatBalanceGlobals::default();
        let mut selected = SelectedZoneInputOwners::default();
        let mut output = SourceOutputRegistration::default();
        let prescan = SourceOutputPreScan::default();
        let report_input = ep_model::ReportInputs::default();
        let clock = crate::weather::day::WeatherGlobalState {
            minutes_in_time_step: i32::try_from(60 / timesteps)?,
            ..Default::default()
        };
        let developer = false;
        let mut files = MemoryMtd(None);
        let (mut error, mut console, mut eio) = (Vec::new(), Vec::new(), Vec::new());
        let mut error_state = SourceErrorState {
            output_error_header: true,
            version_string: "authored Rust-only equipment storage".into(),
            idd_version_string: "authored typed/raw Source owner table".into(),
            doing_input_processing: false,
            print_console_output: true,
            kickoff_simulation: false,
            developer_flag: false,
            warmup_flag: false,
            doing_sizing: false,
            tracking: SourceErrorTracking {
                ask_for_plant_check_on_abort: false,
                exit_during_simulations: false,
                ask_for_connections_report: false,
                ask_for_surfaces_report: false,
                total_multiplied_windows: 0,
                total_coincident_vertices: 0,
                total_degenerate_surfaces: 0,
                total_receiving_non_convex_surfaces: 0,
                total_casting_non_convex_surfaces: 0,
                match_counts: [0; 20],
                abort_processing: false,
                total_severe_errors: 0,
                last_severe_error: String::new(),
                total_severe_errors_during_warmup: 0,
                total_severe_errors_during_sizing: 0,
                total_warning_errors: 0,
                total_warning_errors_during_warmup: 0,
                total_warning_errors_during_sizing: 0,
            },
        };
        {
            let mut backend = SourceErrorBackend {
                state: &mut error_state,
                error_stream: Some(&mut error),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            selected.get_zone_data(
                &raw,
                &control,
                &model,
                &mut ZoneGetterOwners {
                    input: &mut input,
                    shortcut: &mut shortcut,
                    globals: &mut globals,
                    output: &mut output,
                    prescan: &prescan,
                    report_input: &report_input,
                    minimum_report_frequency: "",
                    developer: &developer,
                    clock: Some(&clock),
                    backend: &mut backend,
                    minutes_in_time_step: clock.minutes_in_time_step,
                    host: &mut files,
                    eio: &mut eio,
                },
            )?;
        }
        let (zone_ids, space_ids) = selected.source_implicit_space_ids(&model, &globals)?;
        assert_eq!(globals.num_of_zones, i32::try_from(total)?);
        assert_eq!(globals.num_spaces, i32::try_from(total)?);
        assert_eq!(
            zone_ids,
            model.zones.iter().map(|zone| zone.id).collect::<Vec<_>>()
        );
        assert_eq!(
            space_ids,
            model
                .spaces
                .iter()
                .map(|space| space.id)
                .collect::<Vec<_>>()
        );
        assert!(
            raw.objects["Zone"]
                .iter()
                .all(|row| !input.is_object_unused("Zone", &row.object_name))
        );
        assert!(files.0.is_some());
        assert!(error.is_empty() && console.is_empty() && eio.is_empty());
        let source_clock = OrdinaryEquipmentInput::from_source_family_inventory([], timesteps)?;
        if total > 1 {
            assert!(matches!(
                OrdinaryEquipment::new(source_clock, &zone_ids, &space_ids),
                Err(Unavailable::GeometryOrTimestep)
            ));
            assert!(matches!(
                OrdinaryEquipment::new_for_source(&model, source_clock),
                Err(Unavailable::GeometryOrTimestep)
            ));
        }
        let mut source =
            OrdinaryEquipment::new_for_source_ordered(&model, source_clock, &zone_ids, &space_ids)?;
        let mut gains = crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains::default();
        let mut schedules =
            crate::heat_balance::selected_schedule_manager::SelectedScheduleManager::default();
        let mut sizing = crate::heat_balance::selected_sizing::SelectedSizingManager::default();
        source.get_zone_equipment_source(
            &mut input,
            source_clock,
            false,
            false,
            &globals,
            &mut gains,
            SourceEquipmentInputOwners {
                zones: selected
                    .zones
                    .as_deref_mut()
                    .ok_or("actual Source Zone allocation")?,
                schedules: &mut schedules,
                sizing: &mut sizing,
                branch: None,
                source_is_epjson: true,
            },
        )?;
        assert_eq!(source.zone_ids, zone_ids);
        assert_eq!(source.space_ids, space_ids);
        assert_eq!(source.equipment_data_found, 1);
        assert_eq!(
            source.equipment_availability.as_deref(),
            Some(vec![0; total].as_slice())
        );
        let lists = source
            .equipment_lists
            .as_deref()
            .ok_or("actual Source equipment lists")?;
        assert_eq!(lists.len(), total);
        assert!(
            lists
                .iter()
                .all(|row| row.name.is_empty() && row.number_of_equipment_types == 0)
        );
        let configs = source
            .zone_config
            .as_deref()
            .ok_or("actual Source equipment configs")?;
        assert_eq!(configs.len(), total);
        assert_eq!(
            configs
                .iter()
                .map(|row| row.equipment_list_index)
                .collect::<Vec<_>>(),
            (0..total)
                .map(|index| if index == 0 { total } else { 0 })
                .collect::<Vec<_>>()
        );
        assert!(
            configs
                .iter()
                .all(|row| row.zone_name == "Uncontrolled Zone" && !row.is_controlled)
        );
        assert!(source.internal_gains.zone.is_none() && source.internal_gains.space.is_none());
        let retained_source = source.clone();
        let retained_input = input.clone();
        source.get_zone_equipment_source(
            &mut input,
            source_clock,
            false,
            false,
            &globals,
            &mut gains,
            SourceEquipmentInputOwners {
                zones: selected
                    .zones
                    .as_deref_mut()
                    .ok_or("same actual Source Zone allocation")?,
                schedules: &mut schedules,
                sizing: &mut sizing,
                branch: None,
                source_is_epjson: true,
            },
        )?;
        assert_eq!(source, retained_source);
        assert_eq!(input, retained_input);
    }
    Ok(())
}
