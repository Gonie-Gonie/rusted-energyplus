mod material_details;
mod window_material_blind;
mod window_material_drape_equivalent_layer;
mod window_material_screen;
mod window_material_screen_equivalent_layer;
mod window_material_shade;
mod window_material_shade_equivalent_layer;

use crate::{
    EioError, EioSurfaceVertex, OrderedTimestampDivergenceReason, SeriesAlignment,
    SeriesComparisonStatus, SeriesDivergenceKind, SeriesSample, Tolerance,
    compare_ordered_timestamp_samples_v2, compare_series, compare_series_samples_v2,
    compare_series_v2, parse_eio_construction_ctf, parse_eio_construction_ctf_coefficients,
    parse_eio_construction_material_summaries, parse_eio_heat_transfer_surfaces,
    parse_eio_material_ctf_summary, parse_eio_other_equipment_nominal,
    parse_eio_surface_geometry_rules, parse_eio_warmup_environments,
    parse_eio_window_material_gap_equivalent_layer, parse_eio_window_material_gas,
    parse_eio_window_material_glazing, parse_eio_window_material_glazing_equivalent_layer,
    parse_eio_zone_geometry, parse_eso_series, parse_eso_time_series, parse_mtr_time_series,
    parse_mtr_time_series_for_frequency,
};

#[test]
fn tolerance_accepts_close_values() {
    let tolerance = Tolerance::default();

    assert!(tolerance.accepts(1.0, 1.0 + 1.0e-10));
    assert!(!tolerance.accepts(1.0, 1.1));
}

#[test]
fn parses_eso_series_by_key_and_variable() -> Result<(), Box<dyn std::error::Error>> {
    let values = parse_eso_series(
        r#"Program Version,EnergyPlus
1,5,Environment Title[],Latitude[deg],Longitude[deg],Time Zone[],Elevation[m]
494,1,ALWAYSON,Schedule Value [] !Hourly
End of Data Dictionary
1,Run Period,39.74,-105.18,-7.00,1829.00
494,1.0
494,1.0
"#,
        "AlwaysOn",
        "Schedule Value",
    )?;

    assert_eq!(values, vec![1.0, 1.0]);

    Ok(())
}

#[test]
fn parses_eso_time_series_with_hourly_timestamps() -> Result<(), Box<dyn std::error::Error>> {
    let series = parse_eso_time_series(
        r#"Program Version,EnergyPlus
1,5,Environment Title[],Latitude[deg],Longitude[deg],Time Zone[],Elevation[m]
2,8,Day of Simulation[],Month[],Day of Month[],DST Indicator[1=yes 0=no],Hour[],StartMinute[],EndMinute[],DayType
7,1,ALWAYSON,Schedule Value [] !Hourly
End of Data Dictionary
1,RUN PERIOD 1,39.74,-105.18,-7.00,1829.00
2,1,1,1,0,1,0.00,60.00,Tuesday
7,1.0
2,1,1,1,0,2,0.00,60.00,Tuesday
7,2.0
"#,
        "AlwaysOn",
        "Schedule Value",
    )?;

    assert_eq!(series.metadata.id, "7");
    assert_eq!(series.metadata.key, "ALWAYSON");
    assert_eq!(series.metadata.variable, "Schedule Value");
    assert_eq!(series.metadata.units, None);
    assert_eq!(series.metadata.frequency.as_deref(), Some("Hourly"));
    assert_eq!(
        series
            .samples
            .iter()
            .map(|sample| sample.value)
            .collect::<Vec<_>>(),
        vec![1.0, 2.0]
    );
    assert!(
        series.samples[0]
            .timestamp
            .as_deref()
            .unwrap_or_default()
            .contains("hour=1")
    );
    assert!(
        series.samples[1]
            .timestamp
            .as_deref()
            .unwrap_or_default()
            .contains("hour=2")
    );

    Ok(())
}

#[test]
fn parses_mtr_time_series_with_hourly_timestamps() -> Result<(), Box<dyn std::error::Error>> {
    let series = parse_mtr_time_series(
        r#"Program Version,EnergyPlus
1,5,Environment Title[],Latitude[deg],Longitude[deg],Time Zone[],Elevation[m]
2,8,Day of Simulation[],Month[],Day of Month[],DST Indicator[1=yes 0=no],Hour[],StartMinute[],EndMinute[],DayType
150,1,DistrictHeatingWater:Facility [J] !Hourly
End of Data Dictionary
1,RUN PERIOD 1,39.74,-105.18,-7.00,1829.00
2,1,1,1,0,1,0.00,60.00,Tuesday
150,3.5
2,1,1,1,0,2,0.00,60.00,Tuesday
150,4.5
"#,
        "DistrictHeatingWater:Facility",
    )?;

    assert_eq!(series.metadata.id, "150");
    assert_eq!(series.metadata.meter, "DistrictHeatingWater:Facility");
    assert_eq!(series.metadata.units.as_deref(), Some("J"));
    assert_eq!(series.metadata.frequency.as_deref(), Some("Hourly"));
    assert_eq!(
        series
            .samples
            .iter()
            .map(|sample| sample.value)
            .collect::<Vec<_>>(),
        vec![3.5, 4.5]
    );
    assert_eq!(
        series.samples[0].timestamp.as_deref(),
        Some(
            "env=RUN PERIOD 1;day=1;month=1;date=1;dst=0;hour=1;start=0.00;end=60.00;day_type=Tuesday"
        )
    );

    Ok(())
}

#[test]
fn reports_missing_mtr_meter() {
    let error = parse_mtr_time_series(
        r#"Program Version,EnergyPlus
150,1,DistrictHeatingWater:Facility [J] !Hourly
End of Data Dictionary
"#,
        "DistrictCooling:Facility",
    )
    .expect_err("expected missing meter");

    assert_eq!(
        error.to_string(),
        "MTR meter not found: DistrictCooling:Facility"
    );
}

#[test]
fn parses_mtr_time_series_by_frequency() -> Result<(), Box<dyn std::error::Error>> {
    let contents = r#"Program Version,EnergyPlus
1,5,Environment Title[],Latitude[deg],Longitude[deg],Time Zone[],Elevation[m]
2,8,Day of Simulation[],Month[],Day of Month[],DST Indicator[1=yes 0=no],Hour[],StartMinute[],EndMinute[],DayType
4,2,Cumulative Days of Simulation[],Month[]  ! When Monthly Meters Requested
5,1,Cumulative Days of Simulation[] ! When Run Period Meters Requested
150,1,DistrictHeatingWater:Facility [J] !Hourly
151,9,DistrictHeatingWater:Facility [J] !Monthly [Value,Min,Day,Hour,Minute,Max,Day,Hour,Minute]
152,11,DistrictHeatingWater:Facility [J] !RunPeriod [Value,Min,Month,Day,Hour,Minute,Max,Month,Day,Hour,Minute]
End of Data Dictionary
1,RUN PERIOD 1,39.74,-105.18,-7.00,1829.00
2,1,1,1,0,1,0.00,60.00,Tuesday
150,3.5
4,1,1
151,10.0,0.0,1,1,15,10.0,1,1,60
5,1
152,10.0,0.0,1,1,1,15,10.0,1,1,60
"#;

    let monthly =
        parse_mtr_time_series_for_frequency(contents, "DistrictHeatingWater:Facility", "Monthly")?;
    let run_period = parse_mtr_time_series_for_frequency(
        contents,
        "DistrictHeatingWater:Facility",
        "RunPeriod",
    )?;

    assert_eq!(monthly.metadata.id, "151");
    assert_eq!(monthly.metadata.frequency.as_deref(), Some("Monthly"));
    assert_eq!(monthly.samples.len(), 1);
    assert_eq!(monthly.samples[0].value, 10.0);
    assert_eq!(run_period.metadata.id, "152");
    assert_eq!(run_period.metadata.frequency.as_deref(), Some("RunPeriod"));
    assert_eq!(run_period.samples.len(), 1);
    assert_eq!(run_period.samples[0].value, 10.0);

    Ok(())
}

