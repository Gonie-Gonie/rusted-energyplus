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
            owner.nodes.physical.as_ref().map_or(0, NodeStateStore::len),
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
            let supplied = branch.input.families[5];
            branch.get_zone_equipment(timesteps)?;
            assert_eq!(
                branch.input.families[5],
                match supplied {
                    FamilyShape::ObjectMap(count) => FamilyShape::ObjectMap(count),
                    _ => FamilyShape::Null,
                }
            );
            assert!(
                branch.input.families[6..]
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
    owner.nodes.physical = Some(NodeStateStore {
        air_nodes: Vec::new(),
    });
    assert_eq!(owner.nodes.number_of_nodes, 0);
    assert_eq!(
        owner.validate_initialized(&zones, &spaces),
        Err(Unavailable::InconsistentOwner)
    );
    owner.nodes.physical = None;
    owner.validate_initialized(&zones, &spaces)?;
    Ok(())
}
