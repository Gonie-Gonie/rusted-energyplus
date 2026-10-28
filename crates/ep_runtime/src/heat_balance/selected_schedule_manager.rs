//! Genuine Constant/Compact ScheduleManager with retained native day/week owners.
use super::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use super::selected_input_processor::{SelectedInputProcessor, SelectedInputUnavailable};
use super::source_initialization::{
    ScheduleAuditFileHost, ScheduleInputFileWriters, SourceFileSearchState, SourceNativePathKey,
};
use crate::weather::day::WeatherGlobalState;
use ep_model::ScheduleUnitType;
use external_data::NativeExternalDataValue;
use std::collections::BTreeMap;
mod accessors;
mod annual;
mod compact;
#[cfg(test)]
mod compact_native_tests;
mod day_group;
mod detailed;
mod external_data;
mod file;
mod input_reports;
pub(crate) mod minute;
#[cfg(all(test, feature = "native-controlled-air-cohort"))]
mod native_gap_cohort;
mod predefined;
mod regular;
mod report;

/// Actual source-ordered ScheduleTypeLimits getter values after raw admission.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceScheduleTypeInput {
    /// Exact original object-map identity used by InputProcessor unused tracking.
    pub object_name: String,
    /// Getter uppercase name.
    pub name: String,
    /// Number field presence, preserving the constructor value when blank.
    pub lower: Option<f64>,
    /// Number field presence, preserving the constructor value when blank.
    pub upper: Option<f64>,
    /// Entered uppercase numeric-type key; used only when both bounds are present.
    pub numeric_type: Option<String>,
    /// Entered nonblank unit, otherwise the actual retained Invalid constructor state.
    pub unit: ScheduleUnitType,
}

/// Actual source-ordered Schedule:Constant getter values after raw admission.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceConstantScheduleInput {
    /// Exact original object-map identity used by InputProcessor unused tracking.
    pub object_name: String,
    /// Getter uppercase name, separate from built-in schedule names/indices.
    pub name: String,
    /// Getter uppercase type name, absent/blank retained for explicit selected stop.
    pub type_name: Option<String>,
    /// Actual number or schema default +0.0.
    pub value: f64,
}

/// Sole admitted source projection; no precomputed schedule-result table.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SelectedScheduleInput {
    /// Actual loader input format, required only by the new mixed local getters.
    /// Legacy Constant/Compact callers do not manufacture this owner.
    pub source_is_epjson: Option<bool>,
    /// Actual type declaration order.
    pub types: Vec<SourceScheduleTypeInput>,
    /// Actual constant declaration order.
    pub constants: Vec<SourceConstantScheduleInput>,
}