#[test]
fn reports_missing_mtr_frequency() {
    let error = parse_mtr_time_series_for_frequency(
        r#"Program Version,EnergyPlus
150,1,DistrictHeatingWater:Facility [J] !Hourly
End of Data Dictionary
"#,
        "DistrictHeatingWater:Facility",
        "Monthly",
    )
    .expect_err("expected missing frequency");

    assert_eq!(
        error.to_string(),
        "MTR meter not found: DistrictHeatingWater:Facility (Monthly)"
    );
}

#[test]
fn parses_eio_zone_geometry_rows() -> Result<(), Box<dyn std::error::Error>> {
    let zones = parse_eio_zone_geometry(
        r#"! <Zone Information>,Zone Name,...
 Zone Information, ZONE ONE,0.0,0.00,0.00,0.00,7.62,7.62,2.29,1,1,1,0.00,15.24,0.00,15.24,0.00,4.57,4.57,1061.88,TARP,DOE-2,232.26,278.71,278.71,0.00,6,0,0,Yes
"#,
    )?;

    assert_eq!(zones.len(), 1);
    assert_eq!(zones[0].zone_name, "ZONE ONE");
    assert_eq!(zones[0].surface_count, 6);
    assert_eq!(zones[0].floor_area_m2, 232.26);
    assert_eq!(zones[0].volume_m3, 1061.88);
    assert_eq!(zones[0].exterior_gross_wall_area_m2, 278.71);

    Ok(())
}

#[test]
fn series_v2_reports_rmse_and_relative_delta() -> Result<(), Box<dyn std::error::Error>> {
    let result = compare_series_v2(&[10.0, 20.0], &[10.0, 22.0], Tolerance::default());

    assert_eq!(result.alignment, SeriesAlignment::Index);
    assert_eq!(result.expected_samples, 2);
    assert_eq!(result.observed_samples, 2);
    assert_eq!(result.compared_samples, 2);
    assert!(!result.passed());
    assert_eq!(result.max_abs_delta, 2.0);
    assert!((result.rmse_delta - 2.0_f64.sqrt()).abs() < 1.0e-12);
    assert!((result.max_rel_delta - (2.0 / 22.0)).abs() < 1.0e-12);

    let divergence = result
        .first_divergence
        .ok_or_else(|| std::io::Error::other("expected first divergence"))?;
    assert_eq!(divergence.kind, SeriesDivergenceKind::Tolerance);
    assert_eq!(divergence.index, 1);
    assert_eq!(divergence.expected, Some(20.0));
    assert_eq!(divergence.observed, Some(22.0));
    assert_eq!(divergence.abs_delta, Some(2.0));

    Ok(())
}

#[test]
fn series_v2_aligns_timestamped_samples() -> Result<(), Box<dyn std::error::Error>> {
    let expected = vec![
        SeriesSample::timestamped(0, "t2", 2.0),
        SeriesSample::timestamped(1, "t1", 1.0),
    ];
    let observed = vec![
        SeriesSample::timestamped(0, "t1", 1.0),
        SeriesSample::timestamped(1, "t2", 2.5),
    ];

    let result = compare_series_samples_v2(&expected, &observed, Tolerance::default());

    assert_eq!(result.alignment, SeriesAlignment::Timestamp);
    assert_eq!(result.compared_samples, 2);
    assert!(!result.passed());
    let divergence = result
        .first_divergence
        .ok_or_else(|| std::io::Error::other("expected first divergence"))?;
    assert_eq!(divergence.kind, SeriesDivergenceKind::Tolerance);
    assert_eq!(divergence.timestamp.as_deref(), Some("t2"));
    assert_eq!(divergence.expected, Some(2.0));
    assert_eq!(divergence.observed, Some(2.5));

    Ok(())
}

#[test]
fn series_v2_reports_missing_observed_timestamp() -> Result<(), Box<dyn std::error::Error>> {
    let expected = vec![
        SeriesSample::timestamped(0, "t1", 1.0),
        SeriesSample::timestamped(1, "t2", 2.0),
    ];
    let observed = vec![SeriesSample::timestamped(0, "t1", 1.0)];

    let result = compare_series_samples_v2(&expected, &observed, Tolerance::default());

    assert_eq!(result.alignment, SeriesAlignment::Timestamp);
    assert_eq!(result.compared_samples, 1);
    assert!(!result.passed());
    let divergence = result
        .first_divergence
        .ok_or_else(|| std::io::Error::other("expected first divergence"))?;
    assert_eq!(divergence.kind, SeriesDivergenceKind::MissingObservedSample);
    assert_eq!(divergence.timestamp.as_deref(), Some("t2"));
    assert_eq!(divergence.expected, Some(2.0));
    assert_eq!(divergence.observed, None);

    Ok(())
}

#[test]
fn ordered_timestamp_contract_passes_exact_unique_series() {
    let expected = vec![
        SeriesSample::timestamped(0, "t1", 1.0),
        SeriesSample::timestamped(1, "t2", 2.0),
    ];
    let observed = vec![
        SeriesSample::timestamped(0, "t1", 1.0),
        SeriesSample::timestamped(1, "t2", 2.0),
    ];

    let result = compare_ordered_timestamp_samples_v2(&expected, &observed, Tolerance::default());

    assert!(result.passed());
    assert!(result.timestamp_contract_passed());
    assert_eq!(result.comparison.alignment, SeriesAlignment::Timestamp);
    assert_eq!(result.comparison.compared_samples, 2);
    assert_eq!(result.comparison.rmse_delta, 0.0);
    assert!(result.expected_unique_timestamps);
    assert!(result.observed_unique_timestamps);
    assert!(result.timestamp_order_match);
    assert_eq!(result.first_timestamp_divergence, None);
}

#[test]
fn ordered_timestamp_contract_rejects_reordered_series() -> Result<(), Box<dyn std::error::Error>> {
    let expected = vec![
        SeriesSample::timestamped(0, "t1", 1.0),
        SeriesSample::timestamped(1, "t2", 2.0),
    ];
    let observed = vec![
        SeriesSample::timestamped(0, "t2", 2.0),
        SeriesSample::timestamped(1, "t1", 1.0),
    ];

    let result = compare_ordered_timestamp_samples_v2(&expected, &observed, Tolerance::default());

    assert!(!result.passed());
    assert_eq!(result.contract_status, SeriesComparisonStatus::Fail);
    assert!(!result.timestamp_order_match);
    let divergence = result
        .first_timestamp_divergence
        .ok_or_else(|| std::io::Error::other("expected timestamp divergence"))?;
    assert_eq!(divergence.index, 0);
    assert_eq!(divergence.expected.as_deref(), Some("t1"));
    assert_eq!(divergence.observed.as_deref(), Some("t2"));
    assert_eq!(
        divergence.reason,
        OrderedTimestampDivergenceReason::TimestampMismatch
    );
    assert_eq!(divergence.reason.as_str(), "timestamp_mismatch");

    Ok(())
}

