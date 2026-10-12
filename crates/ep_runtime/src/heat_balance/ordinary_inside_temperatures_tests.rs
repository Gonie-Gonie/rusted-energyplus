use super::*;
use crate::heat_balance::inside_convection_layout::{
    InsideConvectionAssociation, InsideConvectionGroup,
};
use ep_model::SpaceId;

fn layout() -> Result<InsideConvectionLayout, Box<dyn std::error::Error>> {
    Ok(InsideConvectionLayout::new(
        vec![ZoneId(7)],
        vec![(SpaceId(3), ZoneId(7)), (SpaceId(1), ZoneId(7))],
        vec![InsideConvectionAssociation {
            surface_id: SurfaceId(8),
            zone_id: ZoneId(7),
            space_id: SpaceId(3),
        }],
        vec![
            InsideConvectionGroup {
                zone_id: ZoneId(7),
                space_id: SpaceId(1),
                surfaces: vec![],
            },
            InsideConvectionGroup {
                zone_id: ZoneId(7),
                space_id: SpaceId(3),
                surfaces: vec![SurfaceId(8)],
            },
        ],
    )
    .map_err(|_| "invalid authored layout")?)
}

#[test]
fn solved_zt_is_separate_from_space_and_old_and_wrong_zone_retains_prefix()
-> Result<(), Box<dyn std::error::Error>> {
    let mut owner =
        OrdinaryInsideTemperatures::allocate(layout()?, NoUserConvectionInput::new([0; 6])?, 0.1)
            .map_err(|_| "allocation unavailable")?;
    assert_eq!(owner.inside_old, vec![(SurfaceId(8), 0.0)]);
    assert_eq!(owner.space_air.rows()[0].space_id, SpaceId(3));
    let mut declared = owner.space_air.rows().to_vec();
    declared[0].mat = -7.0;
    declared[0].zt = 31.0;
    declared[0].ztav = -0.0;
    owner.space_air.write_all(&owner.layout, &declared)?;
    owner
        .retain_solution(ZoneId(7), f64::from_bits(0x8000_0000_0000_0000))
        .map_err(|_| "actual solution unavailable")?;
    let before = owner.clone();
    assert!(owner.retain_solution(ZoneId(2), 400.0).is_err());
    assert_eq!(owner, before);
    assert_eq!(owner.zone_zt.1.to_bits(), (-0.0_f64).to_bits());
    assert_eq!(owner.space_air.rows()[0].mat, -7.0);
    assert_eq!(owner.space_air.rows()[0].zt, 31.0);
    assert_eq!(
        owner.space_air.rows()[0].ztav.to_bits(),
        (-0.0_f64).to_bits()
    );
    Ok(())
}

#[test]
fn adaptive_and_multiple_inside_passes_are_explicitly_unavailable() {
    let options = super::super::state::HeatBalanceSimulationOptions::hourly_samples(1);
    let config = options.zone_air_algorithm.runtime_config();
    assert!(admit_config(config, 1).is_ok());
    assert!(admit_config(config, 2).is_err());
    let compatibility =
        super::super::algorithm::CompatibilityHeatBalanceAlgorithm::SourceOrder1ZoneOpaqueCompat
            .runtime_config();
    assert!(admit_config(compatibility, 1).is_err());
}

