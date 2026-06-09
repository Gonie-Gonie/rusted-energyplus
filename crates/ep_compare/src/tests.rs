use crate::{
    SeriesAlignment, SeriesDivergenceKind, SeriesSample, Tolerance, compare_series,
    compare_series_samples_v2, compare_series_v2, parse_eio_construction_ctf,
    parse_eio_construction_ctf_coefficients, parse_eio_heat_transfer_surfaces,
    parse_eio_material_ctf_summary, parse_eio_other_equipment_nominal,
    parse_eio_warmup_environments, parse_eio_zone_geometry, parse_eso_series,
    parse_eso_time_series,
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

    Ok(())
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
    let runtime_history_times = coefficients
        .iter()
        .filter(|coefficient| coefficient.time_index > 0)
        .map(|coefficient| coefficient.time_index)
        .collect::<Vec<_>>();
    assert_eq!(runtime_history_times, vec![5, 4, 3, 2, 1]);

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