#[test]
fn ordered_timestamp_contract_rejects_duplicate_before_length_divergence()
-> Result<(), Box<dyn std::error::Error>> {
    let expected = vec![
        SeriesSample::timestamped(0, "t1", 1.0),
        SeriesSample::timestamped(1, "t1", 1.0),
    ];
    let observed = vec![SeriesSample::timestamped(0, "t1", 1.0)];

    let result = compare_ordered_timestamp_samples_v2(&expected, &observed, Tolerance::default());

    assert!(!result.passed());
    assert!(!result.expected_unique_timestamps);
    assert!(result.observed_unique_timestamps);
    let divergence = result
        .first_timestamp_divergence
        .ok_or_else(|| std::io::Error::other("expected timestamp divergence"))?;
    assert_eq!(divergence.index, 1);
    assert_eq!(divergence.expected.as_deref(), Some("t1"));
    assert_eq!(divergence.observed, None);
    assert_eq!(
        divergence.reason,
        OrderedTimestampDivergenceReason::DuplicateExpectedTimestamp
    );

    Ok(())
}

#[test]
fn ordered_timestamp_contract_rejects_missing_timestamp() -> Result<(), Box<dyn std::error::Error>>
{
    let expected = vec![SeriesSample::indexed(0, 1.0)];
    let observed = vec![SeriesSample::timestamped(0, "t1", 1.0)];

    let result = compare_ordered_timestamp_samples_v2(&expected, &observed, Tolerance::default());

    assert!(!result.passed());
    assert!(!result.timestamp_order_match);
    let divergence = result
        .first_timestamp_divergence
        .ok_or_else(|| std::io::Error::other("expected timestamp divergence"))?;
    assert_eq!(divergence.index, 0);
    assert_eq!(divergence.expected, None);
    assert_eq!(divergence.observed.as_deref(), Some("t1"));
    assert_eq!(
        divergence.reason,
        OrderedTimestampDivergenceReason::MissingExpectedTimestamp
    );

    Ok(())
}

#[test]
fn ordered_timestamp_contract_reports_value_tolerance_and_rmse()
-> Result<(), Box<dyn std::error::Error>> {
    let expected = vec![
        SeriesSample::timestamped(0, "t1", 1.0),
        SeriesSample::timestamped(1, "t2", 2.0),
    ];
    let observed = vec![
        SeriesSample::timestamped(0, "t1", 1.0),
        SeriesSample::timestamped(1, "t2", 2.5),
    ];

    let result = compare_ordered_timestamp_samples_v2(&expected, &observed, Tolerance::default());

    assert!(!result.passed());
    assert!(result.timestamp_contract_passed());
    assert_eq!(result.first_timestamp_divergence, None);
    assert_eq!(result.comparison.max_abs_delta, 0.5);
    assert!((result.comparison.rmse_delta - 0.5 / 2.0_f64.sqrt()).abs() < 1.0e-12);
    let divergence = result
        .comparison
        .first_divergence
        .ok_or_else(|| std::io::Error::other("expected value divergence"))?;
    assert_eq!(divergence.index, 1);
    assert_eq!(divergence.timestamp.as_deref(), Some("t2"));
    assert_eq!(divergence.kind, SeriesDivergenceKind::Tolerance);

    Ok(())
}

#[test]
fn legacy_timestamp_alignment_remains_order_insensitive() {
    let expected = vec![
        SeriesSample::timestamped(0, "t1", 1.0),
        SeriesSample::timestamped(1, "t2", 2.0),
    ];
    let observed = vec![
        SeriesSample::timestamped(0, "t2", 2.0),
        SeriesSample::timestamped(1, "t1", 1.0),
    ];

    let result = compare_series_samples_v2(&expected, &observed, Tolerance::default());

    assert!(result.passed());
    assert_eq!(result.alignment, SeriesAlignment::Timestamp);
}

#[test]
fn parses_eio_heat_transfer_surface_rows() -> Result<(), Box<dyn std::error::Error>> {
    let surfaces = parse_eio_heat_transfer_surfaces(
        r#"! <HeatTransfer Surface>,Surface Name,...
 HeatTransfer Surface,WALL X0,Wall,,CTF - ConductionTransferFunction,WALL CONSTRUCTION,1.000,0.870,,1.00,1.00,1.00,90.00,90.00,1.00,1.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,NoSun,NoWind,0.50,0.50,0.50,0.50,4
"#,
    )?;

    assert_eq!(surfaces.len(), 1);
    assert_eq!(surfaces[0].surface_name, "WALL X0");
    assert_eq!(surfaces[0].surface_class, "Wall");
    assert_eq!(surfaces[0].construction_name, "WALL CONSTRUCTION");
    assert_eq!(surfaces[0].area_net_m2, 1.0);
    assert_eq!(surfaces[0].area_gross_m2, 1.0);
    assert_eq!(surfaces[0].azimuth_deg, 90.0);
    assert_eq!(surfaces[0].tilt_deg, 90.0);
    assert_eq!(surfaces[0].side_count, 4);
    assert_eq!(surfaces[0].world_vertices, None);

    Ok(())
}

#[test]
fn parses_eio_heat_transfer_surface_details_with_vertices() -> Result<(), Box<dyn std::error::Error>>
{
    let surfaces = parse_eio_heat_transfer_surfaces(
        r#"! <HeatTransfer Surface>,Surface Name,...,#Sides,Vertex 1 X {m},Vertex 1 Y {m},Vertex 1 Z {m},{etc}
 HeatTransfer Surface,TRANSFORMED WALL,Wall,,CTF - ConductionTransferFunction,WALL CONSTRUCTION,1.000,0.870,,6.00,6.00,6.00,123.45,90.00,2.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4,-13.25,-18.50,34.75,-10.00,-18.50,34.75,-10.00,-18.50,37.25,-13.25,-18.50,37.25
"#,
    )?;

    assert_eq!(surfaces.len(), 1);
    assert_eq!(surfaces[0].surface_name, "TRANSFORMED WALL");
    assert_eq!(surfaces[0].side_count, 4);
    assert_eq!(
        surfaces[0].world_vertices,
        Some(vec![
            EioSurfaceVertex {
                x_m: -13.25,
                y_m: -18.5,
                z_m: 34.75,
            },
            EioSurfaceVertex {
                x_m: -10.0,
                y_m: -18.5,
                z_m: 34.75,
            },
            EioSurfaceVertex {
                x_m: -10.0,
                y_m: -18.5,
                z_m: 37.25,
            },
            EioSurfaceVertex {
                x_m: -13.25,
                y_m: -18.5,
                z_m: 37.25,
            },
        ])
    );

    Ok(())
}

#[test]
fn rejects_eio_heat_transfer_surface_vertex_count_mismatch() {
    let result = parse_eio_heat_transfer_surfaces(
        "HeatTransfer Surface,TRANSFORMED WALL,Wall,,CTF - ConductionTransferFunction,WALL CONSTRUCTION,1.000,0.870,,6.00,6.00,6.00,123.45,90.00,2.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,4,-13.25,-18.50,34.75,-10.00,-18.50,34.75,-10.00,-18.50,37.25\n",
    );

    assert!(matches!(
        result,
        Err(EioError::InvalidHeatTransferSurface { line: 1, .. })
    ));
}

