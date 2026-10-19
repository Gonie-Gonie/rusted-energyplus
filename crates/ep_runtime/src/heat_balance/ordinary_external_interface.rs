//! Genuine ExternalInterface first input and non-external exchange path.
use super::error_backend::SourceErrorBackend;
use super::selected_heat_balance_input::HeatBalanceInputError as Error;
use super::selected_input_processor::{SelectedInputProcessor, SourceInputProcessorShortCut};

/// Narrow actual ExternalInterfaceData owner: only fields reached by this
/// getter/zero-exchange control flow. FMU/socket storage and kernels are unported.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceExternalInterface {
    number_external_interfaces: i32,
    number_bcvtb: i32,
    number_fmu_import: i32,
    number_fmu_export: i32,
    number_fmu_objects: i32,
    fmu_export_activate: i32,
    have_bcvtb: bool,
    have_fmu_import: bool,
    have_fmu_export: bool,
    simulation_status: i32,
    socket_fd: i32,
    errors_found: bool,
    get_input: bool,
}
impl Default for SourceExternalInterface {
    fn default() -> Self {
        Self {
            number_external_interfaces: 0,
            number_bcvtb: 0,
            number_fmu_import: 0,
            number_fmu_export: 0,
            number_fmu_objects: 0,
            fmu_export_activate: 0,
            have_bcvtb: false,
            have_fmu_import: false,
            have_fmu_export: false,
            simulation_status: 1,
            socket_fd: -1,
            errors_found: false,
            get_input: true,
        }
    }
}

fn native_count(input: &SelectedInputProcessor, object_type: &str) -> Result<i32, Error> {
    i32::try_from(input.get_num_objects_found(object_type))
        .map_err(|_| Error::Unavailable("actual ExternalInterface object count outside Native int"))
}

fn fatal(backend: &mut SourceErrorBackend<'_>, message: &str) -> Result<(), Error> {
    match backend.show_fatal_error(message) {
        Ok(never) => match never {},
        Err(stop) => Err(Error::Fatal(stop)),
    }
}

fn warn_if_external_objects_are_used(
    input: &SelectedInputProcessor,
    object_word: &str,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), Error> {
    let number = native_count(input, object_word)?;
    if number > 0 {
        backend.show_warning_error(&format!("IDF file contains object \"{object_word}\","))?;
        backend.show_continue_error("but object \"ExternalInterface\" with appropriate key entry is not specified. Values will not be updated.")?;
    }
    Ok(())
}

impl SourceExternalInterface {
    /// Native98--139: the sole first-input flag clears only after the actual
    /// getter and its real warning/error children return successfully.
    pub(crate) fn exchange_source_variables(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        if self.get_input {
            self.get_source_input(input, shortcut, backend)?;
            self.get_input = false;
        }
        if self.have_bcvtb || self.have_fmu_export {
            return Err(Error::Unavailable("actual InitExternalInterface114"));
        }
        if self.have_fmu_import {
            let mut error_message = String::new();
            error_message.reserve(100);
            return Err(Error::Unavailable("actual checkOperatingSystem127"));
        }
        Ok(())
    }

