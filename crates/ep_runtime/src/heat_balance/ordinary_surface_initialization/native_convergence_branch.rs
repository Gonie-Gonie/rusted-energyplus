//! Specified convergence suffix replay, not a full inside surface solver call.
use super::{
    OrdinarySurfaceInitialization, SourceInsideCtfProviders, SourceSurfaceHeatBalanceArrays,
};
use crate::heat_balance as hb;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use hb::error_backend::{SourceErrorBackend, SourceErrorState, SourceErrorTracking};
use hb::ordinary_air_heat_balance::SourceAirHeatBalance;
use hb::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use hb::selected_heat_balance_input::{
    HeatBalanceInputError, surface::SurfaceGeometryInputState, zone::SelectedZoneInputOwners,
};
use hb::selected_simulation_startup::SelectedHvacGlobals;
use hb::source_initialization::SourceRecurringErrorState;
use serde_json::{Value, json};
use std::{collections::BTreeMap, io};

type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}
fn member<'a>(v: &'a Value, key: &str) -> Result<&'a Value> {
    v.as_object()
        .and_then(|v| v.get(key))
        .ok_or_else(|| invalid(key).into())
}
fn array(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().ok_or_else(|| invalid("actual array").into())
}
fn text(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| invalid("actual string").into())
}
fn integer(v: &Value) -> Result<i32> {
    Ok(i32::try_from(
        v.as_i64().ok_or_else(|| invalid("signed integer"))?,
    )?)
}
fn boolean(v: &Value) -> Result<bool> {
    v.as_bool().ok_or_else(|| invalid("actual bool").into())
}
fn real(v: &Value) -> Result<f64> {
    let s = text(member(v, "value_bits")?)?;
    if v.as_object().is_none_or(|v| v.len() != 1)
        || s.len() != 16
        || !s
            .bytes()
            .all(|c| c.is_ascii_digit() || (b'a'..=b'f').contains(&c))
    {
        return Err(invalid("one finite raw16 primary").into());
    }
    let n = f64::from_bits(u64::from_str_radix(s, 16)?);
    if !n.is_finite() {
        return Err(invalid("finite raw primary").into());
    }
    Ok(n)
}
fn number(v: f64) -> Value {
    json!({"value_bits":format!("{:016x}",v.to_bits())})
}
fn pointer<T>(v: &T) -> Value {
    json!(format!("{v:p}"))
}
fn same<T: std::fmt::Debug + PartialEq>(a: T, b: T, label: &str) -> Result {
    if a != b {
        return Err(format!("{label}: Source{a:?} != Native{b:?}").into());
    }
    Ok(())
}

