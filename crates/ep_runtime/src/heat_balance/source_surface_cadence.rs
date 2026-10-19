//! Optional passive records from the actual ordinary Source surface owners.
//! Inert unless explicitly captured. No getter, numerical call or science write.
use super::ordinary_surface_initialization::SourceSurfaceHeatBalanceArrays;
use super::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};
use std::cell::{Cell, RefCell};
use std::io::{self, Write};

const MAX_RECORDS: usize = 200_000;
const MAX_DEPTH: usize = 16;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Actual surface-manager scopes, including separately labeled Source-only warmup scopes.
pub enum Procedure {
    /// ManageSurfaceHeatBalance entry and return.
    Manage,
    /// InitSurfaceHeatBalance entry and return.
    Initialize,
    /// Initial temperature and flux-history setup.
    InitializeHistories,
    /// Accepted surface temperature and flux-history update.
    UpdateHistories,
    /// Opaque CTF inside-surface iteration.
    InsideCtf,
    /// Surface report calculation and output.
    Report,
    /// Source-only warmup heat-balance recording.
    SourceRecord,
    /// Source-only warmup convergence check.
    SourceCheck,
}
impl Procedure {
    /// Stable Native method name or explicitly Source-only tag written into the trace.
    pub fn name(self) -> &'static str {
        match self {
            Self::Manage => "ManageSurfaceHeatBalance",
            Self::Initialize => "InitSurfaceHeatBalance",
            Self::InitializeHistories => "InitThermalAndFluxHistories",
            Self::UpdateHistories => "UpdateThermalHistories",
            Self::InsideCtf => "CalcHeatBalanceInsideSurf2CTFOnly",
            Self::Report => "ReportSurfaceHeatBalance",
            Self::SourceRecord => "SourceOnly_RecKeepHeatBalance",
            Self::SourceCheck => "SourceOnly_CheckWarmupConvergence",
        }
    }
}

/// Real scalar frame. Environment fields are unchanged during an inside call;
/// inner-loop updates read every current clock/global operand again.
#[derive(Clone, Copy, Debug)]
pub struct Frame {
    /// Identity of the currently borrowed Source owner; valid within this call, not across moves.
    pub owner: usize,
    /// Actual current environment ordinal.
    pub environment_index: i32,
    /// Actual simulation calendar year.
    pub calendar_year: i32,
    /// Actual weather-environment month.
    pub month: i32,
    /// Actual weather-environment day of month.
    pub day_of_month: i32,
    /// Actual simulation day counter, including the warmup transition reset.
    pub day_of_sim: i32,
    /// Actual hour-of-day counter.
    pub hour: i32,
    /// Actual zone-timestep ordinal within the hour.
    pub timestep: i32,
    /// Actual kickoff state supplied by the caller.
    pub kickoff: bool,
    /// Actual warmup flag.
    pub warmup: bool,
    /// Actual beginning-of-simulation flag.
    pub begin_sim: bool,
    /// Actual beginning-of-environment flag.
    pub begin_environment: bool,
    /// Actual beginning-of-day flag.
    pub begin_day: bool,
    /// Actual beginning-of-hour flag.
    pub begin_hour: bool,
    /// Actual beginning-of-zone-timestep flag.
    pub begin_timestep: bool,
    /// Actual ZoneSizingCalc state, distinct from DoingSizing.
    pub sizing: bool,
    /// Completed inside-surface iterations at this observation point.
    pub completed_inside_iterations: i32,
    /// Additional Source-only field, absent from the retained Native collector.
    pub end_day: bool,
}
impl Frame {
    pub(crate) fn from_owners<T>(
        owner: &T,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
        environment: &WeatherEnvironmentState,
        kickoff: bool,
        completed_inside_iterations: i32,
    ) -> Self {
        Self {
            owner: std::ptr::from_ref(owner) as usize,
            environment_index: environment.cur_envir_num,
            calendar_year: clock.calendar_year,
            month: environment.month,
            day_of_month: environment.day_of_month,
            day_of_sim: clock.day_of_sim,
            hour: clock.hour_of_day,
            timestep: clock.time_step,
            kickoff,
            warmup: clock.warmup_flag,
            begin_sim: clock.begin_sim_flag,
            begin_environment: clock.begin_envrn_flag,
            begin_day: clock.begin_day_flag,
            begin_hour: clock.begin_hour_flag,
            begin_timestep: clock.begin_time_step_flag,
            sizing: globals.zone_sizing_calculation,
            completed_inside_iterations,
            end_day: clock.end_day_flag,
        }
    }
    fn current_clock(
        mut self,
        clock: &WeatherGlobalState,
        globals: &SelectedHeatBalanceGlobals,
        kickoff: bool,
        completed_inside_iterations: i32,
    ) -> Self {
        self.calendar_year = clock.calendar_year;
        self.day_of_sim = clock.day_of_sim;
        self.hour = clock.hour_of_day;
        self.timestep = clock.time_step;
        self.kickoff = kickoff;
        self.warmup = clock.warmup_flag;
        self.begin_sim = clock.begin_sim_flag;
        self.begin_environment = clock.begin_envrn_flag;
        self.begin_day = clock.begin_day_flag;
        self.begin_hour = clock.begin_hour_flag;
        self.begin_timestep = clock.begin_time_step_flag;
        self.sizing = globals.zone_sizing_calculation;
        self.completed_inside_iterations = completed_inside_iterations;
        self.end_day = clock.end_day_flag;
        self
    }
}