#[test]
fn distinct_space_mat_supplies_complete_surface_map_and_rejects_stale_association()
-> Result<(), Box<dyn std::error::Error>> {
    let two_spaces = |second| {
        InsideConvectionLayout::new(
            vec![ZoneId(7)],
            vec![(SpaceId(3), ZoneId(7)), (second, ZoneId(7))],
            vec![
                InsideConvectionAssociation {
                    surface_id: SurfaceId(8),
                    zone_id: ZoneId(7),
                    space_id: SpaceId(3),
                },
                InsideConvectionAssociation {
                    surface_id: SurfaceId(9),
                    zone_id: ZoneId(7),
                    space_id: second,
                },
            ],
            vec![
                InsideConvectionGroup {
                    zone_id: ZoneId(7),
                    space_id: SpaceId(3),
                    surfaces: vec![SurfaceId(8)],
                },
                InsideConvectionGroup {
                    zone_id: ZoneId(7),
                    space_id: second,
                    surfaces: vec![SurfaceId(9)],
                },
            ],
        )
    };
    let mut owner = OrdinaryInsideTemperatures::allocate(
        two_spaces(SpaceId(1)).map_err(|_| "invalid input layout")?,
        NoUserConvectionInput::new([0; 6])?,
        0.1,
    )
    .map_err(|_| "allocation unavailable")?;
    let mut rows = owner.space_air.rows().to_vec();
    rows[0].mat = 11.0;
    rows[1].mat = 29.0;
    rows[1].ztav = -0.0;
    owner.space_air.write_all(&owner.layout, &rows)?;
    owner
        .retain_solution(ZoneId(7), 99.0)
        .map_err(|_| "solution unavailable")?;
    let before = owner.clone();
    assert_eq!(
        owner
            .reference_air_by_layout()
            .map_err(|_| "map unavailable")?,
        BTreeMap::from([(SurfaceId(8), 11.0), (SurfaceId(9), 29.0)])
    );
    assert_eq!(owner, before);
    owner.layout = two_spaces(SpaceId(2)).map_err(|_| "invalid stale input layout")?;
    assert!(owner.reference_air_by_layout().is_err());
    assert_eq!(owner.space_air.rows(), rows);
    Ok(())
}

// Replay actual selected Native before/after owners through the private
// retained-temperature operations. Native values are test data only.
use serde_json::Value;

fn observed_bits(value: &Value) -> u64 {
    u64::from_str_radix(value["value_bits"].as_str().expect("observed raw bits"), 16)
        .expect("validated Native scalar bits")
}

fn observed_id(value: &Value) -> u32 {
    value
        .as_u64()
        .expect("observed positive ID")
        .try_into()
        .expect("typed ID range")
}

