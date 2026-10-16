//! Seven real out-parameter writes and Native early-return/body predicates.
use super::*;
impl SourceLowTemperatureRadiantAverages {
    pub(super) fn update_source_value_average(
        &mut self,
        on: &mut Option<bool>,
    ) -> Result<(), Error> {
        *on = Some(false); // LowTemp5825..5852, before count read.
        if self.total_num_of_radiant_systems == 0 {
            return Ok(());
        }
        Err(Error::Unavailable(
            "UpdateRadSysSourceValAvg actual QRadSysSource whole zero5861",
        ))
    }
}
impl SourceHighTemperatureRadiantAverages {
    pub(super) fn update_source_value_average(
        &mut self,
        on: &mut Option<bool>,
    ) -> Result<(), Error> {
        *on = Some(false); // HighTemp1118, before count read.
        if self.number_of_high_temperature_radiant_systems == 0 {
            return Ok(());
        }
        Err(Error::Unavailable(
            "UpdateHTRadSourceValAvg actual HighTempRadSys array1126",
        ))
    }
}
impl SourceHotWaterBaseboardAverages {
    pub(super) fn update_source_value_average(
        &mut self,
        on: &mut Option<bool>,
    ) -> Result<(), Error> {
        *on = Some(false);
        if self.number_of_hot_water_baseboards == 0 {
            return Ok(());
        }
        Err(Error::Unavailable(
            "UpdateBBRadSourceValAvg actual HWBaseboard array1450",
        ))
    }
}
impl SourceSteamBaseboardAverages {
    pub(super) fn update_source_value_average(
        &mut self,
        on: &mut Option<bool>,
    ) -> Result<(), Error> {
        *on = Some(false);
        if self.number_of_steam_baseboards == 0 {
            return Ok(());
        }
        Err(Error::Unavailable(
            "UpdateBBSteamRadSourceValAvg actual SteamBaseboard array1435",
        ))
    }
}
impl SourceElectricBaseboardAverages {
    pub(super) fn update_source_value_average(
        &mut self,
        on: &mut Option<bool>,
    ) -> Result<(), Error> {
        *on = Some(false);
        if self.number_of_electric_baseboards == 0 {
            return Ok(());
        }
        Err(Error::Unavailable(
            "UpdateBBElecRadSourceValAvg actual ElecBaseboard array865",
        ))
    }
}
impl SourceCoolingPanelAverages {
    pub(super) fn update_source_value_average(
        &mut self,
        on: &mut Option<bool>,
    ) -> Result<(), Error> {
        *on = Some(false);
        let Some(panels) = self.panels.as_mut() else {
            return Ok(());
        };
        for panel in panels.iter() {
            if panel.source_average != 0.0 {
                *on = Some(true);
                break;
            }
        }
        for panel in panels {
            panel.source = panel.source_average;
        }
        // This is reached for allocated-empty as well. No count/default shortcut.
        Err(Error::Unavailable(
            "UpdateCoolingPanelSourceValAvg actual DistributeCoolingPanelRadGains1545",
        ))
    }
}
impl SourceSwimmingPoolAverages {
    pub(super) fn update_source_value_average(
        &mut self,
        on: &mut Option<bool>,
    ) -> Result<(), Error> {
        *on = Some(false);
        if self.number_of_swimming_pools == 0 {
            return Ok(());
        }
        Err(Error::Unavailable(
            "UpdatePoolSourceValAvg actual Pool/QRadSrc average loop1128",
        ))
    }
}