#[derive(Clone, Copy, Debug, Default)]
/// Optional passive operands attached to one actual surface-cadence event.
pub struct Detail {
    /// Actual physical surface ordinal when this checkpoint names a surface.
    pub surface: Option<i32>,
    /// Raw binary64 bits of the actual inside convection coefficient.
    pub convection_bits: Option<u64>,
    /// Address of the borrowed convection-array element for within-call storage checks.
    pub convection_storage: Option<usize>,
    /// Raw binary64 bits of the actual maximum convergence temperature delta.
    pub maximum_delta_bits: Option<u64>,
    /// Actual convergence decision when present at this checkpoint.
    pub converged: Option<bool>,
    /// Actual pre-solve or temperature-only surface observation.
    pub inside_snapshot: Option<InsideSnapshot>,
    /// True only for an early exit where live owners are no longer borrowed.
    pub entry_frame_on_failure: bool,
}
/// Only actual retained fields. Full first-two pre-solve snapshots have the
/// same21 reals/three flags as the existing Native Copy::Inside observer.
#[derive(Clone, Copy, Debug)]
// Retain the actual observer snapshot payload layout; boxing solely for this lint would alter its ownership representation.
#[allow(clippy::large_enum_variant)]
pub enum InsideSnapshot {
    /// First-two pre-solve fields in the retained Native inside-observer order.
    Full {
        /// Raw bits for the21 actual real fields; None preserves unavailable storage.
        values: [Option<u64>; 21],
        /// Actual adiabatic, pool and source/sink flags in the retained Native order.
        flags: [Option<i32>; 3],
    },
    /// Actual inside and temporary temperatures after assignment or at loop exit.
    Temperature {
        /// Raw binary64 bits of SurfTempIn, if its actual owner row exists.
        inside: Option<u64>,
        /// Raw binary64 bits of SurfTempInTmp, if its actual owner row exists.
        temporary: Option<u64>,
    },
}
impl InsideSnapshot {
    pub(crate) fn full(arrays: &SourceSurfaceHeatBalanceArrays, surface: i32) -> Self {
        let at = surface.checked_sub(1).and_then(|v| usize::try_from(v).ok());
        let real = |v: &Option<Vec<f64>>| {
            at.and_then(|i| v.as_deref().and_then(|v| v.get(i)))
                .map(|v| v.to_bits())
        };
        let flag = |v: &Option<Vec<i32>>| {
            at.and_then(|i| v.as_deref().and_then(|v| v.get(i)))
                .copied()
        };
        Self::Full {
            values: [
                real(&arrays.pool_heat_trans_coefs),
                real(&arrays.q_pool_surf_numerator),
                real(&arrays.q_rad_surf_afn_duct),
                real(&arrays.ref_air_temp),
                real(&arrays.surf_ctf_const_in_part),
                real(&arrays.surf_ctf_cross0),
                real(&arrays.surf_ctf_inside0),
                real(&arrays.surf_ctf_source_in0),
                real(&arrays.surf_h_conv_int),
                real(&arrays.surf_opaq_q_rad_sw_in_abs),
                real(&arrays.surf_q_additional_heat_source_inside),
                real(&arrays.surf_q_source_sink_hist),
                real(&arrays.surf_qdot_rad_hvac_in_per_area),
                real(&arrays.surf_qdot_rad_int_gains_in_per_area),
                real(&arrays.surf_qdot_rad_net_lw_in_per_area),
                real(&arrays.surf_temp_div),
                real(&arrays.surf_temp_in),
                real(&arrays.surf_temp_in_tmp),
                real(&arrays.surf_temp_ins_old),
                real(&arrays.surf_temp_out_hist),
                real(&arrays.surf_temp_term),
            ],
            flags: [
                flag(&arrays.surf_is_adiabatic),
                flag(&arrays.surf_is_operating_pool),
                flag(&arrays.surf_is_source_or_sink),
            ],
        }
    }
    pub(crate) fn temperature(arrays: &SourceSurfaceHeatBalanceArrays, surface: i32) -> Self {
        let at = surface.checked_sub(1).and_then(|v| usize::try_from(v).ok());
        let real = |v: &Option<Vec<f64>>| {
            at.and_then(|i| v.as_deref().and_then(|v| v.get(i)))
                .map(|v| v.to_bits())
        };
        Self::Temperature {
            inside: real(&arrays.surf_temp_in),
            temporary: real(&arrays.surf_temp_in_tmp),
        }
    }
    fn write_json(self, out: &mut impl Write) -> io::Result<()> {
        match self {
            Self::Full { values, flags } => {
                const NAMES: [&str; 21] = [
                    "PoolHeatTransCoefs",
                    "QPoolSurfNumerator",
                    "QRadSurfAFNDuct",
                    "RefAirTemp",
                    "SurfCTFConstInPart",
                    "SurfCTFCross0",
                    "SurfCTFInside0",
                    "SurfCTFSourceIn0",
                    "SurfHConvInt",
                    "SurfOpaqQRadSWInAbs",
                    "SurfQAdditionalHeatSourceInside",
                    "SurfQSourceSinkHist",
                    "SurfQdotRadHVACInPerArea",
                    "SurfQdotRadIntGainsInPerArea",
                    "SurfQdotRadNetLWInPerArea",
                    "SurfTempDiv",
                    "SurfTempIn",
                    "SurfTempInTmp",
                    "SurfTempInsOld",
                    "SurfTempOutHist",
                    "SurfTempTerm",
                ];
                out.write_all(b"{")?;
                for (at, (name, value)) in NAMES.into_iter().zip(values).enumerate() {
                    if at > 0 {
                        out.write_all(b",")?;
                    }
                    write!(out, "\"{name}\":")?;
                    optional_bits(out, value)?;
                }
                for (name, value) in [
                    "SurfIsAdiabatic",
                    "SurfIsOperatingPool",
                    "SurfIsSourceOrSink",
                ]
                .into_iter()
                .zip(flags)
                {
                    write!(out, ",\"{name}\":")?;
                    optional_number(out, value)?;
                }
                out.write_all(b"}")
            }
            Self::Temperature { inside, temporary } => {
                out.write_all(b"{\"SurfTempIn\":")?;
                optional_bits(out, inside)?;
                out.write_all(b",\"SurfTempInTmp\":")?;
                optional_bits(out, temporary)?;
                out.write_all(b"}")
            }
        }
    }
}
#[derive(Clone, Copy, Debug)]
/// One bounded passive event from the currently executing Source scope.
pub struct Event {
    /// Monotonic observation sequence within this capture.
    pub sequence: u64,
    /// Static checkpoint tag identifying the real entry, return or loop point.
    pub checkpoint: &'static str,
    /// Actual active surface-manager procedure, when a scope is open.
    pub procedure: Option<Procedure>,
    /// Actual invocation ordinal within that procedure.
    pub invocation: Option<u64>,
    /// Actual enclosing procedure, when nested in another observed scope.
    pub parent_procedure: Option<Procedure>,
    /// Actual invocation ordinal of the enclosing procedure.
    pub parent_invocation: Option<u64>,
    /// Actual clock and borrowed-owner identity at the checkpoint.
    pub frame: Frame,
    /// Optional actual operands observed at the checkpoint.
    pub detail: Detail,
}
#[derive(Clone, Copy)]
struct Context {
    procedure: Procedure,
    invocation: u64,
    frame: Frame,
}
struct Buffer {
    generation: u64,
    limit: usize,
    attempted: u64,
    omitted: u64,
    counter_overflow: bool,
    stack_error: bool,
    failed_scopes: u64,
    calls: [u64; 8],
    stack: [Option<Context>; MAX_DEPTH],
    depth: usize,
    events: Vec<Event>,
    handoffs: Vec<serde_json::Value>,
    handoff_error: bool,
}
thread_local! {
    // Initializer is already const; constrain the Rust1.96 macro-expansion lint to this exact thread-local declaration.
    #[allow(clippy::missing_const_for_thread_local)]
    static ACTIVE: RefCell<Option<Buffer>> = const { RefCell::new(None) };
    // Initializer is already const; constrain the Rust1.96 macro-expansion lint to this exact thread-local declaration.
    #[allow(clippy::missing_const_for_thread_local)]
    static GENERATION: Cell<u64> = const { Cell::new(0) };
}

