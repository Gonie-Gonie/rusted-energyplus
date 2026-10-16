//! Actual PlantManager and PlantUtilities empty-loop branches, including real input effects.
use super::super::{Error, SourceHvacManager, SourceHvacOwners};
impl SourceHvacManager {
    pub(super) fn get_source_plant_loop_data(
        &mut self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        // Native automatic Array1D_string Alpha(19), Array1D<Real64> Num(30).
        let _alpha = vec![String::new(); 19];
        let _numbers: Vec<Option<f64>> = vec![None; 30];
        let mut current_module_object = String::from("PlantLoop");
        o.hvac.number_plant_loops =
            i32::try_from(o.input.get_num_objects_found(&current_module_object))
                .map_err(|_| Error::Unavailable("actual PlantLoop count int domain320"))?;
        current_module_object = "CondenserLoop".into();
        o.hvac.number_condenser_loops =
            i32::try_from(o.input.get_num_objects_found(&current_module_object))
                .map_err(|_| Error::Unavailable("actual CondenserLoop count int domain324"))?;
        self.simulation.plant.total_loops = o
            .hvac
            .number_plant_loops
            .checked_add(o.hvac.number_condenser_loops)
            .ok_or(Error::Unavailable(
                "actual Plant total-loop signed overflow326",
            ))?;
        if self.simulation.plant.total_loops > 0 {
            return Err(Error::Unavailable(
                "actual positive PlantLoop allocation329",
            ));
        }
        // Native return335 does not allocate/reallocate the already Startup-owned PlantLoop.
        Ok(())
    }
    pub(super) fn get_source_plant_input(
        &self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        let splitter = o
            .input
            .get_object_def_max_args("Connector:Splitter")
            .map_err(|_| Error::Unavailable("actual Connector:Splitter schema definition818"))?;
        let mut max_num_alphas = splitter.alphas;
        let mixer = o
            .input
            .get_object_def_max_args("Connector:Mixer")
            .map_err(|_| Error::Unavailable("actual Connector:Mixer schema definition820"))?;
        max_num_alphas = max_num_alphas.max(mixer.alphas);
        let _ = max_num_alphas;
        let _half_loop_number = 0;
        if self.simulation.plant.total_loops > 0 {
            return Err(Error::Unavailable(
                "actual GetPlantInput first PlantLoop reference825",
            ));
        }
        // All following loops are genuinely empty; each ventilation allocation has its own >0 guard.
        if o.hvac.number_plant_loops > 0 {
            return Err(Error::Unavailable(
                "actual VentRepPlant Supply allocation1702",
            ));
        }
        if o.hvac.number_condenser_loops > 0 {
            return Err(Error::Unavailable(
                "actual VentRepCond Supply allocation1808",
            ));
        }
        Ok(())
    }
    pub(super) fn setup_source_initial_plant_calling_order(&mut self) -> Result<(), Error> {
        self.simulation.plant.total_half_loops = self
            .simulation
            .plant
            .total_loops
            .checked_mul(2)
            .ok_or(Error::Unavailable(
            "actual TotNumHalfLoops signed overflow3571",
        ))?;
        if self.simulation.plant.total_half_loops <= 0 {
            return Ok(());
        }
        Err(Error::Unavailable(
            "actual PlantCallingOrderInfo allocation3580",
        ))
    }
    pub(super) fn setup_source_plant_branch_control_types(
        &self,
        o: &SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        let count = o.plant_loops.as_ref().map_or(0, Vec::len);
        if count > 0 {
            return Err(Error::Unavailable(
                "actual SetupBranchControlTypes first Loop row3775",
            ));
        }
        Ok(())
    }
    pub(super) fn setup_source_plant_reports(
        &self,
        o: &SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        let _max_branches = 0;
        if o.plant_loops.as_ref().is_some_and(|v| !v.is_empty()) {
            return Err(Error::Unavailable(
                "actual SetupReports first PlantLoop row1942",
            ));
        }
        if self.simulation.plant.total_loops > 0 {
            return Err(Error::Unavailable("actual SetupReports first output1960"));
        }
        Ok(())
    }
    pub(super) fn reinitialize_source_plant_loops(
        &mut self,
        o: &SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.simulation.plant.environment_flag && o.clock.begin_envrn_flag {
            if self.simulation.plant.total_loops > 0 {
                return Err(Error::Unavailable(
                    "actual Plant begin-environment Loop reference2694",
                ));
            }
            if o.plant_loops.as_ref().is_some_and(|v| !v.is_empty()) {
                return Err(Error::Unavailable(
                    "actual Plant begin-environment EPVector row2834",
                ));
            }
            self.simulation.plant.environment_flag = false;
        }
        if !o.clock.begin_envrn_flag {
            self.simulation.plant.environment_flag = true;
        }
        if self.simulation.plant.total_loops > 0 {
            return Err(Error::Unavailable(
                "actual Plant first-HVAC setpoint dereference2858",
            ));
        }
        Ok(())
    }
    pub(super) fn manage_source_plant_loops(
        &self,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.plant_loops.as_ref().is_some_and(|v| !v.is_empty()) {
            return Err(Error::Unavailable(
                "actual Plant common-pipe any_of predicate183",
            ));
        }
        let _current_minimum_sub_iterations = o.project.convergence.min_plant_sub_iterations;
        if self.simulation.plant.total_loops <= 0 {
            o.hvac.sim_plant_loops = false;
            return Ok(());
        }
        Err(Error::Unavailable(
            "actual positive Plant InitializeLoops195",
        ))
    }
    pub(super) fn set_source_all_plant_sim_flags(
        &self,
        _o: &SourceHvacOwners<'_, '_>,
        _value: bool,
    ) -> Result<(), Error> {
        if self.simulation.plant.total_loops > 0 {
            return Err(Error::Unavailable(
                "actual PlantUtility SimLoopSideNeeded row1926",
            ));
        }
        Ok(())
    }
    pub(super) fn set_source_all_flow_locks(&self, _lock: bool) -> Result<(), Error> {
        if self.simulation.plant.total_loops > 0 {
            return Err(Error::Unavailable("actual PlantUtility FlowLock row854"));
        }
        Ok(())
    }
    pub(super) fn reset_source_plant_interconnect_flags(&self) -> Result<(), Error> {
        if self.simulation.plant.total_loops > 0 {
            return Err(Error::Unavailable(
                "actual PlantUtility interconnect row875",
            ));
        }
        Ok(())
    }
    pub(super) fn any_source_plant_loop_sides_need_sim(&self) -> Result<bool, Error> {
        let any = false;
        if self.simulation.plant.total_loops > 0 {
            return Err(Error::Unavailable(
                "actual AnyPlantLoopSidesNeedSim row1898",
            ));
        }
        Ok(any)
    }
}
