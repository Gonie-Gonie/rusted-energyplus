//! Actual PluginManager188-384 body for the explicit LINK_WITH_PYTHON=1 domain.
//! Installed-reference configuration is not inferred from bundled Python files.
use super::{
    SelectedInputProcessor, SelectedInputUnavailable, SelectedPluginManager, SelectedRunInputError,
};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::selected_input_processor::SourceInputFamily;

impl SelectedPluginManager {
    /// Native Python-enabled output setup; receiver is the retained real manager.
    /// The caller must establish LINK_WITH_PYTHON=1 before selecting this API.
    /// No runtime build flag is invented. The proven source-built OFF body is
    /// empty and must not select this count-query path.
    pub(crate) fn setup_source_output_variables_with_python(
        &self,
        input: &mut SelectedInputProcessor,
        backend: &mut SourceErrorBackend<'_>,
    ) -> Result<(), SelectedRunInputError> {
        const FAMILY: &str = "PythonPlugin:OutputVariable";
        let number = i32::try_from(input.get_num_objects_found(FAMILY))
            .map_err(|_| SelectedInputUnavailable::Size)?;
        if number > 0 {
            // Exact epJSON.find identity, independently of case-fallback counts.
            let first_name = match input.find_exact_family(FAMILY) {
                None => {
                    backend.show_severe_error(&format!(
                        "{FAMILY}: Somehow getNumObjectsFound was > 0 but epJSON.find found 0"
                    ))?;
                    return Err(SelectedRunInputError::PositiveFamily(
                        "PythonPlugin:OutputVariable instances.value on end200",
                    ));
                }
                Some(SourceInputFamily::Null) => None,
                Some(SourceInputFamily::Objects(rows)) => rows
                    .iter()
                    .min_by(|a, b| a.object_name.cmp(&b.object_name))
                    .map(|row| row.object_name.clone()),
            };
            if let Some(name) = first_name {
                // Transient key identity releases the immutable family borrow;
                // the same IP's independent unusedInputs owner is mutated.
                input.mark_object_as_used(FAMILY, &name);
                return Err(SelectedRunInputError::PositiveFamily(
                    "PythonPlugin:OutputVariable fields.at python_plugin_variable_name206",
                ));
            }
        }
        Ok(())
    }
}
