//! Genuine InitSolarViewFactors/InitInteriorRadExchange report children.
//! Read only the same Source arrays at their original call position; no seed,
//! corrected matrix, thermal state or report cache is reconstructed here.
use super::{OrdinaryRadiationError as Error, OrdinaryRadiationUnavailable as Unavailable};
use super::{RadiantRadiationEnclosure, SolarRadiationEnclosure};
use crate::error::WarmupOutputError;
use crate::heat_balance::selected_heat_balance_input::surface::{
    SurfaceClass, SurfaceGeometryInputState, SurfaceTmpInput,
};
use std::io::Write;

pub(super) fn header(solar: bool, eio: &mut dyn Write) -> Result<(), Error> {
    let text = if solar {
        "! <Solar View Factor Information>\n! <Solar View Factor - Zone/Enclosure Information>,Zone/Enclosure Name,Number of Surfaces\n! <Solar View Factor - Surface Information>,Surface Name,Surface Class,Area {m2},Azimuth,Tilt,Solar Absorbtance,#Sides,Vertices\n! <Solar View Factor / Interchange Type>,Surface Name(s)\n! <Solar View Factor>,Surface Name,Surface Class,Row Sum,View Factors for each Surface\n"
    } else {
        "! <Surface View Factor and Grey Interchange Information>\n! <View Factor - Zone/Enclosure Information>,Zone/Enclosure Name,Number of Surfaces\n! <View Factor - Surface Information>,Surface Name,Surface Class,Area {m2},Azimuth,Tilt,Thermal Emissivity,#Sides,Vertices\n! <View Factor / Grey Interchange Type>,Surface Name(s)\n! <View Factor>,Surface Name,Surface Class,Row Sum,View Factors for each Surface\n"
    };
    // Native emits the five lines separately, preserving every earlier write.
    for line in text.split_inclusive('\n') {
        eio.write_all(line.as_bytes())?;
    }
    Ok(())
}

fn at(value: i32) -> Result<usize, Error> {
    value
        .checked_sub(1)
        .and_then(|v| usize::try_from(v).ok())
        .ok_or_else(|| Unavailable::StorageDomain.into())
}
fn round(value: f64, precision: usize) -> Result<String, Error> {
    crate::heat_balance::source_round_format::round(value, precision).map_err(|error| {
        Error::Output(WarmupOutputError::Format(format!(
            "ViewFactorInfo .{precision}R: {error:?}"
        )))
    })
}
fn class(surface: &SurfaceTmpInput) -> Result<&'static str, Error> {
    match surface.class {
        SurfaceClass::Wall => Ok("Wall"),
        SurfaceClass::Floor => Ok("Floor"),
        SurfaceClass::Roof => Ok("Roof"),
        _ => Err(Unavailable::InputBranch.into()),
    }
}

struct View<'a> {
    name: &'a str,
    count: usize,
    pointers: &'a [i32],
    order: &'a [i32],
    area: &'a [f64],
    azimuth: &'a [f64],
    tilt: &'a [f64],
    absorptance_or_emissivity: &'a [f64],
    f: &'a [f64],
    geometry: &'a SurfaceGeometryInputState,
}
impl View<'_> {
    fn row(&self, position: i32) -> Result<&SurfaceTmpInput, Error> {
        let number = *self
            .pointers
            .get(at(position)?)
            .ok_or(Unavailable::StorageDomain)?;
        self.geometry
            .final_surfaces
            .as_ref()
            .and_then(|rows| rows.get(at(number).ok()?))
            .map(|surface| &surface.row)
            .ok_or_else(|| Unavailable::StorageDomain.into())
    }
    fn value(&self, values: &[f64], position: i32) -> Result<f64, Error> {
        values
            .get(at(position)?)
            .copied()
            .ok_or_else(|| Unavailable::StorageDomain.into())
    }
    /// Flat Original F(first,second), not a report-order remapped matrix.
    fn matrix(&self, values: &[f64], first: i32, second: i32) -> Result<f64, Error> {
        let first = at(first)?;
        let second = at(second)?;
        if first >= self.count || second >= self.count {
            return Err(Unavailable::StorageDomain.into());
        }
        let index = first
            .checked_mul(self.count)
            .and_then(|v| v.checked_add(second))
            .ok_or(Unavailable::StorageDomain)?;
        values
            .get(index)
            .copied()
            .ok_or_else(|| Unavailable::StorageDomain.into())
    }
    fn column_sum(&self, values: &[f64], column: i32) -> Result<f64, Error> {
        let mut sum = 0.0;
        // Native sum(F(_,Findex)) spans all enclosure rows, not report order.
        for first in 1..=self.count {
            sum += self.matrix(
                values,
                i32::try_from(first).map_err(|_| Unavailable::StorageDomain)?,
                column,
            )?;
        }
        Ok(sum)
    }
}

