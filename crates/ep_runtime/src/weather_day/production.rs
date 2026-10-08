//! Fallible production weather access without borrowed cursor guards.

use super::{WeatherDayError, WeatherEnvironmentConfiguration, WeatherSession, WeatherVars};
use crate::weather::{EpwRecord, WeatherTimestepSample};
use ep_model::FirstHourInterpolationStartingValues;
use std::cell::RefCell;

/// Explicit caller phase; repeated warmup days retain independent call identities.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeatherDayPhase {
    /// A thermal warmup day using the first source weather day.
    Warmup {
        /// One-based warmup iteration.
        day: u32,
    },
    /// A reported run-period day, one-based within the environment.
    Run {
        /// One-based simulation day within the environment.
        day: usize,
    },
}

/// Existing shadowing-period coefficients, separate from actual daily coefficients.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProductionSolarMetadata {
    /// Sine of shadowing-period solar declination.
    pub sin_declination: f64,
    /// Cosine of shadowing-period solar declination.
    pub cos_declination: f64,
    /// Shadowing-period equation of time in hours.
    pub equation_of_time_hours: f64,
}

/// Owned operands from the actual Today owner at one thermal timestep.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ProductionWeatherContext {
    /// Actual raw-hour projection, separate from civil calendar identity.
    pub record: EpwRecord,
    /// Compatibility psychrometric and scalar sample from the current day.
    pub sample: WeatherTimestepSample,
    /// Complete processed Today slot, directly used by sky/rain/solar consumers.
    pub weather: WeatherVars,
    /// Existing calendar-derived shadowing metadata, not weather-preview values.
    pub solar: ProductionSolarMetadata,
    /// Actual raw-hour ending clock at the current zone timestep.
    pub local_hour: f64,
}

/// Production weather lifecycle shared by A and B consumers.
///
/// Its interior owner is borrowed only while advancing or copying operands.
/// Thermal callbacks receive owned values after the borrow has been released.
#[derive(Debug)]
pub struct ProductionWeatherTimestepSeries {
    session: RefCell<WeatherSession>,
    prepared_phase: RefCell<Option<WeatherDayPhase>>,
    current_phase: RefCell<Option<WeatherDayPhase>>,
    last_consumed: RefCell<Option<(usize, u32)>>,
}

impl ProductionWeatherTimestepSeries {
    /// Opens actual input bytes; no eager physical series is accepted.
    pub fn from_bytes(
        bytes: Vec<u8>,
        configuration: WeatherEnvironmentConfiguration,
    ) -> Result<Self, WeatherDayError> {
        Ok(Self {
            session: RefCell::new(WeatherSession::new(bytes, configuration)?),
            prepared_phase: RefCell::new(None),
            current_phase: RefCell::new(None),
            last_consumed: RefCell::new(None),
        })
    }

    /// Prepares actual Today before humidity and CTF seeds. The first day hook
    /// consumes the phase token without another handoff or another prefetch.
    pub fn prepare_initial_phase(&self, phase: WeatherDayPhase) -> Result<(), WeatherDayError> {
        if self.current_phase.borrow().is_some() {
            return Err(WeatherDayError::admission(
                "initial weather phase already prepared",
            ));
        }
        self.admit_phase(phase)?;
        let mut session = self.session.borrow_mut();
        session.set_phase(phase, true);
        session.get_next_environment()?;
        session.initialize_weather()?;
        *self.prepared_phase.borrow_mut() = Some(phase);
        *self.current_phase.borrow_mut() = Some(phase);
        Ok(())
    }

    /// Performs one actual source BeginDay transport and its optional prefetch.
    pub fn begin_day(&self, phase: WeatherDayPhase) -> Result<(), WeatherDayError> {
        self.admit_phase(phase)?;
        let prepared = *self.prepared_phase.borrow();
        if let Some(prepared) = prepared {
            if prepared != phase {
                return Err(WeatherDayError::admission(
                    "first weather hook differs from prepared phase",
                ));
            }
            *self.prepared_phase.borrow_mut() = None;
            return Ok(());
        }
        if self.current_phase.borrow().is_none() {
            return Err(WeatherDayError::admission(
                "weather must be prepared before thermal seeds",
            ));
        }
        let mut session = self.session.borrow_mut();
        // SimulationManager.cc:549 completes the previous hour before the next
        // day resets HourOfDay. The weather handoff itself only writes this
        // clock on BeginEnvrn.
        if self
            .last_consumed
            .borrow()
            .is_some_and(|(index, timestep)| {
                index % 24 == 23 && timestep == session.configuration.steps()
            })
        {
            session.state.global.previous_hour = session.state.global.hour_of_day;
        }
        session.set_phase(phase, false);
        session.initialize_weather()?;
        *self.current_phase.borrow_mut() = Some(phase);
        Ok(())
    }

    fn admit_phase(&self, phase: WeatherDayPhase) -> Result<(), WeatherDayError> {
        let valid = match phase {
            WeatherDayPhase::Warmup { day } => day > 0,
            WeatherDayPhase::Run { day } => {
                day > 0 && day <= self.session.borrow().configuration.total_days()
            }
        };
        if valid {
            Ok(())
        } else {
            Err(WeatherDayError::admission(
                "weather phase outside environment",
            ))
        }
    }

