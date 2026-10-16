//! Actual WaterManager zero-object getter, manager and timestep initialization.
//! DataWater and WaterManager remain distinct from WaterUse and water-heater owners.
mod rainfall_report;
use super::Error;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::selected_schedule_manager::SourceScheduleIndex;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::weather::day::WeatherGlobalState;
use std::convert::Infallible;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceRainfallMode {
    Invalid = -1,
    None,
    RainSchedDesign,
    EpwPrecipitation,
    Num,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceIrrigationMode {
    Invalid = -1,
    SchedDesign,
    SmartSched,
    Num,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSiteRainfall {
    pub(crate) mode: SourceRainfallMode,
    pub(crate) design_annual_rain: f64,
    pub(crate) rain_schedule: Option<SourceScheduleIndex>,
    pub(crate) nominal_annual_rain: f64,
    pub(crate) current_rate: f64,
    pub(crate) current_amount: f64,
    pub(crate) monthly_total_precipitation_weather: [f64; 12],
    pub(crate) monthly_total_precipitation_site: [f64; 12],
    pub(crate) number_rainy_hours_weather: [i32; 12],
}
impl Default for SourceSiteRainfall {
    fn default() -> Self {
        Self {
            mode: SourceRainfallMode::None,
            design_annual_rain: 0.0,
            rain_schedule: None,
            nominal_annual_rain: 0.0,
            current_rate: 0.0,
            current_amount: 0.0,
            monthly_total_precipitation_weather: [0.0; 12],
            monthly_total_precipitation_site: [0.0; 12],
            number_rainy_hours_weather: [0; 12],
        }
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceWaterIrrigation {
    pub(crate) mode: SourceIrrigationMode,
    pub(crate) schedule: Option<SourceScheduleIndex>,
    pub(crate) scheduled_amount: f64,
    pub(crate) actual_amount: f64,
    pub(crate) threshold: f64,
}
impl Default for SourceWaterIrrigation {
    fn default() -> Self {
        Self {
            mode: SourceIrrigationMode::Invalid,
            schedule: None,
            scheduled_amount: 0.0,
            actual_amount: 0.0,
            threshold: 0.4,
        }
    }
}
/// Actual DataWaterData default constructors and unallocated arrays.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceWaterData {
    pub(crate) rainfall: SourceSiteRainfall,
    pub(crate) irrigation: SourceWaterIrrigation,
    pub(crate) storage: Option<Vec<Infallible>>,
    pub(crate) rain_collectors: Option<Vec<Infallible>>,
    pub(crate) groundwater_wells: Option<Vec<Infallible>>,
    pub(crate) number_storage_tanks: usize,
    pub(crate) number_rain_collectors: usize,
    pub(crate) number_groundwater_wells: usize,
    pub(crate) number_site_rainfall: usize,
    pub(crate) any_water_systems: bool,
    pub(crate) input_called: bool,
    pub(crate) any_irrigation: bool,
    pub(crate) precipitation_overwritten_by_rain_flag: i32,
}
/// Sole WaterManagerData flags; DataWater storage is retained beside them once.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceWaterManager {
    pub(crate) data: SourceWaterData,
    pub(crate) one_time_flag: bool,
    pub(crate) get_input_flag: bool,
    pub(crate) environment_flag: bool,
    pub(crate) warmup_flag: bool,
    pub(crate) tank_demand_check_flag: bool,
    pub(crate) overflow_water_temperature: f64,
}
impl Default for SourceWaterManager {
    fn default() -> Self {
        Self {
            data: SourceWaterData::default(),
            one_time_flag: true,
            get_input_flag: true,
            environment_flag: true,
            warmup_flag: false,
            tank_demand_check_flag: true,
            overflow_water_temperature: 0.0,
        }
    }
}
impl SourceWaterManager {
    pub(crate) fn manage_source_water(
        &mut self,
        input: &mut SelectedInputProcessor,
    ) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_source_input(input)?;
            self.get_input_flag = false;
        }
        if !self.data.any_water_systems {
            return Ok(());
        }
        if (1..=self.data.number_storage_tanks).next().is_some() {
            return Err(Error::Unavailable("actual CalcWaterStorageTank155"));
        }
        if (1..=self.data.number_rain_collectors).next().is_some() {
            return Err(Error::Unavailable("actual CalcRainCollector159"));
        }
        if (1..=self.data.number_groundwater_wells).next().is_some() {
            return Err(Error::Unavailable("actual CalcGroundwaterWell163"));
        }
        if (1..=self.data.number_storage_tanks).next().is_some() {
            return Err(Error::Unavailable("actual second CalcWaterStorageTank168"));
        }
        Ok(())
    }
    pub(crate) fn manage_source_water_inits(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        hvac: &SelectedHvacGlobals,
    ) -> Result<(), Error> {
        if !self.data.any_water_systems {
            return Ok(());
        }
        self.update_source_water_manager(globals, clock)?;
        self.update_source_irrigation(hvac)
    }
    fn get_source_input(&mut self, input: &mut SelectedInputProcessor) -> Result<(), Error> {
        if self.one_time_flag && !self.data.input_called {
            self.data.rainfall.mode = SourceRainfallMode::None;
            let counts = input
                .get_object_def_max_args("WaterUse:Storage")
                .map_err(|_| Error::Unavailable("actual WaterUse:Storage definition213"))?;
            let mut max_numbers = counts.numerics;
            let mut max_alphas = counts.alphas;
            for family in [
                "WaterUse:RainCollector",
                "WaterUse:Well",
                "Site:Precipitation",
                "RoofIrrigation",
            ] {
                let counts = input
                    .get_object_def_max_args(family)
                    .map_err(|_| Error::Unavailable("actual WaterManager definition216-230"))?;
                max_numbers = max_numbers.max(counts.numerics);
                max_alphas = max_alphas.max(counts.alphas);
            }
            // These are function-local scratch arrays, not dataIPShortCut.
            let alpha_field_names = vec![String::new(); max_alphas];
            let alpha_args = vec![String::new(); max_alphas];
            let alpha_blanks = vec![false; max_alphas];
            let numeric_field_names = vec![String::new(); max_numbers];
            let numeric_args = vec![0.0; max_numbers];
            let numeric_blanks = vec![false; max_numbers];
            self.one_time_flag = false;
            self.data.number_storage_tanks = input.get_num_objects_found("WaterUse:Storage");
            if self.data.number_storage_tanks > 0 {
                self.data.any_water_systems = true;
                if self.data.storage.is_none() {
                    return Err(Error::Unavailable("actual WaterStorage allocation251"));
                }
                return Err(Error::Unavailable("actual WaterStorage getObjectItem255"));
            }
            self.data.number_rain_collectors =
                input.get_num_objects_found("WaterUse:RainCollector");
            if self.data.number_rain_collectors > 0 {
                if self.data.rain_collectors.is_none() {
                    return Err(Error::Unavailable("actual RainCollector allocation396"));
                }
                self.data.any_water_systems = true;
                if self.data.rainfall.mode == SourceRainfallMode::None {
                    self.data.rainfall.mode = SourceRainfallMode::EpwPrecipitation;
                }
                return Err(Error::Unavailable("actual RainCollector getObjectItem406"));
            }
            self.data.number_groundwater_wells = input.get_num_objects_found("WaterUse:Well");
            if self.data.number_groundwater_wells > 0 {
                self.data.any_water_systems = true;
                return Err(Error::Unavailable("actual GroundwaterWell allocation518"));
            }
            // Connecting overflow/supply tanks at613-654 is a zero-length storage loop.
            self.data.number_site_rainfall = input.get_num_objects_found("Site:Precipitation");
            if self.data.number_site_rainfall > 1 {
                return Err(Error::Unavailable(
                    "actual Site:Precipitation severe diagnostic662",
                ));
            }
            if self.data.number_site_rainfall == 1 {
                self.data.any_water_systems = true;
                return Err(Error::Unavailable(
                    "actual Site:Precipitation getObjectItem669",
                ));
            }
            let number_irrigation = input.get_num_objects_found("RoofIrrigation");
            if number_irrigation > 1 {
                return Err(Error::Unavailable(
                    "actual RoofIrrigation severe diagnostic701",
                ));
            }
            if number_irrigation == 1 {
                self.data.any_irrigation = true;
                return Err(Error::Unavailable("actual RoofIrrigation getObjectItem708"));
            }
            if self.data.rainfall.mode == SourceRainfallMode::EpwPrecipitation {
                return Err(Error::Unavailable("actual precipitation warning755"));
            }
            // Native760 is unconditional even when all five object counts are zero.
            self.data.any_water_systems = true;
            self.data.input_called = true;
            self.one_time_flag = false;
            drop(alpha_field_names);
            drop(alpha_args);
            drop(alpha_blanks);
            drop(numeric_field_names);
            drop(numeric_args);
            drop(numeric_blanks);
            // Actual storage/irrigation/rain/well output loops all have zero iterations.
        }
        Ok(())
    }
    fn update_source_water_manager(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
    ) -> Result<(), Error> {
        if clock.begin_envrn_flag && self.environment_flag {
            if (1..=self.data.number_storage_tanks).next().is_some() {
                return Err(Error::Unavailable("actual tank initial-volume writes1598"));
            }
            if !globals.doing_sizing && !globals.kickoff_simulation && self.tank_demand_check_flag {
                if self.data.number_storage_tanks > 0 {
                    return Err(Error::Unavailable("actual tank-demand warning1606"));
                }
                self.tank_demand_check_flag = false;
            }
            self.environment_flag = false;
            self.warmup_flag = true;
        }
        if !clock.begin_envrn_flag {
            self.environment_flag = true;
        }
        if self.warmup_flag && !clock.warmup_flag {
            if (1..=self.data.number_storage_tanks).next().is_some() {
                return Err(Error::Unavailable("actual after-warmup tank writes1624"));
            }
            self.warmup_flag = false;
        }
        if (1..=self.data.number_storage_tanks).next().is_some() {
            return Err(Error::Unavailable("actual new-step tank writes1634"));
        }
        if (1..=self.data.number_rain_collectors).next().is_some() {
            return Err(Error::Unavailable("actual rain collector reset1660"));
        }
        if (1..=self.data.number_groundwater_wells).next().is_some() {
            return Err(Error::Unavailable("actual well reset1665"));
        }
        Ok(())
    }
    fn update_source_irrigation(&mut self, hvac: &SelectedHvacGlobals) -> Result<(), Error> {
        let _time_step_sys = hvac.time_step_sys;
        self.data.irrigation.scheduled_amount = 0.0;
        if self.data.irrigation.mode == SourceIrrigationMode::SchedDesign {
            return Err(Error::Unavailable(
                "actual scheduled irrigation schedule read1010",
            ));
        } else if self.data.irrigation.mode == SourceIrrigationMode::SmartSched {
            return Err(Error::Unavailable(
                "actual smart irrigation schedule read1014",
            ));
        }
        Ok(())
    }
}