fn surfaces(view: &View<'_>, solar: bool, eio: &mut dyn Write) -> Result<(), Error> {
    let kind = if solar { "Solar" } else { "Surface" };
    writeln!(
        eio,
        "{kind} View Factor - Zone/Enclosure Information,{},{}",
        view.name, view.count
    )?;
    for &position in view.order {
        let row = view.row(position)?;
        write!(
            eio,
            "{kind} View Factor - Surface Information,{},{},{},{},{},{},{}",
            row.name,
            class(row)?,
            round(view.value(view.area, position)?, 4)?,
            round(view.value(view.azimuth, position)?, 4)?,
            round(view.value(view.tilt, position)?, 4)?,
            round(view.value(view.absorptance_or_emissivity, position)?, 4)?,
            row.sides
        )?;
        for vertex in 1..=row.sides {
            let vertex = row
                .vertex
                .as_ref()
                .and_then(|vertices| vertices.get(at(vertex).ok()?))
                .and_then(Option::as_ref)
                .ok_or(Unavailable::StorageDomain)?;
            write!(
                eio,
                ",{},{},{}",
                round(vertex[0], 4)?,
                round(vertex[1], 4)?,
                round(vertex[2], 4)?
            )?;
        }
        eio.write_all(b"\n")?;
    }
    Ok(())
}

fn factors(
    view: &View<'_>,
    matrix: &[f64],
    heading: &str,
    solar: bool,
    eio: &mut dyn Write,
) -> Result<(), Error> {
    write!(eio, "{heading},To Surface,Surface Class,RowSum")?;
    for &position in view.order {
        write!(eio, ",{}", view.row(position)?.name)?;
    }
    eio.write_all(b"\n")?;
    for &to in view.order {
        let row = view.row(to)?;
        write!(
            eio,
            "{},{},{},{}",
            if solar {
                "Solar View Factor"
            } else {
                "View Factor"
            },
            row.name,
            class(row)?,
            round(view.column_sum(matrix, to)?, 4)?
        )?;
        for &from in view.order {
            write!(eio, ",{}", round(view.matrix(matrix, from, to)?, 4)?)?;
        }
        eio.write_all(b"\n")?;
    }
    Ok(())
}

fn debug_idf(view: &View<'_>, solar: bool, debug: &mut dyn Write) -> Result<(), Error> {
    for original in [true, false] {
        debug.write_all(if original {
            b"!======== original input factors ===========================\n"
        } else {
            b"!============ final view factors =======================\n"
        })?;
        writeln!(
            debug,
            "ZoneProperty:UserViewFactors:BySurfaceName,{},",
            view.name
        )?;
        for &from in view.order {
            for &to in view.order {
                write!(
                    debug,
                    "  {},{},{}",
                    view.row(from)?.name,
                    view.row(to)?.name,
                    round(view.matrix(view.f, to, from)?, 6)?
                )?;
                // Preserve Original's different radiant final-block predicate,
                // including nonidentity report order; both blocks use final F.
                let last = if solar || original {
                    i32::try_from(view.count).map_err(|_| Unavailable::StorageDomain)?
                } else {
                    *view.order.last().ok_or(Unavailable::StorageDomain)?
                };
                debug.write_all(if from == last && to == last {
                    b";\n"
                } else {
                    b",\n"
                })?;
            }
        }
        debug.write_all(b"!============= end of data ======================\n")?;
    }
    Ok(())
}

pub(super) fn solar(
    row: &SolarRadiationEnclosure,
    geometry: &SurfaceGeometryInputState,
    approximate: &[f64],
    option: &str,
    eio: &mut dyn Write,
    debug: &mut Option<&mut dyn Write>,
) -> Result<(), Error> {
    let view = View {
        name: &row.name,
        count: usize::try_from(row.num_surfaces).map_err(|_| Unavailable::StorageDomain)?,
        pointers: row
            .source_surface_ptr
            .as_deref()
            .ok_or(Unavailable::StorageDomain)?,
        order: &row.source_surface_report_nums,
        area: &row.area,
        azimuth: &row.azimuth,
        tilt: &row.tilt,
        absorptance_or_emissivity: &row.absorptance,
        f: &row.f,
        geometry,
    };
    surfaces(&view, true, eio)?;
    factors(
        &view,
        approximate,
        "Approximate or User Input Solar ViewFactors",
        true,
        eio,
    )?;
    factors(&view, view.f, "Final Solar ViewFactors", true, eio)?;
    if option == "IDF" {
        debug_idf(
            &view,
            true,
            &mut **debug.as_mut().ok_or(Unavailable::ViewFactorReport)?,
        )?;
    }
    Ok(())
}

