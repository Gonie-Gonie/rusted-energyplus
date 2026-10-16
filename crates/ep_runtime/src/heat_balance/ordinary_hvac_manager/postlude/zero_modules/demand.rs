//! Actual ManageDemand97-190 and zero-input GetDemandManagerInput/List children.
use super::{Error, SourceDemandManager};
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
use crate::weather::day::WeatherGlobalState;
mod startup;
impl SourceDemandManager {
    pub(crate) fn manage_source_demand(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        clock: &WeatherGlobalState,
        input: &SelectedInputProcessor,
    ) -> Result<(), Error> {
        if self.get_input && !globals.doing_sizing {
            self.get_source_manager_input(input)?;
            self.get_source_list_input(input)?;
            self.get_input = false;
        }
        if self.number_lists > 0 {
            if clock.warmup_flag {
                self.begin_simulation = true;
                if self.clear_history {
                    return Err(Error::Unavailable(
                        "actual DemandManagerList.History first write120",
                    ));
                }
                self.clear_history = false;
            }
            if !clock.warmup_flag && !globals.doing_sizing {
                if self.begin_simulation {
                    self.begin_simulation = false;
                    self.clear_history = true;
                }
                self.exterior_iterations = 0;
                self.heat_balance_iterations = 0;
                self.hvac_iterations = 0;
                self.first_time = true;
                self.resimulate_exterior = false;
                self.resimulate_heat_balance = false;
                self.resimulate_hvac = false;
                // Native while enters from the actual firstTime=true assignment above.
                self.first_time = false;
                return Err(Error::Unavailable(
                    "actual positive-list Resimulate child162",
                ));
            }
        }
        Ok(())
    }
    fn get_source_manager_input(&mut self, input: &SelectedInputProcessor) -> Result<(), Error> {
        // Native two automatic Objexx arrays remain unallocated on this route.
        let _alpha_arguments: Option<Vec<String>> = None;
        let _numeric_arguments: Option<Vec<f64>> = None;
        let _max_alphas = 0_i32;
        let _max_numbers = 0_i32;
        let mut counts = [0_i32; 5];
        for (at, (family, definition_boundary)) in [
            (
                "DemandManager:ExteriorLights",
                "actual DemandManager:ExteriorLights definition561",
            ),
            (
                "DemandManager:Lights",
                "actual DemandManager:Lights definition568",
            ),
            (
                "DemandManager:ElectricEquipment",
                "actual DemandManager:ElectricEquipment definition575",
            ),
            (
                "DemandManager:Thermostats",
                "actual DemandManager:Thermostats definition582",
            ),
            (
                "DemandManager:Ventilation",
                "actual DemandManager:Ventilation definition589",
            ),
        ]
        .into_iter()
        .enumerate()
        {
            counts[at] = i32::try_from(input.get_num_objects_found(family))
                .map_err(|_| Error::Unavailable("actual DemandManager count int domain"))?;
            if counts[at] > 0 {
                return Err(Error::Unavailable(definition_boundary));
            }
        }
        self.number_managers = counts[0]
            .checked_add(counts[1])
            .and_then(|n| n.checked_add(counts[2]))
            .and_then(|n| n.checked_add(counts[3]))
            .and_then(|n| n.checked_add(counts[4]))
            .ok_or(Error::Unavailable(
                "actual total NumDemandMgr signed domain594",
            ))?;
        // Native AlphArray/NumArray dimension600/601, DemandMgr allocate605,
        // UniqueNames.reserve606, all five record loops and deallocations1129/30
        // are inside the actual NumDemandMgr>0 guard. ErrorsFound remains false.
        Ok(())
    }
    fn get_source_list_input(&mut self, input: &SelectedInputProcessor) -> Result<(), Error> {
        // Native cCurrentModuleObject is constexpr local, not the shared shortcut.
        self.number_lists =
            i32::try_from(input.get_num_objects_found("DemandManagerAssignmentList")).map_err(
                |_| Error::Unavailable("actual DemandManagerAssignmentList count int domain321"),
            )?;
        if self.number_lists > 0 {
            return Err(Error::Unavailable("actual DemandManagerList allocation330"));
        }
        // All record getters and the three iteration output declarations507-529
        // are inside NumDemandManagerList>0. No output/scratch/flag is fabricated.
        Ok(())
    }
}