#[test]
fn rejects_eio_heat_transfer_surface_zero_or_overflowing_side_count() {
    let zero_sides = parse_eio_heat_transfer_surfaces(
        "HeatTransfer Surface,ZERO SIDES,Wall,,CTF - ConductionTransferFunction,WALL CONSTRUCTION,1.000,0.870,,6.00,6.00,6.00,123.45,90.00,2.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,0\n",
    );
    assert!(matches!(
        zero_sides,
        Err(EioError::InvalidHeatTransferSurface { line: 1, .. })
    ));

    let overflowing_sides = parse_eio_heat_transfer_surfaces(&format!(
        "HeatTransfer Surface,OVERFLOWING SIDES,Wall,,CTF - ConductionTransferFunction,WALL CONSTRUCTION,1.000,0.870,,6.00,6.00,6.00,123.45,90.00,2.00,3.00,0.00,ExternalEnvironment,DOE-2,ASHRAETARP,SunExposed,WindExposed,0.50,0.50,0.50,0.50,{}\n",
        usize::MAX
    ));
    assert!(matches!(
        overflowing_sides,
        Err(EioError::InvalidHeatTransferSurface { line: 1, .. })
    ));
}

#[test]
fn parses_eio_surface_geometry_rules() -> Result<(), Box<dyn std::error::Error>> {
    let rules = parse_eio_surface_geometry_rules(
        r#"! <Surface Geometry>,Starting Corner,Vertex Input Direction,Coordinate System,Daylight Reference Point Coordinate System,Rectangular (Simple) Surface Coordinate System
 Surface Geometry,UpperLeftCorner,Counterclockwise,RelativeCoordinateSystem,WorldCoordinateSystem,RelativeToZoneOrigin
"#,
    )?;

    assert_eq!(rules.starting_corner, "UpperLeftCorner");
    assert_eq!(rules.vertex_input_direction, "Counterclockwise");
    assert_eq!(rules.coordinate_system, "RelativeCoordinateSystem");
    assert_eq!(
        rules.daylight_reference_point_coordinate_system,
        "WorldCoordinateSystem"
    );
    assert_eq!(
        rules.rectangular_surface_coordinate_system,
        "RelativeToZoneOrigin"
    );

    Ok(())
}

#[test]
fn rejects_missing_duplicate_and_invalid_eio_surface_geometry_rules() {
    assert!(matches!(
        parse_eio_surface_geometry_rules("Program Version,EnergyPlus\n"),
        Err(EioError::MissingSurfaceGeometry)
    ));

    let duplicate = "Surface Geometry,UpperLeftCorner,Counterclockwise,WorldCoordinateSystem,RelativeCoordinateSystem,RelativeToZoneOrigin\n".repeat(2);
    assert!(matches!(
        parse_eio_surface_geometry_rules(&duplicate),
        Err(EioError::InvalidSurfaceGeometry { line: 2, .. })
    ));

    assert!(matches!(
        parse_eio_surface_geometry_rules(
            "Surface Geometry,UpperLeftCorner,Counterclockwise,Bogus,RelativeCoordinateSystem,RelativeToZoneOrigin\n"
        ),
        Err(EioError::InvalidSurfaceGeometry { line: 1, .. })
    ));
    assert!(matches!(
        parse_eio_surface_geometry_rules("Surface Geometry,,,,,\n"),
        Err(EioError::InvalidSurfaceGeometry { line: 1, .. })
    ));
}

#[test]
fn parses_eio_other_equipment_nominal_rows() -> Result<(), Box<dyn std::error::Error>> {
    let equipment = parse_eio_other_equipment_nominal(
        r#"! <OtherEquipment Internal Gains Nominal>,Name,...
 OtherEquipment Internal Gains Nominal, TEST 352A,ALWAYSON,ZONE ONE,232.26,0.0,352.000,1.516,N/A,0.000,0.100,0.200,0.700,352.000,352.000
"#,
    )?;

    assert_eq!(equipment.len(), 1);
    assert_eq!(equipment[0].equipment_name, "TEST 352A");
    assert_eq!(equipment[0].schedule_name, "ALWAYSON");
    assert_eq!(equipment[0].zone_name, "ZONE ONE");
    assert_eq!(equipment[0].zone_floor_area_m2, 232.26);
    assert_eq!(equipment[0].equipment_level_w, 352.0);
    assert_eq!(equipment[0].equipment_per_floor_area_w_per_m2, 1.516);
    assert_eq!(equipment[0].fraction_latent, 0.0);
    assert_eq!(equipment[0].fraction_radiant, 0.1);
    assert_eq!(equipment[0].fraction_lost, 0.2);
    assert_eq!(equipment[0].fraction_convected, 0.7);

    Ok(())
}

#[test]
fn parses_eio_construction_ctf_rows() -> Result<(), Box<dyn std::error::Error>> {
    let constructions = parse_eio_construction_ctf(
        r#"! <Construction CTF>,Construction Name,...
 Construction CTF,R13WALL,   1,   1,   1,   0.250,         0.4365,   0.900,   0.900,   0.750,   0.750,Rough
"#,
    )?;

    assert_eq!(constructions.len(), 1);
    assert_eq!(constructions[0].construction_name, "R13WALL");
    assert_eq!(constructions[0].index, 1);
    assert_eq!(constructions[0].layer_count, 1);
    assert_eq!(constructions[0].ctf_count, 1);
    assert_eq!(constructions[0].timestep_hours, 0.25);
    assert_eq!(constructions[0].thermal_conductance_w_per_m2_k, 0.4365);
    assert_eq!(constructions[0].roughness, "Rough");

    Ok(())
}

#[test]
fn parses_eio_construction_ctf_coefficient_rows() -> Result<(), Box<dyn std::error::Error>> {
    let coefficients = parse_eio_construction_ctf_coefficients(
        r#"! <Construction CTF>,Construction Name,...
! <Material CTF Summary>,Material Name,...
! <CTF>,Time,Outside,Cross,Inside,Flux (except final one)
 Construction CTF,FLOOR,   2,   1,   5,   0.250,          17.04,   0.900,   0.900,   0.650,   0.650,MediumRough
 Material CTF Summary,C5 - 4 IN HW CONCRETE,  0.1015,         1.730,   2242.585,      836.800,     0.05868
 CTF,   1,          -62.622544,           4.7096437,          -62.622544,          0.60555731
 CTF,   0,            58.08561,          0.72354869,            58.08561
"#,
    )?;

    assert_eq!(coefficients.len(), 2);
    assert_eq!(coefficients[0].construction_name, "FLOOR");
    assert_eq!(coefficients[0].time_index, 1);
    assert_eq!(coefficients[0].outside, -62.622544);
    assert_eq!(coefficients[0].cross, 4.7096437);
    assert_eq!(coefficients[0].inside, -62.622544);
    assert_eq!(coefficients[0].flux, Some(0.60555731));
    assert_eq!(coefficients[1].construction_name, "FLOOR");
    assert_eq!(coefficients[1].time_index, 0);
    assert_eq!(coefficients[1].outside, 58.08561);
    assert_eq!(coefficients[1].cross, 0.72354869);
    assert_eq!(coefficients[1].inside, 58.08561);
    assert_eq!(coefficients[1].flux, None);

    Ok(())
}

