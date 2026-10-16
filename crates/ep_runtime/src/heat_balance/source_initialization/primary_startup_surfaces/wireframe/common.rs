//! Actual WriteDXFCommon arrow geometry, labels and report-order minimum scan.
use super::*;
pub(super) fn write_common(
    reports: &CtfInitializationReports,
    of: &mut dyn Write,
    color_scheme: &str,
) -> Result<(), Error> {
    let mut stem = [
        [-10.0, 3.0, 0.1],
        [-10.0, 3.0, 0.0],
        [-10.0, 0.0, 0.0],
        [-10.0, 0.0, 0.1],
    ];
    let mut head1 = [
        [-10.0, 3.0, 0.1],
        [-10.0, 3.0, 0.0],
        [-10.5, 2.133975, 0.0],
        [-10.5, 2.133975, 0.1],
    ];
    let mut head2 = [
        [-10.0, 3.0, 0.1],
        [-10.0, 3.0, 0.0],
        [-9.5, 2.133975, 0.0],
        [-9.5, 2.133975, 0.1],
    ];
    let mut side1 = [
        [-10.5, 4.5, 0.1],
        [-10.5, 4.5, 0.0],
        [-10.5, 3.5, 0.0],
        [-10.5, 3.5, 0.1],
    ];
    let mut side2 = [
        [-10.5, 4.5, 0.1],
        [-10.5, 4.5, 0.0],
        [-9.5, 3.5, 0.0],
        [-9.5, 3.5, 0.1],
    ];
    let mut side3 = [
        [-9.5, 4.5, 0.1],
        [-9.5, 4.5, 0.0],
        [-9.5, 3.5, 0.0],
        [-9.5, 3.5, 0.1],
    ];
    if color_scheme.is_empty() {
        of.write_all(b"999\nColor Scheme,Default\n")?;
    } else {
        write!(of, "999\nColor Scheme,{color_scheme}\n")?;
    }
    let mut minx = 99999.0;
    let mut miny = 99999.0;
    for &number in &reports.surface_geometry_input.all_surface_list_report_order {
        let row = surface(reports, number)?;
        if row.class == SurfaceClass::IntMass {
            continue;
        }
        for v in 1..=row.sides {
            minx = source_min(minx, vertex(row, v)?[0]);
            miny = source_min(miny, vertex(row, v)?[1]);
        }
    }
    for v in 0..4 {
        stem[v][0] += minx;
        stem[v][1] += miny;
        head1[v][0] += minx;
        head1[v][1] += miny;
        head2[v][0] += minx;
        head2[v][1] += miny;
        side1[v][0] += minx;
        side1[v][1] += miny;
        side2[v][0] += minx;
        side2[v][1] += miny;
        side3[v][0] += minx;
        side3[v][1] += miny;
    }
    let colors = &reports.source_surface_colors;
    of.write_all(b"999\nText - True North\n")?;
    write!(
        of,
        "  0\nTEXT\n  8\n1\n  6\nContinuous\n 62\n{:3}\n 10\n{}\n 20\n{}\n 30\n{}\n 40\n .25\n  1\nTrue North\n 41\n 0.0\n  7\nMONOTXT\n210\n0.0\n220\n0.0\n230\n1.0\n",
        color(colors, Some(0))?,
        fixed(stem[0][0] - 1.0, 15, 5),
        fixed(stem[0][1], 15, 5),
        fixed(stem[0][2], 15, 5)
    )?;
    of.write_all(b"999\nText - Building Title\n")?;
    write!(
        of,
        "  0\nTEXT\n  8\n1\n  6\nContinuous\n 62\n{:3}\n 10\n{}\n 20\n{}\n 30\n{}\n 40\n .4\n  1\nBuilding - {}\n 41\n 0.0\n  7\nMONOTXT\n210\n0.0\n220\n0.0\n230\n1.0\n",
        color(colors, Some(0))?,
        fixed(stem[0][0] - 4.0, 15, 5),
        fixed(stem[0][1] - 4.0, 15, 5),
        fixed(stem[0][2], 15, 5),
        reports.heat_balance_input_state.building_name
    )?;
    face(of, colors, "North Arrow Stem", &stem)?;
    face(of, colors, "North Arrow Head 1", &head1)?;
    face(of, colors, "North Arrow Head 2", &head2)?;
    face(of, colors, "North Arrow Side 1", &side1)?;
    face(of, colors, "North Arrow Side 2", &side2)?;
    face(of, colors, "North Arrow Side 3", &side3)?;
    of.write_all(b"999\nZone Names\n")?;
    for zone in 1..=zone_count(reports)? {
        write!(
            of,
            "999\nZone={zone}:{}\n",
            normalize_name(zone_name(reports, zone)?)
        )?;
    }
    Ok(())
}
fn face(
    of: &mut dyn Write,
    colors: &SourceSurfaceColors,
    title: &str,
    points: &[[f64; 3]; 4],
) -> Result<(), Error> {
    write!(of, "999\n{title}\n")?;
    write!(
        of,
        "  0\n3DFACE\n  8\n1\n 62\n{:3}\n",
        color(colors, Some(0))?
    )?;
    for (index, point) in points.iter().enumerate() {
        write!(
            of,
            " {}\n{}\n {}\n{}\n {}\n{}\n",
            10 + index,
            fixed(point[0], 15, 5),
            20 + index,
            fixed(point[1], 15, 5),
            30 + index,
            fixed(point[2], 15, 5)
        )?;
    }
    Ok(())
}
