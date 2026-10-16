//! Actual startup allocation/structure and InitEnergyReports reached control flow.
use super::state::SourceZoneVentReportVariables;
use super::*;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::heat_balance::source_output_registration::{
    SourceOutputDictionaryHost, SourceOutputFileHost, SourceOutputRegistration,
};

impl SourceSystemReports {
    /// Actual SimulationManager294 call, before CreateEnergyReportStructure333.
    pub(crate) fn allocate_and_setup_source_vent_reports<
        H: SourceOutputFileHost,
        D: SourceOutputDictionaryHost,
    >(
        &mut self,
        o: &mut SourceSystemReportOwners<'_>,
        registration: &mut SourceOutputRegistration,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        output: &mut SourceSystemReportsOutputContext<'_, H, D>,
    ) -> Result<(), Error> {
        let number_primary = o.hvac.number_of_primary_air_systems;
        let zones = usize::try_from(o.globals.num_of_zones)
            .map_err(|_| Error::Unavailable("actual ZoneVentRepVars allocation count1624"))?;
        // EPVector::allocate98–103 sets allocated, resizes, then fills actual T{}.
        // Repeated native calls reinitialize; no false already-allocated boundary.
        self.zone_vent_rep_vars = Some(vec![SourceZoneVentReportVariables::default(); zones]);
        if number_primary > 0 {
            return Err(Error::Unavailable(
                "actual positive SysLoadRepVars constructor1625",
            ));
        }
        if number_primary < 0 {
            return Err(Error::Unavailable(
                "undefined negative SysLoadRepVars allocation1625",
            ));
        }
        self.sys_load_rep_vars = Some(Vec::new());
        self.sys_vent_rep_vars = Some(Vec::new());
        self.sys_pre_def_rep = Some(Vec::new());
        // Both native system loops1629/1642 genuinely have zero actual rows.
        for zone in 1..=o.globals.num_of_zones {
            if !*o
                .equipment
                .source_zone_equipment_config(zone)?
                .is_controlled()
            {
                continue;
            }
            return Err(Error::Unavailable(
                "actual controlled ZoneVentRepVars/ZoneEquipConfig.ZoneName1993–1994 before zone declarations",
            ));
        }
        self.register_source_facility_ventilation_outputs(registration, input, backend, output)
    }