#[test]
fn preserves_energyplus_mass_ctf_emission_order() -> Result<(), Box<dyn std::error::Error>> {
    let coefficients = parse_eio_construction_ctf_coefficients(
        r#"! <Construction CTF>,Construction Name,...
! <CTF>,Time,Outside,Cross,Inside,Flux (except final one)
 Construction CTF,FLOOR,   2,   1,   5,   0.250,          17.04,   0.900,   0.900,   0.650,   0.650,MediumRough
 CTF,   5,      -4.1142049E-08,       1.5543709E-08,      -4.1142049E-08,       1.2297289E-11
 CTF,   4,       0.00057884701,       0.00022976293,       0.00057884701,      -4.0580373E-07
 CTF,   3,         -0.33051123,         0.091914804,         -0.33051123,        0.0006592243
 CTF,   2,           12.566595,           2.1743923,           12.566595,        -0.058066613
 CTF,   1,          -62.622544,           4.7096437,          -62.622544,          0.60555731
 CTF,   0,            58.08561,          0.72354869,            58.08561
"#,
    )?;

    let emitted_times = coefficients
        .iter()
        .map(|coefficient| coefficient.time_index)
        .collect::<Vec<_>>();
    assert_eq!(emitted_times, vec![5, 4, 3, 2, 1, 0]);
    let emitted_history_times = coefficients
        .iter()
        .filter(|coefficient| coefficient.time_index > 0)
        .map(|coefficient| coefficient.time_index)
        .collect::<Vec<_>>();
    assert_eq!(emitted_history_times, vec![5, 4, 3, 2, 1]);

    Ok(())
}

#[test]
fn parses_eio_material_ctf_summary_rows() -> Result<(), Box<dyn std::error::Error>> {
    let materials = parse_eio_material_ctf_summary(
        r#"! <Material CTF Summary>,Material Name,...
 Material CTF Summary,R13LAYER,  0.0000,         0.000,      0.000,        0.000,       2.291
"#,
    )?;

    assert_eq!(materials.len(), 1);
    assert_eq!(materials[0].material_name, "R13LAYER");
    assert_eq!(materials[0].thickness_m, 0.0);
    assert_eq!(materials[0].conductivity_w_per_m_k, 0.0);
    assert_eq!(materials[0].density_kg_per_m3, 0.0);
    assert_eq!(materials[0].specific_heat_j_per_kg_k, 0.0);
    assert_eq!(materials[0].thermal_resistance_m2_k_per_w, 2.291);

    Ok(())
}

#[test]
fn parses_eio_window_material_glazing_rows_and_preserves_repeats()
-> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_eio_window_material_glazing(
        r#"! <WindowMaterial:Glazing>, Material Name, Optical Data Type, Spectral Data Set Name, Thickness {m}, Solar Transmittance,Front Solar Reflectance, Back Solar Reflectance, Visible Transmittance, Front Visible Reflectance,Back Visible Reflectance,Infrared Transmittance, Front Thermal Emissivity, Back Thermal Emissivity,Conductivity {W/m-K},Dirt Factor,Solar Diffusing
 WindowMaterial:Glazing, clear glass , SpectralAverage, , 0.00600, 0.77500, 0.07100, 0.07100, 0.88100, 0.08000, 0.08000, 0.00000, 0.84000, 0.84000, 1.00000, 0.90000, No
 WindowMaterial:Glazing, clear glass , SpectralAverage, , 0.00600, 0.77500, 0.07100, 0.07100, 0.88100, 0.08000, 0.08000, 0.00000, 0.84000, 0.84000, 1.00000, 0.90000, Yes
 WindowMaterial:Glazing, spectral glass, Spectral, custom spectral data, 0.00300, 0.70000, 0.10000, 0.11000, 0.80000, 0.09000, 0.10000, 0.01000, 0.82000, 0.83000, 0.90000, 1.00000, No
"#,
    )?;

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].material_name, "CLEAR GLASS");
    assert_eq!(rows[0].optical_data_type, "SpectralAverage");
    assert_eq!(rows[0].spectral_data_set_name, None);
    assert_eq!(rows[0].thickness_m, 0.006);
    assert_eq!(rows[0].solar_transmittance, 0.775);
    assert_eq!(rows[0].front_solar_reflectance, 0.071);
    assert_eq!(rows[0].back_solar_reflectance, 0.071);
    assert_eq!(rows[0].visible_transmittance, 0.881);
    assert_eq!(rows[0].front_visible_reflectance, 0.08);
    assert_eq!(rows[0].back_visible_reflectance, 0.08);
    assert_eq!(rows[0].infrared_transmittance, 0.0);
    assert_eq!(rows[0].front_thermal_emissivity, 0.84);
    assert_eq!(rows[0].back_thermal_emissivity, 0.84);
    assert_eq!(rows[0].conductivity_w_per_m_k, 1.0);
    assert_eq!(rows[0].dirt_factor, 0.9);
    assert!(!rows[0].solar_diffusing);
    assert_eq!(rows[1].material_name, rows[0].material_name);
    assert!(rows[1].solar_diffusing);
    assert_eq!(
        rows[2].spectral_data_set_name.as_deref(),
        Some("CUSTOM SPECTRAL DATA")
    );

    Ok(())
}

#[test]
fn eio_window_material_glazing_parser_requires_exact_field_count() {
    let too_few = parse_eio_window_material_glazing(
        "WindowMaterial:Glazing,GLASS,SpectralAverage,,0.006,0.775,0.071,0.071,0.881,0.08,0.08,0.0,0.84,0.84,1.0,0.9\n",
    )
    .expect_err("a glazing row with one missing value must fail");
    let too_many = parse_eio_window_material_glazing(
        "WindowMaterial:Glazing,GLASS,SpectralAverage,,0.006,0.775,0.071,0.071,0.881,0.08,0.08,0.0,0.84,0.84,1.0,0.9,No,EXTRA\n",
    )
    .expect_err("a glazing row with an extra value must fail");

    assert!(matches!(
        too_few,
        EioError::InvalidWindowMaterialGlazing { line: 1, .. }
    ));
    assert!(matches!(
        too_many,
        EioError::InvalidWindowMaterialGlazing { line: 1, .. }
    ));
}

#[test]
fn eio_window_material_glazing_parser_rejects_invalid_field_types() {
    let invalid_number = parse_eio_window_material_glazing(
        "WindowMaterial:Glazing,GLASS,SpectralAverage,,not-a-number,0.775,0.071,0.071,0.881,0.08,0.08,0.0,0.84,0.84,1.0,0.9,No\n",
    )
    .expect_err("invalid numeric fields must fail");
    let invalid_bool = parse_eio_window_material_glazing(
        "WindowMaterial:Glazing,GLASS,SpectralAverage,,0.006,0.775,0.071,0.071,0.881,0.08,0.08,0.0,0.84,0.84,1.0,0.9,Maybe\n",
    )
    .expect_err("Solar Diffusing must be Yes or No");

    assert!(matches!(
        &invalid_number,
        EioError::InvalidWindowMaterialGlazing { .. }
    ));
    if let EioError::InvalidWindowMaterialGlazing { line, reason, .. } = invalid_number {
        assert_eq!(line, 1);
        assert_eq!(reason, "invalid Thickness {m}");
    }
    assert!(matches!(
        &invalid_bool,
        EioError::InvalidWindowMaterialGlazing { .. }
    ));
    if let EioError::InvalidWindowMaterialGlazing { line, reason, .. } = invalid_bool {
        assert_eq!(line, 1);
        assert!(reason.contains("expected Yes or No"));
    }
}

#[test]
fn eio_window_material_glazing_parser_reports_missing_rows() {
    assert!(matches!(
        parse_eio_window_material_glazing("Program Version,EnergyPlus\n"),
        Err(EioError::MissingWindowMaterialGlazing)
    ));
}

