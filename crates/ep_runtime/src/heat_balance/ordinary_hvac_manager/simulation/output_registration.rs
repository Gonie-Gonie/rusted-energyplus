//! Actual SimHVAC765-792 four declarations; source fields stay live through every read.
use super::super::{Error, SourceHvacManager};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::selected_heat_balance_input::SelectedHeatBalanceInputState;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceHvacRealOutputField as Field, SourceIntegerOutputBinding, SourceIntegerOutputDeclaration,
    SourceIntegerOutputOwners, SourceIntegerOutputTarget as Integer, SourceOutputDictionaryHost,
    SourceOutputFileHost, SourceOutputRegistration, SourceOutputStoreType as Store,
    SourceOutputUnits as Units, SourceRealOutputBinding, SourceRealOutputDeclaration,
    SourceRealOutputOwners, SourceRealOutputTarget as Real, SourceRealRegistrationOptions,
    SourceReportVariableInput, TimeStepKind,
};
use ep_model::ReportInputs;
use std::io::Write;

pub(crate) struct SourceHvacOutputOwners<'a> {
    pub(crate) manager: &'a SourceHvacManager,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
}
impl SourceIntegerOutputOwners for SourceHvacOutputOwners<'_> {
    fn integer_output(&self, target: Integer) -> Result<&i32, OrdinaryRadiationError> {
        Ok(match target {
            Integer::HvacSystemSolverIterationCount => &self.manager.manage_iteration,
            Integer::AirSystemSolverIterationCount => &self.manager.report_iteration_air,
            Integer::PlantSubIterationCount => &self.manager.simulation.plant.manage_sub_iterations,
            Integer::PlantHalfLoopCallCount => {
                &self.manager.simulation.plant.manage_half_loop_calls
            }
            _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        })
    }
}
impl SourceRealOutputOwners for SourceHvacOutputOwners<'_> {
    fn real_output(&self, target: Real) -> Result<&f64, OrdinaryRadiationError> {
        let Real::Hvac { field } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        Ok(match field {
            Field::SystemReliefHeatLoss => &self.control.sys_total_hvac_relief_heat_loss,
            Field::SystemRejectHeatLoss => &self.control.sys_total_hvac_reject_heat_loss,
        })
    }
}

/// Report/IP/backend/EIO/clock operands are borrowed afresh per actual declaration call.
pub(crate) struct SourceHvacOutputContext<'a, H, D> {
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) report_flags:
        &'a mut crate::heat_balance::radiation::ordinary::source_children::SourceSolarReportFlags,
    pub(crate) output_control:
        &'a crate::heat_balance::ordinary_output_control::SourceOutputControl,
    pub(crate) tabular_raw: &'a crate::heat_balance::ordinary_tabular_reports::SelectedTabularInput,
    pub(crate) sizing_raw: &'a crate::heat_balance::selected_sizing::SelectedSizingInput,
    pub(crate) results:
        &'a mut crate::heat_balance::ordinary_output_control::SourceResultsFrameworkOptions,
    pub(crate) standard_output_record_count: &'a mut i64,
    pub(crate) standard_meter_record_count: &'a mut i64,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) reports: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
    pub(crate) host: &'a mut H,
    pub(crate) dictionary_host: &'a mut D,
}
impl<H: SourceOutputFileHost, D: SourceOutputDictionaryHost> SourceHvacOutputContext<'_, H, D> {
    pub(crate) fn register_source_solver_outputs(
        &mut self,
        owners: &SourceHvacOutputOwners<'_>,
        input: &mut SelectedInputProcessor,
        minutes_in_time_step: i32,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        self.declare_integer(
            owners,
            Integer::HvacSystemSolverIterationCount,
            "HVAC System Solver Iteration Count",
            input,
            minutes_in_time_step,
            eio,
        )?;
        self.declare_integer(
            owners,
            Integer::AirSystemSolverIterationCount,
            "Air System Solver Iteration Count",
            input,
            minutes_in_time_step,
            eio,
        )?;
        self.declare_real(
            owners,
            Field::SystemReliefHeatLoss,
            "Air System Relief Air Total Heat Loss Energy",
            input,
            minutes_in_time_step,
            backend,
            eio,
        )?;
        self.declare_real(
            owners,
            Field::SystemRejectHeatLoss,
            "HVAC System Total Heat Rejection Energy",
            input,
            minutes_in_time_step,
            backend,
            eio,
        )
    }
    pub(crate) fn register_source_plant_solver_outputs(
        &mut self,
        owners: &SourceHvacOutputOwners<'_>,
        input: &mut SelectedInputProcessor,
        minutes_in_time_step: i32,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        self.declare_integer(
            owners,
            Integer::PlantSubIterationCount,
            "Plant Solver Sub Iteration Count",
            input,
            minutes_in_time_step,
            eio,
        )?;
        self.declare_integer(
            owners,
            Integer::PlantHalfLoopCallCount,
            "Plant Solver Half Loop Calls Count",
            input,
            minutes_in_time_step,
            eio,
        )
    }
    fn declare_integer(
        &mut self,
        owners: &SourceHvacOutputOwners<'_>,
        target: Integer,
        name: &str,
        input: &mut SelectedInputProcessor,
        minutes_in_time_step: i32,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let binding = SourceIntegerOutputBinding::new(target, owners).map_err(Error::Output)?;
        let mut report_input =
            SourceReportVariableInput::new(self.reports, input, self.minimum_report_frequency);
        self.registration
            .setup_bound_integer(
                &mut report_input,
                self.prescan,
                SourceIntegerOutputDeclaration {
                    name,
                    key: "SimHVAC",
                    units: Units::Dimensionless,
                    store_type: Store::Sum,
                },
                minutes_in_time_step,
                self.host,
                eio,
                binding,
                self.dictionary_host,
                TimeStepKind::System,
            )
            .map_err(Error::Output)
    }
    fn declare_real(
        &mut self,
        owners: &SourceHvacOutputOwners<'_>,
        field: Field,
        name: &str,
        input: &mut SelectedInputProcessor,
        minutes_in_time_step: i32,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let binding =
            SourceRealOutputBinding::new(Real::Hvac { field }, owners).map_err(Error::Output)?;
        let mut report_input =
            SourceReportVariableInput::new(self.reports, input, self.minimum_report_frequency);
        self.registration
            .setup_bound_real(
                &mut report_input,
                self.prescan,
                SourceRealOutputDeclaration {
                    name,
                    key: "SimHVAC",
                    units: Units::J,
                    store_type: Store::Sum,
                },
                minutes_in_time_step,
                self.host,
                eio,
                binding,
                self.dictionary_host,
                SourceRealRegistrationOptions::unmetered(TimeStepKind::System),
                backend,
            )
            .map_err(Error::Output)
    }
}