    /// Actual SimulationManager333 call; the flag write precedes all loops.
    pub(crate) fn create_source_energy_report_structure(
        &mut self,
        o: &SourceSystemReportOwners<'_>,
    ) -> Result<(), Error> {
        self.vent_report_structure_created = true;
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual CreateEnergyReportStructure2246 PrimaryAirSystems.NumBranches",
            ));
        }
        // The second primary loop2379 is also empty, independently of Plant counts.
        for zone in 1..=o.globals.num_of_zones {
            if !*o
                .equipment
                .source_zone_equipment_config(zone)?
                .is_controlled()
            {
                continue;
            }
            return Err(Error::Unavailable(
                "actual CreateEnergyReportStructure2502 controlled ZoneEquipList.NumOfEquipTypes",
            ));
        }
        for zone in 1..=o.globals.num_of_zones {
            if !*o
                .equipment
                .source_zone_equipment_config(zone)?
                .is_controlled()
            {
                continue;
            }
            return Err(Error::Unavailable(
                "actual CreateEnergyReportStructure2663 controlled ZoneEquipList.NumOfEquipTypes",
            ));
        }
        let total = o
            .hvac
            .number_plant_loops
            .checked_add(o.hvac.number_condenser_loops)
            .ok_or(Error::Unavailable(
                "undefined plant plus condenser loop count2747",
            ))?;
        if total > 0 {
            return Err(Error::Unavailable(
                "actual CreateEnergyReportStructure2755/2758 live VentRepPlant/VentRepCond row",
            ));
        }
        // Both real outer plant loops2747/2877 have zero actual rows.
        Ok(())
    }

    pub(crate) fn initialize_source_energy_reports(
        &mut self,
        o: &mut SourceSystemReportOwners<'_>,
    ) -> Result<(), Error> {
        if !self.vent_report_structure_created {
            return Ok(());
        }
        if self.one_time_flag_init_energy_reports {
            for zone in 1..=o.globals.num_of_zones {
                if !*o
                    .equipment
                    .source_zone_equipment_config(zone)?
                    .is_controlled()
                {
                    continue;
                }
                return Err(Error::Unavailable(
                    "actual InitEnergyReports145 FindItemInList/EquipListIndex assignment",
                ));
            }
            for zone in 1..=o.globals.num_of_zones {
                if !*o
                    .equipment
                    .source_zone_equipment_config(zone)?
                    .is_controlled()
                {
                    continue;
                }
                return Err(Error::Unavailable(
                    "actual InitEnergyReports303 FindItemInList/EquipListIndex assignment",
                ));
            }
            let zone_comps = o.air_loop.zone_comp_to_plant.as_ref().map_or(0, Vec::len);
            let zone_subcomps = o
                .air_loop
                .zone_sub_comp_to_plant
                .as_ref()
                .map_or(0, Vec::len);
            let zone_subsubcomps = o
                .air_loop
                .zone_sub_sub_comp_to_plant
                .as_ref()
                .map_or(0, Vec::len);
            let air_comps = o
                .air_loop
                .air_sys_comp_to_plant
                .as_ref()
                .map_or(0, Vec::len);
            let air_subcomps = o
                .air_loop
                .air_sys_sub_comp_to_plant
                .as_ref()
                .map_or(0, Vec::len);
            let air_subsubcomps = o
                .air_loop
                .air_sys_sub_sub_comp_to_plant
                .as_ref()
                .map_or(0, Vec::len);
            self.one_time_flag_init_energy_reports = false;
            if zone_comps > 0 {
                return Err(Error::Unavailable(
                    "actual InitEnergyReports805 ZoneCompToPlant.PlantLoopType",
                ));
            }
            if zone_subcomps > 0 {
                return Err(Error::Unavailable(
                    "actual InitEnergyReports826 ZoneSubCompToPlant.PlantLoopType",
                ));
            }
            if zone_subsubcomps > 0 {
                return Err(Error::Unavailable(
                    "actual InitEnergyReports847 ZoneSubSubCompToPlant.PlantLoopType",
                ));
            }
            if air_comps > 0 {
                return Err(Error::Unavailable(
                    "actual InitEnergyReports867 AirSysCompToPlant.PlantLoopType",
                ));
            }
            if air_subcomps > 0 {
                return Err(Error::Unavailable(
                    "actual InitEnergyReports888 AirSysSubCompToPlant.PlantLoopType",
                ));
            }
            if air_subsubcomps > 0 {
                return Err(Error::Unavailable(
                    "actual InitEnergyReports909 AirSysSubSubCompToPlant.PlantLoopType",
                ));
            }
            self.report_source_air_loop_topology(o)?;
            self.report_source_zone_equipment_topology(o)?;
            self.report_source_air_distribution_units(o)?;
            self.one_time_flag_init_energy_reports = false;
        }
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual InitEnergyReports940 PrimaryAirSystems live row",
            ));
        }
        for zone in 1..=o.globals.num_of_zones {
            if !*o
                .equipment
                .source_zone_equipment_config(zone)?
                .is_controlled()
            {
                continue;
            }
            return Err(Error::Unavailable(
                "actual InitEnergyReports978 ZoneEquipList live row",
            ));
        }
        // Native LoopSideKeys is Demand0, Supply1, retaining both count checks.
        for _loop_side in [0_i32, 1_i32] {
            if o.hvac.number_plant_loops > 0 {
                return Err(Error::Unavailable(
                    "actual InitEnergyReports1011 VentRepPlant live row",
                ));
            }
            if o.hvac.number_condenser_loops > 0 {
                return Err(Error::Unavailable(
                    "actual InitEnergyReports1028 VentRepCond live row",
                ));
            }
        }
        Ok(())
    }
    fn report_source_air_loop_topology(
        &self,
        o: &SourceSystemReportOwners<'_>,
    ) -> Result<(), Error> {
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual reportAirLoopToplogy4947 PrimaryAirSystems live row",
            ));
        }
        Ok(())
    }
    fn report_source_zone_equipment_topology(
        &self,
        o: &mut SourceSystemReportOwners<'_>,
    ) -> Result<(), Error> {
        let mut row_counter = 1_i32;
        for zone in 1..=o.globals.num_of_zones {
            let name = &o
                .zones
                .zones
                .as_deref()
                .and_then(|v| v.get(at(zone).ok()?))
                .ok_or(Error::Unavailable("actual topology5227 Zone.Name"))?
                .name;
            let column = *o
                .predefined_registry
                .named_indices
                .get("pdchTopZnEqpName")
                .ok_or(Error::Unavailable("actual topology pdchTopZnEqpName"))?;
            o.predefined_table
                .string_entry(column, &format!("{row_counter}"), name)?;
            row_counter = row_counter
                .checked_add(1)
                .ok_or(Error::Unavailable("undefined topology row counter5229"))?;
            if !*o
                .equipment
                .source_zone_equipment_config(zone)?
                .is_controlled()
            {
                continue;
            }
            return Err(Error::Unavailable(
                "actual topology5233 ZoneEquipList live row",
            ));
        }
        Ok(())
    }
    fn report_source_air_distribution_units(
        &self,
        o: &SourceSystemReportOwners<'_>,
    ) -> Result<(), Error> {
        if o.equipment
            .source_air_distribution_units()
            .is_some_and(|v| !v.is_empty())
        {
            return Err(Error::Unavailable(
                "actual reportAirDistributionUnits5282 ADU.Name/ZoneName first table entry",
            ));
        }
        Ok(())
    }
}
