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
            let mut zone_errors_found = false;
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
                &mut zone_errors_found,
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
        // Unit94 authored owner checkpoint, after the unchanged Native empty
        // equipment fixture/table above. Real ZoneList getObjectItem consumes
        // ordered raw members, then real list targets read original report cells.
        {
            use crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::{
                SourceZoneAirPredictorCorrector, SourceZoneAirRegistrationChildren,
                SourceZoneListGroupLoadReports,
            };
            use crate::heat_balance::selected_input_processor::{
                SourceInputStatisticsInput, SourceInputStatisticsValue as RawValue,
            };
            use crate::heat_balance::source_output_registration::{
                SourceAirOutputField as AirField, SourceOutputStoreType, SourceRealOutputTarget,
                TimeStepKind,
            };
            let ordered_names = ["ALL", "REVERSE", "ZONE 1"];
            model.zone_lists.clear();
            model.zone_list_names = Default::default();
            let mut list_fields = BTreeMap::new();
            let mut list_shapes = Vec::new();
            for (ordinal, name) in ordered_names.into_iter().enumerate() {
                let members = if ordinal == 2 {
                    vec![0]
                } else if ordinal == 1 {
                    (0..total).rev().collect::<Vec<_>>()
                } else {
                    (0..total).collect::<Vec<_>>()
                };
                let typed_members = members
                    .iter()
                    .map(|&i| model.zones[i].id)
                    .collect::<Vec<_>>();
                let id = ep_model::ZoneListId(u32::try_from(ordinal)?);
                assert!(model.zone_list_names.insert(name, id).is_none());
                model.zone_lists.push(ep_model::ZoneList {
                    id,
                    name: ep_model::NormalizedName::new(name),
                    zones: typed_members,
                    max_zone_name_length: members
                        .iter()
                        .map(|&i| model.zones[i].name.0.as_str().len())
                        .max()
                        .ok_or("positive authored list")?,
                });
                let values = members
                    .iter()
                    .map(|&i| {
                        RawValue::Object(BTreeMap::from([(
                            "zone_name".into(),
                            RawValue::String(model.zones[i].name.0.as_str().into()),
                        )]))
                    })
                    .collect::<Vec<_>>();
                list_shapes.push(SourceObjectShape {
                    object_name: name.into(),
                    field_sizes: BTreeMap::from([("zones".into(), values.len())]),
                });
                list_fields.insert(
                    name.into(),
                    BTreeMap::from([("zones".into(), RawValue::Array(values))]),
                );
            }
            let variables = [
                (
                    "Zone List Sensible Heating Energy",
                    AirField::ZoneListSensibleHeatingEnergy,
                ),
                (
                    "Zone List Sensible Cooling Energy",
                    AirField::ZoneListSensibleCoolingEnergy,
                ),
                (
                    "Zone List Sensible Heating Rate",
                    AirField::ZoneListSensibleHeatingRate,
                ),
                (
                    "Zone List Sensible Cooling Rate",
                    AirField::ZoneListSensibleCoolingRate,
                ),
            ];
            let mut requested = ep_model::ReportInputs::default();
            for name in ordered_names {
                for (variable, _) in variables {
                    requested
                        .output_variables
                        .push(ep_model::SourceOutputVariableInput {
                            object_name: format!(
                                "LIST REQUEST {:02}",
                                requested.output_variables.len()
                            ),
                            key: name.into(),
                            variable_name: variable.into(),
                            reporting_frequency: "TIMESTEP".into(),
                            schedule_name: String::new(),
                            alpha_blank: [false, false, false, true],
                            default_applied: [false; 4],
                            num_alphas: 3,
                            is_epjson: true,
                        });
                }
            }
            let definitions = [
                SourceObjectDefinition {
                    object_type: "Zone".into(),
                    alpha_fields: 4,
                    numeric_fields: 9,
                    alpha_extensions: 0,
                    numeric_extensions: 0,
                    extension: None,
                },
                SourceObjectDefinition {
                    object_type: "ZoneList".into(),
                    alpha_fields: 1,
                    numeric_fields: 0,
                    alpha_extensions: 1,
                    numeric_extensions: 0,
                    extension: Some("zones".into()),
                },
                SourceObjectDefinition {
                    object_type: "Output:Variable".into(),
                    alpha_fields: 4,
                    numeric_fields: 0,
                    alpha_extensions: 0,
                    numeric_extensions: 0,
                    extension: None,
                },
            ]
            .into();
            let zone_fields = raw.objects["Zone"]
                .iter()
                .map(|row| (row.object_name.clone(), BTreeMap::new()))
                .collect::<BTreeMap<_, _>>();
            let request_fields = requested
                .output_variables
                .iter()
                .map(|row| (row.object_name.clone(), BTreeMap::new()))
                .collect::<BTreeMap<_, _>>();
            let families = BTreeMap::from([
                (
                    "Zone".into(),
                    SourceInputFamily::Objects(
                        zone_fields
                            .keys()
                            .map(|name| SourceObjectShape {
                                object_name: name.clone(),
                                field_sizes: BTreeMap::new(),
                            })
                            .collect(),
                    ),
                ),
                ("ZoneList".into(), SourceInputFamily::Objects(list_shapes)),
                (
                    "Output:Variable".into(),
                    SourceInputFamily::Objects(
                        request_fields
                            .keys()
                            .map(|name| SourceObjectShape {
                                object_name: name.clone(),
                                field_sizes: BTreeMap::new(),
                            })
                            .collect(),
                    ),
                ),
            ]);
            let mut ip = SelectedInputProcessor::from_admitted_input(definitions, families)?;
            ip.retain_source_input_statistics(SourceInputStatisticsInput {
                objects: BTreeMap::from([
                    ("Zone".into(), zone_fields),
                    ("ZoneList".into(), list_fields),
                    ("Output:Variable".into(), request_fields),
                ]),
                object_order: BTreeMap::from([
                    (
                        "Zone".into(),
                        raw.objects["Zone"]
                            .iter()
                            .map(|row| row.object_name.clone())
                            .collect(),
                    ),
                    (
                        "ZoneList".into(),
                        ordered_names.into_iter().map(str::to_owned).collect(),
                    ),
                    (
                        "Output:Variable".into(),
                        requested
                            .output_variables
                            .iter()
                            .map(|row| row.object_name.clone())
                            .collect(),
                    ),
                ]),
            })?;
            let mut local = SourceInputProcessorShortCut::default();
            local.allocate_from_input(&ip)?;
            let mut live = SelectedZoneInputOwners::default();
            let mut live_globals = SelectedHeatBalanceGlobals {
                doing_sizing: true, // actual caller cell differs from legacy backend false
                ..Default::default()
            };
            let mut registration = SourceOutputRegistration::default();
            let mut scan = SourceOutputPreScan::default();
            let (mut list_errors, mut list_console, mut list_eio, mut list_eso) =
                (Vec::new(), Vec::new(), Vec::new(), Vec::new());
            error_state.output_error_header = false;
            let mut backend = SourceErrorBackend {
                state: &mut error_state,
                error_stream: Some(&mut list_errors),
                console: &mut list_console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            for name in ordered_names {
                for (variable, _) in variables {
                    scan.add_record(name, variable, &mut backend)?;
                }
            }
            let mut found = false;
            live.get_zone_data(
                &raw,
                &control,
                &model,
                &mut ZoneGetterOwners {
                    input: &mut ip,
                    shortcut: &mut local,
                    globals: &mut live_globals,
                    output: &mut registration,
                    prescan: &scan,
                    report_input: &requested,
                    minimum_report_frequency: "",
                    developer: &developer,
                    clock: Some(&clock),
                    backend: &mut backend,
                    minutes_in_time_step: clock.minutes_in_time_step,
                    host: &mut files,
                    eio: &mut list_eio,
                },
                &mut found,
            )?;
            assert!(!found);
            assert_eq!(live.num_zone_lists, 3);
            assert_eq!(live.num_zone_groups, 0);
            let lists = live.zone_lists.as_ref().ok_or("actual list allocation")?;
            assert_eq!(
                lists
                    .iter()
                    .map(|row| row.name.as_str())
                    .collect::<Vec<_>>(),
                ordered_names
            );
            assert_eq!(
                lists[0].zone.as_deref(),
                Some((1..=i32::try_from(total)?).collect::<Vec<_>>().as_slice())
            );
            assert_eq!(
                lists[1].zone.as_deref(),
                Some(
                    (1..=i32::try_from(total)?)
                        .rev()
                        .collect::<Vec<_>>()
                        .as_slice()
                )
            );
            assert_eq!(lists[2].zone.as_deref(), Some([1].as_slice()));
            assert!(
                lists
                    .iter()
                    .all(|row| row.max_zone_name_length == 6 && row.typed_zone_list_id.is_some())
            );
            assert_eq!(live.source_zone_list_ids(&model)?.len(), 3);
            assert_eq!(backend.state.tracking.total_warning_errors_during_sizing, 1);
            assert_eq!(backend.state.tracking.total_warning_errors, 1);
            assert!(!ip.is_object_unused("ZoneList", "ALL"));
            assert_eq!(local.current_module_object, "ZoneProperty:LocalEnvironment");
            assert_eq!(local.alpha_args[0], "ZONE 1");
            assert_eq!(local.alpha_args[1], "ZONE 1");
            assert_eq!(local.alpha_field_names[1], "zone_name");
            assert!(local.numeric_args.iter().all(|v| *v == 0.0));
            let mut controller = SourceZoneAirPredictorCorrector::default();
            controller.demand.zones = Some(vec![Default::default(); total]);
            for (i, demand) in controller
                .demand
                .zones
                .as_mut()
                .ok_or("actual demand allocation")?
                .iter_mut()
                .enumerate()
            {
                let ordinal = f64::from(u32::try_from(i + 1)?);
                demand.air_sys_heat_energy = ordinal * 10.0;
                demand.air_sys_cool_energy = ordinal * 20.0;
                demand.air_sys_heat_rate = if total == 3 {
                    [5.0e15, -2.5e15, 0.5][i]
                } else {
                    ordinal
                };
                demand.air_sys_cool_rate = ordinal * 3.0;
            }
            for (i, zone) in live
                .zones
                .as_mut()
                .ok_or("actual live Zones")?
                .iter_mut()
                .enumerate()
            {
                zone.multiplier = if i == 1 { 4 } else { 2 };
                zone.list_multiplier = 17; // must never enter Native2220
            }
            let mut reports = SourceZoneListGroupLoadReports {
                list_heat_energy: Some(vec![999.0; 3]),
                list_cool_energy: Some(vec![999.0; 3]),
                list_heat_rate: Some(vec![999.0; 3]),
                list_cool_rate: Some(vec![999.0; 3]),
                ..Default::default()
            };
            crate::heat_balance::ordinary_hvac_manager::SourceHvacManager::sum_source_zone_list_loads(
                &live,&controller.demand,&mut reports)?;
            let first = reports.clone();
            crate::heat_balance::ordinary_hvac_manager::SourceHvacManager::sum_source_zone_list_loads(
                &live,&controller.demand,&mut reports)?;
            assert_eq!(
                reports, first,
                "real reset prevents accumulation across calls"
            );
            let expected_energy = [20.0, 100.0, 160.0][total - 1];
            assert_eq!(
                reports.list_heat_energy.as_ref().ok_or("list energy")?,
                &vec![expected_energy, expected_energy, 20.0]
            );
            if total == 3 {
                assert_eq!(
                    reports.list_heat_rate.as_ref().ok_or("ordered heat rate")?,
                    &vec![1.0, 0.0, 1.0e16]
                );
            }
            let flags = crate::heat_balance::error_backend::SourceDiagnosticFlags {
                kickoff: &live_globals.kickoff_simulation,
                developer: &developer,
                warmup: &clock.warmup_flag,
                doing_sizing: &live_globals.doing_sizing,
            };
            let mut output_variables = Vec::new();
            let mut dictionary =
                crate::heat_balance::source_output_registration::SourceOutputDictionaryStreams {
                    eso: Some(&mut list_eso),
                    sqlite_present: false,
                    results_framework_output_variables: &mut output_variables,
                };
            {
                let mut host=crate::heat_balance::ordinary_air_heat_balance::output_registration::SourceInitAirOutputHost {
                    registration:&mut registration,prescan:&scan,reports:&requested,minimum_report_frequency:"",
                    host:&mut files,dictionary_host:&mut dictionary,
                };
                for (i, name) in ordered_names.into_iter().enumerate() {
                    host.register_zone_list(
                        i32::try_from(i + 1)?,
                        name,
                        &reports,
                        &clock,
                        Some(flags),
                        &mut ip,
                        &mut backend,
                        &mut list_eio,
                    )?;
                }
            }
            assert_eq!(
                registration
                    .out_vars
                    .iter()
                    .filter(|row| row.report)
                    .count(),
                12
            );
            assert_eq!(
                std::str::from_utf8(&list_errors)?
                    .lines()
                    .collect::<Vec<_>>(),
                [
                    "   ** Warning ** GetZoneData: ZoneList=\"ZONE 1\":  is a duplicate of a zone name.",
                    "   **   ~~~   ** This could be a problem in places where either a Zone Name or a Zone List can be used.",
                ]
            );
            let air =
                crate::heat_balance::ordinary_air_heat_balance::SourceAirHeatBalance::default();
            let fan=crate::heat_balance::ordinary_air_heat_balance::environment::SourceZoneFanHeatBalanceArrays::default();
            let air_report=crate::heat_balance::ordinary_air_heat_balance::predictor_corrector::simple_air::SourceAirReportOutputOwners { controller:&controller };
            let resilience =
                crate::heat_balance::ordinary_surface_management::SourceResilienceOutputOwners {
                    zones: &live,
                };
            let read=|reports:&SourceZoneListGroupLoadReports| -> Result<Vec<f64>,Box<dyn std::error::Error>> {
                let owners=crate::heat_balance::ordinary_air_heat_balance::output_registration::SourceAirOutputOwners {
                    air:&air,demand:&controller.demand,fan:&fan,zone_lists:reports,report:&air_report,resilience:&resilience,
                };
                let mut values=Vec::new();
                for (i,row) in registration.out_vars.iter().enumerate() {
                    if !row.report { continue; }
                    assert_eq!(row.time_step_type,TimeStepKind::System);
                    assert_eq!(row.store_type,if row.name.ends_with("Energy") {SourceOutputStoreType::Sum}else{SourceOutputStoreType::Average});
                    assert!(matches!(row.which,Some(SourceRealOutputTarget::Air { space:false,.. })));
                    values.push(*registration.read_registered_real(i,&owners)?);
                }
                Ok(values)
            };
            let before = read(&reports)?;
            reports
                .list_heat_energy
                .as_mut()
                .ok_or("actual mutable report owner")?[0] += 7.0;
            let after = read(&reports)?;
            assert_eq!(
                after[0],
                before[0] + 7.0,
                "registered Which follows original cell"
            );
            assert_eq!(&after[1..], &before[1..]);
        }
    }
    Ok(())
}
