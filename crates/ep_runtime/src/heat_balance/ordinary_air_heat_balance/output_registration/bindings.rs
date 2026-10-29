//! Actual field references for InitZoneAir; no copied report values.
use super::*;

#[derive(Clone, Copy)]
pub(super) enum Record<'a> {
    Air(&'a SourceZoneSpaceHeatBalanceData),
    Sensible(&'a SourceSystemSensibleDemand),
    Moisture(&'a SourceSystemMoistureDemand),
    Thermostat(&'a SourceZoneFanHeatBalanceArrays),
    ZoneListGroup(&'a SourceZoneListGroupLoadReports),
}
pub(super) struct Row<'a> {
    pub(super) number: i32,
    pub(super) space: bool,
    pub(super) record: Record<'a>,
}
fn index(number: i32) -> Result<usize, OrdinaryRadiationError> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or_else(|| OrdinaryRadiationUnavailable::StorageDomain.into())
}
impl<'a> Record<'a> {
    fn real(self, number: i32, field: Field) -> Result<&'a f64, OrdinaryRadiationError> {
        Ok(match self {
            Self::ZoneListGroup(v) => {
                let values = match field {
                    Field::ZoneListSensibleHeatingEnergy => &v.list_heat_energy,
                    Field::ZoneListSensibleCoolingEnergy => &v.list_cool_energy,
                    Field::ZoneListSensibleHeatingRate => &v.list_heat_rate,
                    Field::ZoneListSensibleCoolingRate => &v.list_cool_rate,
                    Field::ZoneGroupSensibleHeatingEnergy => &v.group_heat_energy,
                    Field::ZoneGroupSensibleCoolingEnergy => &v.group_cool_energy,
                    Field::ZoneGroupSensibleHeatingRate => &v.group_heat_rate,
                    Field::ZoneGroupSensibleCoolingRate => &v.group_cool_rate,
                    _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
                };
                values
                    .as_deref()
                    .and_then(|rows| rows.get(index(number).ok()?))
                    .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
            }
            Self::Air(v) => match field {
                Field::AirTemperature => &v.zt,
                Field::AirHumidityRatio => &v.air_hum_rat,
                Field::AirRelativeHumidity => &v.air_rel_hum,
                Field::MeanRadiantTemperature => &v.mrt,
                _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
            },
            Self::Sensible(v) => match field {
                Field::SensibleHeatingEnergy => &v.air_sys_heat_energy,
                Field::SensibleCoolingEnergy => &v.air_sys_cool_energy,
                Field::SensibleHeatingRate => &v.air_sys_heat_rate,
                Field::SensibleCoolingRate => &v.air_sys_cool_rate,
                Field::PredictedSensibleRate => &v.predicted_rate,
                Field::PredictedHeatingSetpointRate => &v.predicted_heating_sp_rate,
                Field::PredictedCoolingSetpointRate => &v.predicted_cooling_sp_rate,
                Field::SystemPredictedSensibleRate => &v.base.total_output_required,
                Field::SystemPredictedHeatingSetpointRate => &v.output_required_to_heating_sp,
                Field::SystemPredictedCoolingSetpointRate => &v.output_required_to_cooling_sp,
                _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
            },
            Self::Moisture(v) => match field {
                Field::PredictedMoistureRate => &v.predicted_rate,
                Field::PredictedHumidifyingSetpointRate => &v.predicted_humid_sp_rate,
                Field::PredictedDehumidifyingSetpointRate => &v.predicted_dehumid_sp_rate,
                Field::SystemPredictedMoistureRate => &v.base.total_output_required,
                Field::SystemPredictedHumidifyingSetpointRate => {
                    &v.output_required_to_humidifying_sp
                }
                Field::SystemPredictedDehumidifyingSetpointRate => {
                    &v.output_required_to_dehumidifying_sp
                }
                _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
            },
            Self::Thermostat(v) => {
                let i = index(number)?;
                match field {
                    Field::ThermostatAirTemperature => v
                        .temp_tstat_air
                        .as_deref()
                        .and_then(|r| r.get(i))
                        .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?,
                    Field::LoadCorrectionFactor => v
                        .thermostat
                        .load_correction_factor
                        .as_deref()
                        .and_then(|r| r.get(i))
                        .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?,
                    Field::ThermostatHeatingSetpoint
                    | Field::ThermostatCoolingSetpoint
                    | Field::AdaptiveComfortCoolingSetpoint => {
                        let row = v
                            .thermostat
                            .zone_tstat_setpoints
                            .as_deref()
                            .and_then(|r| r.get(i))
                            .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?;
                        match field {
                            Field::ThermostatHeatingSetpoint => &row.setpt_lo,
                            Field::ThermostatCoolingSetpoint => &row.setpt_hi,
                            Field::AdaptiveComfortCoolingSetpoint => &row.setpt_adap_comfort_cool,
                            _ => unreachable!(),
                        }
                    }
                    _ => return Err(OrdinaryRadiationUnavailable::StorageDomain.into()),
                }
            }
        })
    }
}
impl SourceRealOutputOwners for Row<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::Air {
            field,
            index: number,
            space,
        } = target
        else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        if number != self.number || space != self.space {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        }
        self.record.real(number, field)
    }
}
impl SourceIntegerOutputOwners for Row<'_> {
    fn integer_output(
        &self,
        target: SourceIntegerOutputTarget,
    ) -> Result<&i32, OrdinaryRadiationError> {
        let SourceIntegerOutputTarget::ZoneThermostatControlType { index: number } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        if number != self.number || self.space {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        }
        let Record::Thermostat(fan) = self.record else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        fan.thermostat
            .temp_control_type_report
            .as_deref()
            .and_then(|r| r.get(index(number).ok()?))
            .ok_or_else(|| OrdinaryRadiationUnavailable::StorageDomain.into())
    }
}

