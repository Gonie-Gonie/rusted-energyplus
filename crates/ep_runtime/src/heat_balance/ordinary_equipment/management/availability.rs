//! Actual SystemAvailabilityManager136–309/310–1220/1561–1680 empty-manager path.
use crate::heat_balance::ordinary_hvac_manager::{SourceHvacManager, SourceHvacOwners};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use std::convert::Infallible;
const MANAGER_TYPES: [&str; 12] = [
    "AvailabilityManager:Scheduled",
    "AvailabilityManager:ScheduledOn",
    "AvailabilityManager:ScheduledOff",
    "AvailabilityManager:NightCycle",
    "AvailabilityManager:DifferentialThermostat",
    "AvailabilityManager:HighTemperatureTurnOff",
    "AvailabilityManager:HighTemperatureTurnOn",
    "AvailabilityManager:LowTemperatureTurnOff",
    "AvailabilityManager:LowTemperatureTurnOn",
    "AvailabilityManager:NightVentilation",
    "AvailabilityManager:HybridVentilation",
    "AvailabilityManager:OptimumStart",
];
const COMPONENT_TYPES: [&str; 14] = [
    "ZoneHVAC:FourPipeFanCoil",
    "ZoneHVAC:PackagedTerminalHeatPump",
    "ZoneHVAC:PackagedTerminalAirConditioner",
    "ZoneHVAC:WaterToAirHeatPump",
    "ZoneHVAC:WindowAirConditioner",
    "ZoneHVAC:UnitHeater",
    "ZoneHVAC:UnitVentilator",
    "ZoneHVAC:EnergyRecoveryVentilator",
    "ZoneHVAC:VentilatedSlab",
    "ZoneHVAC:OutdoorAirUnit",
    "ZoneHVAC:TerminalUnit:VariableRefrigerantFlow",
    "ZoneHVAC:IdealLoadsAirSystem",
    "ZoneHVAC:EvaporativeCoolerUnit",
    "ZoneHVAC:HybridUnitaryHVAC",
];
const QUERY_TYPES: [&str; 11] = [
    "AvailabilityManager:Scheduled",
    "AvailabilityManager:ScheduledOn",
    "AvailabilityManager:ScheduledOff",
    "AvailabilityManager:NightCycle",
    "AvailabilityManager:OptimumStart",
    "AvailabilityManager:DifferentialThermostat",
    "AvailabilityManager:HighTemperatureTurnOff",
    "AvailabilityManager:HighTemperatureTurnOn",
    "AvailabilityManager:LowTemperatureTurnOff",
    "AvailabilityManager:LowTemperatureTurnOn",
    "AvailabilityManager:NightVentilation",
];
/// Native count/declaration order differs from both definition and reset order; each fixed index maps to one same member.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceAvailability {
    pub(crate) get_input_flag: bool,
    pub(crate) initialize_one_time_flag: bool,
    pub(crate) counts: [i32; 11],
    pub(crate) managers: [Option<Vec<Infallible>>; 11],
    pub(crate) zone_components: Option<Vec<SourceZoneComponentType>>,
    pub(crate) optimum_start: Option<Vec<SourceOptimumStart>>,
}
impl Default for SourceAvailability {
    fn default() -> Self {
        Self {
            get_input_flag: true,
            initialize_one_time_flag: true,
            counts: [0; 11],
            managers: std::array::from_fn(|_| None),
            zone_components: None,
            optimum_start: None,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceZoneComponentType {
    pub(crate) managers: Option<Vec<SourceZoneComponentManager>>,
    pub(crate) total_components: i32,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceZoneComponentManager {
    pub(crate) number_managers: i32,
    pub(crate) status: i32,
    pub(crate) start_time: i32,
    pub(crate) stop_time: i32,
    pub(crate) manager_list_name: String,
    pub(crate) managers: Option<Vec<Infallible>>,
    pub(crate) zone_number: i32,
    pub(crate) input: bool,
    pub(crate) count: i32,
}
impl Default for SourceZoneComponentManager {
    fn default() -> Self {
        Self {
            number_managers: 0,
            status: 0,
            start_time: 0,
            stop_time: 0,
            manager_list_name: String::new(),
            managers: None,
            zone_number: 0,
            input: true,
            count: 0,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceOptimumStart {
    pub(crate) actual_zone_number: i32,
    pub(crate) occupancy_start_time: f64,
    pub(crate) start_flag: bool,
}
impl SourceAvailability {
    fn get_source_inputs(&mut self, input: &mut SelectedInputProcessor) -> Result<(), Error> {
        let mut max_numbers = 0;
        let mut max_alphas = 0;
        for family in MANAGER_TYPES {
            let args = input
                .get_object_def_max_args(family)
                .map_err(|_| Error::Unavailable("actual availability getObjectDefMaxArgs385"))?;
            max_numbers = max_numbers.max(args.numerics);
            max_alphas = max_alphas.max(args.alphas);
        }
        let alpha_field_names = vec![String::new(); max_alphas];
        let alpha_args = vec![String::new(); max_alphas];
        let alpha_blanks = vec![false; max_alphas];
        let numeric_field_names = vec![String::new(); max_numbers];
        let numeric_args = vec![0.0_f64; max_numbers];
        let numeric_blanks = vec![false; max_numbers];
        if self.zone_components.is_none() {
            self.zone_components = Some(vec![SourceZoneComponentType::default(); 14]);
        }
        for (at, family) in COMPONENT_TYPES.into_iter().enumerate() {
            let row = self
                .zone_components
                .as_mut()
                .and_then(|r| r.get_mut(at))
                .ok_or(Error::Unavailable("actual ZoneComp allocation400"))?;
            if row.managers.is_none() {
                row.total_components =
                    i32::try_from(input.get_num_objects_found(family)).map_err(|_| {
                        Error::Unavailable("actual availability component count int domain")
                    })?;
                if row.total_components > 0 {
                    return Err(Error::Unavailable(
                        "actual ZoneCompAvailMgrs positive allocation405",
                    ));
                }
            }
        }
        for (at, family) in QUERY_TYPES.into_iter().enumerate() {
            self.counts[at] = i32::try_from(input.get_num_objects_found(family))
                .map_err(|_| Error::Unavailable("actual availability manager count int domain"))?;
            if self.counts[at] > 0 {
                return Err(Error::Unavailable(
                    "actual positive availability manager allocation",
                ));
            }
        }
        drop(alpha_field_names);
        drop(alpha_args);
        drop(alpha_blanks);
        drop(numeric_field_names);
        drop(numeric_args);
        drop(numeric_blanks);
        Ok(())
    }
    fn initialize_source_managers(&mut self, zones: i32) -> Result<(), Error> {
        if self.initialize_one_time_flag {
            if self.counts[4] > 0 {
                return Err(Error::Unavailable("actual optimumStartMgr record1582"));
            }
            self.initialize_one_time_flag = false;
        }
        // Source reset order: Sched/On/Off/NightCycle/NightVent/Diff/HiOff/HiOn/LoOff/LoOn/OptimumStart.
        for index in [0, 1, 2, 3, 10, 5, 6, 7, 8, 9, 4] {
            if let Some(rows) = &self.managers[index] {
                for row in rows {
                    match *row {}
                }
            }
            if index == 4 && self.managers[index].is_some() {
                if self.optimum_start.is_none() {
                    let zones = usize::try_from(zones).map_err(|_| {
                        Error::Unavailable("actual OptStart NumOfZones allocation1659")
                    })?;
                    self.optimum_start = Some(vec![SourceOptimumStart::default(); zones]);
                }
                for row in self.optimum_start.as_mut().into_iter().flatten() {
                    row.start_flag = false;
                }
            }
        }
        if let Some(rows) = self.zone_components.as_mut() {
            for index in 0..14 {
                let row = rows
                    .get_mut(index)
                    .ok_or(Error::Unavailable("actual InitSysAvail ZoneComp1669"))?;
                if row.total_components > 0 {
                    if let Some(managers) = row.managers.as_mut() {
                        for manager in managers {
                            manager.status = 0;
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
impl SourceHvacManager {
    pub(crate) fn manage_source_system_availability(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        let availability = &mut self.ancillary.availability;
        if availability.get_input_flag {
            availability.get_source_inputs(o.input)?;
            availability.get_input_flag = false;
            return Ok(());
        }
        availability.initialize_source_managers(o.globals.num_of_zones)?;
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable("actual primary availability record179"));
        }
        if o.hvac.number_plant_loops > 0 {
            return Err(Error::Unavailable("actual PlantAvailMgr224"));
        }
        let Some(rows) = availability.zone_components.as_mut() else {
            return Ok(());
        };
        for index in 0..14 {
            let row = rows
                .get_mut(index)
                .ok_or(Error::Unavailable("actual same ZoneComp251"))?;
            if row.total_components == 0 {
                continue;
            }
            if row.managers.is_none() {
                continue;
            }
            return Err(Error::Unavailable(
                "actual positive zone availability manager257",
            ));
        }
        // Hybrid flags/count remain sole existing ancillary.hybrid_ventilation; zero components never read its inner branch.
        Ok(())
    }
}

impl SourceAvailability {
    pub(crate) fn fill_source_predefined_availability_manager_tables(
        &self,
        hvac: &crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals,
    ) -> Result<(), Error> {
        if hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual FillPredefinedTablesForAvailManager first PriAirSysAvailMgr row",
            ));
        }
        Ok(())
    }
}
