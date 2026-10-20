//! Actual OutputReports::CostInfoOut1432-1500 on existing Source owners.
use super::{
    SourceInitializationFiles, surface_report_class, surface_report_row, surface_report_slot,
};
use crate::heat_balance::ctf_initialization_context::CtfInitializationReports;
use crate::heat_balance::ctf_reports::formatting::fixed;
use crate::heat_balance::error_backend::{
    SourceDiagnosticFlags, SourceErrorBackend, SourceFatalStop,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use std::io::{self, Write};

#[derive(Debug)]
struct CostInfoOpenFailure {
    source: io::Error,
    stop: SourceFatalStop,
}
impl std::fmt::Display for CostInfoOpenFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.stop, self.source)
    }
}
impl std::error::Error for CostInfoOpenFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.source)
    }
}

/// None is the exact Native unallocated Surface early return, before file open.
pub(crate) fn unique_cost_surfaces(
    reports: &CtfInitializationReports,
) -> Result<Option<Vec<bool>>, Error> {
    let geometry = &reports.surface_geometry_input;
    if geometry.total_surfaces > 0 && geometry.final_surfaces.is_none() {
        return Ok(None);
    }
    let count = usize::try_from(geometry.total_surfaces)
        .map_err(|_| Error::Unavailable("actual CostInfoOut TotSurfaces dimension"))?;
    let mut unique = vec![true; count];
    for &number in &geometry.all_surface_list_report_order {
        let row = surface_report_row(reports, number)?;
        let slot = surface_report_slot(number)?;
        let keep = unique
            .get_mut(slot)
            .ok_or(Error::Unavailable("actual CostInfoOut uniqueSurf index"))?;
        if row.exterior_boundary > 0 && row.exterior_boundary < number {
            *keep = false;
        }
        if row.construction == 0 {
            *keep = false;
        }
    }
    Ok(Some(unique))
}

/// Prints directly to the real fresh stream; preceding rows survive IO errors.
pub(crate) fn write_cost_info_body(
    reports: &CtfInitializationReports,
    unique: &[bool],
    output: &mut dyn Write,
) -> Result<(), Error> {
    writeln!(
        output,
        "{:12}{:12}",
        reports.surface_geometry_input.total_surfaces,
        unique.iter().filter(|&&value| value).count()
    )?;
    output.write_all(b" data for surfaces useful for cost information\n")?;
    output.write_all(b" Number, Name, Construction, class, area, grossarea\n")?;
    for &number in &reports.surface_geometry_input.all_surface_list_report_order {
        let slot = surface_report_slot(number)?;
        if !*unique
            .get(slot)
            .ok_or(Error::Unavailable("actual CostInfoOut uniqueSurf row"))?
        {
            continue;
        }
        let row = surface_report_row(reports, number)?;
        if row.construction != 0 {
            let construction = reports
                .heat_balance_input_state
                .construction_usage
                .get(surface_report_slot(row.construction)?)
                .ok_or(Error::Unavailable("actual CostInfoOut Construct.Name"))?;
            writeln!(
                output,
                "{:5},{},{},{},{},{}",
                number,
                row.name,
                construction.name,
                surface_report_class(row)?,
                fixed(row.area, 14, 5),
                fixed(row.gross_area, 14, 5)
            )?;
        }
    }
    Ok(())
}

pub(super) fn write_source_cost_info(
    reports: &CtfInitializationReports,
    files: &SourceInitializationFiles,
    warmup: &bool,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    // Native dimensions/marks uniqueSurf before opening its local IOFilePath.
    let Some(unique) = unique_cost_surfaces(reports)? else {
        return Ok(());
    };
    let mut output = match files.open_source_cost_info(reports.source_output_control.sci) {
        Ok(file) => file,
        Err(source) => {
            let message = format!(
                "CostInfoOut: Could not open file {} for output (write).",
                files.source_cost_info_path().display()
            );
            // Read the same actual phase cells at the reached Fatal call.
            // No legacy backend ctor flag is synchronized or used as a proxy.
            let globals = reports
                .selected_heat_balance_globals
                .as_ref()
                .ok_or(Error::Unavailable("actual CostInfoOut Fatal DataGlobal"))?;
            let project = reports
                .selected_project_data
                .as_ref()
                .ok_or(Error::Unavailable("actual CostInfoOut Fatal SysVars"))?;
            let flags = SourceDiagnosticFlags {
                kickoff: &globals.kickoff_simulation,
                developer: &project.system.developer_flag,
                warmup,
                doing_sizing: &globals.doing_sizing,
            };
            return match backend.show_source_fatal_error(&message, &flags) {
                Ok(never) => match never {},
                Err(Error::Fatal(stop)) => {
                    let kind = source.kind();
                    Err(Error::Io(io::Error::new(
                        kind,
                        CostInfoOpenFailure { source, stop },
                    )))
                }
                // Preserve a genuine earlier callback/IO/unavailable failure;
                // it cannot be rewritten as a completed Fatal return.
                Err(error) => Err(error),
            };
        }
    };
    write_cost_info_body(reports, &unique, &mut output)
}

