//! Runtime result-store value types and duplicate diagnostics.

use super::{
    OutputIdentity, RuntimeDiagnostic, RuntimeDiagnosticCode, RuntimeDiagnosticSeverity,
    RuntimeDiagnosticStore, RuntimeOutputFrequency,
};
use ep_model::OutputHandle;
use std::collections::BTreeSet;
/// One output series stored by the runtime.
#[derive(Clone, Debug, PartialEq)]
pub struct OutputSeries {
    /// Stable output handle for the current run.
    pub handle: OutputHandle,
    /// EnergyPlus-style output key.
    pub key: String,
    /// Output variable name.
    pub variable_name: String,
    /// Display units.
    pub units: String,
    /// Sampled output values.
    pub values: Vec<f64>,
}

/// Structured output store for runtime-native results.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ResultStore {
    /// Output series in handle order.
    pub series: Vec<OutputSeries>,
}

impl ResultStore {
    /// Creates an empty result store.
    #[must_use]
    pub fn new() -> Self {
        Self { series: Vec::new() }
    }

    /// Adds a complete output series.
    pub fn add_series(&mut self, series: OutputSeries) {
        self.series.push(series);
    }

    /// Returns the maximum sample count across all output series.
    #[must_use]
    pub fn sample_count(&self) -> usize {
        self.series
            .iter()
            .map(|series| series.values.len())
            .max()
            .unwrap_or(0)
    }

    /// Finds one output series by EnergyPlus-style key and variable name.
    #[must_use]
    pub fn find_series(&self, key: &str, variable_name: &str) -> Option<&OutputSeries> {
        self.series.iter().find(|series| {
            series.key.eq_ignore_ascii_case(key)
                && series.variable_name.eq_ignore_ascii_case(variable_name)
        })
    }

    /// Finds one output series by runtime output handle.
    #[must_use]
    pub fn find_handle(&self, handle: OutputHandle) -> Option<&OutputSeries> {
        self.series.iter().find(|series| series.handle == handle)
    }

    /// Returns result-store diagnostics for duplicate handles or identities.
    #[must_use]
    pub fn diagnostics(&self) -> RuntimeDiagnosticStore {
        let mut diagnostics = RuntimeDiagnosticStore::new();
        let mut handles = BTreeSet::new();
        let mut identities = BTreeSet::new();

        for series in &self.series {
            if !handles.insert(series.handle.0) {
                diagnostics.push(RuntimeDiagnostic {
                    severity: RuntimeDiagnosticSeverity::Error,
                    code: RuntimeDiagnosticCode::DuplicateOutputHandle,
                    message: format!("duplicate runtime output handle {}", series.handle.0),
                    key: Some(series.key.clone()),
                    variable_name: Some(series.variable_name.clone()),
                    meter_name: None,
                    handle: Some(series.handle),
                });
            }

            let identity = OutputIdentity::new(
                &series.key,
                &series.variable_name,
                RuntimeOutputFrequency::Hourly,
            );
            if !identities.insert(identity) {
                diagnostics.push(RuntimeDiagnostic {
                    severity: RuntimeDiagnosticSeverity::Error,
                    code: RuntimeDiagnosticCode::DuplicateOutputSeries,
                    message: format!(
                        "duplicate runtime output series {} / {}",
                        series.key, series.variable_name
                    ),
                    key: Some(series.key.clone()),
                    variable_name: Some(series.variable_name.clone()),
                    meter_name: None,
                    handle: Some(series.handle),
                });
            }
        }

        diagnostics
    }

    /// Returns a compact profile snapshot for reports and release evidence.
    #[must_use]
    pub fn profile(&self) -> ResultStoreProfile {
        ResultStoreProfile {
            series_count: self.series.len(),
            sample_count: self.sample_count(),
            empty_series_count: self
                .series
                .iter()
                .filter(|series| series.values.is_empty())
                .count(),
        }
    }
}

/// Compact result-store profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResultStoreProfile {
    /// Number of output series.
    pub series_count: usize,
    /// Maximum sample count across series.
    pub sample_count: usize,
    /// Number of output series without samples.
    pub empty_series_count: usize,
}