/// Native ScheduleManager index, never a typed user ScheduleId alias.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SourceScheduleIndex(pub(crate) usize);

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedScheduleType {
    pub(crate) name: String,
    pub(crate) number: usize,
    pub(crate) is_limited: bool,
    pub(crate) min_value: f64,
    pub(crate) max_value: f64,
    pub(crate) is_real: bool,
    pub(crate) limit_units: ScheduleUnitType,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedSchedule {
    pub(crate) name: String,
    pub(crate) number: SourceScheduleIndex,
    pub(crate) is_used: bool,
    pub(crate) min_value: f64,
    pub(crate) max_value: f64,
    pub(crate) is_min_max_set: bool,
    pub(crate) schedule_type_number: Option<usize>,
    pub(crate) ems_actuated_on: bool,
    pub(crate) ems_value: f64,
    pub(crate) current_value: f64,
    pub(crate) data: SelectedScheduleData,
}

#[derive(Clone, Debug, PartialEq)]
// Retain the actual owned schedule representation; boxing solely for this lint would alter physical owner layout.
#[allow(clippy::large_enum_variant)]
pub(crate) enum SelectedScheduleData {
    Constant { timestep_values: Vec<f64> },
    Detailed(SelectedDetailedSchedule),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedDetailedSchedule {
    pub(crate) schedule_kind: i32,
    pub(crate) week_schedules: [Option<usize>; 367],
    pub(crate) max_min_by_day_type_set: [bool; 13],
    pub(crate) min_by_day_type: [f64; 13],
    pub(crate) max_by_day_type: [f64; 13],
    pub(crate) use_daylight_saving: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedWeekSchedule {
    pub(crate) name: String,
    pub(crate) number: usize,
    pub(crate) is_used: bool,
    pub(crate) min_value: f64,
    pub(crate) max_value: f64,
    pub(crate) is_min_max_set: bool,
    pub(crate) day_schedules: [Option<usize>; 13],
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedDaySchedule {
    pub(crate) name: String,
    pub(crate) number: usize,
    pub(crate) is_used: bool,
    pub(crate) min_value: f64,
    pub(crate) max_value: f64,
    pub(crate) is_min_max_set: bool,
    pub(crate) schedule_type_number: Option<usize>,
    pub(crate) interpolation: i32,
    pub(crate) timestep_values: Vec<f64>,
    pub(crate) sum_timestep_values: f64,
}

/// One retained real manager, including actual built-ins and selected user objects.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SelectedScheduleManager {
    pub(crate) check_min_max_run_once_only: bool,
    pub(crate) do_schedule_reporting_setup: bool,
    pub(crate) schedule_input_processed: bool,
    pub(crate) schedule_file_shading_processed: bool,
    pub(crate) unique_processed_external_files:
        BTreeMap<SourceNativePathKey, NativeExternalDataValue>,
    pub(crate) schedule_types: Vec<SelectedScheduleType>,
    pub(crate) schedules: Vec<SelectedSchedule>,
    pub(crate) day_schedules: Vec<SelectedDaySchedule>,
    pub(crate) week_schedules: Vec<SelectedWeekSchedule>,
    pub(crate) schedule_type_map: BTreeMap<String, usize>,
    pub(crate) schedule_map: BTreeMap<String, SourceScheduleIndex>,
    pub(crate) day_schedule_map: BTreeMap<String, usize>,
    pub(crate) week_schedule_map: BTreeMap<String, usize>,
}
impl Default for SelectedScheduleManager {
    fn default() -> Self {
        Self {
            check_min_max_run_once_only: true,
            do_schedule_reporting_setup: true,
            schedule_input_processed: false,
            schedule_file_shading_processed: false,
            unique_processed_external_files: BTreeMap::new(),
            schedule_types: Vec::new(),
            schedules: Vec::new(),
            day_schedules: Vec::new(),
            week_schedules: Vec::new(),
            schedule_type_map: BTreeMap::new(),
            schedule_map: BTreeMap::new(),
            day_schedule_map: BTreeMap::new(),
            week_schedule_map: BTreeMap::new(),
        }
    }
}

#[derive(Debug)]
pub(crate) enum SelectedScheduleError {
    Input(SelectedInputUnavailable),
    HeatBalance(Box<super::selected_heat_balance_input::HeatBalanceInputError>),
    Unavailable(&'static str),
    Audit(std::io::Error),
    ExternalFile {
        path: std::path::PathBuf,
        source: std::io::Error,
    },
    Report(std::io::Error),
    AuditOpen {
        path: std::path::PathBuf,
        source: std::io::Error,
    },
}
impl From<SelectedInputUnavailable> for SelectedScheduleError {
    fn from(value: SelectedInputUnavailable) -> Self {
        Self::Input(value)
    }
}
impl From<super::selected_heat_balance_input::HeatBalanceInputError> for SelectedScheduleError {
    fn from(value: super::selected_heat_balance_input::HeatBalanceInputError) -> Self {
        Self::HeatBalance(Box::new(value))
    }
}
impl From<std::io::Error> for SelectedScheduleError {
    fn from(value: std::io::Error) -> Self {
        Self::Audit(value)
    }
}
impl std::fmt::Display for SelectedScheduleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input(e) => std::fmt::Display::fmt(e, f),
            Self::HeatBalance(e) => std::fmt::Display::fmt(e, f),
            Self::Audit(e) | Self::Report(e) => std::fmt::Display::fmt(e, f),
            Self::ExternalFile { path, source } => {
                write!(f, "actual external file {}: {source}", path.display())
            }
            Self::AuditOpen { path, .. } => write!(
                f,
                "ProcessScheduleInput: Could not open file {} for output (write).",
                path.display()
            ),
            Self::Unavailable(e) => write!(f, "selected ScheduleManager unavailable: {e}"),
        }
    }
}
impl std::error::Error for SelectedScheduleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Input(e) => Some(e),
            Self::HeatBalance(e) => Some(e.as_ref()),
            Self::Audit(e) | Self::Report(e) => Some(e),
            Self::AuditOpen { source, .. } | Self::ExternalFile { source, .. } => Some(source),
            Self::Unavailable(_) => None,
        }
    }
}

