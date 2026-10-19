//! Genuine InitAirLoops1421-2307 empty-system route and ConnectReturnNodes.
use super::*;
use crate::heat_balance::selected_project_data::{
    SourceNodeConvergenceLog, SourceZoneInletConvergence,
};
#[derive(Clone, Debug, Default)]
struct AirloopZone {
    #[allow(
        dead_code,
        reason = "SimAirServingZones.cc InitAirLoops keeps the original AirloopZone scratch-row layout; selected zero-primary-loop traversal does not read these members."
    )]
    controlled_zone_number: i32,
    #[allow(
        dead_code,
        reason = "SimAirServingZones.cc InitAirLoops keeps the original AirloopZone scratch-row layout; selected zero-primary-loop traversal does not read these members."
    )]
    zone_inlet_node: i32,
    #[allow(
        dead_code,
        reason = "SimAirServingZones.cc InitAirLoops keeps the original AirloopZone scratch-row layout; selected zero-primary-loop traversal does not read these members."
    )]
    terminal_unit_inlet_node: i32,
    #[allow(
        dead_code,
        reason = "SimAirServingZones.cc InitAirLoops keeps the original AirloopZone scratch-row layout; selected zero-primary-loop traversal does not read these members."
    )]
    terminal_unit_sizing_index: i32,
}
impl SourceAirLoopData {
    pub(super) fn initialize_source_air_loops(
        &mut self,
        first: bool,
        o: &mut SourceAirLoopOwners<'_>,
    ) -> Result<(), Error> {
        let primary = o.hvac.number_of_primary_air_systems;
        o.hvac.air_loop_init = true;
        if self.initialize_air_loops_one_time {
            let n = usize::try_from(o.globals.num_of_zones)
                .map_err(|_| Error::Unavailable("actual InitAir NumOfZones1462"))?;
            let unit_count = o
                .equipment
                .source_air_distribution_units()
                .map_or(0, <[Infallible]>::len);
            let atu_array_size = n.max(unit_count);
            let _cooled_zone = vec![AirloopZone::default(); atu_array_size];
            let _heated_zone = vec![AirloopZone::default(); atu_array_size];
            self.mass_flow_set_tolerance = Some(0.01 * 0.00001);
            if *o.equipment.source_number_supply_paths() > 0 {
                return Err(Error::Unavailable(
                    "positive actual SupplyAirPath NumOfComponents1479",
                ));
            }
            if primary > 0 {
                return Err(Error::Unavailable(
                    "positive actual InitAirLoops PrimaryAirSystems1611",
                ));
            }
            let _maximum_number_air_loop_zones = 0_i32;
            o.project.convergence.zone_inlet_convergence =
                Some(vec![SourceZoneInletConvergence::default(); n]);
            for number in 1..=o.globals.num_of_zones {
                let config = o.equipment.source_zone_equipment_config(number)?;
                let inlet_count = *config.number_inlet_nodes();
                if inlet_count > 0 {
                    let target = o
                        .project
                        .convergence
                        .zone_inlet_convergence
                        .as_mut()
                        .and_then(|v| v.get_mut(index(number).ok()?))
                        .ok_or(Error::Unavailable("same actual ZoneInletConvergence1934"))?;
                    target.number_inlet_nodes = i32::try_from(inlet_count)
                        .map_err(|_| Error::Unavailable("actual signed NumInletNodes1934"))?;
                    target.inlet_nodes =
                        Some(vec![SourceNodeConvergenceLog::default(); inlet_count]);
                    for ordinal in 1..=inlet_count {
                        let actual = *config
                            .inlet_node_number(ordinal)
                            .ok_or(Error::Unavailable("actual ZoneEquipConfig InletNode1939"))?;
                        target
                            .inlet_nodes
                            .as_mut()
                            .and_then(|rows| rows.get_mut(ordinal - 1))
                            .ok_or(Error::Unavailable(
                                "actual ZoneInletConvergence InletNode1939",
                            ))?
                            .node_number = i32::try_from(actual).map_err(|_| {
                            Error::Unavailable("actual convergence NodeNum integer domain1939")
                        })?;
                    }
                }
            }
            self.connect_source_return_nodes(o)?;
            self.initialize_air_loops_one_time = false;
            // ErrorsFoundfalse and all three later primary-system loops1953,
            // 2015,2056 are genuinely empty, not skipped required callbacks.
        }
        if !o.globals.system_sizing_calculation && self.initialize_air_loops_branch_sizing {
            if primary > 0 {
                return Err(Error::Unavailable(
                    "positive actual SizeAirLoopBranches2102",
                ));
            }
            self.initialize_air_loops_branch_sizing = false;
            for number in 1..=o.globals.num_of_zones {
                let config = o.equipment.source_zone_equipment_config(number)?;
                if !*config.is_controlled() {
                    continue;
                }
                for ordinal in 1..=*config.number_return_nodes() {
                    let air_loop = *config
                        .return_node_air_loop_number(ordinal)
                        .ok_or(Error::Unavailable("actual ReturnNodeAirLoopNum2133"))?;
                    if air_loop > 0 {
                        return Err(Error::Unavailable("actual AirLoopFlow DesSupply2135"));
                    }
                }
            }
        }
        if o.clock.begin_envrn_flag && first && self.my_environment_flag {
            if primary > 0 {
                return Err(Error::Unavailable("positive actual PriAirSysAvailMgr2145"));
            }
            self.my_environment_flag = false;
        }
        if !o.clock.begin_envrn_flag {
            self.my_environment_flag = true;
        }
        // Actual BeginDay block is empty. Every timestep primary loop2203 is empty.
        if primary > 0 {
            return Err(Error::Unavailable(
                "positive actual InitAirLoops current PrimaryAirSystems2204",
            ));
        }
        Ok(())
    }
    fn connect_source_return_nodes(&mut self, o: &SourceAirLoopOwners<'_>) -> Result<(), Error> {
        if !*o.equipment.source_equipment_inputs_filled() {
            return Ok(());
        }
        for number in 1..=o.globals.num_of_zones {
            let config = o.equipment.source_zone_equipment_config(number)?;
            if !*config.is_controlled() {
                continue;
            }
            for ordinal in 1..=*config.number_return_nodes() {
                let _this_return_node =
                    *config
                        .return_node_number(ordinal)
                        .ok_or(Error::Unavailable(
                            "actual ConnectReturnNodes ReturnNode2333",
                        ))?;
                // Native starts this local at0 and only the following actual
                // return-path child may produce a positive airloop association.
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _return_path in 1..=*o.equipment.source_number_return_paths() {
                    return Err(Error::Unavailable(
                        "actual ConnectReturnNodes ReturnAirPath2336",
                    ));
                }
                // No return-path child executed: actual local airLoopNum remains0.
                // Its positive matching-inlet branch therefore performs no writes.
            }
        }
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "positive actual ConnectReturnNodes AirToZoneNodeInfo2396",
            ));
        }
        Ok(())
    }
}
