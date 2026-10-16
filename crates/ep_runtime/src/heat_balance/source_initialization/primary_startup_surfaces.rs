//! Actual OutputReports::ReportSurfaces on the retained report cache and geometry.
//! Optional unported exporters stop only at their reached native child call.
mod wireframe;
use super::SourceInitializationFiles;
use crate::RuntimeError;
use crate::heat_balance::ctf_init_postlude::CtfInitPostludeError;
use crate::heat_balance::ctf_initialization_context::{
    CtfInitializationError, CtfInitializationReports, hb_input_error,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_daylighting::SourceDaylighting;
use crate::heat_balance::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, SourceReportScanInput, try_scan_for_reports,
    try_scan_for_reports_with_input,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use ep_model::ReportInputs;

// DataSurfaceColors.hh65,100: this is the actual module constructor array.
const DEFAULT_COLORS: [i32; 15] = [3, 43, 143, 143, 45, 8, 15, 195, 9, 13, 174, 143, 143, 10, 5];
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSurfaceColors {
    pub(crate) dxf_color_numbers: [i32; 15],
}
impl Default for SourceSurfaceColors {
    fn default() -> Self {
        Self {
            dxf_color_numbers: DEFAULT_COLORS,
        }
    }
}
fn unavailable(reason: &'static str) -> RuntimeError {
    hb_input_error(Error::Unavailable(reason))
}
impl CtfInitializationReports {
    fn scan_source_surface_report(
        &mut self,
        input: &ReportInputs,
        key: &str,
        option1: Option<&mut String>,
        option2: Option<&mut String>,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<bool, RuntimeError> {
        let mut diagnostic = |message| match message {
            ReportScanDiagnostic::Warning(text) => backend.show_warning_error(&text),
            ReportScanDiagnostic::Continue(text) => backend.show_continue_error(&text),
        };
        // Source already-scanned calls do not replay first-cache input validation.
        let result = if self.scan.get_report_input {
            let input_processor = self
                .selected_input_processor
                .as_mut()
                .ok_or_else(|| unavailable("actual ReportSurfaces shared InputProcessor"))?;
            let mut source = SourceReportScanInput::new(
                input_processor,
                &mut self.input_processor_shortcut,
                input,
            )
            .map_err(|value| {
                RuntimeError::from(CtfInitializationError::Postlude(
                    CtfInitPostludeError::Scan(value),
                ))
            })?;
            try_scan_for_reports_with_input(
                input,
                &mut self.scan,
                "Surfaces",
                Some(key),
                option1,
                option2,
                &mut source,
                &mut diagnostic,
            )
        } else {
            try_scan_for_reports(
                input,
                &mut self.scan,
                "Surfaces",
                Some(key),
                option1,
                option2,
                &mut diagnostic,
            )
        };
        result.map_err(|error| {
            CtfInitializationError::Postlude(match error {
                ReportScanFailure::Unavailable(value) => CtfInitPostludeError::Scan(value),
                ReportScanFailure::Diagnostic(value) => CtfInitPostludeError::Backend(value),
            })
            .into()
        })
    }
    /// Actual SimulationManager289 child, using the sole shared report/IP owners.
    /// The drawing stream is freshly opened only inside its requested exporter.
    pub(crate) fn report_source_surfaces(
        &mut self,
        input: &ReportInputs,
        files: &SourceInitializationFiles,
        daylighting: &SourceDaylighting,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), RuntimeError> {
        backend.state.tracking.ask_for_surfaces_report = false;
        let mut surface_details = 0;
        let mut surface_vertices = false;
        let mut surface_detail = false;
        // Native true write143 is after the currently unsupported solid-DXF child.
        let dxf_done = false;
        let mut option1 = String::new();
        let mut option2 = String::new();
        if self.scan_source_surface_report(input, "Lines", Some(&mut option1), None, backend)? {
            return Err(unavailable(
                "actual OutputReports105 LinesOut183 optional exporter",
            ));
        }
        if self.scan_source_surface_report(input, "Vertices", None, None, backend)?
            && !surface_vertices
        {
            surface_details += 1;
            surface_vertices = true;
        }
        if self.scan_source_surface_report(input, "Details", None, None, backend)?
            && !surface_detail
        {
            surface_details += 10;
            surface_detail = true;
        }
        if self.scan_source_surface_report(input, "DetailsWithVertices", None, None, backend)? {
            if !surface_detail {
                surface_details += 10;
                surface_detail = true;
            }
            if !surface_vertices {
                surface_details += 1;
                surface_vertices = true;
            }
        }
        if self.scan_source_surface_report(
            input,
            "DXF",
            Some(&mut option1),
            Some(&mut option2),
            backend,
        )? {
            if !dxf_done {
                if !option2.is_empty() {
                    return Err(unavailable(
                        "actual ReportSurfaces140 SetUpSchemeColors optional child",
                    ));
                }
                return Err(unavailable(
                    "actual ReportSurfaces142 solid DXFOut463 child",
                ));
            } else {
                backend.show_warning_error(&format!("ReportSurfaces: DXF output already generated.  DXF with option=[{option1}] will not be generated."))?;
            }
        }
        if self.scan_source_surface_report(
            input,
            "DXF:WireFrame",
            Some(&mut option1),
            Some(&mut option2),
            backend,
        )? {
            if !dxf_done {
                if !option2.is_empty() {
                    return Err(unavailable(
                        "actual ReportSurfaces154 SetUpSchemeColors optional child",
                    ));
                }
                wireframe::write_source_dxf_wireframe(self, files, daylighting, &option2, backend)
                    .map_err(hb_input_error)?;
                // Native wireframe branch does not assign DXFDone.
            } else {
                backend.show_warning_error("ReportSurfaces: DXF output already generated.  DXF:WireFrame will not be generated.")?;
            }
        }
        if self.scan_source_surface_report(
            input,
            "VRML",
            Some(&mut option1),
            Some(&mut option2),
            backend,
        )? {
            return Err(unavailable(
                "actual ReportSurfaces166 VRMLOut1502 child (local VRMLDone is false)",
            ));
        }
        if self.scan_source_surface_report(input, "CostInfo", None, None, backend)? {
            return Err(unavailable(
                "actual ReportSurfaces175 CostInfoOut1432 child",
            ));
        }
        if surface_detail || surface_vertices {
            let _actual_surface_details = surface_details;
            return Err(unavailable(
                "actual ReportSurfaces179 DetailsForSurfaces1034 child",
            ));
        }
        Ok(())
    }
}
