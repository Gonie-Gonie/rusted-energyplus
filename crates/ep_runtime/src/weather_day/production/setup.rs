//! Weather owners used by the real selected SetupSimulation traversal.

use super::ProductionWeatherTimestepSeries;
use crate::RuntimeError;
use crate::heat_balance::ctf_initialization_context::{
    CtfInitializationError, CtfInitializationReports,
};
use crate::weather::day::{
    DailyWeatherVariables, WeatherDayError, WeatherEnvironmentState, WeatherGlobalState,
    WeatherVars,
};
use chrono::{Datelike, Local};

/// The six actual SimulationControl input switches, independent of kickoff.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SetupSimulationControls {
    zone_sizing: bool,
    system_sizing: bool,
    plant_sizing: bool,
    hvac_sizing_simulation: bool,
    run_sizing_periods: bool,
    run_weather_periods: bool,
}

impl SetupSimulationControls {
    /// Retains the already-admitted raw flags in their real input-field order.
    #[must_use]
    pub fn from_admitted_input_flags(
        zone_sizing: bool,
        system_sizing: bool,
        plant_sizing: bool,
        hvac_sizing_simulation: bool,
        run_sizing_periods: bool,
        run_weather_periods: bool,
    ) -> Self {
        Self {
            zone_sizing,
            system_sizing,
            plant_sizing,
            hvac_sizing_simulation,
            run_sizing_periods,
            run_weather_periods,
        }
    }

    /// Actual DoZoneSizing input.
    #[must_use]
    pub fn zone_sizing(self) -> bool {
        self.zone_sizing
    }
    /// Actual DoSystemSizing input.
    #[must_use]
    pub fn system_sizing(self) -> bool {
        self.system_sizing
    }
    /// Actual DoPlantSizing input.
    #[must_use]
    pub fn plant_sizing(self) -> bool {
        self.plant_sizing
    }
    /// Actual DoHVACSizingSimulation input.
    #[must_use]
    pub fn hvac_sizing_simulation(self) -> bool {
        self.hvac_sizing_simulation
    }
    /// Actual DoDesDaySim input; false does not skip DDs during SetupSimulation.
    #[must_use]
    pub fn run_sizing_periods(self) -> bool {
        self.run_sizing_periods
    }
    /// Actual DoWeathSim input.
    #[must_use]
    pub fn run_weather_periods(self) -> bool {
        self.run_weather_periods
    }
}

/// Actual call site within one SetupSimulation environment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupSimulationRole {
    /// SimulationManager.cc:2015-2019, first hour 1 / timestep 1.
    FirstHour,
    /// SimulationManager.cc:2031-2035, a second real call at the same clock.
    RepeatedFirstHour,
    /// SimulationManager.cc:2046-2050, hour 24 / final timestep.
    LastHour,
}

/// Owned call identity and actual weather operands after ManageWeather.
///
/// This is deliberately separate from the ordinary CTF history clock. A repeated
/// first-hour call retains the same SimTimeSteps and a distinct actual ordinal.
#[derive(Clone, Debug, PartialEq)]
pub struct SetupSimulationCall {
    /// One-based real ManageWeather/Exterior/HeatBalance call ordinal.
    pub ordinal: u64,
    /// Exact selected source call site, not a fabricated simulation phase.
    pub role: SetupSimulationRole,
    /// Actual WeatherManager Envrn, including preceding DesignDay environments.
    pub environment_number: i32,
    /// Actual selected environment name.
    pub environment_name: String,
    /// Real SetupSimulation caller flag at this boundary.
    pub kickoff_simulation: bool,
    /// Retained actual KickOffSizing flag, false in the admitted non-sizing path.
    pub kickoff_sizing: bool,
    /// Real reporting authority from SimulationManager:204, restored only at281.
    pub do_weather_init_reporting: bool,
    /// Full actual caller flags and physical clock after SetCurrentWeather.
    pub global: WeatherGlobalState,
    /// Actual UpdateWeatherData daily owner; no RunPeriod date projection.
    pub daily: DailyWeatherVariables,
    /// Exact selected Today slot, including source beam/diffuse before masking.
    pub today: WeatherVars,
}

