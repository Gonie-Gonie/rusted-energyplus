//! Genuine DetectOscillatingZoneTemp5715-5861 on the same retained ZTPC owner.
use super::*;
use crate::heat_balance::ordinary_thermal_comfort::SourceThermalComfort;
use crate::heat_balance::radiation::ordinary::{
    OrdinaryRadiationError, OrdinaryRadiationUnavailable,
};
use crate::heat_balance::source_output_registration::{
    SourceOscillationOutputField, SourceRealOutputOwners, SourceRealOutputTarget,
};
mod output;
pub(crate) use output::SourceOscillationOutputContext;
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceZoneTemperatureOscillation {
    pub(crate) history: Option<Vec<[Option<f64>; 4]>>,
    pub(crate) zone_time: Option<Vec<f64>>,
    pub(crate) zone_occupied_time: Option<Vec<f64>>,
    pub(crate) zone_deadband_time: Option<Vec<f64>>,
    pub(crate) any_time: f64,
    pub(crate) any_occupied_time: f64,
    pub(crate) any_deadband_time: f64,
    pub(crate) annual_any_time: f64,
    pub(crate) annual_any_occupied_time: f64,
    pub(crate) annual_any_deadband_time: f64,
    pub(crate) variables_needed: bool,
    pub(crate) setup_output: bool,
}
impl Default for SourceZoneTemperatureOscillation {
    fn default() -> Self {
        Self {
            history: None,
            zone_time: None,
            zone_occupied_time: None,
            zone_deadband_time: None,
            any_time: 0.0,
            any_occupied_time: 0.0,
            any_deadband_time: 0.0,
            annual_any_time: 0.0,
            annual_any_occupied_time: 0.0,
            annual_any_deadband_time: 0.0,
            variables_needed: false,
            setup_output: true,
        }
    }
}
pub(crate) struct SourceOscillationOwners<'a, 'backend> {
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) zones: &'a SelectedZoneInputOwners,
    pub(crate) air: &'a SourceAirHeatBalance,
    pub(crate) current_deadband_or_setback: &'a Option<Vec<bool>>,
    pub(crate) comfort: &'a SourceThermalComfort,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) input: &'a mut SelectedInputProcessor,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
    pub(crate) eio: &'a mut dyn std::io::Write,
}
fn array_ref(values: &Option<Vec<f64>>, at: usize) -> Result<&f64, Error> {
    values
        .as_deref()
        .and_then(|v| v.get(at))
        .ok_or(Error::Unavailable(
            "actual oscillation allocated output cell",
        ))
}
fn array_mut(values: &mut Option<Vec<f64>>, at: usize) -> Result<&mut f64, Error> {
    values
        .as_deref_mut()
        .and_then(|v| v.get_mut(at))
        .ok_or(Error::Unavailable(
            "actual oscillation allocated output cell",
        ))
}
impl SourceRealOutputOwners for SourceZoneTemperatureOscillation {
    fn real_output(&self, target: SourceRealOutputTarget) -> Result<&f64, OrdinaryRadiationError> {
        let SourceRealOutputTarget::Oscillation {
            field,
            index: number,
        } = target
        else {
            return Err(OrdinaryRadiationUnavailable::StorageDomain.into());
        };
        use SourceOscillationOutputField::*;
        let result = match field {
            FacilityTime if number == 0 => Ok(&self.any_time),
            FacilityOccupiedTime if number == 0 => Ok(&self.any_occupied_time),
            FacilityDeadbandTime if number == 0 => Ok(&self.any_deadband_time),
            ZoneTime => index(number).and_then(|at| array_ref(&self.zone_time, at)),
            ZoneOccupiedTime => {
                index(number).and_then(|at| array_ref(&self.zone_occupied_time, at))
            }
            ZoneDeadbandTime => {
                index(number).and_then(|at| array_ref(&self.zone_deadband_time, at))
            }
            _ => Err(Error::Unavailable(
                "actual Facility oscillation scalar identity",
            )),
        };
        result.map_err(|error| OrdinaryRadiationError::Input(Box::new(error)))
    }
}
impl SourceZoneTemperatureOscillation {
    pub(crate) fn detect_source_zone_temperature<H, D>(
        &mut self,
        o: &mut SourceOscillationOwners<'_, '_>,
        outputs: &mut SourceOscillationOutputContext<'_, H, D>,
    ) -> Result<(), Error>
    where
        H: crate::heat_balance::source_output_registration::SourceOutputFileHost,
        D: crate::heat_balance::source_output_registration::SourceOutputDictionaryHost,
    {
        if self.setup_output {
            let n = usize::try_from(o.globals.num_of_zones)
                .map_err(|_| Error::Unavailable("actual oscillation NumOfZones dimension5734"))?;
            // Native Array2D(4,NumOfZones), first dimension varies fastest; allocation precedes scalar fill.
            self.history = Some(vec![[None; 4]; n]);
            if let Some(rows) = self.history.as_mut() {
                for row in rows {
                    row.fill(Some(0.0));
                }
            }
            self.zone_time = Some(vec![0.0; n]);
            self.zone_occupied_time = Some(vec![0.0; n]);
            self.zone_deadband_time = Some(vec![0.0; n]);
            for number in 1..=o.globals.num_of_zones {
                let zone = o
                    .zones
                    .zones
                    .as_deref()
                    .and_then(|rows| rows.get(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual oscillation Zone.Name5741"))?;
                outputs.declare(
                    self,
                    SourceOscillationOutputField::ZoneTime,
                    number,
                    "Zone Oscillating Temperatures Time",
                    &zone.name,
                    o.clock,
                    o.input,
                    o.backend,
                    o.eio,
                )?;
                outputs.declare(
                    self,
                    SourceOscillationOutputField::ZoneOccupiedTime,
                    number,
                    "Zone Oscillating Temperatures During Occupancy Time",
                    &zone.name,
                    o.clock,
                    o.input,
                    o.backend,
                    o.eio,
                )?;
                outputs.declare(
                    self,
                    SourceOscillationOutputField::ZoneDeadbandTime,
                    number,
                    "Zone Oscillating Temperatures in Deadband Time",
                    &zone.name,
                    o.clock,
                    o.input,
                    o.backend,
                    o.eio,
                )?;
            }
            outputs.declare(
                self,
                SourceOscillationOutputField::FacilityTime,
                0,
                "Facility Any Zone Oscillating Temperatures Time",
                "Facility",
                o.clock,
                o.input,
                o.backend,
                o.eio,
            )?;
            outputs.declare(
                self,
                SourceOscillationOutputField::FacilityOccupiedTime,
                0,
                "Facility Any Zone Oscillating Temperatures During Occupancy Time",
                "Facility",
                o.clock,
                o.input,
                o.backend,
                o.eio,
            )?;
            outputs.declare(
                self,
                SourceOscillationOutputField::FacilityDeadbandTime,
                0,
                "Facility Any Zone Oscillating Temperatures in Deadband Time",
                "Facility",
                o.clock,
                o.input,
                o.backend,
                o.eio,
            )?;
            if outputs
                .registration
                .reporting_source_variable("Zone Oscillating Temperatures Time")
                .map_err(Error::Output)?
                || outputs
                    .registration
                    .reporting_source_variable(
                        "Zone Oscillating Temperatures During Occupancy Time",
                    )
                    .map_err(Error::Output)?
                || outputs
                    .registration
                    .reporting_source_variable("Zone Oscillating Temperatures in Deadband Time")
                    .map_err(Error::Output)?
                || outputs
                    .registration
                    .reporting_source_variable("Facility Any Zone Oscillating Temperatures Time")
                    .map_err(Error::Output)?
                || outputs
                    .registration
                    .reporting_source_variable(
                        "Facility Any Zone Oscillating Temperatures During Occupancy Time",
                    )
                    .map_err(Error::Output)?
                || outputs
                    .registration
                    .reporting_source_variable(
                        "Facility Any Zone Oscillating Temperatures in Deadband Time",
                    )
                    .map_err(Error::Output)?
            {
                self.variables_needed = true;
            }
            self.setup_output = false;
        }
        let time_step_sys = o.hvac.time_step_sys;
        if self.variables_needed {
            let negative_magnitude = -0.15;
            let mut any = false;
            let mut any_occupied = false;
            let mut any_deadband = false;
            for number in 1..=o.globals.num_of_zones {
                let at = index(number)?;
                let mut oscillating = false;
                let history = self
                    .history
                    .as_mut()
                    .and_then(|rows| rows.get_mut(at))
                    .ok_or(Error::Unavailable("actual ZoneTempHist5809"))?;
                history[3] = history[2];
                history[2] = history[1];
                history[1] = history[0];
                history[0] = Some(
                    o.air
                        .zones
                        .as_deref()
                        .and_then(|rows| rows.get(at))
                        .ok_or(Error::Unavailable(
                            "actual oscillation ZoneHeatBalance.ZT5812",
                        ))?
                        .air
                        .zt,
                );
                let h = |i: usize| {
                    history[i].ok_or(Error::Unavailable("actual written ZoneTempHist cell"))
                };
                let diff34 = h(2)? - h(3)?;
                let diff23 = h(1)? - h(2)?;
                let diff12 = h(0)? - h(1)?;
                if diff12 > 0.15 && diff23 < negative_magnitude && diff34 > 0.15 {
                    oscillating = true;
                }
                if diff12 < negative_magnitude && diff23 > 0.15 && diff34 < negative_magnitude {
                    oscillating = true;
                }
                *array_mut(&mut self.zone_occupied_time, at)? = 0.0;
                *array_mut(&mut self.zone_deadband_time, at)? = 0.0;
                if oscillating {
                    *array_mut(&mut self.zone_time, at)? = time_step_sys;
                    any = true;
                    if let Some(rows) = o.comfort.thermal_comfort_in_ash55.as_deref() {
                        if rows
                            .get(at)
                            .ok_or(Error::Unavailable(
                                "actual ThermalComfortInASH55 ZoneIsOccupied5838",
                            ))?
                            .zone_is_occupied
                        {
                            *array_mut(&mut self.zone_occupied_time, at)? = time_step_sys;
                            any_occupied = true;
                        }
                    }
                    if *o
                        .current_deadband_or_setback
                        .as_deref()
                        .and_then(|rows| rows.get(at))
                        .ok_or(Error::Unavailable("actual CurDeadBandOrSetback5843"))?
                    {
                        *array_mut(&mut self.zone_deadband_time, at)? = time_step_sys;
                        any_deadband = true;
                    }
                } else {
                    *array_mut(&mut self.zone_time, at)? = 0.0;
                }
            }
            self.any_time = if any { time_step_sys } else { 0.0 };
            self.any_occupied_time = if any_occupied { time_step_sys } else { 0.0 };
            self.any_deadband_time = if any_deadband { time_step_sys } else { 0.0 };
            self.annual_any_time += self.any_time;
            self.annual_any_occupied_time += self.any_occupied_time;
            self.annual_any_deadband_time += self.any_deadband_time;
        }
        Ok(())
    }
}
