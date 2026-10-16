//! Shared actual OutputProcessor GetMeterIndex4097/GetInstantMeterValue4141 kernels.
use super::{SourceOutputRegistration, SourceRealOutputOwners, TimeStepKind};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
impl SourceOutputRegistration {
    /// Native GetCurrentMeterValue4128 reads CurTSValue, without recomputing Which.
    pub(crate) fn current_source_meter_value(&self, index: i32) -> Result<f64, Error> {
        if index == -1 {
            return Ok(0.0);
        }
        let ordinal = usize::try_from(index)
            .map_err(|_| Error::Unavailable("actual defined current meter ordinal4138"))?;
        let meter = self
            .meters
            .meters
            .get(ordinal)
            .ok_or(Error::Unavailable("same actual current meter4138"))?;
        Ok(meter.current_time_step_value)
    }
    pub(crate) fn source_meter_index(&self, name: &str) -> Result<i32, Error> {
        match self.meters.meter_map.get(name) {
            Some(&index) => {
                i32::try_from(index).map_err(|_| Error::Unavailable("actual int meter index4097"))
            }
            None => Ok(-1),
        }
    }
    pub(crate) fn instant_source_meter_value(
        &self,
        index: i32,
        time_step: TimeStepKind,
        owners: &dyn SourceRealOutputOwners,
    ) -> Result<f64, Error> {
        let mut value = 0.0;
        if index == -1 {
            return Ok(value);
        }
        let meter = self
            .meters
            .meters
            .get(
                usize::try_from(index)
                    .map_err(|_| Error::Unavailable("actual defined meter ordinal4165"))?,
            )
            .ok_or(Error::Unavailable("same actual meter4165"))?;
        // Actual represented Normal0 only; Custom/CustomDec remain real unavailable branches.
        if meter.meter_type as i32 != 0 {
            return Err(Error::Unavailable("actual unsupported meter type4167"));
        }
        for &source in &meter.source_variable_numbers {
            let var = self
                .out_vars
                .get(source)
                .ok_or(Error::Unavailable("same actual meter OutVar4169"))?;
            if var.time_step_type != time_step {
                continue;
            }
            let target = var
                .which
                .ok_or(Error::Unavailable("actual real OutVar Which4177"))?;
            value += *owners.real_output(target).map_err(Error::Output)?
                * f64::from(var.zone_multiplier)
                * f64::from(var.zone_list_multiplier);
        }
        Ok(value)
    }
}
