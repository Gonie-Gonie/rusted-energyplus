//! Actual selected PostIPProcessing and DataOutputs pre-scan storage.
//! Raw lexical strings are separate from later uppercase getObjectItem alphas.

use super::error_backend::{SourceErrorBackend, SourceFatalStop};
use super::selected_input_processor::{
    SelectedInputProcessor, SourceInputFamily, SourceInputProcessorShortCut,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

// Build metadata observed in the pinned official Windows DLL stderr, separate
// from model values and from UtilityRoutines' error/console stream owners.
// unit72 official-regex-budget-t698992/stderr.log SHA256 8b7cce9206571b5d88a63965bfafbe2b996b13b36c914dfb4b50b86ecd4403e6.
pub(crate) const REPORTING_RE2_LOG_ORIGIN: ep_re2::TargetDiagnosticOrigin<'static> =
    ep_re2::TargetDiagnosticOrigin {
        source_file: br"D:\a\EnergyPlus\EnergyPlus\third_party\re2\re2\re2.cc",
        line_ending: b"\r\n",
    };

/// Raw Output:Variable fields in exact epJSON object-map order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourcePreScanVariable {
    /// Exact original object-map identity.
    pub object_name: String,
    /// Raw key, with only the actual absent/empty-string default applied.
    pub key: String,
    /// Raw variable name, before any getter case conversion.
    pub variable_name: String,
}

/// Raw SummaryReports extension strings in their actual array order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourcePreScanSummary {
    /// Exact original object-map identity.
    pub object_name: String,
    /// Actual report_name values; no monthly variables are precomputed here.
    pub report_names: Vec<String>,
}

/// Caller-owned raw operands; family presence/count remains with the sole IP.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceReportingPreScanInput {
    /// Actual Output:Variable objects, in raw JSON lexical order.
    pub variables: Vec<SourcePreScanVariable>,
    /// Actual Output:Table:SummaryReports objects, in raw JSON lexical order.
    pub summaries: Vec<SourcePreScanSummary>,
}

/// Original record spelling survives case-insensitive map insertion.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceOutputReportingVariable {
    /// First inserted key spelling.
    pub key: String,
    /// Name of this inserted record, including its original case.
    pub variable_name: String,
    /// Native detector classification; only the exact single-star key is exempt.
    pub is_simple_string: bool,
    /// Actual raw-pattern construction; absent for a simple string.
    pub pattern: Option<Arc<ep_re2::Re2>>,
    /// Separately constructed (?i)-prefixed pattern; absent for a simple string.
    pub case_insensitive_pattern: Option<Arc<ep_re2::Re2>>,
}

/// The outer map's first key spelling and its case-insensitive key map.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceOutputVariableKeys {
    /// First inserted outer variable-name spelling.
    pub variable_name: String,
    /// ASCII-folded lookup keys; records retain the original spelling.
    pub keys: BTreeMap<String, SourceOutputReportingVariable>,
}

/// Sole selected OutputsData members; distinct from OutputProcessor registration.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SourceOutputPreScan {
    /// Original MaxConsideredOutputVariables, constructor zero.
    pub max_considered_output_variables: i32,
    /// Original increments on every call, including duplicate emplace attempts.
    pub num_considered_output_variables: i32,
    /// Actual OutputsData.iNumberOfRecords, constructor zero; Native322 produces it.
    pub number_input_records: i32,
    /// Actual OutputsData.iNumberOfDefaultedFields, constructor zero; Native322 produces it.
    pub number_defaulted_fields: i32,
    /// Actual OutputsData.iTotalFieldsWithDefaults, constructor zero; Native322 produces it.
    pub total_fields_with_defaults: i32,
    /// Actual OutputsData.iNumberOfAutoSizedFields, constructor zero; Native322 produces it.
    pub number_autosized_fields: i32,
    /// Actual OutputsData.iTotalAutoSizableFields, constructor zero; Native322 produces it.
    pub total_autosizable_fields: i32,
    /// Actual OutputsData.iNumberOfAutoCalcedFields, constructor zero; Native322 produces it.
    pub number_autocalculated_fields: i32,
    /// Actual OutputsData.iTotalAutoCalculatableFields, constructor zero; Native322 produces it.
    pub total_autocalculatable_fields: i32,
    /// ASCII-folded lookup names; values preserve the first outer spelling.
    pub output_variables_for_simulation: BTreeMap<String, SourceOutputVariableKeys>,
}