/// Bounded results only. A complete trace requires no omissions, errors or open scopes.
pub struct Trace {
    /// Configured bound on retained cadence events.
    pub record_limit: usize,
    /// Number of attempted cadence-event records.
    pub attempted: u64,
    /// Number of cadence events omitted after the configured bound.
    pub omitted: u64,
    /// Whether an observation counter overflowed.
    pub counter_overflow: bool,
    /// Whether scope order or capture-generation validation failed.
    pub stack_error: bool,
    /// Number of scopes that exited without a successful completion marker.
    pub failed_scopes: u64,
    /// Number of scopes still open when capture ended.
    pub remaining_scope_depth: usize,
    /// Actual invocation counts in Procedure discriminant order.
    pub calls: [u64; 8],
    /// Cadence events retained in their actual observation order.
    pub events: Vec<Event>,
    /// Up to two transition observations; the finite A24 comparer requires both.
    pub handoffs: Vec<serde_json::Value>,
    /// Whether a selected transition had invalid order, shape or excess observations.
    pub handoff_error: bool,
}
impl Trace {
    /// Whether capture ended without omissions, errors or open scopes; does not require two handoffs.
    pub fn complete(&self) -> bool {
        !self.handoff_error
            && self.omitted == 0
            && !self.counter_overflow
            && !self.stack_error
            && self.failed_scopes == 0
            && self.remaining_scope_depth == 0
    }
    /// Cadence rows use fixed field/tag names; handoff values use serde_json.
    /// This writer serializes retained observations without a scientific call.
    pub fn write_json(&self, out: &mut impl Write) -> io::Result<()> {
        write!(
            out,
            "{{\"schema\":\"src06-source-Surface-cadence.v1\",\"complete\":{},\"record_limit\":{},\"attempted\":{},\"omitted\":{},\"counter_overflow\":{},\"stack_error\":{},\"failed_scopes\":{},\"remaining_scope_depth\":{},\"calls\":{:?},\"events\":[",
            self.complete(),
            self.record_limit,
            self.attempted,
            self.omitted,
            self.counter_overflow,
            self.stack_error,
            self.failed_scopes,
            self.remaining_scope_depth,
            self.calls
        )?;
        for (at, event) in self.events.iter().enumerate() {
            if at > 0 {
                out.write_all(b",")?;
            }
            write!(out, "{{\"sequence\":{},\"comparison_row\":", event.sequence)?;
            event.write_comparison_row(out)?;
            let f = event.frame;
            let d = event.detail;
            write!(
                out,
                ",\"Source_owner\":{},\"Source_EndDayFlag\":{},\"entry_frame_on_failure\":{},\"surface\":",
                f.owner, f.end_day, d.entry_frame_on_failure
            )?;
            optional_number(out, d.surface)?;
            out.write_all(b",\"convection_bits\":")?;
            optional_bits(out, d.convection_bits)?;
            out.write_all(b",\"convection_storage\":")?;
            optional_number(out, d.convection_storage)?;
            out.write_all(b",\"maximum_delta_bits\":")?;
            optional_bits(out, d.maximum_delta_bits)?;
            out.write_all(b",\"converged\":")?;
            optional_number(out, d.converged)?;
            out.write_all(b",\"inside_snapshot\":")?;
            if let Some(snapshot) = d.inside_snapshot {
                snapshot.write_json(out)?;
            } else {
                out.write_all(b"null")?;
            }
            out.write_all(b"}")?;
        }
        write!(
            out,
            "],\"handoff_error\":{},\"handoffs\":",
            self.handoff_error
        )?;
        serde_json::to_writer(&mut *out, &self.handoffs).map_err(io::Error::other)?;
        out.write_all(b"}\n")
    }
}
fn optional_number<T: std::fmt::Display>(out: &mut impl Write, value: Option<T>) -> io::Result<()> {
    if let Some(v) = value {
        write!(out, "{v}")
    } else {
        out.write_all(b"null")
    }
}
fn optional_bits(out: &mut impl Write, value: Option<u64>) -> io::Result<()> {
    if let Some(v) = value {
        write!(out, "\"{v:016x}\"")
    } else {
        out.write_all(b"null")
    }
}
fn optional_procedure(out: &mut impl Write, value: Option<Procedure>) -> io::Result<()> {
    if let Some(v) = value {
        write!(out, "\"{}\"", v.name())
    } else {
        out.write_all(b"null")
    }
}
impl Event {
    /// Same canonical array schema used by the retained34,437-row Native index.
    pub fn write_comparison_row(&self, out: &mut impl Write) -> io::Result<()> {
        write!(out, "[\"{}\",", self.checkpoint)?;
        optional_procedure(out, self.procedure)?;
        out.write_all(b",")?;
        optional_number(out, self.invocation)?;
        out.write_all(b",")?;
        optional_procedure(out, self.parent_procedure)?;
        out.write_all(b",")?;
        optional_number(out, self.parent_invocation)?;
        let f = self.frame;
        write!(
            out,
            ",{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}]",
            f.environment_index,
            f.calendar_year,
            f.month,
            f.day_of_month,
            f.day_of_sim,
            f.hour,
            f.timestep,
            f.kickoff,
            f.warmup,
            f.begin_sim,
            f.begin_environment,
            f.begin_day,
            f.begin_hour,
            f.begin_timestep,
            f.sizing,
            f.completed_inside_iterations
        )
    }
}
struct CaptureGuard {
    previous: Option<Buffer>,
}
impl Drop for CaptureGuard {
    fn drop(&mut self) {
        ACTIVE.with_borrow_mut(|active| *active = self.previous.take());
    }
}
/// One call of the original closure, including its original error return. A
/// nested capture and panic restore the previous collector through Drop.
pub fn capture<T>(limit: usize, call: impl FnOnce() -> T) -> io::Result<(T, Trace)> {
    if limit == 0 || limit > MAX_RECORDS {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Source cadence record limit outside1..=200000",
        ));
    }
    let mut events = Vec::new();
    events.try_reserve_exact(limit).map_err(io::Error::other)?;
    let generation = GENERATION
        .with(|value| value.get().checked_add(1).inspect(|next| value.set(*next)))
        .ok_or_else(|| io::Error::other("Source cadence capture generation exhausted"))?;
    let previous = ACTIVE.with_borrow_mut(|active| {
        active.replace(Buffer {
            generation,
            limit,
            attempted: 0,
            omitted: 0,
            counter_overflow: false,
            stack_error: false,
            failed_scopes: 0,
            calls: [0; 8],
            stack: [None; MAX_DEPTH],
            depth: 0,
            events,
            handoffs: Vec::new(),
            handoff_error: false,
        })
    });
    let guard = CaptureGuard { previous };
    let result = call();
    let current = ACTIVE
        .with_borrow_mut(Option::take)
        .ok_or_else(|| io::Error::other("Source cadence capture disappeared"))?;
    let trace = Trace {
        record_limit: current.limit,
        attempted: current.attempted,
        omitted: current.omitted,
        counter_overflow: current.counter_overflow,
        stack_error: current.stack_error,
        failed_scopes: current.failed_scopes,
        remaining_scope_depth: current.depth,
        calls: current.calls,
        events: current.events,
        handoffs: current.handoffs,
        handoff_error: current.handoff_error,
    };
    drop(guard);
    Ok((result, trace))
}
fn emit(buffer: &mut Buffer, checkpoint: &'static str, frame: Frame, detail: Detail) {
    let Some(sequence) = buffer.attempted.checked_add(1) else {
        buffer.counter_overflow = true;
        return;
    };
    buffer.attempted = sequence;
    if buffer.events.len() >= buffer.limit {
        if let Some(n) = buffer.omitted.checked_add(1) {
            buffer.omitted = n;
        } else {
            buffer.counter_overflow = true;
        }
        return;
    }
    let current = buffer.depth.checked_sub(1).and_then(|i| buffer.stack[i]);
    let parent = buffer.depth.checked_sub(2).and_then(|i| buffer.stack[i]);
    buffer.events.push(Event {
        sequence,
        checkpoint,
        procedure: current.map(|v| v.procedure),
        invocation: current.map(|v| v.invocation),
        parent_procedure: parent.map(|v| v.procedure),
        parent_invocation: parent.map(|v| v.invocation),
        frame,
        detail,
    });
}
pub(crate) fn record(checkpoint: &'static str, frame: Frame) {
    ACTIVE.with_borrow_mut(|active| {
        if let Some(buffer) = active {
            emit(buffer, checkpoint, frame, Detail::default());
        }
    });
}
pub(crate) fn active() -> bool {
    ACTIVE.with_borrow(Option::is_some)
}
pub(crate) struct Scope {
    token: Option<(u64, Procedure, u64)>,
    entry: Frame,
}
pub(crate) fn enter(procedure: Procedure, frame: Frame) -> Scope {
    let token = ACTIVE.with_borrow_mut(|active| {
        let buffer = active.as_mut()?;
        if buffer.depth >= MAX_DEPTH {
            buffer.stack_error = true;
            return None;
        }
        let Some(invocation) = buffer.calls[procedure as usize].checked_add(1) else {
            buffer.counter_overflow = true;
            return None;
        };
        buffer.calls[procedure as usize] = invocation;
        buffer.stack[buffer.depth] = Some(Context {
            procedure,
            invocation,
            frame,
        });
        buffer.depth += 1;
        emit(buffer, "scope_entry", frame, Detail::default());
        Some((buffer.generation, procedure, invocation))
    });
    Scope {
        token,
        entry: frame,
    }
}
impl Scope {
    fn finish_inner(&mut self, frame: Frame, success: bool, early: bool) {
        let Some((generation, procedure, invocation)) = self.token.take() else {
            return;
        };
        ACTIVE.with_borrow_mut(|active| {
            let Some(buffer) = active else {
                return;
            };
            if buffer.generation != generation {
                return;
            }
            let top = buffer.depth.checked_sub(1).and_then(|i| buffer.stack[i]);
            if top.is_none_or(|v| v.procedure != procedure || v.invocation != invocation) {
                buffer.stack_error = true;
                return;
            }
            let tag = if success {
                "scope_normal_exit"
            } else if early {
                "scope_early_exit"
            } else {
                "scope_source_error"
            };
            emit(
                buffer,
                tag,
                frame,
                Detail {
                    entry_frame_on_failure: early,
                    ..Detail::default()
                },
            );
            if !success {
                if let Some(n) = buffer.failed_scopes.checked_add(1) {
                    buffer.failed_scopes = n;
                } else {
                    buffer.counter_overflow = true;
                }
            }
            buffer.depth -= 1;
            buffer.stack[buffer.depth] = None;
        });
    }
    pub(crate) fn finish(mut self, frame: Frame, success: bool) {
        self.finish_inner(frame, success, false);
    }
}
impl Drop for Scope {
    fn drop(&mut self) {
        self.finish_inner(self.entry, false, true);
    }
}
/// Direct current loop values; environment identity is the exact enclosing
/// Inside scope entry frame, unchanged by the inside-only method signature.
pub(crate) fn inside_event(
    checkpoint: &'static str,
    clock: &WeatherGlobalState,
    globals: &SelectedHeatBalanceGlobals,
    kickoff: bool,
    completed_inside_iterations: i32,
    detail: Detail,
) {
    ACTIVE.with_borrow_mut(|active| {
        let Some(buffer) = active else {
            return;
        };
        let Some(context) = buffer.depth.checked_sub(1).and_then(|i| buffer.stack[i]) else {
            buffer.stack_error = true;
            return;
        };
        if context.procedure != Procedure::InsideCtf {
            buffer.stack_error = true;
            return;
        }
        let frame =
            context
                .frame
                .current_clock(clock, globals, kickoff, completed_inside_iterations);
        emit(buffer, checkpoint, frame, detail);
    });
}

