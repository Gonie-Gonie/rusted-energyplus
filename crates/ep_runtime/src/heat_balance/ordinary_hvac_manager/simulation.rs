//! Genuine SimHVAC688-1798 and SimSelectedEquipment1800-1946 caller bodies.
//! Actual positive systems and diagnostics stop at their first unported operation.
pub(crate) mod air_loops;
pub(crate) mod output_registration;
mod plant;
mod providers;
mod setpoints;
pub(crate) mod state;
use super::{Error, SourceHvacEmsCallingPoint, SourceHvacManager, SourceHvacOwners};
use SourceHvacSimulationChild as Child;
pub(crate) use providers::{SourceHvacSimulationChild, SourceHvacSimulationChildren};

impl SourceHvacManager {
    pub(crate) fn simulate_source_hvac(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
        children: &mut impl SourceHvacSimulationChildren,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        let number_primary_air_systems = o.hvac.number_of_primary_air_systems;
        o.hvac.sim_zone_equipment = true;
        o.hvac.sim_non_zone_equipment = true;
        o.hvac.sim_air_loops = true;
        o.hvac.sim_plant_loops = true;
        o.hvac.sim_electric_circuits = true;
        let mut first_hvac_iteration = true;
        if self.simulation.air_loop.inputs_filled {
            // The genuine EPVector traversal is empty before positive GetAirPathData allocation.
            if let Some(rows) = &self.simulation.air_loop.control_info {
                for row in rows {
                    match *row {}
                }
            }
        }
        self.manage_iteration = 0;
        self.simulation.plant.manage_sub_iterations = 0;
        self.simulation.plant.manage_half_loop_calls = 0;
        self.set_source_all_plant_sim_flags(o, true)?;
        if !self.simulate_hvac_iteration_setup {
            children.run_simulation(Child::RegisterSolverOutputs, self, o, eio)?;
            self.manage_source_setpoints(o)?;
            self.get_source_plant_loop_data(o)?;
            self.get_source_plant_input(o)?;
            self.setup_source_initial_plant_calling_order()?;
            self.setup_source_plant_branch_control_types(o)?;
            self.setup_source_plant_reports(o)?;
            if o.globals.any_energy_management_system_in_model {
                children.run_simulation(Child::SetupPlantEmsActuators, self, o, eio)?;
            }
            if self.simulation.plant.total_loops > 0 {
                children.run_simulation(Child::RegisterPlantSolverOutputs, self, o, eio)?;
                for loop_number in 1..=self.simulation.plant.total_loops {
                    children.run_simulation(
                        Child::InitializeOneTimePlantSizing { loop_number },
                        self,
                        o,
                        eio,
                    )?;
                }
            }
            self.simulate_hvac_iteration_setup = true;
        }
        if o.globals.zone_sizing_calculation {
            children.run_simulation(
                Child::ManageZoneEquipment {
                    first_hvac_iteration,
                },
                self,
                o,
                eio,
            )?;
            children.run_simulation(
                Child::ManageNonZoneEquipment {
                    first_hvac_iteration,
                },
                self,
                o,
                eio,
            )?;
            children.run_simulation(
                Child::ManageElectricPowerService {
                    first_hvac_iteration,
                },
                self,
                o,
                eio,
            )?;
            return Ok(());
        }
        self.reset_source_hvac_control(o)?;
        let mut any_ems_ran = Some(false);
        self.manage_source_ems(
            o,
            SourceHvacEmsCallingPoint::BeforeHvacManagers,
            &mut any_ems_ran,
        )?;
        self.manage_source_setpoints(o)?;
        self.reinitialize_source_plant_loops(o)?;
        children.run_simulation(Child::ManageSystemAvailability, self, o, eio)?;
        self.manage_source_ems(
            o,
            SourceHvacEmsCallingPoint::AfterHvacManagers,
            &mut any_ems_ran,
        )?;
        self.manage_source_ems(
            o,
            SourceHvacEmsCallingPoint::HvacIterationLoop,
            &mut any_ems_ran,
        )?;
        self.simulate_source_selected_equipment(first_hvac_iteration, false, o, children, eio)?;
        o.hvac.sim_plant_loops = true;
        self.set_source_all_plant_sim_flags(o, true)?;
        first_hvac_iteration = false;
        while (o.hvac.sim_air_loops
            || o.hvac.sim_zone_equipment
            || o.hvac.sim_non_zone_equipment
            || o.hvac.sim_plant_loops
            || o.hvac.sim_electric_circuits)
            && self.manage_iteration <= o.project.convergence.max_iter
        {
            if o.globals.stop_simulation {
                break;
            }
            self.manage_source_ems(
                o,
                SourceHvacEmsCallingPoint::HvacIterationLoop,
                &mut any_ems_ran,
            )?;
            self.simulate_source_selected_equipment(first_hvac_iteration, false, o, children, eio)?;
            self.update_source_zone_inlet_convergence(o)?;
            self.manage_iteration =
                self.manage_iteration
                    .checked_add(1)
                    .ok_or(Error::Unavailable(
                        "actual HVACManageIteration Native signed overflow906",
                    ))?;
            if any_ems_ran.ok_or(Error::Unavailable("actual ManageEMS output unwritten"))?
                && self.manage_iteration <= 2
            {
                o.hvac.sim_air_loops = true;
            }
            if self.manage_iteration < o.hvac.minimum_air_loop_iterations_after_first {
                o.hvac.sim_air_loops = true;
                o.hvac.sim_zone_equipment = true;
            }
        }
        if o.globals.any_plant_in_model {
            return Err(Error::Unavailable(
                "actual AnyPlantSplitterMixerLacksContinuity919",
            ));
        }
        if self.simulation.plant.total_loops > 0 {
            return Err(Error::Unavailable(
                "actual plant mixer/splitter consistency985",
            ));
        }
        if self.manage_iteration > o.project.convergence.max_iter && !o.clock.warmup_flag {
            self.error_count = self.error_count.checked_add(1).ok_or(Error::Unavailable(
                "actual SimHVAC ErrCount signed overflow991",
            ))?;
            return Err(Error::Unavailable(
                "actual SimHVAC nonconvergence diagnostics992",
            ));
        }
        self.check_source_air_loop_flow_balance(o)?;
        if !o.globals.zone_sizing_calculation && !o.globals.system_sizing_calculation {
            if self.my_setpoint_init {
                if *o.equipment.source_number_of_nodes() > 0 {
                    return Err(Error::Unavailable("actual SimHVAC Node setpoint reset1769"));
                }
                self.my_setpoint_init = false;
                o.hvac.do_setpoint_test = true;
            } else {
                o.hvac.do_setpoint_test = false;
            }
            if self
                .simulation
                .cooling_dx
                .still_need_to_report_standard_ratings
                && !o.clock.warmup_flag
            {
                if !self.simulation.cooling_dx.coils.is_empty() {
                    return Err(Error::Unavailable(
                        "actual CoilCoolingDX standard-ratings EIO980",
                    ));
                }
                self.simulation
                    .cooling_dx
                    .still_need_to_report_standard_ratings = false;
            }
        }
        if o.hvac.setpoint_error_flag {
            return match o
                .backend
                .show_fatal_error("Previous severe set point errors cause program termination")
            {
                Ok(never) => match never {},
                Err(error) => Err(Error::Fatal(error)),
            };
        }
        let _ = number_primary_air_systems; // Original diagnostic-only local, no implied count writer.
        Ok(())
    }

