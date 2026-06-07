use crate::{
    Tolerance, compare_series, parse_eio_construction_ctf, parse_eio_heat_transfer_surfaces,
    parse_eio_material_ctf_summary, parse_eio_other_equipment_nominal, parse_eio_zone_geometry,
    parse_eso_series,
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
