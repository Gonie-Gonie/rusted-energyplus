//! Actual first-input/init/predict dispatch state and mandatory registration children.
use super::*;
mod controls;
mod demand;
mod environment;
mod initialization;
mod input;
mod prediction;
mod thermostat_predefined;
pub(crate) use controls::{
    SourceAirThermostatArrays, SourceZoneAirControls, SourceZoneAirFlags,
    SourceZoneListGroupLoadReports,
};
pub(crate) enum SourceAirUpdate {
    #[allow(
        dead_code,
        reason = "DataHeatBalFanSys.hh PredictorCorrectorCtrl retains invalid/revert dispatch identities even when the selected production caller uses the other update routes."
    )]
    Invalid,
    GetZoneSetPoints,
    PredictStep,
    CorrectStep,
    #[allow(
        dead_code,
        reason = "DataHeatBalFanSys.hh PredictorCorrectorCtrl retains invalid/revert dispatch identities even when the selected production caller uses the other update routes."
    )]
    RevertZoneHistories,
    PushZoneHistories,
    PushSystemHistories,
}
/// Real setUpOutputVars children; implementation must retain actual live owner targets.
/// No default implementation or successful empty host is provided.
pub(crate) trait SourceZoneAirRegistrationChildren {
    fn register_air(
        &mut self,
        number: i32,
        space: bool,
        name: &str,
        air: &SourceZoneSpaceHeatBalanceData,
        clock: &WeatherGlobalState,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error>;
    fn register_sensible_demand(
        &mut self,
        number: i32,
        space: bool,
        name: &str,
        staged: bool,
        attach_meters: bool,
        zone_multiplier: i32,
        list_multiplier: i32,
        demand: &super::demand::SourceSystemSensibleDemand,
        clock: &WeatherGlobalState,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error>;
    fn register_moisture_demand(
        &mut self,
        number: i32,
        space: bool,
        name: &str,
        latent_sizing: bool,
        demand: &super::demand::SourceSystemMoistureDemand,
        clock: &WeatherGlobalState,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error>;
    fn register_thermostat(
        &mut self,
        number: i32,
        name: &str,
        fan: &super::super::environment::SourceZoneFanHeatBalanceArrays,
        clock: &WeatherGlobalState,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error>;
}

impl SourceZoneAirPredictorCorrector {
    /// Genuine dispatcher195-244; init and first input precede every update branch.
    pub(crate) fn manage_source_zone_air_updates(
        &mut self,
        update: SourceAirUpdate,
        temperature_change: &mut f64,
        shorten: bool,
        use_zone_history: bool,
        prior_time_step: f64,
        air: &mut SourceAirHeatBalance,
        owners: &mut SourceZoneAirOwners<'_, '_>,
        outputs: &mut impl SourceZoneAirRegistrationChildren,
        eio: &mut dyn std::io::Write,
    ) -> Result<(), Error> {
        if self.controls.get_zone_air_stats_input {
            self.get_source_zone_air_setpoints(owners, eio)?;
            self.controls.get_zone_air_stats_input = false;
        }
        self.initialize_source_air_setpoints(air, owners, outputs, eio)?;
        match update {
            SourceAirUpdate::GetZoneSetPoints => {
                self.calculate_source_air_temperature_setpoints(owners)?
            }
            SourceAirUpdate::PredictStep => self.predict_source_system_loads(
                shorten,
                use_zone_history,
                prior_time_step,
                air,
                owners,
            )?,
            SourceAirUpdate::CorrectStep => {
                *temperature_change =
                    air.correct_source_zone_air_temperatures(use_zone_history, self, owners)?
            }
            SourceAirUpdate::RevertZoneHistories => {
                air.revert_source_zone_timestep_histories(self, owners)?
            }
            SourceAirUpdate::PushZoneHistories => {
                air.push_source_zone_timestep_histories(self, owners)?
            }
            SourceAirUpdate::PushSystemHistories => {
                air.push_source_system_timestep_histories(self, owners)?
            }
            SourceAirUpdate::Invalid => {}
        }
        Ok(())
    }
}