impl SelectedScheduleManager {
    fn add_constant(&mut self, name: &str, value: f64, slots: usize) -> SourceScheduleIndex {
        let number = SourceScheduleIndex(self.schedules.len());
        self.schedules.push(SelectedSchedule {
            name: name.into(),
            number,
            is_used: false,
            min_value: 0.0,
            max_value: 0.0,
            is_min_max_set: false,
            schedule_type_number: None,
            ems_actuated_on: false,
            ems_value: 0.0,
            current_value: value,
            data: SelectedScheduleData::Constant {
                timestep_values: vec![value; slots],
            },
        });
        self.schedule_map.insert(name.to_ascii_uppercase(), number);
        number
    }

    /// EnergyPlusData's real init_constant_state guard remains caller-owned.
    /// The actual constructor timestep operand may be zero, as in Original.
    pub(crate) fn init_constant_state(
        &mut self,
        time_steps_in_hour: usize,
    ) -> Result<(), SelectedScheduleError> {
        if !self.schedules.is_empty()
            || !self.day_schedules.is_empty()
            || !self.week_schedules.is_empty()
        {
            return Err(SelectedScheduleError::Unavailable(
                "repeated init_constant_state outside caller guard",
            ));
        }
        let slots = 24usize
            .checked_mul(time_steps_in_hour.max(1))
            .ok_or(SelectedScheduleError::Unavailable("timestep storage size"))?;
        let off = self.add_constant("Constant-0.0", 0.0, slots);
        self.schedules[off.0].is_used = true;
        let on = self.add_constant("Constant-1.0", 1.0, slots);
        self.schedules[on.0].is_used = true;
        self.day_schedules.push(SelectedDaySchedule {
            name: "MissingDaySchedule-0.0".into(),
            number: 0,
            is_used: true,
            min_value: 0.0,
            max_value: 0.0,
            is_min_max_set: false,
            schedule_type_number: None,
            interpolation: 0,
            timestep_values: vec![0.0; slots],
            sum_timestep_values: 0.0,
        });
        self.day_schedule_map
            .insert("MISSINGDAYSCHEDULE-0.0".into(), 0);
        Ok(())
    }

    /// The actual getter does not trigger ProcessScheduleInput.
    pub(crate) fn always_on(&self) -> Result<SourceScheduleIndex, SelectedScheduleError> {
        self.schedules
            .get(1)
            .filter(|s| s.name == "Constant-1.0")
            .map(|s| s.number)
            .ok_or(SelectedScheduleError::Unavailable(
                "init_constant_state prerequisite",
            ))
    }

