//! Actual DD SetupOutputVariable calls and same-owner SPSiteSchedules Which.
use super::state::SourceSizingPeriodSiteSchedule;
use super::*;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_schedule_manager::{
    SelectedScheduleManager, SourceDayScheduleIndex,
};
use crate::heat_balance::source_output_prescan::SourceOutputPreScan;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
    SourceOutputStoreType, SourceOutputUnits, SourceRealOutputBinding, SourceRealOutputDeclaration,
    SourceRealOutputOwners, SourceRealOutputTarget, SourceReportVariableEnvironment,
    SourceReportVariableInput, SourceWeatherRealOutputField as Field,
};
use crate::weather::day::source_initialization::SourceWeatherReportFiles;
use crate::weather::day::source_weather_initialization::SourceWeatherEio;
use ep_model::ReportInputs;

pub(crate) struct SourceDesignDaySiteScheduleOutputOwners<'a> {
    pub(crate) schedules: &'a [SourceSizingPeriodSiteSchedule],
}
impl<'a> SourceDesignDaySiteScheduleOutputOwners<'a> {
    pub(crate) fn resolve(
        &self,
        target: SourceRealOutputTarget,
    ) -> Result<&'a f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::Weather { field } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        let environment = match field {
            Field::SizingPeriodDryBulbSchedule { environment }
            | Field::SizingPeriodHumiditySchedule { environment }
            | Field::SizingPeriodBeamSolarSchedule { environment }
            | Field::SizingPeriodDiffuseSolarSchedule { environment } => environment,
            _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        };
        let index = environment
            .checked_sub(1)
            .and_then(|v| usize::try_from(v).ok())
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        let row = self
            .schedules
            .get(index)
            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
        Ok(match field {
            Field::SizingPeriodDryBulbSchedule { .. } => &row.outdoor_dry_bulb_temperature,
            Field::SizingPeriodHumiditySchedule { .. } => &row.outdoor_relative_humidity,
            Field::SizingPeriodBeamSolarSchedule { .. } => &row.beam_solar_radiation,
            Field::SizingPeriodDiffuseSolarSchedule { .. } => &row.diffuse_solar_radiation,
            _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
        })
    }
}

impl SourceRealOutputOwners for SourceDesignDaySiteScheduleOutputOwners<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        self.resolve(target)
    }
}

/// Independent real owners split only for the actual registration call.
pub(crate) struct SourceDesignDayRegistrationOwners<'a, 'backend> {
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) schedules: &'a mut SelectedScheduleManager,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) developer_flag: &'a bool,
    pub(crate) registration: &'a mut SourceOutputRegistration,
    pub(crate) files: &'a mut dyn SourceWeatherReportFiles,
    pub(crate) eio: &'a mut dyn SourceWeatherEio,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
pub(crate) trait SourceDesignDayOutputRegistration {
    fn register(
        &mut self,
        declaration: SourceRealOutputDeclaration<'_>,
        binding: SourceRealOutputBinding<'_>,
        owners: &mut SourceDesignDayRegistrationOwners<'_, '_>,
    ) -> Result<(), Error>;
}
pub(crate) struct SourceDesignDayOutputContext<'a, H> {
    pub(crate) output_host: &'a mut H,
    pub(crate) prescan: &'a SourceOutputPreScan,
    pub(crate) report_inputs: &'a ReportInputs,
    pub(crate) minimum_report_frequency: &'a str,
}
struct DictionaryBorrow<'a>(&'a mut dyn SourceWeatherReportFiles);
impl SourceOutputDictionaryHost for DictionaryBorrow<'_> {
    fn eso_good(&self) -> bool {
        SourceOutputDictionaryHost::eso_good(&*self.0)
    }
    fn write_eso_dictionary(&mut self, line: &str) -> std::io::Result<()> {
        SourceOutputDictionaryHost::write_eso_dictionary(&mut *self.0, line)
    }
    fn sqlite_present(&self) -> bool {
        SourceOutputDictionaryHost::sqlite_present(&*self.0)
    }
    fn results_framework_output_variables(&mut self) -> &mut Vec<String> {
        SourceOutputDictionaryHost::results_framework_output_variables(&mut *self.0)
    }
}
impl<H: SourceOutputFileHost> SourceDesignDayOutputRegistration
    for SourceDesignDayOutputContext<'_, H>
{
    fn register(
        &mut self,
        declaration: SourceRealOutputDeclaration<'_>,
        binding: SourceRealOutputBinding<'_>,
        o: &mut SourceDesignDayRegistrationOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut input = SourceReportVariableInput::with_schedules(
            self.report_inputs,
            o.input,
            SourceReportVariableEnvironment::from_owners(
                self.minimum_report_frequency,
                o.globals,
                o.developer_flag,
                Some(o.clock),
            ),
            o.schedules,
            o.backend,
        );
        let mut dictionary = DictionaryBorrow(o.files);
        o.registration
            .setup_bound_unmetered_zone_real(
                &mut input,
                self.prescan,
                declaration,
                o.clock.minutes_in_time_step,
                self.output_host,
                o.eio,
                binding,
                &mut dictionary,
            )
            .map_err(Error::Output)
    }
}

/// Native spSiteSchedNums de-duplicates by the single actual DaySchedule::Num
/// across DDs and roles; append precedes the original registration side effects.
#[allow(clippy::too_many_arguments)]
pub(super) fn register(
    numbers: &mut Vec<i32>,
    sites: &[SourceSizingPeriodSiteSchedule],
    day: SourceDayScheduleIndex,
    field: Field,
    name: &str,
    key: &str,
    units: SourceOutputUnits,
    o: &mut SourceDesignDayInputOwners<'_, '_>,
) -> Result<(), Error> {
    let number =
        i32::try_from(day.0).map_err(|_| Error::Unavailable("actual DD DaySchedule Num i32"))?;
    if numbers.contains(&number) {
        return Ok(());
    }
    numbers.push(number);
    let actual = SourceDesignDaySiteScheduleOutputOwners { schedules: sites };
    let binding = SourceRealOutputBinding::new(SourceRealOutputTarget::Weather { field }, &actual)
        .map_err(Error::Output)?;
    let mut owners = SourceDesignDayRegistrationOwners {
        input: o.input,
        schedules: o.schedules,
        clock: o.clock,
        globals: o.globals,
        developer_flag: o.developer_flag,
        registration: o.registration,
        files: o.files,
        eio: o.eio,
        backend: o.backend,
    };
    o.output.register(
        SourceRealOutputDeclaration {
            name,
            key,
            units,
            store_type: SourceOutputStoreType::Average,
        },
        binding,
        &mut owners,
    )
}