    pub(crate) fn simulate_source_selected_equipment(
        &mut self,
        first_hvac_iteration: bool,
        lock_plant_flows: bool,
        o: &mut SourceHvacOwners<'_, '_>,
        children: &mut impl SourceHvacSimulationChildren,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        self.set_source_all_flow_locks(lock_plant_flows)?;
        self.reset_source_plant_interconnect_flags()?;
        if o.clock.begin_envrn_flag && self.my_environment_flag2 {
            children.run_simulation(
                Child::ManageZoneEquipment {
                    first_hvac_iteration,
                },
                self,
                o,
                eio,
            )?;
            self.my_environment_flag2 = false;
        }
        if !o.clock.begin_envrn_flag {
            self.my_environment_flag2 = true;
        }
        if first_hvac_iteration {
            self.report_iteration_air = 0;
            if o.controller.subsystems.airflow_network.control_type != 0 {
                children.run_simulation(
                    Child::AirflowNetworkBalance {
                        first_hvac_iteration,
                        iteration: None,
                    },
                    self,
                    o,
                    eio,
                )?;
            }
            children.run_simulation(
                Child::ManageAirLoops {
                    first_hvac_iteration,
                },
                self,
                o,
                eio,
            )?;
            self.simulation.air_loop.inputs_filled = true;
            o.hvac.sim_air_loops = true;
            o.hvac.air_loops_sim_once = true;
            self.reset_source_terminal_unit_flow_limits(o)?;
            self.flow_max_available_already_reset = true;
            children.run_simulation(
                Child::ManageZoneEquipment {
                    first_hvac_iteration,
                },
                self,
                o,
                eio,
            )?;
            o.hvac.sim_zone_equipment = true;
            children.run_simulation(
                Child::ManageNonZoneEquipment {
                    first_hvac_iteration,
                },
                self,
                o,
                eio,
            )?;
            children.run_simulation(
                Child::ManageElectricPowerService {
                    first_hvac_iteration,
                },
                self,
                o,
                eio,
            )?;
            self.manage_source_plant_loops(o)?;
            o.backend.state.tracking.ask_for_plant_check_on_abort = true;
            children.run_simulation(
                Child::ManageElectricPowerService {
                    first_hvac_iteration,
                },
                self,
                o,
                eio,
            )?;
        } else {
            self.flow_resolution_needed = false;
            let mut iteration_air = 0;
            while (o.hvac.sim_air_loops || o.hvac.sim_zone_equipment) && iteration_air <= 5 {
                iteration_air += 1;
                // Actual ResimulateAirZone local false; positive AFN child must implement its by-ref result.
                let resimulate_air_zone = false;
                if o.controller.subsystems.airflow_network.control_type != 0 {
                    return Err(Error::Unavailable(
                        "actual AFN balance by-reference ResimulateAirZone1881",
                    ));
                }
                if o.hvac.sim_air_loops {
                    children.run_simulation(
                        Child::ManageAirLoops {
                            first_hvac_iteration,
                        },
                        self,
                        o,
                        eio,
                    )?;
                    o.hvac.sim_electric_circuits = true;
                }
                if self.flow_resolution_needed {
                    o.hvac.sim_zone_equipment = true;
                }
                if o.hvac.sim_zone_equipment {
                    if iteration_air == 1 && !self.flow_max_available_already_reset {
                        self.flow_resolution_needed = true;
                    } else {
                        self.resolve_source_air_loop_flow_limits(o)?;
                        self.flow_resolution_needed = false;
                    }
                    children.run_simulation(
                        Child::ManageZoneEquipment {
                            first_hvac_iteration,
                        },
                        self,
                        o,
                        eio,
                    )?;
                    o.hvac.sim_electric_circuits = true;
                }
                self.flow_max_available_already_reset = false;
                if o.controller.subsystems.airflow_network.control_type != 0 && resimulate_air_zone
                {
                    o.hvac.sim_air_loops = true;
                    o.hvac.sim_zone_equipment = true;
                }
                if o.controller.subsystems.duct_loss.simulated && iteration_air < 4 {
                    o.hvac.sim_air_loops = true;
                    o.hvac.sim_zone_equipment = true;
                }
            }
            self.report_iteration_air = self
                .report_iteration_air
                .checked_add(iteration_air)
                .ok_or(Error::Unavailable("actual RepIterAir signed overflow1919"))?;
            self.resolve_source_lockout_flags(o)?;
            if o.hvac.sim_non_zone_equipment {
                children.run_simulation(
                    Child::ManageNonZoneEquipment {
                        first_hvac_iteration,
                    },
                    self,
                    o,
                    eio,
                )?;
                o.hvac.sim_electric_circuits = true;
            }
            if o.hvac.sim_electric_circuits {
                children.run_simulation(
                    Child::ManageElectricPowerService {
                        first_hvac_iteration,
                    },
                    self,
                    o,
                    eio,
                )?;
            }
            if !o.hvac.sim_plant_loops && self.any_source_plant_loop_sides_need_sim()? {
                o.hvac.sim_plant_loops = true;
            }
            if o.hvac.sim_plant_loops {
                self.manage_source_plant_loops(o)?;
            }
            if o.hvac.sim_electric_circuits {
                children.run_simulation(
                    Child::ManageElectricPowerService {
                        first_hvac_iteration,
                    },
                    self,
                    o,
                    eio,
                )?;
            }
        }
        Ok(())
    }
    fn reset_source_hvac_control(&self, o: &SourceHvacOwners<'_, '_>) -> Result<(), Error> {
        if o.hvac.number_of_primary_air_systems == 0 {
            return Ok(());
        }
        Err(Error::Unavailable(
            "actual ResetHVACControl AirLoop controls2141",
        ))
    }
    fn reset_source_terminal_unit_flow_limits(
        &self,
        o: &SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual terminal-unit Node flow limits1965",
            ));
        }
        Ok(())
    }
    fn resolve_source_air_loop_flow_limits(
        &self,
        o: &SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable("actual ResolveAirLoopFlowLimits row"));
        }
        Ok(())
    }
    fn resolve_source_lockout_flags(&self, o: &SourceHvacOwners<'_, '_>) -> Result<(), Error> {
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable("actual ResolveLockoutFlags row"));
        }
        Ok(())
    }
    fn check_source_air_loop_flow_balance(
        &self,
        o: &SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if !o.clock.warmup_flag
            && o.hvac.air_loops_sim_once
            && o.hvac.number_of_primary_air_systems > 0
        {
            return Err(Error::Unavailable("actual CheckAirLoopFlowBalance row3054"));
        }
        Ok(())
    }
    fn update_source_zone_inlet_convergence(
        &self,
        o: &SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        // The real InitAirLoops1931 must allocate and initialize each ZoneInletConvergence record.
        for number in 1..=o.globals.num_of_zones {
            let at = super::index(number)?;
            let row = o
                .project
                .convergence
                .zone_inlet_convergence
                .as_ref()
                .and_then(|r| r.get(at))
                .ok_or(Error::Unavailable(
                    "actual ZoneInletConvergence allocation3024",
                ))?;
            if row.number_inlet_nodes > 0 {
                return Err(Error::Unavailable(
                    "actual ZoneInletConvergence Node history3026",
                ));
            }
        }
        Ok(())
    }
}
