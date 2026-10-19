//! ReportScheduleVals3612–3643 and UpdateScheduleVals2465–2488.
//! Retained Which reads currentVal; getCurrentVal EMS semantics are separate.
use super::{SelectedScheduleManager, SourceScheduleIndex};
use crate::weather::day::{WeatherGlobalState, WeatherEnvironmentState};
use crate::heat_balance::{
    radiation::ordinary::{OrdinaryRadiationError, OrdinaryRadiationUnavailable},
    selected_heat_balance_input::HeatBalanceInputError as Error,
    source_output_prescan::SourceOutputPreScan,
    source_output_registration::{
        SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
        SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding,
        SourceRealOutputDeclaration, SourceRealOutputOwners, SourceRealOutputTarget,
        SourceReportVariableInput,
    },
};
impl SourceRealOutputOwners for SelectedScheduleManager {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::Schedule { index } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        self.schedules
            .get(index.0)
            .map(|row| &row.current_value)
            .ok_or_else(|| OrdinaryRadiationUnavailable::StorageDomain.into())
    }
}
impl SelectedScheduleManager {
    pub(crate) fn report_source_schedule_values<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &mut self,
        input: &mut SourceReportVariableInput<'_>,
        registration: &mut SourceOutputRegistration,
        prescan: &SourceOutputPreScan,
        minutes_in_time_step: i32,
        clock: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        host: &mut H,
        eio: &mut dyn std::io::Write,
        dictionary_host: &mut D,
    ) -> Result<(), Error> {
        if self.do_schedule_reporting_setup {
            for schedule in &self.schedules {
                if schedule.number.0 == 0 || schedule.number.0 == 1 {
                    continue;
                }
                let binding = SourceRealOutputBinding::new(
                    SourceRealOutputTarget::Schedule {
                        index: schedule.number,
                    },
                    self,
                )
                .map_err(Error::Output)?;
                registration
                    .setup_bound_unmetered_zone_real(
                        input,
                        prescan,
                        SourceRealOutputDeclaration {
                            name: "Schedule Value",
                            key: &schedule.name,
                            units: SourceOutputUnits::Dimensionless,
                            store_type: SourceOutputStoreType::Average,
                        },
                        minutes_in_time_step,
                        host,
                        eio,
                        binding,
                        dictionary_host,
                    )
                    .map_err(Error::Output)?;
            }
            self.do_schedule_reporting_setup = false;
        }
        self.update_source_schedule_values(clock, environment)
    }
    /// Actual UpdateScheduleVals2465 uses the same live global/environment.
    pub(crate) fn update_source_schedule_values(
        &mut self, clock: &WeatherGlobalState, environment: &WeatherEnvironmentState,
    ) -> Result<(), Error> {
        for index in 0..self.schedules.len() {
            let value = if self.schedules[index].ems_actuated_on {
                self.schedules[index].ems_value
            } else {
                self.source_hr_ts_value(SourceScheduleIndex(index), clock.hour_of_day,
                    clock.time_step, clock, environment)?
            };
            self.schedules[index].current_value = value;
        }
        Ok(())
    }
    /// Actual Schedule::getCurrentVal266: EMS override before retained currentVal.
    pub(crate) fn source_current_schedule_value(
        &self,
        index: SourceScheduleIndex,
    ) -> Result<f64, Error> {
        let schedule = self.schedules.get(index.0).ok_or(Error::Unavailable(
            "actual Schedule getCurrentVal pointer domain",
        ))?;
        Ok(if schedule.ems_actuated_on {
            schedule.ems_value
        } else {
            schedule.current_value
        })
    }
}
