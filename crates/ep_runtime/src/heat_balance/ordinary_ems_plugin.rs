//! Actual EMS quick-return and Plugin trend children used by ManageHeatBalance.
//! Constructor-empty plugin trends are read from the genuine retained module.
use super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use super::selected_heat_balance_input::HeatBalanceInputError as Error;
use super::selected_simulation_startup::SelectedRuntimeLanguage;
use std::convert::Infallible;
mod endrun;

/// Actual EMSManager.hh calling points used by simulation, heat balance and weather.
#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SourceHeatBalanceEmsCallingPoint {
    BeginNewEnvironment = 2,
    BeginNewEnvironmentAfterWarmup = 3,
    EndZoneTimestepBeforeZoneReporting = 10,
    EndZoneTimestepAfterZoneReporting = 11,
    SetupSimulation = 12,
    BeginZoneTimestepBeforeInitHeatBalance = 17,
    BeginZoneTimestepAfterInitHeatBalance = 18,
    BeginZoneTimestepBeforeSetCurrentWeather = 19,
}
/// Native263 writes anyProgramRan before the real AnyEMS quick-return predicate.
/// None represents the caller's unwritten local; it never supplies a probe value.
pub(crate) fn manage_source_heat_balance_ems(
    globals: &SelectedHeatBalanceGlobals,
    calling_point: SourceHeatBalanceEmsCallingPoint,
    any_program_ran: &mut Option<bool>,
) -> Result<(), Error> {
    *any_program_ran = Some(false);
    if !globals.any_energy_management_system_in_model {
        return Ok(());
    }
    if calling_point == SourceHeatBalanceEmsCallingPoint::BeginNewEnvironment {
        // Native269 precedes Plugin::onBeginEnvironment270 and InitEMS273.
        // Positive EMS requires that same genuine runtime-language initializer.
        return Err(Error::Unavailable(
            "actual positive EMS BeginEnvrnInitializeRuntimeLanguage269",
        ));
    }
    // The other seven actual calling points first reach the existing InitEMS
    // boundary, before plugins, programs and actuator/report operations.
    Err(Error::Unavailable(
        "actual positive ManageHeatBalance EMS InitEMS274",
    ))
}
impl SelectedRuntimeLanguage {
    pub(crate) fn update_source_ems_trend_variables(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
    ) -> Result<(), Error> {
        if !globals.any_energy_management_system_in_model {
            return Ok(());
        }
        if self.number_of_erl_trend_variables == 0 {
            return Ok(());
        }
        if self.number_of_erl_trend_variables >= 1 {
            return Err(Error::Unavailable("actual EMS TrendVariable first row1468"));
        }
        // Native negative count has an empty signed 1..count loop as well.
        Ok(())
    }
}
/// Actual PluginManagerData237 std::vector ctor-empty; not maxIndex=-1 inference.
/// Positive PluginTrendVariable construction requires native arguments not admitted here.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SourcePluginData {
    /// Actual PluginManager.hh242 constructorfalse, written by Simulation prelude.
    pub(crate) fully_ready: bool,
    trends: Vec<Infallible>,
}
impl SourcePluginData {
    pub(crate) fn update_source_plugin_values(&mut self) -> Result<(), Error> {
        // LINK_WITH_PYTHON body1461 first traverses this genuine vector; the
        // no-Python body is empty. Both actual admitted empty-vector paths agree.
        if !self.trends.is_empty() {
            return Err(Error::Unavailable(
                "actual Plugin trend getGlobalVariableValue1462",
            ));
        }
        Ok(())
    }
}
