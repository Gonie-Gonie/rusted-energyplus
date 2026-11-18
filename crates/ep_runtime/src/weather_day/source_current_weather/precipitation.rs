//! Actual UpdatePrecipitation947–990 on the sole WaterManager/DataWater and EcoRoof owners.
use super::{Error, SourceRainfallMode, SourceWaterManager, month_index};
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};

/// Sole persistent EcoRoofManagerData, shared by weather, water and heat balance.
/// Explicit header defaults and C++ value-initialized scalar zeros are genuine
/// constructor state; the independent precipitation marker records producer entry.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceEcoRoofData {
    pub(crate) cum_runoff: f64,
    pub(crate) cum_et: f64,
    pub(crate) cum_precip: f64,
    pub(crate) cum_irrigation: f64,
    pub(crate) monthly_irrigation: [f64; 12],
    pub(crate) current_runoff: f64,
    pub(crate) current_et: f64,
    pub(crate) current_precipitation: f64,
    pub(crate) current_irrigation: f64,
    pub(crate) tfold: f64,
    pub(crate) tgold: f64,
    pub(crate) eco_roof_begin_flag: bool,
    pub(crate) calc_eco_roof_my_envrn_flag: bool,
    pub(crate) first_eco_surf: i32,
    pub(crate) quick_conduction_surf: bool,
    pub(crate) lai: f64,
    pub(crate) epsilonf: f64,
    pub(crate) epsilong: f64,
    pub(crate) alphag: f64,
    pub(crate) alphaf: f64,
    pub(crate) e0: f64,
    pub(crate) rh: f64,
    pub(crate) pa: f64,
    pub(crate) tg: f64,
    pub(crate) tf: f64,
    pub(crate) zf: f64,
    pub(crate) moisture: f64,
    pub(crate) moisture_residual: f64,
    pub(crate) moisture_max: f64,
    pub(crate) mean_root_moisture: f64,
    pub(crate) soil_thickness: f64,
    pub(crate) stomatal_resistance_min: f64,
    pub(crate) f3: f64,
    pub(crate) zog: f64,
    pub(crate) za: f64,
    pub(crate) lf: f64,
    pub(crate) vfluxf: f64,
    pub(crate) qsoil: f64,
    pub(crate) sheatf: f64,
    pub(crate) sensiblef: f64,
    pub(crate) sheatg: f64,
    pub(crate) sensibleg: f64,
    pub(crate) lg: f64,
    pub(crate) vfluxg: f64,
    pub(crate) top_depth: f64,
    pub(crate) root_depth: f64,
    pub(crate) time_step_zone_sec: f64,
    pub(crate) dry_cond: f64,
    pub(crate) dry_dens: f64,
    pub(crate) dry_absorp: f64,
    pub(crate) dry_spec_heat: f64,
    pub(crate) update_begin_flag: bool,
    pub(crate) capillary_potential_top: f64,
    pub(crate) capillary_potential_root: f64,
    pub(crate) soil_hydro_conductivity_top: f64,
    pub(crate) soil_hydro_conductivity_root: f64,
    pub(crate) soil_conductivity_ave_top: f64,
    pub(crate) soil_conductivity_ave_root: f64,
    pub(crate) relative_soil_saturation_top: f64,
    pub(crate) relative_soil_saturation_root: f64,
    pub(crate) test_moisture: f64,
    pub(crate) err_index: i32,
    pub(crate) precipitation_producer_written: bool,
}
impl Default for SourceEcoRoofData {
    fn default() -> Self {
        Self {
            cum_runoff: 0.0,
            cum_et: 0.0,
            cum_precip: 0.0,
            cum_irrigation: 0.0,
            monthly_irrigation: [0.0; 12],
            current_runoff: 0.0,
            current_et: 0.0,
            current_precipitation: 0.0,
            current_irrigation: 0.0,
            tfold: 0.0,
            tgold: 0.0,
            eco_roof_begin_flag: true,
            calc_eco_roof_my_envrn_flag: true,
            first_eco_surf: 0,
            quick_conduction_surf: false,
            lai: 0.2,
            epsilonf: 0.95,
            epsilong: 0.95,
            alphag: 0.3,
            alphaf: 0.2,
            e0: 2.0,
            rh: 50.0,
            pa: 101325.0,
            tg: 10.0,
            tf: 10.0,
            zf: 0.2,
            moisture: 0.0,
            moisture_residual: 0.05,
            moisture_max: 0.5,
            mean_root_moisture: 0.0,
            soil_thickness: 0.2,
            stomatal_resistance_min: 0.0,
            f3: 1.0,
            zog: 0.001,
            za: 2.0,
            lf: 0.0,
            vfluxf: 0.0,
            qsoil: 0.0,
            sheatf: 0.0,
            sensiblef: 0.0,
            sheatg: 0.0,
            sensibleg: 0.0,
            lg: 0.0,
            vfluxg: 0.0,
            top_depth: 0.0,
            root_depth: 0.0,
            time_step_zone_sec: 0.0,
            dry_cond: 0.0,
            dry_dens: 0.0,
            dry_absorp: 0.0,
            dry_spec_heat: 0.0,
            update_begin_flag: true,
            capillary_potential_top: -3.8997,
            capillary_potential_root: -3.8997,
            soil_hydro_conductivity_top: 8.72e-06,
            soil_hydro_conductivity_root: 8.72e-06,
            soil_conductivity_ave_top: 8.72e-06,
            soil_conductivity_ave_root: 8.72e-06,
            relative_soil_saturation_top: 0.0,
            relative_soil_saturation_root: 0.0,
            test_moisture: 0.15,
            err_index: 0,
            precipitation_producer_written: false,
        }
    }
}
impl SourceWaterManager {
    pub(crate) fn update_source_precipitation(
        &mut self,
        clock: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        schedules: &SelectedScheduleManager,
        eco_roof: &mut SourceEcoRoofData,
    ) -> Result<(), Error> {
        if self.data.rainfall.mode == SourceRainfallMode::RainSchedDesign {
            let index = self.data.rainfall.rain_schedule.ok_or(Error::Unavailable(
                "actual UpdatePrecipitation rainSched pointer962",
            ))?;
            let rate = schedules.source_current_schedule_value(index)?;
            let scale = if self.data.rainfall.nominal_annual_rain > 0.0 {
                self.data.rainfall.design_annual_rain / self.data.rainfall.nominal_annual_rain
            } else {
                0.0
            };
            self.data.rainfall.current_rate = rate * scale / 3600.0;
        } else if environment.current_weather.liquid_precipitation > 0.0 {
            self.data.rainfall.current_rate =
                environment.current_weather.liquid_precipitation / clock.time_step_zone_sec;
        } else {
            self.data.rainfall.current_rate = 0.0;
        }
        self.data.rainfall.current_amount =
            self.data.rainfall.current_rate * clock.time_step_zone_sec;
        eco_roof.current_precipitation = self.data.rainfall.current_amount;
        eco_roof.precipitation_producer_written = true;
        if self.data.rainfall.mode == SourceRainfallMode::RainSchedDesign
            && environment.run_period_environment
            && !clock.warmup_flag
        {
            let month = month_index(environment.month)?;
            self.data.rainfall.monthly_total_precipitation_site[month] +=
                self.data.rainfall.current_amount * 1000.0;
        }
        Ok(())
    }
}