impl ProductionWeatherTimestepSeries {
    /// Begin the selected weather subsystem before ordinary weather access.
    /// This does not run Exterior, HeatBalance or the Setup postludes.
    ///
    /// # Errors
    /// Reentry, active sizing, or missing real DesignDay/control owners are unavailable.
    pub fn begin_setup_simulation(&self) -> Result<(), WeatherDayError> {
        if self.setup_started.get() || self.current_phase.borrow().is_some() {
            return Err(WeatherDayError::admission(
                "SetupSimulation must precede the initial ordinary weather phase",
            ));
        }
        let mut session = self.session.borrow_mut();
        let controls = session
            .configuration
            .setup_simulation_controls
            .ok_or_else(|| {
                WeatherDayError::admission(
                    "SetupSimulation requires actual raw SimulationControl flags",
                )
            })?;
        if controls.zone_sizing()
            || controls.system_sizing()
            || controls.plant_sizing()
            || controls.hvac_sizing_simulation()
        {
            return Err(WeatherDayError::admission(
                "selected SetupSimulation excludes active sizing producers",
            ));
        }
        if session.configuration.design_days.len()
            != session.configuration.design_day_count as usize
        {
            return Err(WeatherDayError::admission(
                "actual DesignDay inputs do not match the environment count",
            ));
        }
        // Real RunSimulation initialization (201), before progress output267.
        session.state.global.begin_sim_flag = true;
        session.state.global.do_des_day_sim = controls.run_sizing_periods();
        session.state.global.do_weath_sim = controls.run_weather_periods();
        self.setup_started.set(true);
        Ok(())
    }

    /// Apply the actual pre-traversal Envrn=0 assignment at SimulationManager270.
    /// The real caller invokes this once after its early registry/progress prefix.
    pub fn reset_setup_environment_counter(&self) {
        self.session.borrow_mut().state.weather.envrn = 0; // Original270/Weather159.
    }

    /// Advance the actual DesignDay/RunPeriod weather getter and Setup flags.
    /// Call after the sole early registry preparation; retain both getter booleans.
    ///
    /// # Errors
    /// Actual getter/report/owner failures retain their original source.
    pub fn next_setup_environment(
        &self,
        reports: &mut CtfInitializationReports,
    ) -> Result<(bool, bool), RuntimeError> {
        if !self.setup_started.get() || self.setup_completed.get() {
            return Err(WeatherDayError::admission(
                "SetupSimulation environment traversal is not active",
            )
            .into());
        }
        let mut session = self.session.borrow_mut();
        if !self.setup_design_day_reports_registered.get() {
            // WeatherManager712/4786/6639: first real input-getter suffix,
            // after early SetPredefinedTables215 and before close/increment774.
            session
                .configuration
                .register_design_day_predefined_reports(reports)
                .map_err(CtfInitializationError::Owner)?;
            self.setup_design_day_reports_registered.set(true);
        }
        session.cursor.close();
        session.state.weather.envrn = session
            .state
            .weather
            .envrn
            .checked_add(1)
            .ok_or_else(|| WeatherDayError::admission("environment counter overflow"))?;
        session.state.weather.dates_should_be_reset = false;
        let ordinal = session.state.weather.envrn;
        if ordinal > session.configuration.design_day_count + 1 {
            session.available = false;
            session.state.weather.envrn = 0;
            session.state.environment.cur_envir_num = 0;
            return Ok((false, session.errors_found));
        }
        if ordinal <= session.configuration.design_day_count {
            let input = session.configuration.design_days[(ordinal - 1) as usize].clone();
            input.select_environment(&mut session.state, ordinal)?;
            session.available = true;
        } else {
            // Same admitted real RunPeriod getter body used by the legacy route.
            session.admit_selected_run_period()?;
        }
        if session.errors_found {
            // The real getter returns Available=false with ErrorsFound retained.
            // Setup breaks and reaches its own genuine final fatal boundary.
            session.available = false;
            return Ok((false, true));
        }
        // Literal SetupSimulation1992-2007 order; EndDay is not set at hour24.
        session.state.global.begin_envrn_flag = true;
        session.state.global.end_envrn_flag = false;
        session.state.environment.end_month_flag = false;
        session.state.global.warmup_flag = true;
        session.state.global.day_of_sim = 0;
        session.state.global.day_of_sim += 1;
        session.state.global.begin_day_flag = true;
        session.state.global.end_day_flag = false;
        session.state.global.hour_of_day = 1;
        session.state.global.begin_hour_flag = true;
        session.state.global.end_hour_flag = false;
        session.state.global.time_step = 1;
        Ok((session.available, session.errors_found))
    }