#[test]
fn parses_eio_window_material_gas_rows_and_preserves_repeats()
-> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_eio_window_material_gas(
        r#"! <WindowMaterial:Gas>,Material Name,GasType,Thickness {m}
 Program Version,EnergyPlus
 WindowMaterial:Gas, argon gap ,Argon,0.0127000
 WindowMaterial:Gas, argon gap ,Argon,1.27000E-002
 WindowMaterial:Gas, air gap,Air,0.0063500
 WindowMaterial:Glazing,IGNORED,SpectralAverage
"#,
    )?;

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].material_name, "ARGON GAP");
    assert_eq!(rows[0].gas_type, "Argon");
    assert_eq!(rows[0].thickness_m, 0.0127);
    assert_eq!(rows[1].material_name, rows[0].material_name);
    assert_eq!(rows[1].gas_type, rows[0].gas_type);
    assert_eq!(rows[1].thickness_m, rows[0].thickness_m);
    assert_eq!(rows[2].material_name, "AIR GAP");
    assert_eq!(rows[2].gas_type, "Air");
    assert_eq!(rows[2].thickness_m, 0.00635);
    Ok(())
}

#[test]
fn eio_window_material_gas_parser_requires_exact_field_count() {
    let too_few = parse_eio_window_material_gas("WindowMaterial:Gas,GAP,Argon\n")
        .expect_err("a gas row with one missing value must fail");
    let too_many = parse_eio_window_material_gas("WindowMaterial:Gas,GAP,Argon,0.0127,EXTRA\n")
        .expect_err("a gas row with an extra value must fail");

    assert!(matches!(
        too_few,
        EioError::InvalidWindowMaterialGas { line: 1, .. }
    ));
    assert!(matches!(
        too_many,
        EioError::InvalidWindowMaterialGas { line: 1, .. }
    ));
}

#[test]
fn eio_window_material_gas_parser_rejects_invalid_fields() {
    let invalid_number =
        parse_eio_window_material_gas("WindowMaterial:Gas,GAP,Argon,not-a-number\n")
            .expect_err("invalid gas thickness must fail");
    let missing_name = parse_eio_window_material_gas("WindowMaterial:Gas,,Argon,0.0127\n")
        .expect_err("missing gas material name must fail");
    let missing_type = parse_eio_window_material_gas("WindowMaterial:Gas,GAP,,0.0127\n")
        .expect_err("missing gas type must fail");

    assert!(matches!(
        &invalid_number,
        EioError::InvalidWindowMaterialGas { .. }
    ));
    if let EioError::InvalidWindowMaterialGas { line, reason, .. } = invalid_number {
        assert_eq!(line, 1);
        assert_eq!(reason, "invalid Thickness {m}");
    }
    assert!(matches!(
        &missing_name,
        EioError::InvalidWindowMaterialGas { .. }
    ));
    if let EioError::InvalidWindowMaterialGas { line, reason, .. } = missing_name {
        assert_eq!(line, 1);
        assert_eq!(reason, "missing Material Name");
    }
    assert!(matches!(
        &missing_type,
        EioError::InvalidWindowMaterialGas { .. }
    ));
    if let EioError::InvalidWindowMaterialGas { line, reason, .. } = missing_type {
        assert_eq!(line, 1);
        assert_eq!(reason, "missing Gas Type");
    }

    for invalid_thickness in ["0", "-0.0127", "NaN", "inf"] {
        let error = parse_eio_window_material_gas(&format!(
            "WindowMaterial:Gas,GAP,Argon,{invalid_thickness}\n"
        ))
        .expect_err("non-positive or non-finite gas thickness must fail");
        assert!(matches!(&error, EioError::InvalidWindowMaterialGas { .. }));
        if let EioError::InvalidWindowMaterialGas { line, reason, .. } = error {
            assert_eq!(line, 1);
            assert_eq!(reason, "Thickness {m} must be finite and greater than zero");
        }
    }
}

#[test]
fn eio_window_material_gas_parser_reports_missing_rows() {
    assert!(matches!(
        parse_eio_window_material_gas("Program Version,EnergyPlus\n"),
        Err(EioError::MissingWindowMaterialGas)
    ));
}

#[test]
fn parses_eio_equivalent_layer_gap_rows_and_preserves_repeats()
-> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_eio_window_material_gap_equivalent_layer(
        r#"! <WindowMaterial:Gap:EquivalentLayer>, Material Name, GasType, Gap Thickness {m}, Gap Vent Type
 Program Version,EnergyPlus
 WindowMaterial:Gap:EquivalentLayer, argon eql gap ,Argon,0.0127000,Sealed
 WindowMaterial:Gap:EquivalentLayer, argon eql gap ,Argon,1.27000E-002,VentedIndoor
 WindowMaterial:Gap:EquivalentLayer, air eql gap,Air,0.0063500,VentedOutdoor
 WindowMaterial:Gap:EquivalentLayerExtra,IGNORED,Air,0.1,Sealed
 windowmaterial:gap:equivalentlayer,IGNORED,Air,0.1,Sealed
"#,
    )?;

    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].material_name, "ARGON EQL GAP");
    assert_eq!(rows[0].gas_type, "Argon");
    assert_eq!(rows[0].gap_thickness_m, 0.0127);
    assert_eq!(rows[0].gap_vent_type, "Sealed");
    assert_eq!(rows[1].material_name, rows[0].material_name);
    assert_eq!(rows[1].gas_type, rows[0].gas_type);
    assert_eq!(rows[1].gap_thickness_m, rows[0].gap_thickness_m);
    assert_eq!(rows[1].gap_vent_type, "VentedIndoor");
    assert_eq!(rows[2].material_name, "AIR EQL GAP");
    assert_eq!(rows[2].gas_type, "Air");
    assert_eq!(rows[2].gap_thickness_m, 0.00635);
    assert_eq!(rows[2].gap_vent_type, "VentedOutdoor");
    Ok(())
}

#[test]
fn eio_equivalent_layer_gap_parser_requires_exact_field_count() {
    let too_few = parse_eio_window_material_gap_equivalent_layer(
        "WindowMaterial:Gap:EquivalentLayer,GAP,Argon,0.0127\n",
    )
    .expect_err("an equivalent-layer gap row with one missing value must fail");
    let too_many = parse_eio_window_material_gap_equivalent_layer(
        "WindowMaterial:Gap:EquivalentLayer,GAP,Argon,0.0127,Sealed,EXTRA\n",
    )
    .expect_err("an equivalent-layer gap row with an extra value must fail");

    assert!(matches!(
        too_few,
        EioError::InvalidWindowMaterialGapEquivalentLayer { line: 1, .. }
    ));
    assert!(matches!(
        too_many,
        EioError::InvalidWindowMaterialGapEquivalentLayer { line: 1, .. }
    ));
}