/// Actual fatal/backend stop or explicit unrepresented input/storage branch.
#[derive(Debug)]
pub enum SourcePreScanError {
    /// Original PostIPProcessing fatal path, retaining the backend cause.
    Fatal(SourceFatalStop),
    /// Positive branch, invalid raw association, or unrepresented integer range.
    Unavailable(&'static str),
}
impl std::fmt::Display for SourcePreScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fatal(error) => std::fmt::Display::fmt(error, f),
            Self::Unavailable(error) => write!(f, "selected output pre-scan unavailable: {error}"),
        }
    }
}
impl std::error::Error for SourcePreScanError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Fatal(error) => Some(error),
            Self::Unavailable(_) => None,
        }
    }
}

fn raw_object_names<'a>(input: &'a SelectedInputProcessor, family: &str) -> Vec<&'a str> {
    match input.family(family) {
        Some(SourceInputFamily::Objects(objects)) => objects
            .iter()
            .map(|object| object.object_name.as_str())
            .collect(),
        None | Some(SourceInputFamily::Null) => Vec::new(),
    }
}

fn validate_names<'a>(
    input: &SelectedInputProcessor,
    family: &str,
    names: impl Iterator<Item = &'a str>,
) -> Result<(), SourcePreScanError> {
    let names: Vec<_> = names.collect();
    if names != raw_object_names(input, family)
        || names.iter().copied().collect::<BTreeSet<_>>().len() != names.len()
    {
        return Err(SourcePreScanError::Unavailable(
            "raw lexical object association",
        ));
    }
    Ok(())
}

/// Exact DataOutputs::isKeyRegexLike byte detector, shared by both owners.
pub(crate) fn is_key_regex_like(key: &str) -> bool {
    key != "*"
        && key
            .as_bytes()
            .iter()
            .any(|byte| b"*+?()|[]\\.".contains(byte))
}

// ObjexxFCL::lessthani lowercases ASCII then compares char values. The pinned
// Windows/GCC and Windows/MSVC targets have signed char. Preserve that scan
// order even though retained folded String lookup keys use Rust byte ordering.
// Only comparison operands are folded; compiled patterns retain original bytes.
fn native_key_order(left: &str, right: &str) -> std::cmp::Ordering {
    left.bytes()
        .map(|byte| byte.to_ascii_lowercase() as i8)
        .cmp(right.bytes().map(|byte| byte.to_ascii_lowercase() as i8))
}