    fn get_source_input(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), Error> {
        shortcut.current_module_object = "ExternalInterface".into();
        self.number_external_interfaces = native_count(input, &shortcut.current_module_object)?;
        if self.number_external_interfaces > 0 {
            return Err(Error::Unavailable(
                "actual positive ExternalInterface getObjectItem164",
            ));
        }
        if self.number_bcvtb == 0 {
            for object_word in [
                "ExternalInterface:Schedule",
                "ExternalInterface:Variable",
                "ExternalInterface:Actuator",
            ] {
                warn_if_external_objects_are_used(input, object_word, backend)?;
            }
        }
        if self.number_fmu_export == 0 {
            for object_word in [
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule",
                "ExternalInterface:FunctionalMockupUnitExport:To:Variable",
                "ExternalInterface:FunctionalMockupUnitExport:To:Actuator",
            ] {
                warn_if_external_objects_are_used(input, object_word, backend)?;
            }
        }
        if self.number_fmu_import == 0 {
            for object_word in [
                "ExternalInterface:FunctionalMockupUnitImport:To:Schedule",
                "ExternalInterface:FunctionalMockupUnitImport:To:Variable",
                "ExternalInterface:FunctionalMockupUnitImport:To:Actuator",
            ] {
                warn_if_external_objects_are_used(input, object_word, backend)?;
            }
        }
        if self.number_bcvtb == 1 && self.number_fmu_export == 0 {
            self.have_bcvtb = true;
            backend.display_string("Instantiating Building Controls Virtual Test Bed")?;
            return Err(Error::Unavailable(
                "actual ExternalInterface varKeys allocation211",
            ));
        } else if self.number_bcvtb == 0 && self.number_fmu_export == 1 {
            self.have_fmu_export = true;
            self.fmu_export_activate = 1;
            backend.display_string("Instantiating FunctionalMockupUnitExport interface")?;
            return Err(Error::Unavailable(
                "actual ExternalInterface varKeys allocation220",
            ));
        } else if self.number_bcvtb == 1 && self.number_fmu_export != 0 {
            backend.show_severe_error("GetExternalInterfaceInput: Cannot have Ptolemy and FMU-Export interface simultaneously.")?;
            self.errors_found = true;
        }
        if self.number_fmu_import == 1 && self.number_fmu_export == 0 {
            self.have_fmu_import = true;
            backend.display_string("Instantiating FunctionalMockupUnitImport interface")?;
            shortcut.current_module_object = "ExternalInterface:FunctionalMockupUnitImport".into();
            self.number_fmu_objects = native_count(input, &shortcut.current_module_object)?;
            return Err(Error::Unavailable(
                "actual VerifyExternalInterfaceObject235",
            ));
        } else if self.number_fmu_import == 1 && self.number_fmu_export != 0 {
            backend.show_severe_error("GetExternalInterfaceInput: Cannot have FMU-Import and FMU-Export interface simultaneously.")?;
            self.errors_found = true;
        }
        if self.number_bcvtb > 1 {
            backend.show_severe_error(
                "GetExternalInterfaceInput: Cannot have more than one Ptolemy interface.",
            )?;
            backend.show_continue_error("GetExternalInterfaceInput: Errors found in input.")?;
            self.errors_found = true;
        }
        if self.number_fmu_export > 1 {
            backend.show_severe_error(
                "GetExternalInterfaceInput: Cannot have more than one FMU-Export interface.",
            )?;
            backend.show_continue_error("Errors found in input.")?;
            self.errors_found = true;
        }
        if self.number_fmu_import > 1 {
            backend.show_severe_error(
                "GetExternalInterfaceInput: Cannot have more than one FMU-Import interface.",
            )?;
            backend.show_continue_error("Errors found in input.")?;
            self.errors_found = true;
        }
        if self.errors_found {
            return fatal(
                backend,
                "GetExternalInterfaceInput: preceding conditions cause termination.",
            );
        }
        self.stop_source_if_error(backend)
    }

    fn stop_source_if_error(&self, backend: &mut SourceErrorBackend<'_>) -> Result<(), Error> {
        if (self.number_bcvtb != 0 || self.number_fmu_export != 0) && self.errors_found {
            if self.socket_fd >= 0 {
                // Both Native simulationStatus branches first call the real socket.
                // No fabricated return code or send success is available here.
                let _flag = if self.simulation_status == 1 {
                    -10
                } else {
                    -20
                };
                return Err(Error::Unavailable("actual sendclientmessage289/291"));
            }
            return fatal(
                backend,
                "Error in ExternalInterface: Check EnergyPlus *.err file.",
            );
        }
        if self.number_fmu_import != 0 && self.errors_found {
            return fatal(
                backend,
                "ExternalInterface/StopExternalInterfaceIfError: Error in ExternalInterface: Check EnergyPlus *.err file.",
            );
        }
        Ok(())
    }
}

impl SourceExternalInterface {
    pub(crate) fn close_source_socket_at_success(&self) -> Result<(), Error> {
        if self.number_external_interfaces > 0 && self.have_bcvtb {
            return Err(Error::Unavailable(
                "actual EndEnergyPlus ExternalInterface.CloseSocket592",
            ));
        }
        Ok(())
    }
}
