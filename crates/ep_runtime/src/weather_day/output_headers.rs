//! Genuine ReportOutputFileHeaders4611?4665 on the sole OP and weather owners.
use super::{SourceWeatherInitializationError, WeatherOwnerState};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::source_output_registration::SourceOutputRegistration;
use crate::heat_balance::source_output_registration::SourceOutputReportStreams;
impl WeatherOwnerState {
    pub(crate) fn report_source_output_file_headers(
        &mut self,
        op: &mut SourceOutputRegistration,
        files: &mut dyn SourceOutputReportStreams,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SourceWeatherInitializationError> {
        const ENV: &str =
            ",5,Environment Title[],Latitude[deg],Longitude[deg],Time Zone[],Elevation[m]";
        const FREQ: [&str; 7] = [
            "",
            ",8,Day of Simulation[],Month[],Day of Month[],DST Indicator[1=yes 0=no],Hour[],StartMinute[],EndMinute[],DayType",
            "",
            ",5,Cumulative Day of Simulation[],Month[],Day of Month[],DST Indicator[1=yes 0=no],DayType  ! When Daily ",
            ",2,Cumulative Days of Simulation[],Month[]  ! When Monthly ",
            ",1,Cumulative Days of Simulation[] ! When Run Period ",
            ",1,Calendar Year of Simulation[] ! When Annual ",
        ];
        op.report_number_counter =
            op.report_number_counter
                .checked_add(1)
                .ok_or(Error::Unavailable(
                    "actual defined ReportNumberCounter increment4646",
                ))?;
        self.environment_report_number = op.report_number_counter;
        if self.environment_report_number != 1 {
            return Err(match backend.show_fatal_error("ReportOutputFileHeaders: Assigned report number for Environment title is not 1.  Contact Support.") {
                Ok(never)=>match never{},Err(error)=>Error::Fatal(error)
            }.into());
        }
        self.environment_report_character = self.environment_report_number.to_string();
        self.environment_report_character = self.environment_report_character.trim().into();
        files
            .write_eso(format!("{}{}\n", self.environment_report_character, ENV).as_bytes())
            .map_err(Error::Io)?;
        files
            .write_mtr(format!("{}{}\n", self.environment_report_character, ENV).as_bytes())
            .map_err(Error::Io)?;
        op.report_number_counter =
            op.report_number_counter
                .checked_add(1)
                .ok_or(Error::Unavailable(
                    "actual defined ReportNumberCounter increment4656",
                ))?;
        // Chained assignment writes TimeStep before Hour.
        op.frequency_stamp_report_numbers[1] = op.report_number_counter;
        op.frequency_stamp_report_numbers[2] = op.frequency_stamp_report_numbers[1];
        files
            .write_eso(format!("{}{}\n", op.frequency_stamp_report_numbers[1], FREQ[1]).as_bytes())
            .map_err(Error::Io)?;
        files
            .write_mtr(format!("{}{}\n", op.frequency_stamp_report_numbers[1], FREQ[1]).as_bytes())
            .map_err(Error::Io)?;
        for frequency in [3, 4, 5, 6] {
            op.report_number_counter =
                op.report_number_counter
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "actual defined ReportNumberCounter increment4661",
                    ))?;
            op.frequency_stamp_report_numbers[frequency] = op.report_number_counter;
            files
                .write_eso(
                    format!(
                        "{}{}Report Variables Requested\n",
                        op.frequency_stamp_report_numbers[frequency], FREQ[frequency]
                    )
                    .as_bytes(),
                )
                .map_err(Error::Io)?;
            files
                .write_mtr(
                    format!(
                        "{}{}Meters Requested\n",
                        op.frequency_stamp_report_numbers[frequency], FREQ[frequency]
                    )
                    .as_bytes(),
                )
                .map_err(Error::Io)?;
        }
        Ok(())
    }
}
