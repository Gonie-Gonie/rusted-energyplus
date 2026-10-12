use super::ReportScanUnavailable;
use ep_model::{ReportInputObjectType as Kind, ReportInputs};

/// Actual first-scan getter dependency, borrowing the one input and full shortcut.
pub(crate) struct SourceReportScanInput<'a> {
    input: &'a mut crate::heat_balance::selected_input_processor::SelectedInputProcessor,
    shortcut: &'a mut crate::heat_balance::selected_input_processor::SourceInputProcessorShortCut,
}

impl<'a> SourceReportScanInput<'a> {
    pub(crate) fn new(
        input: &'a mut crate::heat_balance::selected_input_processor::SelectedInputProcessor,
        shortcut: &'a mut crate::heat_balance::selected_input_processor::SourceInputProcessorShortCut,
        reports: &ReportInputs,
    ) -> Result<Self, ReportScanUnavailable> {
        use crate::heat_balance::selected_input_processor::SourceInputFamily;
        for kind in [
            Kind::SurfacesList,
            Kind::SurfacesDrawing,
            Kind::VariableDictionary,
            Kind::Constructions,
            Kind::EnergyManagementSystem,
        ] {
            // Admission validation inspects storage without adding a getter call.
            let raw_count = match input.family(kind.source_name()) {
                Some(SourceInputFamily::Objects(objects)) => objects.len(),
                Some(SourceInputFamily::Null) | None => 0,
            };
            let count = reports
                .objects
                .iter()
                .filter(|object| object.object_type == kind)
                .count();
            if raw_count != count {
                return Err(ReportScanUnavailable::SourceInputAssociation);
            }
            for (index, object) in reports
                .objects
                .iter()
                .enumerate()
                .filter(|(_, object)| object.object_type == kind)
            {
                if !input.has_object(kind.source_name(), &object.object_name)
                    || reports.objects[..index].iter().any(|prior| {
                        prior.object_type == kind && prior.object_name == object.object_name
                    })
                {
                    return Err(ReportScanUnavailable::SourceInputAssociation);
                }
            }
        }
        let required = reports
            .objects
            .iter()
            .map(|object| Self::alpha_fields(object.object_type).len())
            .max()
            .unwrap_or(0);
        let alphas = shortcut.alpha_args.len();
        let numerics = shortcut.numeric_args.len();
        if alphas < required
            || shortcut.alpha_field_names.len() != alphas
            || shortcut.alpha_field_blanks.len() != alphas
            || shortcut.numeric_field_names.len() != numerics
            || shortcut.numeric_field_blanks.len() != numerics
            || reports.objects.iter().any(|object| {
                usize::from(object.num_alphas) > Self::alpha_fields(object.object_type).len()
            })
        {
            return Err(ReportScanUnavailable::SourceInputAssociation);
        }
        Ok(Self { input, shortcut })
    }

    // Literal labels of the five already-projected report getters. This supplies
    // no definition counts or second generic schema authority.
    fn alpha_fields(kind: Kind) -> &'static [(&'static str, &'static str)] {
        match kind {
            Kind::SurfacesList => &[
                ("report_type", "Report Type"),
                ("report_specifications", "Report Specifications"),
            ],
            Kind::SurfacesDrawing => &[
                ("report_type", "Report Type"),
                ("report_specifications_1", "Report Specifications 1"),
                ("report_specifications_2", "Report Specifications 2"),
            ],
            Kind::VariableDictionary => {
                &[("key_field", "Key Field"), ("sort_option", "Sort Option")]
            }
            Kind::Constructions => &[
                ("details_type_1", "Details Type 1"),
                ("details_type_2", "Details Type 2"),
            ],
            Kind::EnergyManagementSystem => &[
                (
                    "actuator_availability_dictionary_reporting",
                    "Actuator Availability Dictionary Reporting",
                ),
                (
                    "internal_variable_availability_dictionary_reporting",
                    "Internal Variable Availability Dictionary Reporting",
                ),
                (
                    "ems_runtime_language_debug_output_level",
                    "EMS Runtime Language Debug Output Level",
                ),
            ],
        }
    }

    pub(super) fn legacy_sections(&self) -> Result<i32, ReportScanUnavailable> {
        self.input
            .get_num_sections_found("Report Variable Dictionary")
            .map_err(|_| ReportScanUnavailable::SourceInputAssociation)
    }

    pub(super) fn enter_family(&mut self, kind: Kind) {
        self.shortcut.current_module_object = kind.source_name().into();
        let _ = self
            .input
            .get_num_objects_found(&self.shortcut.current_module_object);
    }

    pub(super) fn get_object(&mut self, object: &ep_model::ReportInputObject) {
        // InputProcessor1011-1023 resets the entire allocated arrays, not merely
        // the current getter's prefix. Definitions then write all fixed fields,
        // including defaults beyond the actual returned NumAlphas.
        self.shortcut.alpha_args.fill(String::new());
        self.shortcut.numeric_args.fill(0.0);
        self.shortcut.numeric_field_blanks.fill(true);
        self.shortcut.alpha_field_blanks.fill(true);
        self.shortcut.alpha_field_names.fill(String::new());
        self.shortcut.numeric_field_names.fill(String::new());
        // Source1026-1029 erases this exact object AFTER all six resets and
        // BEFORE writing any field. Earlier/later objects keep their own state.
        self.input
            .mark_object_as_used(object.object_type.source_name(), &object.object_name);
        for (index, &(key, label)) in Self::alpha_fields(object.object_type).iter().enumerate() {
            self.shortcut.alpha_args[index].clone_from(&object.alphas[index]);
            self.shortcut.alpha_field_blanks[index] = object.alpha_blank[index];
            self.shortcut.alpha_field_names[index] =
                if object.is_epjson { key } else { label }.into();
        }
    }
}
