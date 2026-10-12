//! Literal selected GetProjectData order; no independent count/schema authority.
use super::super::error_backend::SourceErrorBackend;
use super::{
    ProjectDataError, ProjectDataOwners, SelectedProjectData, SelectedProjectDataInput, fatal,
};
use std::io::Write;

fn absent(owners: &ProjectDataOwners<'_>, family: &'static str) -> Result<(), ProjectDataError> {
    if owners.input.get_num_objects_found(family) != 0 {
        return Err(ProjectDataError::Unavailable(family));
    }
    Ok(())
}
fn rounded(value: f64, precision: usize) -> Result<String, ProjectDataError> {
    super::super::source_round_format::round(value, precision).map_err(|e| {
        ProjectDataError::Format(format!("GetProjectData .{precision}R unavailable: {e:?}"))
    })
}
fn yes_no(value: bool) -> &'static str {
    if value { "Yes" } else { "No" }
}

impl SelectedProjectData {
    pub(crate) fn get_project_data(
        &mut self,
        raw: &SelectedProjectDataInput,
        owners: &mut ProjectDataOwners<'_>,
        backend: &mut SourceErrorBackend<'_>,
        eio: &mut dyn Write,
    ) -> Result<(), ProjectDataError> {
        let mut errors_found = false;
        let mut version_id = String::new();
        let count = owners.input.get_num_objects_found("Version");
        if count == 1 {
            let values = raw.get_object_item("Version", owners.input, owners.shortcut)?;
            // Generated DataStringGlobals MatchVersion is major.minor = 26.1.
            if !values.alphas[0].starts_with("26.1") {
                backend.show_warning_error(&format!(
                    "Version: in IDF=\"{}\" not the same as expected=\"26.1\"",
                    values.alphas[0]
                ))?;
            }
            version_id = values.alphas[0].clone();
        } else if count == 0 {
            backend.show_warning_error(
                "Version: missing in IDF, processing for EnergyPlus version=\"26.1\"",
            )?;
        } else {
            backend.show_severe_error("Too many Version Objects found.")?;
            errors_found = true;
        }

        let mut cond_fd = false;
        if owners.input.get_num_objects_found("HeatBalanceAlgorithm") > 0 {
            let values =
                raw.get_object_item("HeatBalanceAlgorithm", owners.input, owners.shortcut)?;
            cond_fd = [
                "CONDUCTIONFINITEDIFFERENCE",
                "CONDFD",
                "CONDUCTIONFINITEDIFFERENCEDETAILED",
                "CONDUCTIONFINITEDIFFERENCESIMPLIFIED",
            ]
            .contains(&values.alphas[0].as_str());
        }
        for family in [
            "SurfaceProperty:HeatTransferAlgorithm",
            "SurfaceProperty:HeatTransferAlgorithm:MultipleSurface",
            "SurfaceProperty:HeatTransferAlgorithm:SurfaceList",
            "SurfaceProperty:HeatTransferAlgorithm:Construction",
        ] {
            absent(owners, family)?;
        }

        let count = owners.input.get_num_objects_found("Timestep");
        if count == 1 {
            let values = raw.get_object_item("Timestep", owners.input, owners.shortcut)?;
            let number = values.numbers[0];
            if !number.is_finite() || number < f64::from(i32::MIN) || number > f64::from(i32::MAX) {
                return Err(ProjectDataError::Unavailable(
                    "Timestep signed int conversion",
                ));
            }
            let mut steps = number as i32;
            if self.system.ci_force_time_step {
                steps = 2;
            }
            // This assignment precedes the real warning backend, so failure
            // retains the requested/CI value just as the Original does.
            owners.weather.global.time_steps_in_hour = steps;
            if steps <= 0 || steps > 60 {
                backend.show_warning_error(&format!(
                    "Timestep: Requested number ({steps}) invalid, Defaulted to 4"
                ))?;
                steps = 4;
            } else if 60 % steps != 0 {
                let divisors: [i32; 12] = [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60];
                let (mut minimum, mut which) = (9999, 0);
                for (index, divisor) in divisors.iter().enumerate() {
                    if (steps - divisor).abs() > minimum {
                        continue;
                    }
                    // Original assigns the signed difference, not abs(difference).
                    minimum = steps - divisor;
                    which = index;
                }
                backend.show_warning_error(&format!("Timestep: Requested number ({steps}) not evenly divisible into 60, defaulted to nearest ({}).", divisors[which]))?;
                steps = divisors[which];
            }
            owners.weather.global.time_steps_in_hour = steps;
            if cond_fd && owners.weather.global.time_steps_in_hour < 20 {
                backend.show_warning_error(&format!("Timestep: Requested number ({}) cannot be used when Conduction Finite Difference algorithm is selected.", owners.weather.global.time_steps_in_hour))?;
                backend.show_continue_error("...Timestep is set to 20.")?;
                owners.weather.global.time_steps_in_hour = 20;
            }
            if owners.weather.global.time_steps_in_hour < 4
                && owners.input.get_num_objects_found("Zone") > 0
            {
                backend.show_warning_error(&format!(
                    "Timestep: Requested number ({}) is less than the suggested minimum of 4.",
                    owners.weather.global.time_steps_in_hour
                ))?;
                backend.show_continue_error("Please see entry for Timestep in Input/Output Reference for discussion of considerations.")?;
            }
        } else if count == 0 && owners.input.get_num_objects_found("Zone") > 0 && !cond_fd {
            backend.show_warning_error(
                "No Timestep object found.  Number of TimeSteps in Hour defaulted to 4.",
            )?;
            owners.weather.global.time_steps_in_hour = 4;
        } else if count == 0 && !cond_fd {
            owners.weather.global.time_steps_in_hour = 4;
        } else if count == 0 && owners.input.get_num_objects_found("Zone") > 0 && cond_fd {
            backend.show_warning_error(
                "No Timestep object found.  Number of TimeSteps in Hour defaulted to 20.",
            )?;
            backend.show_continue_error(
                "...Due to presence of Conduction Finite Difference Algorithm selection.",
            )?;
            owners.weather.global.time_steps_in_hour = 20;
        } else if count == 0 && cond_fd {
            owners.weather.global.time_steps_in_hour = 20;
        } else {
            backend.show_severe_error("Too many Timestep Objects found.")?;
            errors_found = true;
        }
        if owners.weather.global.time_steps_in_hour == 0 {
            return Err(ProjectDataError::Unavailable(
                "zero actual timestep before Original division",
            ));
        }
        owners.weather.global.time_step_zone =
            1.0 / f64::from(owners.weather.global.time_steps_in_hour);
        owners.weather.global.minutes_in_time_step =
            (owners.weather.global.time_step_zone * 60.0) as i32;
        owners.weather.global.time_step_zone_sec = owners.weather.global.time_step_zone * 3600.0;

        absent(owners, "ConvergenceLimits")?;
        self.convergence.min_time_step_sys = 1.0 / 60.0;
        self.convergence.max_iter = 20;
        self.convergence.min_plant_sub_iterations = 2;
        self.convergence.max_plant_sub_iterations = 8;
        owners.hvac.limit_num_sys_steps =
            (owners.weather.global.time_step_zone / self.convergence.min_time_step_sys) as i32;

        self.debug_output = false;
        self.even_during_warmup = false;
        absent(owners, "Output:DebuggingData")?;
        let count = owners.input.get_num_objects_found("Output:Diagnostics");
        if count > 1 {
            backend.show_warning_error("Output:Diagnostics: More than 1 occurrence of this object found, only first will be used.")?;
        }
        if count != raw.diagnostics.len() {
            return Err(ProjectDataError::Unavailable(
                "diagnostics/IP count association",
            ));
        }
        if let Some(object) = raw.diagnostics.first() {
            if !owners
                .input
                .has_object("Output:Diagnostics", &object.object_name)
                || raw
                    .diagnostics
                    .windows(2)
                    .any(|pair| pair[0].object_name >= pair[1].object_name)
            {
                return Err(ProjectDataError::Unavailable(
                    "diagnostics lexical/IP association",
                ));
            }
            owners
                .input
                .mark_object_as_used("Output:Diagnostics", &object.object_name);
            for key in &object.keys {
                let Some(key) = key else {
                    backend.show_warning_error("Output:Diagnostics: empty key found, consider removing it to avoid this warning.")?;
                    continue;
                };
                match key.to_ascii_uppercase().as_str() {
                    "DISPLAYEXTRAWARNINGS" => owners.globals.display_extra_warnings = true,
                    "DISPLAYADVANCEDREPORTVARIABLES" => owners.globals.display_advanced_report_variables = true,
                    "DISPLAYALLWARNINGS" => {
                        owners.globals.display_all_warnings = true;
                        owners.globals.display_extra_warnings = true;
                        owners.globals.display_unused_objects = true;
                        owners.globals.display_unused_schedules = true;
                    }
                    "DISPLAYUNUSEDOBJECTS" => owners.globals.display_unused_objects = true,
                    "DISPLAYUNUSEDSCHEDULES" => owners.globals.display_unused_schedules = true,
                    "DISPLAYZONEAIRHEATBALANCEOFFBALANCE" => owners.globals.display_zone_air_heat_balance_off_balance = true,
                    "DONOTMIRRORDETACHEDSHADING" => self.make_mirrored_detached_shading = false,
                    "DONOTMIRRORATTACHEDSHADING" => self.make_mirrored_attached_shading = false,
                    "REPORTDURINGWARMUP" => self.system.report_during_warmup = true,
                    "DISPLAYWEATHERMISSINGDATAWARNINGS" => owners.weather.environment.display_weather_missing_data_warnings = true,
                    "IGNORESOLARRADIATION" => owners.weather.environment.ignore_solar_radiation = true,
                    "IGNOREBEAMRADIATION" => owners.weather.environment.ignore_beam_radiation = true,
                    "IGNOREDIFFUSERADIATION" => owners.weather.environment.ignore_diffuse_radiation = true,
                    "DEVELOPERFLAG" => self.system.developer_flag = true,
                    "TIMINGFLAG" => self.system.timing_flag = true,
                    "REPORTDETAILEDWARMUPCONVERGENCE" => self.system.report_detailed_warmup_convergence = true,
                    "REPORTDURINGHVACSIZINGSIMULATION" => self.system.report_during_hvac_sizing_simulation = true,
                    "CREATEMINIMALSURFACEVARIABLES" | "CREATENORMALSURFACEVARIABLES" | "" => {}
                    _ => backend.show_warning_error(&format!("GetProjectData: Output:Diagnostics=\"{key}\", Invalid value for field, entered value ignored."))?,
                }
            }
        }
        absent(owners, "OutputControl:ReportingTolerances")?;
        *owners.heat_balance_current_module_object = "OutputControl:ResilienceSummaries".into();
        absent(owners, "OutputControl:ResilienceSummaries")?;

        owners.globals.do_zone_sizing = false;
        owners.globals.do_system_sizing = false;
        owners.globals.do_plant_sizing = false;
        owners.weather.global.do_des_day_sim = true;
        owners.weather.global.do_weath_sim = true;
        owners.globals.do_hvac_sizing_simulation = false;
        owners.globals.hvac_sizing_sim_max_iterations = 0;
        let mut simulation_values = None;
        if owners.input.get_num_objects_found("SimulationControl") > 0 {
            self.run_control_in_input = true;
            let values = raw.get_object_item("SimulationControl", owners.input, owners.shortcut)?;
            owners.globals.do_zone_sizing = values.alphas[0] == "YES";
            owners.globals.do_system_sizing = values.alphas[1] == "YES";
            owners.globals.do_plant_sizing = values.alphas[2] == "YES";
            if values.alphas[3] == "NO" {
                owners.weather.global.do_des_day_sim = false;
            }
            if values.alphas[4] == "NO" {
                owners.weather.global.do_weath_sim = false;
            }
            if values.num_alphas > 5 && values.alphas[5] == "YES" {
                owners.globals.do_hvac_sizing_simulation = true;
                if !owners.globals.do_plant_sizing {
                    owners.globals.do_hvac_sizing_simulation = false;
                    backend.show_warning_error("GetProjectData: Mismatch in the Sizing Flags Do HVAC Sizing and Do Plant Sizing in SimulationControl object")?;
                    for line in [
                        "...The Do HVAC Sizing Simulation for Sizing Periods flag is YES, but the Do Plant Sizing",
                        "...Calculation flag is NO.  This is not allowed.  Either set the Do HVAC Sizing Simulation",
                        "...for Sizing Periods flag to NO or set the Do Plant Sizing Calculation to YES and add the",
                        "...appropriate Sizing:Plant object(s) to the input file.  The simulation continues with",
                        "...the Do HVAC Sizing flag reset to NO.",
                    ] {
                        backend.show_continue_error(line)?;
                    }
                } else if owners.input.get_num_objects_found("Sizing:Plant") == 0 {
                    backend.show_severe_error("GetProjectData: No Sizing:Plant object entered when the Do HVAC Sizing Simulation and Do Plant Sizing are both YES in the SimulationControl object.")?;
                    backend.show_continue_error("...When these input flags are both yes, a Sizing:Plant object is required.")?;
                    backend.show_continue_error("...Either add one or more appropriate Sizing:Plant objects to the input file")?;
                    backend.show_continue_error("...or change both the Do HVAC Sizing Simulation and Do Plant Sizing are both YES. ")?;
                    errors_found = true;
                }
            }
            simulation_values = Some(values);
        }
        if self.system.dd_only {
            owners.weather.global.do_des_day_sim = true;
            owners.weather.global.do_weath_sim = false;
        }
        if self.system.full_annual_run {
            owners.weather.global.do_des_day_sim = false;
            owners.weather.global.do_weath_sim = true;
        }
        if !owners.weather.global.do_des_day_sim
            && !owners.weather.global.do_weath_sim
            && !owners.globals.do_hvac_sizing_simulation
            && (owners.globals.do_zone_sizing || owners.globals.do_system_sizing)
        {
            owners.globals.do_pure_load_calc = true;
        }

        let count = owners
            .input
            .get_num_objects_found("PerformancePrecisionTradeoffs");
        if count > 1 {
            return fatal(
                backend,
                "GetProjectData: Only one (\"1\") PerformancePrecisionTradeoffs object per simulation is allowed.",
            );
        }
        owners.globals.create_perf_log = count > 0;
        absent(owners, "PerformancePrecisionTradeoffs")?;
        if errors_found {
            return fatal(backend, "Errors found getting Project Input");
        }

        writeln!(eio, "! <Version>, Version ID")?;
        writeln!(eio, " Version, {version_id}")?;
        writeln!(
            eio,
            "! <Timesteps per Hour>, #TimeSteps, Minutes per TimeStep {{minutes}}"
        )?;
        writeln!(
            eio,
            " Timesteps per Hour, {:2}, {:2}",
            owners.weather.global.time_steps_in_hour, owners.weather.global.minutes_in_time_step
        )?;
        writeln!(
            eio,
            "! <System Convergence Limits>, Minimum System TimeStep {{minutes}}, Max HVAC Iterations, Minimum Plant Iterations, Maximum Plant Iterations"
        )?;
        writeln!(
            eio,
            " System Convergence Limits, {}, {}, {}, {}",
            (self.convergence.min_time_step_sys * 60.0) as i32,
            self.convergence.max_iter,
            self.convergence.min_plant_sub_iterations,
            self.convergence.max_plant_sub_iterations
        )?;
        if owners.globals.do_hvac_sizing_simulation {
            if let Some(values) = simulation_values
                .as_ref()
                .filter(|values| values.num_numbers >= 1)
            {
                owners.globals.hvac_sizing_sim_max_iterations = values.numbers[0] as i32;
            }
        }
        writeln!(
            eio,
            "! <Simulation Control>, Do Zone Sizing, Do System Sizing, Do Plant Sizing, Do Design Days, Do Weather Simulation, Do HVAC Sizing Simulation"
        )?;
        write!(eio, " Simulation Control")?;
        for flag in [
            owners.globals.do_zone_sizing,
            owners.globals.do_system_sizing,
            owners.globals.do_plant_sizing,
            owners.weather.global.do_des_day_sim,
            owners.weather.global.do_weath_sim,
            owners.globals.do_hvac_sizing_simulation,
        ] {
            write!(eio, ", {}", yes_no(flag))?;
        }
        writeln!(eio)?;
        if owners.globals.do_coil_direct_solutions {
            backend.show_warning_error(
                "PerformancePrecisionTradeoffs: Coil Direct Solution simulation is selected.",
            )?;
        }
        if self.carroll_method {
            backend.show_warning_error(
                "PerformancePrecisionTradeoffs: Carroll MRT radiant exchange method is selected.",
            )?;
        }
        let performance = [
            yes_no(owners.globals.do_coil_direct_solutions).to_string(),
            if self.carroll_method {
                "CarrollMRT"
            } else {
                "ScriptF"
            }
            .into(),
            "Normal".into(),
            owners.weather.global.time_steps_in_hour.to_string(),
            yes_no(self.override_zone_air_solution_algorithm).into(),
            self.min_number_of_warmup_days.to_string(),
            yes_no(owners.weather.environment.force_begin_env_reset_suppress).into(),
            rounded(self.convergence.min_time_step_sys * 60.0, 1)?,
            rounded(self.convergence.max_zone_temp_diff, 3)?,
            rounded(self.max_allowed_delta_temperature, 4)?,
        ];
        writeln!(
            eio,
            "! <Performance Precision Tradeoffs>, Use Coil Direct Simulation, Zone Radiant Exchange Algorithm, Override Mode, Number of Timestep In Hour, Force Euler Method, Minimum Number of Warmup Days, Force Suppress All Begin Environment Resets, Minimum System Timestep, MaxZoneTempDiff, MaxAllowedDelTemp"
        )?;
        write!(eio, " Performance Precision Tradeoffs")?;
        for value in &performance {
            write!(eio, ", {value}")?;
        }
        writeln!(eio)?;
        writeln!(
            eio,
            "! <Output Reporting Tolerances>, Tolerance for Time Heating Setpoint Not Met, Tolerance for Zone Cooling Setpoint Not Met Time"
        )?;
        writeln!(
            eio,
            " Output Reporting Tolerances, {}, {}, ",
            rounded(owners.hvac.deviation_from_setpoint_heating.abs(), 3)?,
            rounded(owners.hvac.deviation_from_setpoint_cooling, 3)?
        )?;
        Ok(())
    }
}
