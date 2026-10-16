//! Actual ManageHeatBalance167--187 first-call child, prior to all timestep consumers.
use super::SourceHeatBalanceManager;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, surface::SurfaceGeometryInputState,
};
use crate::heat_balance::selected_heat_balance_space_controls::SelectedHeatBalanceSpaceControls;
use crate::heat_balance::selected_input_processor::SelectedInputProcessor;
/// Every accessor borrows the actual same caller owner; GetHBInput is mandatory.
/// This host must never replay an old input preparation or seed the manager flag.
pub(crate) trait SourceManageHeatBalanceFirstCallChildren {
    fn manager(&self) -> &SourceHeatBalanceManager;
    fn manager_mut(&mut self) -> &mut SourceHeatBalanceManager;
    fn get_heat_balance_input(&mut self) -> Result<(), Error>;
    fn globals(&self) -> Result<&SelectedHeatBalanceGlobals, Error>;
    fn space_controls(&mut self) -> &mut SelectedHeatBalanceSpaceControls;
    fn geometry(&mut self) -> &mut SurfaceGeometryInputState;
    fn input(&self) -> Result<&SelectedInputProcessor, Error>;
}
pub(crate) fn prepare_source_manage_heat_balance_first_call(
    children: &mut impl SourceManageHeatBalanceFirstCallChildren,
) -> Result<(), Error> {
    if children.manager().manage_heat_balance_get_input_flag {
        children.get_heat_balance_input()?; //168, preceding all first-call suffix writes.
        if children.globals()?.doing_sizing {
            let spaces = children.space_controls();
            spaces.current = spaces.sizing; //170, same DataHB controls, no copied flags.
        }
        if children.geometry().total_surfaces >= 100 {
            //literal Dayltg octreeCrossover
            if children
                .input()?
                .get_num_objects_found("Daylighting:Controls")
                > 0
            {
                return Err(Error::Unavailable(
                    "actual surfaceOctree.init(SAME Surface)178",
                ));
            }
        }
        children.geometry().set_source_computed_surface_geometry()?; //182--184
        children.manager_mut().manage_heat_balance_get_input_flag = false; //186, success-only
    }
    Ok(())
}