#[test]
fn eio_equivalent_layer_gap_parser_rejects_invalid_fields() {
    let invalid_number = parse_eio_window_material_gap_equivalent_layer(
        "WindowMaterial:Gap:EquivalentLayer,GAP,Argon,not-a-number,Sealed\n",
    )
    .expect_err("invalid equivalent-layer gap thickness must fail");
    let missing_name = parse_eio_window_material_gap_equivalent_layer(
        "WindowMaterial:Gap:EquivalentLayer,,Argon,0.0127,Sealed\n",
    )
    .expect_err("missing equivalent-layer gap material name must fail");
    let missing_type = parse_eio_window_material_gap_equivalent_layer(
        "WindowMaterial:Gap:EquivalentLayer,GAP,,0.0127,Sealed\n",
    )
    .expect_err("missing equivalent-layer gap gas type must fail");
    let missing_vent_type = parse_eio_window_material_gap_equivalent_layer(
        "WindowMaterial:Gap:EquivalentLayer,GAP,Argon,0.0127,\n",
    )
    .expect_err("missing equivalent-layer gap vent type must fail");

    assert!(matches!(
        &invalid_number,
        EioError::InvalidWindowMaterialGapEquivalentLayer { .. }
    ));
    if let EioError::InvalidWindowMaterialGapEquivalentLayer { line, reason, .. } = invalid_number {
        assert_eq!(line, 1);
        assert_eq!(reason, "invalid Gap Thickness {m}");
    }
    for (error, expected_reason) in [
        (missing_name, "missing Material Name"),
        (missing_type, "missing Gas Type"),
        (missing_vent_type, "missing Gap Vent Type"),
    ] {
        assert!(matches!(
            &error,
            EioError::InvalidWindowMaterialGapEquivalentLayer { .. }
        ));
        if let EioError::InvalidWindowMaterialGapEquivalentLayer { line, reason, .. } = error {
            assert_eq!(line, 1);
            assert_eq!(reason, expected_reason);
        }
    }

    for invalid_thickness in ["0", "-0.0127", "NaN", "inf", "-inf"] {
        let error = parse_eio_window_material_gap_equivalent_layer(&format!(
            "WindowMaterial:Gap:EquivalentLayer,GAP,Argon,{invalid_thickness},Sealed\n"
        ))
        .expect_err("non-positive or non-finite equivalent-layer gap thickness must fail");
        assert!(matches!(
            &error,
            EioError::InvalidWindowMaterialGapEquivalentLayer { .. }
        ));
        if let EioError::InvalidWindowMaterialGapEquivalentLayer { line, reason, .. } = error {
            assert_eq!(line, 1);
            assert_eq!(
                reason,
                "Gap Thickness {m} must be finite and greater than zero"
            );
        }
    }
}

#[test]
fn eio_equivalent_layer_gap_parser_reports_missing_rows() {
    assert!(matches!(
        parse_eio_window_material_gap_equivalent_layer("Program Version,EnergyPlus\n"),
        Err(EioError::MissingWindowMaterialGapEquivalentLayer)
    ));
}

#[test]
fn parses_eio_equivalent_layer_glazing_rows_and_preserves_repeats()
-> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_eio_window_material_glazing_equivalent_layer(
        r#"! <WindowMaterial:Glazing:EquivalentLayer>,Material Name,Optical Data Type,Spectral Data Set Name,Front Beam-Beam Solar Transmittance,Back Beam-Beam Solar Transmittance,Front Beam-Beam Solar Reflectance,Back Beam-Beam Solar Reflectance,Front Beam-Diffuse Solar Transmittance,Back Beam-Diffuse Solar Transmittance,Front Beam-Diffuse Solar Reflectance,Back Beam-Diffuse Solar Reflectance,Diffuse-Diffuse Solar Transmittance,Front Diffuse-Diffuse Solar Reflectance,Back Diffuse-Diffuse Solar Reflectance,Infrared Transmittance,Front Infrared Emissivity,Back Infrared Emissivity
 WindowMaterial:Glazing:EquivalentLayer, equivalent glass ,SpectralAverage,,0.61200,0.61300,0.13700,0.14900,3.10000E-002,0.03200,0.14100,0.14200,0.50100,0.20100,0.20200,1.10000E-002,0.82300,0.78600
 WindowMaterial:Glazing:EquivalentLayer, equivalent glass ,SpectralAverage,,0.71200,0.71300,0.23700,0.24900,0.04100,0.04200,0.24100,0.24200,-99999.00000,-99999.00000,-99999.00000,0.02100,0.83300,0.79600
"#,
    )?;

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].material_name, "EQUIVALENT GLASS");
    assert_eq!(rows[0].optical_data_type, "SpectralAverage");
    assert_eq!(rows[0].spectral_data_set_name, None);
    assert_eq!(rows[0].front_beam_beam_solar_transmittance, 0.612);
    assert_eq!(rows[0].back_beam_beam_solar_transmittance, 0.613);
    assert_eq!(rows[0].front_beam_beam_solar_reflectance, 0.137);
    assert_eq!(rows[0].back_beam_beam_solar_reflectance, 0.149);
    assert_eq!(rows[0].front_beam_diffuse_solar_transmittance, 0.031);
    assert_eq!(rows[0].back_beam_diffuse_solar_transmittance, 0.032);
    assert_eq!(rows[0].front_beam_diffuse_solar_reflectance, 0.141);
    assert_eq!(rows[0].back_beam_diffuse_solar_reflectance, 0.142);
    assert_eq!(rows[0].diffuse_diffuse_solar_transmittance, 0.501);
    assert_eq!(rows[0].front_diffuse_diffuse_solar_reflectance, 0.201);
    assert_eq!(rows[0].back_diffuse_diffuse_solar_reflectance, 0.202);
    assert_eq!(rows[0].infrared_transmittance, 0.011);
    assert_eq!(rows[0].front_infrared_emissivity, 0.823);
    assert_eq!(rows[0].back_infrared_emissivity, 0.786);
    assert_eq!(rows[1].material_name, rows[0].material_name);
    assert_eq!(rows[1].front_beam_beam_solar_transmittance, 0.712);
    assert_eq!(rows[1].diffuse_diffuse_solar_transmittance, -99_999.0);
    assert_eq!(rows[1].front_diffuse_diffuse_solar_reflectance, -99_999.0);
    assert_eq!(rows[1].back_diffuse_diffuse_solar_reflectance, -99_999.0);
    Ok(())
}

#[test]
fn eio_equivalent_layer_glazing_parser_requires_exact_field_count() {
    let too_few = parse_eio_window_material_glazing_equivalent_layer(
        "WindowMaterial:Glazing:EquivalentLayer,GLASS,SpectralAverage,,0.612,0.613,0.137,0.149,0.031,0.032,0.141,0.142,0.501,0.201,0.202,0.011,0.823\n",
    )
    .expect_err("an equivalent-layer row with one missing value must fail");
    let too_many = parse_eio_window_material_glazing_equivalent_layer(
        "WindowMaterial:Glazing:EquivalentLayer,GLASS,SpectralAverage,,0.612,0.613,0.137,0.149,0.031,0.032,0.141,0.142,0.501,0.201,0.202,0.011,0.823,0.786,EXTRA\n",
    )
    .expect_err("an equivalent-layer row with an extra value must fail");

    assert!(matches!(
        too_few,
        EioError::InvalidWindowMaterialGlazingEquivalentLayer { line: 1, .. }
    ));
    assert!(matches!(
        too_many,
        EioError::InvalidWindowMaterialGlazingEquivalentLayer { line: 1, .. }
    ));
}

