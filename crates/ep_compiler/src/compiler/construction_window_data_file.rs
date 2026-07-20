use super::{Compiler, WINDOW_DATA_FILE_CONSTRUCTION_OBJECT_TYPE};
use ep_model::{
    ConstructionWindowDataFileRequest, NormalizedName, TypedModel, WindowDataFileSource,
};

impl Compiler<'_> {
    pub(super) fn parse_window_data_file_construction_requests(&mut self, model: &mut TypedModel) {
        const OBJECT_TYPE: &str = WINDOW_DATA_FILE_CONSTRUCTION_OBJECT_TYPE;

        for (source_index, (name, object)) in self.objects(OBJECT_TYPE).into_iter().enumerate() {
            let mut fields_valid = true;
            if name.trim().is_empty() {
                self.error(
                    "MissingRequiredField",
                    OBJECT_TYPE,
                    Some(&name),
                    Some("name"),
                    format!("{OBJECT_TYPE} requires a nonblank object name"),
                );
                fields_valid = false;
            }

            let source = match self.optional_reference_name_checked(
                OBJECT_TYPE,
                &name,
                &object,
                "file_name",
            ) {
                Some(Some(file_name)) => WindowDataFileSource::Explicit(file_name),
                Some(None) => WindowDataFileSource::DefaultWorkingDirectory,
                None => {
                    fields_valid = false;
                    WindowDataFileSource::DefaultWorkingDirectory
                }
            };
            if !fields_valid {
                continue;
            }

            let Some(source_index) = self.checked_id(OBJECT_TYPE, &name, source_index) else {
                continue;
            };
            model
                .construction_window_data_file_requests
                .push(ConstructionWindowDataFileRequest {
                    source_index,
                    name: NormalizedName::new(&name),
                    source,
                });
        }
    }
}