// Same actual constructor fields as DataGlobal/Utility/ErrorTracking. This is
// explicit test-only entry construction, not a reset of a previously run owner.
fn cold_error_state() -> SourceErrorState {
    SourceErrorState {
        output_error_header: true,
        version_string: String::new(),
        idd_version_string: String::new(),
        doing_input_processing: false,
        print_console_output: true,
        kickoff_simulation: false,
        developer_flag: false,
        warmup_flag: false,
        doing_sizing: false,
        tracking: SourceErrorTracking {
            ask_for_plant_check_on_abort: false,
            exit_during_simulations: false,
            match_counts: [0; 20],
            abort_processing: false,
            ask_for_connections_report: false,
            ask_for_surfaces_report: false,
            total_multiplied_windows: 0,
            total_coincident_vertices: 0,
            total_degenerate_surfaces: 0,
            total_receiving_non_convex_surfaces: 0,
            total_casting_non_convex_surfaces: 0,
            total_severe_errors: 0,
            total_severe_errors_during_warmup: 0,
            total_severe_errors_during_sizing: 0,
            last_severe_error: String::new(),
            total_warning_errors: 0,
            total_warning_errors_during_warmup: 0,
            total_warning_errors_during_sizing: 0,
        },
    }
}
struct Owners {
    surface: OrdinarySurfaceInitialization,
    globals: SelectedHeatBalanceGlobals,
    clock: WeatherGlobalState,
    environment: WeatherEnvironmentState,
    hvac: SelectedHvacGlobals,
    recurring: SourceRecurringErrorState,
    errors: SourceErrorState,
    error: Vec<u8>,
    console: Vec<u8>,
}
impl Owners {
    fn new(named: &Value) -> Result<Box<Self>> {
        let mut out = Box::new(Self {
            surface: Default::default(),
            globals: Default::default(),
            clock: Default::default(),
            environment: Default::default(),
            hvac: Default::default(),
            recurring: Default::default(),
            errors: cold_error_state(),
            error: Vec::new(),
            console: Vec::new(),
        });
        out.environment.environment_name = text(member(named, "EnvironmentName")?)?.into();
        out.environment.current_month_day = text(member(named, "CurMnDy")?)?.into();
        Ok(out)
    }
    fn observe(&self) -> Result<Value> {
        let s = &self.surface;
        let e = &self.errors.tracking;
        let rows = self.recurring.rows.as_ref();
        let size = rows.map_or(0, Vec::len);
        let mut items = Vec::new();
        if let Some(rows) = rows {
            for r in rows {
                items.push(json!({"address":pointer(r),"Message":r.message,"Count":r.count,
            "WarmupCount":r.warmup_count,"SizingCount":r.sizing_count,"MaxValue":number(r.max_value),
            "MinValue":number(r.min_value),"SumValue":number(r.sum_value),"MaxUnits":r.max_units,"MinUnits":r.min_units,
            "SumUnits":r.sum_units,"ReportMax":r.report_max,"ReportMin":r.report_min,"ReportSum":r.report_sum}));
            }
        }
        Ok(
            json!({"state_address":pointer(self),"manager_address":pointer(s),"tracking_address":pointer(e),"limits_address":pointer(s),
            "InsideSurfIterations":s.inside_surface_iterations,"MinIterations":s.minimum_inside_surface_iterations,"MaxIterations":500,
            "MaxAllowedDelTemp":number(s.maximum_allowed_temperature_difference),"MaxAllowedDelTempCondFD":number(s.maximum_allowed_temperature_difference_cond_fd),
            "calcHeatBalInsideSurfErrCount":s.ctf_inside_error_count,"calcHeatBalInsideSurfErrPointer":s.ctf_inside_error_pointer,
            "TotalWarningErrors":e.total_warning_errors,"TotalWarningErrorsDuringWarmup":e.total_warning_errors_during_warmup,
            "TotalWarningErrorsDuringSizing":e.total_warning_errors_during_sizing,"TotalSevereErrors":e.total_severe_errors,
            "TotalSevereErrorsDuringWarmup":e.total_severe_errors_during_warmup,"TotalSevereErrorsDuringSizing":e.total_severe_errors_during_sizing,
            "AbortProcessing":e.abort_processing,"MatchCounts":{"allocated":true,"size":e.match_counts.len(),"lower":1,"upper":e.match_counts.len(),"items":e.match_counts},
            "RecurringErrors":{"allocated":rows.is_some(),"size":size,"lower":1,"upper":size,"NumRecurringErrors":self.recurring.number,"items":items},
            "WarmupFlag":self.clock.warmup_flag,"DoingSizing":self.globals.doing_sizing,"KickOffSimulation":self.globals.kickoff_simulation,
            "CurrentTime":number(self.clock.current_time),"TimeStepZone":number(self.clock.time_step_zone),
            "SysTimeElapsed":number(self.hvac.system_time_elapsed),"TimeStepSys":number(self.hvac.time_step_sys),
            "EnvironmentName":self.environment.environment_name,"CurMnDy":self.environment.current_month_day,
            "outputErrorHeader":self.errors.output_error_header,"VerStringVar":self.errors.version_string,"IDDVerString":self.errors.idd_version_string,
            "DoingInputProcessing":self.errors.doing_input_processing,"printConsoleOutput":self.errors.print_console_output,
            "errorCallback_present":false,"sqlite_present":false,"error_stream":std::str::from_utf8(&self.error)?}),
        )
    }
    fn ingress(&mut self, input: &Value) -> Result<(f64, bool)> {
        let iterations = integer(member(input, "InsideSurfIterations")?)?;
        if !(1..=501).contains(&iterations) {
            return Err(invalid("bounded specified branch iteration").into());
        }
        self.surface.inside_surface_iterations = iterations;
        let maximum = real(member(input, "MaxDelTemp")?)?;
        if maximum < 0.0 {
            return Err(invalid("nonnegative reduced branch maximum").into());
        }
        self.clock.warmup_flag = boolean(member(input, "WarmupFlag")?)?;
        self.globals.doing_sizing = boolean(member(input, "DoingSizing")?)?;
        self.clock.current_time = real(member(input, "CurrentTime")?)?;
        self.clock.time_step_zone = real(member(input, "TimeStepZone")?)?;
        self.hvac.system_time_elapsed = real(member(input, "SysTimeElapsed")?)?;
        self.hvac.time_step_sys = real(member(input, "TimeStepSys")?)?;
        Ok((maximum, boolean(member(input, "Converged")?)?))
    }
    fn invoke(&mut self, maximum: f64, converged: &mut bool) -> Result<bool> {
        let mut backend = SourceErrorBackend {
            state: &mut self.errors,
            error_stream: Some(&mut self.error),
            console: &mut self.console,
            out_unit1: None,
            out_unit2: None,
            f_message: None,
            message_callback: None,
            error_callback: None,
            sqlite: None,
        };
        let mut providers = DiagnosticOwners {
            clock: &self.clock,
            globals: &self.globals,
            environment: &self.environment,
            hvac: &self.hvac,
            recurring: &mut self.recurring,
        };
        let result = self.surface.check_source_inside_convergence(
            maximum,
            converged,
            &self.globals,
            &self.clock,
            &mut providers,
            &mut backend,
        )?;
        if let Some(outcome) = result {
            same(
                outcome.iterations,
                self.surface.inside_surface_iterations,
                "actual suffix outcome iteration",
            )?;
            same(
                outcome.converged,
                *converged,
                "actual suffix outcome convergence",
            )?;
            same(
                outcome.maximum_temperature_difference.to_bits(),
                maximum.to_bits(),
                "actual suffix outcome maximum",
            )?;
        } else {
            same(*converged, false, "actual suffix continuation")?;
        }
        // The production helper returns an outcome on either maximum break or
        // convergence. Its unchanged literal maximum predicate distinguishes
        // these here; this is code-derived, not a new passive Source event.
        Ok(self.surface.inside_surface_iterations <= 500)
    }
}
struct DiagnosticOwners<'a> {
    clock: &'a WeatherGlobalState,
    globals: &'a SelectedHeatBalanceGlobals,
    environment: &'a WeatherEnvironmentState,
    hvac: &'a SelectedHvacGlobals,
    recurring: &'a mut SourceRecurringErrorState,
}
impl SourceInsideCtfProviders for DiagnosticOwners<'_> {
    fn initialize_inside_convection(
        &mut self,
        _geometry: &mut SurfaceGeometryInputState,
        _arrays: &mut SourceSurfaceHeatBalanceArrays,
        _zones: &SelectedZoneInputOwners,
        _air: &SourceAirHeatBalance,
        _globals: &SelectedHeatBalanceGlobals,
        _clock: &WeatherGlobalState,
        _backend: &mut SourceErrorBackend<'_>,
    ) -> std::result::Result<(), HeatBalanceInputError> {
        Err(HeatBalanceInputError::Unavailable(
            "specified convergence suffix never calls inside convection",
        ))
    }
    fn test_surface_temperature(
        &mut self,
        _temperature: f64,
        _surface: i32,
        _zone: i32,
        _geometry: &mut SurfaceGeometryInputState,
        _zones: &mut SelectedZoneInputOwners,
        _warmup_error_count: &mut i32,
        _backend: &mut SourceErrorBackend<'_>,
    ) -> std::result::Result<(), HeatBalanceInputError> {
        Err(HeatBalanceInputError::Unavailable(
            "specified convergence suffix never calls surface-temperature diagnostic",
        ))
    }
    fn warn_nonconvergence(
        &mut self,
        maximum: f64,
        allowed: f64,
        backend: &mut SourceErrorBackend<'_>,
    ) -> std::result::Result<(), HeatBalanceInputError> {
        super::inside_providers::source_inside_nonconvergence_warning(
            maximum,
            allowed,
            self.clock,
            self.globals,
            self.environment,
            self.hvac,
            backend,
        )
    }
    fn recurring_nonconvergence(
        &mut self,
        maximum: f64,
        index: &mut i32,
        backend: &mut SourceErrorBackend<'_>,
    ) -> std::result::Result<(), HeatBalanceInputError> {
        super::inside_providers::source_inside_nonconvergence_recurring(
            maximum,
            index,
            self.recurring,
            self.clock,
            self.globals,
            backend,
        )
    }
}

