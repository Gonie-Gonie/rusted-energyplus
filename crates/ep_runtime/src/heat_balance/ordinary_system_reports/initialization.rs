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
            // Native1993 takes the real report-row reference before the name and flag.
            self.zone_vent_rep_vars
                .as_ref()
                .and_then(|rows| rows.get(at(zone).ok()?))
                .ok_or(Error::Unavailable("actual ZoneVentRepVars1993 live row"))?;
            let config = o.equipment.source_zone_equipment_config(zone)?;
            let name = config.zone_name();
            self.register_source_zone_ventilation_outputs(
                zone, name, registration, input, backend, output,
            )?;
        }
        self.register_source_facility_ventilation_outputs(registration, input, backend, output)
    }

    /// Actual SimulationManager333 call; the flag write precedes all loops.
    pub(crate) fn create_source_energy_report_structure(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        hvac: &SelectedHvacGlobals,
        equipment: &mut OrdinaryEquipment,
        branch: &crate::heat_balance::startup_integrity::SourceBranchNodeConnections,
        registration: &SourceOutputRegistration,
    ) -> Result<(), Error> {
        self.vent_report_structure_created = true;
        if hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual CreateEnergyReportStructure2246 PrimaryAirSystems.NumBranches",
            ));
        }
        // Both actual primary-system outer loops have zero rows here.
        equipment.create_source_zone_equipment_energy_report_structure(
            globals.num_of_zones, branch, registration,
        )?;
        let total = hvac.number_plant_loops.checked_add(hvac.number_condenser_loops)
            .ok_or(Error::Unavailable("undefined plant plus condenser loop count2747"))?;
        if total > 0 {
            return Err(Error::Unavailable(
                "actual CreateEnergyReportStructure2755/2758 live VentRepPlant/VentRepCond row",
            ));
        }
        // Both real outer plant loops2747/2877 have zero actual rows.
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn initialize_source_energy_reports(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        hvac: &SelectedHvacGlobals,
        equipment: &mut OrdinaryEquipment,
        air_loop: &SourceAirLoopData,
        zones: &SelectedZoneInputOwners,
        registry: &PredefinedRegistry,
        table: &mut PredefinedStringTable,
        registration: &SourceOutputRegistration,
    ) -> Result<(), Error> {
        if !self.vent_report_structure_created { return Ok(()); }
        if self.one_time_flag_init_energy_reports {
            equipment.initialize_source_equipment_energy_connections(globals.num_of_zones, hvac, air_loop)?;
            // Native783-800 captures all six actual arrays after both passes.
            let zone_comps = air_loop.zone_comp_to_plant.as_ref().map_or(0, Vec::len);
            let zone_subcomps = air_loop.zone_sub_comp_to_plant.as_ref().map_or(0, Vec::len);
            let zone_subsubcomps = air_loop.zone_sub_sub_comp_to_plant.as_ref().map_or(0, Vec::len);
            let air_comps = air_loop.air_sys_comp_to_plant.as_ref().map_or(0, Vec::len);
            let air_subcomps = air_loop.air_sys_sub_comp_to_plant.as_ref().map_or(0, Vec::len);
            let air_subsubcomps = air_loop.air_sys_sub_sub_comp_to_plant.as_ref().map_or(0, Vec::len);
            self.one_time_flag_init_energy_reports = false;
            for _row in 1..=zone_comps { return Err(Error::Unavailable("actual InitEnergyReports805 ZoneCompToPlant.PlantLoopType")); }
            for _row in 1..=zone_subcomps { return Err(Error::Unavailable("actual InitEnergyReports826 ZoneSubCompToPlant.PlantLoopType")); }
            for _row in 1..=zone_subsubcomps { return Err(Error::Unavailable("actual InitEnergyReports847 ZoneSubSubCompToPlant.PlantLoopType")); }
            for _row in 1..=air_comps { return Err(Error::Unavailable("actual InitEnergyReports867 AirSysCompToPlant.PlantLoopType")); }
            for _row in 1..=air_subcomps { return Err(Error::Unavailable("actual InitEnergyReports888 AirSysSubCompToPlant.PlantLoopType")); }
            for _row in 1..=air_subsubcomps { return Err(Error::Unavailable("actual InitEnergyReports909 AirSysSubSubCompToPlant.PlantLoopType")); }
            self.report_source_air_loop_topology(hvac)?;
            equipment.report_source_zone_energy_topology(globals.num_of_zones, zones, registry, table)?;
            self.report_source_air_distribution_units(equipment)?;
            self.one_time_flag_init_energy_reports = false;
        }
        if hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable("actual InitEnergyReports940 PrimaryAirSystems live row"));
        }
        equipment.load_source_zone_equipment_energy_readings(globals.num_of_zones, registration)?;
        // Native LoopSideKeys is Demand0, Supply1; both count loops remain real.
        for _loop_side in [0_i32, 1_i32] {
            for _plant in 1..=hvac.number_plant_loops { return Err(Error::Unavailable("actual InitEnergyReports1011 VentRepPlant live row")); }
            for _plant in 1..=hvac.number_condenser_loops { return Err(Error::Unavailable("actual InitEnergyReports1028 VentRepCond live row")); }
        }
        Ok(())
    }
    fn report_source_air_loop_topology(&self, hvac: &SelectedHvacGlobals) -> Result<(), Error> {
        if hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable("actual reportAirLoopToplogy4947 PrimaryAirSystems live row"));
        }
        Ok(())
    }
    fn report_source_air_distribution_units(&self, equipment: &OrdinaryEquipment) -> Result<(), Error> {
        if equipment.source_air_distribution_units().is_some_and(|rows| !rows.is_empty()) {
            return Err(Error::Unavailable("actual reportAirDistributionUnits5282 ADU.Name/ZoneName first table entry"));
        }
        Ok(())
    }
}
