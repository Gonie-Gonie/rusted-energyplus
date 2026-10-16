//! Genuine empty-device branches of the eight InternalHeatGains module calls.
//! Counts are retained module constructor/input fields, never typed-model guesses.
mod refrigeration;
pub(crate) mod water_use;

use super::ctf_initialization_context::internal_heat_gains_input::{
    SelectedInternalHeatGains, SourceGainModule, SourceGainModuleProvider,
};
use super::error_backend::SourceErrorBackend;
use super::facility_electric_service::FacilityElectricService;
use super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use super::selected_heat_balance_input::{
    HeatBalanceInputError as E, zone::SelectedZoneInputOwners,
};
use super::selected_input_processor::SelectedInputProcessor;
use crate::weather::day::WeatherGlobalState;

pub(crate) use refrigeration::SourceRefrigerationGainModule;

/// Selected module members read/written by these actual zero-device branches.
/// Unreached positive-device fields and static-local WaterUse flags are unpaired.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceGainModules {
    /// WaterThermalTanks.hh1025/1039, independent constructor fields.
    pub(crate) number_water_thermal_tanks: i32,
    pub(crate) get_water_thermal_tank_input: bool,
    /// PipeHeatTransfer.hh267, WaterUse.hh252, FuelCell.hh651, MicroCHP.hh270.
    pub(crate) number_pipe_heat_transfer: i32,
    pub(crate) number_water_equipment: i32,
    /// Remaining fields of SAME WaterUseData; existing numWaterEquipment cell stays above.
    pub(crate) water_use: water_use::SourceWaterUseModule,
    pub(crate) number_fuel_cell_generators: i32,
    pub(crate) number_micro_chps: i32,
    /// Actual TDDPipe allocation/cardinality only; positive elements unported.
    pub(crate) tdd_pipe: Option<Vec<()>>,
    /// Same DataDaylightingDevicesData.Shelf, constructor-unallocated; positive rows unported.
    pub(crate) shelf: Option<Vec<()>>,
    pub(crate) refrigeration: SourceRefrigerationGainModule,
}
impl Default for SourceGainModules {
    fn default() -> Self {
        Self {
            number_water_thermal_tanks: 0,
            get_water_thermal_tank_input: true,
            number_pipe_heat_transfer: 0,
            number_water_equipment: 0,
            water_use: Default::default(),
            number_fuel_cell_generators: 0,
            number_micro_chps: 0,
            tdd_pipe: None,
            shelf: None,
            refrigeration: SourceRefrigerationGainModule::default(),
        }
    }
}

/// All fields borrow their sole live owner; facility was made at actual SM222.
pub(crate) struct SourceModuleGainProvider<'a> {
    pub(crate) modules: &'a mut SourceGainModules,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) facility: Option<&'a mut FacilityElectricService>,
}

fn unavailable(message: &'static str) -> E {
    E::Unavailable(message)
}

impl SourceModuleGainProvider<'_> {
    fn initialize_electric_facility(&mut self, begin_environment: bool) -> Result<(), E> {
        let facility = self
            .facility
            .as_deref_mut()
            .ok_or_else(|| unavailable("same actual facilityElectricServiceObj"))?;
        if facility.new_environment_internal_gains_flag && begin_environment {
            facility.reinitialize_source_zone_gains()?;
            facility.new_environment_internal_gains_flag = false;
        }
        if !begin_environment {
            facility.new_environment_internal_gains_flag = true;
        }
        Ok(())
    }
}

impl SourceGainModuleProvider for SourceModuleGainProvider<'_> {
    fn run_module(
        &mut self,
        module: SourceGainModule,
        _gains: &mut SelectedInternalHeatGains,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
        _zones: &mut SelectedZoneInputOwners,
        _backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), E> {
        match module {
            SourceGainModule::WaterThermalTanks => {
                if self.modules.number_water_thermal_tanks == 0 {
                    if !globals.doing_sizing {
                        return Ok(());
                    }
                    if self.modules.get_water_thermal_tank_input {
                        return Err(unavailable("actual GetWaterThermalTankInput during sizing"));
                    }
                    if self.modules.number_water_thermal_tanks == 0 {
                        return Ok(());
                    }
                }
                Err(unavailable("positive tank environment/zone-gain owners"))
            }
            SourceGainModule::PipeHeatTransfer => {
                if self.modules.number_pipe_heat_transfer == 0 {
                    return Ok(());
                }
                Err(unavailable("positive pipe environment/zone-gain owners"))
            }
            SourceGainModule::WaterUse => {
                if self.modules.number_water_equipment == 0 {
                    return Ok(());
                }
                Err(unavailable(
                    "positive WaterUse static environment/zone-gain owners",
                ))
            }
            SourceGainModule::FuelCell => {
                if self.modules.number_fuel_cell_generators == 0 {
                    return Ok(());
                }
                Err(unavailable(
                    "positive FuelCell environment/zone-gain owners",
                ))
            }
            SourceGainModule::MicroChp => {
                if self.modules.number_micro_chps == 0 {
                    return Ok(());
                }
                Err(unavailable(
                    "positive MicroCHP environment/zone-gain owners",
                ))
            }
            SourceGainModule::ElectricFacility => {
                self.initialize_electric_facility(clock.begin_envrn_flag)
            }
            SourceGainModule::TubularDaylighting => {
                if self.modules.tdd_pipe.as_ref().map_or(0, Vec::len) == 0 {
                    return Ok(());
                }
                Err(unavailable("positive TDD environment/zone-gain owners"))
            }
            SourceGainModule::Refrigeration => self
                .modules
                .refrigeration
                .figure_zone_gains(self.input, clock, globals),
        }
    }
}
