//! Genuine HeatBalanceAirManager wrappers, same retained controller and HBAM flags.
//! Hosts must reborrow the actual run state for every required child; no defaults.
use super::*;

/// Borrow splitting only: each child must execute its actual Source body on the
/// same run owners. Positive external HVAC and unported report/PSY children are
/// explicit host obligations, never silently replaced by the ordinary route.
pub(crate) trait SourceManageAirHeatBalanceChildren {
    fn get_air_heat_balance_input(
        &mut self,
        controller: &mut SourceZoneAirPredictorCorrector,
    ) -> Result<(), Error>;
    fn heat_balance_control(&self) -> &SelectedHeatBalanceInputState;
    fn calculate_heat_balance_air(
        &mut self,
        controller: &mut SourceZoneAirPredictorCorrector,
    ) -> Result<(), Error>;
    fn report_zone_mean_air_temperature(
        &mut self,
        controller: &mut SourceZoneAirPredictorCorrector,
    ) -> Result<(), Error>;
}

impl SourceZoneAirPredictorCorrector {
    /// Exact ManageAirHeatBalance135-161. The input flag stays true if any
    /// reached input child fails, and stays false if a later step child fails.
    pub(crate) fn manage_source_air_heat_balance(
        &mut self,
        children: &mut impl SourceManageAirHeatBalanceChildren,
    ) -> Result<(), Error> {
        if self.simple_air.manage_air_heat_balance_get_input_flag {
            children.get_air_heat_balance_input(self)?;
            self.simple_air.manage_air_heat_balance_get_input_flag = false;
        }
        self.initialize_source_air_heat_balance(children.heat_balance_control())?;
        self.calculate_source_heat_balance_air(children)?;
        self.report_source_zone_mean_air_temperature(children)
    }

    /// Exact InitAirHeatBalance4494-4507 invokes the genuine same-owner
    /// InitSimpleMixingConvectiveHeatGains, including its real positive stops.
    pub(crate) fn initialize_source_air_heat_balance(
        &mut self,
        control: &SelectedHeatBalanceInputState,
    ) -> Result<(), Error> {
        self.simple_air.initialize_source_simple_mixing(control)
    }

    /// The mandatory host owns actual external-HVAC pointer selection4596 or
    /// genuine ordinary ManageHVAC4602. A default/empty callback is unavailable.
    pub(crate) fn calculate_source_heat_balance_air(
        &mut self,
        children: &mut impl SourceManageAirHeatBalanceChildren,
    ) -> Result<(), Error> {
        children.calculate_heat_balance_air(self)
    }

    /// Mandatory actual ReportZoneMeanAirTemp4615-4728 child. Its one-time flag
    /// is retained on the same HBAM owner, and this wrapper never clears it.
    pub(crate) fn report_source_zone_mean_air_temperature(
        &mut self,
        children: &mut impl SourceManageAirHeatBalanceChildren,
    ) -> Result<(), Error> {
        children.report_zone_mean_air_temperature(self)
    }
}