#[test]
fn eio_equivalent_layer_glazing_parser_rejects_invalid_fields() {
    let invalid_number = parse_eio_window_material_glazing_equivalent_layer(
        "WindowMaterial:Glazing:EquivalentLayer,GLASS,SpectralAverage,,not-a-number,0.613,0.137,0.149,0.031,0.032,0.141,0.142,0.501,0.201,0.202,0.011,0.823,0.786\n",
    )
    .expect_err("invalid equivalent-layer numeric fields must fail");
    let missing_name = parse_eio_window_material_glazing_equivalent_layer(
        "WindowMaterial:Glazing:EquivalentLayer,,SpectralAverage,,0.612,0.613,0.137,0.149,0.031,0.032,0.141,0.142,0.501,0.201,0.202,0.011,0.823,0.786\n",
    )
    .expect_err("missing equivalent-layer material name must fail");

    assert!(
        matches!(
            invalid_number,
            EioError::InvalidWindowMaterialGlazingEquivalentLayer { .. }
        ),
        "unexpected equivalent-layer numeric error: {invalid_number}"
    );
    if let EioError::InvalidWindowMaterialGlazingEquivalentLayer { line, reason, .. } =
        invalid_number
    {
        assert_eq!(line, 1);
        assert_eq!(reason, "invalid Front Side Beam-Beam Solar Transmittance");
    }
    assert!(
        matches!(
            missing_name,
            EioError::InvalidWindowMaterialGlazingEquivalentLayer { .. }
        ),
        "unexpected equivalent-layer name error: {missing_name}"
    );
    if let EioError::InvalidWindowMaterialGlazingEquivalentLayer { line, reason, .. } = missing_name
    {
        assert_eq!(line, 1);
        assert_eq!(reason, "missing Material Name");
    }
}

#[test]
fn eio_equivalent_layer_glazing_parser_reports_missing_rows() {
    assert!(matches!(
        parse_eio_window_material_glazing_equivalent_layer("Program Version,EnergyPlus\n"),
        Err(EioError::MissingWindowMaterialGlazingEquivalentLayer)
    ));
}

#[test]
fn groups_ordered_generic_and_air_material_rows_by_construction()
-> Result<(), Box<dyn std::error::Error>> {
    let summaries = parse_eio_construction_material_summaries(
        r#"! <Construction CTF>,Construction Name,...
! <Material CTF Summary>,Material Name,...
! <Material:Air CTF Summary>,Material Name,ThermalResistance {m2-K/w}
 Construction CTF,MIXED WALL,   1,   3,   1,   0.250,         2.5000,   0.900,   0.900,   0.750,   0.750,Rough
 Material CTF Summary,OUTSIDE SOLID,  0.0500,         1.000,    800.000,      900.000,       0.050
 Material:Air CTF Summary,CENTER AIR GAP,       0.180
 Material CTF Summary,INSIDE SOLID,  0.1200,         1.200,    900.000,     1000.000,       0.100
 Construction CTF,IR ONLY,   2,   1,   1,   0.250,       100.0000,   1.000,   1.000,   1.000,   1.000,None
 Material CTF Summary,IR TRANSPARENT,  0.0000,         0.000,      0.000,        0.000,       0.010
"#,
    )?;

    assert_eq!(summaries.len(), 2);
    assert_eq!(summaries[0].construction.construction_name, "MIXED WALL");
    assert_eq!(summaries[0].construction.layer_count, 3);
    assert_eq!(
        summaries[0]
            .layers
            .iter()
            .map(|layer| layer.material_name.as_str())
            .collect::<Vec<_>>(),
        vec!["OUTSIDE SOLID", "CENTER AIR GAP", "INSIDE SOLID"]
    );
    assert_eq!(
        summaries[0]
            .layers
            .iter()
            .map(|layer| layer.thermal_resistance_m2_k_per_w)
            .collect::<Vec<_>>(),
        vec![0.05, 0.18, 0.1]
    );
    assert_eq!(
        summaries[0].layers[1].summary_format,
        crate::EioMaterialCtfSummaryFormat::Air
    );
    assert_eq!(summaries[0].layers[1].thickness_m, None);

    // EnergyPlus emits IRT through the generic row, which does not encode its object type.
    assert_eq!(summaries[1].layers[0].material_name, "IR TRANSPARENT");
    assert_eq!(
        summaries[1].layers[0].summary_format,
        crate::EioMaterialCtfSummaryFormat::Material
    );
    assert_eq!(summaries[1].layers[0].thermal_resistance_m2_k_per_w, 0.01);

    Ok(())
}

#[test]
fn grouped_material_parser_rejects_declared_layer_count_mismatch() {
    let error = parse_eio_construction_material_summaries(
        r#" Construction CTF,MIXED WALL,   1,   2,   1,   0.250,         4.3478,   0.900,   0.900,   0.750,   0.750,Rough
 Material CTF Summary,ONLY ONE,  0.0500,         1.000,    800.000,      900.000,       0.050
"#,
    )
    .expect_err("declared and emitted layer counts must agree");

    assert!(matches!(
        error,
        EioError::InvalidConstructionMaterialSummary { .. }
    ));
    if let EioError::InvalidConstructionMaterialSummary { line, reason, .. } = error {
        assert_eq!(line, 1);
        assert!(reason.contains("declares 2 layers but has 1"));
    }
}

#[test]
fn grouped_material_parser_rejects_layer_before_construction() {
    let error =
        parse_eio_construction_material_summaries(" Material:Air CTF Summary,ORPHAN GAP,0.180\n")
            .expect_err("a layer must belong to a construction");

    assert!(matches!(
        error,
        EioError::InvalidConstructionMaterialSummary { line: 1, .. }
    ));
}

#[test]
fn parses_eio_warmup_environment_rows() -> Result<(), Box<dyn std::error::Error>> {
    let rows = parse_eio_warmup_environments(
        r#"! <Environment>,Environment Name,Environment Type
Environment,DENVER ANN HTG,SizingPeriod:DesignDay,12/21,12/21
Environment:WarmupDays, 22
Environment,RUN PERIOD 1,WeatherFileRunPeriod,01/01/2013,12/31/2013
Environment:WarmupDays, 20
"#,
    )?;

    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0].environment_name, "DENVER ANN HTG");
    assert_eq!(rows[0].environment_type, "SizingPeriod:DesignDay");
    assert_eq!(rows[0].warmup_days, 22);
    assert_eq!(rows[1].environment_name, "RUN PERIOD 1");
    assert_eq!(rows[1].environment_type, "WeatherFileRunPeriod");
    assert_eq!(rows[1].warmup_days, 20);

    Ok(())
}

#[test]
fn series_comparison_tracks_max_delta() {
    let result = compare_series(&[1.0, 2.0], &[1.0, 2.000_000_000_1], Tolerance::default());

    assert!(result.passed);
    assert_eq!(result.samples, 2);
    assert!(result.max_abs_delta > 0.0);
    assert_eq!(result.first_divergence, None);
}

#[test]
fn series_comparison_reports_first_value_divergence() -> Result<(), Box<dyn std::error::Error>> {
    let result = compare_series(&[1.0, 2.0, 3.0], &[1.0, 2.5, 4.0], Tolerance::default());

    assert!(!result.passed);
    let divergence = result
        .first_divergence
        .ok_or_else(|| std::io::Error::other("expected first divergence"))?;
    assert_eq!(divergence.index, 1);
    assert_eq!(divergence.expected, Some(2.0));
    assert_eq!(divergence.observed, Some(2.5));
    assert_eq!(divergence.abs_delta, Some(0.5));

    Ok(())
}

#[test]
fn series_comparison_reports_length_divergence() -> Result<(), Box<dyn std::error::Error>> {
    let result = compare_series(&[1.0, 2.0], &[1.0], Tolerance::default());

    assert!(!result.passed);
    let divergence = result
        .first_divergence
        .ok_or_else(|| std::io::Error::other("expected first divergence"))?;
    assert_eq!(divergence.index, 1);
    assert_eq!(divergence.expected, Some(2.0));
    assert_eq!(divergence.observed, None);
    assert_eq!(divergence.abs_delta, None);

    Ok(())
}
