//! Literal FaultsManager203-335 zero-input closure; positive allocation stops honestly.
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use std::convert::Infallible;
const FAULT_FAMILIES: [&str; 16] = [
    "FaultModel:TemperatureSensorOffset:OutdoorAir",
    "FaultModel:HumiditySensorOffset:OutdoorAir",
    "FaultModel:EnthalpySensorOffset:OutdoorAir",
    "FaultModel:TemperatureSensorOffset:ReturnAir",
    "FaultModel:EnthalpySensorOffset:ReturnAir",
    "FaultModel:Fouling:Coil",
    "FaultModel:ThermostatOffset",
    "FaultModel:HumidistatOffset",
    "FaultModel:Fouling:AirFilter",
    "FaultModel:TemperatureSensorOffset:ChillerSupplyWater",
    "FaultModel:TemperatureSensorOffset:CondenserSupplyWater",
    "FaultModel:Fouling:CoolingTower",
    "FaultModel:TemperatureSensorOffset:CoilSupplyAir",
    "FaultModel:Fouling:Boiler",
    "FaultModel:Fouling:Chiller",
    "FaultModel:Fouling:EvaporativeCooler",
];
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceFaultsManager {
    run_fault_manager_once_flag: bool,
    errors_found: bool,
    any_faults_in_model: bool,
    number_faults: i32,
    number_faulty_economizer: i32,
    number_fouled_coil: i32,
    number_faulty_thermostat: i32,
    number_faulty_humidistat: i32,
    number_faulty_air_filter: i32,
    number_faulty_chiller_swt_sensor: i32,
    number_faulty_condenser_swt_sensor: i32,
    number_faulty_tower_fouling: i32,
    number_faulty_coil_sat_sensor: i32,
    number_faulty_boiler_fouling: i32,
    number_faulty_chiller_fouling: i32,
    number_faulty_evap_cooler_fouling: i32,
    faults_economizer: Option<Vec<Infallible>>,
    fouled_coils: Option<Vec<Infallible>>,
    faults_thermostat_offset: Option<Vec<Infallible>>,
    faults_humidistat_offset: Option<Vec<Infallible>>,
    faults_fouled_air_filters: Option<Vec<Infallible>>,
    faults_chiller_swt_sensor: Option<Vec<Infallible>>,
    faults_condenser_swt_sensor: Option<Vec<Infallible>>,
    faults_tower_fouling: Option<Vec<Infallible>>,
    faults_coil_sat_sensor: Option<Vec<Infallible>>,
    faults_boiler_fouling: Option<Vec<Infallible>>,
    faults_chiller_fouling: Option<Vec<Infallible>>,
    faults_evap_cooler_fouling: Option<Vec<Infallible>>,
}
#[derive(Debug)]
pub(crate) enum SourceFaultsError {
    CountDomain,
    PositiveAllocation(&'static str),
}
impl std::fmt::Display for SourceFaultsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CountDomain => {
                f.write_str("FaultsManager Native signed-count domain unavailable")
            }
            Self::PositiveAllocation(member) => {
                write!(f, "FaultsManager positive {member} allocation unavailable")
            }
        }
    }
}
impl std::error::Error for SourceFaultsError {}
impl SourceFaultsManager {
    pub(crate) fn check_and_read_source_faults(
        &mut self,
        input: &SelectedInputProcessor,
    ) -> Result<(), SourceFaultsError> {
        // Native local constructors230-236 precede even the once-flag return.
        // None represents unread, uninitialized numeric slots, never seeded values.
        let _locals = (
            vec![String::new(); 10],
            vec![false; 10],
            vec![false; 10],
            vec![String::new(); 10],
            vec![String::new(); 10],
            vec![None::<f64>; 10],
            String::new(),
        );
        if self.run_fault_manager_once_flag {
            return Ok(());
        }
        self.number_faults = 0;
        self.number_faulty_economizer = 0;
        for (i, family) in FAULT_FAMILIES.iter().enumerate() {
            let count = i32::try_from(input.get_num_objects_found(family))
                .map_err(|_| SourceFaultsError::CountDomain)?;
            self.number_faults = self
                .number_faults
                .checked_add(count)
                .ok_or(SourceFaultsError::CountDomain)?;
            match i {
                0..=4 => {
                    self.number_faulty_economizer = self
                        .number_faulty_economizer
                        .checked_add(count)
                        .ok_or(SourceFaultsError::CountDomain)?
                }
                5 => self.number_fouled_coil = count,
                6 => self.number_faulty_thermostat = count,
                7 => self.number_faulty_humidistat = count,
                8 => self.number_faulty_air_filter = count,
                9 => self.number_faulty_chiller_swt_sensor = count,
                10 => self.number_faulty_condenser_swt_sensor = count,
                11 => self.number_faulty_tower_fouling = count,
                12 => self.number_faulty_coil_sat_sensor = count,
                13 => self.number_faulty_boiler_fouling = count,
                14 => self.number_faulty_chiller_fouling = count,
                15 => self.number_faulty_evap_cooler_fouling = count,
                _ => return Err(SourceFaultsError::CountDomain),
            }
        }
        self.any_faults_in_model = self.number_faults > 0;
        if !self.any_faults_in_model {
            self.run_fault_manager_once_flag = true;
            return Ok(());
        }
        // Stop at the first real unavailable Native record allocation, not a fake row.
        if self.number_faulty_economizer > 0 {
            return Err(SourceFaultsError::PositiveAllocation("FaultsEconomizer301"));
        }
        if self.number_fouled_coil > 0 {
            return Err(SourceFaultsError::PositiveAllocation("FouledCoils304"));
        }
        if self.number_faulty_thermostat > 0 {
            return Err(SourceFaultsError::PositiveAllocation(
                "FaultsThermostatOffset307",
            ));
        }
        if self.number_faulty_humidistat > 0 {
            return Err(SourceFaultsError::PositiveAllocation(
                "FaultsHumidistatOffset310",
            ));
        }
        if self.number_faulty_air_filter > 0 {
            return Err(SourceFaultsError::PositiveAllocation(
                "FaultsFouledAirFilters313",
            ));
        }
        if self.number_faulty_chiller_swt_sensor > 0 {
            return Err(SourceFaultsError::PositiveAllocation(
                "FaultsChillerSWTSensor316",
            ));
        }
        if self.number_faulty_condenser_swt_sensor > 0 {
            return Err(SourceFaultsError::PositiveAllocation(
                "FaultsCondenserSWTSensor319",
            ));
        }
        if self.number_faulty_tower_fouling > 0 {
            return Err(SourceFaultsError::PositiveAllocation(
                "FaultsTowerFouling322",
            ));
        }
        if self.number_faulty_coil_sat_sensor > 0 {
            return Err(SourceFaultsError::PositiveAllocation(
                "FaultsCoilSATSensor325",
            ));
        }
        if self.number_faulty_boiler_fouling > 0 {
            return Err(SourceFaultsError::PositiveAllocation(
                "FaultsBoilerFouling328",
            ));
        }
        if self.number_faulty_chiller_fouling > 0 {
            return Err(SourceFaultsError::PositiveAllocation(
                "FaultsChillerFouling331",
            ));
        }
        if self.number_faulty_evap_cooler_fouling > 0 {
            return Err(SourceFaultsError::PositiveAllocation(
                "FaultsEvapCoolerFouling334",
            ));
        }
        // Positive total necessarily reached one of those twelve allocations.
        Err(SourceFaultsError::CountDomain)
    }
}
