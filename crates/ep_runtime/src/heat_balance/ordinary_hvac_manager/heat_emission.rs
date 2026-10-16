//! Genuine CalcHeatEmissionReport4260-4462 on sole control/device owners.
//! Each positive first-record operation is unavailable; zero traversals are native.
use super::{Error, SourceHvacManager, SourceHvacOwners};
pub(crate) mod state;
use crate::heat_balance::{
    ordinary_equipment::OrdinaryEquipment, selected_gain_modules::SourceGainModules,
    selected_heat_balance_input::SelectedHeatBalanceInputState,
    selected_simulation_startup::SelectedHvacGlobals,
};
use crate::weather::day::WeatherEnvironmentState;
use state::SourceHeatEmissionDeviceModules;
/// Actual disjoint inputs to the sole CalcHeatEmissionReport body.
/// The manager and TAB callers reborrow the same control/module cells per call.
pub(crate) struct SourceHeatEmissionOwners<'a> {
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) control: &'a mut SelectedHeatBalanceInputState,
    pub(crate) equipment: &'a OrdinaryEquipment,
    pub(crate) gain_modules: &'a SourceGainModules,
}
fn positive(number: i32, operation: &'static str) -> Result<(), Error> {
    if number > 0 {
        return Err(Error::Unavailable(operation));
    }
    Ok(())
}
fn array_positive<T>(array: &Option<Vec<T>>, operation: &'static str) -> Result<(), Error> {
    if array.as_ref().is_some_and(|rows| !rows.is_empty()) {
        return Err(Error::Unavailable(operation));
    }
    Ok(())
}
impl SourceHeatEmissionDeviceModules {
    pub(crate) fn calculate_source_heat_emission_report(
        &self,
        o: &mut SourceHeatEmissionOwners<'_>,
    ) -> Result<(), Error> {
        let _time_step_sys_sec = o.hvac.time_step_sys_seconds;
        let _water_vapor_enthalpy = crate::psychrometrics::energyplus_psy_hg_air_fn_w_tdb(
            o.environment.current_weather.out_hum_rat,
            o.environment.current_weather.out_dry_bulb_temp,
        );
        let _water_density = crate::psychrometrics::energyplus_rho_h2o(
            o.environment.current_weather.out_dry_bulb_temp,
        );
        o.control.sys_total_hvac_relief_heat_loss = 0.0;
        o.control.sys_total_hvac_reject_heat_loss = 0.0;
        let modules = self;
        positive(
            modules.number_oa_controllers,
            "actual OAController.RelTotalLossRate first read4278",
        )?;
        // Native exhaust loop deliberately uses iExhSys < NumExhaustAirSystems.
        if *o.equipment.source_number_exhaust_air_systems() > 1 {
            return Err(Error::Unavailable(
                "actual ExhaustAirSystem heat-loss first read4283",
            ));
        }
        array_positive(
            &modules.towers,
            "actual CoolingTower.Qactual first read4288",
        )?;
        array_positive(
            &modules.evaporative_fluid_coolers,
            "actual EvapFluidCooler.Qactual first read4293",
        )?;
        array_positive(
            &modules.fluid_coolers,
            "actual FluidCooler.Qactual first read4296",
        )?;
        positive(
            modules.number_electric_chillers,
            "actual ElectricChiller first row4301",
        )?;
        positive(
            modules.number_engine_chillers,
            "actual EngineDrivenChiller first row4308",
        )?;
        positive(
            modules.number_gas_turbine_chillers,
            "actual GTChiller first row4315",
        )?;
        positive(
            modules.number_constant_cop_chillers,
            "actual ConstCOPChiller first row4322",
        )?;
        array_positive(
            &modules.electric_eir_chillers,
            "actual ElectricEIRChiller first row4330",
        )?;
        array_positive(
            &modules.reformulated_eir_chillers,
            "actual ReformulatedEIRChiller first row4337",
        )?;
        if !modules.boilers.is_empty() {
            return Err(Error::Unavailable(
                "actual Boiler.FuelConsumed first read4347",
            ));
        }
        positive(modules.number_dx_coils, "actual DXCoil first row4352")?;
        positive(
            modules.number_variable_speed_coils,
            "actual VarSpeedCoil first row4374",
        )?;
        positive(
            modules.number_heating_coils,
            "actual HeatingCoil first row4392",
        )?;
        positive(
            modules.number_thermal_storage_coils,
            "actual TESCoil first row4401",
        )?;
        positive(
            o.gain_modules.number_water_thermal_tanks,
            "actual WaterThermalTank first row4414",
        )?;
        positive(modules.number_vrf_condensers, "actual VRF first row4423")?;
        positive(
            o.gain_modules.refrigeration.counts.racks,
            "actual RefrigRack first row4439",
        )?;
        positive(
            o.gain_modules.refrigeration.counts.condensers,
            "actual Refrigeration Condenser first read4453",
        )?;
        positive(
            modules.number_evaporative_coolers,
            "actual EvapCond first row4458",
        )?;
        Ok(())
    }
}

impl SourceHvacManager {
    pub(crate) fn calculate_source_heat_emission_report(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.ancillary
            .heat_emission
            .calculate_source_heat_emission_report(&mut SourceHeatEmissionOwners {
                hvac: o.hvac,
                environment: o.environment,
                control: o.control,
                equipment: o.equipment,
                gain_modules: o.gain_modules,
            })
    }
}
