//! Genuine HB2627 WindowManager opaque/no-window call and its real descendants.
//! Positive custom spectrum/glazing/frame/blind/screen families stop at their
//! first actual unsupported operation after preserving all preceding writes.
use super::error_backend::SourceErrorBackend;
use super::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays;
use super::report_scan::{
    ReportScanDiagnostic, ReportScanFailure, ReportScanState, SourceReportScanInput,
    try_scan_for_reports_with_input,
};
use super::selected_heat_balance_input::surface::{SurfaceClass, SurfaceGeometryInputState};
use super::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInputState, SelectedWindowThermalModel,
};
use super::selected_input_processor::{SelectedInputProcessor, SourceInputProcessorShortCut};
use super::selected_material_data::SelectedMaterialData;
use super::selected_simulation_startup::SelectedConstructionBounds;
use ep_model::ReportInputs;
mod glass;
mod w5;

/// Genuine selected WindowManagerData constructor flags524-530, not readiness.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceWindowOpticalInitialization {
    pub(crate) run_me_once_flag: bool,
    pub(crate) do_report: bool,
    pub(crate) has_windows: bool,
    pub(crate) has_complex_windows: bool,
    pub(crate) has_eql_windows: bool,
}
/// Temporary live borrows; no copied Source scalar/array or second IP is stored.
pub(crate) struct SourceWindowOpticalOwners<'a, 'b> {
    pub(crate) arrays: &'a mut SourceSurfaceHeatBalanceArrays,
    pub(crate) geometry: &'a mut SurfaceGeometryInputState,
    pub(crate) control: &'a mut SelectedHeatBalanceInputState,
    pub(crate) materials: &'a SelectedMaterialData,
    pub(crate) bounds: &'a SelectedConstructionBounds,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) report_inputs: &'a ReportInputs,
    pub(crate) scan: &'a mut ReportScanState,
    pub(crate) backend: &'a mut SourceErrorBackend<'b>,
}
impl SourceWindowOpticalInitialization {
    pub(crate) fn initialize_source_window_optical_calculations(
        &mut self,
        owners: &mut SourceWindowOpticalOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.check_and_read_custom_spectrum(owners)?; //141
        let total = usize::try_from(owners.geometry.total_surfaces)
            .map_err(|_| Error::Unavailable("Window144 actual TotSurfaces"))?;
        owners.arrays.surf_win_coeff_adj_ratio = Some(vec![1.0; total]); //144
        // Same actual CWindowModel factory result already written by project control.
        match owners.control.window_thermal_model {
            Some(SelectedWindowThermalModel::BuiltIn) => {
                self.init_glass_optical_calculations(owners)
            }
            None => Err(Error::Unavailable("Window146 actual CWindowModel pointer")),
        }
    }
    fn check_and_read_custom_spectrum(
        &mut self,
        owners: &mut SourceWindowOpticalOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.run_me_once_flag {
            return Ok(());
        } //8379
        // Local cCurrentModuleObject: do not mutate shared IP shortcut module.
        let count = owners
            .input
            .get_num_objects_found("Site:SolarAndVisibleSpectrum"); //8385
        if count == 0 {
            self.run_me_once_flag = true; //8389
            return Ok(());
        }
        if count > 1 {
            owners
                .backend
                .show_severe_error("Only one Site:SolarAndVisibleSpectrum object is allowed")?; //8395
        }
        Err(Error::Unavailable(
            "CheckAndReadCustomSpectrum positive getObjectDefMaxArgs8399",
        ))
    }
    fn report_glass(
        &mut self,
        owners: &mut SourceWindowOpticalOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut source =
            SourceReportScanInput::new(owners.input, owners.shortcut, owners.report_inputs)
                .map_err(|_| Error::Unavailable("ReportGlass6718 actual shared report IP"))?;
        self.do_report = try_scan_for_reports_with_input(
            owners.report_inputs,
            owners.scan,
            "Constructions",
            Some("Constructions"),
            None,
            None,
            &mut source,
            &mut |diagnostic| match diagnostic {
                ReportScanDiagnostic::Warning(text) => owners.backend.show_warning_error(&text),
                ReportScanDiagnostic::Continue(text) => owners.backend.show_continue_error(&text),
            },
        )
        .map_err(|error| match error {
            ReportScanFailure::Unavailable(_) => {
                Error::Unavailable("ReportGlass6718 actual report projection")
            }
            ReportScanFailure::Diagnostic(error) => Error::Backend(error),
        })?;
        if owners
            .control
            .construction_usage
            .iter()
            .any(|row| row.type_is_window)
        {
            self.has_windows = true;
        }
        if owners
            .control
            .construction_usage
            .iter()
            .any(|row| row.window_type_bsdf)
        {
            self.has_complex_windows = true;
        }
        if owners
            .control
            .construction_usage
            .iter()
            .any(|row| row.window_type_eql)
        {
            self.has_eql_windows = true;
        }
        if self.do_report && (self.has_windows || self.has_complex_windows || self.has_eql_windows)
        {
            return Err(Error::Unavailable(
                "ReportGlass positive WindowConstruction EIO header6738",
            ));
        } else if self.has_windows {
            for number in 0..owners.control.total_constructions {
                let row =
                    owners
                        .control
                        .construction_usage
                        .get(number)
                        .ok_or(Error::Unavailable(
                            "ReportGlass actual Construct ordinal7187",
                        ))?;
                if !row.type_is_window {
                    continue;
                }
                if row.window_type_eql {
                    continue;
                }
                return Err(Error::Unavailable(
                    "ReportGlass positive CalcNominalWindowCond7199",
                ));
            }
        }
        Ok(())
    }
}
fn source_surface_index(number: i32) -> Result<usize, Error> {
    usize::try_from(
        number
            .checked_sub(1)
            .ok_or(Error::Unavailable("Window actual positive ordinal"))?,
    )
    .map_err(|_| Error::Unavailable("Window actual positive ordinal"))
}
