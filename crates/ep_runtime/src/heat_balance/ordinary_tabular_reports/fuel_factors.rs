//! Actual GetInputFuelAndPollutionFactors2826–3016; all thirteen borrowed calls.
use super::*;
use crate::heat_balance::ordinary_pollution::SourcePollutionFuel;
impl SourceTabularReports {
    pub(super) fn get_source_fuel_and_pollution_factors(
        &mut self,
        o: &mut SourceTabularInputOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.input_state.source_factor_electric = 3.167;
        self.input_state.source_factor_natural_gas = 1.084;
        self.input_state.source_factor_district_heating_steam = 1.20;
        self.input_state.source_factor_gasoline = 1.05;
        self.input_state.source_factor_diesel = 1.05;
        self.input_state.source_factor_coal = 1.05;
        self.input_state.source_factor_fuel_oil1 = 1.05;
        self.input_state.source_factor_fuel_oil2 = 1.05;
        self.input_state.source_factor_propane = 1.05;
        self.input_state.source_factor_other_fuel1 = 1.0;
        self.input_state.source_factor_other_fuel2 = 1.0;
        self.input_state.efficiency_district_cooling = 3.0;
        self.input_state.efficiency_district_heating_water = 0.3;
        // Native local outputs are unwritten before the first getter; that getter
        // writes all four before any read. Keep one actual ffSched local across calls.
        let mut fuel_factor_used = false;
        let mut current_source_factor = 0.0;
        let mut schedule_used = false;
        let mut schedule = None;
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::NaturalGas,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.source_factor_natural_gas = current_source_factor;
            self.input_state.fuel_factors_used[1] = true;
            self.input_state.ff_used[1] = true;
        }
        self.input_state.source_factors[1] = current_source_factor;
        if schedule_used {
            self.input_state.fuel_factor_schedules_used = true;
            self.input_state.ff_schedule_used[1] = true;
            self.input_state.ff_schedules[1] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::FuelOil2,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.source_factor_fuel_oil2 = current_source_factor;
            self.input_state.fuel_factors_used[6] = true;
            self.input_state.ff_used[10] = true;
        }
        self.input_state.source_factors[10] = current_source_factor;
        if schedule_used {
            self.input_state.fuel_factor_schedules_used = true;
            self.input_state.ff_schedule_used[10] = true;
            self.input_state.ff_schedules[10] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::FuelOil1,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.source_factor_fuel_oil1 = current_source_factor;
            self.input_state.fuel_factors_used[5] = true;
            self.input_state.ff_used[9] = true;
        }
        self.input_state.source_factors[9] = current_source_factor;
        if schedule_used {
            self.input_state.fuel_factor_schedules_used = true;
            self.input_state.ff_schedule_used[9] = true;
            self.input_state.ff_schedules[9] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::Coal,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.source_factor_coal = current_source_factor;
            self.input_state.fuel_factors_used[4] = true;
            self.input_state.ff_used[8] = true;
        }
        self.input_state.source_factors[8] = current_source_factor;
        if schedule_used {
            self.input_state.fuel_factor_schedules_used = true;
            self.input_state.ff_schedule_used[8] = true;
            self.input_state.ff_schedules[8] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::Electricity,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.source_factor_electric = current_source_factor;
            self.input_state.fuel_factors_used[0] = true;
            self.input_state.ff_used[0] = true;
        }
        self.input_state.source_factors[0] = current_source_factor;
        if schedule_used {
            self.input_state.fuel_factor_schedules_used = true;
            self.input_state.ff_schedule_used[0] = true;
            self.input_state.ff_schedules[0] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::Gasoline,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.source_factor_gasoline = current_source_factor;
            self.input_state.fuel_factors_used[2] = true;
            self.input_state.ff_used[5] = true;
        }
        self.input_state.source_factors[5] = current_source_factor;
        if schedule_used {
            self.input_state.fuel_factor_schedules_used = true;
            self.input_state.ff_schedule_used[5] = true;
            self.input_state.ff_schedules[5] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::Propane,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.source_factor_propane = current_source_factor;
            self.input_state.fuel_factors_used[7] = true;
            self.input_state.ff_used[11] = true;
        }
        self.input_state.source_factors[11] = current_source_factor;
        if schedule_used {
            self.input_state.fuel_factor_schedules_used = true;
            self.input_state.ff_schedule_used[11] = true;
            self.input_state.ff_schedules[11] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::Diesel,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.source_factor_diesel = current_source_factor;
            self.input_state.fuel_factors_used[3] = true;
            self.input_state.ff_used[7] = true;
        }
        self.input_state.source_factors[7] = current_source_factor;
        if schedule_used {
            self.input_state.fuel_factor_schedules_used = true;
            self.input_state.ff_schedule_used[7] = true;
            self.input_state.ff_schedules[7] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::DistrictCooling,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.ff_used[2] = true;
        }
        self.input_state.source_factors[2] = current_source_factor;
        if schedule_used {
            self.input_state.ff_schedule_used[2] = true;
            self.input_state.ff_schedules[2] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::DistrictHeatingWater,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.ff_used[3] = true;
        }
        self.input_state.source_factors[3] = current_source_factor;
        if schedule_used {
            self.input_state.ff_schedule_used[3] = true;
            self.input_state.ff_schedules[3] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::DistrictHeatingSteam,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.ff_used[4] = true;
        }
        self.input_state.source_factors[4] = current_source_factor;
        if schedule_used {
            self.input_state.ff_schedule_used[4] = true;
            self.input_state.ff_schedules[4] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::OtherFuel1,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.source_factor_other_fuel1 = current_source_factor;
            self.input_state.fuel_factors_used[10] = true;
            self.input_state.ff_used[12] = true;
        }
        self.input_state.source_factors[12] = current_source_factor;
        if schedule_used {
            self.input_state.fuel_factor_schedules_used = true;
            self.input_state.ff_schedule_used[12] = true;
            self.input_state.ff_schedules[12] = schedule;
        }
        o.pollution.get_source_fuel_factor_info(
            SourcePollutionFuel::OtherFuel2,
            &mut fuel_factor_used,
            &mut current_source_factor,
            &mut schedule_used,
            &mut schedule,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        if fuel_factor_used {
            self.input_state.source_factor_other_fuel2 = current_source_factor;
            self.input_state.fuel_factors_used[11] = true;
            self.input_state.ff_used[13] = true;
        }
        self.input_state.source_factors[13] = current_source_factor;
        if schedule_used {
            self.input_state.fuel_factor_schedules_used = true;
            self.input_state.ff_schedule_used[13] = true;
            self.input_state.ff_schedules[13] = schedule;
        }
        o.pollution.get_source_environmental_impact_factor_info(
            &mut self.input_state.efficiency_district_heating_water,
            &mut self.input_state.efficiency_district_cooling,
            &mut self.input_state.source_factor_district_heating_steam,
            o.input,
            o.shortcut,
            o.backend,
        )?;
        Ok(())
    }
}