#[test]
fn actual_native_before_owners_replay_selected_retained_temperature_operations()
-> Result<(), Box<dyn std::error::Error>> {
    use ep_model::{
        AutoOrNumber, Construction, ConstructionId, ConstructionKind, Material, MaterialDefinition,
        MaterialId, MaterialSurfaceRoughness, NoMassMaterial, NormalizedName,
        OpaqueSurfaceProperties, OutsideBoundaryCondition, Point3, SimulationModel, Space,
        SpaceOrigin, SpaceTypeId, SunExposure, Surface, SurfaceType, WindExposure, Zone,
    };
    // Authored thermal primitives provide initialized state templates only. They
    // are not Native numerical expectations and do not expose another factory API.
    let mut template = TypedModel::default();
    template.materials.push(Material {
        id: MaterialId(0),
        name: NormalizedName::new("template R"),
        definition: MaterialDefinition::NoMass(NoMassMaterial {
            roughness: MaterialSurfaceRoughness::Rough,
            thermal_resistance_m2_k_per_w: 1.0,
            surface: OpaqueSurfaceProperties::default(),
        }),
    });
    template.constructions.push(Construction {
        id: ConstructionId(0),
        name: NormalizedName::new("template C"),
        kind: ConstructionKind::Opaque,
        outside_layer: Some(MaterialId(0)),
        layers: vec![MaterialId(0)],
        thermochromic_master: None,
        ground_factor: None,
        air_boundary: None,
        complex_fenestration: None,
        window_equivalent_layer: None,
        internal_heat_source: None,
    });
    template.zones.push(Zone {
        id: ZoneId(0),
        name: NormalizedName::new("template Z"),
        direction_of_relative_north_deg: 0.0,
        origin: Point3 {
            x_m: 0.0,
            y_m: 0.0,
            z_m: 0.0,
        },
        zone_type: 1,
        multiplier: 1,
        list_multiplier: 1,
        list_group: None,
        ceiling_height: AutoOrNumber::Value(1.0),
        volume: AutoOrNumber::Value(1.0),
        floor_area: AutoOrNumber::Value(1.0),
        inside_convection_algorithm: ep_model::ZoneConvectionAlgorithm::Inherited(
            ep_model::InsideSurfaceConvectionAlgorithm::Tarp,
        ),
        outside_convection_algorithm: ep_model::ZoneConvectionAlgorithm::Inherited(
            ep_model::OutsideSurfaceConvectionAlgorithm::Doe2,
        ),
        is_part_of_total_floor_area: true,
        is_nominal_controlled: false,
        linked_outdoor_air_node: None,
        spaces: vec![SpaceId(0)],
    });
    template.spaces.push(Space {
        id: SpaceId(0),
        name: NormalizedName::new("template S"),
        zone: ZoneId(0),
        ceiling_height: AutoOrNumber::AutoCalculate,
        volume: AutoOrNumber::AutoCalculate,
        floor_area: AutoOrNumber::AutoCalculate,
        space_type: NormalizedName::new("GENERAL"),
        space_type_id: SpaceTypeId(0),
        tags: Vec::new(),
        origin: SpaceOrigin::Authored,
    });
    template.surfaces.push(Surface {
        id: SurfaceId(0),
        name: NormalizedName::new("template wall"),
        surface_type: SurfaceType::Wall,
        construction: ConstructionId(0),
        zone: ZoneId(0),
        space: SpaceId(0),
        outside_boundary_condition: OutsideBoundaryCondition::Outdoors,
        outside_boundary_condition_object: None,
        sun_exposure: SunExposure::NoSun,
        wind_exposure: WindExposure::NoWind,
        view_factor_to_ground: AutoOrNumber::Value(0.0),
        computed_geometry: None,
        vertices: vec![
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 1.0,
                y_m: 0.0,
                z_m: 0.0,
            },
            Point3 {
                x_m: 1.0,
                y_m: 0.0,
                z_m: 1.0,
            },
            Point3 {
                x_m: 0.0,
                y_m: 0.0,
                z_m: 1.0,
            },
        ],
    });
    let shell = crate::heat_balance::initialization::initialize_heat_balance_state(
        &SimulationModel::from_typed(template.clone()),
        23.0,
    )?;
    let zone_template = shell
        .zones
        .first()
        .ok_or("missing initialized Zone template")?;
    let surface_template = shell
        .surfaces
        .first()
        .ok_or("missing initialized surface template")?;

    // The frozen fixture contains actual before/after Native observations.
    let fixture: Value = serde_json::from_str(include_str!(
        "fixtures/ordinary_temperature_selected_cases.json"
    ))?;
    assert_eq!(
        fixture["schema"],
        "src06-actual-native-temperature-transition-cases.v1"
    );
    let cases = fixture["cases"].as_array().ok_or("missing actual cases")?;
    let allocation = cases
        .iter()
        .find(|case| case["operation"] == "temperature_surface_allocation_complete")
        .ok_or("missing actual allocation case")?;
    let geometry = allocation["event"]["owners"]["surface_geometry"]["values"]
        .as_array()
        .ok_or("missing full observed surface associations")?;
    let mut observed_operations = std::collections::BTreeSet::new();
    for case in cases {
        let operation = case["operation"].as_str().ok_or("missing operation")?;
        let pre = case.get("before").unwrap_or(&case["event"]);
        let post = case.get("after").unwrap_or(pre);
        let air = &pre["owners"]["air"];
        let zone_geometry = air["zone_geometry"]
            .as_array()
            .ok_or("missing observed zones")?;
        assert_eq!(zone_geometry.len(), 1, "bounded one-zone owner");
        let zone_id = ZoneId(observed_id(&zone_geometry[0]["index"]));
        let spaces = air["space_geometry"]
            .as_array()
            .ok_or("missing observed spaces")?;
        let mut typed = template.clone();
        typed.zones[0].id = zone_id;
        typed.zones[0].spaces = zone_geometry[0]["spaceIndexes"]
            .as_array()
            .ok_or("missing zone-space order")?
            .iter()
            .map(|id| SpaceId(observed_id(id)))
            .collect();
        typed.spaces = spaces
            .iter()
            .map(|row| {
                let mut space = template.spaces[0].clone();
                space.id = SpaceId(observed_id(&row["index"]));
                space.zone = ZoneId(observed_id(&row["zoneNum"]));
                space
            })
            .collect();
        typed.surfaces = geometry
            .iter()
            .map(|row| {
                let mut surface = template.surfaces[0].clone();
                surface.id = SurfaceId(observed_id(&row["index"]));
                surface.zone = ZoneId(observed_id(&row["Zone"]));
                surface.space = SpaceId(observed_id(&row["spaceNum"]));
                surface
            })
            .collect();
        let layout = InsideConvectionLayout::from_typed_model(&typed)
            .map_err(|_| "observed layout unavailable")?;
        let mut owner =
            OrdinaryInsideTemperatures::allocate(layout, NoUserConvectionInput::new([0; 6])?, 0.1)
                .map_err(|_| "allocation unavailable")?;
        if operation == "temperature_surface_allocation_complete" {
            let old = pre["owners"]["surface_arrays"]["SurfTempInsOld"]["values"]
                .as_array()
                .ok_or("missing allocated Old")?;
            assert_eq!(owner.inside_old.len(), old.len());
            for (&(id, value), expected) in owner.inside_old.iter().zip(old) {
                assert_eq!(id, SurfaceId(observed_id(&expected["index"])));
                assert_eq!(value.to_bits(), observed_bits(&expected["value"]));
            }
            observed_operations.insert(operation);
            continue;
        }
        let mut zones = vec![zone_template.clone()];
        zones[0].zone_id = zone_id;
        let fields = &air["zone_owners"]["values"][0]["fields"];
        zones[0].mean_air_temperature_c = f64::from_bits(observed_bits(&fields["MAT"]));
        zones[0].zone_timestep_average_air_temperature_c =
            f64::from_bits(observed_bits(&fields["ZTAV"]));
        owner
            .retain_solution(zone_id, f64::from_bits(observed_bits(&fields["ZT"])))
            .map_err(|_| "test-local BEFORE ZT unavailable")?;
        let mut space_rows = owner.space_air.rows().to_vec();
        let native_spaces = air["space_owners"]["values"]
            .as_array()
            .ok_or("missing before spaces")?;
        assert_eq!(space_rows.len(), native_spaces.len());
        for (row, actual) in space_rows.iter_mut().zip(native_spaces) {
            assert_eq!(row.space_id, SpaceId(observed_id(&actual["index"])));
            row.mat = f64::from_bits(observed_bits(&actual["fields"]["MAT"]));
            row.zt = f64::from_bits(observed_bits(&actual["fields"]["ZT"]));
            row.ztav = f64::from_bits(observed_bits(&actual["fields"]["ZTAV"]));
        }
        owner.space_air.write_all(&owner.layout, &space_rows)?;
        let mut surfaces = typed
            .surfaces
            .iter()
            .map(|input| {
                let mut surface = surface_template.clone();
                surface.surface_id = input.id;
                surface.zone_id = input.zone;
                surface
            })
            .collect::<Vec<_>>();
        if let Some(incoming) = pre["owners"]["surface_arrays"]["SurfTempIn"]["values"].as_array() {
            let old = pre["owners"]["surface_arrays"]["SurfTempInsOld"]["values"]
                .as_array()
                .ok_or("missing before Old")?;
            assert_eq!(surfaces.len(), incoming.len());
            assert_eq!(owner.inside_old.len(), old.len());
            for ((surface, inside_old), (incoming, old)) in surfaces
                .iter_mut()
                .zip(&mut owner.inside_old)
                .zip(incoming.iter().zip(old))
            {
                assert_eq!(
                    surface.surface_id,
                    SurfaceId(observed_id(&incoming["index"]))
                );
                assert_eq!(inside_old.0, SurfaceId(observed_id(&old["index"])));
                surface.inside_face_temperature_c =
                    f64::from_bits(observed_bits(&incoming["value"]));
                inside_old.1 = f64::from_bits(observed_bits(&old["value"]));
            }
        }
        match operation {
            "temperature_air_reset_before" => owner
                .begin_environment(&mut zones)
                .map_err(|_| "reset unavailable")?,
            "temperature_mat_commit_before" => {
                assert_eq!(
                    air["selection"]["requested_space"], 0,
                    "space correction is outside admitted A"
                );
                owner
                    .commit_zone_mat(&mut zones)
                    .map_err(|_| "MAT commit unavailable")?;
            }
            "temperature_space_copy_group" => {
                // Root preserves first BEFORE and last AFTER of all actual
                // Zone.spaceIndexes pairs in one invocation, including singleton.
                owner
                    .after_full_correction(&zones)
                    .map_err(|_| "space copy unavailable")?;
            }
            "temperature_old_copy_before" => owner
                .capture_inside_entry(&surfaces)
                .map_err(|_| "Old copy unavailable")?,
            "temperature_reference_returned" => {
                let surface = &pre["owners"]["surface"];
                assert!(
                    matches!(surface["SurfTAirRef"].as_i64(), Some(-1) | Some(0)),
                    "selected Original default/ZoneMeanAirTemp branches only"
                );
                let actual = owner
                    .reference_air_by_layout()
                    .map_err(|_| "shared MAT map unavailable")?;
                let value = actual[&SurfaceId(observed_id(&surface["index"]))].to_bits();
                assert_eq!(value, observed_bits(&pre["locals"]["RefAirTemp"]));
                assert_eq!(
                    value,
                    observed_bits(&post["owners"]["surface"]["RefAirTemp"])
                );
                assert_eq!(
                    value,
                    observed_bits(&post["owners"]["surface"]["SurfTempEffBulkAir"])
                );
            }
            _ => return Err(format!("unreviewed fixture operation: {operation}").into()),
        }
        let after = &post["owners"]["air"];
        let fields = &after["zone_owners"]["values"][0]["fields"];
        assert_eq!(
            owner.zone_zt.1.to_bits(),
            observed_bits(&fields["ZT"]),
            "{operation}: ZoneZT"
        );
        assert_eq!(
            zones[0].mean_air_temperature_c.to_bits(),
            observed_bits(&fields["MAT"]),
            "{operation}: ZoneMAT"
        );
        assert_eq!(
            zones[0].zone_timestep_average_air_temperature_c.to_bits(),
            observed_bits(&fields["ZTAV"]),
            "{operation}: ZoneZTAV"
        );
        let expected = after["space_owners"]["values"]
            .as_array()
            .ok_or("missing after spaces")?;
        assert_eq!(owner.space_air.rows().len(), expected.len());
        for (row, expected) in owner.space_air.rows().iter().zip(expected) {
            assert_eq!(row.space_id, SpaceId(observed_id(&expected["index"])));
            assert_eq!(
                row.mat.to_bits(),
                observed_bits(&expected["fields"]["MAT"]),
                "{operation}: SpaceMAT"
            );
            assert_eq!(
                row.zt.to_bits(),
                observed_bits(&expected["fields"]["ZT"]),
                "{operation}: SpaceZT"
            );
            assert_eq!(
                row.ztav.to_bits(),
                observed_bits(&expected["fields"]["ZTAV"]),
                "{operation}: SpaceZTAV"
            );
        }
        if let Some(incoming) = post["owners"]["surface_arrays"]["SurfTempIn"]["values"].as_array()
        {
            let old = post["owners"]["surface_arrays"]["SurfTempInsOld"]["values"]
                .as_array()
                .ok_or("missing after Old")?;
            assert_eq!(surfaces.len(), incoming.len());
            assert_eq!(owner.inside_old.len(), old.len());
            for ((surface, inside_old), (incoming, old)) in surfaces
                .iter()
                .zip(&owner.inside_old)
                .zip(incoming.iter().zip(old))
            {
                assert_eq!(
                    surface.inside_face_temperature_c.to_bits(),
                    observed_bits(&incoming["value"]),
                    "{operation}: In"
                );
                assert_eq!(
                    inside_old.1.to_bits(),
                    observed_bits(&old["value"]),
                    "{operation}: Old"
                );
            }
        }
        observed_operations.insert(operation);
    }
    assert_eq!(
        observed_operations,
        std::collections::BTreeSet::from([
            "temperature_surface_allocation_complete",
            "temperature_air_reset_before",
            "temperature_mat_commit_before",
            "temperature_space_copy_group",
            "temperature_reference_returned",
            "temperature_old_copy_before"
        ])
    );
    Ok(())
}