    /// Existing callers without a diagnostic owner stop at the first requested warning.
    #[allow(
        dead_code,
        reason = "Keep the explicit no-diagnostic-owner ProcessScheduleInput entry; normal production uses the same getter with its actual diagnostic backend."
    )]
    pub(crate) fn process_schedule_input(
        &mut self,
        input: &mut SelectedInputProcessor,
        declarations: &SelectedScheduleInput,
        time_steps_in_hour: usize,
        audit_files: &mut dyn ScheduleAuditFileHost,
    ) -> Result<(), SelectedScheduleError> {
        self.process_schedule_input_with_optional_backend(
            input,
            declarations,
            time_steps_in_hour,
            audit_files,
            None,
            None,
            None,
            None,
        )
    }

    /// Existing backend-only callers retain their explicit legacy phase contract.
    #[allow(
        dead_code,
        reason = "Keep the backend-only entry for gated owner callers; actual Source uses borrowed live phase cells."
    )]
    pub(crate) fn process_schedule_input_with_backend(
        &mut self,
        input: &mut SelectedInputProcessor,
        declarations: &SelectedScheduleInput,
        time_steps_in_hour: usize,
        audit_files: &mut dyn ScheduleAuditFileHost,
        backend: &mut super::error_backend::SourceErrorBackend<'_>,
    ) -> Result<(), SelectedScheduleError> {
        self.process_schedule_input_with_optional_backend(
            input,
            declarations,
            time_steps_in_hour,
            audit_files,
            Some(backend),
            None,
            None,
            None,
        )
    }

    /// Same actual producer with genuine Source phase cells, never copied
    /// into the legacy backend's constructor state. Numeric owners are shared.
    #[allow(
        dead_code,
        reason = "Preserve the existing no-report-clock entry for explicit owner callers."
    )]
    pub(crate) fn process_schedule_input_with_live_backend(
        &mut self,
        input: &mut SelectedInputProcessor,
        declarations: &SelectedScheduleInput,
        time_steps_in_hour: usize,
        audit_files: &mut dyn ScheduleAuditFileHost,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
    ) -> Result<(), SelectedScheduleError> {
        self.process_schedule_input_with_optional_backend(
            input,
            declarations,
            time_steps_in_hour,
            audit_files,
            Some(backend),
            Some(flags),
            None,
            None,
        )
    }

    /// Actual Source entry with the same live clock consumed by ReportScheduleDetails.
    #[allow(
        dead_code,
        clippy::too_many_arguments,
        reason = "Retain the existing explicit clock-only entry for owner tests and callers without a CLI file capability."
    )]
    pub(crate) fn process_schedule_input_with_live_backend_and_clock(
        &mut self,
        input: &mut SelectedInputProcessor,
        declarations: &SelectedScheduleInput,
        time_steps_in_hour: usize,
        audit_files: &mut dyn ScheduleAuditFileHost,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
        clock: &WeatherGlobalState,
    ) -> Result<(), SelectedScheduleError> {
        self.process_schedule_input_with_optional_backend(
            input,
            declarations,
            time_steps_in_hour,
            audit_files,
            Some(backend),
            Some(flags),
            Some(clock),
            None,
        )
    }

    /// Actual Source entry carrying the same real SystemVars file-search cells.
    #[allow(
        clippy::too_many_arguments,
        reason = "Borrow independent real file, phase, clock and SystemVars owners."
    )]
    pub(crate) fn process_schedule_input_with_live_backend_clock_and_files(
        &mut self,
        input: &mut SelectedInputProcessor,
        declarations: &SelectedScheduleInput,
        time_steps_in_hour: usize,
        audit_files: &mut dyn ScheduleAuditFileHost,
        backend: &mut SourceErrorBackend<'_>,
        flags: &SourceDiagnosticFlags<'_>,
        clock: &WeatherGlobalState,
        file_search: &mut SourceFileSearchState,
    ) -> Result<(), SelectedScheduleError> {
        self.process_schedule_input_with_optional_backend(
            input,
            declarations,
            time_steps_in_hour,
            audit_files,
            Some(backend),
            Some(flags),
            Some(clock),
            Some(file_search),
        )
    }

    #[allow(
        clippy::too_many_arguments,
        reason = "Preserve optional real report-clock and diagnostic capabilities for legacy entries."
    )]
    fn process_schedule_input_with_optional_backend(
        &mut self,
        input: &mut SelectedInputProcessor,
        declarations: &SelectedScheduleInput,
        time_steps_in_hour: usize,
        audit_files: &mut dyn ScheduleAuditFileHost,
        mut backend: Option<&mut super::error_backend::SourceErrorBackend<'_>>,
        flags: Option<&SourceDiagnosticFlags<'_>>,
        report_clock: Option<&WeatherGlobalState>,
        file_search: Option<&mut SourceFileSearchState>,
    ) -> Result<(), SelectedScheduleError> {
        if self.schedule_input_processed {
            return Ok(());
        }
        self.schedule_input_processed = true;
        self.always_on()?;
        let mut max_numbers = 1;
        let mut max_alphas = 0;
        for (family, extra_alpha) in [
            ("ScheduleTypeLimits", 0),
            ("Schedule:Day:Hourly", 0),
            ("Schedule:Day:Interval", 0),
            ("Schedule:Day:List", 0),
            ("Schedule:Week:Daily", 0),
            ("Schedule:Week:Compact", 0),
            ("Schedule:Year", 0),
            ("Schedule:Compact", 1),
            ("Schedule:File", 0),
            ("Schedule:Constant", 0),
            ("ExternalInterface:Schedule", 1),
            (
                "ExternalInterface:FunctionalMockupUnitImport:To:Schedule",
                1,
            ),
            (
                "ExternalInterface:FunctionalMockupUnitExport:To:Schedule",
                1,
            ),
        ] {
            let count = input.get_num_objects_found(family);
            if count > 0 {
                let args = input.get_object_def_max_args(family)?;
                max_numbers = max_numbers.max(args.numerics);
                max_alphas = max_alphas.max(
                    args.alphas
                        .checked_add(extra_alpha)
                        .ok_or(SelectedScheduleError::Unavailable("local alpha size"))?,
                );
                if !matches!(
                    family,
                    "ScheduleTypeLimits"
                        | "Schedule:Constant"
                        | "Schedule:Compact"
                        | "Schedule:Day:Hourly"
                        | "Schedule:Day:Interval"
                        | "Schedule:Day:List"
                        | "Schedule:Week:Daily"
                        | "Schedule:Week:Compact"
                        | "Schedule:Year"
                        | "Schedule:File"
                ) {
                    return Err(SelectedScheduleError::Unavailable(
                        "nonconstant schedule input",
                    ));
                }
            }
        }
        let args = input.get_object_def_max_args("Output:Schedules")?;
        max_numbers = max_numbers.max(args.numerics);
        max_alphas = max_alphas.max(args.alphas);
        // Real getter-local arrays, distinct from dataIPShortCut and its false blanks.
        let mut locals: SourceScheduleGetterLocals = (
            vec![String::new(); max_alphas],
            vec![String::new(); max_alphas],
            vec![String::new(); max_numbers],
            vec![0.0; max_numbers],
            vec![true; max_alphas],
            vec![true; max_numbers],
        );
        // Native527-555: real Compact prescan, before audit or type getters.
        let mut max_compact_numbers = 0usize;
        for number in 0..input.get_num_objects_found("Schedule:Compact") {
            let alphas = Self::get_source_compact_object_item(input, number, &mut locals, false)?;
            for field in 2..alphas {
                if locals.0[field].starts_with("UNTIL") {
                    max_compact_numbers = max_compact_numbers.checked_add(1).ok_or(
                        SelectedScheduleError::Unavailable("actual Compact Until count"),
                    )?;
                }
            }
        }
        if max_compact_numbers > max_numbers {
            max_numbers = max_compact_numbers;
            // Native deallocates cNumericFields, Numbers, lNumericBlanks first.
            locals.2 = Vec::new();
            locals.3 = Vec::new();
            locals.5 = Vec::new();
            locals.2 = vec![String::new(); max_numbers];
            locals.3 = vec![0.0; max_numbers];
            locals.5 = vec![true; max_numbers];
        }
        if input.get_num_objects_found("Schedule:File:Shading") > 0 {
            return Err(SelectedScheduleError::Unavailable("file shading"));
        }
        if declarations.types.len() != input.get_num_objects_found("ScheduleTypeLimits")
            || declarations.constants.len() != input.get_num_objects_found("Schedule:Constant")
        {
            return Err(SelectedScheduleError::Unavailable(
                "raw declaration association",
            ));
        }
        for (family, names) in [
            (
                "ScheduleTypeLimits",
                declarations
                    .types
                    .iter()
                    .map(|row| row.object_name.as_str())
                    .collect::<Vec<_>>(),
            ),
            (
                "Schedule:Constant",
                declarations
                    .constants
                    .iter()
                    .map(|row| row.object_name.as_str())
                    .collect::<Vec<_>>(),
            ),
        ] {
            for (index, name) in names.iter().enumerate() {
                if !input.has_object(family, name) || names[..index].contains(name) {
                    return Err(SelectedScheduleError::Unavailable(
                        "raw object identity association",
                    ));
                }
            }
        }
        let audit_path = audit_files.audit_path().to_path_buf();
        let ScheduleInputFileWriters {
            audit,
            eio,
            external_files,
        } = audit_files
            .ensure_schedule_input_writers()
            .map_err(|source| SelectedScheduleError::AuditOpen {
                path: audit_path,
                source,
            })?;
        writeln!(audit, "  Processing Schedule Input -- Start")?;
        for row in &declarations.types {
            // getObjectItem resets all six getter-local arrays before unused erase.
            locals.0.fill(String::new());
            locals.1.fill(String::new());
            locals.2.fill(String::new());
            locals.3.fill(0.0);
            locals.4.fill(true);
            locals.5.fill(true);
            input.mark_object_as_used("ScheduleTypeLimits", &row.object_name);
            if self.schedule_type_map.contains_key(&row.name) {
                return Err(SelectedScheduleError::Unavailable(
                    "duplicate type diagnostic",
                ));
            }
            let number = self.schedule_types.len();
            self.schedule_types.push(SelectedScheduleType {
                name: row.name.clone(),
                number,
                is_limited: row.lower.is_some() && row.upper.is_some(),
                min_value: row.lower.unwrap_or(0.0),
                max_value: row.upper.unwrap_or(0.0),
                is_real: true,
                limit_units: ScheduleUnitType::Invalid,
            });
            self.schedule_type_map.insert(row.name.clone(), number);
            let retained = &mut self.schedule_types[number];
            if retained.is_limited {
                retained.is_real = match row.numeric_type.as_deref() {
                    Some("DISCRETE" | "INTEGER") => false,
                    Some("CONTINUOUS" | "REAL") => true,
                    _ => {
                        return Err(SelectedScheduleError::Unavailable(
                            "numeric type diagnostic",
                        ));
                    }
                };
            }
            retained.limit_units = row.unit;
            if retained.is_limited && retained.min_value > retained.max_value {
                return Err(SelectedScheduleError::Unavailable(
                    "reversed bounds diagnostic",
                ));
            }
        }
        let slots = 24usize
            .checked_mul(time_steps_in_hour)
            .ok_or(SelectedScheduleError::Unavailable("timestep storage size"))?;
        let mut errors_found = false; // Actual ProcessScheduleInput local.
        self.process_source_regulars(
            input,
            declarations.source_is_epjson,
            time_steps_in_hour,
            &mut locals,
            &mut backend,
            flags,
            &mut errors_found,
        )?;
        // Native1269 resizes the same MissingDaySchedule before Compact input.
        self.day_schedules
            .get_mut(0)
            .ok_or(SelectedScheduleError::Unavailable(
                "actual MissingDaySchedule0 before Compact",
            ))?
            .timestep_values = vec![0.0; slots];
        self.process_source_compacts(
            input,
            time_steps_in_hour,
            &mut locals,
            &mut backend,
            flags,
            &mut errors_found,
        )?;
        self.process_source_files(
            input,
            declarations.source_is_epjson,
            time_steps_in_hour,
            &mut locals,
            external_files,
            file_search,
            audit,
            &mut backend,
            flags,
            &mut errors_found,
        )?;
        for row in &declarations.constants {
            // getObjectItem resets all six getter-local arrays before unused erase.
            locals.0.fill(String::new());
            locals.1.fill(String::new());
            locals.2.fill(String::new());
            locals.3.fill(0.0);
            locals.4.fill(true);
            locals.5.fill(true);
            input.mark_object_as_used("Schedule:Constant", &row.object_name);
            if self.schedule_map.contains_key(&row.name) {
                return Err(SelectedScheduleError::Unavailable(
                    "duplicate schedule diagnostic",
                ));
            }
            let number = self.add_constant(&row.name, row.value, slots);
            let type_number = row
                .type_name
                .as_ref()
                .and_then(|name| self.schedule_type_map.get(name))
                .copied();
            if let Some(type_number) = type_number {
                self.schedules[number.0].schedule_type_number = Some(type_number);
            } else {
                let backend = backend
                    .as_deref_mut()
                    .ok_or(SelectedScheduleError::Unavailable(
                        "blank or unknown type warning",
                    ))?;
                // Native1988-1994 invokes the real UtilityRoutines helpers.
                // InputProcessor.cc1052 supplies the JSON key or IDF field
                // title as cAlphaFields(2), using the actual loader format.
                // no absent type is replaced by a declared ScheduleTypeLimits.
                backend
                    .show_warning_error(&format!(
                        "ProcessScheduleInput: Schedule:Constant = {}",
                        row.name,
                    ))
                    .map_err(super::selected_heat_balance_input::HeatBalanceInputError::from)?;
                let type_field = if declarations.source_is_epjson == Some(true) {
                    "schedule_type_limits_name"
                } else {
                    "Schedule Type Limits Name"
                };
                let field_message = match row.type_name.as_deref().filter(|name| !name.is_empty()) {
                    None => format!("{type_field} is empty."),
                    Some(name) => format!("{type_field} = {name}, item not found."),
                };
                backend
                    .show_continue_error(&field_message)
                    .map_err(super::selected_heat_balance_input::HeatBalanceInputError::from)?;
                backend
                    .show_continue_error("Schedule will not be validated.")
                    .map_err(super::selected_heat_balance_input::HeatBalanceInputError::from)?;
                // AddScheduleConstant's actual Invalid (-1), represented by
                // None on the retained Source schedule, remains untouched.
            }
        }
        self.resize_source_constant(0, slots, 0.0)?;
        self.resize_source_constant(1, slots, 1.0)?;
        for index in 0..self.schedules.len() {
            let Some(type_number) = self.schedules[index].schedule_type_number else {
                continue;
            };
            let limits = &self.schedule_types[type_number];
            if !limits.is_limited {
                continue;
            }
            let (minimum, maximum) = (limits.min_value, limits.max_value);
            self.ensure_source_min_max_with_steps(SourceScheduleIndex(index), time_steps_in_hour)?;
            let schedule = &self.schedules[index];
            // Original ScheduleBase::checkMinMaxVals, inclusive subtraction first.
            let min_ok = f64::from(f32::EPSILON) >= minimum - schedule.min_value;
            let max_ok = schedule.max_value - maximum <= f64::from(f32::EPSILON);
            if !(min_ok && max_ok) {
                return Err(SelectedScheduleError::Unavailable(
                    "schedule bounds severe/fatal",
                ));
            }
        }
        if errors_found {
            return schedule_fatal(
                &mut backend,
                flags,
                "ProcessScheduleInput: Preceding Errors cause termination.",
            );
        }
        if input.get_num_objects_found("Output:Schedules") > 0 {
            self.report_source_schedule_inputs(
                input,
                declarations.source_is_epjson,
                time_steps_in_hour,
                report_clock,
                &mut locals,
                eio.ok_or(SelectedScheduleError::Unavailable(
                    "actual schedule report EIO stream",
                ))?,
                &mut backend,
                flags,
            )?;
        }
        drop(locals);
        writeln!(audit, "  Processing Schedule Input -- Complete")?;
        Ok(())
    }
}

