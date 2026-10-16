//! Genuine ReadUserWeatherInput driver and same-owner zero weather-property child.
use super::source_initialization::{
    SourceWeatherInitializationChildren, SourceWeatherInitializationOwners,
};
use super::{
    DailyWeatherVariables, SourceEnvironmentData, SourceWeatherInitializationError as Failure,
    WeatherSession,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;

/// Actual WeatherManager.hh416–425, owned by WPSkyTemperature alone.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceWeatherProperty {
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub name: String,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub is_schedule: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub sky_temperature_model: i32,
    pub(crate) schedule: Option<SourceScheduleIndex>,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub used_for_environment: bool,
    /// Actual retained Native member; see the type's pinned constructor authority.
    pub use_weather_file_horizontal_infrared: bool,
}
impl Default for SourceWeatherProperty {
    fn default() -> Self {
        Self {
            name: String::new(),
            is_schedule: true,
            sky_temperature_model: 0,
            schedule: None,
            used_for_environment: false,
            use_weather_file_horizontal_infrared: true,
        }
    }
}
fn count(value: usize) -> Result<i32, Failure> {
    Ok(i32::try_from(value)
        .map_err(|_| Error::Unavailable("Native weather input object-count i32 domain"))?)
}
fn size(value: i32) -> Result<usize, Failure> {
    Ok(usize::try_from(value)
        .map_err(|_| Error::Unavailable("Native weather input array dimension"))?)
}
fn checked(value: Option<i32>) -> Result<i32, Failure> {
    Ok(value.ok_or(Error::Unavailable(
        "Native weather input signed count arithmetic",
    ))?)
}
fn fatal(backend: &mut SourceErrorBackend<'_>, text: &str) -> Failure {
    match backend.show_fatal_error(text) {
        Ok(never) => match never {},
        Err(error) => Error::Fatal(error).into(),
    }
}
impl WeatherSession {
    /// Real GetDesignDayData child bridge. The caller supplies the existing
    /// predefined registry/table separately; all weather/IP arguments are live
    /// borrows of the owners already allocated by this ReadUser invocation.
    pub(crate) fn read_source_design_day_input(
        &mut self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        registry: &crate::heat_balance::predefined_tables::PredefinedRegistry,
        table: &mut crate::heat_balance::ctf_reports::PredefinedStringTable,
        errors_found: &mut bool,
    ) -> Result<(), Failure> {
        let weather = &mut self.state.weather;
        weather.source_design_day_input.get_source_design_day_data(
            self.state.environment.tot_des_days,
            &mut super::source_design_day::SourceDesignDayInputOwners {
                raw: owners.raw,
                input: owners.input,
                shortcut: owners.shortcut,
                clock: &self.state.global,
                force_begin_env_reset_suppress: &self
                    .state
                    .environment
                    .force_begin_env_reset_suppress,
                reverse_design_days: owners.reverse_design_days,
                interpolation: &weather.interpolation,
                environments: weather.source_environments.as_deref_mut().ok_or(
                    Error::Unavailable("actual ReadUser4767 Environment allocation for DD child"),
                )?,
                design_day_weather: weather.source_design_day_weather.as_deref().ok_or(
                    Error::Unavailable("actual ReadUser4766 DesignDay allocation for DD child"),
                )?,
                registry,
                table,
                backend: owners.backend,
            },
            errors_found,
        )?;
        Ok(())
    }
    pub(crate) fn read_source_user_weather_input(
        &mut self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        children: &mut impl SourceWeatherInitializationChildren,
    ) -> Result<(), Failure> {
        let mut errors_found = false;
        self.state.environment.tot_des_days =
            count(owners.input.get_num_objects_found("SizingPeriod:DesignDay"))?;
        let rpd1 = count(
            owners
                .input
                .get_num_objects_found("SizingPeriod:WeatherFileDays"),
        )?;
        let rpd2 = count(
            owners
                .input
                .get_num_objects_found("SizingPeriod:WeatherFileConditionType"),
        )?;
        self.state.weather.tot_run_pers = count(owners.input.get_num_objects_found("RunPeriod"))?;
        self.state.weather.num_of_envrn = checked(
            self.state
                .environment
                .tot_des_days
                .checked_add(self.state.weather.tot_run_pers)
                .and_then(|v| v.checked_add(rpd1))
                .and_then(|v| v.checked_add(rpd2)),
        )?;
        self.state.global.weather_simulation_requested = self.state.weather.tot_run_pers > 0;
        self.state.weather.tot_report_periods = count(
            owners
                .input
                .get_num_objects_found("Output:Table:ReportPeriod"),
        )?;
        self.state.weather.source_design_day_weather =
            Some(vec![
                DailyWeatherVariables::default();
                size(self.state.environment.tot_des_days)?
            ]);
        self.state.weather.source_environments = Some(vec![
            SourceEnvironmentData::default();
            size(self.state.weather.num_of_envrn)?
        ]);
        let environments = self
            .state
            .weather
            .source_environments
            .as_mut()
            .ok_or(Error::Unavailable("actual Environment allocation4767"))?;
        for ordinal in 0..size(self.state.environment.tot_des_days)? {
            environments
                .get_mut(ordinal)
                .ok_or(Error::Unavailable("actual design Environment row4772"))?
                .kind_of_environment = 1;
        }
        for ordinal in 0..size(checked(rpd1.checked_add(rpd2))?)? {
            let index = size(checked(
                self.state
                    .environment
                    .tot_des_days
                    .checked_add(i32::try_from(ordinal).map_err(|_| {
                        Error::Unavailable("actual design-run Environment ordinal")
                    })?),
            )?)?;
            environments
                .get_mut(index)
                .ok_or(Error::Unavailable("actual design-run Environment row4776"))?
                .kind_of_environment = if !owners.system.dd_only { 2 } else { 3 };
        }
        for ordinal in 0..size(self.state.weather.tot_run_pers)? {
            let index = size(checked(
                self.state
                    .environment
                    .tot_des_days
                    .checked_add(rpd1)
                    .and_then(|v| v.checked_add(rpd2))
                    .and_then(|v| v.checked_add(i32::try_from(ordinal).ok()?)),
            )?)?;
            environments
                .get_mut(index)
                .ok_or(Error::Unavailable("actual RunPeriod Environment row4782"))?
                .kind_of_environment = 3;
        }
        if self.state.environment.tot_des_days >= 1 {
            // Mandatory genuine same-owner child; no typed/configuration projection
            // or default-success implementation can complete the real A DD2 route.
            children.get_source_design_day_data(self, owners, &mut errors_found)?;
        }
        if rpd1 >= 1 || rpd2 >= 1 {
            return Err(Error::Unavailable(
                "ReadUserWeatherInput4789 genuine GetRunPeriodDesignData positive child",
            )
            .into());
        }
        if self.state.weather.tot_run_pers >= 1 || owners.system.full_annual_run {
            self.get_source_run_period_data(
                self.state.weather.tot_run_pers,
                owners,
                &mut errors_found,
            )?;
        }
        if self.state.weather.tot_report_periods > 0 {
            return Err(Error::Unavailable(
                "ReadUserWeatherInput4800 genuine GetReportPeriodData positive child",
            )
            .into());
        }
        if owners.system.full_annual_run {
            self.state.weather.num_of_envrn = checked(
                self.state
                    .environment
                    .tot_des_days
                    .checked_add(self.state.weather.tot_run_pers)
                    .and_then(|v| v.checked_add(rpd1))
                    .and_then(|v| v.checked_add(rpd2)),
            )?;
        }
        if rpd1 >= 1
            || rpd2 >= 1
            || self.state.weather.tot_run_pers >= 1
            || owners.system.full_annual_run
        {
            self.cursor
                .get_source_special_day_period_data(owners.input, owners.shortcut)?;
            self.cursor.get_source_dst_data(
                owners.input,
                owners.shortcut,
                owners.backend,
                &mut errors_found,
            )?;
            self.cursor.apply_source_idf_dst_override();
        }
        self.read_source_location_ground_and_auxiliary_inputs(owners, &mut errors_found)?;
        self.setup_source_environment_types()?;
        self.get_source_weather_properties(owners, &mut errors_found)?;
        if errors_found {
            return Err(fatal(
                owners.backend,
                "GetWeatherInput: Above errors cause termination",
            ));
        }
        Ok(())
    }
    fn get_source_weather_properties(
        &mut self,
        owners: &mut SourceWeatherInitializationOwners<'_, '_>,
        _errors: &mut bool,
    ) -> Result<(), Failure> {
        owners.shortcut.current_module_object = "WeatherProperty:SkyTemperature".into();
        self.state.weather.number_weather_properties = count(
            owners
                .input
                .get_num_objects_found(&owners.shortcut.current_module_object),
        )?;
        self.state.weather.source_weather_properties =
            Some(vec![
                SourceWeatherProperty::default();
                size(self.state.weather.number_weather_properties)?
            ]);
        if self.state.weather.number_weather_properties > 0 {
            return Err(Error::Unavailable(
                "GetWeatherProperties6745 genuine shared getObjectItem positive child",
            )
            .into());
        }
        for environment in
            self.state
                .weather
                .source_environments
                .as_mut()
                .ok_or(Error::Unavailable(
                    "actual GetWeatherProperties Environment traversal6894",
                ))?
        {
            if environment.weather_properties_type_one != 0
                && self.state.weather.number_weather_properties
                    >= environment.weather_properties_type_one
            {
                let index = size(checked(
                    environment.weather_properties_type_one.checked_sub(1),
                )?)?;
                let property = self
                    .state
                    .weather
                    .source_weather_properties
                    .as_ref()
                    .and_then(|v| v.get(index))
                    .ok_or(Error::Unavailable("actual WPSkyTemperature Source row6896"))?;
                environment.sky_temperature_model = property.sky_temperature_model;
                environment.use_weather_file_horizontal_infrared =
                    property.use_weather_file_horizontal_infrared;
            }
        }
        Ok(())
    }
}