#[cfg(test)]
mod live_open_failure_tests {
    use super::*;
    use crate::heat_balance::error_backend::{SourceErrorState, SourceErrorTracking};
    use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
    use crate::heat_balance::selected_project_data::SelectedProjectData;
    use crate::heat_balance::source_initialization::SourceInitializationFilePaths;
    #[test]
    fn source_cost_info_fatal_borrows_real_phase_instead_of_backend_ctor_and_preserves_summary() {
        // Unit constructors only, not an active Source simulation claim.
        let mut reports = CtfInitializationReports {
            selected_heat_balance_globals: Some(SelectedHeatBalanceGlobals {
                kickoff_simulation: true,
                ..Default::default()
            }),
            selected_project_data: Some(SelectedProjectData::default()),
            ..Default::default()
        };
        let path = std::path::PathBuf::new(); // invalid actual output filename
        let files = SourceInitializationFiles::new(SourceInitializationFilePaths {
            stat_input: path.clone(),
            eso: path.clone(),
            eio: path.clone(),
            mtr: path.clone(),
            bnd: path.clone(),
            dxf: path.clone(),
            sln: path.clone(),
            sci: path.clone(),
            rdd: path.clone(),
            mdd: path.clone(),
            audit: path.clone(),
            debug: path.clone(),
            end: path.clone(),
            error: path.clone(),
            dfs: path.clone(),
            tabular_csv: path.clone(),
            tabular_tab: path.clone(),
            tabular_fixed: path.clone(),
            tabular_html: path.clone(),
            tabular_xml: path,
        });
        let mut state = SourceErrorState {
            output_error_header: false,
            version_string: "unit version".into(),
            idd_version_string: "unit IDD".into(),
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
                total_severe_errors: 2,
                total_severe_errors_during_warmup: 1,
                total_severe_errors_during_sizing: 0,
                last_severe_error: "actual prior unit severe".into(),
                total_warning_errors: 0,
                total_warning_errors_during_warmup: 0,
                total_warning_errors_during_sizing: 0,
            },
        };
        let mut err = Vec::new();
        let mut console = Vec::new();
        let live_warmup = true;
        let result = {
            let mut backend = SourceErrorBackend {
                state: &mut state,
                error_stream: Some(&mut err),
                console: &mut console,
                out_unit1: None,
                out_unit2: None,
                f_message: None,
                message_callback: None,
                error_callback: None,
                sqlite: None,
            };
            let result = write_source_cost_info(&reports, &files, &live_warmup, &mut backend);
            let lines = super::super::lines::write_source_lines(
                &mut reports,
                &files,
                "IDF",
                &live_warmup,
                &mut backend,
            );
            assert!(matches!(lines, Err(Error::Io(_))));
            assert!(reports.source_output_reports.option_done);
            assert_eq!(reports.source_output_reports.last_option, "IDF");
            reports
                .selected_heat_balance_globals
                .as_mut()
                .unwrap()
                .kickoff_simulation = false;
            // The failed open already set the genuine once flag. A repeated
            // request warns (using live warmup=true), then returns before open.
            super::super::lines::write_source_lines(
                &mut reports,
                &files,
                "",
                &live_warmup,
                &mut backend,
            )
            .unwrap();
            assert_eq!(reports.source_output_reports.last_option, "IDF");
            assert_eq!(backend.state.tracking.total_warning_errors_during_warmup, 1);
            result
        };
        assert!(matches!(result, Err(Error::Io(_))));
        // Live kickoff=true/developer=false suppresses display. The stale
        // backend kickoff=false would incorrectly emit the Fatal to console.
        assert!(console.is_empty());
        let err = String::from_utf8(err).unwrap();
        assert!(
            err.contains(" **  Fatal  ** CostInfoOut: Could not open file  for output (write).")
        );
        assert!(err.contains(" **  Fatal  ** LinesOut: Could not open file  for output (write)."));
        assert!(err.contains(" ** Warning ** Report of Surfaces/Lines Option has already been completed with option=IDF"));
        assert!(err.contains(" **   ~~~   ** ..option=\"\" will not be done this time."));
        assert!(err.contains(" ..... Reference severe error count=2"));
        assert!(err.contains(" ..... Last severe error=actual prior unit severe"));
        assert_eq!(
            (
                state.tracking.total_severe_errors,
                state.tracking.total_severe_errors_during_warmup
            ),
            (2, 1)
        );
        assert!(!state.kickoff_simulation && !state.warmup_flag);
    }
}