type SourceScheduleGetterLocals = (
    Vec<String>,
    Vec<String>,
    Vec<String>,
    Vec<f64>,
    Vec<bool>,
    Vec<bool>,
);

// Existing numerical callers can remain backend-free and stop at the first
// requested diagnostic. Actual Source calls borrow the real phase roots.
fn schedule_diagnostic(
    backend: &mut Option<&mut SourceErrorBackend<'_>>,
    flags: Option<&SourceDiagnosticFlags<'_>>,
    kind: minute::SourceScheduleMinuteDiagnostic,
    message: &str,
) -> Result<(), SelectedScheduleError> {
    use super::selected_heat_balance_input::HeatBalanceInputError;
    use minute::SourceScheduleMinuteDiagnostic::{Continue, Severe, Warning};
    let backend = backend
        .as_deref_mut()
        .ok_or(SelectedScheduleError::Unavailable(
            "actual Compact minute diagnostic backend",
        ))?;
    if let Some(flags) = flags {
        match kind {
            Warning => backend.show_source_warning_error(message, flags)?,
            Severe => backend.show_source_severe_error(message, flags)?,
            Continue => backend
                .show_continue_error(message)
                .map_err(HeatBalanceInputError::from)?,
        }
    } else {
        match kind {
            Warning => backend.show_warning_error(message),
            Severe => backend.show_severe_error(message),
            Continue => backend.show_continue_error(message),
        }
        .map_err(HeatBalanceInputError::from)?;
    }
    Ok(())
}
fn schedule_fatal(
    backend: &mut Option<&mut SourceErrorBackend<'_>>,
    flags: Option<&SourceDiagnosticFlags<'_>>,
    message: &str,
) -> Result<(), SelectedScheduleError> {
    use super::selected_heat_balance_input::HeatBalanceInputError;
    let backend = backend
        .as_deref_mut()
        .ok_or(SelectedScheduleError::Unavailable(
            "actual Compact final fatal backend",
        ))?;
    if let Some(flags) = flags {
        match backend.show_source_fatal_error(message, flags) {
            Ok(never) => match never {},
            Err(error) => Err(error.into()),
        }
    } else {
        match backend.show_fatal_error(message) {
            Ok(never) => match never {},
            Err(error) => Err(HeatBalanceInputError::Fatal(error).into()),
        }
    }
}
