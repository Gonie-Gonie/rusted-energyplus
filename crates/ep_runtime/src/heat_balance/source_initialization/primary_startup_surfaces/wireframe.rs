//! Actual wireframe prints on final Source Surface order and coordinates.
mod common;
use super::{SourceInitializationFiles, SourceSurfaceColors};
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::heat_balance::ctf_reports::formatting::fixed;
use crate::heat_balance::error_backend::{SourceErrorBackend, SourceFatalStop};
use crate::heat_balance::ordinary_daylighting::SourceDaylighting;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_heat_balance_input::surface::{SurfaceClass, SurfaceTmpInput};
use std::io::{self, Write};

#[derive(Debug)]
struct DxfOpenFailure {
    source: io::Error,
    stop: SourceFatalStop,
}
impl std::fmt::Display for DxfOpenFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.stop, self.source)
    }
}
impl std::error::Error for DxfOpenFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
fn ordinal(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual DXF positive Source ordinal"))
}
fn surface(reports: &CtfInitializationReports, number: i32) -> Result<&SurfaceTmpInput, Error> {
    reports
        .surface_geometry_input
        .final_surfaces
        .as_ref()
        .and_then(|rows| ordinal(number).ok().and_then(|i| rows.get(i)))
        .map(|row| &row.row)
        .ok_or(Error::Unavailable("actual DXF final Source Surface array"))
}
fn vertex(row: &SurfaceTmpInput, number: i32) -> Result<&[f64; 3], Error> {
    row.vertex
        .as_ref()
        .and_then(|rows| ordinal(number).ok().and_then(|i| rows.get(i)))
        .and_then(Option::as_ref)
        .ok_or(Error::Unavailable(
            "actual DXF initialized Source Surface.Vertex",
        ))
}
fn pv(reports: &CtfInitializationReports, number: i32) -> Result<bool, Error> {
    reports
        .surface_geometry_input
        .parallel_arrays
        .as_ref()
        .and_then(|rows| ordinal(number).ok().and_then(|i| rows.surf_is_pv.get(i)))
        .copied()
        .ok_or(Error::Unavailable("actual DXF Source SurfIsPV array"))
}
fn zone_count(reports: &CtfInitializationReports) -> Result<i32, Error> {
    reports
        .selected_heat_balance_globals
        .as_ref()
        .map(|globals| globals.num_of_zones)
        .ok_or(Error::Unavailable("actual DXF NumOfZones"))
}
fn zone_name(reports: &CtfInitializationReports, number: i32) -> Result<&str, Error> {
    reports
        .selected_zone_input
        .as_ref()
        .and_then(|owner| owner.zones.as_ref())
        .and_then(|rows| ordinal(number).ok().and_then(|i| rows.get(i)))
        .map(|row| row.name.as_str())
        .ok_or(Error::Unavailable("actual DXF Zone name"))
}
// EnergyPlus.hh imports ObjexxFCL::min, whose non-template double overload wins.
fn source_min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}
fn normalize_name(name: &str) -> String {
    name.replace(' ', "_").replace(':', "_")
}
fn color(colors: &SourceSurfaceColors, index: Option<usize>) -> Result<i32, Error> {
    index
        .and_then(|i| colors.dxf_color_numbers.get(i))
        .copied()
        .ok_or(Error::Unavailable(
            "actual DXF undefined Invalid color array index",
        ))
}
fn polyline(of: &mut dyn Write, layer: &str, color: i32, minz: f64) -> Result<(), Error> {
    write!(
        of,
        "  0\nPOLYLINE\n  8\n{layer}\n 62\n{color:3}\n 66\n  1\n 10\n 0.0\n 20\n 0.0\n 30\n{}\n 70\n   9\n 40\n 0.55\n 41\n 0.55\n",
        fixed(minz, 15, 5)
    )?;
    Ok(())
}
fn vertices(of: &mut dyn Write, layer: &str, row: &SurfaceTmpInput) -> Result<(), Error> {
    for number in 1..=row.sides {
        let point = vertex(row, number)?;
        write!(
            of,
            "  0\nVERTEX\n  8\n{layer}\n 10\n{}\n 20\n{}\n 30\n{}\n",
            fixed(point[0], 15, 5),
            fixed(point[1], 15, 5),
            fixed(point[2], 15, 5)
        )?;
    }
    write!(of, "  0\nSEQEND\n  8\n{layer}\n")?;
    Ok(())
}
pub(super) fn write_source_dxf_wireframe(
    reports: &CtfInitializationReports,
    files: &SourceInitializationFiles,
    daylighting: &SourceDaylighting,
    color_scheme: &str,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let geometry = &reports.surface_geometry_input;
    if geometry.total_surfaces > 0 && geometry.final_surfaces.is_none() {
        return Ok(());
    }
    if !reports.source_output_control.dxf {
        return Err(Error::Unavailable(
            "actual IOFiles285 DXF output_to_file=false dev-null stream",
        ));
    }
    let mut file = match files.open_source_dxf() {
        Ok(file) => file,
        Err(source) => {
            let message = format!(
                "DXFOutWireFrame: Could not open file {} for output (write).",
                files.source_dxf_path().display()
            );
            return match backend.show_fatal_error(&message) {
                Ok(never) => match never {},
                Err(stop) => {
                    let kind = source.kind();
                    Err(Error::Io(io::Error::new(
                        kind,
                        DxfOpenFailure { source, stop },
                    )))
                }
            };
        }
    };
    file.write_all(b"  0\nSECTION\n  2\nENTITIES\n")?;
    file.write_all(b"999\nDXF created from EnergyPlus\n")?;
    write!(
        file,
        "999\nProgram Version,{}\n",
        backend.state.version_string
    )?;
    file.write_all(b"999\nDXF using Wireframe  \n")?;
    common::write_common(reports, &mut file, color_scheme)?;
    let mut surface_count = 0;
    let mut color_index = None;
    for &number in &geometry.all_surface_list_report_order {
        let mut shade_type = String::new();
        let row = surface(reports, number)?;
        if row.heat_transfer_surface {
            continue;
        }
        if row.class == SurfaceClass::Shading {
            continue;
        }
        if row.class == SurfaceClass::DetachedFixed {
            color_index = Some(8);
        }
        if row.class == SurfaceClass::DetachedBuilding {
            color_index = Some(7);
        }
        if pv(reports, number)? {
            color_index = Some(10);
        }
        if row.class == SurfaceClass::DetachedFixed {
            shade_type = "Fixed Shading".into();
            write!(file, "999\nFixed Shading:{}\n", row.name)?;
        } else if row.class == SurfaceClass::DetachedBuilding {
            shade_type = "Building Shading".into();
            write!(file, "999\nBuilding Shading:{}\n", row.name)?;
        }
        surface_count += 1;
        shade_type.push_str(&format!("_{surface_count}"));
        let mut minz = 99999.0;
        for v in 1..=row.sides {
            minz = source_min(minz, vertex(row, v)?[2]);
        }
        polyline(
            &mut file,
            &shade_type,
            color(&reports.source_surface_colors, color_index)?,
            minz,
        )?;
        vertices(&mut file, &shade_type, row)?;
    }
    for zone in 1..=zone_count(reports)? {
        let saved_zone_name = normalize_name(zone_name(reports, zone)?);
        surface_count = 0;
        for &number in &geometry.all_surface_list_report_order {
            let final_row = geometry
                .final_surfaces
                .as_ref()
                .and_then(|rows| ordinal(number).ok().and_then(|i| rows.get(i)))
                .ok_or(Error::Unavailable("actual DXF final Source Surface"))?;
            let row = &final_row.row;
            if row.zone != zone {
                continue;
            }
            if row.class == SurfaceClass::IntMass {
                continue;
            }
            if row.class == SurfaceClass::Wall {
                color_index = Some(1);
            }
            if row.class == SurfaceClass::Roof {
                color_index = Some(6);
            }
            if row.class == SurfaceClass::Floor {
                color_index = Some(5);
            }
            if row.class == SurfaceClass::Door {
                color_index = Some(4);
            }
            if row.class == SurfaceClass::Window {
                if final_row.original_class == SurfaceClass::Window {
                    color_index = Some(2);
                }
                if final_row.original_class == SurfaceClass::GlassDoor {
                    color_index = Some(3);
                }
                if final_row.original_class == SurfaceClass::TddDome {
                    color_index = Some(11);
                }
                if final_row.original_class == SurfaceClass::TddDiffuser {
                    color_index = Some(12);
                }
            }
            if pv(reports, number)? {
                color_index = Some(10);
            }
            surface_count += 1;
            write!(file, "999\n{}:{}\n", row.zone_name, row.name)?;
            let layer = format!("{saved_zone_name}_{surface_count}");
            let mut minz = 99999.0;
            for v in 1..=row.sides {
                minz = source_min(minz, vertex(row, v)?[2]);
            }
            polyline(
                &mut file,
                &layer,
                color(&reports.source_surface_colors, color_index)?,
                minz,
            )?;
            vertices(&mut file, &layer, row)?;
        }
        surface_count = 0;
        for &number in &geometry.all_surface_list_report_order {
            let row = surface(reports, number)?;
            if row.class != SurfaceClass::Shading {
                continue;
            }
            if row.zone_name != zone_name(reports, zone)? {
                continue;
            }
            color_index = Some(9);
            if pv(reports, number)? {
                color_index = Some(10);
            }
            surface_count += 1;
            write!(file, "999\n{}:{}\n", row.zone_name, row.name)?;
            let layer = format!("{saved_zone_name}_{surface_count}");
            let mut minz = 99999.0;
            for v in 1..=row.sides {
                minz = source_min(minz, vertex(row, v)?[2]);
            }
            polyline(
                &mut file,
                &layer,
                color(&reports.source_surface_colors, color_index)?,
                minz,
            )?;
            vertices(&mut file, &layer, row)?;
        }
    }
    if daylighting.source_daylight_reference_point_count() > 0 {
        return Err(Error::Unavailable(
            "actual DXFDaylightingReferencePoints437 positive daylightControl traversal",
        ));
    }
    file.write_all(b"  0\nENDSEC\n  0\nEOF\n")?;
    // Native local InputOutputFile destruction closes this same reached File.
    Ok(())
}
