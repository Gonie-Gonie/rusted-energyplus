//! Actual HeatBalanceManager140-238 orchestration on the same live run owners.
//! The real session must implement every required child; no successful stub exists.
use super::first_call::{
    SourceManageHeatBalanceFirstCallChildren, prepare_source_manage_heat_balance_first_call,
};
use crate::heat_balance::ordinary_ems_plugin::{
    SourceHeatBalanceEmsCallingPoint as Point, SourcePluginData, manage_source_heat_balance_ems,
};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::weather::day::WeatherGlobalState;

/// Existing genuine Init/Surface/RecKeep/Report/Check/report helpers are invoked
/// by a per-call concrete session host, borrowing their original persistent owners.
pub(crate) trait SourceManageHeatBalanceBodyChildren {
    fn globals(
        &self,
    ) -> &crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
    fn clock(&self) -> &WeatherGlobalState;
    fn clock_mut(&mut self) -> &mut WeatherGlobalState;
    fn initialize_heat_balance(&mut self) -> Result<(), Error>;
    fn manage_surface_heat_balance(&mut self) -> Result<(), Error>;
    fn record_heat_balance(&mut self) -> Result<(), Error>;
    fn report_heat_balance(&mut self) -> Result<(), Error>;
    fn update_ems_trend_variables(&mut self) -> Result<(), Error>;
    fn plugin_data(&mut self) -> &mut SourcePluginData;
    fn check_warmup_convergence(&mut self) -> Result<(), Error>;
    fn report_warmup_convergence(&mut self) -> Result<(), Error>;
}
#[allow(
    dead_code,
    reason = "Keep the full HeatBalanceManager first-call/body entry contract; the real session deliberately invokes its input prefix and same body separately to acquire live owners."
)]
pub(crate) trait SourceManageHeatBalanceChildren:
    SourceManageHeatBalanceFirstCallChildren + SourceManageHeatBalanceBodyChildren
{
}
impl<T: SourceManageHeatBalanceFirstCallChildren + SourceManageHeatBalanceBodyChildren>
    SourceManageHeatBalanceChildren for T
{
}

#[allow(
    dead_code,
    reason = "Keep the full HeatBalanceManager first-call/body entry contract; the real session deliberately invokes its input prefix and same body separately to acquire live owners."
)]
pub(crate) fn manage_source_heat_balance(
    children: &mut impl SourceManageHeatBalanceChildren,
) -> Result<(), Error> {
    prepare_source_manage_heat_balance_first_call(children)?; //167-187
    manage_source_heat_balance_after_input(children)
}

/// Same Native189-238 suffix; session constructs live body views only after
/// real GetHBInput has created the actual Zone/Solar/equipment owners.
pub(crate) fn manage_source_heat_balance_after_input(
    children: &mut impl SourceManageHeatBalanceBodyChildren,
) -> Result<(), Error> {
    let mut any_ran = None; //Actual unwritten anyRan189, first real EMS call writes false.
    manage_source_heat_balance_ems(
        children.globals(),
        Point::BeginZoneTimestepBeforeInitHeatBalance,
        &mut any_ran,
    )?; //191
    children.initialize_heat_balance()?; //198
    manage_source_heat_balance_ems(
        children.globals(),
        Point::BeginZoneTimestepAfterInitHeatBalance,
        &mut any_ran,
    )?; //199
    children.manage_surface_heat_balance()?; //209
    manage_source_heat_balance_ems(
        children.globals(),
        Point::EndZoneTimestepBeforeZoneReporting,
        &mut any_ran,
    )?; //210
    children.record_heat_balance()?; //211
    children.report_heat_balance()?; //217
    manage_source_heat_balance_ems(
        children.globals(),
        Point::EndZoneTimestepAfterZoneReporting,
        &mut any_ran,
    )?; //219
    children.update_ems_trend_variables()?; //221
    children.plugin_data().update_source_plugin_values()?; //222, unconditional actual vector traversal
    if children.clock().warmup_flag && children.clock().end_day_flag {
        children.check_warmup_convergence()?; //226; may change actual WarmupFlag
        if !children.clock().warmup_flag {
            let clock = children.clock_mut();
            clock.day_of_sim = 0; //229
            clock.day_of_sim_chr = "0".to_owned(); //230
            manage_source_heat_balance_ems(
                children.globals(),
                Point::BeginNewEnvironmentAfterWarmup,
                &mut any_ran,
            )?; //231
        }
    }
    if !children.clock().warmup_flag
        && children.clock().end_day_flag
        && children.clock().day_of_sim == 1
        && !children.globals().doing_sizing
    {
        children.report_warmup_convergence()?; //236
    }
    Ok(())
}