    /// Returns owned actual current-day operands for an explicit thermal call.
    #[track_caller]
    pub fn current_for(
        &self,
        record_index: usize,
        timestep: u32,
    ) -> Result<ProductionWeatherContext, WeatherDayError> {
        let phase = self
            .current_phase
            .borrow()
            .ok_or_else(|| WeatherDayError::admission("Today is not prepared"))?;
        let mut session = self.session.borrow_mut();
        let steps = session.configuration.steps();
        if !(1..=steps).contains(&timestep) {
            return Err(WeatherDayError::admission("weather timestep outside hour"));
        }
        let day = match phase {
            WeatherDayPhase::Warmup { .. } => 1,
            WeatherDayPhase::Run { day } => day,
        };
        if record_index / 24 + 1 != day {
            return Err(WeatherDayError::admission(
                "weather request does not belong to actual current day",
            ));
        }
        let hour_zero = record_index % 24;
        let produced = session
            .today_produced
            .as_ref()
            .ok_or_else(|| WeatherDayError::admission("Today physical owner unavailable"))?;
        let record = produced.records[hour_zero];
        let mut sample = produced.samples[hour_zero * steps as usize + timestep as usize - 1];
        sample.record_index = record_index;
        let weather = session.state.today_values.hour(hour_zero + 1)?[timestep as usize - 1];
        let period_start = ((day - 1) / 20) * 20;
        let period_days = 20
            .min(session.configuration.total_days() - period_start)
            .max(1);
        let period_point = session
            .configuration
            .day_point(period_start + 1)
            .ok_or_else(|| WeatherDayError::admission("missing shadowing calendar metadata"))?;
        // Preserve existing solar wrapper's non-leap metadata policy. The actual
        // solar position below is computed separately from the actual raw date.
        let ordinal = super::configuration::ordinal(
            period_point.month as i32,
            period_point.day_of_month as i32,
            0,
        )
        .map_err(WeatherDayError::admission)?;
        let (sin_declination, cos_declination, equation_of_time_hours) =
            crate::heat_balance::solar::energyplus_average_solar_coefficients(
                ordinal as u32,
                period_days,
            );
        let context = ProductionWeatherContext {
            record,
            sample,
            weather,
            solar: ProductionSolarMetadata {
                sin_declination,
                cos_declination,
                equation_of_time_hours,
            },
            local_hour: f64::from(record.hour.saturating_sub(1))
                + f64::from(timestep) / f64::from(steps),
        };
        let hour = hour_zero as i32 + 1;
        if session.state.global.hour_of_day != hour {
            session.state.global.previous_hour = session.state.global.hour_of_day;
        }
        session.state.global.hour_of_day = hour;
        session.state.global.time_step = timestep as i32;
        session.state.global.begin_day_flag = hour_zero == 0 && timestep == 1;
        session.state.global.begin_envrn_flag &= hour_zero == 0 && timestep == 1;
        session.state.global.begin_sim_flag &= hour_zero == 0 && timestep == 1;
        session.state.global.begin_hour_flag = timestep == 1;
        session.state.global.begin_time_step_flag = true;
        session.state.global.end_hour_flag = timestep == steps;
        session.state.global.end_day_flag = hour_zero == 23 && timestep == steps;
        session.state.global.end_envrn_flag = matches!(phase, WeatherDayPhase::Run { .. })
            && record_index + 1 == session.configuration.time_axis.points.len()
            && timestep == steps;
        // InitializeWeather runs before current weather reaches the thermal
        // consumer. BeginDay was already performed by the shared day hook.
        if !session.state.global.begin_day_flag {
            session.initialize_weather()?;
        }
        super::production_trace::record_consumer(record_index, timestep, phase, context, &session);
        *self.last_consumed.borrow_mut() = Some((record_index, timestep));
        Ok(context)
    }

    /// Actual Today hour-one dry bulb for initial CTF boundary histories.
    pub fn initial_hourly_dry_bulb_c(&self) -> Result<f64, WeatherDayError> {
        let session = self.session.borrow();
        session
            .today_produced
            .as_ref()
            .map(|day| day.records[0].dry_bulb_c)
            .ok_or_else(|| WeatherDayError::admission("initial Today record is unavailable"))
    }

    /// Completes an environment if its final timestep did not already rewind it.
    pub fn finish_environment(&self) -> Result<(), WeatherDayError> {
        let mut session = self.session.borrow_mut();
        let last = (
            session.configuration.time_axis.points.len() - 1,
            session.configuration.steps(),
        );
        if *self.last_consumed.borrow() != Some(last) {
            return Ok(());
        }
        if session.state.global.end_envrn_flag {
            return Ok(());
        }
        session.state.global.begin_envrn_flag = false;
        session.state.global.begin_sim_flag = false;
        session.state.global.begin_day_flag = false;
        session.state.global.end_day_flag = true;
        session.state.global.end_envrn_flag = true;
        session.initialize_weather()
    }

    /// Number of metadata-axis hours; no placeholder weather values are returned.
    #[must_use]
    pub fn hourly_count(&self) -> usize {
        self.session.borrow().configuration.time_axis.points.len()
    }
    /// Zone timesteps in each hour.
    #[must_use]
    pub fn zone_steps_per_hour(&self) -> u32 {
        self.session.borrow().configuration.steps()
    }
    /// RunPeriod's declared first-hour policy.
    #[must_use]
    pub fn first_hour_interpolation_starting_values(&self) -> FirstHourInterpolationStartingValues {
        self.session.borrow().configuration.first_policy()
    }
    /// Read-only owned snapshot for independently collected evidence.
    #[must_use]
    pub fn snapshot(&self) -> super::production_trace::WeatherSessionSnapshot {
        super::production_trace::WeatherSessionSnapshot::from_session(&self.session.borrow())
    }
}
