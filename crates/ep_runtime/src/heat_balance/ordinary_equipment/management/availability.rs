//! Actual same-owner SystemAvailabilityManager component allocation and no-manager lifecycle.
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
    /// Actual independent SystemAvailabilityManagerData.hh584/588 cold cells.
    pub(crate) get_lists_input: bool,
    pub(crate) number_manager_lists: i32,
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
            get_lists_input: true,
            number_manager_lists: 0,
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
                    let count = usize::try_from(row.total_components)
                        .map_err(|_| Error::Unavailable("actual ZoneCompAvailMgrs allocation405 integer domain"))?;
                    row.managers = Some(vec![SourceZoneComponentManager::default(); count]);
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
            let Some(managers) = row.managers.as_mut() else {
                continue;
            };
            for component_number in 1..=row.total_components {
                let at = component_number.checked_sub(1)
                    .and_then(|number| usize::try_from(number).ok())
                    .ok_or(Error::Unavailable("actual zone availability component ordinal257"))?;
                let manager = managers.get_mut(at)
                    .ok_or(Error::Unavailable("actual same ZoneCompAvailMgrs259"))?;
                if manager.number_managers > 0 {
                    let _previous_status = manager.status;
                    manager.status = 0; // Native Status::NoAction265 precedes first manager record.
                    return Err(Error::Unavailable("positive zone availability availManagers first record268"));
                } else {
                    manager.status = 0; // Literal no-manager branch287, not a forced readiness flag.
                }
                if manager.zone_number == 0 {
                    continue;
                }
                if self.ancillary.hybrid_ventilation.number_hybrid_ventilation_managers == 0 {
                    continue;
                }
                return Err(Error::Unavailable("positive zone availability HybridVentData first record298"));
            }
        }
        // The hybrid count is borrowed from the same retained ancillary owner only after real ZoneNum != 0.
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

impl SourceAvailability {
    /// Native GetSysAvailManagerListInputs1219: same input flag and real list count.
    fn get_source_manager_list_inputs(&mut self, input: &mut SelectedInputProcessor) -> Result<(), Error> {
        if self.get_input_flag {
            self.get_source_inputs(input)?;
            self.get_input_flag = false;
        }
        self.number_manager_lists = i32::try_from(input.get_num_objects_found("AvailabilityManagerAssignmentList"))
            .map_err(|_|Error::Unavailable("actual availability list count integer domain1240"))?;
        if self.number_manager_lists > 0 {
            // Native only allocates ListData for positive count. No represented
            // positive List record or empty allocated substitute is invented.
            return Err(Error::Unavailable("actual AvailabilityManager ListData.allocate1244"));
        }
        Ok(())
    }

    /// Native1448-1512. The real no-list branch intentionally takes two calls to clear Input.
    pub(super) fn get_source_zone_equipment_availability(&mut self, equipment_type: i32, component: i32,
        input: &mut SelectedInputProcessor, _errors_found: &mut bool) -> Result<(), Error> {
        if self.get_lists_input {
            self.get_source_manager_list_inputs(input)?;
            self.get_lists_input = false;
        }
        let type_index = equipment_type.checked_sub(1).and_then(|v|usize::try_from(v).ok())
            .ok_or(Error::Unavailable("actual GetZoneEqAvailabilityManager type ordinal1471"))?;
        let component_index = component.checked_sub(1).and_then(|v|usize::try_from(v).ok())
            .ok_or(Error::Unavailable("actual GetZoneEqAvailabilityManager component ordinal1472"))?;
        let row = self.zone_components.as_mut().and_then(|rows|rows.get_mut(type_index))
            .and_then(|row|row.managers.as_mut()).and_then(|rows|rows.get_mut(component_index))
            .ok_or(Error::Unavailable("actual same ZoneCompAvailMgrs1472"))?;
        if row.input {
            let _availability_list_name = &row.manager_list_name;
            let found = 0;
            if self.number_manager_lists > 0 {
                return Err(Error::Unavailable("actual GetZoneEqAvailabilityManager ListData FindItemInList1478"));
            }
            // A positive Found cannot arise before the actual unrepresented
            // list lookup above. Its manager-copy operations remain unclaimed.
            if row.count > 0 || found > 0 { row.input = false; }
            row.count = row.count.checked_add(1)
                .ok_or(Error::Unavailable("actual GetZoneEqAvailabilityManager Count integer domain1509"))?;
        }
        Ok(())
    }
}
