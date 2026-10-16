//! Genuine UpdateFinalSurfaceHeatBalance5176 and seven same-module average children.
//! Constructor fields are retained module state; no typed absence or default callback.
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
mod state;
mod updates;
pub(crate) use state::*;
impl SourceFinalSurfaceHeatBalance {
    /// Exact all-seven call order. Native local bools stay unwritten until each
    /// actual child writes its out parameter. This method never queries input.
    pub(crate) fn update_final_surface_heat_balance(&mut self) -> Result<(), Error> {
        let mut low = None;
        let mut high = None;
        let mut water = None;
        let mut steam = None;
        let mut electric = None;
        let mut cooling = None;
        let mut pool = None;
        self.low_temperature.update_source_value_average(&mut low)?;
        self.high_temperature
            .update_source_value_average(&mut high)?;
        self.hot_water_baseboard
            .update_source_value_average(&mut water)?;
        self.steam_baseboard
            .update_source_value_average(&mut steam)?;
        self.electric_baseboard
            .update_source_value_average(&mut electric)?;
        self.cooling_panel
            .update_source_value_average(&mut cooling)?;
        self.swimming_pool.update_source_value_average(&mut pool)?;
        if flag(low)?
            || flag(high)?
            || flag(water)?
            || flag(steam)?
            || flag(electric)?
            || flag(cooling)?
            || flag(pool)?
        {
            return Err(Error::Unavailable(
                "UpdateFinalSurfaceHeatBalance actual CalcHeatBalanceOutsideSurf5215 re-simulation",
            ));
        }
        Ok(())
    }
}
fn flag(value: Option<bool>) -> Result<bool, Error> {
    value.ok_or(Error::Unavailable(
        "actual UpdateSourceValAvg out parameter is unwritten",
    ))
}
