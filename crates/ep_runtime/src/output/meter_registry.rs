//! Runtime meter registry and request resolution.

use super::{
    MeterIdentity, RuntimeDiagnostic, RuntimeDiagnosticCode, RuntimeDiagnosticSeverity,
    RuntimeDiagnosticStore, RuntimeMeterDefinition, RuntimeMeterRequest, RuntimeMeterResolution,
    RuntimeOutputFrequency, RuntimeOutputSource, RuntimeResolvedMeter,
};
use ep_model::OutputHandle;
use std::collections::BTreeSet;
/// Runtime meter registry.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct RuntimeMeterRegistry {
    meters: Vec<RuntimeMeterDefinition>,
}

impl RuntimeMeterRegistry {
    /// Creates an empty meter registry.
    #[must_use]
    pub fn new() -> Self {
        Self { meters: Vec::new() }
    }

    /// Returns meter definitions in handle order.
    #[must_use]
    pub fn meters(&self) -> &[RuntimeMeterDefinition] {
        &self.meters
    }

    /// Returns the number of registered meters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.meters.len()
    }

    /// Returns true when the registry contains no meters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.meters.is_empty()
    }

    pub(super) fn push_meter(
        &mut self,
        name: &str,
        units: &str,
        frequency: RuntimeOutputFrequency,
        source: RuntimeOutputSource,
    ) {
        let identity = MeterIdentity::new(name, frequency);
        if self
            .meters
            .iter()
            .any(|definition| definition.identity() == identity)
        {
            return;
        }

        self.meters.push(RuntimeMeterDefinition {
            handle: OutputHandle(self.meters.len() as u32),
            name: name.to_string(),
            units: units.to_string(),
            frequency,
            source,
        });
    }

    /// Resolves meter requests. v0.24 intentionally records unsupported meters
    /// as diagnostics rather than silently creating empty series.
    #[must_use]
    pub fn resolve_meter_requests(
        &self,
        requests: &[RuntimeMeterRequest],
    ) -> RuntimeMeterResolution {
        let mut seen = BTreeSet::new();
        let mut resolved = Vec::new();
        let mut diagnostics = RuntimeDiagnosticStore::new();

        for request in requests {
            let identity = request.identity();
            if !seen.insert(identity) {
                diagnostics.push(RuntimeDiagnostic {
                    severity: RuntimeDiagnosticSeverity::Error,
                    code: RuntimeDiagnosticCode::DuplicateMeterRequest,
                    message: format!(
                        "duplicate runtime meter request {} ({})",
                        request.name,
                        request.frequency.id()
                    ),
                    key: None,
                    variable_name: None,
                    meter_name: Some(request.name.clone()),
                    handle: None,
                });
                continue;
            }

            if let Some(definition) = self
                .meters
                .iter()
                .find(|definition| definition.identity() == request.identity())
            {
                resolved.push(RuntimeResolvedMeter {
                    request: request.clone(),
                    definition: definition.clone(),
                });
            } else {
                diagnostics.push(RuntimeDiagnostic {
                    severity: RuntimeDiagnosticSeverity::Error,
                    code: RuntimeDiagnosticCode::MeterUnavailable,
                    message: format!(
                        "runtime meter unavailable: {} ({})",
                        request.name,
                        request.frequency.id()
                    ),
                    key: None,
                    variable_name: None,
                    meter_name: Some(request.name.clone()),
                    handle: None,
                });
            }
        }

        RuntimeMeterResolution {
            resolved,
            diagnostics,
        }
    }
}
