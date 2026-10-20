//! Actual OutputReports::LinesOut183-283 on the retained final Surface array.
use super::{
    SourceInitializationFiles, surface_report_class, surface_report_round, surface_report_row,
    surface_report_slot,
};
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::heat_balance::ctf_reports::formatting::fixed;
use crate::heat_balance::error_backend::{
    SourceDiagnosticFlags, SourceErrorBackend, SourceFatalStop,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_heat_balance_input::surface::{SurfaceClass, SurfaceTmpInput};
use std::io::{self, Write};

#[derive(Debug)]
struct LinesOpenFailure {
    source: io::Error,
    stop: SourceFatalStop,
}
impl std::fmt::Display for LinesOpenFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.stop, self.source)
    }
}
impl std::error::Error for LinesOpenFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}
fn flags<'a>(
    reports: &'a CtfInitializationReports,
    warmup: &'a bool,
) -> Result<SourceDiagnosticFlags<'a>, Error> {
    let globals = reports
        .selected_heat_balance_globals
        .as_ref()
        .ok_or(Error::Unavailable("actual LinesOut diagnostic DataGlobal"))?;
    let project = reports
        .selected_project_data
        .as_ref()
        .ok_or(Error::Unavailable("actual LinesOut diagnostic SysVars"))?;
    Ok(SourceDiagnosticFlags {
        kickoff: &globals.kickoff_simulation,
        developer: &project.system.developer_flag,
        warmup,
        doing_sizing: &globals.doing_sizing,
    })
}
fn vertex(row: &SurfaceTmpInput, number: i32) -> Result<&[f64; 3], Error> {
    row.vertex
        .as_ref()
        .and_then(|rows| {
            surface_report_slot(number)
                .ok()
                .and_then(|slot| rows.get(slot))
        })
        .and_then(Option::as_ref)
        .ok_or(Error::Unavailable(
            "actual LinesOut initialized Surface.Vertex",
        ))
}

/// Direct stream writes preserve the literal closing edge and completed prefix.
pub(crate) fn write_lines_body(
    reports: &CtfInitializationReports,
    option: &str,
    output: &mut dyn Write,
) -> Result<(), Error> {
    let idf = option == "IDF";
    if idf {
        output.write_all(b" Building North Axis = 0\nGlobalGeometryRules,UpperLeftCorner,CounterClockwise,WorldCoordinates;\n")?;
    }
    for &number in &reports.surface_geometry_input.all_surface_list_report_order {
        let row = surface_report_row(reports, number)?;
        if row.class == SurfaceClass::IntMass || row.sides == 0 {
            continue;
        }
        if idf {
            let class = surface_report_class(row)?;
            let properties = row
                .vertex_progress
                .properties
                .as_ref()
                .ok_or(Error::Unavailable(
                    "actual LinesOut written Surface.Azimuth",
                ))?;
            writeln!(
                output,
                " Surface={}, Name={}, Azimuth={}",
                class,
                row.name,
                surface_report_round(properties.azimuth_deg, 1)?
            )?;
            writeln!(
                output,
                "  {},  !- Number of (X,Y,Z) groups in this surface",
                row.sides
            )?;
            for v in 1..=row.sides {
                let p = vertex(row, v)?;
                let terminator = if v == row.sides { ";" } else { "," };
                writeln!(
                    output,
                    "  {},{},{}{}  !- X,Y,Z ==> Vertex {}",
                    fixed(p[0], 10, 2),
                    fixed(p[1], 10, 2),
                    fixed(p[2], 10, 2),
                    terminator,
                    v
                )?;
            }
        } else {
            writeln!(output, "{}:{}", row.zone_name, row.name)?;
            for v in 1..=row.sides {
                let p = vertex(row, v)?;
                let q = vertex(row, if v == row.sides { 1 } else { v + 1 })?;
                writeln!(
                    output,
                    "{},{},{},{},{},{}",
                    fixed(p[0], 10, 2),
                    fixed(p[1], 10, 2),
                    fixed(p[2], 10, 2),
                    fixed(q[0], 10, 2),
                    fixed(q[1], 10, 2),
                    fixed(q[2], 10, 2)
                )?;
            }
        }
    }
    Ok(())
}

pub(super) fn write_source_lines(
    reports: &mut CtfInitializationReports,
    files: &SourceInitializationFiles,
    option: &str,
    warmup: &bool,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let geometry = &reports.surface_geometry_input;
    if geometry.total_surfaces > 0 && geometry.final_surfaces.is_none() {
        return Ok(());
    }
    if reports.source_output_reports.option_done {
        backend.show_source_warning_error(
            &format!(
                "Report of Surfaces/Lines Option has already been completed with option={}",
                reports.source_output_reports.last_option
            ),
            &flags(reports, warmup)?,
        )?;
        backend.show_continue_error(&format!(
            "..option=\"{}\" will not be done this time.",
            option
        ))?;
        return Ok(());
    }
    // These genuine module writes precede open and survive every later failure.
    reports.source_output_reports.last_option = option.to_owned();
    reports.source_output_reports.option_done = true;
    let mut output = match files.open_source_lines(reports.source_output_control.sln) {
        Ok(file) => file,
        Err(source) => {
            let message = format!(
                "LinesOut: Could not open file {} for output (write).",
                files.source_lines_path().display()
            );
            return match backend.show_source_fatal_error(&message, &flags(reports, warmup)?) {
                Ok(never) => match never {},
                Err(Error::Fatal(stop)) => {
                    let kind = source.kind();
                    Err(Error::Io(io::Error::new(
                        kind,
                        LinesOpenFailure { source, stop },
                    )))
                }
                Err(error) => Err(error),
            };
        }
    };
    write_lines_body(reports, option, &mut output)
    // Actual local InputOutputFile destructor closes here; close errors are not diagnosed.
}