/// Current genuine radiant route copies the finalized Solar F; Native omits its
/// approximate block. Other numerical branches stay at their real unavailable call.
pub(super) fn radiant(
    row: &RadiantRadiationEnclosure,
    geometry: &SurfaceGeometryInputState,
    option: &str,
    eio: &mut dyn Write,
    debug: &mut Option<&mut dyn Write>,
) -> Result<(), Error> {
    let view = View {
        name: &row.name,
        count: usize::try_from(row.num_surfaces).map_err(|_| Unavailable::StorageDomain)?,
        pointers: row
            .source_surface_ptr
            .as_deref()
            .ok_or(Unavailable::StorageDomain)?,
        order: &row.source_surface_report_nums,
        area: row.area.as_deref().ok_or(Unavailable::StorageDomain)?,
        azimuth: row.azimuth.as_deref().ok_or(Unavailable::StorageDomain)?,
        tilt: row.tilt.as_deref().ok_or(Unavailable::StorageDomain)?,
        absorptance_or_emissivity: row
            .emissivity
            .as_deref()
            .ok_or(Unavailable::StorageDomain)?,
        f: row.f.as_deref().ok_or(Unavailable::StorageDomain)?,
        geometry,
    };
    surfaces(&view, false, eio)?;
    factors(&view, view.f, "Final ViewFactors", false, eio)?;
    if option == "IDF" {
        debug_idf(
            &view,
            false,
            &mut **debug.as_mut().ok_or(Unavailable::ViewFactorReport)?,
        )?;
    }
    eio.write_all(b"Script F Factors,X Surface")?;
    for &position in view.order {
        write!(eio, ",{}", view.row(position)?.name)?;
    }
    eio.write_all(b"\n")?;
    let script = row.script_f.as_deref().ok_or(Unavailable::StorageDomain)?;
    for &first in view.order {
        write!(eio, "Script F Factor,{}", view.row(first)?.name)?;
        for &second in view.order {
            write!(eio, ",{}", round(view.matrix(script, first, second)?, 4)?)?;
        }
        eio.write_all(b"\n")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn source_report_bodies_keep_distinct_pre_final_script_matrices_and_idf_quirks()
    -> Result<(), Error> {
        // Boundary constructors only. These are report operands, not simulated
        // thermal matrices or a fabricated active Source owner/Native fixture.
        let mut geometry = SurfaceGeometryInputState {
            final_surfaces: Some(vec![Default::default(); 3]),
            ..Default::default()
        };
        for (i, surface) in geometry
            .final_surfaces
            .as_mut()
            .unwrap()
            .iter_mut()
            .enumerate()
        {
            surface.row.name = format!("SURFACE{}", i + 1);
            surface.row.class = SurfaceClass::Wall;
            surface.row.sides = 4;
            surface.row.vertex = Some(vec![
                Some([0., 0., 0.]),
                Some([1., 0., 0.]),
                Some([1., 1., 0.]),
                Some([0., 1., 0.]),
            ]);
        }
        let f = vec![0.11, 0.12, 0.13, 0.21, 0.22, 0.23, 0.31, 0.32, 0.33];
        let approximate = vec![0.41, 0.42, 0.43, 0.51, 0.52, 0.53, 0.61, 0.62, 0.63];
        let solar_row = SolarRadiationEnclosure {
            name: "ENCLOSURE".into(),
            num_surfaces: 3,
            source_surface_ptr: Some(vec![1, 2, 3]),
            source_surface_report_nums: vec![3, 1, 2],
            area: vec![1., 2., 3.],
            azimuth: vec![0., 90., 180.],
            tilt: vec![90.; 3],
            absorptance: vec![0.25, 0.5, 0.75],
            f: f.clone(),
            ..Default::default()
        };
        let mut eio = Vec::new();
        let mut debug = Vec::new();
        solar(
            &solar_row,
            &geometry,
            &approximate,
            "IDF",
            &mut eio,
            &mut Some(&mut debug),
        )?;
        let text = String::from_utf8(eio).unwrap();
        assert!(text.contains("Approximate or User Input Solar ViewFactors,To Surface,Surface Class,RowSum,SURFACE3,SURFACE1,SURFACE2\nSolar View Factor,SURFACE3,Wall,1.5900,0.6300,0.4300,0.5300\n"));
        assert!(text.contains("Final Solar ViewFactors,To Surface,Surface Class,RowSum,SURFACE3,SURFACE1,SURFACE2\nSolar View Factor,SURFACE3,Wall,0.6900,0.3300,0.1300,0.2300\n"));
        assert!(text.contains("Solar View Factor - Surface Information,SURFACE3,Wall,3.0000,180.0000,90.0000,0.7500,4,0.0000,0.0000,0.0000"));
        let debug = String::from_utf8(debug).unwrap();
        assert_eq!(debug.matches("  SURFACE3,SURFACE3,0.330000;\n").count(), 2);
        assert!(
            !debug.contains("0.630000"),
            "Native original+final IDF both use actual final F"
        );
        let radiant_row = RadiantRadiationEnclosure {
            name: "ENCLOSURE".into(),
            num_surfaces: 3,
            source_surface_ptr: Some(vec![1, 2, 3]),
            source_surface_report_nums: vec![3, 1, 2],
            area: Some(vec![1., 2., 3.]),
            azimuth: Some(vec![0., 90., 180.]),
            tilt: Some(vec![90.; 3]),
            emissivity: Some(vec![0.6, 0.7, 0.8]),
            f: Some(f),
            script_f: Some(approximate),
            ..Default::default()
        };
        let mut eio = Vec::new();
        let mut debug = Vec::new();
        radiant(
            &radiant_row,
            &geometry,
            "IDF",
            &mut eio,
            &mut Some(&mut debug),
        )?;
        let text = String::from_utf8(eio).unwrap();
        assert!(
            !text.contains("Approximate"),
            "genuine copied-Solar branch omits approximate radiant section"
        );
        assert!(text.contains("Script F Factors,X Surface,SURFACE3,SURFACE1,SURFACE2\nScript F Factor,SURFACE3,0.6300,0.6100,0.6200\n"));
        let debug = String::from_utf8(debug).unwrap();
        assert_eq!(debug.matches("  SURFACE3,SURFACE3,0.330000;\n").count(), 1);
        assert!(debug.contains("  SURFACE2,SURFACE2,0.220000;\n!============= end of data"));
        let mut eio = Vec::new();
        radiant(&radiant_row, &geometry, "", &mut eio, &mut None)?;
        assert!(
            String::from_utf8(eio)
                .unwrap()
                .contains("Script F Factor,SURFACE3")
        );
        Ok(())
    }
    #[test]
    fn genuine_report_matrix_keeps_source_indices_column_sum_and_r_precision() -> Result<(), Error>
    {
        // Unit data exercise the pure boundary; not an active Source owner claim.
        let geometry = SurfaceGeometryInputState::default();
        let view = View {
            name: "unit",
            count: 3,
            pointers: &[],
            order: &[3, 1, 2],
            area: &[],
            azimuth: &[],
            tilt: &[],
            absorptance_or_emissivity: &[],
            f: &[1., 2., 3., 4., 5., 6., 7., 8., 9.],
            geometry: &geometry,
        };
        assert_eq!(view.matrix(view.f, 3, 1)?, 7.);
        assert_eq!(view.column_sum(view.f, 1)?, 12.);
        assert_eq!(view.column_sum(view.f, 3)?, 18.);
        assert_eq!(round(0.0, 4)?, "0.0000");
        assert_eq!(round(-0.0, 6)?, "0.000000");
        assert_eq!(round(0.5, 4)?, "0.5000");
        Ok(())
    }
    #[test]
    fn genuine_report_header_failure_retains_preceding_lines() {
        struct Stop {
            bytes: Vec<u8>,
            writes: usize,
        }
        impl Write for Stop {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                if self.writes == 2 {
                    return Err(std::io::Error::other("actual unit write stop"));
                }
                self.writes += 1;
                self.bytes.extend_from_slice(bytes);
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut output = Stop {
            bytes: Vec::new(),
            writes: 0,
        };
        assert!(header(true, &mut output).is_err());
        assert_eq!(output.bytes,b"! <Solar View Factor Information>\n! <Solar View Factor - Zone/Enclosure Information>,Zone/Enclosure Name,Number of Surfaces\n");
        let mut radiant = Vec::new();
        header(false, &mut radiant).unwrap();
        assert_eq!(radiant.iter().filter(|&&c| c == b'\n').count(), 5);
    }
}
