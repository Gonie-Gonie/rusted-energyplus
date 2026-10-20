//! InputProcessor::checkVersionMatch on the same retained original raw JSON.
use super::{SelectedInputProcessor, SourceInputStatisticsValue};
use crate::heat_balance::error_backend::{SourceErrorBackend, SourceErrorBackendFailure};

/// An unavailable raw owner or a failed reached warning effect.
#[derive(Debug)]
pub enum SourceVersionCheckError {
    /// No original raw string is available; getter defaults cannot replace it.
    Unavailable,
    /// The genuine warning backend failed after its reached effects.
    Backend(SourceErrorBackendFailure),
}
impl std::fmt::Display for SourceVersionCheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unavailable => f.write_str("InputProcessor Version raw string unavailable"),
            Self::Backend(error) => error.fmt(f),
        }
    }
}
impl std::error::Error for SourceVersionCheckError {}

impl SelectedInputProcessor {
    /// Check advisory input text before initializeMaps and GetProjectData.
    /// A false match controls optional Native conversion, not simulation admission.
    ///
    /// # Errors
    /// Missing raw ownership/types and actual warning sink failures are preserved.
    pub fn check_source_version_match(
        &self,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<bool, SourceVersionCheckError> {
        let raw = self
            .statistics_input
            .as_ref()
            .ok_or(SourceVersionCheckError::Unavailable)?;
        let Some(objects) = raw.objects.get("Version") else {
            return Ok(true);
        };
        if objects.len() != self.get_num_objects_found("Version") {
            return Err(SourceVersionCheckError::Unavailable);
        }
        for fields in objects.values() {
            let Some(SourceInputStatisticsValue::String(version)) =
                fields.get("version_identifier")
            else {
                return Err(SourceVersionCheckError::Unavailable);
            };
            if version.is_empty() {
                backend
                    .show_warning_error(
                        "Input errors occurred and version ID was left blank, verify file version",
                    )
                    .map_err(SourceVersionCheckError::Backend)?;
            } else if !version.starts_with("26.1") {
                backend
                    .show_warning_error(&format!(
                        "Version: in IDF=\"{version}\" not the same as expected=\"26.1\""
                    ))
                    .map_err(SourceVersionCheckError::Backend)?;
                return Ok(false);
            }
        }
        Ok(true)
    }
}
