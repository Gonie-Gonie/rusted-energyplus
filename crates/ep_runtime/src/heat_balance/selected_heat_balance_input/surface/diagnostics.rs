//! Literal aggregate diagnostics at GetSurfaceData2795-2848.
//! Message-only helpers preserve the Source's separate bulk counter additions.
use super::{Error, SourceErrorBackend, SurfaceGeometryInputState};

fn add_count(total: &mut i32, count: i32) -> Result<(), Error> {
    *total = total.checked_add(count).ok_or(Error::Unavailable(
        "Source signed aggregate diagnostic count",
    ))?;
    Ok(())
}

impl SurfaceGeometryInputState {
    pub(crate) fn report_surface_input_diagnostics(
        &self,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.warning1_count > 0 {
            backend.show_warning_message(&format!(
                "GetSurfaceData: Window dimensions differ from Window 5/6 data file dimensions, {} times.",
                self.warning1_count,
            ))?;
            backend.show_continue_error("This will affect the frame heat transfer calculation if the frame in the Data File entry")?;
            backend.show_continue_error("is not uniform, i.e., has sections with different geometry and/or thermal properties.")?;
            backend.show_continue_error(
                "For explicit details on each window, use Output:Diagnostics,DisplayExtraWarnings;",
            )?;
        }
        if self.warning2_count > 0 {
            backend.show_warning_message(&format!(
                "GetSurfaceData: Exterior Windows have been replaced with Window 5/6 two glazing systems, {} times.",
                self.warning2_count,
            ))?;
            backend
                .show_continue_error("Note that originally entered dimensions are overridden.")?;
            backend.show_continue_error(
                "For explicit details on each window, use Output:Diagnostics,DisplayExtraWarnings;",
            )?;
        }
        if self.warning3_count > 0 {
            backend.show_warning_message(&format!(
                "GetSurfaceData: Interior Windows have been replaced with Window 5/6 two glazing systems, {} times.",
                self.warning3_count,
            ))?;
            backend
                .show_continue_error("Note that originally entered dimensions are overridden.")?;
            backend.show_continue_error(
                "For explicit details on each window, use Output:Diagnostics,DisplayExtraWarnings;",
            )?;
        }
        if backend.state.tracking.total_multiplied_windows > 0 {
            backend.show_warning_message(&format!(
                "GetSurfaceData: There are {} window/glass door(s) that may cause inaccurate shadowing due to Solar Distribution.",
                backend.state.tracking.total_multiplied_windows,
            ))?;
            backend.show_continue_error(
                "For explicit details on each window, use Output:Diagnostics,DisplayExtraWarnings;",
            )?;
            // The callback/SQLite/continue prefix above must succeed first.
            // Re-read the actual mutable owner, as the Original does at2828.
            let count = backend.state.tracking.total_multiplied_windows;
            add_count(&mut backend.state.tracking.total_warning_errors, count)?;
        }
        if backend.state.tracking.total_coincident_vertices > 0 {
            backend.show_warning_message(&format!(
                "GetSurfaceData: There are {} coincident/collinear vertices; These have been deleted unless the deletion would bring the number of surface sides < 3.",
                backend.state.tracking.total_coincident_vertices,
            ))?;
            backend.show_continue_error(
                "For explicit details on each problem surface, use Output:Diagnostics,DisplayExtraWarnings;",
            )?;
            let count = backend.state.tracking.total_coincident_vertices;
            add_count(&mut backend.state.tracking.total_warning_errors, count)?;
        }
        if backend.state.tracking.total_degenerate_surfaces > 0 {
            backend.show_severe_message(&format!(
                "GetSurfaceData: There are {} degenerate surfaces; Degenerate surfaces are those with number of sides < 3.",
                backend.state.tracking.total_degenerate_surfaces,
            ))?;
            backend.show_continue_error("These surfaces should be deleted.")?;
            backend.show_continue_error(
                "For explicit details on each problem surface, use Output:Diagnostics,DisplayExtraWarnings;",
            )?;
            let count = backend.state.tracking.total_degenerate_surfaces;
            add_count(&mut backend.state.tracking.total_severe_errors, count)?;
        }
        Ok(())
    }
}