#[derive(Default)]
struct Identity {
    native_to_source: BTreeMap<(String, String), String>,
    checks: usize,
}
impl Identity {
    fn compare(&mut self, source: &Value, native: &Value, path: &str) -> Result {
        let key = path.rsplit('.').next().unwrap_or(path);
        if key == "address" || key.ends_with("_address") {
            let a = text(source)?;
            let b = text(native)?;
            for v in [a, b] {
                let hex = v.strip_prefix("0x").unwrap_or(v);
                if u128::from_str_radix(hex, 16)? == 0 {
                    return Err(invalid("nonnull actual owner address").into());
                }
            }
            if let Some(previous) = self
                .native_to_source
                .insert((path.into(), b.into()), a.into())
            {
                same(
                    previous,
                    a.to_owned(),
                    "same per-lifetime observed owner/storage",
                )?;
            }
            self.checks += 1;
            return Ok(());
        }
        match (source, native) {
            (Value::Object(a), Value::Object(b)) => {
                same(
                    a.keys().collect::<Vec<_>>(),
                    b.keys().collect::<Vec<_>>(),
                    &format!("{path} actual record keys"),
                )?;
                for (key, a) in a {
                    self.compare(a, &b[key], &format!("{path}.{key}"))?;
                }
            }
            (Value::Array(a), Value::Array(b)) => {
                same(a.len(), b.len(), &format!("{path} extent"))?;
                for (i, (a, b)) in a.iter().zip(b).enumerate() {
                    self.compare(a, b, &format!("{path}.{i}"))?;
                }
            }
            _ => {
                same(source, native, path)?;
                self.checks += 1;
            }
        }
        Ok(())
    }
}
impl OrdinarySurfaceInitialization {
    pub(crate) fn run_native_inside_convergence_branch_cohort(fixture: &Value) -> Result<usize> {
        let named = member(
            member(fixture, "named_inputs")?,
            "inside_convergence_branch",
        )?;
        let native = member(
            member(member(fixture, "native_output")?, "groups")?,
            "inside_convergence_branch",
        )?;
        same(
            text(member(named, "schema")?)?,
            "src06-literal-Original-inside-convergence-branch-input.v1",
            "named branch schema",
        )?;
        same(
            text(member(native, "schema")?)?,
            "src06-literal-Original-inside-convergence-branch-output.v1",
            "Native branch schema",
        )?;
        let inputs = array(member(named, "lifetimes")?)?;
        let outputs = array(member(native, "lifetimes")?)?;
        same(
            (inputs.len(), outputs.len()),
            (3, 3),
            "all persistent branch lifetimes",
        )?;
        let mut records = 0usize;
        let mut checks = 0usize;
        for (input, output) in inputs.iter().zip(outputs) {
            same(
                input,
                member(output, "input")?,
                "whole declared lifetime primary",
            )?;
            same(
                member(input, "name")?,
                member(output, "name")?,
                "lifetime name",
            )?;
            let mut owner = Owners::new(input)?;
            let mut identity = Identity::default();
            identity.compare(
                &owner.observe()?,
                member(output, "constructor_state")?,
                "state",
            )?;
            let calls = array(member(input, "calls")?)?;
            let expected = array(member(output, "records")?)?;
            same(
                calls.len(),
                expected.len(),
                "whole persistent call sequence",
            )?;
            for (input, expected) in calls.iter().zip(expected) {
                same(
                    input,
                    member(expected, "input")?,
                    "whole declared branch entry",
                )?;
                let (maximum, mut converged) = owner.ingress(input)?;
                identity.compare(&owner.observe()?, member(expected, "entry")?, "state")?;
                let continuation = owner.invoke(maximum, &mut converged)?;
                identity.compare(&owner.observe()?, member(expected, "exit")?, "state")?;
                same(
                    converged,
                    boolean(member(expected, "Converged")?)?,
                    "actual suffix convergence",
                )?;
                same(
                    number(maximum),
                    member(expected, "MaxDelTemp")?.clone(),
                    "unchanged branch-entry maximum",
                )?;
                same(
                    continuation,
                    boolean(member(expected, "continued_past_maximum_block")?)?,
                    "Original literal maximum control vs unchanged Source predicate",
                )?;
                records += 1;
            }
            identity.compare(&owner.observe()?, member(output, "final_state")?, "state")?;
            checks += identity.checks;
        }
        same(
            records,
            30,
            "all specified literal Original branch evaluations",
        )?;
        println!(
            "Source SUR05 literal branch cohort: {records} suffix evaluations; 3 persistent lifetimes; {checks} exact scalar/text/identity checks; 0 full inside-solver calls; Source maximum control classification code-derived"
        );
        Ok(records)
    }
}
