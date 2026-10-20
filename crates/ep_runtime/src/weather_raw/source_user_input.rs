//! Real ReadUserWeatherInput special-day/DST children on the same parsed header.
use super::{RawDayFailure, RawEpwCursorOwner, RawEpwHeaderError};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInput, SourceLocalObjectArguments,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::weather::day::SourceWeatherInitializationError as Failure;

use super::DateType;
use super::source_typical::{SourceDateScratch, process_source_date_string};

fn raw(error: RawEpwHeaderError) -> Failure {
    RawDayFailure::Header(error).into()
}
const DAY_TYPES: [&str; 13] = [
    "UNUSED",
    "SUNDAY",
    "MONDAY",
    "TUESDAY",
    "WEDNESDAY",
    "THURSDAY",
    "FRIDAY",
    "SATURDAY",
    "HOLIDAY",
    "SUMMERDESIGNDAY",
    "WINTERDESIGNDAY",
    "CUSTOMDAY1",
    "CUSTOMDAY2",
];
fn defined<T>(value: Option<T>, reason: &'static str) -> Result<T, Failure> {
    value.ok_or_else(|| Error::Unavailable(reason).into())
}
fn invalid_key(
    backend: &mut SourceErrorBackend<'_>,
    routine: &str,
    family: &str,
    name: &str,
    field: &str,
    value: &str,
) -> Result<(), Failure> {
    backend.show_severe_error(&format!("{routine}: {family} = {name}"))?;
    backend.show_continue_error(&format!("{field} = {value}, invalid key."))?;
    Ok(())
}
impl RawEpwCursorOwner {
    /// GetSpecialDayPeriodData5690–5810; no local arrays or count stores skipped.
    pub(crate) fn get_source_special_day_period_data(
        &mut self,
        operands: &SelectedHeatBalanceInput,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Failure> {
        shortcut.current_module_object = "RunPeriodControl:SpecialDays".into();
        let number = input.get_num_objects_found(&shortcut.current_module_object);
        let number_i32 = i32::try_from(number)
            .map_err(|_| Error::Unavailable("Native NumSpecDays query5734 i32 domain"))?;
        let mut count = if self.header.special_days.allocated {
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
        let mut arguments = SourceLocalObjectArguments::<3, 1>::allocated_unwritten();
        for ordinal in 0..number {
            operands.get_no_optional_source_object_item(
                "RunPeriodControl:SpecialDays",
                ordinal,
                input,
                &mut arguments,
            )?;
            let index = usize::try_from(
                count
                    .checked_sub(1)
                    .ok_or(Error::Unavailable("actual SpecialDays Count5754"))?,
            )
            .map_err(|_| Error::Unavailable("actual SpecialDays Count5754"))?;
            let special = self
                .header
                .special_days
                .values
                .get_mut(index)
                .ok_or(Error::Unavailable("actual retained SpecialDays Count5754"))?;
            special.name = arguments.alphas[0].clone();
            let mut parsed = SourceDateScratch::default();
            process_source_date_string(&arguments.alphas[1], &mut parsed, false, errors, backend)?;
            if parsed.date_type == DateType::MonthDay {
                special.date_type = parsed.date_type;
                special.month = defined(parsed.month, "actual special PMonth")?;
                special.day = defined(parsed.day, "actual special PDay")?;
                special.weekday = 0;
                special.compressed_date = special
                    .month
                    .checked_mul(32)
                    .and_then(|v| v.checked_add(special.day))
                    .ok_or(Error::Unavailable("actual SpecialDays CompDate5769"))?;
                special.weather_file = false;
            } else if parsed.date_type != DateType::Invalid {
                special.date_type = parsed.date_type;
                special.month = defined(parsed.month, "actual special PMonth")?;
                special.day = defined(parsed.day, "actual special PDay")?;
                special.weekday = defined(parsed.weekday, "actual special PWeekDay")?;
                special.compressed_date = 0;
                special.weather_file = false;
            } else {
                invalid_key(
                    backend,
                    "GetSpecialDayPeriodData",
                    &shortcut.current_module_object,
                    &special.name,
                    shortcut
                        .alpha_field_names
                        .get(1)
                        .ok_or(Error::Unavailable("actual shared alpha name5779"))?,
                    &arguments.alphas[1],
                )?;
                *errors = true;
            }
            let duration =
                arguments.numbers[0].ok_or(Error::Unavailable("actual local Duration5783"))?;
            if duration > 0.0 {
                if !duration.is_finite() || duration.trunc() > f64::from(i32::MAX) {
                    return Err(Error::Unavailable("actual SpecialDays int duration5784").into());
                }
                special.duration = duration.trunc() as i32;
            } else {
                let value = crate::heat_balance::source_round_format::truncate(duration, 0)
                    .map_err(|_| Error::Unavailable("actual Duration .0T5789"))?;
                let field = shortcut
                    .numeric_field_names
                    .first()
                    .ok_or(Error::Unavailable("actual shared numeric name5789"))?;
                backend.show_severe_error(&format!(
                    "{}: {} Invalid {field}={value}",
                    shortcut.current_module_object, arguments.alphas[0]
                ))?;
                *errors = true;
            }
            let day_type = DAY_TYPES
                .iter()
                .position(|v| *v == arguments.alphas[2])
                .map_or(-1, |i| i as i32);
            if day_type == 0 {
                invalid_key(
                    backend,
                    "GetSpecialDayPeriodData",
                    &shortcut.current_module_object,
                    &special.name,
                    shortcut
                        .alpha_field_names
                        .get(2)
                        .ok_or(Error::Unavailable("actual shared alpha name5795"))?,
                    &arguments.alphas[2],
                )?;
                *errors = true;
            } else {
                special.day_type = day_type;
            }
            count = count
                .checked_add(1)
                .ok_or(Error::Unavailable("actual SpecialDays Count loop5807"))?;
        }
        Ok(())
    }
    /// Actual GetDSTData shared arguments, retained references and diagnostics.
    pub(crate) fn get_source_dst_data(
        &mut self,
        operands: &SelectedHeatBalanceInput,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        errors: &mut bool,
    ) -> Result<(), Failure> {
        shortcut.current_module_object = "RunPeriodControl:DaylightSavingTime".into();
        let number = input.get_num_objects_found(&shortcut.current_module_object);
        if number == 1 {
            let counts = operands.get_shared_source_object_item(
                "RunPeriodControl:DaylightSavingTime",
                0,
                input,
                shortcut,
            )?;
            if counts.num_alphas != 2 {
                backend.show_severe_error(&format!(
                    "{}: Insufficient fields, must have Start AND End Dates",
                    shortcut.current_module_object
                ))?;
                *errors = true;
            } else {
                let title = shortcut
                    .alpha_args
                    .first()
                    .ok_or(Error::Unavailable("actual IDFDST alpha5893"))?
                    .clone();
                // These are retained reference arguments, unlike the EPW's
                // unwritten locals. Preserve any child-unwritten actual cells.
                for start in [true, false] {
                    let d = &mut self.header.idf_dst;
                    let mut parsed = if start {
                        SourceDateScratch {
                            date_type: d.start_date_type,
                            month: Some(d.start_month),
                            day: Some(d.start_day),
                            weekday: d.start_weekday,
                            year: None,
                        }
                    } else {
                        SourceDateScratch {
                            date_type: d.end_date_type,
                            month: Some(d.end_month),
                            day: Some(d.end_day),
                            weekday: d.end_weekday,
                            year: None,
                        }
                    };
                    let at = usize::from(!start);
                    let text = shortcut
                        .alpha_args
                        .get(at)
                        .ok_or(Error::Unavailable("actual IDFDST shared alpha5910"))?;
                    let outcome =
                        process_source_date_string(text, &mut parsed, false, errors, backend);
                    // Reference outputs already written remain visible if later
                    // input diagnostics fail; no replacement constructor is used.
                    if start {
                        d.start_date_type = parsed.date_type;
                        d.start_month = defined(parsed.month, "actual IDFDST start month")?;
                        d.start_day = defined(parsed.day, "actual IDFDST start day")?;
                        d.start_weekday = parsed.weekday;
                    } else {
                        d.end_date_type = parsed.date_type;
                        d.end_month = defined(parsed.month, "actual IDFDST end month")?;
                        d.end_day = defined(parsed.day, "actual IDFDST end day")?;
                        d.end_weekday = parsed.weekday;
                    }
                    outcome?;
                    if parsed.date_type == DateType::Invalid {
                        invalid_key(
                            backend,
                            "GetDSTData",
                            &shortcut.current_module_object,
                            &title,
                            shortcut
                                .alpha_field_names
                                .get(at)
                                .ok_or(Error::Unavailable("actual IDFDST field name5917"))?,
                            text,
                        )?;
                        *errors = true;
                    }
                }
                self.header.idf_daylight_saving = true;
            }
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
