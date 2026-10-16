//! Genuine four Site ground models, manager cache and Weather client pointers.
use super::{WeatherDayError, WeatherEnvironmentState, WeatherSession};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, SelectedHeatBalanceInput,
};
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
use crate::weather::raw::RawEpwHeaderState;
use std::io::Write;
mod factory;
mod input;
mod output;
pub(crate) struct SourceGroundTemperatureOwners<'a, 'backend> {
    pub(crate) raw: &'a SelectedHeatBalanceInput,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) shortcut: &'a mut SourceInputProcessorShortCut,
    pub(crate) environment: &'a mut WeatherEnvironmentState,
    pub(crate) header: &'a RawEpwHeaderState,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    pub(crate) eio: &'a mut dyn Write,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum SiteGroundModelKind {
    BuildingSurface,
    FCFactorMethod,
    Shallow,
    Deep,
}
impl SiteGroundModelKind {
    fn index(self) -> usize {
        match self {
            Self::BuildingSurface => 0,
            Self::Shallow => 1,
            Self::Deep => 2,
            Self::FCFactorMethod => 3,
        }
    }
    fn model_type(self) -> i32 {
        match self {
            Self::BuildingSurface => 2,
            Self::Shallow => 3,
            Self::Deep => 4,
            Self::FCFactorMethod => 5,
        }
    }
    fn family(self) -> &'static str {
        match self {
            Self::BuildingSurface => "Site:GroundTemperature:BuildingSurface",
            Self::Shallow => "Site:GroundTemperature:Shallow",
            Self::Deep => "Site:GroundTemperature:Deep",
            Self::FCFactorMethod => "Site:GroundTemperature:FCfactorMethod",
        }
    }
    fn family_upper(self) -> &'static str {
        match self {
            Self::BuildingSurface => "SITE:GROUNDTEMPERATURE:BUILDINGSURFACE",
            Self::Shallow => "SITE:GROUNDTEMPERATURE:SHALLOW",
            Self::Deep => "SITE:GROUNDTEMPERATURE:DEEP",
            Self::FCFactorMethod => "SITE:GROUNDTEMPERATURE:FCFACTORMETHOD",
        }
    }
    fn output_name(self) -> &'static str {
        match self {
            Self::BuildingSurface => "BuildingSurface",
            Self::Shallow => "Shallow",
            Self::Deep => "Deep",
            Self::FCFactorMethod => "FCfactorMethod",
        }
    }
    fn default_temperature(self) -> f64 {
        match self {
            Self::BuildingSurface => 18.0,
            Self::Shallow => 13.0,
            Self::Deep => 16.0,
            Self::FCFactorMethod => 13.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSiteGroundTemperatureModel {
    pub(crate) name: String,
    /// Actual BaseGroundTempsModel ctor Invalid=-1, assigned after count query.
    pub(crate) model_type: i32,
    pub(crate) time_of_sim_in_months: i32,
    pub(crate) monthly_temperatures: [f64; 12],
}
impl SourceSiteGroundTemperatureModel {
    fn construct(kind: SiteGroundModelKind) -> Self {
        // C++ std::array={13} initializes first13, every remaining element0.
        let monthly_temperatures = [13.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        Self {
            name: String::new(),
            model_type: -1,
            time_of_sim_in_months: if kind == SiteGroundModelKind::Deep {
                12
            } else {
                0
            },
            monthly_temperatures,
        }
    }
    fn at_month(&mut self, month: i32) -> Result<f64, WeatherDayError> {
        self.time_of_sim_in_months = if (1..=12).contains(&month) {
            month
        } else {
            month % 12
        };
        let index = self
            .time_of_sim_in_months
            .checked_sub(1)
            .and_then(|n| usize::try_from(n).ok())
            .ok_or_else(|| WeatherDayError::admission("actual ground-model month subscript"))?;
        self.monthly_temperatures
            .get(index)
            .copied()
            .ok_or_else(|| WeatherDayError::admission("actual ground-model month bounds"))
    }
}

/// Source manager owns models; four indices are WeatherManager non-owning pointers.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceGroundTemperatureModels {
    pub(crate) models: Vec<SourceSiteGroundTemperatureModel>,
    pub(crate) building_surface: Option<usize>,
    pub(crate) fc_factor_method: Option<usize>,
    pub(crate) shallow: Option<usize>,
    pub(crate) deep: Option<usize>,
    /// Actual newly allocated model before manager push, retained on failure.
    pub(crate) factory_local: Option<SourceSiteGroundTemperatureModel>,
}
impl SourceGroundTemperatureModels {
    pub(crate) fn get_ground_temperatures(
        &mut self,
        owners: &mut SourceGroundTemperatureOwners<'_, '_>,
    ) -> Result<(), Error> {
        // Genuine WeatherManager6919-6931, preserving partial pointer writes.
        self.building_surface =
            Some(self.get_or_initialize(SiteGroundModelKind::BuildingSurface, "", owners)?);
        self.fc_factor_method =
            Some(self.get_or_initialize(SiteGroundModelKind::FCFactorMethod, "", owners)?);
        self.shallow = Some(self.get_or_initialize(SiteGroundModelKind::Shallow, "", owners)?);
        self.deep = Some(self.get_or_initialize(SiteGroundModelKind::Deep, "", owners)?);
        Ok(())
    }
    fn get_or_initialize(
        &mut self,
        kind: SiteGroundModelKind,
        name: &str,
        owners: &mut SourceGroundTemperatureOwners<'_, '_>,
    ) -> Result<usize, Error> {
        for (index, model) in self.models.iter().enumerate() {
            if model.model_type == kind.model_type() && model.name == name {
                return Ok(index);
            }
        }
        self.factory_local = Some(SourceSiteGroundTemperatureModel::construct(kind));
        let model = self
            .factory_local
            .as_mut()
            .ok_or(Error::Unavailable("actual new ground model"))?;
        let count = owners.input.get_num_objects_found(kind.family_upper());
        model.model_type = kind.model_type();
        model.name = name.into();
        let success = factory::initialize(model, kind, count, owners)?;
        if success {
            let index = self.models.len();
            self.models.push(
                self.factory_local
                    .take()
                    .ok_or(Error::Unavailable("actual ground model manager push"))?,
            );
            return Ok(index);
        }
        match owners.backend.show_fatal_error(&format!(
            "{}--Errors getting input for ground temperature model",
            kind.family()
        )) {
            Err(error) => Err(Error::Fatal(error)),
            Ok(never) => match never {},
        }
    }
    fn monthly(&mut self, pointer: Option<usize>, month: i32) -> Result<f64, WeatherDayError> {
        let model = pointer
            .and_then(|p| self.models.get_mut(p))
            .ok_or_else(|| {
                WeatherDayError::admission("actual Weather ground-model pointer unavailable")
            })?;
        model.at_month(month)
    }
    /// Actual2094-2102. No None/default result substitutes for a model read.
    pub(crate) fn update_current(
        &mut self,
        environment: &mut WeatherEnvironmentState,
    ) -> Result<(), WeatherDayError> {
        let month = environment.month;
        environment.ground_temp[0] = self.monthly(self.building_surface, month)?;
        environment.ground_temp_kelvin = environment.ground_temp[0] + 273.15;
        environment.ground_temp[3] = self.monthly(self.fc_factor_method, month)?;
        environment.ground_temp[1] = self.monthly(self.shallow, month)?;
        environment.ground_temp[2] = self.monthly(self.deep, month)?;
        Ok(())
    }
}
impl WeatherSession {
    pub(crate) fn initialize_source_ground_temperatures(
        &mut self,
        raw: &SelectedHeatBalanceInput,
        input: &mut SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), Error> {
        let mut owners = SourceGroundTemperatureOwners {
            raw,
            input,
            shortcut,
            environment: &mut self.state.environment,
            header: self.cursor.header(),
            backend,
            eio,
        };
        self.state
            .ground_temperature_models
            .get_ground_temperatures(&mut owners)
    }
}
