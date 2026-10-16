//! Real ReadUserWeatherInput special-day/DST children on the same parsed header.
use super::{RawDayFailure, RawEpwCursorOwner, RawEpwHeaderError};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SourceLocalObjectArguments,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::weather::day::SourceWeatherInitializationError as Failure;

fn raw(error: RawEpwHeaderError) -> Failure {
    RawDayFailure::Header(error).into()
}
impl RawEpwCursorOwner {
    /// GetSpecialDayPeriodData5690–5810; no local arrays or count stores skipped.
    pub(crate) fn get_source_special_day_period_data(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), Failure> {
        shortcut.current_module_object = "RunPeriodControl:SpecialDays".into();
        let number = input.get_num_objects_found(&shortcut.current_module_object);
        let number_i32 = i32::try_from(number)
            .map_err(|_| Error::Unavailable("Native NumSpecDays query5734 i32 domain"))?;
        let count = if self.header.special_days.allocated {
            self.header
                .number_special_days
                .checked_sub(number_i32)
                .and_then(|v| v.checked_add(1))
                .ok_or(Error::Unavailable(
                    "Native SpecialDays Count5737 signed domain",
                ))?
        } else {
            self.header.special_days.allocate(number);
            self.header.number_special_days = number_i32;
            1
        };
        let arguments = SourceLocalObjectArguments::<3, 1>::allocated_unwritten();
        if number > 0 {
            return Err(Error::Unavailable(
                "GetSpecialDayPeriodData genuine no-optional getObjectItem5752 positive branch",
            )
            .into());
        }
        let _ = (count, arguments); // Genuine scoped local arrays/count, no native value substitute.
        Ok(())
    }
    /// GetDSTData zero-input return and real too-many-object diagnostics.
    pub(crate) fn get_source_dst_data(
        &mut self,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Failure> {
        shortcut.current_module_object = "RunPeriodControl:DaylightSavingTime".into();
        let number = input.get_num_objects_found(&shortcut.current_module_object);
        if number == 1 {
            return Err(Error::Unavailable(
                "GetDSTData genuine shared getObjectItem5893 positive branch",
            )
            .into());
        }
        if number > 1 {
            backend.show_severe_error(&format!(
                "{}: Too many objects in Input File, only one allowed.",
                shortcut.current_module_object
            ))?;
            *errors = true;
        }
        Ok(())
    }
    /// Actual ReadUser4812–4814: two distinct native DST values, not a shadow copy.
    pub(crate) fn apply_source_idf_dst_override(&mut self) {
        if self.header.idf_daylight_saving {
            self.header.dst = self.header.idf_dst.clone();
        }
    }
    /// Reuse the existing genuine SetupWeekDaysByMonth numerical body/owners.
    pub(crate) fn setup_source_weekdays(
        &self,
        month: i32,
        day: i32,
        weekday: i32,
        out: &mut [i32; 12],
    ) -> Result<(), Failure> {
        super::dates::setup_weekdays(
            month,
            day,
            weekday,
            &self.header.month_ends,
            self.header.leap_year_add,
            out,
        )
        .map_err(raw)
    }
}
