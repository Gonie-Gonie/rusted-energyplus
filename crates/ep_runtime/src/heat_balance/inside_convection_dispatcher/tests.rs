use super::*;
use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
use crate::heat_balance::inside_convection_layout::{
    InsideConvectionAssociation, InsideConvectionGroup,
};

fn layout() -> Result<InsideConvectionLayout, InsideConvectionUnavailable> {
    Ok(InsideConvectionLayout::new(
        vec![ZoneId(7)],
        vec![(SpaceId(3), ZoneId(7)), (SpaceId(1), ZoneId(7))],
        vec![
            InsideConvectionAssociation {
                surface_id: SurfaceId(8),
                zone_id: ZoneId(7),
                space_id: SpaceId(3),
            },
            InsideConvectionAssociation {
                surface_id: SurfaceId(2),
                zone_id: ZoneId(7),
                space_id: SpaceId(1),
            },
        ],
        vec![
            InsideConvectionGroup {
                zone_id: ZoneId(7),
                space_id: SpaceId(1),
                surfaces: vec![SurfaceId(2)],
            },
            InsideConvectionGroup {
                zone_id: ZoneId(7),
                space_id: SpaceId(3),
                surfaces: vec![SurfaceId(8)],
            },
        ],
    )?)
}
fn zones() -> Vec<InsideConvectionZone> {
    vec![InsideConvectionZone {
        zone_id: ZoneId(7),
        tarp: true,
        ceiling_height: 3.0,
        volume: 90.0,
        floor_area: 30.0,
        xy_bounds: [0.0, 6.0, 0.0, 5.0],
        exterior_gross_wall_area: 33.0,
        exterior_window_area: 0.0,
        multiplier: 1,
        list_multiplier: 1,
    }]
}
fn surfaces() -> Vec<InsideConvectionSurface> {
    [SurfaceId(8), SurfaceId(2)]
        .into_iter()
        .map(|surface_id| InsideConvectionSurface {
            surface_id,
            class: SurfaceType::Wall,
            exterior: true,
            heat_transfer: true,
            construction_is_window: false,
            kiva: false,
            representative_calculations: false,
            ems_override: false,
            outward_cos_tilt: 0.0,
            tilt: 90.0,
            azimuth: 0.0,
            area: 9.0,
            gross_area: 9.0,
            width: 3.0,
            perimeter: 12.0,
            vertices: vec![
                [0.0, 0.0, 0.0],
                [3.0, 0.0, 0.0],
                [3.0, 0.0, 3.0],
                [0.0, 0.0, 3.0],
            ],
        })
        .collect()
}
fn state(
    layout: &InsideConvectionLayout,
) -> Result<(InsideConvectionState, [(SurfaceId, f64); 2]), InsideConvectionUnavailable> {
    let state = InsideConvectionState::new(
        layout,
        vec![
            InsideConvectionSurfaceState::new(SurfaceId(8), [11, 12]),
            InsideConvectionSurfaceState::new(SurfaceId(2), [21, 22]),
        ],
    )?;
    Ok((state, [(SurfaceId(8), -7.0), (SurfaceId(2), -8.0)]))
}
fn readiness() -> InsideConvectionReadiness {
    InsideConvectionReadiness {
        begin_environment: true,
        system_sizing: false,
        zone_sizing: false,
        equipment_inputs_filled: false,
        nodes_allocated: false,
        equipment_simulated_once: false,
    }
}
fn error_state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: "declared test version".into(),
        idd_version_string: "declared test IDD".into(),
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
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}
fn run(
    layout: &InsideConvectionLayout,
    state: &mut InsideConvectionState,
    zone_equipment: &mut [(ZoneId, [bool; 3])],
    coefficients: &mut [(SurfaceId, f64)],
    space_air: &SpaceAirTemperatures,
    temperatures: InsideSurfaceTemperatures<'_>,
    readiness: InsideConvectionReadiness,
    low: f64,
    reports: &ReportInputs,
    scan: &mut ReportScanState,
) -> Result<(), InsideConvectionError> {
    let mut error = error_state();
    let mut stream = Vec::new();
    let mut console = Vec::new();
    let mut backend = SourceErrorBackend {
        state: &mut error,
        error_stream: Some(&mut stream),
        console: &mut console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    initialize_inside_convection(
        layout,
        &zones(),
        &surfaces(),
        NoUserConvectionInput::new([0; 6])?,
        state,
        zone_equipment,
        coefficients,
        space_air,
        temperatures,
        readiness,
        low,
        reports,
        scan,
        &mut backend,
    )
}

#[test]
fn true_first_call_executes_static_setup_and_separate_space_mat()
-> Result<(), Box<dyn std::error::Error>> {
    let layout = layout()?;
    let (mut state, mut coefficients) = state(&layout)?;
    let mut active_elements = [(ZoneId(7), [true; 3])];
    let mut space_air = SpaceAirTemperatures::allocate(&layout);
    space_air.write_all(
        &layout,
        &[
            SpaceAirTemperature {
                space_id: SpaceId(3),
                zone_id: ZoneId(7),
                mat: 18.0,
                zt: 81.0,
                ztav: 82.0,
            },
            SpaceAirTemperature {
                space_id: SpaceId(1),
                zone_id: ZoneId(7),
                mat: 26.0,
                zt: 91.0,
                ztav: 92.0,
            },
        ],
    )?;
    let rows = [(SurfaceId(8), 24.0), (SurfaceId(2), 24.0)];
    let mut scan = ReportScanState::default();
    run(
        &layout,
        &mut state,
        &mut active_elements[..],
        &mut coefficients[..],
        &space_air,
        InsideSurfaceTemperatures {
            owner: InsideSurfaceTemperatureOwner::SurfTempInTmp,
            rows: &rows,
        },
        readiness(),
        0.1,
        &ReportInputs::default(),
        &mut scan,
    )?;
    assert!(!state.get_user_input);
    assert!(!scan.get_report_input);
    assert!(state.node_check && state.active_surface_check);
    assert!(!state.my_environment);
    assert!(state.first_roof_surface);
    assert!(
        state
            .user_arrays
            .iter()
            .all(|a| matches!(a, Some(v) if v.is_empty()))
    );
    assert_eq!(
        state.surfaces[0].interior_geometry,
        [3.0, 6.0, 120.0 / 22.0, 0.0]
    );
    assert_eq!(state.surfaces[0].reference_air, [11, 12]);
    assert_eq!(state.surfaces[1].reference_air, [21, 22]);
    assert_eq!(
        coefficients[0].1,
        detailed_inside_coefficient_w_per_m2_k(24.0, 18.0, 0.0, 0.1)
    );
    assert_eq!(
        coefficients[1].1,
        detailed_inside_coefficient_w_per_m2_k(24.0, 26.0, 0.0, 0.1)
    );
    assert_ne!(coefficients[0].1, coefficients[1].1);
    let first_coefficient = coefficients[0].1;
    let second_coefficient = coefficients[1].1;
    let first_map = space_air.reference_air_by_layout(&layout)?;
    assert_eq!(first_map[&SurfaceId(8)], 18.0);
    assert_eq!(first_map[&SurfaceId(2)], 26.0);
    let mut declared = space_air.rows().to_vec();
    declared[0].mat = 20.0;
    space_air.write_all(&layout, &declared)?;
    run(
        &layout,
        &mut state,
        &mut active_elements[..],
        &mut coefficients[..],
        &space_air,
        InsideSurfaceTemperatures {
            owner: InsideSurfaceTemperatureOwner::SurfTempIn,
            rows: &rows,
        },
        readiness(),
        0.1,
        &ReportInputs::default(),
        &mut scan,
    )?;
    assert_ne!(coefficients[0].1, first_coefficient);
    assert_eq!(coefficients[1].1, second_coefficient);
    let next_map = space_air.reference_air_by_layout(&layout)?;
    assert_eq!(next_map[&SurfaceId(8)], 20.0);
    assert_eq!(next_map[&SurfaceId(2)], 26.0);
    assert_eq!(space_air.rows(), declared);
    Ok(())
}

#[test]
fn readiness_changes_only_reached_flags_and_retains_first_setup()
-> Result<(), Box<dyn std::error::Error>> {
    let layout = layout()?;
    let (mut state, mut coefficients) = state(&layout)?;
    let mut active_elements = [(ZoneId(7), [true; 3])];
    let space_air = SpaceAirTemperatures::allocate(&layout);
    let rows = [(SurfaceId(8), 23.0), (SurfaceId(2), 23.0)];
    let mut scan = ReportScanState::default();
    let temperatures = InsideSurfaceTemperatures {
        owner: InsideSurfaceTemperatureOwner::SurfTempIn,
        rows: &rows,
    };
    let mut ready = readiness();
    ready.nodes_allocated = true;
    ready.equipment_inputs_filled = true;
    ready.equipment_simulated_once = true;
    ready.system_sizing = true;
    run(
        &layout,
        &mut state,
        &mut active_elements[..],
        &mut coefficients[..],
        &space_air,
        temperatures,
        ready,
        0.7,
        &ReportInputs::default(),
        &mut scan,
    )?;
    assert!(state.node_check && state.active_surface_check);
    assert_eq!(active_elements[0].1, [true; 3]);
    // The same independent Config owner can change between actual calls.
    active_elements[0].1 = [false, true, false];
    state.surfaces[0].interior_geometry[0] = 41.0;
    ready.system_sizing = false;
    ready.begin_environment = false;
    run(
        &layout,
        &mut state,
        &mut active_elements[..],
        &mut coefficients[..],
        &space_air,
        temperatures,
        ready,
        0.7,
        &ReportInputs::default(),
        &mut scan,
    )?;
    assert!(!state.node_check && !state.active_surface_check);
    assert_eq!(active_elements[0].1, [false; 3]);
    // Once the actual setup flag clears, a later caller write is retained.
    active_elements[0].1 = [true, false, true];
    run(
        &layout,
        &mut state,
        &mut active_elements[..],
        &mut coefficients[..],
        &space_air,
        temperatures,
        ready,
        0.7,
        &ReportInputs::default(),
        &mut scan,
    )?;
    assert_eq!(active_elements[0].1, [true, false, true]);
    assert!(state.my_environment);
    assert_eq!(state.surfaces[0].interior_geometry[0], 41.0);
    assert!(
        coefficients
            .iter()
            .all(|r| r.1.to_bits() == 0.7_f64.to_bits())
    );
    Ok(())
}

#[test]
fn full_temperature_membership_and_stale_layout_stop_before_writes()
-> Result<(), Box<dyn std::error::Error>> {
    let layout = layout()?;
    let (mut state, mut coefficients) = state(&layout)?;
    let mut active_elements = [(ZoneId(7), [true; 3])];
    let space_air = SpaceAirTemperatures::allocate(&layout);
    let before = state.clone();
    let coefficient_before = coefficients;
    let rows = [(SurfaceId(2), 23.0), (SurfaceId(8), 23.0)];
    let mut scan = ReportScanState::default();
    let result = run(
        &layout,
        &mut state,
        &mut active_elements[..],
        &mut coefficients[..],
        &space_air,
        InsideSurfaceTemperatures {
            owner: InsideSurfaceTemperatureOwner::SurfTempIn,
            rows: &rows,
        },
        readiness(),
        0.1,
        &ReportInputs::default(),
        &mut scan,
    );
    assert!(matches!(
        result,
        Err(InsideConvectionError::Unavailable(
            InsideConvectionUnavailable::TemperatureOwner
        ))
    ));
    assert_eq!(state, before);
    assert_eq!(coefficients, coefficient_before);
    assert!(scan.get_report_input);
    let valid_rows = [(SurfaceId(8), 23.0), (SurfaceId(2), 23.0)];
    for mut invalid in [
        Vec::new(),
        vec![(ZoneId(8), [false; 3])],
        vec![(ZoneId(7), [false; 3]), (ZoneId(8), [true; 3])],
    ] {
        let unchanged = invalid.clone();
        let result = run(
            &layout,
            &mut state,
            &mut invalid[..],
            &mut coefficients[..],
            &space_air,
            InsideSurfaceTemperatures {
                owner: InsideSurfaceTemperatureOwner::SurfTempIn,
                rows: &valid_rows,
            },
            readiness(),
            0.1,
            &ReportInputs::default(),
            &mut scan,
        );
        assert!(matches!(
            result,
            Err(InsideConvectionError::Unavailable(
                InsideConvectionUnavailable::Association
            ))
        ));
        assert_eq!(state, before);
        assert_eq!(coefficients, coefficient_before);
        assert_eq!(invalid, unchanged);
        assert!(scan.get_report_input);
    }
    for mut invalid in [
        Vec::new(),
        vec![(SurfaceId(8), -7.0), (SurfaceId(3), -8.0)],
        vec![(SurfaceId(2), -8.0), (SurfaceId(8), -7.0)],
        vec![
            (SurfaceId(8), -7.0),
            (SurfaceId(2), -8.0),
            (SurfaceId(9), -9.0),
        ],
    ] {
        let unchanged = invalid.clone();
        let result = run(
            &layout,
            &mut state,
            &mut active_elements[..],
            &mut invalid[..],
            &space_air,
            InsideSurfaceTemperatures {
                owner: InsideSurfaceTemperatureOwner::SurfTempIn,
                rows: &valid_rows,
            },
            readiness(),
            0.1,
            &ReportInputs::default(),
            &mut scan,
        );
        assert!(matches!(
            result,
            Err(InsideConvectionError::Unavailable(
                InsideConvectionUnavailable::Association
            ))
        ));
        assert_eq!(state, before);
        assert_eq!(invalid, unchanged);
        assert_eq!(active_elements[0].1, [true; 3]);
        assert!(scan.get_report_input);
    }
    let duplicate = InsideConvectionLayout::new(
        vec![ZoneId(7)],
        vec![(SpaceId(3), ZoneId(7))],
        Vec::new(),
        vec![
            InsideConvectionGroup {
                zone_id: ZoneId(7),
                space_id: SpaceId(3),
                surfaces: Vec::new(),
            },
            InsideConvectionGroup {
                zone_id: ZoneId(7),
                space_id: SpaceId(3),
                surfaces: Vec::new(),
            },
        ],
    );
    assert!(duplicate.is_err());
    assert!(NoUserConvectionInput::new([0, 0, 0, 1, 0, 0]).is_err());
    Ok(())
}

#[test]
fn selected_space_reset_and_copy_preserve_independent_ztav()
-> Result<(), Box<dyn std::error::Error>> {
    let layout = layout()?;
    let mut air = SpaceAirTemperatures::allocate(&layout);
    let mut rows = air.rows().to_vec();
    rows[0].ztav = 55.0;
    rows[1].ztav = 66.0;
    air.write_all(&layout, &rows)?;
    air.copy_zone_after_correction(&layout, ZoneId(7), 31.0, 32.0)?;
    assert_eq!(air.rows()[0].ztav, 55.0);
    assert_eq!(air.rows()[1].ztav, 66.0);
    assert!(air.rows().iter().all(|r| r.mat == 32.0 && r.zt == 31.0));
    air.reset_begin_environment(&layout)?;
    assert!(
        air.rows()
            .iter()
            .all(|r| r.mat == 23.0 && r.zt == 23.0 && r.ztav == 23.0)
    );
    Ok(())
}

#[test]
fn reached_details_unavailable_retains_static_and_scan_prefix()
-> Result<(), Box<dyn std::error::Error>> {
    let layout = layout()?;
    let (mut state, mut coefficients) = state(&layout)?;
    let mut active_elements = [(ZoneId(7), [true; 3])];
    let space_air = SpaceAirTemperatures::allocate(&layout);
    let mut scan = ReportScanState {
        get_report_input: false,
        surface_details: true,
        ..ReportScanState::default()
    };
    let rows = [(SurfaceId(8), 23.0), (SurfaceId(2), 23.0)];
    let result = run(
        &layout,
        &mut state,
        &mut active_elements[..],
        &mut coefficients[..],
        &space_air,
        InsideSurfaceTemperatures {
            owner: InsideSurfaceTemperatureOwner::SurfTempInTmp,
            rows: &rows,
        },
        readiness(),
        0.1,
        &ReportInputs::default(),
        &mut scan,
    );
    assert!(matches!(
        result,
        Err(InsideConvectionError::Unavailable(
            InsideConvectionUnavailable::SurfaceDetailsReport
        ))
    ));
    assert!(
        state.get_user_input
            && state.node_check
            && state.active_surface_check
            && state.my_environment
    );
    assert_eq!(coefficients[0].1, -7.0);
    assert_eq!(state.surfaces[0].interior_geometry[0], 3.0);
    assert!(!scan.get_report_input && scan.surface_details);
    Ok(())
}

#[test]
fn real_warning_sink_failure_preserves_static_prefix_and_error_cause()
-> Result<(), Box<dyn std::error::Error>> {
    use std::error::Error;
    struct FailedWriter;
    impl std::io::Write for FailedWriter {
        fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("declared diagnostic sink failure"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let layout = layout()?;
    let (mut state, mut coefficients) = state(&layout)?;
    let mut active_elements = [(ZoneId(7), [true; 3])];
    let space_air = SpaceAirTemperatures::allocate(&layout);
    let mut scan = ReportScanState::default();
    let mut reports = ReportInputs::default();
    reports.objects.push(ep_model::ReportInputObject {
        object_name: "actual invalid test request".into(),
        object_type: ep_model::ReportInputObjectType::SurfacesList,
        alphas: ["INVALID".into(), String::new(), String::new()],
        alpha_blank: [false, true, true],
        default_applied: [false; 3],
        num_alphas: 1,
        is_epjson: false,
    });
    let mut tracking = error_state();
    let mut writer = FailedWriter;
    let mut console = Vec::new();
    let mut backend = SourceErrorBackend {
        state: &mut tracking,
        error_stream: Some(&mut writer),
        console: &mut console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    let rows = [(SurfaceId(8), 23.0), (SurfaceId(2), 23.0)];
    let result = initialize_inside_convection(
        &layout,
        &zones(),
        &surfaces(),
        NoUserConvectionInput::new([0; 6])?,
        &mut state,
        &mut active_elements[..],
        &mut coefficients[..],
        &space_air,
        InsideSurfaceTemperatures {
            owner: InsideSurfaceTemperatureOwner::SurfTempInTmp,
            rows: &rows,
        },
        readiness(),
        0.1,
        &reports,
        &mut scan,
        &mut backend,
    );
    let error = match result {
        Err(e) => e,
        Ok(()) => return Err(std::io::Error::other("missing reached backend stop").into()),
    };
    assert!(matches!(&error, InsideConvectionError::Backend(_)));
    assert!(error.source().and_then(|source| source.source()).is_some());
    assert_eq!(tracking.tracking.total_warning_errors, 1);
    assert!(
        scan.get_report_input
            && state.get_user_input
            && state.node_check
            && state.active_surface_check
    );
    assert_eq!(coefficients[0].1, -7.0);
    assert_eq!(state.surfaces[0].interior_geometry[0], 3.0);
    Ok(())
}

#[test]
fn roof_shared_edge_cancellation_and_compass_boundaries_are_literal()
-> Result<(), Box<dyn std::error::Error>> {
    let mut rows = surfaces();
    rows[0].tilt = 0.0;
    rows[0].area = 1.0;
    rows[0].azimuth = 10.0;
    rows[0].vertices = vec![
        [0.0, 0.0, 3.0],
        [1.0, 0.0, 3.0],
        [1.0, 1.0, 3.0],
        [0.0, 1.0, 3.0],
    ];
    rows[1].tilt = 0.0;
    rows[1].area = 1.0;
    rows[1].azimuth = 30.0;
    rows[1].vertices = vec![
        [1.0, 0.0, 3.0],
        [2.0, 0.0, 3.0],
        [2.0, 1.0, 3.0],
        [1.0, 1.0, 3.0],
    ];
    let roof = geometry::roof(&rows);
    assert_eq!(
        (roof.area, roof.perimeter, roof.height, roof.azimuth),
        (2.0, 6.0, 0.0, 20.0)
    );
    assert_eq!(geometry::compass(22.5)?, 1);
    assert_eq!(geometry::compass(292.5)?, 7);
    assert_eq!(geometry::compass(337.5)?, 0);
    Ok(())
}

#[test]
fn distinct_roof_edge_with_underflowed_length_stops_before_setup()
-> Result<(), Box<dyn std::error::Error>> {
    let layout = layout()?;
    let (mut state, mut coefficients) = state(&layout)?;
    let mut active_elements = [(ZoneId(7), [true; 3])];
    let space_air = SpaceAirTemperatures::allocate(&layout);
    let before = state.clone();
    let coefficient_before = coefficients;
    let mut surfaces = surfaces();
    surfaces[0].tilt = 0.0;
    surfaces[0].vertices = vec![
        [0.0, 0.0, 3.0],
        [1.0e-200, 0.0, 3.0],
        [1.0, 1.0, 3.0],
        [0.0, 1.0, 3.0],
    ];
    assert_ne!(surfaces[0].vertices[0], surfaces[0].vertices[1]);
    let rows = [(SurfaceId(8), 23.0), (SurfaceId(2), 23.0)];
    let mut scan = ReportScanState::default();
    let mut error = error_state();
    let mut stream = Vec::new();
    let mut console = Vec::new();
    let mut backend = SourceErrorBackend {
        state: &mut error,
        error_stream: Some(&mut stream),
        console: &mut console,
        out_unit1: None,
        out_unit2: None,
        f_message: None,
        message_callback: None,
        error_callback: None,
        sqlite: None,
    };
    let result = initialize_inside_convection(
        &layout,
        &zones(),
        &surfaces,
        NoUserConvectionInput::new([0; 6])?,
        &mut state,
        &mut active_elements[..],
        &mut coefficients[..],
        &space_air,
        InsideSurfaceTemperatures {
            owner: InsideSurfaceTemperatureOwner::SurfTempInTmp,
            rows: &rows,
        },
        readiness(),
        0.1,
        &ReportInputs::default(),
        &mut scan,
        &mut backend,
    );
    assert!(matches!(
        result,
        Err(InsideConvectionError::Unavailable(
            InsideConvectionUnavailable::Geometry
        ))
    ));
    assert_eq!(state, before);
    assert_eq!(coefficients, coefficient_before);
    assert!(scan.get_report_input);
    assert!(stream.is_empty() && console.is_empty());
    Ok(())
}
