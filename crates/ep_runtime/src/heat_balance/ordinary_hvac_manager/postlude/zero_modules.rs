//! Actual constructor-owned Ice/EIR/Demand state and their genuine empty updater traversals.
//! These native updaters never call their module input getters; true input flags remain true.
use super::super::Error;
use std::convert::Infallible;
mod demand;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceIceThermalStorage {
    pub(crate) get_input: bool,
    pub(crate) number_thermal_storage_sizing: i32,
    pub(crate) number_simple: i32,
    pub(crate) number_detailed: i32,
    pub(crate) total_number: i32,
    pub(crate) sizing: Option<Vec<Infallible>>,
    pub(crate) simple: Option<Vec<Infallible>>,
    pub(crate) detailed: Option<Vec<Infallible>>,
}
impl Default for SourceIceThermalStorage {
    fn default() -> Self {
        Self {
            get_input: true,
            number_thermal_storage_sizing: 0,
            number_simple: 0,
            number_detailed: 0,
            total_number: 0,
            sizing: None,
            simple: None,
            detailed: None,
        }
    }
}
impl SourceIceThermalStorage {
    pub(super) fn update_source_ice_fractions(&self) -> Result<(), Error> {
        if let Some(rows) = self.simple.as_deref() {
            for row in rows {
                match *row {}
            }
        }
        if let Some(rows) = self.detailed.as_deref() {
            for row in rows {
                match *row {}
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceEirPlantLoopHeatPumps {
    pub(crate) heat_pumps: Vec<Infallible>,
    pub(crate) get_input: bool,
}
impl Default for SourceEirPlantLoopHeatPumps {
    fn default() -> Self {
        Self {
            heat_pumps: Vec::new(),
            get_input: true,
        }
    }
}
impl SourceEirPlantLoopHeatPumps {
    pub(super) fn check_source_concurrent_operation(&self) -> Result<(), Error> {
        for heat_pump in &self.heat_pumps {
            match *heat_pump {}
        }
        Ok(())
    }
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceDemandManager {
    pub(crate) number_lists: i32,
    pub(crate) number_managers: i32,
    pub(crate) exterior_iterations: i32,
    pub(crate) heat_balance_iterations: i32,
    pub(crate) hvac_iterations: i32,
    pub(crate) get_input: bool,
    pub(crate) lists: Option<Vec<Infallible>>,
    pub(crate) managers: Option<Vec<Infallible>>,
    pub(crate) unique_manager_names: std::collections::HashMap<String, String>,
    pub(crate) clear_history: bool,
    pub(crate) begin_simulation: bool,
    pub(crate) resimulate_hvac: bool,
    pub(crate) resimulate_heat_balance: bool,
    pub(crate) resimulate_exterior: bool,
    pub(crate) first_time: bool,
}
impl Default for SourceDemandManager {
    fn default() -> Self {
        Self {
            number_lists: 0,
            number_managers: 0,
            exterior_iterations: 0,
            heat_balance_iterations: 0,
            hvac_iterations: 0,
            get_input: true,
            lists: None,
            managers: None,
            unique_manager_names: Default::default(),
            clear_history: true,
            begin_simulation: true,
            resimulate_hvac: true,
            resimulate_heat_balance: true,
            resimulate_exterior: true,
            first_time: true,
        }
    }
}
impl SourceDemandManager {
    pub(super) fn update_source_demand_managers(&self) -> Result<(), Error> {
        if self.number_managers > 0 {
            return Err(Error::Unavailable(
                "actual DemandMgr first record1291 before availSched read1294",
            ));
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceHvacPostludeModules {
    pub(crate) ice: SourceIceThermalStorage,
    pub(crate) eir: SourceEirPlantLoopHeatPumps,
    pub(crate) demand: SourceDemandManager,
}
