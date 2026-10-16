//! Actual ExhaustControl/ExhaustSystem/ReturnAir module flags and empty-input child effects.
use crate::heat_balance::ordinary_hvac_manager::SourceHvacOwners;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{
    SelectedInputProcessor, SourceInputProcessorShortCut,
};
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceEquipmentManagementModules {
    pub(crate) get_exhaust_system_input: bool,
    pub(crate) exhaust_mixer_index_map: std::collections::BTreeMap<i32, i32>,
    pub(crate) exhaust_mapping_done: bool,
    pub(crate) get_exhaust_control_input: bool,
    pub(crate) get_return_air_path_input: bool,
}
impl Default for SourceEquipmentManagementModules {
    fn default() -> Self {
        Self {
            get_exhaust_system_input: true,
            exhaust_mixer_index_map: Default::default(),
            exhaust_mapping_done: false,
            get_exhaust_control_input: true,
            get_return_air_path_input: true,
        }
    }
}
impl SourceEquipmentManagementModules {
    pub(super) fn simulate_source_exhaust_controls(
        &mut self,
        input: &SelectedInputProcessor,
        number_controls: i32,
    ) -> Result<(), Error> {
        if self.get_exhaust_control_input {
            if input.find_exact_family("ZoneHVAC:ExhaustControl").is_some() {
                return Err(Error::Unavailable("actual ExhaustControl schemaProps378"));
            }
            self.get_exhaust_control_input = false;
        }
        if number_controls > 0 {
            return Err(Error::Unavailable("actual CalcZoneHVACExhaustControl550"));
        }
        Ok(())
    }
    pub(super) fn simulate_source_exhaust_system(
        &mut self,
        input: &SelectedInputProcessor,
        number_systems: i32,
        number_controls: i32,
    ) -> Result<(), Error> {
        if self.get_exhaust_system_input {
            // Actual inner GetExhaustAirSystemInput has the same flag test104.
            if self.get_exhaust_system_input
                && input
                    .find_exact_family("AirLoopHVAC:ExhaustSystem")
                    .is_some()
            {
                return Err(Error::Unavailable("actual ExhaustSystem schemaProps118"));
            }
            self.get_exhaust_system_input = false;
        }
        if number_systems > 0 {
            return Err(Error::Unavailable("actual CalcExhaustAirSystem92"));
        }
        if number_controls > 0 {
            return Err(Error::Unavailable("actual UpdateZoneExhaustControl743"));
        }
        Ok(())
    }
    pub(super) fn simulate_source_return_air_paths(
        &mut self,
        number_paths: &mut usize,
        paths: &Option<Vec<()>>,
        input: &SelectedInputProcessor,
        shortcut: &mut SourceInputProcessorShortCut,
    ) -> Result<(), Error> {
        if self.get_return_air_path_input {
            // Genuine allocated-empty early input array returns before cCurrentModuleObject assignment.
            if paths.is_none() {
                shortcut.current_module_object = "AirLoopHVAC:ReturnPath".into();
                *number_paths = input.get_num_objects_found(&shortcut.current_module_object);
                if *number_paths > 0 {
                    return Err(Error::Unavailable("actual ReturnAirPath allocation132"));
                }
            }
            self.get_return_air_path_input = false;
        }
        if *number_paths > 0 {
            return Err(Error::Unavailable("actual CalcReturnAirPath99"));
        }
        Ok(())
    }
}
pub(super) fn simulate_source_duct_loss_default(
    o: &mut SourceHvacOwners<'_, '_>,
) -> Result<(), Error> {
    let duct = &mut o.controller.subsystems.duct_loss;
    if duct.get_input_flag {
        let conduction = o.input.get_num_objects_found("Duct:Loss:Conduction");
        let leakage = o.input.get_num_objects_found("Duct:Loss:Leakage");
        let makeup = o.input.get_num_objects_found("Duct:Loss:MakeupAir");
        let total = conduction
            .checked_add(leakage)
            .and_then(|n| n.checked_add(makeup))
            .ok_or(Error::Unavailable(
                "actual duct input count addition domain",
            ))?;
        if total > 0 {
            return Err(Error::Unavailable(
                "actual positive duct-input AirLoop query198",
            ));
        }
        duct.get_input_flag = false;
    }
    // Literal no-argument caller4187 uses actual default PathNum=0; return89 occurs before any Node/airLoop read.
    Ok(())
}