/// Final report consumption borrows all genuine persistent Air state groups.
pub(crate) struct SourceAirOutputOwners<'a> {
    pub(crate) air: &'a SourceAirHeatBalance,
    pub(crate) demand: &'a SourceZoneAirDemand,
    pub(crate) fan: &'a SourceZoneFanHeatBalanceArrays,
    pub(crate) zone_lists: &'a SourceZoneListGroupLoadReports,
    pub(crate) report: &'a dyn SourceRealOutputOwners,
    pub(crate) resilience: &'a dyn SourceRealOutputOwners,
}
impl SourceRealOutputOwners for SourceAirOutputOwners<'_> {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::Air {
            field,
            index: number,
            space,
        } = target
        else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        let record = match field {
            Field::ZoneListSensibleHeatingEnergy
            | Field::ZoneListSensibleCoolingEnergy
            | Field::ZoneListSensibleHeatingRate
            | Field::ZoneListSensibleCoolingRate
            | Field::ZoneGroupSensibleHeatingEnergy
            | Field::ZoneGroupSensibleCoolingEnergy
            | Field::ZoneGroupSensibleHeatingRate
            | Field::ZoneGroupSensibleCoolingRate => {
                if space {
                    return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
                }
                Record::ZoneListGroup(self.zone_lists)
            }

            Field::AirTemperature
            | Field::AirHumidityRatio
            | Field::AirRelativeHumidity
            | Field::MeanRadiantTemperature => {
                let i = index(number)?;
                let row = if space {
                    &self
                        .air
                        .spaces
                        .as_deref()
                        .and_then(|r| r.get(i))
                        .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
                        .air
                } else {
                    &self
                        .air
                        .zones
                        .as_deref()
                        .and_then(|r| r.get(i))
                        .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?
                        .air
                };
                Record::Air(row)
            }
            Field::SensibleHeatingEnergy
            | Field::SensibleCoolingEnergy
            | Field::SensibleHeatingRate
            | Field::SensibleCoolingRate
            | Field::PredictedSensibleRate
            | Field::PredictedHeatingSetpointRate
            | Field::PredictedCoolingSetpointRate
            | Field::SystemPredictedSensibleRate
            | Field::SystemPredictedHeatingSetpointRate
            | Field::SystemPredictedCoolingSetpointRate => {
                let rows = if space {
                    &self.demand.spaces
                } else {
                    &self.demand.zones
                };
                Record::Sensible(
                    rows.as_deref()
                        .and_then(|r| r.get(index(number).ok()?))
                        .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?,
                )
            }
            Field::PredictedMoistureRate
            | Field::PredictedHumidifyingSetpointRate
            | Field::PredictedDehumidifyingSetpointRate
            | Field::SystemPredictedMoistureRate
            | Field::SystemPredictedHumidifyingSetpointRate
            | Field::SystemPredictedDehumidifyingSetpointRate => {
                let rows = if space {
                    &self.demand.space_moisture
                } else {
                    &self.demand.zone_moisture
                };
                Record::Moisture(
                    rows.as_deref()
                        .and_then(|r| r.get(index(number).ok()?))
                        .ok_or(OrdinaryRadiationUnavailable::StorageDomain)?,
                )
            }
            Field::ThermostatAirTemperature
            | Field::ThermostatHeatingSetpoint
            | Field::ThermostatCoolingSetpoint
            | Field::AdaptiveComfortCoolingSetpoint
            | Field::LoadCorrectionFactor => {
                if space {
                    return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
                }
                Record::Thermostat(self.fan)
            }
            Field::ZoneHeatIndex | Field::ZoneHumidex => {
                return self.resilience.real_output(target);
            }
            _ => return self.report.real_output(target),
        };
        record.real(number, field)
    }
}
impl SourceIntegerOutputOwners for SourceAirOutputOwners<'_> {
    fn integer_output(
        &self,
        target: SourceIntegerOutputTarget,
    ) -> Result<&i32, OrdinaryRadiationError> {
        let SourceIntegerOutputTarget::ZoneThermostatControlType { index: number } = target else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        self.fan
            .thermostat
            .temp_control_type_report
            .as_deref()
            .and_then(|r| r.get(index(number).ok()?))
            .ok_or_else(|| OrdinaryRadiationUnavailable::StorageDomain.into())
    }
}