impl SourceOutputReportingVariable {
    fn new(
        key: &str,
        variable_name: &str,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<Self, SourcePreScanError> {
        let mut record = Self {
            key: key.into(),
            variable_name: variable_name.into(),
            is_simple_string: !is_key_regex_like(key),
            pattern: None,
            case_insensitive_pattern: None,
        };
        if record.is_simple_string {
            return Ok(record);
        }
        // Native constructs BOTH objects before testing only raw pattern.ok().
        record.pattern = Some(Arc::new(ep_re2::Re2::new_logged(
            key,
            REPORTING_RE2_LOG_ORIGIN,
        )));
        record.case_insensitive_pattern = Some(Arc::new(ep_re2::Re2::new_logged(
            &format!("(?i){key}"),
            REPORTING_RE2_LOG_ORIGIN,
        )));
        let pattern = record
            .pattern
            .as_ref()
            .ok_or(SourcePreScanError::Unavailable("raw pattern storage"))?;
        if !pattern.ok() {
            backend
                .show_severe_error(&format!(
                    "Regular expression \"{key}\" for variable name \"{variable_name}\" in input file is incorrect"
                ))
                .map_err(|error| SourcePreScanError::Fatal(error.into()))?;
            backend
                .show_continue_error_bytes(pattern.error().as_bytes())
                .map_err(|error| SourcePreScanError::Fatal(error.into()))?;
            return match backend.show_fatal_error(
                "Error found in regular expression. Previous error(s) cause program termination.",
            ) {
                Ok(never) => match never {},
                Err(error) => Err(SourcePreScanError::Fatal(error)),
            };
        }
        Ok(record)
    }
}

impl SourceOutputPreScan {
    /// Original addRecordToOutputVariableStructure, including regex construction.
    ///
    /// # Errors
    /// Retains reached backend effects/map writes; signed overflow is unavailable.
    pub fn add_record(
        &mut self,
        key: &str,
        variable_name: &str,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SourcePreScanError> {
        let name = variable_name
            .split_once('[')
            .map_or(variable_name, |(name, _)| name)
            .trim_end_matches(' ');
        let name_lookup = name.to_ascii_lowercase();
        let key_lookup = key.to_ascii_lowercase();
        // Native finds the outer map first, then constructs even on duplicate
        // emplace. Invalid duplicates therefore still execute the fatal path.
        let name_present = self
            .output_variables_for_simulation
            .contains_key(&name_lookup);
        let record = SourceOutputReportingVariable::new(key, name, backend)?;
        if name_present {
            self.output_variables_for_simulation
                .get_mut(&name_lookup)
                .ok_or(SourcePreScanError::Unavailable("outer variable storage"))?
                .keys
                .entry(key_lookup)
                .or_insert(record);
        } else {
            let mut keys = BTreeMap::new();
            keys.insert(key_lookup, record);
            self.output_variables_for_simulation.insert(
                name_lookup,
                SourceOutputVariableKeys {
                    variable_name: name.into(),
                    keys,
                },
            );
        }
        self.num_considered_output_variables =
            self.num_considered_output_variables.checked_add(1).ok_or(
                SourcePreScanError::Unavailable("NumConsideredOutputVariables signed overflow"),
            )?;
        Ok(())
    }

    /// Real DataOutputs::FindItemInVariableList against retained raw+CI patterns.
    #[must_use]
    pub fn find_item(&self, key: &str, variable_name: &str) -> bool {
        let Some(keys) = self
            .output_variables_for_simulation
            .get(&variable_name.to_ascii_lowercase())
        else {
            return false;
        };
        if keys.keys.contains_key(&key.to_ascii_lowercase()) {
            return true;
        }
        if keys.keys.contains_key("*") {
            return true;
        }
        let mut records: Vec<_> = keys.keys.iter().collect();
        records.sort_by(|(left, _), (right, _)| native_key_order(left, right));
        for (_, record) in records {
            if record.key.eq_ignore_ascii_case(key) {
                return true;
            }
            if record.is_simple_string {
                continue;
            }
            if record.pattern.as_ref().is_some_and(|pattern| {
                pattern.full_match_logged(key.as_bytes(), REPORTING_RE2_LOG_ORIGIN)
            }) || record
                .case_insensitive_pattern
                .as_ref()
                .is_some_and(|pattern| {
                    pattern.full_match_logged(key.as_bytes(), REPORTING_RE2_LOG_ORIGIN)
                })
            {
                return true;
            }
        }
        false
    }

    /// Actual selected preScanReportingVariables; it has no once-only guard.
    ///
    /// # Errors
    /// Returns at the reached unsupported positive branch, preserving prior writes.
    pub fn pre_scan_reporting_variables(
        &mut self,
        input: &SelectedInputProcessor,
        raw: &SourceReportingPreScanInput,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SourcePreScanError> {
        self.max_considered_output_variables = 10000;
        validate_names(
            input,
            "Output:Variable",
            raw.variables
                .iter()
                .map(|object| object.object_name.as_str()),
        )?;
        for object in &raw.variables {
            self.add_record(&object.key, &object.variable_name, backend)?;
        }
        // Direct epJSON.find/iteration boundaries, not synthetic IP count queries.
        for family in [
            "Meter:Custom",
            "Meter:CustomDecrement",
            "EnergyManagementSystem:Sensor",
            "EnergyManagementSystem:OutputVariable",
            "Output:Table:TimeBins",
            "Output:Table:Monthly",
            "Output:Table:Annual",
        ] {
            if !raw_object_names(input, family).is_empty() {
                return Err(SourcePreScanError::Unavailable(
                    "positive pre-scan family outside selected admission",
                ));
            }
        }
        // This caller has no API variable-request producer. It does not invent one.
        validate_names(
            input,
            "Output:Table:SummaryReports",
            raw.summaries
                .iter()
                .map(|object| object.object_name.as_str()),
        )?;
        for object in &raw.summaries {
            for report in &object.report_names {
                // The real addVariablesForMonthlyReport has no ALLSUMMARY branch.
                // All other selected-positive summary names are excluded upstream.
                if !report.eq_ignore_ascii_case("ALLSUMMARY") {
                    return Err(SourcePreScanError::Unavailable(
                        "positive monthly summary expansion",
                    ));
                }
            }
        }
        Ok(())
    }
}

/// Actual PostIPProcessing prefix for the admitted no-preprocessor/API domain.
///
/// # Errors
/// Preserves DoingInputProcessing, CurrentModuleObject, flags and reached map writes.
pub fn post_input_processing(
    input: &SelectedInputProcessor,
    shortcut: &mut SourceInputProcessorShortCut,
    raw: &SourceReportingPreScanInput,
    outputs: &mut SourceOutputPreScan,
    pre_p_fatal: bool,
    backend: &mut SourceErrorBackend<'_>,
) -> Result<(), SourcePreScanError> {
    backend.state.doing_input_processing = false;
    shortcut.current_module_object = "Output:PreprocessorMessage".into();
    if input.get_num_objects_found(&shortcut.current_module_object) > 0 {
        return Err(SourcePreScanError::Unavailable(
            "positive Output:PreprocessorMessage getter",
        ));
    }
    if pre_p_fatal {
        return match backend.show_fatal_error("Preprocessor condition(s) cause termination.") {
            Ok(never) => match never {},
            Err(error) => Err(SourcePreScanError::Fatal(error)),
        };
    }
    outputs.pre_scan_reporting_variables(input, raw, backend)
}