    /// Borrow the selected actual environment name for caller progress output.
    pub fn setup_environment_name(&self) -> String {
        self.session
            .borrow()
            .state
            .environment
            .environment_name
            .clone()
    }

    /// Apply the real last-hour clock writes before its caller progress operation.
    pub fn prepare_setup_call(&self, role: SetupSimulationRole) {
        if role == SetupSimulationRole::LastHour {
            let mut session = self.session.borrow_mut();
            // Original2039-2041 precedes the developer DisplayString2043.
            session.state.global.hour_of_day = 24;
            session.state.global.time_step = session.state.global.time_steps_in_hour;
            session.state.global.end_envrn_flag = true;
        }
    }

    /// Run actual selected InitializeWeather and SetCurrentWeather once.
    /// The caller supplies its real ordinal/role and KickOff/reporting authority.
    /// The three roles remain distinct even when the first two physical clocks match.
    ///
    /// # Errors
    /// Missing active environment or an actual weather producer failure is returned.
    pub fn manage_setup_weather(
        &self,
        ordinal: u64,
        role: SetupSimulationRole,
        kickoff_simulation: bool,
        kickoff_sizing: bool,
        do_weather_init_reporting: bool,
    ) -> Result<SetupSimulationCall, WeatherDayError> {
        let mut session = self.session.borrow_mut();
        if !self.setup_started.get() || self.setup_completed.get() || !session.available {
            return Err(WeatherDayError::admission(
                "ManageWeather has no active Setup environment",
            ));
        }
        if role == SetupSimulationRole::FirstHour {
            session.state.global.begin_time_step_flag = true; // Original2013 only.
        }
        if session.state.global.kind_of_sim == 1 {
            let index = (session.state.weather.envrn - 1) as usize;
            // Original SetUpDesignDay calls date_and_time at each actual BeginEnv.
            let year = if session.state.global.begin_envrn_flag {
                Local::now().year()
            } else {
                session.state.today_variables.year
            };
            session.initialize_design_day_weather(index, year, do_weather_init_reporting)?;
        } else {
            session.initialize_weather()?;
        }
        session.set_current_weather()?;
        let hour = session.state.global.hour_of_day as usize;
        let step = session.state.global.time_step as usize;
        let today = *session
            .state
            .today_values
            .hour(hour)?
            .get(step - 1)
            .ok_or_else(|| WeatherDayError::admission("Setup timestep outside Today allocation"))?;
        Ok(SetupSimulationCall {
            ordinal,
            role,
            environment_number: session.state.weather.envrn,
            environment_name: session.state.environment.environment_name.clone(),
            kickoff_simulation,
            kickoff_sizing,
            do_weather_init_reporting,
            global: session.state.global.clone(),
            daily: session.state.today_variables,
            today,
        })
    }

    /// Borrow the sole live environment for a real downstream consumer.
    pub fn with_setup_environment_mut<T>(
        &self,
        consume: impl FnOnce(&mut WeatherEnvironmentState) -> T,
    ) -> T {
        consume(&mut self.session.borrow_mut().state.environment)
    }

    /// Apply Source2021-2024 only after the real first call consumers return.
    pub fn finish_first_setup_call(&self) {
        let mut session = self.session.borrow_mut();
        session.state.global.begin_hour_flag = false;
        session.state.global.begin_day_flag = false;
        session.state.global.begin_envrn_flag = false;
        session.state.global.begin_sim_flag = false;
    }

    /// Apply only the weather warmup/completion handoff after real caller suffixes.
    /// This method does not claim that Exterior/HB/cost/fault/curve operations ran.
    pub fn finish_setup_simulation(&self) {
        self.session.borrow_mut().state.global.warmup_flag = false;
        self.setup_completed.set(true);
    }
}