// Bounded passive observations at the two actual warmup/RunPeriod boundaries.
// This observer never constructs, resets or mutates a scientific owner.
fn handoff_scalar(value: f64) -> serde_json::Value {
    serde_json::json!({ "value_bits": format!("{:016x}", value.to_bits()) })
}
fn handoff_real_array(values: &Option<Vec<f64>>) -> serde_json::Value {
    serde_json::json!({
        "allocated": values.is_some(),
        "size": values.as_ref().map(Vec::len),
        "storage": values.as_ref().map(|v| v.as_ptr() as usize),
        "items": values.as_ref().map(|v| v.iter().copied().map(handoff_scalar).collect::<Vec<_>>()),
    })
}
fn handoff_histories(
    values: &super::ctf_thermal_history::CtfThermalHistoryArrays,
) -> serde_json::Value {
    let rows = |rows: &[Vec<f64>]| {
        rows.iter()
            .enumerate()
            .map(|(term, row)| {
                serde_json::json!({
                    "term": term + 1,
                    "size": row.len(),
                    "storage": row.as_ptr() as usize,
                    "items": row.iter().copied().map(handoff_scalar).collect::<Vec<_>>(),
                })
            })
            .collect::<Vec<_>>()
    };
    // Source normal arrays are real Vec rows; an allocation flag is not
    // invented from their length. Every actual row/extent is retained.
    serde_json::json!({
        "SurfInsideTempHist": rows(&values.inside_temperature),
        "SurfOutsideTempHist": rows(&values.outside_temperature),
        "SurfInsideFluxHist": rows(&values.inside_flux),
        "SurfOutsideFluxHist": rows(&values.outside_flux),
    })
}
fn handoff_air_row(
    number: usize,
    air: &super::ordinary_air_heat_balance::SourceZoneSpaceHeatBalanceData,
) -> serde_json::Value {
    serde_json::json!({
        "ordinal": number,
        "storage": std::ptr::from_ref(air) as usize,
        "MAT": handoff_scalar(air.mat),
        "airHumRat": handoff_scalar(air.air_hum_rat),
    })
}
/// Only the two real selected RunPeriod transitions call this function.
/// The old per-event scope/order/inside snapshots and their counters stay exact.
pub(crate) fn record_handoff(
    checkpoint: &'static str,
    frame: Frame,
    kind_of_sim: i32,
    globals: &SelectedHeatBalanceGlobals,
    arrays: &SourceSurfaceHeatBalanceArrays,
    history: &super::ctf_thermal_history::CtfThermalHistoryState,
    air: &super::ordinary_air_heat_balance::SourceAirHeatBalance,
) {
    ACTIVE.with_borrow_mut(|active| {
        let Some(buffer) = active else {
            return;
        };
        let expected = ["after_warmup_complete", "before_first_run_init"];
        if buffer.handoffs.len() >= expected.len()
            || expected.get(buffer.handoffs.len()) != Some(&checkpoint)
            || kind_of_sim != 3
            || frame.kickoff
            || frame.sizing
            || globals.doing_sizing
            || frame.warmup
        {
            buffer.handoff_error = true;
            return;
        }
        let zones = air.zones.as_ref().map(|rows| {
            rows.iter()
                .enumerate()
                .map(|(i, row)| handoff_air_row(i + 1, &row.air))
                .collect::<Vec<_>>()
        });
        let spaces = air.spaces.as_ref().map(|rows| {
            rows.iter()
                .enumerate()
                .map(|(i, row)| handoff_air_row(i + 1, &row.air))
                .collect::<Vec<_>>()
        });
        let snapshot = serde_json::json!({
            "checkpoint": checkpoint,
            "clock": {
                "environment_index": frame.environment_index,
                "KindOfSim": kind_of_sim,
                "CalendarYear": frame.calendar_year,
                "Month": frame.month,
                "DayOfMonth": frame.day_of_month,
                "DayOfSim": frame.day_of_sim,
                "HourOfDay": frame.hour,
                "TimeStep": frame.timestep,
                "KickOffSimulation": frame.kickoff,
                "WarmupFlag": frame.warmup,
                "BeginSimFlag": frame.begin_sim,
                "BeginEnvrnFlag": frame.begin_environment,
                "BeginDayFlag": frame.begin_day,
                "BeginHourFlag": frame.begin_hour,
                "BeginTimeStepFlag": frame.begin_timestep,
                "ZoneSizingCalc": frame.sizing,
                "DoingSizing": globals.doing_sizing,
                "AnyEMS": globals.any_energy_management_system_in_model,
                "eplusRunningViaAPI": globals.running_via_api,
                "externalHVACManager_present": globals.external_hvac_manager.is_some(),
                "EndDayFlag": frame.end_day,
            },
            "Source_owner": frame.owner,
            "air_zones": {
                "allocated": air.zones.is_some(),
                "size": air.zones.as_ref().map(Vec::len),
                "storage": air.zones.as_ref().map(|v| v.as_ptr() as usize),
                "items": zones,
            },
            "air_spaces": {
                "allocated": air.spaces.is_some(),
                "size": air.spaces.as_ref().map(Vec::len),
                "storage": air.spaces.as_ref().map(|v| v.as_ptr() as usize),
                "items": spaces,
            },
            "surface_ids": history.surface_ids.iter().map(|id| id.0).collect::<Vec<_>>(),
            "SurfTempIn": handoff_real_array(&arrays.surf_temp_in),
            "SurfCTFConstInPart": handoff_real_array(&arrays.surf_ctf_const_in_part),
            "SurfCTFConstOutPart": handoff_real_array(&arrays.surf_ctf_const_out_part),
            "normal_histories": handoff_histories(&history.normal),
            "master_histories": history.masters.as_ref().map(|master| serde_json::json!({
                "arrays": handoff_histories(&master.arrays),
                "SurfCurrNumHist": master.current_num_histories,
            })),
            "UpdateThermalHistoriesFirstTimeFlag": history.update_first_time,
        });
        buffer.handoffs.push(snapshot);
    });
}
